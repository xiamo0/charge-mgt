# charge-mgt-cloud API 接口清单

> **目标 Crate**: `charge-mgt-cloud`（CSMS - Central System Management System）
> **协议**: 当前只支持 OCPP 1.6（未来可扩展 2.0.1+）
> **依赖 DB**: 参见 [db_schema.md](./db_schema.md)（19 张表，11 张 OCPP 相关）
> **版本**: v1.0（2026-06-08）

---

## 一、接口总览

charge-mgt-cloud 作为 CSMS（Central System / OCPP 后端），共提供 **四大类接口**：

| 类别 | 数量 | 优先级 | 说明 |
|---|---|---|---|
| **HTTP REST API** | ~32 个端点 | P0/P1/P2 | 运营后台、前端、移动端接入 |
| **Kafka Consumer** | 9 个 action handler | P0 | CP → Cloud 数据流（OCPP 上报） |
| **Kafka Producer** | ~12 个 action dispatcher | P0/P1 | Cloud → CP 数据流（响应 + 命令） |
| **内部定时任务** | 4 个 job | P1/P2 | 数据清理、账单、健康检查 |

**优先级定义**：
- **P0**：必须（MVP）
- **P1**：二期（生产可用）
- **P2**：远期（扩展功能）

---

## 二、HTTP REST API（行政/管理/查询）

### 2.0 通用约定

```yaml
Base URL:     http://cloud-host:8000/api
Content-Type: application/json
Auth:         Bearer <JWT>（除登录接口外，全部需要）
错误响应格式（RFC 7807 Problem Details）:
  {
    "type": "https://charge-mgt/errors/not-found",
    "title": "Not Found",
    "status": 404,
    "detail": "Charge point 'CP-999' does not exist",
    "trace_id": "uuid-..."
  }
分页参数:
  ?page=1&page_size=20&sort=created_at:desc
```

### 2.1 系统类接口

| 方法 | 路径 | 优先级 | 说明 | 响应示例 |
|---|---|---|---|---|
| `GET` | `/health` | P0 | 健康检查（公开） | `{"status":"ok","version":"0.1.0","db":"up","kafka":"up"}` |
| `GET` | `/api/metrics` | P1 | Prometheus 指标 | `{...}` |

### 2.2 鉴权 / Session 接口 (P1)

| 方法 | 路径 | 说明 | 请求体 | 响应体 |
|---|---|---|---|---|
| `POST` | `/api/auth/login` | 登录（返回 JWT） | `{username, password}` | `{access_token, refresh_token, expires_in}` |
| `POST` | `/api/auth/refresh` | 刷新 token | `{refresh_token}` | `{access_token, expires_in}` |
| `POST` | `/api/auth/logout` | 登出（作废 token） | — | `{}` |
| `GET` | `/api/auth/me` | 当前用户信息 | — | operator 详情 |

### 2.3 运营商 / Operator 接口 (P0)

| 方法 | 路径 | 说明 | 权限 |
|---|---|---|---|
| `GET` | `/api/operators` | 列表（分页） | admin |
| `GET` | `/api/operators/{id}` | 详情 | admin |
| `POST` | `/api/operators` | 创建 | admin |
| `PUT` | `/api/operators/{id}` | 更新 | admin |
| `DELETE` | `/api/operators/{id}` | 软删除（is_deleted=true） | admin |
| `POST` | `/api/operators/{id}/password` | 修改密码 | admin / self |
| `POST` | `/api/operators/{id}/deactivate` | 停用账号 | admin |

### 2.4 充电桩 / Charge Points 接口 (P0)

| 方法 | 路径 | 说明 | 查询参数 / 请求体 |
|---|---|---|---|
| `GET` | `/api/charge-points` | 列表 | `?status=Online&vendor=Alphas&page=1` |
| `GET` | `/api/charge-points/{cp_id}` | 详情（含基础信息 + 当前状态） | — |
| `PUT` | `/api/charge-points/{cp_id}` | 更新业务元数据（如备注、归属分组） | `{note, group_id}` |
| `DELETE` | `/api/charge-points/{cp_id}` | 软删除（is_deleted=true） | — |
| `GET` | `/api/charge-points/{cp_id}/connectors` | 列出所有连接器状态 | — |
| `GET` | `/api/charge-points/{cp_id}/transactions` | 列出该桩交易历史 | `?state=Completed` |
| `GET` | `/api/charge-points/{cp_id}/meter-values` | 列出该桩最新度量 | `?limit=100` |

### 2.5 连接器 / Connectors 接口 (P0)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/connectors/{id}` | 详情（状态、错误码、最近更新时间） |
| `PUT` | `/api/connectors/{id}` | 手动修改状态（运营强制置位） |

### 2.6 ID 标签 / Id Tags 接口 (P0)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/id-tags` | 列表（支持状态、分组过滤） |
| `GET` | `/api/id-tags/{id_tag}` | 详情（鉴权状态、过期时间、父卡） |
| `POST` | `/api/id-tags` | 批量导入 / 单卡创建 |
| `PUT` | `/api/id-tags/{id_tag}` | 更新（状态、过期时间、分组） |
| `DELETE` | `/api/id-tags/{id_tag}` | 软删除 |
| `POST` | `/api/id-tags/block` | 批量冻结 |
| `POST` | `/api/id-tags/unblock` | 批量解冻 |
| `GET` | `/api/id-tags/{id_tag}/transactions` | 该标签所有交易 |

### 2.7 交易 / Transactions 接口 (P0)

| 方法 | 路径 | 说明 | 典型查询参数 |
|---|---|---|---|
| `GET` | `/api/transactions` | 列表 | `?state=Active&id_tag=CARD-001&started_after=...` |
| `GET` | `/api/transactions/{id}` | 详情 | — |
| `GET` | `/api/transactions/{id}/meter-values` | 该交易的度量序列 | `?measurand=EnergyActiveImportRegister` |
| `POST` | `/api/transactions/{id}/abort` | 运营手动标为 Aborted（异常交易处理） | `{reason}` |
| `GET` | `/api/transactions/active-count` | 统计当前 Active 交易数量（运维大盘） | — |

### 2.8 度量 / Meter Values 接口 (P0)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/meter-values` | 列表（按 transaction/connector/measurand 过滤） |
| `GET` | `/api/meter-values/aggregate` | 聚合查询（时间段、桩、标签维度的 kWh 汇总） |

### 2.9 费率 / Tariffs 接口 (P1)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/tariffs` | 列表 |
| `GET` | `/api/tariffs/{id}` | 详情（含阶梯规则 JSON） |
| `POST` | `/api/tariffs` | 创建 |
| `PUT` | `/api/tariffs/{id}` | 更新 |
| `DELETE` | `/api/tariffs/{id}` | 软删除 |
| `GET` | `/api/tariffs/applicable` | 按当前 id_tag 查找适用费率（运营预览） |

### 2.10 账单 / Bills 接口 (P1)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/bills` | 列表（按 id_tag / 状态过滤） |
| `GET` | `/api/bills/{id}` | 详情（含交易明细） |
| `POST` | `/api/bills/generate` | 生成某标签在某周期的账单（运营触发） |
| `POST` | `/api/bills/{id}/pay` | 模拟支付成功回调（测试环境用） |
| `POST` | `/api/bills/{id}/refund` | 退款 |

### 2.11 远程命令 / Remote Commands 接口 (P1)

| 方法 | 路径 | 说明 | outbox 关联 |
|---|---|---|---|
| `POST` | `/api/remote/remote-start` | 远程启动交易 | 插入 `pending_commands` |
| `POST` | `/api/remote/remote-stop` | 远程停止交易 | 插入 `pending_commands` |
| `POST` | `/api/remote/reset` | 远程重启桩 | 插入 `pending_commands` |
| `POST` | `/api/remote/unlock-connector` | 远程解锁连接器 | 插入 `pending_commands` |
| `POST` | `/api/remote/change-availability` | 上线/下线桩 | 插入 `pending_commands` |
| `POST` | `/api/remote/change-configuration` | 修改桩配置 | 插入 `pending_commands` |
| `POST` | `/api/remote/get-configuration` | 读取桩配置 | 插入 `pending_commands` |
| `POST` | `/api/remote/clear-cache` | 清空桩缓存 | 插入 `pending_commands` |
| `POST` | `/api/remote/get-diagnostics` | 拉取诊断日志 | 插入 `pending_commands` |
| `POST` | `/api/remote/update-firmware` | 推送固件更新 | 插入 `pending_commands` |
| `POST` | `/api/remote/trigger-message` | 触发桩重新上报特定消息 | 插入 `pending_commands` |
| `POST` | `/api/remote/send-local-list` | 下发本地授权列表 | 插入 `pending_commands` |
| `DELETE` | `/api/remote/{unique_id}` | 取消命令（仅 Pending 态允许） | — |
| `GET` | `/api/remote/{unique_id}` | 查询命令状态 | — |
| `GET` | `/api/remote/history` | 命令历史（含成功/失败率） | — |

### 2.12 配置 / Configuration Keys 接口 (P1)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/config-keys` | 列出 CSMS 维护的标准配置键 |
| `GET` | `/api/config-keys/{key}` | 详情 |
| `PUT` | `/api/config-keys/{key}` | 更新默认值 |

### 2.13 固件 / Firmware 接口 (P1)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/firmware` | 固件版本列表 |
| `GET` | `/api/firmware/{id}` | 详情 |
| `POST` | `/api/firmware` | 上传新固件（实际文件传 OSS/S3，本接口只存元数据） |
| `DELETE` | `/api/firmware/{id}` | 软删除 |

### 2.14 预约 / Reservations 接口 (P2)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/reservations` | 列表 |
| `POST` | `/api/reservations` | 创建预约（触发 ReserveNow） |
| `DELETE` | `/api/reservations/{id}` | 取消预约（触发 CancelReservation） |

### 2.15 充电功率曲线 / Charging Profiles 接口 (P2)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/charging-profiles` | 列表（按 cp 过滤） |
| `POST` | `/api/charging-profiles` | 创建（触发 SetChargingProfile） |
| `DELETE` | `/api/charging-profiles/{id}` | 删除（触发 ClearChargingProfile） |
| `GET` | `/api/charging-profiles/composite` | 查询复合曲线（触发 GetCompositeSchedule） |

### 2.16 可观测性 / Observability 接口 (P1)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/message-log` | OCPP 消息流日志（按 cp_id 过滤） |
| `GET` | `/api/message-log/{unique_id}/trace` | 单次请求-响应的完整链路（含 latency） |
| `GET` | `/api/sent-messages` | idempotency 表查询 |
| `GET` | `/api/gateway-health` | 各 Gateway 健康状态 |
| `GET` | `/api/dead-letter-queue` | 死信列表 |
| `POST` | `/api/dead-letter-queue/{id}/replay` | 重放死信 |
| `POST` | `/api/dead-letter-queue/{id}/resolve` | 标记为已解决 |
| `GET` | `/api/audit-logs` | 运营操作审计（按 actor / target / action 过滤） |

### 2.17 聚合统计接口 (P1/P2)

| 方法 | 路径 | 说明 |
|---|---|---|
| `GET` | `/api/stats/summary` | 今日/本周/本月的交易数、kWh、营收 |
| `GET` | `/api/stats/top-charge-points` | 充电量 Top N |
| `GET` | `/api/stats/top-id-tags` | 消费 Top N |
| `GET` | `/api/stats/fault-stats` | 故障码分布 |

---

## 三、Kafka Consumer 接口（CP → Cloud 数据流）

### 3.1 总体架构

```
Kafka Topics（1+ 个 vendor）:
  charge_mgt.req.Alphas
  charge_mgt.req.Beta
  charge_mgt.req.SimVendor
  ...

Cloud Consumer Group:
  "cloud-csms-consumer"

消息格式（CloudMessage 信封）:
  {
    "gateway_id": "gateway-001",
    "gateway_ip": "192.168.1.10",
    "vendor": "Alphas",
    "charge_point_id": "ALPHA-CP-042",
    "protocol": "OCPP-1.6",
    "message_type": "Call",         // "Call" | "CallResult" | "CallError"
    "action": "BootNotification",
    "unique_id": "uuid-...",
    "payload": { ... },
    "received_at": "2026-06-08T10:00:00Z",
    "error_code": null,
    "error_description": null
  }
```

### 3.2 Action Handler 清单

根据 `CloudMessage.action` 路由到对应 handler：

| Action | Handler | 业务逻辑 | 响应 |
|---|---|---|---|
| `BootNotification` | `boot_notification_handler` | 注册/更新 `charge_points_ocpp_1_6`，记录 vendor/model/serial | `CallResult {status: Accepted, interval: 30, currentTime}` |
| `Heartbeat` | `heartbeat_handler` | 更新 `charge_points.last_heartbeat_at` | `CallResult {currentTime}` |
| `Authorize` | `authorize_handler` | 查询 `id_tags_ocpp_1_6.status`，判过期/block | `CallResult {idTagInfo: {status, expiryDate, parentIdTag}}` |
| `StartTransaction` | `start_transaction_handler` | 插入 `transactions_ocpp_1_6`（state=Active），返回 transactionId | `CallResult {transactionId, idTagInfo: {status}}` |
| `StopTransaction` | `stop_transaction_handler` | 更新 transactions 设 meter_stop/stop_reason/stop_timestamp/state=Completed | `CallResult {idTagInfo: {status}}` + 调用 billing 计费 |
| `MeterValues` | `meter_values_handler` | 批量插入 `meter_values_ocpp_1_6` | `CallResult {}` |
| `StatusNotification` | `status_notification_handler` | 更新 `connectors_ocpp_1_6.status`/`error_code` | `CallResult {}` |
| `FirmwareStatusNotification` | `firmware_status_handler` | 更新 `firmware_ocpp_1_6.status` | `CallResult {}`（OCPP 1.6 无要求） |
| `DiagnosticsStatusNotification` | `diag_status_handler` | 日志记录即可 | `CallResult {}` |

### 3.3 响应生产流程

```
Handler 处理完 → 构造 CloudResponse { message_type: "CallResult", ... }
   → Kafka Producer 发到 charge_mgt.resp.{gateway_id} topic
   → Gateway 转发到 Charge Point
```

---

## 四、Kafka Producer 接口（Cloud → CP 数据流）

### 4.1 调用响应（对 CP 上报）

| 触发 handler | Action | 响应类型 |
|---|---|---|
| 全部 9 个 | 各自 | `CallResult` / `CallError` |

### 4.2 CSMS 主动命令（outbox pattern）

通过 `pending_commands_ocpp_1_6` 表实现 outbox：

```
HTTP 端点（POST /api/remote/*）
  → Service 校验参数
  → 插入 pending_commands (state = Pending, unique_id = Uuid::new_v4)
  → 触发 Dispatcher Task
  → Dispatcher 读表 → 发送 Kafka 到 charge_mgt.resp.{gateway_id}
  → CP 回报 CallResult
  → Consumer handler 匹配 unique_id → 更新 pending_commands state = Accepted

超时：定时任务扫 expires_at < now() 且 state = Pending 的，标记 Timeout
```

### 4.3 命令清单（与 2.11 一一对应）

| Action | OCPP 消息体（payload 字段） | 关键参数 |
|---|---|---|
| `RemoteStartTransaction` | `{connectorId, idTag, chargingProfile?}` | idTag（必填） |
| `RemoteStopTransaction` | `{transactionId}` | — |
| `Reset` | `{type: "Hard"/"Soft"}` | — |
| `UnlockConnector` | `{connectorId}` | — |
| `ChangeAvailability` | `{connectorId, type: "Operative"/"Inoperative"}` | — |
| `ChangeConfiguration` | `{key, value}` | — |
| `GetConfiguration` | `{key?}` | — |
| `ClearCache` | `{}` | — |
| `GetDiagnostics` | `{location, retries?, retryInterval?, startTime?, stopTime?}` | location 必填 |
| `UpdateFirmware` | `{location, retrieveDate, retries?, retryInterval?}` | location 必填 |
| `TriggerMessage` | `{requestedMessage, connectorId?}` | requestedMessage 必填 |
| `SendLocalList` | `{listVersion, localAuthorizationList?, updateType: "Full"/"Differential"}` | — |

---

## 五、内部定时任务接口

### 5.1 HeartbeatWatchdog（P1）

```rust
pub struct HeartbeatWatchdog;

impl ScheduledJob for HeartbeatWatchdog {
    const INTERVAL: Duration = Duration::from_secs(60);  // 每分钟跑一次
    const NAME: &str = "heartbeat_watchdog";

    async fn run(&self, ctx: &JobContext) -> JobResult {
        // 1. SELECT charge_points_ocpp_1_6
        //       WHERE ocpp_status = 'Online'
        //         AND last_heartbeat_at < NOW() - INTERVAL '5 min' * heartbeat_interval_secs
        // 2. UPDATE SET ocpp_status = 'Offline'
        // 3. 写 audit_logs
        // 4. （可选）推送告警到外部系统
    }
}
```

### 5.2 StuckTransactionCleanup（P1）

```rust
pub struct StuckTransactionCleanup;

impl ScheduledJob for StuckTransactionCleanup {
    const INTERVAL: Duration = Duration::from_secs(600);  // 10 分钟
    const NAME: &str = "stuck_transaction_cleanup";

    async fn run(&self, ctx: &JobContext) -> JobResult {
        // 1. SELECT transactions_ocpp_1_6
        //       WHERE state = 'Active'
        //         AND started_at < NOW() - INTERVAL '24 hours'
        //         AND charge_point_id IN (SELECT id FROM charge_points WHERE ocpp_status = 'Offline')
        // 2. UPDATE SET state = 'Faulted', closed_at = NOW()
        // 3. 触发 billing 重新计费
    }
}
```

### 5.3 PendingCommandsTimeoutScanner（P0/P1）

```rust
pub struct PendingCommandsTimeoutScanner;

impl ScheduledJob for PendingCommandsTimeoutScanner {
    const INTERVAL: Duration = Duration::from_secs(30);
    const NAME: &str = "pending_commands_timeout_scanner";

    async fn run(&self, ctx: &JobContext) -> JobResult {
        // 1. SELECT pending_commands_ocpp_1_6
        //       WHERE state = 'Pending'
        //         AND expires_at < NOW()
        // 2. UPDATE SET state = 'Timeout', responded_at = NOW()
    }
}
```

### 5.4 BillGenerator（P2）

```rust
pub struct BillGenerator;

impl ScheduledJob for BillGenerator {
    const INTERVAL: Duration = Duration::from_secs(3600);  // 每小时跑一次
    const NAME: &str = "bill_generator";

    async fn run(&self, ctx: &JobContext) -> JobResult {
        // 1. 按 id_tag 分组，统计每个标签在上一周/月未生成 bill 的 Completed 交易
        // 2. 调用 BillingService 计算费用（考虑 tariff 阶梯）
        // 3. 插入 bills 表
    }
}
```

---

## 六、内部业务 Service 接口（Rust 层）

HTTP Handler 和 Kafka Handler 都委托以下 Service 完成核心逻辑：

### 6.1 ChargePointService

```rust
pub trait ChargePointService {
    async fn register_or_update(&self, boot: BootNotification) -> Result<ChargePoint>;
    async fn heartbeat(&self, cp_id: &str) -> Result<DateTime<Utc>>;
    async fn list(&self, filter: ChargePointFilter) -> Result<Page<ChargePoint>>;
    async fn get(&self, cp_id: &str) -> Result<Option<ChargePoint>>;
    async fn update(&self, cp_id: &str, patch: ChargePointPatch) -> Result<()>;
    async fn soft_delete(&self, cp_id: &str) -> Result<()>;
    async fn list_connectors(&self, cp_id: &str) -> Result<Vec<Connector>>;
    async fn list_transactions(&self, cp_id: &str, filter: TxFilter) -> Result<Page<Transaction>>;
    async fn mark_offline(&self, cp_id: &str, reason: &str) -> Result<()>;
}
```

### 6.2 IdTagService

```rust
pub trait IdTagService {
    async fn authorize(&self, id_tag: &str) -> Result<AuthorizationStatus>;
    async fn list(&self, filter: IdTagFilter) -> Result<Page<IdTag>>;
    async fn get(&self, id_tag: &str) -> Result<Option<IdTag>>;
    async fn create(&self, new: IdTagCreate) -> Result<IdTag>;
    async fn update(&self, id_tag: &str, patch: IdTagPatch) -> Result<IdTag>;
    async fn soft_delete(&self, id_tag: &str) -> Result<()>;
    async fn batch_block(&self, id_tags: &[String]) -> Result<usize>;
    async fn batch_unblock(&self, id_tags: &[String]) -> Result<usize>;
}
```

### 6.3 TransactionService

```rust
pub trait TransactionService {
    async fn start(&self, req: StartTransaction) -> Result<TransactionId>;
    async fn stop(&self, req: StopTransaction) -> Result<()>;
    async fn get(&self, id: TransactionId) -> Result<Option<Transaction>>;
    async fn list(&self, filter: TxFilter) -> Result<Page<Transaction>>;
    async fn abort(&self, id: TransactionId, reason: &str) -> Result<()>;
    async fn list_meter_values(&self, id: TransactionId, filter: MvFilter) -> Result<Vec<MeterValue>>;
}
```

### 6.4 MeterValueService

```rust
pub trait MeterValueService {
    async fn ingest(&self, cp_id: &str, mv: MeterValuesRequest) -> Result<()>;
    async fn list(&self, filter: MvFilter) -> Result<Page<MeterValue>>;
    async fn aggregate(&self, agg: AggregateQuery) -> Result<Vec<AggregateRow>>;
}
```

### 6.5 RemoteCommandService

```rust
pub trait RemoteCommandService {
    async fn dispatch(&self, action: &str, cp_id: &str, payload: Value, timeout_secs: u64) -> Result<PendingCommand>;
    async fn cancel(&self, unique_id: &str) -> Result<()>;
    async fn get(&self, unique_id: &str) -> Result<Option<PendingCommand>>;
    async fn list(&self, filter: CommandFilter) -> Result<Page<PendingCommand>>;
    async fn handle_response(&self, cp_id: &str, unique_id: &str, result: CallResult) -> Result<()>;  // Kafka handler 调用
}
```

### 6.6 BillingService（P1）

```rust
pub trait BillingService {
    async fn calculate(&self, tx_id: TransactionId, tariff: &Tariff) -> Result<BillBreakdown>;
    async fn generate(&self, id_tag: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Bill>;
    async fn list(&self, filter: BillFilter) -> Result<Page<Bill>>;
    async fn mark_paid(&self, bill_id: BillId, via: &str) -> Result<()>;
    async fn refund(&self, bill_id: BillId, reason: &str) -> Result<()>;
}
```

### 6.7 TariffService（P1）

```rust
pub trait TariffService {
    async fn list(&self, filter: TariffFilter) -> Result<Vec<Tariff>>;
    async fn get(&self, id: TariffId) -> Result<Option<Tariff>>;
    async fn create(&self, new: TariffCreate) -> Result<Tariff>;
    async fn update(&self, id: TariffId, patch: TariffPatch) -> Result<Tariff>;
    async fn soft_delete(&self, id: TariffId) -> Result<()>;
    async fn find_applicable(&self, id_tag: &str, at: DateTime<Utc>) -> Result<Option<Tariff>>;
}
```

### 6.8 ObservabilityService（P1）

```rust
pub trait ObservabilityService {
    async fn log_message(&self, msg: OcppMessage) -> Result<()>;   // Kafka 流写入
    async fn trace(&self, unique_id: &str) -> Result<MessageTrace>;
    async fn gateway_health(&self) -> Result<Vec<GatewayHealth>>;
    async fn dlq_list(&self) -> Result<Vec<DlqEntry>>;
    async fn dlq_replay(&self, id: DlqId) -> Result<()>;
    async fn audit_log(&self, filter: AuditFilter) -> Result<Page<AuditLog>>;
}
```

---

## 七、接口优先级矩阵

| 优先级 | HTTP 接口 | Kafka Consumer | Kafka Producer | 内部 Service | 定时任务 |
|---|---|---|---|---|---|
| **P0** | health, operators, charge-points, id-tags, transactions, meter-values, connectors | 9 个 action handler | CallResult/CallError（响应） | ChargePoint/IdTag/Transaction/MeterValue/RemoteCommand | PendingCommandsTimeoutScanner |
| **P1** | auth, tariffs, bills, remote commands, config-keys, firmware, message-log, dlq, audit-logs, gateway-health, stats | FirmwareStatusNotification, DiagnosticsStatusNotification | RemoteStart, RemoteStop, Reset, Unlock, ... | Billing, Tariff | HeartbeatWatchdog, StuckTransactionCleanup |
| **P2** | reservations, charging-profiles | — | ReserveNow, CancelReservation, SetChargingProfile | Reservation, ChargingProfile | BillGenerator |

---

## 八、实施顺序

### Phase 0（MVP）—— 跑通一条 OCPP 链路
1. `ChargePointService`（register_or_update, heartbeat）
2. Kafka Consumer（BootNotification, Heartbeat）
3. Kafka Producer（CallResult 响应）
4. HTTP `/health` + `/api/charge-points`（最小列表）

### Phase 1（基本可用）—— 完整 Boot→StopTransaction 链路
1. 全部 5 个 handler（Authorize, Start, Stop, MeterValues, Status）
2. `IdTagService` + `TransactionService` + `MeterValueService`
3. HTTP 端点：charge-points, id-tags, transactions, meter-values
4. Operators + Auth

### Phase 2（生产可用）—— 运营能力
1. `RemoteCommandService` + 12 个远程命令端点
2. `BillingService` + `TariffService`
3. Observability（message-log, dlq, audit）
4. 全部定时任务

### Phase 3（扩展）—— 高级特性
1. Reservations + ChargingProfiles
2. Firmware + Config 管理
3. 统计接口（stats/*）
4. 多租户、权限细分

---

## 九、与 db_schema.md 的对应关系

| API 类别 | 主要依赖表 |
|---|---|
| 充电桩相关 | `charge_points_ocpp_1_6`, `connectors_ocpp_1_6` |
| ID 标签 | `id_tags_ocpp_1_6`, `id_tag_groups` |
| 交易 | `transactions_ocpp_1_6`, `meter_values_ocpp_1_6` |
| 费率账单 | `tariffs`, `bills` |
| 远程命令 | `pending_commands_ocpp_1_6` |
| 配置 | `config_keys_ocpp_1_6` |
| 固件 | `firmware_ocpp_1_6` |
| 预约 | `reservations_ocpp_1_6` |
| 功率曲线 | `charging_profiles_ocpp_1_6` |
| 可观测性 | `sent_messages`, `message_log_ocpp_1_6`, `gateway_health`, `dead_letter_queue`, `audit_logs` |
| 运营 | `operators` |

---

## 十、关键设计约定

### 10.1 幂等性
- 所有 Kafka handler 用 `unique_id` 查 `sent_messages` 保证幂等
- HTTP 端点中 POST 创建接口加 `Idempotency-Key` header（业务方自生成）

### 10.2 错误处理
- 统一 RFC 7807 Problem Details
- OCPP 错误用 `GatewayError` 枚举（已有）+ `BusinessError` 枚举（待新建）

### 10.3 审计
- 所有 `create/update/delete` 类 HTTP 接口自动写 `audit_logs`
- `initiated_by` 字段记 operator_id

### 10.4 软删除
- 业务表统一用 `is_deleted` + `deleted_at`（若需要）
- 查询默认过滤 `is_deleted = false`，带 `?include_deleted=true` 可看全量

### 10.5 时间
- UTC 全链路存储和传输
- 显示层（前端）做时区转换

---

## 十一、未决问题（需要进一步决策）

| 编号 | 问题 | 建议 |
|---|---|---|
| 1 | HTTP 鉴权用 JWT 还是 Session？ | JWT（无状态，方便多实例部署） |
| 2 | 运营权限模型（RBAC/ABAC）？ | 先用 RBAC（admin / operator / viewer 三档），未来需要再扩展 |
| 3 | 账单生成是"按交易实时计费"还是"按周期聚合"？ | 先用"按交易实时计费 + 周期生成账单"混合 |
| 4 | DLQ 是否需要 Web UI？ | P1 必须有，至少支持列表 + 重放 |
| 5 | 是否支持 WebSocket 长连接给前端？ | P2 阶段考虑，P0/P1 用 HTTP short polling 即可 |
| 6 | Gateway 多实例时如何保证命令路由到正确的 gateway？ | CSMS 侧维护 `gateway_health` 表，按 last_seen_at 自动剔除 |
