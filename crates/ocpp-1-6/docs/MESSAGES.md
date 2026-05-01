# OCPP 1.6 报文详解

> 本文档详细描述 OCPP 1.6 协议的所有报文格式、字段说明和使用场景。
> 方便编程时参考。

---

## 一、协议概述

OCPP 1.6J 基于 **JSON over WebSocket** 通信，每条消息都是一个请求-响应对：

| 消息类型 | 方向 | 说明 |
|---|---|---|
| `[2, "action", payload]` | CALL | 请求（3元素数组） |
| `[3, "action", payload]` | CALLRESULT | 成功响应 |
| `[4, "action", error]` | CALLERROR | 错误响应 |

**CALL 格式**：
```json
[2, "<message_id>", "<action>", { payload }]
```

**CALLRESULT 格式**：
```json
[3, "<message_id>", { payload }]
```

**CALLERROR 格式**：
```json
[4, "<message_id>", "<error_code>", "<error_description>", { "errorDetails" }]
```

---

## 二、报文总览

### 2.1 Core Profile（核心功能）

| # | 请求 (CP→CS) | 响应 (CS→CP) | 说明 |
|---|---|---|---|
| 1 | `Authorize` | `Authorize.conf` | RFID 鉴权 |
| 2 | `BootNotification` | `BootNotification.conf` | 启动注册 |
| 3 | `Heartbeat` | `Heartbeat.conf` | 心跳保活 |
| 4 | `StartTransaction` | `StartTransaction.conf` | 开始充电 |
| 5 | `StopTransaction` | `StopTransaction.conf` | 结束充电 |
| 6 | `MeterValues` | `MeterValues.conf` | 电能数据上报 |
| 7 | `StatusNotification` | `StatusNotification.conf` | 状态变更 |
| 8 | `RemoteStartTransaction` | `RemoteStartTransaction.conf` | 远程启动 |
| 9 | `RemoteStopTransaction` | `RemoteStopTransaction.conf` | 远程停止 |
| 10 | `ChangeAvailability` | `ChangeAvailability.conf` | 变更可用性 |
| 11 | `ChangeConfiguration` | `ChangeConfiguration.conf` | 修改配置 |
| 12 | `GetConfiguration` | `GetConfiguration.conf` | 查询配置 |
| 13 | `ClearCache` | `ClearCache.conf` | 清除缓存 |
| 14 | `UnlockConnector` | `UnlockConnector.conf` | 解锁连接器 |
| 15 | `DataTransfer` | `DataTransfer.conf` | 厂商自定义传输 |

### 2.2 Firmware Management Profile（固件管理）

| # | 请求 (CP→CS) | 响应 (CS→CP) | 说明 |
|---|---|---|---|
| 16 | `GetDiagnostics` | `GetDiagnostics.conf` | 获取诊断 |
| 17 | `UpdateFirmware` | `UpdateFirmware.conf` | 更新固件 |
| 18 | `DiagnosticsStatusNotification` | 无（单向） | 诊断状态 |
| 19 | `FirmwareStatusNotification` | 无（单向） | 固件状态 |

### 2.3 Reservation Profile（预约）

| # | 请求 (CP→CS) | 响应 (CS→CP) | 说明 |
|---|---|---|---|
| 20 | `ReserveNow` | `ReserveNow.conf` | 预约 |
| 21 | `CancelReservation` | `CancelReservation.conf` | 取消预约 |

### 2.4 Local Auth List Management Profile（本地白名单）

| # | 请求 (CP→CS) | 响应 (CS→CP) | 说明 |
|---|---|---|---|
| 22 | `SendLocalList` | `SendLocalList.conf` | 同步白名单 |
| 23 | `GetLocalListVersion` | `GetLocalListVersion.conf` | 获取版本 |

### 2.5 Smart Charging Profile（智能充电）

| # | 请求 (CP→CS) | 响应 (CS→CP) | 说明 |
|---|---|---|---|
| 24 | `SetChargingProfile` | `SetChargingProfile.conf` | 设置充电曲线 |
| 25 | `ClearChargingProfile` | `ClearChargingProfile.conf` | 清除充电曲线 |
| 26 | `GetCompositeSchedule` | `GetCompositeSchedule.conf` | 获取复合schedule |

### 2.6 Remote Trigger Profile（远程触发）

| # | 请求 (CP→CS) | 响应 (CS→CP) | 说明 |
|---|---|---|---|
| 27 | `TriggerMessage` | `TriggerMessage.conf` | 触发消息 |

---

## 三、报文详细字段

### 3.1 Authorize / Authorize.conf

**方向**：CP → CS
**用途**：用户刷卡时验证 idTag 合法性

**Authorize.req**：
```json
{
  "idTag": "ABC123456789"  // string(20), 必填, RFID卡号
}
```

**Authorize.conf**：
```json
{
  "status": "Accepted",    // AuthorizationStatus, 必填
  "idTagInfo": {           // IdTagInfo, 可选
    "status": "Accepted", // AuthorizationStatus
    "expiryDate": "2025-12-31T23:59:59Z",  // dateTime, 可选
    "parentIdTag": "PARENT001"  // string(20), 可选
  }
}
```

**AuthorizationStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 接受 |
| `Blocked` | 锁定 |
| `Expired` | 已过期 |
| `Invalid` | 无效 |
| `ConcurrentTx` | 并发事务 |

---

### 3.2 BootNotification / BootNotification.conf

**方向**：CP → CS
**用途**：充电桩启动时向云平台注册

**BootNotification.req**：
```json
{
  "chargePointVendor": "VendorA",       // string(20), 必填, 厂商名称
  "chargePointModel": "ModelX",        // string(20), 必填, 型号
  "chargeBoxSerialNumber": "SN12345",  // string(25), 可选, 充电盒序列号
  "chargePointSerialNumber": "CP001",  // string(25), 可选, 充电桩序列号
  "firmwareVersion": "1.2.3",          // string(50), 可选, 固件版本
  "iccid": "89012345678901234567",     // string(20), 可选, SIM卡ICCID
  "imsi": "460001234567890",           // string(20), 可选, SIM卡IMSI
  "meterType": "SDM630",              // string(25), 可选, 电能表型号
  "meterSerialNumber": "MTR12345"      // string(25), 可选, 电能表序列号
}
```

**BootNotification.conf**：
```json
{
  "status": "Accepted",                // RegistrationStatus, 必填
  "currentTime": "2025-05-01T12:00:00Z",  // dateTime, 必填, 云平台当前时间
  "interval": 30                       // int, 必填, 心跳间隔(秒)
}
```

**RegistrationStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受，正常工作 |
| `Pending` | 待定，稍后重试 |
| `Rejected` | 拒绝，停止重试 |

---

### 3.3 Heartbeat / Heartbeat.conf

**方向**：CP → CS
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

### 3.4 StartTransaction / StartTransaction.conf

**方向**：CP → CS
**用途**：用户插枪后开始充电

**StartTransaction.req**：
```json
{
  "connectorId": 1,                     // int(>0), 必填, 连接器ID
  "idTag": "ABC123456789",             // string(20), 必填, 鉴权卡号
  "meterStart": 1000,                  // int, 必填, 起始读数(Wh)
  "reservationId": 123,                // int, 可选, 预约ID
  "timestamp": "2025-05-01T12:00:00Z"  // dateTime, 必填, 开始时间
}
```

**StartTransaction.conf**：
```json
{
  "transactionId": 1,                  // int, 必填, 云平台分配的事务ID
  "idTagInfo": {                        // IdTagInfo, 可选
    "status": "Accepted"
  }
}
```

---

### 3.5 StopTransaction / StopTransaction.conf

**方向**：CP → CS
**用途**：用户拔枪后停止充电

**StopTransaction.req**：
```json
{
  "meterStop": 5000,                   // int, 必填, 结束读数(Wh)
  "timestamp": "2025-05-01T14:00:00Z", // dateTime, 必填, 结束时间
  "transactionId": 1,                   // int, 必填, 事务ID
  "reason": "EVDisconnected",           // Reason, 可选, 停止原因
  "idTag": "ABC123456789",             // string(20), 可选, 结束卡号
  "transactionData": [                 // MeterValue[], 可选, 充电过程数据
    {
      "timestamp": "2025-05-01T12:30:00Z",
      "sampledValue": [
        {
          "value": "15.5",
          "context": "Sample.Periodic",
          "format": "Raw",
          "measurand": "Energy.Active.Import.Register",
          "unit": "kWh"
        }
      ]
    }
  ]
}
```

**StopTransaction.conf**：
```json
{
  "idTagInfo": {                        // IdTagInfo, 可选
    "status": "Accepted"
  }
}
```

**Reason 枚举值**：
| 值 | 说明 |
|---|---|
| `EmergencyStop` | 急停 |
| `EVDisconnected` | EV断开 |
| `HardReset` | 硬复位 |
| `Local` | 本地停止 |
| `Other` | 其他 |
| `PowerLoss` | 掉电 |
| `Reboot` | 重启 |
| `Remote` | 远程停止 |
| `SoftReset` | 软复位 |
| `StormReset` | 暴风雨复位 |
| `TillDisconnection` | 等待断开 |

---

### 3.6 MeterValues / MeterValues.conf

**方向**：CP → CS
**用途**：上报充电过程中的电能采样值

**MeterValues.req**：
```json
{
  "connectorId": 1,                     // int(>0), 必填, 连接器ID
  "transactionId": 1,                   // int, 可选, 事务ID
  "meterValue": [                        // MeterValue[], 必填
    {
      "timestamp": "2025-05-01T12:30:00Z",  // dateTime, 必填, 采样时间
      "sampledValue": [                   // SampledValue[], 必填
        {
          "value": "15.5",               // string, 必填, 采样值
          "context": "Sample.Periodic",  // ReadingContext, 可选
          "format": "Raw",               // ValueFormat, 可选
          "measurand": "Energy.Active.Import.Register",  // Measurand, 可选
          "unit": "kWh"                  // UnitOfMeasure, 可选
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

**Measurand 枚举值**：
| 值 | 说明 |
|---|---|
| `Energy.Active.Import.Register` | 有功电能（最常用） |
| `Energy.Reactive.Import.Register` | 无功电能 |
| `Power.Active.Import` | 有功功率 |
| `Power.Reactive.Import` | 无功功率 |
| `Current.Import` | 电流 |
| `Voltage` | 电压 |
| `Temperature` | 温度 |
| `SoC` | State of Charge（荷电状态） |

**ReadingContext 枚举值**：
| 值 | 说明 |
|---|---|
| `Interruption.Begin` | 中断开始 |
| `Interruption.End` | 中断结束 |
| `Other` | 其他 |
| `Sample.Clock` | 时钟采样 |
| `Sample.Periodic` | 周期采样（最常用） |
| `Transaction.Begin` | 事务开始 |
| `Transaction.End` | 事务结束 |
| `Trigger` | 触发 |

**ValueFormat 枚举值**：
| 值 | 说明 |
|---|---|
| `Raw` | 原始值 |
| `SignedData` | 带符号数据 |

**UnitOfMeasure 枚举值**：
| 值 | 说明 |
|---|---|
| `Wh` | 瓦时 |
| `kWh` | 千瓦时（最常用） |
| `varh` | 乏时 |
| `kvarh` | 千乏时 |
| `W` | 瓦 |
| `kW` | 千瓦 |
| `var` | 乏 |
| `kvar` | 千乏 |
| `A` | 安培 |
| `V` | 伏特 |
| `K` | 开尔文 |
| `Celsius` | 摄氏度 |
| `Percent` | 百分比 |

---

### 3.7 StatusNotification / StatusNotification.conf

**方向**：CP → CS
**用途**：上报充电桩/连接器状态变化

**StatusNotification.req**：
```json
{
  "connectorId": 1,                      // int(>=0), 必填, 连接器ID（0=整个桩）
  "errorCode": "NoError",              // ChargePointErrorCode, 必填
  "info": "Ready to charge",           // string(50), 可选, 附加信息
  "status": "Available",                // ChargePointStatus, 必填
  "timestamp": "2025-05-01T12:00:00Z", // dateTime, 必填
  "vendorId": "vendorA",              // string(50), 可选, 厂商ID
  "vendorErrorCode": ""                // string(50), 可选, 厂商错误码
}
```

**StatusNotification.conf**：空 payload
```json
{}
```

**ChargePointStatus 枚举值**：
| 值 | 说明 | 典型场景 |
|---|---|---|
| `Available` | 可用（空闲） | 枪已插上待机 |
| `Preparing` | 插枪准备中 | 用户刷卡后等待 |
| `Charging` | 充电中 | 正常充电 |
| `SuspendedEVSE` | 暂停（桩侧） | 桩侧故障暂停 |
| `SuspendedEV` | 暂停（EV侧） | EV侧停止充电 |
| `Finishing` | 充电结束 | 等待拔枪 |
| `Reserved` | 已预约 | 被预约占用 |
| `Unavailable` | 不可用 | 维护中 |
| `Faulted` | 故障 | 发生故障 |

**ChargePointErrorCode 枚举值**：
| 值 | 说明 |
|---|---|
| `NoError` | 无错误 |
| `ConnectorLockFailure` | 连接器锁定失败 |
| `EVCommunicationFailure` | EV通信失败 |
| `GroundFailure` | 接地故障 |
| `HighTemperature` | 高温 |
| `InternalError` | 内部错误 |
| `LocalListConflict` | 本地列表冲突 |
| `Mode3Error` | Mode 3错误 |
| `OtherError` | 其他错误 |
| `OverCurrentFailure` | 过流 |
| `OverVoltage` | 过压 |
| `PowerMeterFailure` | 电能表故障 |
| `PowerSwitchFailure` | 电源开关故障 |
| `ReaderFailure` | RFID读卡器故障 |
| `ResetFailure` | 复位失败 |
| `UnderVoltage` | 欠压 |
| `WeakSignal` | 信号弱 |

---

### 3.8 RemoteStartTransaction / RemoteStartTransaction.conf

**方向**：CS → CP
**用途**：云平台远程启动充电

**RemoteStartTransaction.req**：
```json
{
  "connectorId": 1,                    // int, 可选, 连接器ID（空=任意）
  "idTag": "ABC123456789",             // string(20), 必填, 鉴权卡号
  "chargingProfile": {                 // ChargingProfile, 可选, 充电曲线
    "chargingProfileId": 1,
    "stackLevel": 0,
    "chargingProfilePurpose": "TxProfile",
    "chargingProfileKind": "Absolute",
    "validFrom": "2025-05-01T00:00:00Z",
    "validTo": "2025-05-01T23:59:59Z",
    " ChargingSchedule": {
      "duration": 3600,
      "startPeriod": 0,
      "chargingRateUnit": "W",
      "chargingSchedulePeriod": [
        {
          "startPeriod": 0,
          "limit": 7000,
          "numberPhases": 3
        }
      ]
    }
  }
}
```

**RemoteStartTransaction.conf**：
```json
{
  "status": "Accepted"  // RemoteStartStopStatus, 必填
}
```

**RemoteStartStopStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 接受 |
| `Rejected` | 拒绝 |

---

### 3.9 RemoteStopTransaction / RemoteStopTransaction.conf

**方向**：CS → CP
**用途**：云平台远程停止充电

**RemoteStopTransaction.req**：
```json
{
  "transactionId": 1  // int, 必填, 事务ID
}
```

**RemoteStopTransaction.conf**：
```json
{
  "status": "Accepted"  // RemoteStartStopStatus, 必填
}
```

---

### 3.10 ChangeAvailability / ChangeAvailability.conf

**方向**：CS → CP
**用途**：云平台变更充电桩可用性

**ChangeAvailability.req**：
```json
{
  "connectorId": 0,           // int, 必填, 0=整个桩, >0=特定连接器
  "type": "Operative"        // AvailabilityType, 必填
}
```

**ChangeAvailability.conf**：
```json
{
  "status": "Accepted"       // AvailabilityStatus, 必填
}
```

**AvailabilityType 枚举值**：
| 值 | 说明 |
|---|---|
| `Operative` | 可用 |
| `Suspended` | 暂停 |
| `Inoperative` | 不可用 |

**AvailabilityStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 已拒绝 |
| `Scheduled` | 已计划 |

---

### 3.11 ChangeConfiguration / ChangeConfiguration.conf

**方向**：CS → CP
**用途**：云平台修改充电桩配置项

**ChangeConfiguration.req**：
```json
{
  "key": "HeartbeatInterval",     // string, 必填, 配置项名称
  "value": "60"                  // string, 必填, 配置值
}
```

**ChangeConfiguration.conf**：
```json
{
  "status": "Accepted"           // ConfigurationStatus, 必填
}
```

**ConfigurationStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 已拒绝 |
| `RebootRequired` | 需要重启 |

---

### 3.12 GetConfiguration / GetConfiguration.conf

**方向**：CS → CP
**用途**：云平台查询充电桩配置

**GetConfiguration.req**：
```json
{
  "key": ["HeartbeatInterval", "MeterValueSampleInterval"]  // string[], 可选, 特定key
}
```

**GetConfiguration.conf**：
```json
{
  "configurationKey": [           // ConfigurationKey[], 必填
    {
      "key": "HeartbeatInterval",
      "value": "30",
      "readonly": false
    }
  ],
  "unknownKey": ["NotExistKey"] // string[], 必填, 不存在的key
}
```

**ConfigurationKey 结构**：
| 字段 | 类型 | 说明 |
|---|---|---|
| `key` | string | 配置项名称 |
| `value` | string | 配置值 |
| `readonly` | bool | 是否只读 |

---

### 3.13 ClearCache / ClearCache.conf

**方向**：CS → CP
**用途**：云平台清除本地授权缓存

**ClearCache.req**：空 payload
```json
{}
```

**ClearCache.conf**：
```json
{
  "status": "Accepted"  // ClearCacheStatus, 必填
}
```

**ClearCacheStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 已拒绝 |

---

### 3.14 UnlockConnector / UnlockConnector.conf

**方向**：CS → CP
**用途**：云平台远程解锁连接器

**UnlockConnector.req**：
```json
{
  "connectorId": 1  // int(>=1), 必填, 连接器ID
}
```

**UnlockConnector.conf**：
```json
{
  "status": "Unlocked"  // UnlockStatus, 必填
}
```

**UnlockStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Unlocked` | 已解锁 |
| `UnlockFailed` | 解锁失败 |
| `NotSupported` | 不支持 |

---

### 3.15 DataTransfer / DataTransfer.conf

**方向**：双向
**用途**：厂商自定义数据传输

**DataTransfer.req**：
```json
{
  "vendorId": "vendorA",      // string(255), 必填, 厂商ID
  "messageId": "customMsg", // string(50), 可选, 消息ID
  "data": "..."              // string, 可选, 自定义数据
}
```

**DataTransfer.conf**：
```json
{
  "status": "Accepted",      // DataTransferStatus, 必填
  "data": "..."             // string, 可选, 响应数据
}
```

**DataTransferStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 接受 |
| `Rejected` | 拒绝 |
| `UnknownVendorId` | 未知厂商ID |

---

### 3.16 GetDiagnostics / GetDiagnostics.conf

**方向**：CS → CP
**用途**：云平台请求充电桩诊断信息

**GetDiagnostics.req**：
```json
{
  "location": "ftp://diagnostics.example.com/",  // string, 必填, 上传位置
  "protocol": "FTP",                            // string, 可选, 上传协议
  " retries": 3,                                // int, 可选, 重试次数
  "retryInterval": 30,                         // int, 可选, 重试间隔(秒)
  "username": "admin",                         // string, 可选, 用户名
  "password": "secret"                          // string, 可选, 密码
}
```

**GetDiagnostics.conf**：
```json
{
  "filename": "diagnostics_20250501_120000.log"  // string, 可选, 文件名
}
```

---

### 3.17 UpdateFirmware / UpdateFirmware.conf

**方向**：CS → CP
**用途**：云平台发起固件更新

**UpdateFirmware.req**：
```json
{
  "retrieveDate": "2025-05-01T23:00:00Z",  // dateTime, 必填, 开始下载时间
  "firmwareVersion": "1.5.0",               // string(50), 可选, 目标版本
  "location": "https://firmware.example.com/v1.5.0.bin",  // string, 必填, 下载URL
  " retries": 3,                            // int, 可选, 重试次数
  "retryInterval": 30                       // int, 可选, 重试间隔(秒)
}
```

**UpdateFirmware.conf**：
```json
{}
```

---

### 3.18 DiagnosticsStatusNotification（单向）

**方向**：CP → CS
**用途**：上报诊断上传状态

**DiagnosticsStatusNotification.req**：
```json
{
  "status": "Uploaded",           // DiagnosticsStatus, 必填
  "requestId": 123                // int, 可选, 关联的GetDiagnostics请求ID
}
```

**DiagnosticsStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Idle` | 空闲 |
| `Downloading` | 下载中 |
| `Downloaded` | 已下载 |
| `Installing` | 安装中 |
| `Installed` | 已安装 |
| `UploadFailed` | 上传失败 |

---

### 3.19 FirmwareStatusNotification（单向）

**方向**：CP → CS
**用途**：上报固件更新状态

**FirmwareStatusNotification.req**：
```json
{
  "status": "Downloading",        // FirmwareStatus, 必填
  "requestId": 123                // int, 可选, 关联的UpdateFirmware请求ID
}
```

**FirmwareStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Idle` | 空闲 |
| `Downloading` | 下载中 |
| `Downloaded` | 已下载 |
| `Installing` | 安装中 |
| `Installed` | 已安装 |
| `DownloadFailed` | 下载失败 |
| `InstallationFailed` | 安装失败 |

---

### 3.20 ReserveNow / ReserveNow.conf

**方向**：CS → CP
**用途**：云平台创建预约

**ReserveNow.req**：
```json
{
  "connectorId": 1,                        // int(>=1), 必填, 连接器ID
  "expiryDate": "2025-05-01T15:00:00Z",   // dateTime, 必填, 预约过期时间
  "idTag": "ABC123456789",                // string(20), 必填, 鉴权卡号
  "parentIdTag": "PARENT001",             // string(20), 可选, 父卡号
  "reservationId": 123,                   // int, 必填, 预约ID
  "startDate": "2025-05-01T12:00:00Z"     // dateTime, 可选, 开始时间
}
```

**ReserveNow.conf**：
```json
{
  "status": "Accepted"  // ReservationStatus, 必填
}
```

**ReservationStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Faulted` | 故障 |
| `Occupied` | 被占用 |
| `Rejected` | 拒绝 |
| `Unavailable` | 不可用 |

---

### 3.21 CancelReservation / CancelReservation.conf

**方向**：CS → CP
**用途**：云平台取消预约

**CancelReservation.req**：
```json
{
  "reservationId": 123  // int, 必填, 预约ID
}
```

**CancelReservation.conf**：
```json
{
  "status": "Accepted"  // CancelReservationStatus, 必填
}
```

**CancelReservationStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 拒绝 |

---

### 3.22 SendLocalList / SendLocalList.conf

**方向**：CS → CP
**用途**：云平台同步本地授权白名单

**SendLocalList.req**：
```json
{
  "listVersion": 5,                          // int, 必填, 列表版本号
  "updateType": "Full",                    // UpdateType, 必填, 更新类型
  "localAuthorisationList": [               // AuthorizationData[], 可选, 授权列表
    {
      "idTag": "ABC123456789",
      "idTagInfo": {
        "status": "Accepted",
        "expiryDate": "2025-12-31T23:59:59Z",
        "parentIdTag": "PARENT001"
      }
    }
  ]
}
```

**SendLocalList.conf**：
```json
{
  "status": "Accepted"  // UpdateStatus, 必填
}
```

**UpdateType 枚举值**：
| 值 | 说明 |
|---|---|
| `Differential` | 增量更新 |
| `Full` | 全量更新 |

**UpdateStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Failed` | 失败 |
| `NotSupported` | 不支持 |
| `VersionMismatch` | 版本不匹配 |

---

### 3.23 GetLocalListVersion / GetLocalListVersion.conf

**方向**：CP → CS
**用途**：查询本地白名单版本

**GetLocalListVersion.req**：空 payload
```json
{}
```

**GetLocalListVersion.conf**：
```json
{
  "listVersion": 5  // int, 必填, 列表版本号
}
```

---

### 3.24 SetChargingProfile / SetChargingProfile.conf

**方向**：CS → CP
**用途**：云平台设置充电曲线

**SetChargingProfile.req**：
```json
{
  "connectorId": 1,                        // int(>=1), 必填, 连接器ID
  "csChargingProfiles": {                  // ChargingProfile, 必填
    "chargingProfileId": 1,
    "stackLevel": 0,
    "chargingProfilePurpose": "TxProfile",
    "chargingProfileKind": "Absolute",
    "validFrom": "2025-05-01T00:00:00Z",
    "validTo": "2025-05-01T23:59:59Z",
    " ChargingSchedule": {
      "duration": 3600,
      "startPeriod": 0,
      "chargingRateUnit": "W",
      "chargingSchedulePeriod": [
        {
          "startPeriod": 0,
          "limit": 7000,
          "numberPhases": 3
        }
      ]
    }
  }
}
```

**SetChargingProfile.conf**：
```json
{
  "status": "Accepted"  // ChargingProfileStatus, 必填
}
```

**ChargingProfileStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 拒绝 |
| `NotSupported` | 不支持 |

---

### 3.25 ClearChargingProfile / ClearChargingProfile.conf

**方向**：CS → CP
**用途**：云平台清除充电曲线

**ClearChargingProfile.req**：
```json
{
  "id": 1,                       // int, 可选, 特定profile ID
  "connectorId": 1,             // int, 可选, 连接器ID
  "chargingProfilePurpose": "TxProfile",  // ChargingProfilePurpose, 可选
  "stackLevel": 0               // int, 可选, stack level
}
```

**ClearChargingProfile.conf**：
```json
{
  "status": "Accepted"  // ClearChargingProfileStatus, 必填
}
```

**ChargingProfilePurpose 枚举值**：
| 值 | 说明 |
|---|---|
| `ChargePointMaxProfile` | 桩最大profile |
| `TxDefaultProfile` | 默认transaction profile |
| `TxProfile` | Transaction profile |

**ClearChargingProfileStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Unknown` | 未知 |

---

### 3.26 GetCompositeSchedule / GetCompositeSchedule.conf

**方向**：CS → CP
**用途**：云平台获取复合充电schedule

**GetCompositeSchedule.req**：
```json
{
  "connectorId": 1,        // int(>=1), 必填, 连接器ID
  "duration": 3600,       // int, 必填, 持续时间(秒)
  "chargingRateUnit": "W"  // ChargingRateUnit, 必填, 费率单位
}
```

**GetCompositeSchedule.conf**：
```json
{
  "status": "Accepted",                   // GetCompositeScheduleStatus, 必填
  "scheduleStart": "2025-05-01T12:00:00Z", // dateTime, 可选, schedule开始时间
  "schedule": {                            // ChargingSchedule, 可选
    "duration": 3600,
    "startPeriod": 0,
    "chargingRateUnit": "W",
    "chargingSchedulePeriod": [
      {
        "startPeriod": 0,
        "limit": 7000,
        "numberPhases": 3
      }
    ]
  }
}
```

**GetCompositeScheduleStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 拒绝 |

**ChargingRateUnit 枚举值**：
| 值 | 说明 |
|---|---|
| `W` | 瓦 |
| `A` | 安培 |

---

### 3.27 TriggerMessage / TriggerMessage.conf

**方向**：CS → CP
**用途**：云平台触发充电桩发送特定消息

**TriggerMessage.req**：
```json
{
  "requestedMessage": "BootNotification"  // MessageTrigger, 必填, 触发消息类型
}
```

**TriggerMessage.conf**：
```json
{
  "status": "Accepted"  // TriggerMessageStatus, 必填
}
```

**MessageTrigger 枚举值**：
| 值 | 说明 |
|---|---|
| `BootNotification` | 启动通知 |
| `DiagnosticsStatusNotification` | 诊断状态 |
| `FirmwareStatusNotification` | 固件状态 |
| `Heartbeat` | 心跳 |
| `MeterValues` | 电能数据 |
| `StatusNotification` | 状态通知 |

**TriggerMessageStatus 枚举值**：
| 值 | 说明 |
|---|---|
| `Accepted` | 已接受 |
| `Rejected` | 拒绝 |
| `NotImplemented` | 未实现 |

---

## 四、报文交互流程

### 4.1 充电桩启动流程

```
充电桩启动
    │
    ▼
┌─────────────────────────────────┐
│  TCP WebSocket 连接建立          │
└─────────────────────────────────┘
    │
    ▼
BootNotification.req ──────────► 云平台
    │ chargePointVendor, model
    │ firmwareVersion, etc.
    │
◄── BootNotification.conf
    │ status: Accepted
    │ currentTime: 2025-05-01T12:00:00Z
    │ interval: 30
    │
    ▼
StatusNotification.req ────────► (每个connector)
    │ connectorId: 1, status: Available
    │
◄── StatusNotification.conf (空)

    │
    ▼ (每 interval 秒)
Heartbeat.req ─────────────────► 云平台
◄── Heartbeat.conf
    │ currentTime: 2025-05-01T12:00:30Z
    │
    ▼
```

### 4.2 充电流程

```
用户刷卡
    │
    ▼
Authorize.req ──────────────────► 云平台
    │ idTag: "ABC123"
    │
◄── Authorize.conf
    │ status: Accepted
    │ idTagInfo: { status: Accepted }

用户插枪
    │
    ▼
StatusNotification.req ────────► 云平台
    │ connectorId: 1
    │ status: Preparing
    │
◄── StatusNotification.conf (空)

StartTransaction.req ──────────► 云平台
    │ connectorId: 1
    │ idTag: "ABC123"
    │ meterStart: 1000
    │ timestamp: 2025-05-01T12:00:00Z
    │
◄── StartTransaction.conf
    │ transactionId: 1
    │ idTagInfo: { status: Accepted }

StatusNotification.req ────────► 云平台
    │ connectorId: 1
    │ status: Charging
    │
◄── StatusNotification.conf (空)

    │ (定期上报，根据 MeterValueSampleInterval)
    ▼
MeterValues.req ───────────────► 云平台
    │ connectorId: 1
    │ transactionId: 1
    │ meterValue: [{ timestamp, sampledValue }]
    │
◄── MeterValues.conf (空)

    │ (持续上报中...)
    │
用户拔枪/停止
    │
    ▼
StopTransaction.req ───────────► 云平台
    │ transactionId: 1
    │ meterStop: 5000
    │ timestamp: 2025-05-01T14:00:00Z
    │ reason: EVDisconnected
    │
◄── StopTransaction.conf
    │ idTagInfo: { status: Accepted }

StatusNotification.req ────────► 云平台
    │ connectorId: 1
    │ status: Available
    │
◄── StatusNotification.conf (空)
```

---

## 五、关键配置项

| Key | 说明 | 默认值 | 类型 |
|---|---|---|---|
| `HeartbeatInterval` | 心跳间隔(秒) | 60 | int |
| `MeterValueSampleInterval` | 电能采样间隔(秒) | 60 | int |
| `ClockAlignedDataInterval` | 时钟对齐数据间隔(秒) | 0 | int |
| `StopTransactionOnEVSideDisconnect` | EV断开时停止transaction | true | bool |
| `StopTransactionOnInvalidId` | 无效ID时停止transaction | true | bool |
| `AuthorizeRemoteTxRequests` | 授权远程启动 | true | bool |
| `LocalPreAuthorize` | 本地预授权 | false | bool |
| `MaxEnergyOnInvalidId` | 无效ID最大电能(Wh) | 0 | int |
| `WebSocketPingInterval` | WebSocket ping间隔(秒) | 0 | int |
| `ConnectionTimeOut` | 连接超时(秒) | 0 | int |
| `RetryBackOffRandomRange` | 重试随机范围(秒) | 0 | int |
| `RetryBackOffRange` | 重试退避范围(秒) | 0 | int |
| `RetryInterval` | 重试间隔(秒) | 0 | int |
| `LockConnectorZeroEnergyOverflow` | 锁定连接器零能量溢出 | false | bool |

---

## 六、错误处理

### CALLERROR 格式

```json
[4, "<message_id>", "<error_code>", "<error_description>", { "errorDetails" }]
```

**ErrorCode 枚举值**：
| 值 | 说明 |
|---|---|
| `NotImplemented` | 未实现 |
| `NotSupported` | 不支持 |
| `InternalError` | 内部错误 |
| `ProtocolError` | 协议错误 |
| `SecurityError` | 安全错误 |
| `FormationViolation` | 格式错误 |
| `PropertyConstraintViolation` | 属性约束违规 |
| `OccurenceConstraintViolation` | 出现约束违规 |
| `TypeConstraintViolation` | 类型约束违规 |
| `GenericError` | 通用错误 |

---

## 七、实战建议

### 7.1 必须实现的报文（P0）

```
1. BootNotification.req → conf   # 启动注册
2. Heartbeat.req → conf          # 保持连接
3. StatusNotification.req → conf # 状态上报
4. Authorize.req → conf          # 刷卡鉴权
5. StartTransaction.req → conf    # 开始充电
6. StopTransaction.req → conf     # 结束充电
7. MeterValues.req → conf        # 电能上报
```

### 7.2 建议实现的报文（P1）

```
8. RemoteStartTransaction.req → conf  # 远程启动
9. RemoteStopTransaction.req → conf   # 远程停止
10. ChangeAvailability.req → conf      # 可用性变更
11. GetConfiguration.req → conf        # 查询配置
12. ChangeConfiguration.req → conf     # 修改配置
13. DataTransfer.req → conf            # 自定义数据
```

### 7.3 按需实现的报文（P2）

```
14. UnlockConnector.req → conf
15. ClearCache.req → conf
16. GetDiagnostics.req → conf
17. UpdateFirmware.req → conf
18. DiagnosticsStatusNotification.req (单向)
19. FirmwareStatusNotification.req (单向)
20. ReserveNow.req → conf
21. CancelReservation.req → conf
22. SendLocalList.req → conf
23. GetLocalListVersion.req → conf
24. SetChargingProfile.req → conf
25. ClearChargingProfile.req → conf
26. GetCompositeSchedule.req → conf
27. TriggerMessage.req → conf
```

---

## 八、文件位置

本文件位置：`crates/ocpp-1-6/docs/MESSAGES.md`

相关模块：
- `crates/ocpp-1-6/src/common/` — 公共类型定义（枚举、结构体）
- `crates/ocpp-1-6/src/messages/` — 消息定义
- `crates/ocpp-1-6/src/serialization/` — JSON 序列化/反序列化