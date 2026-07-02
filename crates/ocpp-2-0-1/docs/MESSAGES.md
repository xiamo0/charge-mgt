# OCPP 2.0.1 报文详解

> 本文档详细描述 OCPP 2.0.1 协议的所有报文格式、字段说明和使用场景。
> 方便编程时参考。

---

## 一、协议概述

OCPP 2.0.1J 基于 **JSON over WebSocket** 通信，每条消息都是一个请求-响应对：

| 消息类型 | MessageTypeId | 说明 |
|---|---|---|
| CALL | 2 | 请求 |
| CALLRESULT | 3 | 成功响应 |
| CALLERROR | 4 | 错误响应 |

**CALL 格式**：
```json
[2, "<messageId>", "<action>", { payload }]
```

**CALLRESULT 格式**：
```json
[3, "<messageId>", { payload }]
```

**CALLERROR 格式**：
```json
[4, "<messageId>", "<errorCode>", "<errorDescription>", { "errorDetails" }]
```

### 与 OCPP 1.6 的主要区别

| 特性 | OCPP 1.6 | OCPP 2.0.1 |
|---|---|---|
| 设备模型 | 扁平（ChargePoint → Connectors） | 层级（Station → EVSE → Connector） |
| 事务模型 | StartTransaction / StopTransaction 分离 | 统一 TransactionEvent (Started/Updated/Ended) |
| 配置管理 | Key-Value 模型 | Component/Variable 模型 |
| 安全 | 基础认证 | 三级安全配置文件 + 证书管理 |
| ISO 15118 | 不支持 | Plug & Charge 完整支持 |
| 监控 | 不支持 | 变量监控和事件通知 |
| 展示消息 | 不支持 | 显示屏消息和费用显示 |
| 远程启停 | RemoteStart/StopTransaction | RequestStart/StopTransaction |
| 诊断 | GetDiagnostics | GetLog |
| WebSocket子协议 | ocpp1.6 | ocpp2.0.1 |

### 设备模型

```
Charging Station (充电桩)
├── EVSE 1 (充电设备)
│   ├── Connector 1 (如 CCS)
│   └── Connector 2 (如 CHAdeMO)
├── EVSE 2
│   └── Connector 1 (如 Type 2)
└── ...
```

> **重要**: evseId 和 connectorId 均从 1 开始。evseId=0 表示整个充电桩。

---

## 二、报文总览

OCPP 2.0.1 共 **64 条消息**，组织为 12 个功能块。

### 2.1 Core（10 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| BootNotification | CS → CSMS | 充电桩启动注册 |
| Heartbeat | CS → CSMS | 心跳保活 |
| Authorize | CS → CSMS | 用户鉴权请求 |
| TransactionEvent | CS → CSMS | 事务生命周期事件 |
| StatusNotification | CS → CSMS | 连接器状态变更 |
| MeterValues | CS → CSMS | 电能采样数据 |
| SecurityEventNotification | CS → CSMS | 安全事件通知 |
| ClearCache | CSMS → CS | 清除授权缓存 |
| CostUpdated | CSMS → CS | 更新事务实时费用 |
| GetTransactionStatus | CSMS → CS | 查询事务状态 |

### 2.2 Remote Control（6 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| RequestStartTransaction | CSMS → CS | 远程启动充电 |
| RequestStopTransaction | CSMS → CS | 远程停止充电 |
| Reset | CSMS → CS | 重启充电桩 |
| ChangeAvailability | CSMS → CS | 变更可用性状态 |
| TriggerMessage | CSMS → CS | 触发特定消息上报 |
| UnlockConnector | CSMS → CS | 解锁连接器 |

### 2.3 Configuration（6 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| GetVariables | CSMS → CS | 读取配置变量 |
| SetVariables | CSMS → CS | 写入配置变量 |
| GetBaseReport | CSMS → CS | 请求完整配置报告 |
| GetReport | CSMS → CS | 请求过滤配置报告 |
| NotifyReport | CS → CSMS | 上报配置报告（分页） |
| SetNetworkProfile | CSMS → CS | 设置网络配置文件 |

### 2.4 Smart Charging（9 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| SetChargingProfile | CSMS → CS | 设置充电曲线 |
| ClearChargingProfile | CSMS → CS | 清除充电曲线 |
| GetChargingProfiles | CSMS → CS | 获取已安装充电曲线 |
| GetCompositeSchedule | CSMS → CS | 获取复合充电计划 |
| ReportChargingProfiles | CS → CSMS | 上报充电曲线 |
| NotifyChargingLimit | CS → CSMS | 上报外部充电限制 |
| ClearedChargingLimit | CS → CSMS | 充电限制已清除 |
| NotifyEVChargingNeeds | CS → CSMS | 转发EV充电需求 |
| NotifyEVChargingSchedule | CS → CSMS | 转发EV充电计划 |

### 2.5 Firmware Management（7 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| UpdateFirmware | CSMS → CS | 发起固件更新 |
| FirmwareStatusNotification | CS → CSMS | 固件更新状态 |
| GetLog | CSMS → CS | 请求上传日志 |
| LogStatusNotification | CS → CSMS | 日志上传状态 |
| PublishFirmware | CSMS → CS | 发布固件本地分发 |
| PublishFirmwareStatusNotification | CS → CSMS | 固件发布状态 |
| UnpublishFirmware | CSMS → CS | 停止固件发布 |

### 2.6 Monitoring（7 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| SetVariableMonitoring | CSMS → CS | 设置变量监控 |
| ClearVariableMonitoring | CSMS → CS | 清除变量监控 |
| SetMonitoringBase | CSMS → CS | 激活出厂默认监控 |
| SetMonitoringLevel | CSMS → CS | 设置监控阈值 |
| GetMonitoringReport | CSMS → CS | 请求监控配置报告 |
| NotifyMonitoringReport | CS → CSMS | 上报监控配置 |
| NotifyEvent | CS → CSMS | 上报变量监控事件 |

### 2.7 Reservation（3 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| ReserveNow | CSMS → CS | 创建预约 |
| CancelReservation | CSMS → CS | 取消预约 |
| ReservationStatusUpdate | CS → CSMS | 预约状态变更 |

### 2.8 Certificate Management（7 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| SignCertificate | CS → CSMS | CSR签名请求 |
| CertificateSigned | CSMS → CS | 下发签名证书 |
| InstallCertificate | CSMS → CS | 安装CA证书 |
| DeleteCertificate | CSMS → CS | 删除证书 |
| GetInstalledCertificateIds | CSMS → CS | 查询已安装证书 |
| GetCertificateStatus | CS → CSMS | 查询证书OCSP状态 |
| Get15118EVCertificate | CS → CSMS | 获取EV V2G证书 |

### 2.9 Local Auth List（2 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| SendLocalList | CSMS → CS | 同步本地白名单 |
| GetLocalListVersion | CSMS → CS | 查询白名单版本 |

### 2.10 Display Management（4 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| SetDisplayMessage | CSMS → CS | 设置显示消息 |
| ClearDisplayMessage | CSMS → CS | 清除显示消息 |
| GetDisplayMessages | CSMS → CS | 查询显示消息 |
| NotifyDisplayMessages | CS → CSMS | 上报显示消息 |

### 2.11 Customer Information（2 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| CustomerInformation | CSMS → CS | 请求/清除客户数据 |
| NotifyCustomerInformation | CS → CSMS | 上报客户数据 |

### 2.12 Data Transfer（1 条消息）

| 报文名称 | 请求方向 | 说明 |
| :--- | :---: | :--- |
| DataTransfer | 双向 | 厂商自定义数据 |

---

## 三、报文详细字段

### 3.1 Authorize / Authorize.conf

**方向**：CS → CSMS  
**用途**：用户刷卡时验证 IdToken 合法性

**Authorize.req**：
```json
{
  "idToken": {                      // IdTokenType, 必填
    "idToken": "ABC123456789",     // string(36), 必填, 令牌值
    "type": "ISO14443"              // IdTokenEnumType, 必填, 令牌类型
  },
  "certificate": "...",             // string(5500), 可选, X.509 PEM证书
  "iso15118CertificateHashData": [  // OCSPRequestDataType[], 可选, 最多4个
    {
      "hashAlgorithm": "sha256",
      "issuerKeyHash": "...",
      "issuerNameHash": "...",
      "responderURL": "...",
      "serialNumber": "..."
    }
  ]
}
```

**Authorize.conf**：
```json
{
  "idTokenInfo": {                  // IdTokenInfoType, 必填
    "status": "Accepted",          // AuthorizationStatusEnumType, 必填
    "cacheExpiryDateTime": "2025-12-31T23:59:59Z",  // dateTime, 可选
    "chargingPriority": 0,         // int(-9~9), 可选
    "language1": "zh",             // string(8), 可选
    "language2": "en",             // string(8), 可选
    "groupIdToken": {              // IdTokenType, 可选, 组令牌
      "idToken": "GROUP001",
      "type": "Central"
    },
    "personalMessage": {           // MessageContentType, 可选
      "format": "UTF8",
      "content": "欢迎"
    }
  },
  "certificateStatus": "Accepted"  // AuthorizeCertificateStatusEnumType, 可选
}
```

**IdTokenEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Central | 中央系统标识 |
| eMAID | e-Mobility Account Identifier |
| ISO14443 | ISO 14443 RFID |
| ISO15693 | ISO 15693 RFID |
| KeyCode | 密码/PIN |
| Local | 本地标识 |
| MacAddress | MAC地址 |
| NoAuthorization | 无需授权 |

**AuthorizationStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 接受 |
| Blocked | 锁定 |
| ConcurrentTx | 并发事务 |
| Expired | 已过期 |
| Invalid | 无效 |
| NoCredit | 无余额 |
| NotAllowedTypeEVSE | EVSE类型不允许 |
| NotAtThisLocation | 不在此位置 |
| NotAtThisTime | 不在此时间 |
| Unknown | 未知 |

**AuthorizeCertificateStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 接受 |
| SignatureError | 签名错误 |
| CertificateExpired | 证书过期 |
| CertificateRevoked | 证书吊销 |
| NoCertificateAvailable | 无可用证书 |
| CertChainError | 证书链错误 |
| ContractCancelled | 合约取消 |

---

### 3.2 BootNotification / BootNotification.conf

**方向**：CS → CSMS  
**用途**：充电桩启动时向云平台注册

**BootNotification.req**：
```json
{
  "reason": "PowerUp",             // BootReasonEnumType, 必填, 启动原因
  "chargingStation": {              // ChargingStationType, 必填
    "model": "ModelX",             // string(20), 必填, 型号
    "vendorName": "VendorA",       // string(50), 必填, 厂商名称
    "serialNumber": "SN12345",     // string(25), 可选, 序列号
    "firmwareVersion": "1.2.3",    // string(50), 可选, 固件版本
    "modem": {                      // ModemType, 可选
      "iccid": "89012345678901234567",  // string(20), 可选
      "imsi": "460001234567890"         // string(20), 可选
    }
  }
}
```

**BootNotification.conf**：
```json
{
  "status": "Accepted",             // RegistrationStatusEnumType, 必填
  "currentTime": "2025-05-01T12:00:00Z",  // dateTime, 必填
  "interval": 30                    // int, 必填, 心跳间隔(秒)
}
```

**BootReasonEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| ApplicationReset | 应用复位 |
| FirmwareUpdate | 固件更新后 |
| LocalReset | 本地复位 |
| PowerUp | 上电启动 |
| RemoteReset | 远程复位 |
| ScheduledReset | 计划复位 |
| Triggered | 被触发 |
| Unknown | 未知 |
| Watchdog | 看门狗复位 |

**RegistrationStatusEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Accepted | 已接受 |
| Pending | 待定 |
| Rejected | 拒绝 |

---

### 3.3 Heartbeat / Heartbeat.conf

**方向**：CS → CSMS  
**用途**：保持连接，同步时间

**Heartbeat.req**：空 payload
```json
{}
```

**Heartbeat.conf**：
```json
{
  "currentTime": "2025-05-01T12:00:00Z"  // dateTime, 必填
}
```

---

### 3.4 TransactionEvent / TransactionEvent.conf

**方向**：CS → CSMS  
**用途**：报告事务生命周期事件（替代 OCPP 1.6 的 StartTransaction/StopTransaction）

**TransactionEvent.req**：
```json
{
  "eventType": "Started",           // TransactionEventEnumType, 必填
  "timestamp": "2025-05-01T12:00:00Z",  // dateTime, 必填
  "triggerReason": "Authorized",    // TriggerReasonEnumType, 必填
  "seqNo": 0,                       // int, 必填, 递增序列号
  "transactionInfo": {              // TransactionType, 必填
    "transactionId": "TX-001",     // string(36), 必填, 事务ID
    "chargingState": "Charging",   // ChargingStateEnumType, 可选
    "timeSpentCharging": 3600,     // int, 可选, 实际充电时间(秒)
    "stoppedReason": "EVDisconnected",  // ReasonEnumType, 可选（仅Ended时）
    "remoteStartId": 123           // int, 可选, 关联远程启动ID
  },
  "offline": false,                 // bool, 可选, 是否离线时发生
  "numberOfPhasesUsed": 3,          // int, 可选, 使用相数
  "cableMaxCurrent": 32,            // int, 可选, 线缆最大电流(A)
  "reservationId": 456,             // int, 可选, 关联预约ID
  "evse": {                         // EVSEType, 可选
    "id": 1,                        // int(>0), 必填
    "connectorId": 1               // int, 可选
  },
  "idToken": {                      // IdTokenType, 可选
    "idToken": "ABC123456789",
    "type": "ISO14443"
  },
  "meterValue": [                   // MeterValueType[], 可选
    {
      "timestamp": "2025-05-01T12:30:00Z",
      "sampledValue": [
        {
          "value": 15500.0,
          "context": "Sample.Periodic",
          "measurand": "Energy.Active.Import.Register",
          "phase": "L1",
          "location": "Outlet",
          "unitOfMeasure": { "unit": "Wh", "multiplier": 0 }
        }
      ]
    }
  ]
}
```

**TransactionEvent.conf**：
```json
{
  "idTokenInfo": {                  // IdTokenInfoType, 可选
    "status": "Accepted"
  },
  "totalCost": 25.50,              // number, 可选（仅Ended时）
  "chargingPriority": 0,           // int(-9~9), 可选
  "updatedPersonalMessage": {      // MessageContentType, 可选
    "format": "UTF8",
    "content": "充电完成"
  }
}
```

**TransactionEventEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Started | 事务开始 |
| Updated | 事务更新 |
| Ended | 事务结束 |

**TriggerReasonEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Authorized | 授权通过 |
| CablePluggedIn | 插入线缆 |
| ChargingRateChanged | 充电速率变化 |
| ChargingStateChanged | 充电状态变化 |
| Deauthorized | 授权撤销 |
| EnergyLimitReached | 达到能量限制 |
| EVCommunicationLost | EV通信丢失 |
| EVConnectTimeout | EV连接超时 |
| MeterValueClock | 时钟对齐采样 |
| MeterValuePeriodic | 周期采样 |
| TimeLimitReached | 达到时间限制 |
| Trigger | 被触发 |
| UnlockCommand | 解锁命令 |
| StopAuthorized | 停止授权 |
| EVDeparted | EV离开 |
| EVDetected | 检测到EV |
| RemoteStop | 远程停止 |
| RemoteStart | 远程启动 |
| AbnormalCondition | 异常状况 |
| SignedDataReceived | 收到签名数据 |
| ResetCommand | 复位命令 |

**ChargingStateEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Charging | 充电中 |
| EVConnected | EV已连接 |
| SuspendedEV | EV侧暂停 |
| SuspendedEVSE | EVSE侧暂停 |
| Idle | 空闲 |

**ReasonEnumType 枚举值** (stoppedReason)：
| 值 | 说明 |
|---|---|
| DeAuthorized | 授权撤销 |
| EmergencyStop | 急停 |
| EnergyLimitReached | 能量限制 |
| EVDisconnected | EV断开 |
| GroundFault | 接地故障 |
| ImmediateReset | 立即复位 |
| Local | 本地停止 |
| LocalOutOfCredit | 本地余额不足 |
| MasterPass | 主卡 |
| Other | 其他 |
| OvercurrentFault | 过流故障 |
| PowerLoss | 掉电 |
| PowerQuality | 电能质量 |
| Reboot | 重启 |
| Remote | 远程停止 |
| SOCLimitReached | SoC限制 |
| StoppedByEV | EV停止 |
| TimeLimitReached | 时间限制 |
| Timeout | 超时 |

---

### 3.5 StatusNotification / StatusNotification.conf

**方向**：CS → CSMS  
**用途**：上报 EVSE 上连接器的状态变化

**StatusNotification.req**：
```json
{
  "timestamp": "2025-05-01T12:00:00Z",  // dateTime, 必填
  "connectorStatus": "Available",    // ConnectorStatusEnumType, 必填
  "evseId": 1,                       // int(>0), 必填
  "connectorId": 1                   // int(>0), 必填
}
```

**StatusNotification.conf**：空 payload
```json
{}
```

**ConnectorStatusEnumType 枚举值**：
| 值 | 说明 | 典型场景 |
|---|---|---|
| Available | 可用 | 未插枪 |
| Occupied | 占用 | 正在充电 |
| Reserved | 已预约 | 被预约占用 |
| Unavailable | 不可用 | 维护中 |
| Faulted | 故障 | 发生故障 |

---

### 3.6 MeterValues / MeterValues.conf

**方向**：CS → CSMS  
**用途**：上报电能采样数据（事务外部的周期采样）

**MeterValues.req**：
```json
{
  "evseId": 1,                       // int(>=0), 必填（0=总表）
  "meterValue": [                    // MeterValueType[], 必填
    {
      "timestamp": "2025-05-01T12:30:00Z",  // dateTime, 必填
      "sampledValue": [              // SampledValueType[], 必填
        {
          "value": 15500.0,          // number, 必填
          "context": "Sample.Periodic",  // ReadingContextEnumType, 可选
          "measurand": "Energy.Active.Import.Register",  // MeasurandEnumType, 可选
          "phase": "L1",             // PhaseEnumType, 可选
          "location": "Outlet",      // LocationEnumType, 可选
          "signedMeterValue": {      // SignedMeterValueType, 可选
            "signedMeterData": "...",
            "signingMethod": "...",
            "encodingMethod": "...",
            "publicKey": "..."
          },
          "unitOfMeasure": {         // UnitOfMeasureType, 可选
            "unit": "Wh",
            "multiplier": 0
          }
        }
      ]
    }
  ]
}
```

**MeterValues.conf**：空 payload
```json
{}
```

**MeasurandEnumType 枚举值**（常用）：
| 值 | 说明 |
|---|---|
| Energy.Active.Import.Register | 有功电能-输入（最常用） |
| Energy.Active.Export.Register | 有功电能-输出 |
| Energy.Reactive.Import.Register | 无功电能-输入 |
| Power.Active.Import | 有功功率 |
| Power.Active.Export | 有功功率-输出 |
| Power.Reactive.Import | 无功功率 |
| Power.Factor | 功率因数 |
| Current.Import | 电流-输入 |
| Current.Export | 电流-输出 |
| Current.Offered | 电流-供给 |
| Voltage | 电压 |
| Frequency | 频率 |
| SoC | 荷电状态 |

**ReadingContextEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| Interruption.Begin | 中断开始 |
| Interruption.End | 中断结束 |
| Other | 其他 |
| Sample.Clock | 时钟采样 |
| Sample.Periodic | 周期采样（最常用） |
| Transaction.Begin | 事务开始 |
| Transaction.End | 事务结束 |
| Trigger | 触发 |

**PhaseEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| L1, L2, L3 | L1/L2/L3相 |
| N | 中性线 |
| L1-N, L2-N, L3-N | 相-中性线 |
| L1-L2, L2-L3, L3-L1 | 相-相 |

**LocationEnumType 枚举值**：Body(桩体) | Cable(线缆) | EV(车辆) | Inlet(入口) | Outlet(出口,默认)

---

### 3.7 SecurityEventNotification

**方向**：CS → CSMS  
**用途**：上报安全相关事件

**SecurityEventNotification.req**：
```json
{
  "type": "FirmwareSignatureInvalid",  // string(50), 必填
  "timestamp": "2025-05-01T12:00:00Z",  // dateTime, 必填
  "techInfo": "details..."           // string(255), 可选
}
```

**SecurityEventNotification.conf**：空 payload `{}`

---

### 3.8 ClearCache

**方向**：CSMS → CS  
**用途**：清除本地授权缓存

**ClearCache.req**：空 payload `{}`

**ClearCache.conf**：
```json
{
  "status": "Accepted",              // ClearCacheStatusEnumType, 必填
  "statusInfo": {}                   // StatusInfoType, 可选
}
```
ClearCacheStatusEnumType: Accepted | Rejected

---

### 3.9 CostUpdated

**方向**：CSMS → CS  
**用途**：推送事务实时费用

**CostUpdated.req**：
```json
{
  "totalCost": 25.50,                // number, 必填
  "transactionId": "TX-001"          // string(36), 必填
}
```

**CostUpdated.conf**：空 payload `{}`

---

### 3.10 GetTransactionStatus

**方向**：CSMS → CS  
**用途**：查询事务状态

**GetTransactionStatus.req**：
```json
{
  "transactionId": "TX-001"          // string(36), 可选
}
```

**GetTransactionStatus.conf**：
```json
{
  "messagesInQueue": false,          // bool, 必填
  "ongoingIndicator": true           // bool, 可选
}
```

---

### 3.11 RequestStartTransaction

**方向**：CSMS → CS  
**用途**：远程启动充电

**RequestStartTransaction.req**：
```json
{
  "remoteStartId": 123,              // int, 必填
  "idToken": {                       // IdTokenType, 必填
    "idToken": "ABC123456789",
    "type": "ISO14443"
  },
  "evseId": 1,                       // int(>0), 可选
  "groupIdToken": {},                // IdTokenType, 可选
  "chargingProfile": {}              // ChargingProfileType, 可选
}
```

**RequestStartTransaction.conf**：
```json
{
  "status": "Accepted",              // RequestStartStopStatusEnumType, 必填
  "transactionId": "TX-001",        // string(36), 可选
  "statusInfo": {}
}
```
RequestStartStopStatusEnumType: Accepted | Rejected

---

### 3.12 RequestStopTransaction

**方向**：CSMS → CS  
**用途**：远程停止充电

**RequestStopTransaction.req**：
```json
{
  "transactionId": "TX-001"          // string(36), 必填
}
```

**RequestStopTransaction.conf**：
```json
{
  "status": "Accepted",              // RequestStartStopStatusEnumType, 必填
  "statusInfo": {}
}
```

---

### 3.13 Reset

**方向**：CSMS → CS  
**用途**：重启充电桩

**Reset.req**：
```json
{
  "type": "Immediate",               // ResetEnumType: Immediate/OnIdle, 必填
  "evseId": 1                        // int, 可选（空=整桩）
}
```

**Reset.conf**：
```json
{
  "status": "Accepted"               // ResetStatusEnumType: Accepted/Rejected/Scheduled
}
```

---

### 3.14 ChangeAvailability

**方向**：CSMS → CS  
**用途**：变更 EVSE 或连接器可用性

**ChangeAvailability.req**：
```json
{
  "operationalStatus": "Operative",   // OperationalStatusEnumType: Operative/Inoperative
  "evse": { "id": 1, "connectorId": 1 }  // EVSEType, 可选
}
```

**ChangeAvailability.conf**：
```json
{
  "status": "Accepted"               // ChangeAvailabilityStatusEnumType: Accepted/Rejected/Scheduled
}
```

---

### 3.15 TriggerMessage

**方向**：CSMS → CS  
**用途**：触发充电桩发送特定消息

**TriggerMessage.req**：
```json
{
  "requestedMessage": "BootNotification",  // MessageTriggerEnumType, 必填
  "evse": { "id": 1 }                // EVSEType, 可选
}
```

**TriggerMessage.conf**：
```json
{
  "status": "Accepted"               // TriggerMessageStatusEnumType: Accepted/Rejected/NotImplemented
}
```

**MessageTriggerEnumType 枚举值**：
| 值 | 说明 |
|---|---|
| BootNotification | 启动通知 |
| LogStatusNotification | 日志状态 |
| FirmwareStatusNotification | 固件状态 |
| Heartbeat | 心跳 |
| MeterValues | 电能数据 |
| SignChargingStationCertificate | 充电站证书签名 |
| SignV2GCertificate | V2G证书签名 |
| StatusNotification | 状态通知 |
| TransactionEvent | 事务事件 |
| SignCombinedCertificate | 组合证书签名 |
| PublishFirmwareStatusNotification | 固件发布状态 |

---

### 3.16 UnlockConnector

**方向**：CSMS → CS  
**用途**：远程解锁连接器

**UnlockConnector.req**：
```json
{
  "evseId": 1,                       // int(>0), 必填
  "connectorId": 1                   // int(>0), 必填
}
```

**UnlockConnector.conf**：
```json
{
  "status": "Unlocked"               // UnlockStatusEnumType: Unlocked/UnlockFailed/OngoingAuthorizedTransaction
}
```

---

### 3.17 GetVariables

**方向**：CSMS → CS  
**用途**：读取配置变量（替代 1.6 的 GetConfiguration）

**GetVariables.req**：
```json
{
  "getVariableData": [
    {
      "component": { "name": "HeartbeatCtrlr" },
      "variable": { "name": "Interval" },
      "attributeType": "Actual"      // Actual/Target/MinSet/MaxSet
    }
  ]
}
```

**GetVariables.conf**：
```json
{
  "getVariableResult": [
    {
      "attributeStatus": "Accepted", // GetVariableStatusEnumType
      "attributeValue": "30",
      "component": { "name": "HeartbeatCtrlr" },
      "variable": { "name": "Interval" }
    }
  ]
}
```
GetVariableStatusEnumType: Accepted | Rejected | UnknownComponent | UnknownVariable | NotSupportedAttributeType

---

### 3.18 SetVariables

**方向**：CSMS → CS  
**用途**：写入配置变量（替代 1.6 的 ChangeConfiguration）

**SetVariables.req**：
```json
{
  "setVariableData": [
    {
      "attributeValue": "60",
      "component": { "name": "HeartbeatCtrlr" },
      "variable": { "name": "Interval" },
      "attributeType": "Actual"
    }
  ]
}
```

**SetVariables.conf**：
```json
{
  "setVariableResult": [
    {
      "attributeStatus": "Accepted", // SetVariableStatusEnumType
      "component": { "name": "HeartbeatCtrlr" },
      "variable": { "name": "Interval" }
    }
  ]
}
```
SetVariableStatusEnumType: Accepted | Rejected | UnknownComponent | UnknownVariable | NotSupportedAttributeType | RebootRequired | InvalidValue | OutOfRange

---

### 3.19 DataTransfer

**方向**：双向  
**用途**：厂商自定义数据传输

**DataTransfer.req**：
```json
{
  "vendorId": "vendorA",            // string(255), 必填
  "messageId": "customMsg",         // string(50), 可选
  "data": "..."                     // any, 可选
}
```

**DataTransfer.conf**：
```json
{
  "status": "Accepted",             // DataTransferStatusEnumType: Accepted/Rejected/UnknownVendorId/UnknownMessageId
  "data": "..."
}
```

---

## 四、报文交互流程

### 4.1 充电桩启动流程

```
充电桩启动
    │
    ▼
WebSocket 连接建立 (子协议: ocpp2.0.1)
    │
    ▼
BootNotification.req ──────────► CSMS
    │ reason: PowerUp
    │ chargingStation: { model, vendorName, ... }
    │
◄── BootNotification.conf
    │ status: Accepted, interval: 30
    │
    ▼
StatusNotification.req ────────► (每个EVSE的每个Connector)
    │ evseId: 1, connectorId: 1, connectorStatus: Available
    │
◄── StatusNotification.conf (空)
    │
    ▼ (每 interval 秒)
Heartbeat.req ─────────────────► CSMS
◄── Heartbeat.conf (currentTime)
```

### 4.2 充电事务流程

```
用户刷卡 → Authorize.req → conf (status: Accepted)
    │
用户插枪 → StatusNotification.req (Occupied)
    │
    ▼
TransactionEvent.req ──────────► CSMS
    │ eventType: Started, triggerReason: Authorized
    │ seqNo: 0, transactionId: "TX-001"
    │
◄── TransactionEvent.conf (idTokenInfo.status: Accepted)
    │
    │ (定期上报)
    ▼
TransactionEvent.req (eventType: Updated, seqNo: 1,2,3...)
    │ meterValue: [{ periodic sampledValues }]
    │
◄── TransactionEvent.conf (空)
    │
    │ (用户拔枪)
    ▼
TransactionEvent.req ──────────► CSMS
    │ eventType: Ended, triggerReason: EVDisconnected
    │ seqNo: N, stoppedReason: "EVDisconnected"
    │
◄── TransactionEvent.conf (totalCost: 25.50)
    │
StatusNotification.req (Available)
```

### 4.3 远程启动充电流程

```
RequestStartTransaction.req ──► CS (remoteStartId: 123, idToken, evseId)
◄── RequestStartTransaction.conf (status: Accepted)
    │
TransactionEvent.req ──► CSMS
    │ eventType: Started, triggerReason: RemoteStart
    │ transactionInfo: { remoteStartId: 123 }
◄── TransactionEvent.conf
```

---

## 五、关键配置变量（Component/Variable 模型）

| Component | Variable | 说明 | 类型 |
|---|---|---|---|
| HeartbeatCtrlr | Interval | 心跳间隔(秒) | int |
| AuthCtrlr | Enabled | 是否启用授权 | bool |
| AuthCtrlr | OfflineTxForUnknownIdEnabled | 离线允许未知令牌交易 | bool |
| AuthCtrlr | LocalAuthorizeOffline | 离线使用本地白名单 | bool |
| AuthCtrlr | LocalPreAuthorize | 本地预授权 | bool |
| SampledDataCtrlr | TxUpdatedInterval | 事务更新采样间隔(秒) | int |
| SampledDataCtrlr | TxUpdatedMeasurands | 事务更新采样量 | string |
| TxCtrlr | EVConnectionTimeOut | EV连接超时(秒) | int |
| TxCtrlr | StopTxOnEVSideDisconnect | EV断开时停止事务 | bool |
| TxCtrlr | StopTxOnInvalidId | 无效ID时停止事务 | bool |
| OCPPCommCtrlr | RetryBackOffRepeatTimes | 重连重试次数 | int |
| OCPPCommCtrlr | RetryBackOffRandomRange | 重连随机退避范围(秒) | int |
| OCPPCommCtrlr | RetryBackOffWaitMinimum | 重连最小等待(秒) | int |
| OCPPCommCtrlr | WebSocketPingInterval | WebSocket ping间隔(秒) | int |
| AlignedDataCtrlr | Interval | 时钟对齐采样间隔(秒) | int |
| SmartChargingCtrlr | Enabled | 是否支持智能充电 | bool |
| ReservationCtrlr | Enabled | 是否支持预约 | bool |
| DisplayMessageCtrlr | Enabled | 是否支持显示消息 | bool |

---

## 六、错误处理

**CALLERROR 格式**：
```json
[4, "<messageId>", "<errorCode>", "<errorDescription>", { "errorDetails" }]
```

**ErrorCode 枚举值**：
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

## 七、1.6 消息到 2.0.1 映射

| OCPP 1.6 | OCPP 2.0.1 等价物 |
|---|---|
| StartTransaction | TransactionEvent (eventType=Started) |
| StopTransaction | TransactionEvent (eventType=Ended) |
| MeterValues（事务中） | TransactionEvent (eventType=Updated) |
| MeterValues（事务外） | MeterValues |
| GetConfiguration | GetVariables / GetBaseReport |
| ChangeConfiguration | SetVariables |
| RemoteStartTransaction | RequestStartTransaction |
| RemoteStopTransaction | RequestStopTransaction |
| ChangeAvailability | ChangeAvailability（目标改为EVSE） |
| GetDiagnostics | GetLog |
| DiagnosticsStatusNotification | LogStatusNotification |

---

## 八、实战建议

### 8.1 必须实现的报文（P0）

```
1. BootNotification.req → conf          # 启动注册
2. Heartbeat.req → conf                 # 保持连接
3. StatusNotification.req → conf        # 状态上报
4. Authorize.req → conf                 # 用户鉴权
5. TransactionEvent.req → conf          # 事务生命周期
6. MeterValues.req → conf               # 电能上报
```

### 8.2 建议实现的报文（P1）

```
7. RequestStartTransaction.req → conf   # 远程启动
8. RequestStopTransaction.req → conf    # 远程停止
9. ChangeAvailability.req → conf        # 可用性变更
10. GetVariables.req → conf             # 读取配置
11. SetVariables.req → conf             # 写入配置
12. DataTransfer.req → conf             # 自定义数据
13. ClearCache.req → conf               # 清除缓存
14. Reset.req → conf                    # 重启
```

### 8.3 按需实现的报文（P2）

```
15. UnlockConnector.req → conf
16. TriggerMessage.req → conf
17. UpdateFirmware / FirmwareStatusNotification
18. GetLog / LogStatusNotification
19. SetChargingProfile / ClearChargingProfile / GetCompositeSchedule
20. ReserveNow / CancelReservation / ReservationStatusUpdate
21. SendLocalList / GetLocalListVersion
22. SecurityEventNotification
23. Certificate Management 系列
24. CostUpdated / Display Management 系列
25. NotifyReport / GetBaseReport / GetReport
26. Monitoring 系列报文
27. CustomerInformation / NotifyCustomerInformation
```

---

## 九、文件位置

本文件位置：`crates/ocpp-2-0-1/docs/MESSAGES.md`

相关文档和模块：
- `crates/ocpp-1-6/docs/MESSAGES.md` — OCPP 1.6 参考文档
- `crates/ocpp-2-0-1/src/common/` — 公共类型定义（枚举、结构体）
- `crates/ocpp-2-0-1/src/messages/` — 消息定义
- `crates/ocpp-2-0-1/src/serialization/` — JSON 序列化/反序列化
