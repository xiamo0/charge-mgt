# charge-mgt 充电桩管理系统

基于 Rust 实现的全栈充电桩管理平台，支持 OCPP 1.6、OCPP 2.0.1 和 OCPP 2.1 协议。

## 项目组件

| 组件 | 说明 |
|---|---|
| `charge-mgt-cloud` | 云平台（CSMS），负责充电桩接入管理和业务控制 |
| `charge-mgt-gateway` | 信息交换网关，支持 OCPP 1.6/2.0.1/2.1 协议互转 |
| `charge-mgt-simulator` | 充电桩模拟器，用于测试和开发 |

## 协议实现

| 协议版本 | 说明 |
|---|---|
| `ocpp-1-6` | OCPP 1.6J（JSON over WebSocket），核心消息实现 |
| `ocpp-2-0-1` | OCPP 2.0.1，TransactionEvent 替代 Start/StopTransaction |
| `ocpp-2-1` | OCPP 2.1，支持 Tariff&Cost、VariableUsage 等新特性 |

## 技术栈

- **语言**：Rust
- **异步运行时**：Tokio
- **Web 框架**：Axum
- **WebSocket**：tokio-tungstenite
- **数据库**：PostgreSQL + SQLx
- **配置**：TOML + config
- **日志**：tracing + tracing-subscriber

## 项目结构

```
charge-mgt/
├── crates/                      # Workspace 成员
│   ├── ocpp-1-6/               # OCPP 1.6 协议实现
│   ├── ocpp-2-0-1/             # OCPP 2.0.1 协议实现
│   ├── ocpp-2-1/               # OCPP 2.1 协议实现
│   ├── charge-mgt-common/      # 公共基础设施（配置/DB/日志）
│   ├── charge-mgt-cloud/       # 云平台（CSMS）
│   ├── charge-mgt-gateway/     # 信息交换网关
│   └── charge-mgt-simulator/   # 充电桩模拟器
├── migrations/                  # 数据库迁移脚本
├── tests/                       # 协议一致性测试
├── docs/                       # 项目文档
└── scripts/                    # 运维脚本
```

## 构建

```bash
cargo build --workspace
```

## 开发指南

详见 [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)

## 许可证

MIT OR Apache-2.0