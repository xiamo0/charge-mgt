# OCPP 2.1 报文详解

> 本文档详细描述 OCPP 2.1 协议的所有报文格式、字段说明和使用场景。
> 方便编程时参考。
>
> 基于 OCPP 2.1 Edition 2 (2025) — Part 1 Architecture & Topology, Part 2 Specification, Part 4 OCPP-J
> Open Charge Alliance 官方规范 + OCA 官方 OCPP 2.1 JSON Schema 衍生。

---

## 一、协议概述

OCPP 2.1J 基于 **JSON over WebSocket** 通信，WebSocket 子协议标识为 **`ocpp2.1`**。

### 1.1 消息类型

在 2.0.1 的 CALL/CALLRESULT/CALLERROR 基础上，2.1 **新增** 两种消息类型：

| 消息类型 | MessageTypeId | 方向 | 说明 |
|---|---|---|---|
| CALL | 2 | 双向 | 请求（等待 CALLRESULT 或 CALLERROR） |
| CALLRESULT | 3 | 双向 | 成功响应（按 messageId 关联） |
| CALLERROR | 4 | 双向 | 错误响应 |
| **CALLRESULTERROR** | **5** | 双向 | **2.1 新增** — 响应方无法处理 CALLRESULT 时返回 |
| **SEND** | **6** | 单向 (CS → CSMS) | **2.1 新增** — fire-and-forget 单向消息，不需要响应。用于高频遥测（NotifyPeriodicEventStream） |

### 1.2 消息格式

**CALL 格式**：
```json
[2, "<messageId>", "<action>", { "payload" }]
```

**CALLRESULT 格式**：
```json
[3, "<messageId>", { "payload" }]
```

**CALLERROR 格式**：
```json
[4, "<messageId>", "<errorCode>", "<errorDescription>", { "errorDetails" }]
```

**CALLRESULTERROR 格式（2.1 新增）**：
```json
[5, "<messageId>", "<errorCode>", "<errorDescription>", { "errorDetails" }]
```

**SEND 格式（2.1 新增，单向）**：
```json
[6, "<messageId>", "<action>", { "payload" }]
```

### 1.3 与 OCPP 2.0.1 的主要区别

| 特性 | OCPP 2.0.1 | OCPP 2.1 |
|---|---|---|
| 总消息数 | 64 | **91** (+27 net-new) |
| 功能块数 | 15 (A–P) | **19 (A–S)** |
| 子协议 | `ocpp2.0.1` | `ocpp2.1` |
| CALL/CALLRESULT/CALLERROR | ✓ | ✓ (完全兼容) |
| 新功能块 | — | **TariffAndCost (I) / Bidirectional (Q) / DERControl (R) / BatterySwap (S)** |
| 消息类型扩展 | 仅 2/3/4 | **+ CALLRESULTERROR (5)** + **SEND (6, 单向)** |
| V2X / 双向充电 | 不支持 | **原生支持 (Block Q)** |
| DER 控制 | 不支持 | **光伏/储能/电网交互 (Block R)** |
| 电价模型 | 仅 CostUpdated | **完整 Tariff + 结算 + VAT (Block I, 8 消息)** |
| 电池交换站 | 不支持 | **完整交换流程 (Block S, 2 消息)** |
| 周期性事件流 | 不支持 | **高频遥测 SEND 通道 (Block N, 5 消息)** |
| 动态/优先级充电 | 不支持 | **Pull/Push 更新 + PriorityCharging (Block K, 4 消息)** |
| ISO 15118-20 | 仅 -2 | **+ ISO 15118-20 + 证书链查询 (Block M, 1 新消息)** |
| JSON Schema | draft-06 | draft-06 (相同) |
| `customData` 扩展 | 支持 | 支持（模式不变） |
| 向后兼容 | — | 全部 64 条 2.0.1 消息原样保留 |

### 1.4 设备模型

2.1 沿用 2.0.1 的层级设备模型：

```
Charging Station (充电桩, evseId=0 代表整站)
├── EVSE 1 (充电设备)
│   ├── Connector 1 (如 CCS)
│   └── Connector 2 (如 CHAdeMO)
├── EVSE 2
│   └── Connector 1 (如 Type 2)
└── ...
```

> **重要**: `evseId` 和 `connectorId` 均从 1 开始。`evseId=0` 表示整个充电桩。

### 1.5 功能块总表

| 字母 | 功能块 | 消息数 | 备注 |
|:-:|---|:-:|---|
| A | Security | 1 | — |
| B | Provisioning | 9 | — |
| C | Authorization | 2 | — |
| D | LocalAuthList | 2 | — |
| E | Transactions | 2 | — |
| F | RemoteControl | 4 | 2.1 重新划分：Reset 移至 B |
| G | Availability | 2 | — |
| H | Reservation | 3 | — |
| **I** | **TariffAndCost** | **8** | **2.1 新增块** |
| J | MeterValues | 1 | — |
| K | SmartCharging | 13 | +4 新消息 |
| L | Firmware | 5 | — |
| M | Certificates | 8 | +1 新消息 |
| N | Diagnostics | 16 | +5 新消息 |
| O | Display | 4 | CostUpdated 已迁至 I |
| P | DataTransfer | 1 | — |
| **Q** | **Bidirectional / V2X** | **2** | **2.1 新增块** |
| **R** | **DERControl** | **6** | **2.1 新增块** |
| **S** | **BatterySwap** | **2** | **2.1 新增块** |
| **总计** | | **91** | |

---

## 二、报文总览

OCPP 2.1 共 **91 条消息**，组织为 19 个功能块。其中 **64 条从 2.0.1 原样保留**，**27 条为 2.1 新增**。

### 2.1 Block A — Security（1 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| SecurityEventNotification | CS → CSMS | 安全事件通知 | 同 2.0.1 |

### 2.2 Block B — Provisioning（9 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| BootNotification | CS → CSMS | 充电桩启动注册 | 同 2.0.1 |
| Heartbeat | CS → CSMS | 心跳保活 | 同 2.0.1 |
| GetBaseReport | CSMS → CS | 请求完整配置报告 | 同 2.0.1 |
| GetReport | CSMS → CS | 请求过滤配置报告 | 同 2.0.1 |
| NotifyReport | CS → CSMS | 上报配置报告（分页） | 同 2.0.1 |
| GetVariables | CSMS → CS | 读取配置变量 | 同 2.0.1 |
| SetVariables | CSMS → CS | 写入配置变量 | 同 2.0.1 |
| SetNetworkProfile | CSMS → CS | 设置网络配置 | 同 2.0.1 |
| Reset | CSMS → CS | 重启充电桩 | 同 2.0.1（2.0.1 在 F 块） |

### 2.3 Block C — Authorization（2 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| Authorize | CS → CSMS | 用户鉴权请求 | 同 2.0.1 |
| ClearCache | CSMS → CS | 清除授权缓存 | 同 2.0.1 |

### 2.4 Block D — LocalAuthList（2 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| SendLocalList | CSMS → CS | 同步本地白名单 | 同 2.0.1 |
| GetLocalListVersion | CSMS → CS | 查询白名单版本 | 同 2.0.1 |

### 2.5 Block E — Transactions（2 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| TransactionEvent | CS → CSMS | 事务生命周期事件 | 同 2.0.1 |
| GetTransactionStatus | CSMS → CS | 查询事务状态 | 同 2.0.1 |

### 2.6 Block F — RemoteControl（4 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| RequestStartTransaction | CSMS → CS | 远程启动充电 | 同 2.0.1 |
| RequestStopTransaction | CSMS → CS | 远程停止充电 | 同 2.0.1 |
| UnlockConnector | CSMS → CS | 解锁连接器 | 同 2.0.1 |
| TriggerMessage | CSMS → CS | 触发特定消息上报 | 同 2.0.1 |

> **注**: 2.1 将 2.0.1 的 `Reset` 和 `ChangeAvailability` 分别移至 **Block B（Provisioning）** 和 **Block G（Availability）**。

### 2.7 Block G — Availability（2 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| ChangeAvailability | CSMS → CS | 变更可用性状态 | 同 2.0.1 |
| StatusNotification | CS → CSMS | 连接器状态变更 | 同 2.0.1 |

### 2.8 Block H — Reservation（3 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| ReserveNow | CSMS → CS | 创建预约 | 同 2.0.1 |
| CancelReservation | CSMS → CS | 取消预约 | 同 2.0.1 |
| ReservationStatusUpdate | CS → CSMS | 预约状态变更 | 同 2.0.1 |

### 2.9 Block I — TariffAndCost（8 条消息，2.1 **新增块**）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| CostUpdated | CSMS → CS | 更新事务实时费用 | 从 2.0.1 Block O 移入 |
| GetTariffs | CSMS → CS | 查询已安装的电价 | **2.1 新增** |
| SetDefaultTariff | CSMS → CS | 设置 EVSE 默认电价 | **2.1 新增** |
| ChangeTransactionTariff | CSMS → CS | 替换运行中事务使用的电价 | **2.1 新增** |
| ClearTariffs | CSMS → CS | 清除已安装的电价 | **2.1 新增** |
| NotifySettlement | CS → CSMS | 上报事务结算/清算数据 | **2.1 新增** |
| NotifyWebPaymentStarted | CSMS → CS | 通知充电桩：网页/二维码支付已启动 | **2.1 新增** |
| VatNumberValidation | CS → CSMS | 验证客户 VAT 号码（用于发票） | **2.1 新增** |

### 2.10 Block J — MeterValues（1 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| MeterValues | CS → CSMS | 电能采样数据 | 同 2.0.1 |

### 2.11 Block K — SmartCharging（13 条消息，+4 条 2.1 新增）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| SetChargingProfile | CSMS → CS | 设置充电曲线 | 同 2.0.1 |
| GetChargingProfiles | CSMS → CS | 获取已安装充电曲线 | 同 2.0.1 |
| ClearChargingProfile | CSMS → CS | 清除充电曲线 | 同 2.0.1 |
| ReportChargingProfiles | CS → CSMS | 上报充电曲线 | 同 2.0.1 |
| GetCompositeSchedule | CSMS → CS | 获取复合充电计划 | 同 2.0.1 |
| ClearedChargingLimit | CS → CSMS | 充电限制清除通知 | 同 2.0.1 |
| NotifyChargingLimit | CS → CSMS | 上报外部充电限制 | 同 2.0.1 |
| NotifyEVChargingSchedule | CS → CSMS | 转发 EV 充电计划 (ISO 15118) | 同 2.0.1 |
| NotifyEVChargingNeeds | CS → CSMS | 转发 EV 充电需求 (ISO 15118) | 同 2.0.1 |
| PullDynamicScheduleUpdate | CS → CSMS | 充电桩请求新的动态调度设定点 | **2.1 新增** |
| UpdateDynamicSchedule | CSMS → CS | CSMS 推送动态调度更新 | **2.1 新增** |
| NotifyPriorityCharging | CS → CSMS | 上报优先级充电状态 | **2.1 新增** |
| UsePriorityCharging | CSMS → CS | 请求对事务启用优先级充电 | **2.1 新增** |

### 2.12 Block L — Firmware（5 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| UpdateFirmware | CSMS → CS | 发起固件更新 | 同 2.0.1 |
| FirmwareStatusNotification | CS → CSMS | 固件更新状态 | 同 2.0.1 |
| PublishFirmware | CSMS → CS | 发布固件供本地分发 | 同 2.0.1 |
| PublishFirmwareStatusNotification | CS → CSMS | 固件发布状态 | 同 2.0.1 |
| UnpublishFirmware | CSMS → CS | 停止固件发布 | 同 2.0.1 |

### 2.13 Block M — Certificates（8 条消息，+1 条 2.1 新增）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| Get15118EVCertificate | CS → CSMS | 获取 EV V2G 证书 (Plug & Charge) | 同 2.0.1 |
| GetCertificateStatus | CS → CSMS | 查询证书 OCSP 状态 | 同 2.0.1 |
| GetCertificateChainStatus | CS → CSMS | 查询证书链吊销/有效状态 | **2.1 新增** |
| SignCertificate | CS → CSMS | CSR 签名请求 | 同 2.0.1 |
| CertificateSigned | CSMS → CS | 下发签名证书 | 同 2.0.1 |
| InstallCertificate | CSMS → CS | 安装 CA 证书 | 同 2.0.1 |
| DeleteCertificate | CSMS → CS | 删除证书 | 同 2.0.1 |
| GetInstalledCertificateIds | CSMS → CS | 查询已安装证书 | 同 2.0.1 |

### 2.14 Block N — Diagnostics（16 条消息，+5 条 2.1 新增）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| GetLog | CSMS → CS | 请求上传日志 | 同 2.0.1 |
| LogStatusNotification | CS → CSMS | 日志上传状态 | 同 2.0.1 |
| NotifyEvent | CS → CSMS | 上报变量监控事件 | 同 2.0.1 |
| SetMonitoringBase | CSMS → CS | 激活出厂默认监控 | 同 2.0.1 |
| SetVariableMonitoring | CSMS → CS | 设置变量监控 | 同 2.0.1 |
| SetMonitoringLevel | CSMS → CS | 设置监控严重度阈值 | 同 2.0.1 |
| GetMonitoringReport | CSMS → CS | 请求监控配置报告 | 同 2.0.1 |
| ClearVariableMonitoring | CSMS → CS | 清除变量监控 | 同 2.0.1 |
| NotifyMonitoringReport | CS → CSMS | 上报监控配置 | 同 2.0.1 |
| CustomerInformation | CSMS → CS | 请求/清除客户数据 (GDPR) | 同 2.0.1 |
| NotifyCustomerInformation | CS → CSMS | 上报客户数据 | 同 2.0.1 |
| OpenPeriodicEventStream | CS → CSMS | 开启周期性事件流 | **2.1 新增** |
| ClosePeriodicEventStream | CS → CSMS | 关闭已开启的事件流 | **2.1 新增** |
| GetPeriodicEventStream | CSMS → CS | 枚举已开启的事件流 | **2.1 新增** |
| AdjustPeriodicEventStream | CSMS → CS | 调整事件流采样/上报参数 | **2.1 新增** |
| NotifyPeriodicEventStream | CS → CSMS | 推送高频遥测数据（**单向 SEND**，无响应） | **2.1 新增** |

### 2.15 Block O — Display（4 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| SetDisplayMessage | CSMS → CS | 设置显示屏消息 | 同 2.0.1 |
| GetDisplayMessages | CSMS → CS | 查询显示消息 | 同 2.0.1 |
| ClearDisplayMessage | CSMS → CS | 清除显示消息 | 同 2.0.1 |
| NotifyDisplayMessages | CS → CSMS | 上报显示消息 | 同 2.0.1 |

> **注**: 2.0.1 的 `CostUpdated` 已迁至 **Block I (TariffAndCost)**。

### 2.16 Block P — DataTransfer（1 条消息）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| DataTransfer | 双向 | 厂商自定义数据 | 同 2.0.1 |

### 2.17 Block Q — Bidirectional / V2X（2 条消息，2.1 **新增块**）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| NotifyAllowedEnergyTransfer | CSMS → CS | CSMS 通知充电桩允许的能量传输模式/方向 | **2.1 新增** |
| AFRRSignal | CSMS → CS | 自动频率恢复储备（AFRR）信号，用于快速电网平衡 | **2.1 新增** |

### 2.18 Block R — DERControl（6 条消息，2.1 **新增块**）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| GetDERControl | CSMS → CS | 查询充电桩 DER 控制设置（由 ReportDERControl 应答） | **2.1 新增** |
| SetDERControl | CSMS → CS | 安装/更新 DER 控制（曲线/设定点） | **2.1 新增** |
| ClearDERControl | CSMS → CS | 清除 DER 控制设置 | **2.1 新增** |
| ReportDERControl | CS → CSMS | 上报 DER 控制设置 | **2.1 新增** |
| NotifyDERAlarm | CS → CSMS | 上报 DER 电网保护告警/降额事件 | **2.1 新增** |
| NotifyDERStartStop | CS → CSMS | 上报 DER 控制功能启动/停止 | **2.1 新增** |

### 2.19 Block S — BatterySwap（2 条消息，2.1 **新增块**）

| 报文名称 | 请求方向 | 说明 | 2.1 状态 |
| :--- | :---: | :--- | :---: |
| BatterySwap | CS → CSMS | 上报电池交换事件（插入/移除、电池 ID 与状态） | **2.1 新增** |
| RequestBatterySwap | CSMS → CS | 请求执行电池交换 | **2.1 新增** |

---

## 三、报文详细字段

> 本节列出所有 91 条消息的 Request / Response 字段详情。
>
> - **64 条保留消息**（同 2.0.1 部分）：字段直接复用 2.0.1 规范。为避免重复，本节仅提供**功能块迁引表**；完整字段详情参阅 [`crates/ocpp-2-0-1/docs/MESSAGES.md`](../../ocpp-2-0-1/docs/MESSAGES.md)。
> - **27 条 2.1 新增消息**：提供名称、方向、用途、高层结构概述；详细字段级 Schema 待 2.1 JSON Schema 全部公开后补充（以 OCA 官方 draft-06 JSON Schema 为权威来源）。

---

### 3.1 保留消息（同 2.0.1）

OCPP 2.1 **完整保留 OCPP 2.0.1 的全部 64 条消息**，字段名、方向、枚举值完全一致。为避免文档重复，本节的 64 条消息不再展开详情，请参阅：

> **📘 权威参考**: `crates/ocpp-2-0-1/docs/MESSAGES.md` — 包含全部 64 条保留消息的完整 Request/Response JSON 示例、字段约束、枚举值表。

#### 3.1.1 保留消息索引

下表列出所有 64 条保留消息在 2.1 中的归属功能块。

**2.0.1 章节号列说明**：
- `3.x` = 该消息在 [`crates/ocpp-2-0-1/docs/MESSAGES.md`](../../ocpp-2-0-1/docs/MESSAGES.md) 的「三、报文详细字段」小节有完整 Request/Response JSON 示例，章节号即其子标题编号
- `3.*` = 该消息在 2.0.1 文档的"**二、报文总览**"节有简要说明（所属功能块 + 请求方向 + 用途），但**未展开字段详情**；如需字段级详情，请参阅 [OCA 官方 OCPP 2.0.1 JSON Schema](https://www.openchargealliance.org/protocols/ocpp-2-0-1/)

| 2.0.1 章节 | 报文名称 | 2.1 功能块 | 备注 |
|:-:|---|:-:|---|
| 3.1 | Authorize | C - Authorization | — |
| 3.2 | BootNotification | B - Provisioning | — |
| 3.3 | Heartbeat | B - Provisioning | — |
| 3.4 | TransactionEvent | E - Transactions | — |
| 3.5 | StatusNotification | G - Availability | — |
| 3.6 | MeterValues | J - MeterValues | — |
| 3.7 | SecurityEventNotification | A - Security | — |
| 3.8 | ClearCache | C - Authorization | — |
| 3.9 | CostUpdated | **I - TariffAndCost** | ⚠️ 从 2.0.1 Block O 迁至 I |
| 3.10 | GetTransactionStatus | E - Transactions | — |
| 3.11 | RequestStartTransaction | F - RemoteControl | — |
| 3.12 | RequestStopTransaction | F - RemoteControl | — |
| 3.13 | Reset | **B - Provisioning** | ⚠️ 从 2.0.1 RemoteControl 迁至 B |
| 3.14 | ChangeAvailability | G - Availability | — |
| 3.15 | TriggerMessage | F - RemoteControl | — |
| 3.16 | UnlockConnector | F - RemoteControl | — |
| 3.17 | GetVariables | B - Provisioning | — |
| 3.18 | SetVariables | B - Provisioning | — |
| 3.19 | DataTransfer | P - DataTransfer | — |
| 3.* | GetBaseReport | B - Provisioning | 2.0.1 总览§2.3 提及 |
| 3.* | GetReport | B - Provisioning | 2.0.1 总览§2.3 提及 |
| 3.* | NotifyReport | B - Provisioning | 2.0.1 总览§2.3 提及 |
| 3.* | SetNetworkProfile | B - Provisioning | 2.0.1 总览§2.3 提及 |
| 3.* | ReserveNow | H - Reservation | 2.0.1 总览§2.7 提及 |
| 3.* | CancelReservation | H - Reservation | 2.0.1 总览§2.7 提及 |
| 3.* | ReservationStatusUpdate | H - Reservation | 2.0.1 总览§2.7 提及 |
| 3.* | SendLocalList | D - LocalAuthList | 2.0.1 总览§2.9 提及 |
| 3.* | GetLocalListVersion | D - LocalAuthList | 2.0.1 总览§2.9 提及 |
| 3.* | SetChargingProfile | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | ClearChargingProfile | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | GetChargingProfiles | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | GetCompositeSchedule | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | ReportChargingProfiles | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | NotifyChargingLimit | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | ClearedChargingLimit | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | NotifyEVChargingNeeds | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | NotifyEVChargingSchedule | K - SmartCharging | 2.0.1 总览§2.4 提及 |
| 3.* | UpdateFirmware | L - Firmware | 2.0.1 总览§2.5 提及 |
| 3.* | FirmwareStatusNotification | L - Firmware | 2.0.1 总览§2.5 提及 |
| 3.* | GetLog | N - Diagnostics | 2.0.1 总览§2.5 提及 |
| 3.* | LogStatusNotification | N - Diagnostics | 2.0.1 总览§2.5 提及 |
| 3.* | PublishFirmware | L - Firmware | 2.0.1 总览§2.5 提及 |
| 3.* | PublishFirmwareStatusNotification | L - Firmware | 2.0.1 总览§2.5 提及 |
| 3.* | UnpublishFirmware | L - Firmware | 2.0.1 总览§2.5 提及 |
| 3.* | SetVariableMonitoring | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | ClearVariableMonitoring | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | SetMonitoringBase | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | SetMonitoringLevel | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | GetMonitoringReport | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | NotifyMonitoringReport | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | NotifyEvent | N - Diagnostics | 2.0.1 总览§2.6 提及 |
| 3.* | SignCertificate | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | CertificateSigned | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | InstallCertificate | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | DeleteCertificate | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | GetInstalledCertificateIds | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | GetCertificateStatus | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | Get15118EVCertificate | M - Certificates | 2.0.1 总览§2.8 提及 |
| 3.* | SetDisplayMessage | O - Display | 2.0.1 总览§2.10 提及 |
| 3.* | ClearDisplayMessage | O - Display | 2.0.1 总览§2.10 提及 |
| 3.* | GetDisplayMessages | O - Display | 2.0.1 总览§2.10 提及 |
| 3.* | NotifyDisplayMessages | O - Display | 2.0.1 总览§2.10 提及 |
| 3.* | CustomerInformation | N - Diagnostics | 2.0.1 总览§2.11 提及 |
| 3.* | NotifyCustomerInformation | N - Diagnostics | 2.0.1 总览§2.11 提及 |

### 3.2 保留消息的 2.1 兼容性

| 方面 | 行为 |
|---|---|
| JSON Schema | 仍为 draft-06，字段约束完全相同 |
| `customData` 扩展 | 保持不变，2.0.1 实现的扩展字段可直接用于 2.1 |
| `action` 名称 | 完全一致（如 `BootNotification`、`Authorize` 等） |
| 枚举值 | 完全一致（如 `AuthorizationStatusEnumType.Accepted` 等） |
| 功能块归属 | 部分消息**跨块迁移**（见上表 ⚠️ 标记），但报文本身不变 |
| 处理器代码 | 2.0.1 实现的 Handler 在 2.1 下 100% 兼容 |

---

## 四、2.1 新增消息详细字段

OCPP 2.1 共新增 **27 条消息**，分布在 6 个功能块中。本节详细列出每条消息的：

- 报文名称 / 所属功能块 / 方向 / 用途
- Request JSON 示例（带字段注释：字段名、类型、必填/可选、长度或范围约束、说明）
- Response JSON 示例
- 关键枚举类型及其值
- 与保留消息的交互关系

### 4.1 CostUpdated（从 2.0.1 Block O 迁至 Block I）

**方向**：CSMS → CS
**用途**：推送事务实时/最终费用。2.1 将此消息从 Display 块迁移到 TariffAndCost 块，以整合电价体系；**字段定义与 2.0.1 完全相同**。

**CostUpdated.req**：
```json
{
  "totalCost": 25.50,                 // number, 必填, 当前/最终费用（含税费）
  "transactionId": "TX-001"          // string(36), 必填, 事务 ID
}
```

**CostUpdated.conf**：空 payload `{}`

---

### 4.2 GetTariffs

**方向**：CSMS → CS
**用途**：查询充电桩上已安装的电价（default 或 driver tariff）

**GetTariffs.req**：
```json
{
  "evseId": 1                         // integer(>=0), 必填, 0=查询整站所有 EVSE
}
```

**GetTariffs.conf**：
```json
{
  "status": "Accepted",               // TariffGetStatusEnumType, 必填
  "statusInfo": {                     // StatusInfoType, 可选
    "reasonCode": "...",
    "additionalInfo": "..."
  },
  "tariffAssignments": [              // TariffAssignmentType[], 可选, 至少 1 个
    {
      "tariffId": "tariff-001",
      "tariffKind": "DefaultTariff",  // TariffKindEnumType
      "evseIds": [1, 2],              // integer[], 可选
      "idTokens": ["ABC123"],         // string[], 可选
      "validFrom": "2025-01-01T00:00:00Z"  // dateTime, 可选
    }
  ]
}
```

**TariffGetStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |
| NoTariff | 无电价 |

**TariffKindEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| DefaultTariff | 默认电价（按 EVSE 设置） |
| DriverTariff | 驾驶员电价（按 idToken 设置） |

---

### 4.3 SetDefaultTariff

**方向**：CSMS → CS
**用途**：设置 EVSE 默认电价（当没有驾驶员电价时使用）

**SetDefaultTariff.req**：
```json
{
  "evseId": 1,                        // integer(>=0), 必填, 0=对所有 EVSE
  "tariff": {                         // TariffType, 必填
    "tariffId": "tariff-001",
    "currency": "CNY"                 // ISO 4217 货币码
    // ... 其他 Tariff/Price 字段由 OCA schema 完整定义
  }
}
```

**SetDefaultTariff.conf**：
```json
{
  "status": "Accepted",               // TariffSetStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

**TariffSetStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |
| TooManyElements | 元素过多 |
| ConditionNotSupported | 条件不支持 |
| DuplicateTariffId | tariffId 重复 |

---

### 4.4 ChangeTransactionTariff

**方向**：CSMS → CS
**用途**：替**运行中事务**当前使用的电价

**ChangeTransactionTariff.req**：
```json
{
  "transactionId": "TX-001",          // string(36), 必填, 事务 ID
  "tariff": {                         // TariffType, 必填
    "tariffId": "tariff-peak",
    "currency": "CNY"
    // ... 完整 Tariff 字段
  }
}
```

**ChangeTransactionTariff.conf**：
```json
{
  "status": "Accepted",               // TariffChangeStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

**TariffChangeStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |
| TooManyElements | 元素过多 |
| ConditionNotSupported | 条件不支持 |
| TxNotFound | 事务不存在 |
| NoCurrencyChange | 不允许变更货币 |

---

### 4.5 ClearTariffs

**方向**：CSMS → CS
**用途**：清除充电桩已安装的电价

**ClearTariffs.req**：
```json
{
  "evseId": 1,                        // integer(>=0), 可选, 不指定则清所有 EVSE
  "tariffIds": ["tariff-001"]         // string[], 可选, 不指定则清该 EVSE 所有电价
}
```

**ClearTariffs.conf**：
```json
{
  "clearTariffsResult": [             // ClearTariffsResultType[], 必填
    {
      "status": "Accepted",           // TariffClearStatusEnumType
      "tariffId": "tariff-001",       // string(60), 可选, 无则 NoTariff 时不返回
      "statusInfo": {}
    }
  ]
}
```

**TariffClearStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已清除 |
| Rejected | 拒绝 |
| NoTariff | 无电价 |

---

### 4.6 NotifySettlement

**方向**：CS → CSMS
**用途**：上报事务最终结算/清算数据（含 VAT、收据 URL）

**NotifySettlement.req**：
```json
{
  "transactionId": "TX-001",          // string(36), 可选, 事务 ID
  "pspRef": "PSP-20250501-001",       // string(255), 必填, 支付 PSP 参考号
  "settlementAmount": 125.50,         // number, 必填, 结算金额
  "settlementTime": "2025-05-01T15:30:00Z",  // dateTime, 必填
  "status": "Settled",                // PaymentStatusEnumType, 必填
  "vatCompany": {                     // AddressType, 可选
    "name": "ABC Charging Ltd.",
    "address1": "Charging St. 123",
    "city": "Beijing",
    "country": "China"
  },
  "vatNumber": "CN123456789",         // string(20), 可选
  "receiptId": "R-20250501-001",     // string(50), 可选
  "receiptUrl": "https://...",        // string(2000), 可选
  "statusInfo": "..."                 // string(500), 可选, 支付终端附加信息
}
```

**NotifySettlement.conf**：
```json
{
  "receiptId": "CSMS-R-001",         // string(50), 可选, CSMS 生成的收据 ID
  "receiptUrl": "https://..."         // string(2000), 可选, CSMS 生成的收据 URL
}
```

**PaymentStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Settled | 已结算 |
| Canceled | 已取消 |
| Rejected | 拒绝 |
| Failed | 失败 |

---

### 4.7 NotifyWebPaymentStarted

**方向**：CSMS → CS
**用途**：通知充电桩网页/二维码支付已启动（充电桩应在超时前等待 PSP 结果）

**NotifyWebPaymentStarted.req**：
```json
{
  "evseId": 1,                        // integer(>=0), 必填, EVSE ID
  "timeout": 300                      // integer, 必填, 等待支付结果的超时(秒)
}
```

**NotifyWebPaymentStarted.conf**：空 payload `{}`

---

### 4.8 VatNumberValidation

**方向**：CS → CSMS
**用途**：验证客户 VAT 号码（用于发票开具，B2B 场景）

**VatNumberValidation.req**：
```json
{
  "vatNumber": "CN123456789",        // string(20), 必填, VAT 号
  "evseId": 1                         // integer(>=0), 可选
}
```

**VatNumberValidation.conf**：
```json
{
  "status": "Accepted",               // GenericStatusEnumType, 必填
  "vatNumber": "CN123456789",        // string(20), 必填, 原请求 VAT 号
  "statusInfo": {},                   // StatusInfoType, 可选
  "company": {                        // AddressType, 可选, 该公司注册信息
    "name": "ABC Charging Ltd.",
    "address1": "Charging St. 123",
    "city": "Beijing",
    "country": "China"
  },
  "evseId": 1                         // integer(>=0), 可选
}
```

**GenericStatusEnumType 枚举值**（多处复用）：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |

---

### 4.9 PullDynamicScheduleUpdate（Block K 新增）

**方向**：CS → CSMS
**用途**：充电桩主动向 CSMS 请求最新的动态调度设定点（pull-style 更新）

**PullDynamicScheduleUpdate.req**：
```json
{
  "chargingProfileId": 456            // integer, 必填, 动态充电曲线 ID
}
```

**PullDynamicScheduleUpdate.conf**：
```json
{
  "status": "Accepted",               // ChargingProfileStatusEnumType, 必填
  "scheduleUpdate": {                 // ChargingScheduleUpdateType, 可选
    "limit": 22000.0,                 // number, 充电限值 (W 或 A)
    "limit_L2": 7500.0,               // L2 相充电限值 (可选)
    "limit_L3": 7500.0,               // L3 相充电限值 (可选)
    "dischargeLimit": -5000.0,        // 放电限值 (负值, 可选)
    "setpoint": 15000.0,              // 功率设定点 (可选)
    "setpointReactive": 0.0,          // 无功设定点 (可选)
    // 各相 *_L2, *_L3 可选字段省略
  },
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

**ChargingProfileStatusEnumType 枚举值**（同 2.0.1）：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |

---

### 4.10 UpdateDynamicSchedule（Block K 新增）

**方向**：CSMS → CS
**用途**：CSMS 主动推送动态调度的新设定点（push-style 更新）

**UpdateDynamicSchedule.req**：
```json
{
  "chargingProfileId": 456,           // integer, 必填, 动态充电曲线 ID
  "scheduleUpdate": {                 // ChargingScheduleUpdateType, 必填
    "limit": 11000.0,
    "setpoint": 8000.0,
    "dischargeLimit": -3000.0
    // 其余字段可选
  }
}
```

**UpdateDynamicSchedule.conf**：
```json
{
  "status": "Accepted",               // ChargingProfileStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

---

### 4.11 NotifyPriorityCharging（Block K 新增）

**方向**：CS → CSMS
**用途**：上报事务的优先级充电状态（激活/停止）

**NotifyPriorityCharging.req**：
```json
{
  "transactionId": "TX-001",          // string(36), 必填, 事务 ID
  "activated": true                   // boolean, 必填, true=已激活, false=已停止
}
```

**NotifyPriorityCharging.conf**：空 payload `{}`

---

### 4.12 UsePriorityCharging（Block K 新增）

**方向**：CSMS → CS
**用途**：请求对事务启用/停用优先级充电（如司机请求快充满）

**UsePriorityCharging.req**：
```json
{
  "transactionId": "TX-001",          // string(36), 必填
  "activate": true                    // boolean, 必填, true=启用优先级充电
}
```

**UsePriorityCharging.conf**：
```json
{
  "status": "Accepted",               // PriorityChargingStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

**PriorityChargingStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |
| NoProfile | 无可用的优先级曲线 |

---

### 4.13 GetCertificateChainStatus（Block M 新增）

**方向**：CS → CSMS
**用途**：查询证书链中多张证书的吊销/有效状态（相比 2.0.1 的 GetCertificateStatus 支持批量查 CRL 或 OCSP）

**GetCertificateChainStatus.req**：
```json
{
  "certificateStatusRequests": [      // CertificateStatusRequestInfoType[], 必填, 1-4 个
    {
      "source": "OCSP",               // CertificateStatusSourceEnumType, 必填, "CRL"|"OCSP"
      "certificateHashData": {        // CertificateHashDataType, 必填
        "hashAlgorithm": "SHA256",
        "issuerNameHash": "sha256hash...",  // string(128)
        "issuerKeyHash": "sha256...",
        "serialNumber": "abc123"
      },
      "urls": [                       // string[], 必填, 1-5 个
        "https://ocsp.example.com"
      ]
    }
  ]
}
```

**GetCertificateChainStatus.conf**：
```json
{
  "certificateStatus": [              // CertificateStatusType[], 必填, 1-4 个
    {
      "certificateHashData": { /* 同上 */ },
      "source": "OCSP",
      "status": "Good",               // CertificateStatusEnumType
      "nextUpdate": "2025-06-01T00:00:00Z"  // dateTime, 必填, 下次更新时间
    }
  ]
}
```

**CertificateStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Good | 有效 |
| Revoked | 已吊销 |
| Unknown | 未知 |
| Failed | 失败 |

**CertificateStatusSourceEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| OCSP | Online Certificate Status Protocol |
| CRL | Certificate Revocation List |

---

### 4.14 OpenPeriodicEventStream（Block N 新增）

**方向**：CS → CSMS
**用途**：开启一条周期性事件流（高频遥测数据的"通道"）

**OpenPeriodicEventStream.req**：
```json
{
  "constantStreamData": {             // ConstantStreamDataType, 必填
    "id": 1,                          // integer(>=0), 必填, 流 ID
    "variableMonitoringId": 42,       // integer(>=0), 必填, 关联的变量监控 ID
    "params": {                       // PeriodicEventStreamParamsType, 必填
      "interval": 1.0,                // number, 采样间隔(秒, 可小数)
      "valuesPerInterval": 10         // integer, 每间隔采样数
    }
  }
}
```

**OpenPeriodicEventStream.conf**：
```json
{
  "status": "Accepted",               // GenericStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

---

### 4.15 ClosePeriodicEventStream（Block N 新增）

**方向**：CS → CSMS
**用途**：关闭已开启的周期性事件流

**ClosePeriodicEventStream.req**：
```json
{
  "id": 1                             // integer(>=0), 必填, 流 ID
}
```

**ClosePeriodicEventStream.conf**：空 payload `{}`

---

### 4.16 GetPeriodicEventStream（Block N 新增）

**方向**：CSMS → CS
**用途**：枚举充电桩上已开启的周期性事件流

**GetPeriodicEventStream.req**：空 payload `{}`

**GetPeriodicEventStream.conf**：
```json
{
  "constantStreamData": [             // ConstantStreamDataType[], 可选
    {
      "id": 1,
      "variableMonitoringId": 42,
      "params": { "interval": 1.0, "valuesPerInterval": 10 }
    }
  ]
}
```

---

### 4.17 AdjustPeriodicEventStream（Block N 新增）

**方向**：CSMS → CS
**用途**：调整已开启事件流的采样/上报参数

**AdjustPeriodicEventStream.req**：
```json
{
  "id": 1,                            // integer(>=0), 必填, 流 ID
  "params": {                         // PeriodicEventStreamParamsType, 必填
    "interval": 0.5,                  // 调整为 0.5 秒间隔
    "valuesPerInterval": 20
  }
}
```

**AdjustPeriodicEventStream.conf**：
```json
{
  "status": "Accepted",               // GenericStatusEnumType, 必填
  "statusInfo": {}
}
```

---

### 4.18 NotifyPeriodicEventStream（Block N 新增，SEND 单向）

**方向**：CS → CSMS
**消息类型**：**MessageTypeId = 6 (SEND)**，**单向 fire-and-forget**，**无响应**
**用途**：高频推送遥测数据。使用 SEND 而非 CALL 以避免占用唯一的 "outstanding CALL" 槽位，允许 1Hz/10Hz 级采样频率而不阻塞其他业务消息。

**NotifyPeriodicEventStream.req**（payload）：
```json
{
  "id": 1,                            // integer(>=0), 必填, 流 ID
  "basetime": "2025-05-01T15:00:00Z", // dateTime, 必填, 基准时间戳
  "pending": 0,                       // integer(>=0), 必填, 待发送的后续数据元素数
  "data": [                           // StreamDataElementType[], 必填, 至少 1 个
    {
      "t": 0.0,                       // number, 必填, 相对 basetime 的偏移(秒)
      "v": "150.5"                    // string(2500), 必填, 采样值
    },
    { "t": 0.1, "v": "150.7" },
    { "t": 0.2, "v": "150.9" }
  ]
}
```

**NotifyPeriodicEventStream.conf**：**无响应**（MessageTypeId=6 单向 SEND，CSMS 不回复）

> **关键特性**: SEND 是 2.1 唯一支持单向流的机制。如果充电桩发送 SEND，CSMS 收到即处理，**不得**回复 CALLRESULT/CALLERROR。错误处理通过带外通道（如 NotifyEvent 报告 SEND 失败）。

---

### 4.19 NotifyAllowedEnergyTransfer（Block Q 新增）

**方向**：CSMS → CS
**用途**：CSMS 告诉充电桩允许的能量传输模式/方向（V2X 双向充电启用时）

**NotifyAllowedEnergyTransfer.req**：
```json
{
  "transactionId": "TX-001",          // string(36), 必填, 事务 ID
  "allowedEnergyTransfer": [          // EnergyTransferModeEnumType[], 必填, 至少 1 个
    "AC_single_phase",
    "DC"
    // 其他可选: AC_three_phase, AC_BPT, DC_BPT
  ]
}
```

**NotifyAllowedEnergyTransfer.conf**：
```json
{
  "status": "Accepted",               // NotifyAllowedEnergyTransferStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

**NotifyAllowedEnergyTransferStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |

**EnergyTransferModeEnumType 枚举值**（2.1 扩充，含双向）：
| 值 | 说明 |
|---|---|
| AC_single_phase | 单相 AC |
| AC_three_phase | 三相 AC |
| DC | 直流 |
| AC_BPT | AC 双向功率传输 (Bidirectional Power Transfer) |
| DC_BPT | DC 双向功率传输 |

---

### 4.20 AFRRSignal（Block Q 新增）

**方向**：CSMS → CS
**用途**：推送自动频率恢复储备信号（AFRR，用于 V2X 参与电网快速平衡）

**AFRRSignal.req**：
```json
{
  "signal": 100,                      // integer, 必填, AFRR 信号值（参考 v2xSignalWattCurve）
  "timestamp": "2025-05-01T15:30:12Z" // dateTime, 必填, 信号生效时间
}
```

**AFRRSignal.conf**：
```json
{
  "status": "Accepted",               // GenericStatusEnumType, 必填
  "statusInfo": {}
}
```

---

### 4.21 GetDERControl（Block R 新增）

**方向**：CSMS → CS
**用途**：查询充电桩已安装的 DER 控制设置（由 ReportDERControl 应答）

**GetDERControl.req**：
```json
{
  "requestId": 123,                   // integer, 必填, 请求 ID（关联 ReportDERControl）
  "controlId": "ctrl-001",            // string(36), 可选, 特定控制 ID
  "controlType": "VoltWatt",          // DERControlEnumType, 可选
  "isDefault": false                  // boolean, 可选, true=默认; false=调度
}
```

**GetDERControl.conf**：
```json
{
  "status": "Accepted",               // DERControlStatusEnumType, 必填
  "statusInfo": {}                    // StatusInfoType, 可选
}
```

**DERControlStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Rejected | 拒绝 |
| Unknown | 未知 |
| NotSupported | 不支持 |

---

### 4.22 SetDERControl（Block R 新增）

**方向**：CSMS → CS
**用途**：安装/更新 DER 控制设置（曲线或设定点）

**SetDERControl.req**：
```json
{
  "controlId": "ctrl-001",            // string(36), 必填
  "controlType": "VoltWatt",          // DERControlEnumType, 必填
  "isDefault": false,                 // boolean, 必填
  "curve": [                          // DERCurveType[], 可选, 10 个曲线点
    // 曲线设置详见 OCA schema
  ],
  "enterService": {},                 // EnterServiceType, 可选
  "fixedVar": {},                     // FixedVarType, 可选
  "freqDroop": {},                    // FreqDroopType, 可选
  "gradient": {},                     // GradientType, 可选
  "limitMaxDischarge": {}             // LimitMaxDischargeType, 可选
  // 各 DER 控制参数按需选填
}
```

**SetDERControl.conf**：
```json
{
  "status": "Accepted",               // DERControlStatusEnumType, 必填
  "statusInfo": {},
  "supersededIds": ["ctrl-old-1"]     // string[], 可选, 被新控制替代的旧 ID (最多 24 个)
}
```

---

### 4.23 ClearDERControl（Block R 新增）

**方向**：CSMS → CS
**用途**：清除 DER 控制设置

**ClearDERControl.req**：
```json
{
  "isDefault": false,                 // boolean, 必填
  "controlId": "ctrl-001",            // string(36), 可选, 不指定则清全部
  "controlType": "VoltWatt"           // DERControlEnumType, 可选
}
```

**ClearDERControl.conf**：
```json
{
  "status": "Accepted",               // DERControlStatusEnumType, 必填
  "statusInfo": {}
}
```

---

### 4.24 ReportDERControl（Block R 新增）

**方向**：CS → CSMS
**用途**：上报 DER 控制设置（回应 GetDERControl 请求）

**ReportDERControl.req**：
```json
{
  "requestId": 123,                   // integer, 必填, 关联 GetDERControl 的 requestId
  "curve": [],                        // DERCurveGetType[], 可选, 最多 24 个
  "enterService": [],
  "fixedVar": [],
  "freqDroop": [],
  "gradient": [],
  "limitMaxDischarge": [],
  "tbc": false                        // boolean, 可选, To Be Continued (分页)
}
```

**ReportDERControl.conf**：空 payload `{}`

---

### 4.25 NotifyDERAlarm（Block R 新增）

**方向**：CS → CSMS
**用途**：上报 DER 电网保护告警/降额事件

**NotifyDERAlarm.req**：
```json
{
  "controlType": "VoltWatt",          // DERControlEnumType, 必填
  "timestamp": "2025-05-01T15:30:12Z",// dateTime, 必填, 告警开始/结束时间
  "alarmEnded": false,                // boolean, 可选, true=告警结束
  "extraInfo": "ISO 15118-20 trip",   // string(200), 可选
  "gridEventFault": "UnderVoltage"    // GridEventFaultEnumType, 可选
}
```

**NotifyDERAlarm.conf**：空 payload `{}`

**GridEventFaultEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| CurrentImbalance | 电流不平衡 |
| LocalEmergency | 本地紧急事件 |
| LowInputPower | 输入功率过低 |
| OverCurrent | 过流 |
| OverFrequency | 过频 |
| OverVoltage | 过压 |
| PhaseRotation | 相位旋转 |
| RemoteEmergency | 远程紧急事件 |
| UnderFrequency | 欠频 |
| UnderVoltage | 欠压 |
| VoltageImbalance | 电压不平衡 |

---

### 4.26 NotifyDERStartStop（Block R 新增）

**方向**：CS → CSMS
**用途**：上报 DER 控制功能的启动/停止

**NotifyDERStartStop.req**：
```json
{
  "controlId": "ctrl-001",            // string(36), 必填
  "started": true,                    // boolean, 必填, true=启动, false=停止
  "timestamp": "2025-05-01T15:30:00Z",// dateTime, 必填
  "supersededIds": ["ctrl-old-1"]     // string[], 可选, 被替代的旧 ID (最多 24 个)
}
```

**NotifyDERStartStop.conf**：空 payload `{}`

---

### 4.27 BatterySwap（Block S 新增）

**方向**：CS → CSMS
**用途**：上报电池交换事件（插入/移除/超时）

**BatterySwap.req**：
```json
{
  "idToken": {                        // IdTokenType, 必填
    "idToken": "SWAP-USER-001",
    "type": "Central"
  },
  "eventType": "BatteryIn",           // BatterySwapEventEnumType, 必填
  "requestId": 42,                    // integer, 必填, 关联 RequestBatterySwap
  "batteryData": [                    // BatteryDataType[], 必填, 至少 1 个
    {
      "evseId": 1,                    // integer(>=0), 槽位号
      "serialNumber": "BAT-12345",    // string(50), 电池序列号
      "soC": 85.5,                    // number(0-100), SoC(%)
      "soH": 92.3,                    // number(0-100), SoH(%)
      "productionDate": "2024-01-01T00:00:00Z",
      "vendorInfo": "..."             // string(500), 可选
    }
  ]
}
```

**BatterySwap.conf**：空 payload `{}`

**BatterySwapEventEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| BatteryIn | 电池已插入 |
| BatteryOut | 电池已取出 |
| BatteryOutTimeout | 用户未在超时内取出电池 |

---

### 4.28 RequestBatterySwap（Block S 新增）

**方向**：CSMS → CS
**用途**：请求充电桩执行电池交换

**RequestBatterySwap.req**：
```json
{
  "idToken": {                        // IdTokenType, 必填
    "idToken": "SWAP-USER-001",
    "type": "Central"
  },
  "requestId": 42                     // integer, 必填, 匹配后续 BatterySwap 事件
}
```

**RequestBatterySwap.conf**：
```json
{
  "status": "Accepted",               // GenericStatusEnumType, 必填
  "statusInfo": {}
}
```



---

## 五、2.1 新增报文交互流程

针对 2.1 新增的 4 个核心能力，描述其典型报文交换顺序。

### 5.1 电价设置与结算流程（Block I）

```
[启动]
CSMS ──SetDefaultTariff──► CS       (按 EVSE 安装默认电价)
     ◄─Status=Accepted──┘
CSMS ──GetTariffs──► CS              (校验已安装电价)
     ◄─Tariff[]──────┘

[事务开始]
CS ──Authorize──► CSMS
    ◄─idTokenInfo──┘
CS ──TransactionEvent(Started)──► CSMS

[事务中动态调整]
CSMS ──ChangeTransactionTariff──► CS
       ◄─Status=Accepted─────────┘
CSMS ──CostUpdated──► CS            (实时费用推送至显示屏)

[Web/QR 支付]
CSMS ──NotifyWebPaymentStarted──► CS  (通知充电桩网页支付启动)

[事务结束 + 结算]
CS ──TransactionEvent(Ended)──► CSMS
    ◄─idTokenInfo.totalCost──────┘
CS ──NotifySettlement──► CSMS       (上报最终结算数据)
CS ──VatNumberValidation──► CSMS    (如客户需要发票)
    ◄─status=Accepted────────────┘

[电价清理]
CSMS ──ClearTariffs──► CS
       ◄─Status=Accepted──┘
```

### 5.2 DER 控制流程（Block R）

```
CSMS ──SetDERControl──► CS   (如：安装 volt-watt / freq-watt 曲线)
       ◄─Status=Accepted──┘

[电网事件触发]
CS ──NotifyDERAlarm──► CSMS                 (告警/限电事件)
CS ──NotifyDERStartStop──► CSMS             (DER 函数启动/停止)

[查询]
CSMS ──GetDERControl──► CS
       ◄─ReportDERControl[]───────────────┘

[清理]
CSMS ──ClearDERControl──► CS
       ◄─Status=Accepted──┘
```

### 5.3 双向 / V2X 充电流程（Block Q）

```
[事务启动：EV 接入 + 已授权]
CS ──Authorize (idToken)──► CSMS
    ◄─idTokenInfo.status=Accepted──┘
CS ──TransactionEvent(Started, triggerReason=Authorized)──► CSMS

[启用 V2X 放/充电权限]
CSMS ──NotifyAllowedEnergyTransfer──► CS
       (指定允许的能量方向: ImportToEV/ExportToGrid/Both)
       ◄─Status=Accepted──────────────┘

[事务中：CSMS 推 AFRR 信号]
CSMS ──AFRRSignal────► CS                (快速频率恢复储备指令)
       ◄─Status=Accepted────┘
       (CS 调整充/放电功率参与电网平衡)

[动态调度更新 (SmartCharging 配合)]
CS ──PullDynamicScheduleUpdate──► CSMS
    ◄─ChargingScheduleUpdate─────┘
   or
CSMS ──UpdateDynamicSchedule──► CS
       ◄─Status=Accepted──────────┘

[事务结束]
CS ──TransactionEvent(Ended)──► CSMS
```

### 5.4 周期性事件流（高频遥测，Block N）

> **关键特性**: `NotifyPeriodicEventStream` 使用 **MessageTypeId=6 (SEND)** —— 单向消息，无需响应。这使高频遥测（如 100Hz 电流采样）不会占用唯一的 "outstanding CALL" 槽位。

```
[流开启]
CS ──OpenPeriodicEventStream──► CSMS  (声明流 ID、采样率、测量量)
    ◄─Status=Accepted────────────—┘

[高频数据推送]
CS ──SEND NotifyPeriodicEventStream──► CSMS   (单向, 无响应, 高频)
CS ──SEND NotifyPeriodicEventStream──► CSMS
CS ──SEND NotifyPeriodicEventStream──► CSMS
...

[CSMS 调整采样参数]
CSMS ──AdjustPeriodicEventStream──► CS  (调整采样率/测量量)
       ◄─Status=Accepted────────────────┘

[查询当前流]
CSMS ──GetPeriodicEventStream──► CS
       ◄─PeriodicEventStream[]──────—┘

[流结束]
CS ──ClosePeriodicEventStream──► CSMS
    ◄─Status=Accepted────────────—┘
```

### 5.5 电池交换流程（Block S）

```
CSMS ──RequestBatterySwap──► CS   (请求执行交换)
       ◄─Status=Accepted────┘

[物理交换动作：取出旧电池 + 插入新电池]

CS ──BatterySwap──► CSMS         (上报交换事件)
    (内含: oldBatteryId / newBatteryId / batteryStateOfHealth /
           transactionId / idToken 等)
    ◄─Status=Accepted──────—┘
```

### 5.6 优先级充电流程（Block K 新增）

```
[事务进行中]
CSMS ──UsePriorityCharging──► CS   (如：司机请求充电加速)
       ◄─Status=Accepted────┘

[CS 上报优先级充电状态]
CS ──NotifyPriorityCharging──► CSMS
```

### 5.7 动态调度更新（Block K 新增）

```
[方向 1: 充电装主动拉取]
CS ──PullDynamicScheduleUpdate──► CSMS  (请求新的 setpoint)
    ◄─ChargingScheduleUpdate───────┘
CSMS 返回后，CS 应用新的动态调度

[方向 2: CSMS 主动推送]
CSMS ──UpdateDynamicSchedule──► CS
       ◄─Status=Accepted──────—┘
```

---

## 六、错误处理

### 6.1 CALLERROR (MessageTypeId = 4)

与 2.0.1 完全相同：

```json
[4, "<messageId>", "<errorCode>", "<errorDescription>", { "errorDetails" }]
```

**ErrorCode 枚举值**（同 2.0.1）：
| 值 | 说明 |
|---|---|
| NotImplemented | 未实现 |
| NotSupported | 不支持 |
| InternalError | 内部错误 |
| ProtocolError | 协议错误 |
| SecurityError | 安全错误 |
| FormatViolationError | 格式错误（2.1 沿用此拼写） |
| PropertyConstraintViolation | 属性约束违规 |
| OccurrenceConstraintViolation | 出现约束违规 |
| TypeConstraintViolation | 类型约束违规 |
| GenericError | 通用错误 |

### 6.2 CALLRESULTERROR (MessageTypeId = 5，2.1 新增)

**用途**：当接收方成功收到 CALLRESULT，但**无法处理其 payload**（例如：JSON 字段缺失、枚举值未知、业务约束不满足）时发送。

```json
[5, "<messageId>", "<errorCode>", "<errorDescription>", { "errorDetails" }]
```

> **与 CALLERROR 的区别**：
> - `CALLERROR` = 响应方根本生不出合法的 CALLRESULT（协议层错误）
> - `CALLRESULTERROR` = 响应方已经发出 CALLRESULT，但请求方无法处理（业务层错误）

**ErrorCode 枚举值与 CALLERROR 完全一致（见 §6.1）**：

| 值 | 说明 |
|---|---|
| NotImplemented | 未实现 |
| NotSupported | 不支持 |
| InternalError | 内部错误 |
| ProtocolError | 协议错误 |
| SecurityError | 安全错误 |
| FormatViolationError | 格式错误 |
| PropertyConstraintViolation | 属性约束违规 |
| OccurrenceConstraintViolation | 出现约束违规 |
| TypeConstraintViolation | 类型约束违规 |
| GenericError | 通用错误 |

---

## 七、2.0.1 → 2.1 迁移指南

### 7.1 新增消息类型（MessageTypeId）

| MessageTypeId | 2.0.1 | 2.1 | 处理建议 |
|---|---|---|---|
| 2 (CALL) | ✓ | ✓ | 兼容 |
| 3 (CALLRESULT) | ✓ | ✓ | 兼容 |
| 4 (CALLERROR) | ✓ | ✓ | 兼容 |
| **5 (CALLRESULTERROR)** | ✗ | ✓ | 请求方需识别并记录此类型（无法处理 CALLRESULT 时） |
| **6 (SEND)** | ✗ | ✓ | 接收方需处理 fire-and-forget 单向消息（仅 NotifyPeriodicEventStream 使用） |

### 7.2 子协议协商

`Sec-WebSocket-Protocol` 头中的子协议标识扩展为 2.1。兼容 2.0.1 的站应这样协商：

```http
# Charging Station 发起连接（同时支持 2.1/2.0.1/1.6）
Sec-WebSocket-Protocol: ocpp2.1, ocpp2.0.1, ocpp1.6

# CSMS 选择最高兼容版本应答
Sec-WebSocket-Protocol: ocpp2.1
```

### 7.3 功能块迁移表（重要）

2.1 对 64 条保留消息做了少量跨功能块迁移，但报文本身完全不变：

| 报文 | 2.0.1 块 | 2.1 块 | 影响 |
|---|---|---|---|
| Reset | F (RemoteControl) | B (Provisioning) | 归入设备供给/生命周期管理 |
| CostUpdated | O (Display) | I (TariffAndCost) | 归入电价体系 |
| Monitoring 系列（9 消息） | 独立 Monitoring 块 | N (Diagnostics) | 与 Log/Event/Customer 统一归入诊断体系 |
| CustomerInformation | 独立 Customer Information 块 | N (Diagnostics) | 归入诊断体系 |
| NotifyCustomerInformation | 独立 Customer Information 块 | N (Diagnostics) | 归入诊断体系 |

> **说明**：2.0.1 OCA 官方规范中，Monitoring 和 Customer Information 是无字母编号的独立功能块；2.1 把这些消息整合进字母编号 N (Diagnostics) 块。2.0.1 中属于 A-P 字母编号的"Authorization (C)"块与这两个块无关——迁移仅涉及分类，消息本身定义不变。

> **实现影响**：如果 2.0.1 中的 Handler 是按功能块组织的（如 `handlers/remote_control/reset.rs`），迁移 2.1 时只需移动文件即可；消息定义本身无需任何修改。

### 7.4 新能力启用检查

以下 2.1 能力可通过设备模型变量查询是否启用：
| Component | Variable | 说明 |
|---|---|---|
| TxCtrlr | TariffCostCtrlrExists | 是否支持 TariffCost |
| DisplayMessageCtrlr | PersonalMessageSize | 个人消息容量 |
| AlignedDataCtrlr | Interval | Block I 相关 |
| SmartChargingCtrlr | ACDelayPerLevel | 优先级充电延迟 |
| V2XCtrlr (new) | Enabled | V2X 是否启用 |
| DERCtrlr (new) | Enabled | DER 是否启用 |
| BatterySwapCtrlr (new) | Enabled | 电池交换是否启用 |

### 7.5 OCPP 1.6 → 2.1 报文映射

OCPP 2.1 保留 1.6 的迁移路径（沿用 2.0.1 的改设计）。从 1.6 迁移到 2.1 时，请同时参考 `crates/ocpp-2-0-1/docs/MESSAGES.md` §七「1.6 消息到 2.0.1 映射」。下表列出 1.6 到 2.1 的对应关系：

| OCPP 1.6 | OCPP 2.1 等价物 | 所属 2.1 块 |
|---|---|---|
| StartTransaction | TransactionEvent (eventType=Started) | E |
| StopTransaction | TransactionEvent (eventType=Ended) | E |
| MeterValues（事务中） | TransactionEvent (eventType=Updated, 含 meterValue) | E |
| MeterValues（事务外） | MeterValues | J |
| GetConfiguration | GetVariables / GetBaseReport | B |
| ChangeConfiguration | SetVariables | B |
| RemoteStartTransaction | RequestStartTransaction | F |
| RemoteStopTransaction | RequestStopTransaction | F |
| ChangeAvailability | ChangeAvailability | G |
| GetDiagnostics | GetLog | N |
| DiagnosticsStatusNotification | LogStatusNotification | N |
| status (ChargePointStatus) | connectorStatus (ConnectorStatusEnumType) | G |
| ChargePointErrorCode | ⚠️ 1.6 已移除（改用 SecurityEventNotification） | A |
| DataTransfer | DataTransfer | P |

> **2.1 独有概念**（1.6 / 2.0.1 均无对应）：
> - 电价体系（Block I 全部 8 条）
> - V2X / 双向（Block Q 全部 2 条）
> - DER 控制（Block R 全部 6 条）
> - 电池交换（Block S 全部 2 条）
> - 优先级充电 + 动态调度（Block K 4 条新消息）
> - 周期性事件流（Block N 5 条新消息）
> - 证书链查询（Block M +1 条新消息）

---

## 八、实战实现优先级

### 8.1 P0 - 必实现（核心 + 2.1 关键能力）

```
# 64 条保留消息（完全复用 2.0.1 实现）
1. BootNotification / Heartbeat / StatusNotification       # 启动/心跳/状态
2. Authorize                                                # 鉴权
3. TransactionEvent / GetTransactionStatus                  # 事务
4. MeterValues                                              # 电能
5. RequestStartTransaction / RequestStopTransaction         # 远程启停
6. ClearCache / Reset / ChangeAvailability / UnlockConnector  # 管理
7. DataTransfer                                             # 厂商扩展

# 2.1 关键新增
8. NotifyAllowedEnergyTransfer + AFRRSignal                 # V2X 基础
9. GetTariffs + SetDefaultTariff + ClearTariffs             # 电价基础
10. CostUpdated (原 2.0.1)                                  # 费用推送
```

### 8.2 P1 - 建议实现（完整 2.1 能力）

```
11. ChangeTransactionTariff / NotifySettlement              # 完整电价流
12. NotifyWebPaymentStarted / VatNumberValidation           # Web 支付
13. GetDERControl / SetDERControl / ClearDERControl         # DER 基础
14. ReportDERControl / NotifyDERAlarm / NotifyDERStartStop  # DER 报告
15. PullDynamicScheduleUpdate + UpdateDynamicSchedule       # 动态调度
16. NotifyPriorityCharging + UsePriorityCharging            # 优先级充电
17. GetCertificateChainStatus                               # 证书链
```

### 8.3 P2 - 按需实现

```
18. OpenPeriodicEventStream / ClosePeriodicEventStream      # 周期性事件流
19. GetPeriodicEventStream / AdjustPeriodicEventStream
20. NotifyPeriodicEventStream (SEND, 单向)
21. BatterySwap / RequestBatterySwap                        # 电池交换站
```

---

## 九、2.1 关键配置变量（Component/Variable 模型）

OCPP 2.1 沿用 2.0.1 的 Component/Variable 设备模型。2.0.1 已有的所有组件/变量**完全保留**（参阅 [`crates/ocpp-2-0-1/docs/MESSAGES.md`](../../ocpp-2-0-1/docs/MESSAGES.md) §五 关键配置变量）。

2.1 共定义 **82 个 Component / 438 个 Component/Variable 配对**。本节仅聚焦 **2.1 新增的 Component 和变量**，便于迁移参考。

### 9.1 2.1 新增 Component

| Component | 所属 2.1 Block | 用途 |
|---|---|---|
| **BatterySwapCtrlr** | S (BatterySwap) | 电池交换站配置 |
| **ACDERCtrlr** | R (DERControl) | 通过 ISO 15118-20 ChargeLoop 让 EV 逆变器模拟 DER 能力 |
| **DCDERCtrlr** | R (DERControl) | EVSE DC 逆变器本身的 DER 能力（nameplate 信息） |
| **V2XChargingCtrlr** | Q (Bidirectional) | V2X 充/放电配置（位于 EVSE 层级） |
| **TariffCostCtrlr** | I (TariffAndCost) | 电价与成本显示配置 |
| **PaymentCtrlr** | I (TariffAndCost) | 支付终端配置 |
| **WebPaymentsCtrlr** | I (TariffAndCost) | 动态二维码 / 临时支付配置 |
| **ConnectedEV** | Q (Bidirectional) | 通过 ISO 15118 / CHAdeMO 接收的车辆信息（含 VehicleID、Certificate 链、SoC、DepartureTime 等） |

### 9.2 2.1 新增的关键变量

| Component | Variable | 类型 | 单位 | 说明 |
|---|---|---|---|---|
| **BatterySwapCtrlr** | TargetSoC | integer | % | 电池需达到的 SoC 才可被交换 |
| | MaxSoc | integer | % | 最大可充至 SoC |
| | IdToken | string | — | 用于电池交换事务的 idToken |
| | Timeout (In / Out 实例) | integer | s | 授权后插入/移除电池的超时 |
| **ACDERCtrlr** | ModesSupported | MemberList | — | 支持的控制模式（Volt-Watt、Freq-Watt 等） |
| **DCDERCtrlr** | Enabled | boolean | — | DC DER 控制是否启用 |
| | MaxW | decimal | W | 单位功率因数下有功功率额定 |
| | OverExcitedW / OverExcitedPF | decimal | W / — | 过激功率 / 功率因数 |
| | UnderExcitedW / UnderExcitedPF | decimal | W / — | 欠激功率 / 功率因数 |
| | MaxVA | decimal | VA | 最大视在功率 |
| | MaxVar / MaxVarNeg | decimal | Var | 最大注入 / 吸收无功功率 |
| | MaxChargeRateW / MaxChargeRateVA | decimal | W / VA | 最大充电功率 |
| | VNom / MaxV / MinV | decimal | V | 额定 / 最大 / 最小 AC 电压 |
| | ModesSupported | MemberList | — | 支持的 DER 控制模式 |
| | IslandingDetectionMethod | OptionList | — | 防孤岛检测方法 |
| | IslandingDetectionTripTime | decimal | s | 检测到孤岛后的跳闸时间 |
| | ReactiveSusceptance | decimal | s | 停能跳闸状态下仍接入的无功电纳 |
| **V2XChargingCtrlr** | Enabled | boolean | — | V2X 充/放电是否启用 |
| **TariffCostCtrlr** | Enabled | boolean | — | TariffCost 功能是否启用 |
| | Currency | string | — | 货币（ISO 4217，如 CNY / USD / EUR） |
| **WebPaymentsCtrlr** | Available | boolean | — | 是否支持动态 QR 码临时支付 |
| | Enabled | boolean | — | 功能是否启用 |
| **PaymentCtrlr** | Enabled | boolean | — | 支付终端是否启用 |
| **ConnectedEV** | VehicleID | string | — | EVCCID（来自 ISO 15118 SessionSetupReq） |
| | VehicleCertificate (Leaf / SubCA1 / SubCA2 / Root 实例) | string | — | PEM X.509 证书 |
| | ACCurrent (MinSet / MaxSet) | decimal | A | EV 最小/最大 AC 电流 |
| | DCCurrent (MinSet / MaxSet / Target) | decimal | A | EV 最小/最大/目标 DC 电流 |
| | DCVoltage (MinSet / MaxSet / Target) | decimal | V | EV 最小/最大/目标 DC 电压 |
| | Power (MaxSet) | decimal | W | EV 最大充电功率 |
| | DischargePower (MaxSet) | decimal | W | EV 最大放电功率 |
| | EnergyImport (MinSet / MaxSet / Target) | decimal | Wh | EV 最小/最大/目标充电能量 |
| | BatteryCapacity | decimal | Wh | EV 电池容量 |
| | DepartureTime | dateTime | — | 计划出发时间 |
| | StateOfCharge | integer | % | 当前 SoC；MaxSet 实例表示满电 SoC |
| | ChargingState | OptionList | — | EV 充电状态（含错误码 EVTerminationCode） |

### 9.3 2.1 新增的 Block R DERControl 启用检查

在 SetDERControl 之前，建议先通过 GetVariables 查询以下变量，确认充电桩 DER 能力：

```
# AC EVSE 通过 ISO 15118-20 控制 EV 逆变器
Component: ACDERCtrlr
  Variable: ModesSupported    # 支持的模式列表

# DC EVSE 自带逆变器
Component: DCDERCtrlr
  Variable: Enabled           # 是否启用
  Variable: MaxW              # 额定有功
  Variable: ModesSupported    # 支持的模式列表
  Variable: MaxVA             # 额定视在功率
  Variable: MaxVar            # 最大注入无功
  Variable: MaxVarNeg         # 最大吸收无功
```

### 9.4 2.1 新增变量启用查询（汇总）

| 功能 | Component | Variable | 默认 |
|---|---|---|---|
| V2X 双向 | V2XChargingCtrlr | Enabled | false |
| DER 控制 | DCDERCtrlr | Enabled | false |
| 电池交换站 | BatterySwapCtrlr | （任一变量存在即表示支持） | — |
| 电价成本 | TariffCostCtrlr | Enabled | false |
| 网页/二维码支付 | WebPaymentsCtrlr | Available / Enabled | false |
| 支付终端 | PaymentCtrlr | Enabled | false |
| 优先级充电 | SmartChargingCtrlr | ACDelayPerLevel | — |

---

## 十、文件位置

本文件位置：`crates/ocpp-2-1/docs/MESSAGES.md`

相关文档和模块：
- `crates/ocpp-1-6/docs/MESSAGES.md` — OCPP 1.6 参考文档
- `crates/ocpp-2-0-1/docs/MESSAGES.md` — OCPP 2.0.1 参考文档（**64 条保留消息的字段详情均在此**）
- `crates/ocpp-2-1/src/common/` — 公共类型定义（枚举、结构体）
- `crates/ocpp-2-1/src/messages/` — 消息定义
- `crates/ocpp-2-1/src/serialization/` — JSON 序列化/反序列化

### 11.1 权威数据来源

- [OCA OCPP 2.1 官方规范（Part 2 + Part 4）](https://www.openchargealliance.org/protocols/ocpp-2-0-1/)
- [OCPP 2.1 Edition 2 JSON Schemas](https://github.com/mobilityhouse/ocpp) — draft-06，字段级权威
- [ocpp.md OCPP 2.1 参考](https://ocpp.md/ocpp-2.1/) — 机械生成的字段级 Schema + 迁移指南
- [OCA 官方 2.1 Appendix CSVs](https://www.openchargealliance.org/downloads/2-1/) — 82 Component / 438 Component/Variable 配对
