# 充电桩管理项目 — 项目结构定义文档
## Project: charge-mgt

**版本：v1.0**
**日期：2026-05-01**

---

## 一、项目概览

| 项目名称 | charge-mgt |
|---|---|
| 类型 | Rust Workspace 单体仓库 |
| 语言 | Rust |
| 目标 | 充电桩云平台管理系统，包含 Cloud / Gateway / Simulator 三大组件 |

---

## 二、Crate 依赖关系

```
charge-mgt-common ──────────────────► charge-mgt-cloud
                                      charge-mgt-gateway
                                      charge-mgt-simulator

ocpp-1-6 ───────────────────────────► charge-mgt-cloud
                                      charge-mgt-gateway
                                      charge-mgt-simulator

ocpp-2-0-1 ─────────────────────────► charge-mgt-cloud
                                      charge-mgt-gateway

ocpp-2-1 ────────────────────────────► charge-mgt-gateway
```

---

## 三、详细目录结构

```
charge-mgt/
├── Cargo.toml
├── README.md
│
├── crates/
│   ├── ocpp-1-6/                   # OCPP 1.6 协议实现
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── protocol.rs
│   │   │   ├── common/            # 公共类型
│   │   │   ├── messages/
│   │   │   │   ├── calls/         # 请求消息 (CP→CS)
│   │   │   │   └── confs/         # 响应消息 (CS→CP)
│   │   │   ├── profiles/          # OCPP Profiles
│   │   │   ├── serialization/      # JSON 编解码
│   │   │   └── validation/         # 消息校验
│   │   └── Cargo.toml
│   │
│   ├── ocpp-2-0-1/                # OCPP 2.0.1 协议实现
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── protocol.rs
│   │   │   ├── common/
│   │   │   ├── messages/
│   │   │   │   ├── calls/
│   │   │   │   └── confs/
│   │   │   ├── profiles/
│   │   │   └── serialization/
│   │   └── Cargo.toml
│   │
│   ├── ocpp-2-1/                  # OCPP 2.1 协议实现
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── protocol.rs
│   │   │   ├── common/
│   │   │   ├── messages/
│   │   │   │   ├── calls/
│   │   │   │   └── confs/
│   │   │   ├── profiles/
│   │   │   └── serialization/
│   │   └── Cargo.toml
│   │
│   ├── charge-mgt-common/         # 公共基础设施
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   ├── id.rs
│   │   │   ├── tracing.rs
│   │   │   ├── db/
│   │   │   ├── mq/
│   │   │   └── utils/
│   │   └── Cargo.toml
│   │
│   ├── charge-mgt-cloud/          # 云平台 (CSMS)
│   │   ├── config/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── main.rs
│   │   │   ├── app.rs
│   │   │   ├── state.rs
│   │   │   ├── config.rs
│   │   │   ├── routes/
│   │   │   ├── ocpp/
│   │   │   │   ├── server.rs
│   │   │   │   ├── session.rs
│   │   │   │   ├── handler.rs
│   │   │   │   ├── dispatcher.rs
│   │   │   │   └── service/
│   │   │   │       ├── authorize.rs
│   │   │   │       ├── boot.rs
│   │   │   │       ├── transaction.rs
│   │   │   │       ├── meter.rs
│   │   │   │       └── availability.rs
│   │   │   │   └── calls/
│   │   │   │       ├── remote_start.rs
│   │   │   │       ├── remote_stop.rs
│   │   │   │       └── unlock.rs
│   │   │   └── models/
│   │   │       ├── charge_pile.rs
│   │   │       ├── transaction.rs
│   │   │       ├── meter_value.rs
│   │   │       ├── connector.rs
│   │   │       ├── id_tag.rs
│   │   │       └── reservation.rs
│   │   └── Cargo.toml
│   │
│   ├── charge-mgt-gateway/        # 信息交换网关
│   │   ├── config/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── main.rs
│   │   │   ├── app.rs
│   │   │   ├── state.rs
│   │   │   ├── config.rs
│   │   │   ├── north/            # 北向: 充电桩接入
│   │   │   │   ├── server.rs
│   │   │   │   └── session.rs
│   │   │   ├── south/            # 南向: 云平台连接
│   │   │   │   ├── ws_client.rs
│   │   │   │   └── mq_client.rs
│   │   │   ├── proxy.rs
│   │   │   ├── protocol/         # 协议转换
│   │   │   │   ├── router.rs
│   │   │   │   ├── v1_to_v2.rs
│   │   │   │   ├── v2_to_v1.rs
│   │   │   │   ├── v1_to_v21.rs
│   │   │   │   ├── v21_to_v1.rs
│   │   │   │   ├── v2_to_v21.rs
│   │   │   │   └── v21_to_v2.rs
│   │   │   ├── codec/
│   │   │   │   ├── framing.rs
│   │   │   │   └── version_detect.rs
│   │   │   └── routes/
│   │   └── Cargo.toml
│   │
│   └── charge-mgt-simulator/     # 充电桩模拟器
│       ├── config/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── main.rs
│       │   ├── cli.rs
│       │   ├── config.rs
│       │   ├── charger.rs
│       │   ├── charger_state.rs
│       │   ├── evse/
│       │   │   ├── evse.rs
│       │   │   ├── connector.rs
│       │   │   ├── meter.rs
│       │   │   └── rfid.rs
│       │   ├── ocpp/
│       │   │   ├── client.rs
│       │   │   ├── protocol.rs
│       │   │   ├── message_queue.rs
│       │   │   └── sender.rs
│       │   ├── scenarios/
│       │   │   ├── basic.rs
│       │   │   ├── auth.rs
│       │   │   ├── transaction.rs
│       │   │   ├── remote_start.rs
│       │   │   └── error.rs
│       │   └── protocol/
│       │       ├── reconnect.rs
│       │       └── heartbeat.rs
│       └── Cargo.toml
│
├── migrations/
│   └── 001_init.sql
│
├── tests/
│   ├── ocpp-1-6-protocol-tests/
│   ├── ocpp-2-0-1-protocol-tests/
│   ├── ocpp-2-1-protocol-tests/
│   └── integration-tests/
│
├── docs/
│   └── PROJECT_STRUCTURE.md
│
└── scripts/
    ├── run-cloud.sh
    ├── run-gateway.sh
    └── run-simulator.sh
```

---

## 四、OCPP 消息优先级

### Phase 1 — Core Profile

| 优先级 | 消息 | 版本 |
|---|---|---|
| P0 | Authorize | 1.6, 2.0.1, 2.1 |
| P0 | BootNotification | 1.6, 2.0.1, 2.1 |
| P0 | Heartbeat | 1.6, 2.0.1, 2.1 |
| P0 | StartTransaction | 1.6 |
| P0 | StopTransaction | 1.6 |
| P0 | TransactionEvent | 2.0.1, 2.1 |
| P0 | StatusNotification | 1.6, 2.0.1, 2.1 |
| P0 | MeterValues | 1.6, 2.0.1, 2.1 |
| P0 | RemoteStartTransaction | 1.6, 2.0.1, 2.1 |
| P0 | RemoteStopTransaction | 1.6, 2.0.1, 2.1 |

### Phase 2 — Optional

| 优先级 | 消息 | 版本 |
|---|---|---|
| P1 | ChangeConfiguration | 1.6, 2.0.1, 2.1 |
| P1 | GetConfiguration | 1.6, 2.0.1, 2.1 |
| P1 | ChangeAvailability | 1.6, 2.0.1, 2.1 |
| P1 | UnlockConnector | 1.6, 2.0.1, 2.1 |
| P1 | DataTransfer | 1.6, 2.0.1, 2.1 |
| P1 | GetVariables / SetVariables | 2.0.1, 2.1 |

---

## 五、实现顺序

1. ocpp-1-6 (核心协议)
2. charge-mgt-common (基础设施)
3. ocpp-2-0-1 (协议)
4. charge-mgt-cloud (云平台)
5. charge-mgt-simulator (模拟器)
6. charge-mgt-gateway (网关)
7. ocpp-2-1 (协议)
8. 协议转换
9. 测试
10. 文档