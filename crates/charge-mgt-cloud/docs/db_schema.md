# charge-mgt-cloud 数据库 Schema 文档

> **目标 Crate**: `charge-mgt-cloud`（CSMS - Central System Management System）  
> **数据库**: PostgreSQL 14+  
> **驱动协议**: OCPP 1.6J（同时预留 OCPP 2.0.1 / 2.1 扩展位）  
> **总表数**: 19 张（P0: 10 / P1: 7 / P2: 2）  
> **命名规范**: 所有表统一 `mgt_` 前缀；列名小写下划线；ENUM 类型同前缀

---

## 一、总体设计原则

1. **统一前缀**：`mgt_` 前缀避免与未来其他业务系统共享 schema 时命名冲突
2. **关系表化**：OCPP 报文里大量嵌套 JSON，但**只把高频/用于查询/统计的字段提为列**；纯载荷细节存 `JSONB`
3. **精度**：金额/电量用 `BIGINT`（Wh 整数）和 `NUMERIC(12,4)`（元），**绝不用** `FLOAT`
4. **时间统一 UTC**：`TIMESTAMPTZ DEFAULT now()`，**绝不用** `timestamp without time zone`
5. **软删除优先**：`is_deleted BOOLEAN` 而非真删（id_tag 注销、交易争议等场景）
6. **idempotency 是必须的**：Kafka 至少一次语义下，重复消息必须能识别
7. **外键策略**：
   - 核心业务表（`charge_points`, `transactions`）强外键
   - 日志/审计表（`audit_logs`, `dead_letter_queue`）不设外键
8. **ENUM vs VARCHAR**：受协议严格约束的字段用 `CREATE TYPE ... AS ENUM`（强类型）；开放字段用 `VARCHAR + CHECK`
9. **当前仅支持 OCPP 1.6**：所有 OCPP 规范定义的 ENUM 直接命名（不带版本后缀）。`charge_points.protocol_version` 列保留，未来引入 2.0.1 时再考虑拆列/拆表方案。

---

## 二、表清单（按域分组）

| 域 | 表名 | 优先级 | 驱动消息 / 用途 |
|---|---|---|---|
| **身份权限** | `mgt_operators` | P0 | HTTP API 鉴权账号（与 OCPP 无关） |
| **OCPP 设备** 🆕 | `mgt_charge_points_ocpp_1_6` | P0 | BootNotification 注册（保留 protocol_version 列） |
| | `mgt_connectors_ocpp_1_6` | P0 | StatusNotification 按 connector 维度 |
| **OCPP 鉴权** 🆕 | `mgt_id_tags_ocpp_1_6` | P0 | Authorize 鉴权 |
| | `mgt_id_tag_groups` | P0 | id_tag 分组（业务层，跨协议通用） |
| **OCPP 交易** 🆕 | `mgt_transactions_ocpp_1_6` | P0 | Start/StopTransaction |
| | `mgt_meter_values_ocpp_1_6` | P0 | MeterValues 上报 |
| **计费** | `mgt_tariffs` | P1 | 计费费率（业务层） |
| | `mgt_bills` | P1 | 账单聚合（业务层） |
| **OCPP 远程命令** 🆕 | `mgt_pending_commands_ocpp_1_6` | P0 | CSMS 主动命令 outbox 跟踪 |
| | `mgt_config_keys_ocpp_1_6` | P1 | ChangeConfiguration |
| | `mgt_firmware_ocpp_1_6` | P1 | UpdateFirmware |
| **OCPP 扩展** 🆕 | `mgt_reservations_ocpp_1_6` | P2 | ReserveNow |
| | `mgt_charging_profiles_ocpp_1_6` | P2 | SetChargingProfile |
| **可观测性** | `mgt_sent_messages` | P0 | 消息去重 / idempotency（协议无关） |
| | `mgt_message_log_ocpp_1_6` | P1 | 消息流可观测性（原 `mgt_ocpp_message_log`，去掉 `ocpp_` 前缀避免与 `_ocpp_1_6` 后缀重复） |
| | `mgt_gateway_health` | P0 | Gateway 实例健康（协议无关） |
| **审计 / DLQ** | `mgt_audit_logs` | P1 | 运营操作审计（协议无关） |
| | `mgt_dead_letter_queue` | P1 | 失败消息死信（协议无关） |

> **关键设计决策（当前仅支持 OCPP 1.6）**：
> - **OCPP 协议绑定表加 `_ocpp_1_6` 后缀**（11 张）：`charge_points`, `connectors`, `id_tags`, `transactions`, `meter_values`, `pending_commands`, `config_keys`, `firmware`, `reservations`, `charging_profiles`, `message_log`
> - **业务 / 基础设施表不加后缀**（8 张）：`operators`, `id_tag_groups`, `tariffs`, `bills`, `sent_messages`, `gateway_health`, `audit_logs`, `dead_letter_queue`
> - 5 个 OCPP 协议字段（`id_tags.status`, `connectors.status`, `transactions.stop_reason`, `meter_values.measurand`, `meter_values.context`）直接用 OCPP 1.6 ENUM，无版本拆分
> - `charge_points.protocol_version` 列保留（DEFAULT 'OCPP-1.6'），未来引入 2.0.1 时只需新增 `_ocpp_2_0_1` 后缀表
> - CSMS 内部状态机（`transaction_state`, `pending_command_state`）**不**受协议版本影响

---

## 三、ER 关系总览

```
mgt_operators
   │
   │ (actor)
   ▼
mgt_audit_logs ◀──── 各种业务表都在这里留痕

mgt_charge_points_ocpp_1_6 ─┬─ mgt_connectors_ocpp_1_6         (1:N)
                          ├─ mgt_transactions_ocpp_1_6 ─┬─ mgt_meter_values_ocpp_1_6 (1:N)
                          │                            ├─ mgt_bills        (1:N)
                          │                            └─ mgt_pending_commands_ocpp_1_6 (1:N)
                          ├─ mgt_charging_profiles_ocpp_1_6    (1:N)
                          └─ mgt_firmware_ocpp_1_6             (N:M via deployment log)

mgt_id_tags_ocpp_1_6 ─ mgt_id_tag_groups (N:1)
mgt_tariffs ─ mgt_id_tag_groups (N:1, applicable group)
mgt_tariffs ─ mgt_transactions_ocpp_1_6  (1:N via tariff_id)
mgt_reservations_ocpp_1_6 ─ mgt_transactions_ocpp_1_6 (1:1 optional)

mgt_sent_messages      (独立表，idempotency 屏障)
mgt_gateway_health     (独立表，按 gateway_id 主键)
mgt_dead_letter_queue  (独立表，运维清理)
mgt_message_log_ocpp_1_6   (独立表，可观测性)
```

---

## 四、PostgreSQL 扩展依赖

```sql
-- UUID 生成（已默认安装 pgcrypto/uuid-ossp 之一即可，PG13+ 可用 gen_random_uuid()）
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
```

---

## 五、完整 DDL

> ⚠️ **执行顺序**：所有 `CREATE TYPE` 必须在引用它的 `CREATE TABLE` 之前。下面的代码按依赖顺序排列。

```sql
-- ============================================================
-- charge-mgt-cloud PostgreSQL Schema
-- 目标版本: PostgreSQL 14+
-- 字符集: UTF8, 时区: UTC
-- ============================================================

BEGIN;

-- ============================================================
-- 0. 扩展
-- ============================================================
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================
-- 1. ENUM 类型
-- ============================================================

-- ============================================================
-- ENUM 类型（仅支持 OCPP 1.6）
-- ============================================================
-- 当前阶段只考虑 OCPP 1.6 协议（ocpp-2-0-1 / ocpp-2-1 crate 暂未实现）。
-- 所有 OCPP 规范定义的枚举直接命名为不带版本后缀的类型。
-- 未来引入 2.0.1 时按当时需求决定是否拆分为 _v1 / _v2 列。

-- OCPP 1.6 AuthorizationStatus（5 个值）
CREATE TYPE mgt_id_tag_status AS ENUM (
    'Accepted',       -- 允许
    'Blocked',        -- 永久拒绝
    'Expired',        -- 过期
    'Invalid',        -- 无效
    'ConcurrentTx'    -- 已有并发交易
);

-- OCPP 1.6 ChargePointStatus（9 个值）
CREATE TYPE mgt_connector_status AS ENUM (
    'Available',
    'Preparing',
    'Charging',
    'SuspendedEVSE',
    'SuspendedEV',
    'Finishing',
    'Reserved',
    'Unavailable',
    'Faulted'
);

-- OCPP 1.6 StopTransaction Reason（11 个值）
CREATE TYPE mgt_reason AS ENUM (
    'EmergencyStop',
    'EvDisconnected',
    'HardReset',
    'Local',
    'Other',
    'PowerLoss',
    'Reboot',
    'Remote',
    'SoftReset',
    'StormReset',
    'TillDisconnection'
);

-- OCPP 1.6 Measurand（15 个值，PascalCase 无点号）
CREATE TYPE mgt_measurand AS ENUM (
    'EnergyActiveImportRegister',
    'EnergyReactiveImportRegister',
    'EnergyActiveExportRegister',
    'EnergyReactiveExportRegister',
    'PowerActiveImport',
    'PowerReactiveImport',
    'PowerActiveExport',
    'PowerReactiveExport',
    'PowerFactor',
    'CurrentImport',
    'CurrentExport',
    'Voltage',
    'Temperature',
    'SoC',
    'Frequency'
);

-- OCPP 1.6 ReadingContext（8 个值，PascalCase）
CREATE TYPE mgt_reading_context AS ENUM (
    'InterruptionBegin',
    'InterruptionEnd',
    'Other',
    'SampleClock',
    'SamplePeriodic',
    'TransactionBegin',
    'TransactionEnd',
    'Trigger'
);

-- ============================================================
-- CSMS 内部状态机（与 OCPP 协议版本无关）
-- ============================================================

-- 交易状态机
CREATE TYPE mgt_transaction_state AS ENUM (
    'Active',         -- 已 Start，尚未 Stop
    'Completed',      -- 正常结束
    'Faulted',        -- 异常中断（CP 离线 / 重启 / 通讯丢失）
    'Aborted'         -- 主动取消（RemoteStop / 运营撤销）
);

-- CSMS 主动命令的 outbox 状态机
CREATE TYPE mgt_pending_command_state AS ENUM (
    'Pending',        -- 已发出，等响应
    'Accepted',       -- 收到 CallResult 且 accepted=true
    'Rejected',       -- 收到 CallResult 但 rejected，或 CallError
    'Timeout',        -- 超时未响应
    'Cancelled'       -- 运营手动取消
);

-- ============================================================
-- 2. 域：身份与权限
-- ============================================================

-- ------------------------------------------------------------
-- 2.1 mgt_operators（运营/运维账号）
-- ------------------------------------------------------------
-- 用途：HTTP API 鉴权（JWT 主体）、审计日志外键
-- 关键决策：password_hash 存的是 argon2 / bcrypt 哈希；role 限定 3 个值
-- ------------------------------------------------------------
CREATE TABLE mgt_operators (
    id              BIGSERIAL       PRIMARY KEY,
    username        VARCHAR(64)     NOT NULL UNIQUE,
    email           VARCHAR(128)    UNIQUE,
    password_hash   VARCHAR(255)    NOT NULL,                  -- argon2 / bcrypt
    display_name    VARCHAR(128),
    role            VARCHAR(32)     NOT NULL,                  -- 'admin' | 'operator' | 'viewer'
    is_active       BOOLEAN         NOT NULL DEFAULT TRUE,
    is_deleted      BOOLEAN         NOT NULL DEFAULT FALSE,
    last_login_at   TIMESTAMPTZ,
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),

    CONSTRAINT mgt_operators_role_chk
        CHECK (role IN ('admin', 'operator', 'viewer'))
);

COMMENT ON TABLE  mgt_operators IS '运营/运维账号表，HTTP API 鉴权主体';
COMMENT ON COLUMN mgt_operators.password_hash IS 'argon2id 哈希，格式：$argon2id$v=19$m=...';
COMMENT ON COLUMN mgt_operators.role IS 'admin=超级管理员, operator=运营, viewer=只读';
COMMENT ON COLUMN mgt_operators.last_login_at IS '最近一次成功登录时间，用于审计';

-- 仅查未删除的邮箱
CREATE INDEX idx_mgt_operators_email
    ON mgt_operators(email)
    WHERE is_deleted = FALSE;

-- ------------------------------------------------------------
-- 2.2 mgt_charge_points_ocpp_1_6（充电桩主表）⭐ 核心表
-- ------------------------------------------------------------
-- 用途：OCPP BootNotification 注册的充电桩
-- 关键决策：
--   - id 用业务 ID（chargeBoxSerialNumber），不用自增
--   - gateway_id 必有（来自 Gateway 透传）
--   - ocpp_status 用 VARCHAR+CHECK 允许后续扩展
-- ------------------------------------------------------------
CREATE TABLE mgt_charge_points_ocpp_1_6 (
    id                      VARCHAR(64)     PRIMARY KEY,      -- 业务 ID = chargeBoxSerialNumber
    gateway_id              VARCHAR(64)     NOT NULL,         -- 来自 CloudMessage.gateway_id
    vendor                  VARCHAR(128)    NOT NULL,
    model                   VARCHAR(128)    NOT NULL,
    serial_number           VARCHAR(128),                    -- chargePointSerialNumber
    charge_box_serial       VARCHAR(128),                    -- chargeBoxSerialNumber（冗余，便于查询）
    firmware_version        VARCHAR(64),
    iccid                   VARCHAR(64),
    imsi                    VARCHAR(64),
    meter_type              VARCHAR(64),
    meter_serial_number     VARCHAR(128),
    protocol_version        VARCHAR(16)     NOT NULL DEFAULT 'OCPP-1.6',
    ocpp_status             VARCHAR(32)     NOT NULL DEFAULT 'Registered',
    heartbeat_interval_secs INTEGER         NOT NULL DEFAULT 30,
    last_heartbeat_at       TIMESTAMPTZ,
    last_boot_at            TIMESTAMPTZ,
    registered_at           TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ     NOT NULL DEFAULT now(),
    is_deleted              BOOLEAN         NOT NULL DEFAULT FALSE,

    CONSTRAINT mgt_cps_status_chk
        CHECK (ocpp_status IN ('Registered', 'Online', 'Offline', 'Faulted', 'Disabled')),
    CONSTRAINT mgt_cps_protocol_chk
        CHECK (protocol_version IN ('OCPP-1.6')),  -- 当前仅支持 1.6；引入 2.0.1 时放开此 CHECK
    CONSTRAINT mgt_cps_hb_interval_chk
        CHECK (heartbeat_interval_secs BETWEEN 5 AND 86400)   -- 5s ~ 24h
);

COMMENT ON TABLE  mgt_charge_points_ocpp_1_6 IS 'OCPP 1.6 充电桩主表，BootNotification 注册的物理设备';
COMMENT ON COLUMN mgt_charge_points_ocpp_1_6.id IS '业务主键，来自 OCPP chargeBoxSerialNumber；无此字段则拒绝 Boot';
COMMENT ON COLUMN mgt_charge_points_ocpp_1_6.gateway_id IS '该桩经由哪个 Gateway 接入，路由用';
COMMENT ON COLUMN mgt_charge_points_ocpp_1_6.ocpp_status IS 'Registered=已注册未上线, Online=心跳正常, Offline=心跳超时, Faulted=自报故障, Disabled=运营禁用';
COMMENT ON COLUMN mgt_charge_points_ocpp_1_6.last_heartbeat_at IS '最近一次 Heartbeat 到达时间，用于超时巡检';
COMMENT ON COLUMN mgt_charge_points_ocpp_1_6.is_deleted IS '软删除标记，保留历史交易归属';

CREATE INDEX idx_mgt_cps_gateway
    ON mgt_charge_points_ocpp_1_6(gateway_id)
    WHERE is_deleted = FALSE;

CREATE INDEX idx_mgt_cps_vendor
    ON mgt_charge_points_ocpp_1_6(vendor, model)
    WHERE is_deleted = FALSE;

CREATE INDEX idx_mgt_cps_status
    ON mgt_charge_points_ocpp_1_6(ocpp_status)
    WHERE is_deleted = FALSE;

-- 心跳超时巡检专用索引
CREATE INDEX idx_mgt_cps_hb_late
    ON mgt_charge_points_ocpp_1_6(last_heartbeat_at)
    WHERE is_deleted = FALSE AND ocpp_status = 'Online';

-- ------------------------------------------------------------
-- 2.3 mgt_connectors_ocpp_1_6（充电连接器）
-- ------------------------------------------------------------
-- 用途：StatusNotification 按 connector 维度状态
-- 关键决策：
--   - status 用 mgt_connector_status ENUM（OCPP 1.6 ChargePointStatus 9 个值）
--   - 每个 CP 多个 connector，状态独立
-- ------------------------------------------------------------
CREATE TABLE mgt_connectors_ocpp_1_6 (
    id                BIGSERIAL       PRIMARY KEY,
    charge_point_id   VARCHAR(64)     NOT NULL
                          REFERENCES mgt_charge_points_ocpp_1_6(id) ON DELETE CASCADE,
    connector_id      INTEGER         NOT NULL,             -- OCPP connectorId, 0/1/2...
    status            mgt_connector_status NOT NULL DEFAULT 'Available',
    error_code        VARCHAR(32)     NOT NULL DEFAULT 'NoError',
    vendor_id         VARCHAR(128),
    vendor_error_code VARCHAR(128),
    info              TEXT,                                  -- 自由文本
    last_status_at    TIMESTAMPTZ,
    updated_at        TIMESTAMPTZ     NOT NULL DEFAULT now(),

    UNIQUE (charge_point_id, connector_id)
);

COMMENT ON TABLE  mgt_connectors_ocpp_1_6 IS 'OCPP 1.6 充电连接器，StatusNotification 粒度';
COMMENT ON COLUMN mgt_connectors_ocpp_1_6.connector_id IS 'OCPP connectorId，通常 0=整个桩, 1+=物理连接器';
COMMENT ON COLUMN mgt_connectors_ocpp_1_6.status IS 'OCPP 1.6 ChargePointStatus 9 个枚举值之一';
COMMENT ON COLUMN mgt_connectors_ocpp_1_6.error_code IS 'OCPP ChargePointErrorCode 枚举（16+ 值）';

CREATE INDEX idx_mgt_conn_cp
    ON mgt_connectors_ocpp_1_6(charge_point_id);

-- ============================================================
-- 3. 域：授权与凭证
-- ============================================================

-- ------------------------------------------------------------
-- 3.1 mgt_id_tag_groups（idTag 分组）
-- ------------------------------------------------------------
-- 用途：批量管理（VIP/员工/测试卡），按 group 设费率
-- ------------------------------------------------------------
CREATE TABLE mgt_id_tag_groups (
    id          BIGSERIAL       PRIMARY KEY,
    name        VARCHAR(128)    NOT NULL UNIQUE,
    description TEXT,
    created_at  TIMESTAMPTZ     NOT NULL DEFAULT now()
);

COMMENT ON TABLE mgt_id_tag_groups IS 'idTag 分组表，用于批量授权与差异化计费';

-- ------------------------------------------------------------
-- 3.2 mgt_id_tags_ocpp_1_6（RFID / App 账户）⭐
-- ------------------------------------------------------------
-- 用途：Authorize 鉴权
-- 关键决策：
--   - status 用 mgt_id_tag_status ENUM（OCPP 1.6 AuthorizationStatus 5 个值）
--   - parent_id_tag 不强制外键（OCPP 允许树形继承，但需避免循环）
--   - is_deleted 软删除，保留历史交易关联
-- ------------------------------------------------------------
CREATE TABLE mgt_id_tags_ocpp_1_6 (
    id              BIGSERIAL           PRIMARY KEY,
    id_tag          VARCHAR(64)         NOT NULL UNIQUE,    -- OCPP idTag 字符串
    group_id        BIGINT              REFERENCES mgt_id_tag_groups(id) ON DELETE SET NULL,
    status          mgt_id_tag_status NOT NULL DEFAULT 'Accepted',
    parent_id_tag   VARCHAR(64),                              -- 父卡，弱引用
    expiry_date     TIMESTAMPTZ,
    note            TEXT,
    is_deleted      BOOLEAN             NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ         NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ         NOT NULL DEFAULT now()
);

COMMENT ON TABLE  mgt_id_tags_ocpp_1_6 IS 'OCPP 1.6 鉴权卡 / App 账户主表';
COMMENT ON COLUMN mgt_id_tags_ocpp_1_6.id_tag IS 'OCPP 报文中的 idTag 字符串（卡号/手机号/UUID）';
COMMENT ON COLUMN mgt_id_tags_ocpp_1_6.status IS 'OCPP 1.6 AuthorizationStatus 5 个枚举值之一';
COMMENT ON COLUMN mgt_id_tags_ocpp_1_6.parent_id_tag IS 'OCPP 父卡（树形继承），不强制 FK 防循环';
COMMENT ON COLUMN mgt_id_tags_ocpp_1_6.expiry_date IS '鉴权失效时间，NULL=永久';
COMMENT ON COLUMN mgt_id_tags_ocpp_1_6.is_deleted IS '软删除，注销后历史交易仍能查';

CREATE INDEX idx_mgt_idtags_group
    ON mgt_id_tags_ocpp_1_6(group_id)
    WHERE is_deleted = FALSE;

CREATE INDEX idx_mgt_idtags_status
    ON mgt_id_tags_ocpp_1_6(status)
    WHERE is_deleted = FALSE;

-- ============================================================
-- 4. 域：交易与计费 ⭐ 核心域
-- ============================================================

-- ------------------------------------------------------------
-- 4.1 mgt_transactions_ocpp_1_6（交易主表）⭐⭐
-- ------------------------------------------------------------
-- 用途：Start/StopTransaction 全链路记录
-- 关键决策：
--   - id 自增 = OCPP transactionId（CSMS 自行生成）
--   - meter_*_wh 用 BIGINT（Wh 整数，不用 FLOAT 避免精度损失）
--   - raw_stop_payload JSONB 保留原始报文（审计/重算）
--   - state 枚举覆盖异常（CP 失联时标 Faulted），与协议版本无关
--   - started_at（Cloud 收到时间）vs start_timestamp（CP 上报时间）分开
--   - stop_reason 用 mgt_reason ENUM（OCPP 1.6 11 个值）
-- ------------------------------------------------------------
CREATE TABLE mgt_transactions_ocpp_1_6 (
    id                  BIGSERIAL                     PRIMARY KEY,    -- OCPP transactionId
    charge_point_id     VARCHAR(64)                   NOT NULL
                            REFERENCES mgt_charge_points_ocpp_1_6(id),
    connector_id        INTEGER                       NOT NULL,
    id_tag              VARCHAR(64)                   NOT NULL
                            REFERENCES mgt_id_tags_ocpp_1_6(id_tag),    -- 即便卡注销也保留 FK
    meter_start_wh      BIGINT                        NOT NULL,        -- 单位 Wh
    meter_stop_wh       BIGINT,                                       -- Stop 时填
    start_timestamp     TIMESTAMPTZ                   NOT NULL,        -- CP 上报时间
    stop_timestamp      TIMESTAMPTZ,                                  -- CP 上报时间
    stop_reason         mgt_reason,                            -- OCPP 1.6 Reason (11 个值)
    state               mgt_transaction_state  NOT NULL DEFAULT 'Active',
    total_kwh           NUMERIC(10,3),                                -- 结算后算
    total_cost          NUMERIC(12,4),                                -- 结算后算
    tariff_id           BIGINT                        REFERENCES mgt_tariffs(id) ON DELETE SET NULL,
    reservation_id      INTEGER                       REFERENCES mgt_reservations_ocpp_1_6(id) ON DELETE SET NULL,
    bill_id             BIGINT                        REFERENCES mgt_bills(id) ON DELETE SET NULL,
    started_at          TIMESTAMPTZ                   NOT NULL DEFAULT now(),   -- Cloud 收到时间
    closed_at           TIMESTAMPTZ,
    raw_stop_payload    JSONB,                                          -- 完整原始 Stop 报文

    CONSTRAINT mgt_tx_meter_chk
        CHECK (meter_stop_wh IS NULL OR meter_stop_wh >= meter_start_wh),
    CONSTRAINT mgt_tx_state_time_chk
        CHECK (
            (state = 'Active'   AND stop_timestamp IS NULL) OR
            (state IN ('Completed', 'Faulted', 'Aborted') AND stop_timestamp IS NOT NULL)
        )
);

COMMENT ON TABLE  mgt_transactions_ocpp_1_6 IS 'OCPP 1.6 充电交易主表';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.id IS 'OCPP transactionId，CSMS 自行生成（OCPP 允许）';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.meter_start_wh IS '起始电表读数（Wh），BIGINT 避免 FLOAT 精度损失';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.meter_stop_wh IS '结束电表读数，StopTransaction 时填入';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.start_timestamp IS 'CP 上报的开始时间（RFC3339 解析），用于计费';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.started_at IS 'Cloud 收到 StartTransaction 的时间，用于 SLA 监控';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.state IS '交易状态机（CSMS 内部）：Active/Completed/Faulted/Aborted，与协议版本无关';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.stop_reason IS 'OCPP 1.6 Reason 枚举（11 个值）';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.total_kwh IS '结算度数（kWh），3 位小数';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.total_cost IS '结算费用（元），4 位小数';
COMMENT ON COLUMN mgt_transactions_ocpp_1_6.raw_stop_payload IS '完整原始 StopTransaction JSONB，审计/重算用';

CREATE INDEX idx_mgt_tx_cp
    ON mgt_transactions_ocpp_1_6(charge_point_id, started_at DESC);

CREATE INDEX idx_mgt_tx_idtag
    ON mgt_transactions_ocpp_1_6(id_tag, started_at DESC);

-- "哪些交易还没结束" 业务热查询
CREATE INDEX idx_mgt_tx_active
    ON mgt_transactions_ocpp_1_6(id)
    WHERE state = 'Active';

CREATE INDEX idx_mgt_tx_bill
    ON mgt_transactions_ocpp_1_6(bill_id)
    WHERE bill_id IS NOT NULL;

-- ------------------------------------------------------------
-- 4.2 mgt_meter_values_ocpp_1_6（充电度量样本）⭐
-- ------------------------------------------------------------
-- 用途：MeterValues 报文持久化
-- 关键决策：
--   - value_int BIGINT（Wh/W），不存 FLOAT
--   - 整型化 + context 区分不同时刻（Begin/Periodic/End）
--   - measurand / context 用 ENUM 约束（OCPP 1.6 值集）
--   - 高频写入，后期考虑分区
-- ------------------------------------------------------------
CREATE TABLE mgt_meter_values_ocpp_1_6 (
    id                BIGSERIAL       PRIMARY KEY,
    transaction_id    BIGINT          NOT NULL
                          REFERENCES mgt_transactions_ocpp_1_6(id) ON DELETE CASCADE,
    connector_id      INTEGER         NOT NULL,
    sample_timestamp  TIMESTAMPTZ     NOT NULL,            -- OCPP MeterValue.timestamp
    measurand         mgt_measurand NOT NULL,        -- OCPP 1.6 Measurand (15 个值)
    unit              VARCHAR(16)     NOT NULL DEFAULT 'Wh',
    value_int         BIGINT          NOT NULL,
    context           mgt_reading_context,           -- OCPP 1.6 ReadingContext (8 个值)
    format            VARCHAR(16)     DEFAULT 'Raw',
    received_at       TIMESTAMPTZ     NOT NULL DEFAULT now(),
    raw_sampled_value JSONB          NOT NULL              -- 完整原始 SampledValue
);

COMMENT ON TABLE  mgt_meter_values_ocpp_1_6 IS 'OCPP 1.6 度量样本表，MeterValues 报文持久化';
COMMENT ON COLUMN mgt_meter_values_ocpp_1_6.measurand IS 'OCPP 1.6 Measurand 枚举值之一（15 个值，PascalCase 无点号）';
COMMENT ON COLUMN mgt_meter_values_ocpp_1_6.value_int IS '采样值的整数形式（Wh 或 W），BIGINT 保精度';
COMMENT ON COLUMN mgt_meter_values_ocpp_1_6.context IS 'OCPP 1.6 ReadingContext 枚举值之一（8 个值）';
COMMENT ON COLUMN mgt_meter_values_ocpp_1_6.raw_sampled_value IS '原始 SampledValue JSONB，vendor_id 等未来字段';
COMMENT ON TABLE  mgt_meter_values_ocpp_1_6 IS '⚠️ 高频写入（每 5~15s 一条），100 桩 1 交易日 ≈ 864 万条';

CREATE INDEX idx_mgt_mv_tx_time
    ON mgt_meter_values_ocpp_1_6(transaction_id, sample_timestamp);

CREATE INDEX idx_mgt_mv_measurand
    ON mgt_meter_values_ocpp_1_6(transaction_id, measurand, sample_timestamp);

CREATE INDEX idx_mgt_mv_recent
    ON mgt_meter_values_ocpp_1_6(received_at);

-- ------------------------------------------------------------
-- 4.3 mgt_tariffs（费率方案）— P1
-- ------------------------------------------------------------
-- 用途：按 group 差异化计费
-- 关键决策：MVP 用平段（单一 price_per_kwh），P1 扩展 tariff_tiers
-- ------------------------------------------------------------
CREATE TABLE mgt_tariffs (
    id                  BIGSERIAL       PRIMARY KEY,
    name                VARCHAR(128)    NOT NULL UNIQUE,
    currency            VARCHAR(3)      NOT NULL DEFAULT 'CNY',    -- ISO 4217
    price_per_kwh       NUMERIC(8,4)   NOT NULL,                  -- 元/kWh
    description         TEXT,
    applicable_group_id BIGINT          REFERENCES mgt_id_tag_groups(id) ON DELETE SET NULL,
    valid_from          TIMESTAMPTZ     NOT NULL DEFAULT now(),
    valid_to            TIMESTAMPTZ,                              -- NULL=永久
    is_active           BOOLEAN         NOT NULL DEFAULT TRUE,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT now(),

    CONSTRAINT mgt_tariff_valid_chk
        CHECK (valid_to IS NULL OR valid_to > valid_from)
);

COMMENT ON TABLE  mgt_tariffs IS 'OCPP 计费费率，MVP 仅平段，P1 扩展阶梯/尖峰平谷';
COMMENT ON COLUMN mgt_tariffs.applicable_group_id IS '适用 idTag 组，NULL=默认费率';
COMMENT ON COLUMN mgt_tariffs.valid_to IS '失效时间，NULL=永久有效';

CREATE INDEX idx_mgt_tariff_group
    ON mgt_tariffs(applicable_group_id)
    WHERE is_active = TRUE;

-- ------------------------------------------------------------
-- 4.4 mgt_bills（账单）— P1
-- ------------------------------------------------------------
-- 用途：聚合多笔 transactions 为一张账单
-- ------------------------------------------------------------
CREATE TABLE mgt_bills (
    id           BIGSERIAL       PRIMARY KEY,
    id_tag       VARCHAR(64)     NOT NULL
                     REFERENCES mgt_id_tags_ocpp_1_6(id_tag),
    bill_number  VARCHAR(32)     NOT NULL UNIQUE,         -- 业务编号
    period_start TIMESTAMPTZ     NOT NULL,
    period_end   TIMESTAMPTZ     NOT NULL,
    total_kwh    NUMERIC(10,3)   NOT NULL,
    total_cost   NUMERIC(12,4)   NOT NULL,
    currency     VARCHAR(3)      NOT NULL DEFAULT 'CNY',
    status       VARCHAR(16)     NOT NULL DEFAULT 'Unpaid',
    paid_at      TIMESTAMPTZ,
    paid_via     VARCHAR(32),                              -- 'WeChatPay' | 'Alipay' | 'Stripe'
    created_at   TIMESTAMPTZ     NOT NULL DEFAULT now(),

    CONSTRAINT mgt_bill_status_chk
        CHECK (status IN ('Unpaid', 'Paid', 'Refunded', 'Cancelled'))
);

COMMENT ON TABLE mgt_bills IS '账单表，聚合多笔 transactions，MVP 不必实装';

-- ============================================================
-- 5. 域：配置与命令
-- ============================================================

-- ------------------------------------------------------------
-- 5.1 mgt_pending_commands_ocpp_1_6（CSMS 主动命令 outbox）⭐
-- ------------------------------------------------------------
-- 用途：RemoteStart/RemoteStop/Reset 等命令的 outbox pattern 跟踪
-- 关键决策：
--   - 必须在发 Kafka 之前 insert，发到 Kafka 后置 'Pending'
--   - 收到响应后更新 state，expires_at 用于超时巡检
--   - initiated_by 留运营账号外键
-- ------------------------------------------------------------
CREATE TABLE mgt_pending_commands_ocpp_1_6 (
    unique_id         VARCHAR(64)                       PRIMARY KEY,  -- OCPP messageId
    charge_point_id   VARCHAR(64)                       NOT NULL
                          REFERENCES mgt_charge_points_ocpp_1_6(id),
    action            VARCHAR(64)                       NOT NULL,
    request_payload   JSONB                             NOT NULL,    -- 完整请求体
    response_payload  JSONB,                                          -- 收到响应后填
    state             mgt_pending_command_state  NOT NULL DEFAULT 'Pending',
    sent_at           TIMESTAMPTZ                       NOT NULL DEFAULT now(),
    responded_at      TIMESTAMPTZ,
    expires_at        TIMESTAMPTZ                       NOT NULL,    -- sent_at + timeout
    error_code        VARCHAR(64),
    error_description TEXT,
    initiated_by      BIGINT                            REFERENCES mgt_operators(id),  -- NULL=系统

    CONSTRAINT mgt_pc_action_chk
        CHECK (action IN (
            'RemoteStartTransaction', 'RemoteStopTransaction', 'Reset',
            'UnlockConnector', 'ChangeAvailability', 'ChangeConfiguration',
            'GetConfiguration', 'GetDiagnostics', 'UpdateFirmware', 'TriggerMessage',
            'SendLocalList', 'ClearCache', 'ReserveNow', 'CancelReservation'
        )),
    CONSTRAINT mgt_pc_state_time_chk
        CHECK (
            (state = 'Pending'   AND responded_at IS NULL) OR
            (state IN ('Accepted', 'Rejected', 'Timeout', 'Cancelled') AND responded_at IS NOT NULL)
        )
);

COMMENT ON TABLE  mgt_pending_commands_ocpp_1_6 IS 'OCPP 1.6 CSMS 主动命令的 outbox 表（RemoteStart/Stop/Reset 等），保证 at-least-once + 超时管理';
COMMENT ON COLUMN mgt_pending_commands_ocpp_1_6.unique_id IS 'OCPP messageId，与 CP 回包配对';
COMMENT ON COLUMN mgt_pending_commands_ocpp_1_6.state IS 'outbox 状态机：Pending/Accepted/Rejected/Timeout/Cancelled';
COMMENT ON COLUMN mgt_pending_commands_ocpp_1_6.expires_at IS '超时巡检时间，sent_at + timeout';
COMMENT ON COLUMN mgt_pending_commands_ocpp_1_6.initiated_by IS '发起人（运营账号），NULL 表示系统自动';

CREATE INDEX idx_mgt_pc_cp_state
    ON mgt_pending_commands_ocpp_1_6(charge_point_id, state);

-- 超时巡检专用索引：扫待响应的过期命令
CREATE INDEX idx_mgt_pc_expires
    ON mgt_pending_commands_ocpp_1_6(expires_at)
    WHERE state = 'Pending';

-- ------------------------------------------------------------
-- 5.2 mgt_config_keys_ocpp_1_6（CSMS 下发的配置项）— P1
-- ------------------------------------------------------------
-- 用途：ChangeConfiguration / GetConfiguration
-- ------------------------------------------------------------
CREATE TABLE mgt_config_keys_ocpp_1_6 (
    id          BIGSERIAL       PRIMARY KEY,
    key         VARCHAR(128)    NOT NULL UNIQUE,         -- OCPP ConfigurationKey
    value       TEXT,                                        -- 文本（OCPP value 是 String）
    readonly    BOOLEAN         NOT NULL DEFAULT FALSE,   -- OCPP readonly
    description TEXT,
    updated_at  TIMESTAMPTZ     NOT NULL DEFAULT now()
);

COMMENT ON TABLE mgt_config_keys_ocpp_1_6 IS 'OCPP 1.6 ConfigurationKey 表，CSMS 通过 GetConfiguration 推给 CP';

-- ------------------------------------------------------------
-- 5.3 mgt_firmware_ocpp_1_6（固件版本）— P1
-- ------------------------------------------------------------
-- 用途：UpdateFirmware 推送 + FirmwareStatusNotification 跟踪
-- ------------------------------------------------------------
CREATE TABLE mgt_firmware_ocpp_1_6 (
    id              BIGSERIAL       PRIMARY KEY,
    name            VARCHAR(128)    NOT NULL,
    version         VARCHAR(64)     NOT NULL,
    location        TEXT            NOT NULL,                -- 下载 URL（指向 S3/MinIO）
    checksum        VARCHAR(128),                            -- MD5 / SHA256
    signing_cert    TEXT,                                     -- P1 签名证书
    retrieve_date   TIMESTAMPTZ,                              -- UpdateFirmware.retrieveDate
    install_date    TIMESTAMPTZ,                              -- UpdateFirmware.installDate
    status          VARCHAR(16)     NOT NULL DEFAULT 'Available',
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),

    UNIQUE (name, version),
    CONSTRAINT mgt_fw_status_chk
        CHECK (status IN (
            'Available', 'Downloading', 'Downloaded', 'Installing',
            'Installed', 'InstallationFailed', 'DownloadFailed'
        ))
);

COMMENT ON TABLE  mgt_firmware_ocpp_1_6 IS 'OCPP 1.6 固件版本表，配合 UpdateFirmware 流程';
COMMENT ON COLUMN mgt_firmware_ocpp_1_6.location IS '下载 URL，指向对象存储（S3/MinIO）';

-- ============================================================
-- 6. 域：扩展（P1/P2）
-- ============================================================

-- ------------------------------------------------------------
-- 6.1 mgt_reservations_ocpp_1_6（OCPP 预约）— P2
-- ------------------------------------------------------------
-- 用途：ReserveNow / CancelReservation
-- ------------------------------------------------------------
CREATE TABLE mgt_reservations_ocpp_1_6 (
    id                       INTEGER       PRIMARY KEY,        -- OCPP reservationId
    charge_point_id          VARCHAR(64)   NOT NULL
                                REFERENCES mgt_charge_points_ocpp_1_6(id),
    connector_id             INTEGER,
    id_tag                   VARCHAR(64)   REFERENCES mgt_id_tags_ocpp_1_6(id_tag),
    expiry_datetime          TIMESTAMPTZ  NOT NULL,
    state                    VARCHAR(16)   NOT NULL DEFAULT 'Waiting',  -- 'Waiting' | 'Accepted' | 'Used' | 'Cancelled'
    parent_reservation_id    INTEGER       REFERENCES mgt_reservations_ocpp_1_6(id),  -- 嵌套预约
    created_at               TIMESTAMPTZ  NOT NULL DEFAULT now(),

    CONSTRAINT mgt_resv_state_chk
        CHECK (state IN ('Waiting', 'Accepted', 'Used', 'Cancelled'))
);

COMMENT ON TABLE mgt_reservations_ocpp_1_6 IS 'OCPP 1.6 充电预约表';

-- ------------------------------------------------------------
-- 6.2 mgt_charging_profiles_ocpp_1_6（充电功率曲线）— P2
-- ------------------------------------------------------------
-- 用途：SetChargingProfile / GetCompositeSchedule
-- schedule 是复杂嵌套结构，JSONB 合适
-- ------------------------------------------------------------
CREATE TABLE mgt_charging_profiles_ocpp_1_6 (
    id                  BIGSERIAL       PRIMARY KEY,
    charge_point_id     VARCHAR(64)     NOT NULL
                          REFERENCES mgt_charge_points_ocpp_1_6(id),
    connector_id        INTEGER,                                    -- NULL=整个 CP
    profile_id          INTEGER         NOT NULL,                 -- OCPP chargingProfileId
    stack_level         INTEGER         NOT NULL DEFAULT 0,
    purpose             VARCHAR(16)     NOT NULL,
    kind                VARCHAR(16)     NOT NULL,
    valid_from          TIMESTAMPTZ,
    valid_to            TIMESTAMPTZ,
    schedule            JSONB           NOT NULL,                 -- OCPP ChargingSchedule
    is_active           BOOLEAN         NOT NULL DEFAULT TRUE,

    UNIQUE (charge_point_id, profile_id),
    CONSTRAINT mgt_cp_profile_purpose_chk
        CHECK (purpose IN (
            'ChargePointMaxProfile', 'ChargingStationMaxProfile',
            'TxDefaultProfile', 'TxProfile'
        )),
    CONSTRAINT mgt_cp_profile_kind_chk
        CHECK (kind IN ('Absolute', 'Recurring', 'Relative'))
);

COMMENT ON TABLE mgt_charging_profiles_ocpp_1_6 IS 'OCPP 1.6 充电功率曲线配置';

-- ============================================================
-- 7. 域：系统与运维
-- ============================================================

-- ------------------------------------------------------------
-- 7.1 mgt_sent_messages（消息去重 / idempotency）⭐
-- ------------------------------------------------------------
-- 用途：Kafka 至少一次语义下识别重复消息
-- 关键决策：
--   - unique_id PK 保证幂等
--   - raw_envelope JSONB 保留原文（审计/重放）
--   - direction 区分 in/out
--   - ⚠️ 高写入量，后期定期导出
-- ------------------------------------------------------------
CREATE TABLE mgt_sent_messages (
    unique_id        VARCHAR(64)     PRIMARY KEY,             -- OCPP messageId
    gateway_id       VARCHAR(64)     NOT NULL,
    charge_point_id  VARCHAR(64),
    direction        VARCHAR(8)      NOT NULL,               -- 'inbound' | 'outbound'
    action           VARCHAR(64),
    message_type     VARCHAR(16)     NOT NULL,               -- 'Call' | 'CallResult' | 'CallError'
    raw_envelope     JSONB           NOT NULL,
    received_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),
    processed_at     TIMESTAMPTZ,

    CONSTRAINT mgt_sm_dir_chk
        CHECK (direction IN ('inbound', 'outbound'))
);

COMMENT ON TABLE  mgt_sent_messages IS '消息去重 / idempotency 屏障，Kafka 至少一次语义的防线';
COMMENT ON COLUMN mgt_sent_messages.unique_id IS 'OCPP messageId，PK 保证 INSERT ON CONFLICT DO NOTHING';
COMMENT ON COLUMN mgt_sent_messages.direction IS 'inbound=CP→Cloud, outbound=Cloud→CP';
COMMENT ON TABLE  mgt_sent_messages IS '⚠️ 写入量极大（每条消息都写），P1 定期导出到冷存储';

CREATE INDEX idx_mgt_sm_charge_time
    ON mgt_sent_messages(charge_point_id, received_at DESC);

CREATE INDEX idx_mgt_sm_recent
    ON mgt_sent_messages(received_at);

-- ------------------------------------------------------------
-- 7.2 mgt_gateway_health（Gateway 实例健康）⭐
-- ------------------------------------------------------------
-- 用途：Cloud 知道每个 Gateway 是否在线
-- ------------------------------------------------------------
CREATE TABLE mgt_gateway_health (
    gateway_id          VARCHAR(64)     PRIMARY KEY,
    last_seen_at        TIMESTAMPTZ     NOT NULL,
    status              VARCHAR(16)     NOT NULL,             -- 'Healthy' | 'Degraded' | 'Down'
    last_kafka_offset   JSONB,                                -- 各 topic 消费 offset
    pending_count       INTEGER         NOT NULL DEFAULT 0,  -- Gateway 端 pending 数
    updated_at          TIMESTAMPTZ     NOT NULL DEFAULT now(),

    CONSTRAINT mgt_gh_status_chk
        CHECK (status IN ('Healthy', 'Degraded', 'Down'))
);

COMMENT ON TABLE mgt_gateway_health IS 'Gateway 实例健康状态表（每个 Gateway 一行）';
COMMENT ON COLUMN mgt_gateway_health.last_kafka_offset IS '各 Kafka topic 的消费 offset，JSON 存';

-- ------------------------------------------------------------
-- 7.3 mgt_audit_logs（操作审计）— P1
-- ------------------------------------------------------------
-- 用途：合规审计，运营操作留痕
-- ------------------------------------------------------------
CREATE TABLE mgt_audit_logs (
    id                  BIGSERIAL       PRIMARY KEY,
    actor_operator_id   BIGINT          REFERENCES mgt_operators(id),
    actor_type          VARCHAR(16)     NOT NULL DEFAULT 'operator',  -- 'operator' | 'system' | 'cp'
    action              VARCHAR(64)     NOT NULL,                       -- 'create_id_tag' | 'remote_start' ...
    target_type         VARCHAR(32),                                    -- 'id_tag' | 'charge_point' | 'transaction'
    target_id           VARCHAR(64),
    payload             JSONB,
    ip_address          INET,
    user_agent          TEXT,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT now()
);

COMMENT ON TABLE mgt_audit_logs IS '运营操作审计表（合规要求）';

CREATE INDEX idx_mgt_audit_actor_time
    ON mgt_audit_logs(actor_operator_id, created_at DESC);

CREATE INDEX idx_mgt_audit_target
    ON mgt_audit_logs(target_type, target_id);

-- ------------------------------------------------------------
-- 7.4 mgt_dead_letter_queue（死信队列）— P1
-- ------------------------------------------------------------
-- 用途：处理失败的消息暂存，供运维重放
-- ------------------------------------------------------------
CREATE TABLE mgt_dead_letter_queue (
    id              BIGSERIAL       PRIMARY KEY,
    original_topic  VARCHAR(128)    NOT NULL,
    raw_message     BYTEA           NOT NULL,                 -- Kafka 原始字节
    parse_error     TEXT,                                       -- 解析失败
    handler_error   TEXT,                                       -- 处理失败
    retry_count     INTEGER         NOT NULL DEFAULT 0,
    last_retry_at   TIMESTAMPTZ,
    failed_at       TIMESTAMPTZ     NOT NULL DEFAULT now(),
    is_resolved     BOOLEAN         NOT NULL DEFAULT FALSE,
    resolved_at     TIMESTAMPTZ,
    resolution      TEXT
);

COMMENT ON TABLE mgt_dead_letter_queue IS '处理失败的死信队列，运维定期清理/重放';

CREATE INDEX idx_mgt_dlq_unresolved
    ON mgt_dead_letter_queue(failed_at)
    WHERE is_resolved = FALSE;

-- ------------------------------------------------------------
-- 7.5 mgt_message_log_ocpp_1_6（消息流可观测性）— P1
-- ------------------------------------------------------------
-- 用途：trace 每次 OCPP 消息，可观测性 + SLA 监控
-- 与 mgt_sent_messages 区别：前者按生命周期跟踪 + 配对；后者按 idempotency 去重
-- 命名：原名 mgt_ocpp_message_log，重命名后去掉 ocpp_ 前缀避免与 _ocpp_1_6 后缀重复
-- ------------------------------------------------------------
CREATE TABLE mgt_message_log_ocpp_1_6 (
    id              BIGSERIAL       PRIMARY KEY,
    charge_point_id VARCHAR(64)     NOT NULL
                      REFERENCES mgt_charge_points_ocpp_1_6(id),
    direction       VARCHAR(8)      NOT NULL,                -- 'inbound' | 'outbound'
    message_type    VARCHAR(16)     NOT NULL,                -- 'Call' | 'CallResult' | 'CallError'
    action          VARCHAR(64),                              -- 'BootNotification' 等
    unique_id       VARCHAR(64)     NOT NULL,
    request_id      VARCHAR(64),                              -- 关联请求 ID（Call ↔ CallResult 配对）
    latency_ms      INTEGER,                                  -- 响应延迟（仅 response 方向）
    status          VARCHAR(16),                              -- 'ok' | 'error' | 'timeout'
    error_code      VARCHAR(64),
    payload         JSONB,
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),

    CONSTRAINT mgt_ml_dir_chk
        CHECK (direction IN ('inbound', 'outbound'))
);

COMMENT ON TABLE  mgt_message_log_ocpp_1_6 IS 'OCPP 1.6 消息流可观测性日志，按消息生命周期跟踪';
COMMENT ON COLUMN mgt_message_log_ocpp_1_6.request_id IS 'Call→CallResult 配对：Call 的 unique_id';
COMMENT ON COLUMN mgt_message_log_ocpp_1_6.latency_ms IS '响应延迟毫秒（仅 response 方向）';

CREATE INDEX idx_mgt_ml_cp_time
    ON mgt_message_log_ocpp_1_6(charge_point_id, created_at DESC);

CREATE INDEX idx_mgt_ml_action
    ON mgt_message_log_ocpp_1_6(action, created_at DESC);

CREATE INDEX idx_mgt_ml_reqid
    ON mgt_message_log_ocpp_1_6(request_id)
    WHERE request_id IS NOT NULL;

-- ============================================================
-- 提交
-- ============================================================
COMMIT;

-- ============================================================
-- 迁移计划（建议拆分）
-- ============================================================
-- migrations/001_init.sql
--   mgt_operators
--   mgt_charge_points_ocpp_1_6
--   mgt_connectors_ocpp_1_6
--   mgt_id_tag_groups
--   mgt_id_tags_ocpp_1_6
--   mgt_transactions_ocpp_1_6
--   mgt_meter_values_ocpp_1_6
--   mgt_sent_messages
--   mgt_gateway_health
--   mgt_pending_commands_ocpp_1_6
--
-- migrations/002_billing.sql
--   mgt_tariffs
--   mgt_bills
--
-- migrations/003_config.sql
--   mgt_config_keys_ocpp_1_6
--   mgt_firmware_ocpp_1_6
--   mgt_reservations_ocpp_1_6
--   mgt_charging_profiles_ocpp_1_6
--
-- migrations/004_ops.sql
--   mgt_audit_logs
--   mgt_dead_letter_queue
--   mgt_message_log_ocpp_1_6
