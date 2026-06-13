# charge-mgt-cloud OCPP 协议接口详细设计

> **目标 Crate**: `charge-mgt-cloud`
> **协议版本**: OCPP 1.6（与 `ocpp-1-6` crate 对齐）
> **上游文档**: [api_interfaces.md](./api_interfaces.md) 第 3、4 节的细化设计
> **DB Schema**: [db_schema.md](./db_schema.md)（11 张 OCPP 表）
> **版本**: v1.0（2026-06-13）

---

## 一、总体架构

### 1.1 数据流全景

```
                      ┌───────────────────────────────────────────┐
                      │           charge-mgt-cloud (CSMS)          │
                      └───────────────────────────────────────────┘
                                   ▲                    │
                                   │                    │
                      Kafka req.*  │                    │ Kafka resp.{gateway_id}
                      (per vendor) │                    ▼
                                   │
     ┌──────────────┐              │              ┌────────────────────┐
     │   Gateway    │              │              │   Gateway          │
     │   (Consumer) │◄─────────────┘              │   (Producer)       │
     └──────────────┘                              └────────────────────┘
                                   ▲                    │
                                   │                    ▼
                              ChargePoint            ChargePoint

      ┌──────────────────────────────────────────────────────────────────────┐
      │                        Cloud 内部消息流                             │
      │                                                                      │
      │  Kafka Consumer                                                      │
      │     │                                                                │
      │     ▼                                                                │
      │  [Message Dispatcher]  ──→  [sent_messages 去重]  ──→  [Action Handler]
      │                                                             │        │
      │  ┌──────────────────────┬───────────────────────┬───────────┤        │
      │  ▼                      ▼                       ▼           ▼        │
      │  BootNotification    Authorize              Start/Stop   MeterValues │
      │  Heartbeat           StatusNotification     ...                      │
      │                                                                      │
      │  Action Handler 处理完 → 构造 CloudMessage (CallResult/CallError)    │
      │                     → 写业务表                                       │
      │                     → Kafka Producer 发到 resp.{gateway_id}          │
      └──────────────────────────────────────────────────────────────────────┘
```

### 1.2 CloudMessage 信封格式

与 Gateway 定义的 [`CloudMessage`](file:///Users/molinyi/Documents/GitHub/charge-mgt/crates/charge-mgt-gateway/src/cloud/message.rs) 完全一致：

```rust
// 已存在于 ocpp-1-6 生态中，复用即可
pub struct CloudMessage {
    pub gateway_id: String,
    pub gateway_ip: String,
    pub vendor: String,
    pub charge_point_id: String,
    pub protocol: String,          // "OCPP-1.6"
    pub message_type: String,      // "Call" | "CallResult" | "CallError"
    pub action: String,            // "BootNotification" 等
    pub unique_id: String,
    pub payload: Value,
    pub received_at: DateTime<Utc>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
}
```

### 1.3 Action 路由策略

```rust
// 伪代码
match (message_type, action) {
    ("Call", "BootNotification")           => BootNotificationHandler,
    ("Call", "Heartbeat")                  => HeartbeatHandler,
    ("Call", "Authorize")                  => AuthorizeHandler,
    ("Call", "StartTransaction")           => StartTransactionHandler,
    ("Call", "StopTransaction")            => StopTransactionHandler,
    ("Call", "MeterValues")                => MeterValuesHandler,
    ("Call", "StatusNotification")         => StatusNotificationHandler,
    ("Call", "FirmwareStatusNotification") => FirmwareStatusHandler,
    ("Call", "DiagnosticsStatusNotification") => DiagnosticsStatusHandler,
    ("Call", unsupported_action)           => CallError::NotSupported,
    ("CallResult", _)                      => PendingCommandsResponseMatcher,
    ("CallError", _)                       => PendingCommandsResponseMatcher,
    (unknown, _)                           => DlqEntry + log
}
```

---

## 二、Message Dispatcher（核心编排器）

### 2.1 接口定义

```rust
pub trait MessageDispatcher {
    /// 处理一条 Kafka 入站消息（幂等安全）
    async fn dispatch(&self, msg: CloudMessage) -> Result<(), DispatchError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("unparseable envelope: {0}")]
    Unparseable(String),

    #[error("unknown action: {0}")]
    UnknownAction(String),

    #[error("handler failure: {0}")]
    HandlerFailure(Box<dyn std::error::Error + Send + Sync>),

    #[error("response send failure: {0}")]
    ResponseSendFailure(String),

    #[error("storage failure: {0}")]
    StorageFailure(String),
}
```

### 2.2 调度流程

```
输入: raw kafka bytes

Step 1: 反序列化到 CloudMessage
    └─ 失败 → DLQ + 结束

Step 2: Idempotency 检查 (插入 sent_messages)
    └─ 已存在 → 直接丢弃（幂等保护）
    └─ 成功 → 继续

Step 3: 路由到 Handler
    └─ 未知 action → DLQ + 结束

Step 4: Handler 处理
    └─ 业务错误 → Handler 自己决定返回 CallError（业务可恢复）
    └─ 严重错误 → 抛 HandlerFailure，dispatcher 捕获，回滚 sent_messages

Step 5: 构造 Response (CallResult / CallError)
    └─ 如果 Call 类型的请求需要响应（绝大部分都要）

Step 6: Kafka Producer 发送到 resp.{gateway_id}

Step 7: 更新 sent_messages.processed_at

完成。
```

### 2.3 幂等性保证

```sql
-- sent_messages_sent 表，unique_id 作 PK
INSERT INTO charge_mgt_sent_messages_ocpp_1_6 (unique_id, ...)
VALUES ($1, ...)
ON CONFLICT (unique_id) DO NOTHING;

-- 插入返回 0 行表示已存在，跳过处理
```

### 2.4 错误处理分级

| 错误 | 严重度 | 处理方式 |
|---|---|---|
| envelope 反序列化失败 | Critical | DLQ |
| 未知 action | High | DLQ |
| 数据库写入失败 | High | 重试 3 次；仍失败则 DLQ 并回滚 sent_messages |
| Handler 业务错误 | Medium | 返回 CallError 给 CP |
| Kafka 发送失败 | Medium | 重试 3 次；仍失败则记日志（CP 可重试） |
| 响应超时（> 10s） | Low | 日志告警（CP 自己会超时） |

---

## 三、OCPP Handler 详细设计（入站消息）

### 3.1 BootNotification Handler

**触发条件**：
- 充电桩首次上线
- 充电桩重启后

#### 3.1.1 请求 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `chargePointVendor` | string | ✓ | 厂商（≤ 20 字符） |
| `chargePointModel` | string | ✓ | 型号（≤ 20 字符） |
| `chargeBoxSerialNumber` | string? | — | 设备序列号（≤ 25 字符） |
| `chargePointSerialNumber` | string? | — | 充电桩序列号 |
| `firmwareVersion` | string? | — | 固件版本 |
| `iccid` | string? | — | SIM 卡 ICCID |
| `imsi` | string? | — | SIM 卡 IMSI |
| `meterType` | string? | — | 电表类型 |
| `meterSerialNumber` | string? | — | 电表序列号 |

#### 3.1.2 响应 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `status` | string (Accepted/Pending/Rejected) | ✓ | |
| `currentTime` | string (RFC3339) | ✓ | CSMS 当前 UTC 时间 |
| `interval` | integer | ✓ | 心跳间隔（秒），建议 30-3600 |

#### 3.1.3 业务规则

```
1. 校验:
   - chargePointSerialNumber 必填（作为 charge_point_id）
   - vendor/model 必填
   - vendor/model 长度 ≤ 20（OCPP 规范）

2. 注册/更新 charge_points_ocpp_1_6:
   INSERT INTO charge_points_ocpp_1_6 (
       id,                    -- chargePointSerialNumber
       gateway_id,            -- CloudMessage.gateway_id
       vendor,                -- chargePointVendor
       model,                 -- chargePointModel
       serial_number,         -- (重复记录便于查询)
       charge_box_serial,     -- chargeBoxSerialNumber
       firmware_version,
       iccid, imsi, meter_type, meter_serial_number,
       protocol_version,      -- 默认 'OCPP-1.6'
       ocpp_status,           -- 默认 'Online'（首次上线）
       heartbeat_interval_secs,-- 默认 30
       last_boot_at,          -- now
       last_heartbeat_at,     -- now
       registered_at,
       updated_at
   )
   ON CONFLICT (id) DO UPDATE SET
       firmware_version = EXCLUDED.firmware_version,
       ocpp_status = 'Online',
       last_boot_at = now(),
       last_heartbeat_at = now(),
       updated_at = now();

3. 同时为每个 connector 初始化状态:
   INSERT INTO connectors_ocpp_1_6 (charge_point_id, connector_id, status, ...)
   VALUES ($id, 0, 'Available'), ($id, 1, 'Available'), ...
   ON CONFLICT DO NOTHING;

4. 写 audit_logs（register/re-update 事件）
```

#### 3.1.4 Idempotency

- `unique_id` 已作 `sent_messages` PK，重复请求自动丢弃
- 业务侧用 `charge_point_id` 唯一约束保护幂等

#### 3.1.5 错误情况

| 错误 | CallError |
|---|---|
| 缺少 chargePointSerialNumber | `FormationViolation` + "missing charge_point_id" |
| vendor/model 超长 | `FormationViolation` + "vendor/model too long" |
| DB 失败 | `InternalError` + 日志 |

#### 3.1.6 测试用例

| 用例 | 期望 |
|---|---|
| 全新 CP 首次 boot | INSERT 成功，返回 Accepted |
| 同一 CP 重启 boot | UPDATE 成功，firmware 更新，返回 Accepted |
| 缺少 chargePointSerialNumber | 返回 FormationViolation |
| DB 写入失败 | DLQ + 重试 |

---

### 3.2 Heartbeat Handler

#### 3.2.1 请求 Schema

```json
{}  // 空
```

#### 3.2.2 响应 Schema

| 字段 | 类型 | 必填 |
|---|---|---|
| `currentTime` | string (RFC3339) | ✓ |

#### 3.2.3 业务规则

```sql
UPDATE charge_points_ocpp_1_6
SET last_heartbeat_at = now(),
    ocpp_status = 'Online',
    updated_at = now()
WHERE id = $charge_point_id;
```

#### 3.2.4 Idempotency

- 高频消息（每桩每 30s 一条）；用 `sent_messages` 的 `unique_id` 自动去重
- DB UPDATE 单条，幂等自然成立

#### 3.2.5 错误情况

| 错误 | CallError |
|---|---|
| cp 未在 charge_points 表中（理论上不可能，但容错） | `InternalError` |
| DB 失败 | `InternalError` + DLQ |

#### 3.2.6 性能考虑

- **热路径**：最频繁的消息
- 建议：使用批量 UPDATE（按时间窗口合并）或直接写时间戳到 Redis（P1 优化）

---

### 3.3 Authorize Handler

**触发条件**：
- 用户刷卡/扫码
- 充电前鉴权验证

#### 3.3.1 请求 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `idTag` | string | ✓ | 用户标识（卡号/手机号/UUID） |

#### 3.3.2 响应 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `idTagInfo.status` | string | ✓ | Accepted/Blocked/Expired/Invalid/ConcurrentTx |
| `idTagInfo.expiryDate` | string (RFC3339)? | — | 过期时间 |
| `idTagInfo.parentIdTag` | string? | — | 父卡标识 |

#### 3.3.3 业务规则

```
1. 查询 id_tags_ocpp_1_6:
   SELECT status, expiry_date, parent_id_tag, is_deleted
   FROM id_tags_ocpp_1_6
   WHERE id_tag = $id_tag;

2. 状态映射:
   - 不存在 OR is_deleted = true        → Invalid
   - status = 'Blocked'                 → Blocked
   - status = 'Expired'
     AND expiry_date < now()            → Expired
   - status = 'Accepted'
     AND expiry_date IS NOT NULL
     AND expiry_date < now()            → Expired
   - 检查是否存在并发 Active 交易:
     SELECT COUNT(*) FROM transactions_ocpp_1_6
     WHERE id_tag = $id_tag AND state = 'Active'
     AND charge_point_id != $current_cp_id
     -- 如果有,返回 ConcurrentTx
   - 否则                               → Accepted

3. 构造响应:
   {
     "idTagInfo": {
       "status": "<状态>",
       "expiryDate": "<DB 中的 expiry_date 或 null>",
       "parentIdTag": "<DB 中的 parent_id_tag 或 null>"
     }
   }
```

#### 3.3.4 Idempotency

- 纯查询 + 响应构造；`unique_id` 自动幂等
- 不修改任何表

#### 3.3.5 错误情况

| 错误 | CallError |
|---|---|
| idTag 缺失/为空 | `FormationViolation` |
| DB 查询失败 | `InternalError` |

#### 3.3.6 缓存优化（P1）

- 高频查询（每次刷卡）；建议引入本地 LRU 缓存（5 分钟）
- 缓存失效：idTag 更新/删除时广播失效

---

### 3.4 StartTransaction Handler

**触发条件**：
- 桩开始充电时上报

#### 3.4.1 请求 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `connectorId` | integer | ✓ | 连接器 ID |
| `idTag` | string | ✓ | 启动者标识 |
| `meterStart` | integer | ✓ | 开始读数 (Wh) |
| `reservationId` | integer? | — | 关联的预约 ID |
| `timestamp` | string (RFC3339) | ✓ | 开始时间 |

#### 3.4.2 响应 Schema

| 字段 | 类型 | 必填 |
|---|---|---|
| `transactionId` | integer | ✓ |
| `idTagInfo.status` | string | ✓ |
| `idTagInfo.expiryDate` | string? | — |
| `idTagInfo.parentIdTag` | string? | — |

#### 3.4.3 业务规则

```
1. 校验:
   - connectorId 必须存在
   - idTag 必须有效（调用 AuthorizeService.authorize）
     - 如果被 Blocked/Expired/Invalid → 直接返回 CallError 或 CallResult(status=Invalid)
   - meterStart 必须非负
   - timestamp 必须合法（过去时间或未来 5 分钟内）

2. 检查同 connector 是否有其他 Active 交易:
   SELECT id FROM transactions_ocpp_1_6
   WHERE charge_point_id = $cp_id
     AND connector_id = $connector_id
     AND state = 'Active';
   如果存在 → 强制关闭旧交易（标记为 "Faulted"），再继续创建

3. 插入 transactions_ocpp_1_6:
   INSERT INTO transactions_ocpp_1_6 (
       id,                        -- BIGSERIAL 自增
       charge_point_id,
       connector_id,
       id_tag,
       meter_start_wh,
       start_timestamp,
       started_at,                -- CSMS 收到时间
       state = 'Active',
       reservation_id
   )
   RETURNING id INTO $new_tx_id;

4. 如果有 reservationId，更新 reservations 状态:
   UPDATE reservations_ocpp_1_6
   SET state = 'Used'
   WHERE id = $reservationId
     AND id_tag = $id_tag
     AND charge_point_id = $cp_id
     AND state = 'Waiting';
   -- 失败不阻断，仅记录日志

5. 更新相应 connector:
   UPDATE connectors_ocpp_1_6
   SET status = 'Charging',
       last_status_at = now()
   WHERE charge_point_id = $cp_id
     AND connector_id = $connector_id;

6. 再次调用 AuthorizeService 确认 idTag 可用，构造响应:
   {
     "transactionId": $new_tx_id,
     "idTagInfo": {
       "status": "Accepted",     // 或 Block/Expired/Invalid 状态
       "expiryDate": "...",
       "parentIdTag": "..."
     }
   }
```

#### 3.4.4 Idempotency

- 关键场景：同一笔充电可能被多次上报（CP 重试）
- **策略**：基于 `(charge_point_id, connector_id, meter_start, start_timestamp)` 组合判断是否重复
- 如果重复 → 返回已有 `transactionId`

#### 3.4.5 错误情况

| 错误 | CallError |
|---|---|
| connectorId 不存在 | `FormationViolation` |
| idTag 无效 | 返回 CallResult 但 status=Invalid |
| meterStart 负数 | `FormationViolation` |
| DB 失败 | `InternalError` |

#### 3.4.6 测试用例

| 用例 | 期望 |
|---|---|
| 正常启动 | 新建 Active 交易 |
| 同 connector 已有 Active 交易 | 关闭旧交易 + 创建新交易 |
| idTag 被 Blocked | 返回 idTagInfo.status=Blocked + 不创建交易 |
| 重复上报（同 unique_id）| 返回原 transactionId |
| reservationId 有效 | reservation 状态改 Used |
| DB 失败 | DLQ + 重试 |

---

### 3.5 StopTransaction Handler

**触发条件**：
- 用户拔枪 / 充电结束 / 运营中断

#### 3.5.1 请求 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `meterStop` | integer | ✓ | 结束读数 (Wh) |
| `timestamp` | string (RFC3339) | ✓ | 结束时间 |
| `transactionId` | integer | ✓ | 关联的交易 ID |
| `reason` | string? | — | 见枚举值 |
| `idTag` | string? | — | 结束者（如远程停止时填运营账号） |
| `transactionData` | MeterValue[]? | — | 结束时的详细读数（OCPP 1.6 推荐） |

**reason 枚举值**：
- `EmergencyStop`, `EVDisconnected`, `HardReset`, `Local`, `Other`, `PowerLoss`, `Reboot`, `Remote`, `SoftReset`, `StormReset`, `TillDisconnection`

#### 3.5.2 响应 Schema

| 字段 | 类型 | 必填 |
|---|---|---|
| `idTagInfo.status` | string | ✓ |
| `idTagInfo.expiryDate` | string? | — |
| `idTagInfo.parentIdTag` | string? | — |

#### 3.5.3 业务规则

```
1. 校验:
   - transactionId 必须对应一条 state = 'Active' 的交易
   - meterStop >= meter_start_wh（否则数据异常）
   - timestamp 必须晚于 start_timestamp

2. 如果找不到对应的 Active 交易:
   - 检查是否存在 Completed/Aborted 的同 ID 交易（幂等保护）
   - 如果存在 → 直接返回原响应
   - 如果都不存在 → 创建新的 Completed 交易（容错）

3. 更新 transactions_ocpp_1_6:
   UPDATE transactions_ocpp_1_6
   SET state = 'Completed',
       meter_stop_wh = $meter_stop,
       stop_timestamp = $timestamp,
       stop_reason = $reason,
       closed_at = now(),
       raw_stop_payload = $whole_payload,
       total_kwh = ($meter_stop - meter_start_wh) / 1000.0
   WHERE id = $transaction_id
     AND state = 'Active';

4. 如果 transaction_data 非空，批量插入 meter_values_ocpp_1_6

5. 调用 BillingService 计费:
   - 查询交易时长
   - 查询适用 tariff
   - 计算费用 → 存入 bill 关联

6. 更新 connector 状态:
   UPDATE connectors_ocpp_1_6
   SET status = 'Finishing',
       last_status_at = now()
   WHERE charge_point_id = $cp_id
     AND connector_id = $connector_id;

7. 返回 idTagInfo

8. 触发异步事件：TransactionCompleted
```

#### 3.5.4 Idempotency

- 关键场景：同一笔交易可能被多次 Stop 上报（CP 重试）
- **策略**：基于 `transaction_id` 唯一性
  - 若 `state = 'Active'`：执行更新
  - 若 `state = 'Completed'`：幂等返回原响应内容

#### 3.5.5 错误情况

| 错误 | CallError |
|---|---|
| transaction_id 不存在 | `PropertyConstraintViolation` |
| meterStop < meterStart | `PropertyConstraintViolation` |
| DB 失败 | `InternalError` + DLQ |

#### 3.5.6 测试用例

| 用例 | 期望 |
|---|---|
| 正常结束 | 完成交易，计费，connector 状态 Finishing |
| 重复 Stop（同 unique_id）| 返回原响应，幂等保护 |
| Stop 已完成的交易 | 幂等返回原响应 |
| 找不到交易 | 容错创建 |
| meterStop < meterStart | PropertyConstraintViolation |

---

### 3.6 MeterValues Handler

#### 3.6.1 请求 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `connectorId` | integer | ✓ | 连接器 ID |
| `transactionId` | integer? | — | 关联的交易 |
| `meterValue` | MeterValue[] | ✓ | 采样值数组 |

**MeterValue 结构**：
```json
{
  "timestamp": "RFC3339",
  "sampledValue": [
    {
      "value": "1234.56",     // string 形式
      "context": "Sample.Periodic",
      "format": "Raw",
      "measurand": "Energy.Active.Import.Register",
      "unit": "Wh",
      "location": "Outlet",
      "phase": "L1"
    }
  ]
}
```

**measurand 枚举**（15 个值，OCPP 1.6 PascalCase）：
- `EnergyActiveImportRegister`（最常用）
- `EnergyActiveExportRegister`, `EnergyReactiveImportRegister`, `EnergyReactiveExportRegister`
- `PowerActiveImport`, `PowerActiveExport`
- `PowerReactiveImport`, `PowerReactiveExport`
- `PowerFactor`
- `CurrentImport`, `CurrentExport`
- `Voltage`
- `Temperature`
- `SoC`（SoC = State of Charge）
- `Frequency`

**context 枚举**（8 个值）：
- `InterruptionBegin`, `InterruptionEnd`
- `Other`
- `SampleClock`, `SamplePeriodic`
- `TransactionBegin`, `TransactionEnd`
- `Trigger`

#### 3.6.2 响应 Schema

```json
{}  // 空响应
```

#### 3.6.3 业务规则

```
1. 校验:
   - connectorId 必须存在
   - meterValue 数组至少有一条
   - 每条 meterValue 必须有 timestamp + 至少一条 sampledValue
   - 每条 sampledValue 中的 value 必须是数值字符串

2. 批量插入 meter_values_ocpp_1_6:
   -- 按 measurand 和 value 拆分
   FOR EACH mv IN meterValue:
     FOR EACH sv IN mv.sampledValue:
       INSERT INTO meter_values_ocpp_1_6 (
           transaction_id,
           connector_id,
           sample_timestamp,
           measurand,           -- charge_mgt_measurand ENUM
           unit,
           value_int,           -- 取整数部分（Wh 精度）
           context,             -- charge_mgt_reading_context ENUM
           format,
           received_at,
           raw_sampled_value    -- 完整原始 SV 用于审计
       );

3. value 数值解析:
   - 如果 unit = "Wh" (或 unit=NULL 默认 Wh) → 直接用
   - 如果 unit = "kWh" / "kWh" (小写兼容) → × 1000
   - 其他单位 → 记录日志，保留原始值

4. 高频写入 → 使用批量 INSERT（100+ 行）
```

#### 3.6.4 Idempotency

- 同一 unique_id → 自动跳过
- 同一 (transaction_id, sample_timestamp, measurand) 组合 → 业务层去重（P1）

#### 3.6.5 错误情况

| 错误 | CallError |
|---|---|
| connectorId 不存在 | `PropertyConstraintViolation` |
| meterValue 为空 | `FormationViolation` |
| value 非数值 | `FormationViolation` |
| DB 失败 | `InternalError` + DLQ |

#### 3.6.6 性能考虑

- **高频写入**：单桩每 5-15 秒 1 条
- **建议**：
  - 批量 INSERT + prepared statements
  - P1：按 transaction_id 分区（按月/按周）
  - P2：写入 ClickHouse / Timescale 做时序分析

---

### 3.7 StatusNotification Handler

**触发条件**：
- Connector 状态变化时
- 定时上报（可选）

#### 3.7.1 请求 Schema

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `connectorId` | integer | ✓ | 连接器 ID |
| `errorCode` | string | ✓ | ChargePointErrorCode 枚举 |
| `status` | string | ✓ | ChargePointStatus 枚举 |
| `timestamp` | string (RFC3339) | ✓ | |
| `info` | string? | — | 自由文本（≤ 50 字符） |
| `vendorId` | string? | — | 厂商标识 |
| `vendorErrorCode` | string? | — | 厂商自定义错误码 |

**ChargePointStatus 枚举**（9 个值）：
- `Available`, `Preparing`, `Charging`, `SuspendedEVSE`, `SuspendedEV`, `Finishing`, `Reserved`, `Unavailable`, `Faulted`

**ChargePointErrorCode 枚举**（16+ 值，见 OCPP 1.6 规范）：
- `NoError`, `ConnectorLockFailure`, `EVCommunicationFailure`, `GroundFailure`, `HighTemperature`, `InternalError`, `LocalListConflict`, `Mode3Error`, `OtherError`, `OverCurrentFailure`, `OverVoltage`, `PowerMeterFailure`, `PowerSwitchFailure`, `ReaderFailure`, `ResetFailure`, `UnderVoltage`, `WeakSignal`

#### 3.7.2 响应 Schema

```json
{}
```

#### 3.7.3 业务规则

```sql
UPDATE connectors_ocpp_1_6
SET status = $status,
    error_code = $error_code,
    info = $info,
    vendor_id = $vendor_id,
    vendor_error_code = $vendor_error_code,
    last_status_at = $timestamp
WHERE charge_point_id = $cp_id
  AND connector_id = $connector_id;

-- 如果 connector 不存在，INSERT
INSERT INTO connectors_ocpp_1_6 (
    charge_point_id, connector_id, status, error_code, ...
)
VALUES (...)
ON CONFLICT (charge_point_id, connector_id) DO UPDATE SET
    status = EXCLUDED.status,
    ... ;

-- 同步更新 charge_points 整体状态:
-- 如果任一 connector = Faulted,charge_point 状态 = Faulted
-- 否则如果全部 Available,charge_point = Available
-- 否则 charge_point = Charging
UPDATE charge_points_ocpp_1_6
SET ocpp_status = CASE
      WHEN EXISTS(SELECT 1 FROM connectors_ocpp_1_6 WHERE charge_point_id = $cp_id AND status = 'Faulted') THEN 'Faulted'
      WHEN NOT EXISTS(SELECT 1 FROM connectors_ocpp_1_6 WHERE charge_point_id = $cp_id AND status != 'Available') THEN 'Available'
      ELSE 'Online'
    END,
    updated_at = now()
WHERE id = $cp_id;
```

#### 3.7.4 错误情况

| 错误 | CallError |
|---|---|
| status 不在枚举范围 | `PropertyConstraintViolation` |
| errorCode 不在枚举范围 | `PropertyConstraintViolation` |
| DB 失败 | `InternalError` |

---

### 3.8 FirmwareStatusNotification Handler

#### 3.8.1 请求 Schema

| 字段 | 类型 | 必填 |
|---|---|---|
| `status` | string (11 个枚举) | ✓ |

**status 枚举**：
- `Downloaded`, `DownloadFailed`, `DownloadScheduled`, `Downloading`, `Idle`, `InstallationFailed`, `Installed`, `InstallRebooting`, `InstallScheduled`, `InstallVerificationFailed`, `InvalidSignature`, `SignatureVerified`

#### 3.8.2 业务规则

```sql
-- 查找该 cp 最近 UpdateFirmware 任务的 firmware
-- 更新 firmware_ocpp_1_6 的 status
UPDATE firmware_ocpp_1_6 f
SET deployment_status = $status,
    last_status_at = now()
FROM (
    SELECT firmware_id
    FROM pending_commands_ocpp_1_6
    WHERE charge_point_id = $cp_id
      AND action = 'UpdateFirmware'
      AND state = 'Accepted'
    ORDER BY sent_at DESC LIMIT 1
) p
WHERE f.id = p.firmware_id;
```

#### 3.8.3 响应 Schema

```json
{}
```

---

### 3.9 DiagnosticsStatusNotification Handler

#### 3.9.1 请求 Schema

| 字段 | 类型 | 必填 |
|---|---|---|
| `status` | string (4 个枚举) | ✓ |

**status 枚举**：
- `Idle`, `Uploaded`, `UploadFailed`, `Uploading`

#### 3.9.2 业务规则

- 简单日志记录到 `charge_mgt_message_log_ocpp_1_6`
- 可选：更新 pending_commands 对应条目的状态

#### 3.9.3 响应 Schema

```json
{}
```

---

## 四、OCPP Command Dispatcher（出站命令）

### 4.1 总体架构

```
  用户触发 HTTP POST /api/remote/:action
       │
       ▼
  RemoteCommandService.dispatch
       │
       ├── 校验参数
       ├── 查询目标 cp_id 是否存在且 Online
       ├── 生成 unique_id
       ├── 插入 pending_commands_ocpp_1_6
       │   INSERT INTO pending_commands_ocpp_1_6
       │   (unique_id, charge_point_id, action, request_payload,
       │    state = 'Pending', sent_at = now(), expires_at = now() + timeout)
       │
       ├── 触发异步 Dispatcher（不阻塞 HTTP）
       │
       └── 返回 202 Accepted + unique_id

  异步 Dispatcher:
       │
       ├── 读 pending_commands (state = 'Pending', sent_at < now())
       ├── 构造 CloudMessage:
       │   - message_type: "Call"
       │   - action: "RemoteStartTransaction" (e.g.)
       │   - payload: 用户传入
       │   - unique_id: pending_commands.unique_id
       ├── Kafka Producer 发到 resp.{gateway_id}
       └── 更新 pending_commands.sent_at / 标记 sent

  等待 CP 响应（异步）:
       │
       ▼
  Consumer 收到 CallResult/CallError (message_type != "Call"):
       │
       ├── 通过 unique_id 查 pending_commands
       ├── 如果找到:
       │   ├── 如果成功 → state = 'Accepted', response_payload = 响应
       │   ├── 如果失败 → state = 'Rejected', error_code, error_description
       │   └── 状态转换必须满足 Pending → Accepted/Rejected
       └── 如果没找到 → 写 DLQ (可能 CP 响应太晚)

  定时扫描（PendingCommandsTimeoutScanner 每 30s）:
       │
       └── UPDATE pending_commands_ocpp_1_6
           SET state = 'Timeout', responded_at = now()
           WHERE state = 'Pending'
             AND expires_at < now();
```

### 4.2 命令清单（12 类）

#### 4.2.1 RemoteStartTransaction

```rust
// 请求 payload
pub struct RemoteStartTransactionReq {
    pub connector_id: Option<i32>,     // NULL 表示任意 connector
    pub id_tag: String,                 // 必填
    pub charging_profile: Option<ChargingProfile>,  // P2
}

// 期望响应 payload
pub struct RemoteStartTransactionConf {
    pub status: AcceptRejectStatus,  // "Accepted" | "Rejected"
}

// 验证规则
// - idTag 必须有效
// - connector_id 如果存在，必须存在且状态为 Available
// - 必须存在 idTag
```

#### 4.2.2 RemoteStopTransaction

```rust
pub struct RemoteStopTransactionReq {
    pub transaction_id: i32,    // 必填
}

pub struct RemoteStopTransactionConf {
    pub status: AcceptRejectStatus,
}

// 验证: transaction_id 必须对应 Active 交易
```

#### 4.2.3 Reset

```rust
pub struct ResetReq {
    pub reset_type: ResetType,  // "Hard" | "Soft"
}

pub struct ResetConf {
    pub status: ResetStatus,    // "Accepted" | "Rejected"
}

// 副作用: Reset 后 CP 重新 BootNotification，charge_points 状态重置
```

#### 4.2.4 UnlockConnector

```rust
pub struct UnlockConnectorReq {
    pub connector_id: i32,
}

pub struct UnlockConnectorConf {
    pub status: UnlockStatus,   // "Unlocked" | "UnlockFailed" | "NotSupported"
}
```

#### 4.2.5 ChangeAvailability

```rust
pub struct ChangeAvailabilityReq {
    pub connector_id: i32,      // 0 = 整个 CP
    pub availability_type: AvailabilityType,  // "Operative" | "Inoperative"
}

pub struct ChangeAvailabilityConf {
    pub status: AvailabilityStatus,  // "Accepted" | "Rejected" | "Scheduled"
}

// 副作用: connector 状态可能变为 Reserved/Unavailable
```

#### 4.2.6 ChangeConfiguration

```rust
pub struct ChangeConfigurationReq {
    pub key: String,            // OCPP Configuration Key
    pub value: String,
}

pub struct ChangeConfigurationConf {
    pub status: ConfigurationStatus,  // "Accepted" | "Rejected" | "RebootRequired" | "NotSupported"
}

// 如果 RebootRequired，可自动发起 Reset
```

#### 4.2.7 GetConfiguration

```rust
pub struct GetConfigurationReq {
    pub key: Option<Vec<String>>,  // NULL = 全部
}

pub struct GetConfigurationConf {
    pub configuration_key: Vec<ConfigurationKey>,
    pub unknown_key: Vec<String>,
}

pub struct ConfigurationKey {
    pub key: String,
    pub value: Option<String>,
    pub readonly: bool,
}

// 副作用: 更新 config_keys_ocpp_1_6 表同步
```

#### 4.2.8 ClearCache

```rust
pub struct ClearCacheReq {}

pub struct ClearCacheConf {
    pub status: ClearCacheStatus,  // "Accepted" | "Rejected"
}

// 清理 CP 端的本地授权列表缓存
```

#### 4.2.9 GetDiagnostics

```rust
pub struct GetDiagnosticsReq {
    pub location: String,       // 上传目标 URL
    pub retries: Option<i32>,
    pub retry_interval: Option<i32>,
    pub start_time: Option<String>,
    pub stop_time: Option<String>,
}

pub struct GetDiagnosticsConf {
    pub file_name: Option<String>,
}
```

#### 4.2.10 UpdateFirmware

```rust
pub struct UpdateFirmwareReq {
    pub location: String,
    pub retrieve_date: String,
    pub retries: Option<i32>,
    pub retry_interval: Option<i32>,
}

pub struct UpdateFirmwareConf {}  // 空响应

// 副作用: 同步写入 firmware_ocpp_1_6 表 deployment_status
```

#### 4.2.11 TriggerMessage

```rust
pub struct TriggerMessageReq {
    pub requested_message: MessageTrigger,
    pub connector_id: Option<i32>,
}

// MessageTrigger 枚举 (OCPP 1.6 9 种):
// "BootNotification", "DiagnosticsStatusNotification",
// "FirmwareStatusNotification", "Heartbeat", "MeterValues",
// "StatusNotification", "SignCertificateV2" (2.0.1 only),
// "TransactionEventV2" (2.0.1 only), "MeterValuesV2" (2.0.1 only)

pub struct TriggerMessageConf {
    pub status: TriggerMessageStatus,  // "Accepted" | "Rejected" | "NotImplemented"
}
```

#### 4.2.12 SendLocalList

```rust
pub struct SendLocalListReq {
    pub list_version: i32,
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
    pub update_type: UpdateType,  // "Full" | "Differential"
}

pub struct AuthorizationData {
    pub id_tag: String,
    pub id_tag_info: Option<IdTagInfo>,
}

pub struct SendLocalListConf {
    pub status: UpdateStatus,  // "Accepted" | "Failed" | "NotSupported" | "VersionMismatch"
    #[serde(skip_serializing_if="Option::is_none")]
    pub hash: Option<String>,
}
```

### 4.3 Outbox Dispatcher 详细设计

#### 4.3.1 Dispatcher Trait

```rust
#[async_trait]
pub trait OutboxDispatcher {
    /// 处理所有待发送的命令
    async fn dispatch_pending(&self) -> Result<usize, DispatchError>;

    /// 单独发送一条命令
    async fn dispatch_one(&self, unique_id: &str) -> Result<(), DispatchError>;
}
```

#### 4.3.2 Dispatcher 任务

```rust
// 启动为单独的 tokio task
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;
        if let Err(e) = dispatcher.dispatch_pending().await {
            tracing::error!("outbox dispatch failed: {e}");
        }
    }
});
```

#### 4.3.3 重试策略

| 重试次数 | 等待 | 状态 |
|---|---|---|
| 第 1 次 | 立即 | Pending |
| 第 2 次 | 1 秒 | Pending |
| 第 3 次 | 3 秒 | Pending |
| 失败 | — | 标记 Failed（新增状态，P1） |
| expires_at 超时 | — | Timeout |

### 4.4 Response Matcher

```rust
// 当 Consumer 收到非 "Call" 消息时
pub struct ResponseMatcher;

impl ResponseMatcher {
    pub async fn match_and_update(
        &self,
        unique_id: &str,
        result: CloudMessage,
    ) -> Result<(), MatchError> {
        // 1. 查 pending_commands (unique_id)
        let cmd = self.find_pending(unique_id).await?;

        if cmd.is_none() {
            return Err(MatchError::OrphanResponse);  // DLQ
        }

        let cmd = cmd.unwrap();

        // 2. 更新 state
        let new_state = match result.message_type.as_str() {
            "CallResult" => PendingCommandState::Accepted,
            "CallError" => PendingCommandState::Rejected,
            _ => unreachable!(),
        };

        self.db.update_pending_command(
            unique_id,
            new_state,
            result.payload.clone(),
            result.error_code.clone(),
            result.error_description.clone(),
        ).await?;

        Ok(())
    }
}
```

---

## 五、错误码分类

### 5.1 OCPP 标准 CallError Code

| Code | 含义 | 何时使用 |
|---|---|---|
| `NotImplemented` | 不支持的操作 | 收到未定义的 action |
| `NotSupported` | 支持但不接受 | 操作被业务拒绝 |
| `InternalError` | CSMS 内部错误 | DB 故障等 |
| `ProtocolError` | 协议错误 | 极少用 |
| `SecurityError` | 鉴权失败 | 网关级别鉴权 |
| `FormationViolation` | 消息格式错误 | 缺字段、类型不对 |
| `PropertyConstraintViolation` | 属性约束违反 | 值超出范围、枚举不符 |
| `OccurenceConstraintViolation` | 数量约束违反 | 数组元素数超标 |
| `TypeConstraintViolation` | 类型约束违反 | 字段类型不匹配 |
| `GenericTypeError` | 通用类型错误 | 不常用 |

### 5.2 错误码到 Handler

| Handler | 错误类型 | 推荐 CallError |
|---|---|---|
| BootNotification | 缺少字段 | FormationViolation |
| BootNotification | 长度超标 | PropertyConstraintViolation |
| BootNotification | DB 失败 | InternalError |
| Authorize | idTag 不存在 | (不报错，返回 Invalid 状态) |
| StartTransaction | connectorId 不存在 | PropertyConstraintViolation |
| StartTransaction | 重复 transaction | (幂等返回) |
| StopTransaction | transactionId 不存在 | PropertyConstraintViolation |
| MeterValues | 数值异常 | FormationViolation |
| 所有 | 未知 action | NotImplemented |

---

## 六、Kafka Topic 设计

### 6.1 Topic 命名

```
上行（CP → Cloud）:
  charge_mgt.req.{vendor}
  示例：charge_mgt.req.Alphas, charge_mgt.req.Beta, charge_mgt.req.SimVendor

下行响应（Cloud → Gateway → CP）:
  charge_mgt.resp.{gateway_id}
  示例：charge_mgt.resp.gateway-001, charge_mgt.resp.gateway-042

死信队列（DLQ）:
  charge_mgt.dlq.{vendor}
```

### 6.2 消息体格式

**Request** (CloudMessage):
```json
{
  "gateway_id": "gateway-001",
  "gateway_ip": "192.168.1.10",
  "vendor": "Alphas",
  "charge_point_id": "ALPHA-CP-042",
  "protocol": "OCPP-1.6",
  "message_type": "Call",
  "action": "BootNotification",
  "unique_id": "uuid-...",
  "payload": { "chargePointVendor": "Alphas", ... },
  "received_at": "2026-06-13T10:00:00Z",
  "error_code": null,
  "error_description": null
}
```

**Response** (CloudMessage):
```json
{
  "gateway_id": "gateway-001",
  "gateway_ip": "192.168.1.10",
  "vendor": "Alphas",
  "charge_point_id": "ALPHA-CP-042",
  "protocol": "OCPP-1.6",
  "message_type": "CallResult",
  "action": "BootNotification",
  "unique_id": "uuid-...",
  "payload": {
    "status": "Accepted",
    "currentTime": "2026-06-13T10:00:01Z",
    "interval": 30
  },
  "received_at": "2026-06-13T10:00:01Z"
}
```

**Error Response** (CloudMessage):
```json
{
  "gateway_id": "gateway-001",
  "gateway_ip": "192.168.1.10",
  "vendor": "Alphas",
  "charge_point_id": "ALPHA-CP-042",
  "protocol": "OCPP-1.6",
  "message_type": "CallError",
  "action": "BootNotification",
  "unique_id": "uuid-...",
  "payload": {},
  "error_code": "FormationViolation",
  "error_description": "missing chargePointSerialNumber"
}
```

### 6.3 Consumer Group 设计

```
Consumer Group: "cloud-csms-consumer"

消费行为:
  - 多个 Cloud 实例共享同一 consumer group（负载均衡）
  - 每个 vendor topic 对应一个 consumer task
  - 使用 auto.commit 便于运维

Rebalance 行为:
  - Cloud 实例增减时自动 rebalance
  - 单实例故障时，其消息分给其他实例
```

---

## 七、性能与可扩展性

### 7.1 写入热点分析

| 消息类型 | 频率（每桩） | 写入操作 | 瓶颈 |
|---|---|---|---|
| Heartbeat | 30s/次 | 1 UPDATE | 中（量大） |
| MeterValues | 5-15s/次 | N INSERT | **高** |
| StatusNotification | 低频 | 1 UPDATE | 低 |
| BootNotification | 重启时 | 1 INSERT/UPDATE | 低 |
| Authorize | 用户操作时 | 1 SELECT | 中（热点 ID） |
| Start/StopTransaction | 用户操作时 | 1 INSERT/UPDATE | 低 |

### 7.2 优化建议

#### P0 阶段
- MeterValues 使用 bulk INSERT（每次 10 行）
- Heartbeat UPDATE 用 prepared statement
- 使用 connection pool（sqlx PgPool，max=100）

#### P1 阶段
- Heartbeat 走 Redis（CP 心跳状态，Cloud 只定期落库）
- MeterValues 按 transaction_id 分表（按月或按周）
- 引入 ClickHouse / TimescaleDB 做时序查询

#### P2 阶段
- Kafka 消费水平扩展（Cloud 多实例消费不同 partition）
- MeterValues 写入异步化（先写内存队列，批量 flush）

### 7.3 监控指标

```
Kafka Consumer:
  charge_mgt_cloud_kafka_messages_processed_total{action="..."}
  charge_mgt_cloud_kafka_consumer_group_lag
  charge_mgt_cloud_kafka_rebalance_count

Handler:
  charge_mgt_cloud_ocpp_handler_duration_seconds{action="...", status="success|error"}
  charge_mgt_cloud_ocpp_handler_errors_total{action="...", error_type="..."}

DB:
  charge_mgt_cloud_db_query_duration_seconds{query="..."}
  charge_mgt_cloud_db_pool_connections

Command Dispatcher:
  charge_mgt_cloud_command_dispatched_total{action="..."}
  charge_mgt_cloud_command_response_time_seconds{action="..."}
  charge_mgt_cloud_pending_commands_timeout_total{action="..."}
```

---

## 八、测试策略

### 8.1 单元测试

| 层 | 测试内容 | 示例文件 |
|---|---|---|
| Handler | 业务规则 | `boot_notification_handler::tests` |
| Service | Rust 接口 | `charge_point_service::tests` |
| Dispatcher | 消息路由 | `message_dispatcher::tests` |
| Kafka Serde | 消息序列化 | `cloud_message::tests` |

### 8.2 集成测试

```
1. Testcontainers Kafka + PostgreSQL
2. 模拟 CP 发送各种 OCPP 消息
3. 验证：
   - DB 表更新正确
   - 响应构造正确
   - 幂等性保证
4. 验证 DLQ 路由

文件：tests/ocpp_integration.rs
```

### 8.3 端到端测试

```
1. 启动完整环境：Kafka + Postgres + Cloud + Gateway + Simulator
2. Simulator 走完整 Boot → Start → Meter → Stop 流程
3. 验证 Cloud 数据库全部正确
4. 验证运营 HTTP API 能查到正确数据
```

---

## 九、未决设计问题

| # | 问题 | 建议 | 待决策 |
|---|---|---|---|
| 1 | MeterValue 的 unit 处理：支持 "kWh" "Wh" 多种单位吗？ | ✅ 支持，业务层统一转 Wh | 待确认 |
| 2 | Heartbeat 走 Redis 优化：需要 Redis 吗？ | ✅ Cloud 已有 Redis 依赖（可选） | 待确认 |
| 3 | Connector 默认数量：BootNotification 时如何知道 CP 有几个 connector？ | 默认 1 个，按 StatusNotification 自增 | 待确认 |
| 4 | idTag 缓存策略：多长时间失效？ | 默认 5 分钟 LRU | 待确认 |
| 5 | 异常交易（Faulted）如何处理？ | 自动关闭 + 通知运营 | 待确认 |
| 6 | Reservation 是否强制要求？ | MCP 不强制，P2 支持 | 待确认 |
| 7 | 费率计算粒度：按分钟/小时/天？ | 按交易整体计算 + P2 阶梯 | 待确认 |
| 8 | DLQ 是否需要 Web UI？ | P1 必须有 | 待确认 |
| 9 | CSMS 主动命令 timeout 默认设置？ | 30 秒 | 待确认 |
| 10 | Kafka 消息顺序：需要保证同 CP 消息有序吗？ | 使用 partition key = cp_id + timestamp | 待确认 |

---

## 十、Rust 模块架构建议

```
crates/charge-mgt-cloud/src/
├── lib.rs
├── main.rs
├── app.rs
├── config.rs
├── state.rs
│
├── infra/                            # 基础设施层
│   ├── mod.rs
│   ├── kafka/
│   │   ├── mod.rs
│   │   ├── consumer.rs               # 消费 req.{vendor}
│   │   ├── producer.rs               # 发送 resp.{gateway_id}
│   │   └── dispatcher.rs             # Outbox Dispatcher
│   ├── db/
│   │   ├── mod.rs
│   │   ├── pool.rs                   # sqlx PgPool
│   │   └── migrations.rs             # 迁移管理
│   └── error.rs                      # InfraError
│
├── domain/                           # 业务领域层
│   ├── mod.rs
│   ├── charge_point.rs               # 实体 + Repository trait
│   ├── connector.rs
│   ├── id_tag.rs
│   ├── transaction.rs
│   ├── meter_value.rs
│   ├── pending_command.rs
│   ├── tariff.rs
│   ├── bill.rs
│   └── reservation.rs
│
├── service/                          # 业务服务层（实现 domain Repository trait）
│   ├── mod.rs
│   ├── charge_point_service.rs
│   ├── connector_service.rs
│   ├── id_tag_service.rs
│   ├── transaction_service.rs
│   ├── meter_value_service.rs
│   ├── remote_command_service.rs
│   ├── billing_service.rs
│   └── tariff_service.rs
│
├── ocpp/                             # OCPP 协议层
│   ├── mod.rs
│   ├── dispatcher.rs                 # MessageDispatcher trait
│   ├── router.rs                     # Action → Handler 路由
│   ├── envelope.rs                   # CloudMessage 解析
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── boot_notification.rs
│   │   ├── heartbeat.rs
│   │   ├── authorize.rs
│   │   ├── start_transaction.rs
│   │   ├── stop_transaction.rs
│   │   ├── meter_values.rs
│   │   ├── status_notification.rs
│   │   ├── firmware_status.rs
│   │   └── diagnostics_status.rs
│   ├── commands/                     # CSMS 主动命令
│   │   ├── mod.rs
│   │   ├── remote_start.rs
│   │   ├── remote_stop.rs
│   │   ├── reset.rs
│   │   ├── unlock_connector.rs
│   │   ├── change_availability.rs
│   │   ├── change_configuration.rs
│   │   ├── get_configuration.rs
│   │   ├── clear_cache.rs
│   │   ├── get_diagnostics.rs
│   │   ├── update_firmware.rs
│   │   ├── trigger_message.rs
│   │   └── send_local_list.rs
│   └── errors.rs                     # OcppError 枚举
│
├── api/                              # HTTP API 层（行政/运营）
│   ├── mod.rs
│   ├── router.rs
│   ├── auth.rs
│   ├── charge_points.rs
│   ├── id_tags.rs
│   ├── transactions.rs
│   ├── ...
│   └── error.rs
│
└── jobs/                             # 定时任务
    ├── mod.rs
    ├── heartbeat_watchdog.rs
    ├── stuck_transaction_cleanup.rs
    ├── pending_commands_scanner.rs
    └── bill_generator.rs
```

---

## 十一、依赖清单

```toml
[dependencies]
# Web
axum.workspace = true
tower = "*"
tower-http = { version = "*", features = ["cors", "trace"] }

# DB
sqlx = { version = "0.7", features = [
    "runtime-tokio-native-tls",
    "postgres",
    "chrono",
    "uuid",
    "json",
    "migrate"
]}

# Kafka (复用 ocpp-1-6 生态的 rdkafka)
rdkafka = { version = "0.36", features = ["cmake-build"] }

# Redis (P1, 心跳优化)
# redis = { version = "0.25", features = ["aio", "tokio-comp"] }

# Auth (P1)
# jsonwebtoken = "*"
# argon2 = "*"

# Metrics
# prometheus = "*"

# Config
config = "0.14"

# Serialization
serde.workspace = true
serde_json.workspace = true

# Time
chrono.workspace = true

# Logging
tracing.workspace = true
tracing-subscriber = "0.3"

# Error
thiserror.workspace = true
anyhow = "1"

# Async
tokio.workspace = true
futures = "*"

# UUID (生成 unique_id)
uuid.workspace = true
```

---

## 十二、实施路径（细化）

### Phase 0：**跑通一条链路** (1-2 天)
1. `infra/kafka/consumer.rs` - 实现 req.* 消费
2. `infra/kafka/producer.rs` - 实现 resp.{gateway_id} 发送
3. `infra/db/pool.rs` - sqlx PgPool 封装
4. `ocpp/envelope.rs` - CloudMessage 解析
5. `ocpp/handlers/boot_notification.rs`
6. `ocpp/handlers/heartbeat.rs`
7. 集成测试：Simulator 走 Boot + Heartbeat 完整流程

### Phase 1：**核心消息处理** (3-4 天)
1. Authorize、Start/Stop Transaction、MeterValues、StatusNotification
2. 完整 Service 层
3. Outbox Dispatcher + Response Matcher
4. PendingCommandsTimeoutScanner 定时任务
5. 集成测试：完整充电流程

### Phase 2：**运营接口** (3-5 天)
1. Operators + Auth
2. Charge Points / Id Tags / Transactions 列表 API
3. Remote Commands HTTP 端点
4. Audit logs 自动埋点

### Phase 3：**可观测** (2-3 天)
1. ObservabilityService 全接入
2. DLQ 列表 + 重放
3. Message log trace
4. Prometheus metrics

### Phase 4：**扩展** (远期)
1. Billing + Tariffs
2. Reservations
3. Firmware + Config Keys
4. ChargingProfiles

---

## 十三、与已有代码的复用关系

| 复用来源 | 路径 | 用途 |
|---|---|---|
| `ocpp-1-6` crate | `crates/ocpp-1-6` | OCPP 消息结构定义（BootNotificationRequest, etc.） |
| Gateway `CloudMessage` | `crates/charge-mgt-gateway/src/cloud/message.rs` | 复用消息信封（建议抽到 common 或直接复制） |
| Kafka 配置 | 已存在 Gateway | Cloud 复用 brokers/topic 前缀 |

---

## 十四、总结

### 已明确
- ✅ 9 个 OCPP Consumer Handler（入站）
- ✅ 12 个 CSMS 主动命令（出站）
- ✅ 完整的数据流全景
- ✅ Action 路由表
- ✅ 错误码映射
- ✅ 幂等性保证策略
- ✅ Outbox 模式
- ✅ Response Matcher 设计
- ✅ 业务规则（每个 handler 的具体逻辑）
- ✅ DB 交互（每 handler 涉及的表/列/SQL）
- ✅ 性能热点分析
- ✅ Rust 模块架构
- ✅ 实施路径

### 待决策
- 10 个未决问题（第九节）

### 下一步
- 决定哪些未决问题先拍板
- 开始 Phase 0 实施
