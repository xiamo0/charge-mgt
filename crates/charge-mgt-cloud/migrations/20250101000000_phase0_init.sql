-- charge-mgt-cloud Phase 0 初始化迁移
--
-- 创建 OCPP 协议绑定的最小表集合（Phase 0）：
--   - charge_mgt_charge_points_ocpp_1_6       （BootNotification 注册/更新）
--   - charge_mgt_connectors_ocpp_1_6          （StatusNotification 状态）
--   - charge_mgt_sent_messages_ocpp_1_6       （idempotency 屏障）
--
-- 后续阶段会添加：transactions, meter_values, pending_commands 等

BEGIN;

-- ============================================================
-- ENUM 类型（OCPP 1.6 规范值集）
-- ============================================================

-- OCPP 1.6 ChargePointStatus（9 个枚举值）
CREATE TYPE charge_mgt_connector_status AS ENUM (
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

-- ============================================================
-- charge_mgt_charge_points_ocpp_1_6（充电桩主表）
-- ============================================================
CREATE TABLE charge_mgt_charge_points_ocpp_1_6 (
    id                      VARCHAR(64)     PRIMARY KEY,      -- 业务 ID = chargeBoxSerialNumber
    gateway_id              VARCHAR(64)     NOT NULL,         -- 来自 CloudMessage.gateway_id
    gateway_ip              VARCHAR(64),                      -- 来自 CloudMessage.gateway_ip
    vendor                  VARCHAR(128)    NOT NULL,
    model                   VARCHAR(128)    NOT NULL,
    serial_number           VARCHAR(128),
    charge_box_serial       VARCHAR(128),
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

    CONSTRAINT charge_mgt_cps_status_chk
        CHECK (ocpp_status IN ('Registered', 'Online', 'Offline', 'Faulted', 'Disabled')),
    CONSTRAINT charge_mgt_cps_protocol_chk
        CHECK (protocol_version IN ('OCPP-1.6')),
    CONSTRAINT charge_mgt_cps_hb_interval_chk
        CHECK (heartbeat_interval_secs BETWEEN 5 AND 86400)
);

COMMENT ON TABLE  charge_mgt_charge_points_ocpp_1_6 IS 'OCPP 1.6 充电桩主表，BootNotification 注册的物理设备';
COMMENT ON COLUMN charge_mgt_charge_points_ocpp_1_6.id IS '业务主键，来自 OCPP chargeBoxSerialNumber';
COMMENT ON COLUMN charge_mgt_charge_points_ocpp_1_6.gateway_id IS '该桩经由哪个 Gateway 接入';
COMMENT ON COLUMN charge_mgt_charge_points_ocpp_1_6.ocpp_status IS 'Registered=已注册未上线, Online=心跳正常, Offline=心跳超时, Faulted=自报故障, Disabled=运营禁用';
COMMENT ON COLUMN charge_mgt_charge_points_ocpp_1_6.last_heartbeat_at IS '最近一次 Heartbeat 到达时间，用于超时巡检';

CREATE INDEX idx_charge_mgt_cps_gateway
    ON charge_mgt_charge_points_ocpp_1_6(gateway_id)
    WHERE is_deleted = FALSE;
CREATE INDEX idx_charge_mgt_cps_vendor
    ON charge_mgt_charge_points_ocpp_1_6(vendor, model)
    WHERE is_deleted = FALSE;
CREATE INDEX idx_charge_mgt_cps_status
    ON charge_mgt_charge_points_ocpp_1_6(ocpp_status)
    WHERE is_deleted = FALSE;
CREATE INDEX idx_charge_mgt_cps_hb_late
    ON charge_mgt_charge_points_ocpp_1_6(last_heartbeat_at)
    WHERE is_deleted = FALSE AND ocpp_status = 'Online';

-- ============================================================
-- charge_mgt_connectors_ocpp_1_6（充电连接器）
-- ============================================================
CREATE TABLE charge_mgt_connectors_ocpp_1_6 (
    id                BIGSERIAL       PRIMARY KEY,
    charge_point_id   VARCHAR(64)     NOT NULL
                          REFERENCES charge_mgt_charge_points_ocpp_1_6(id) ON DELETE CASCADE,
    connector_id      INTEGER         NOT NULL,             -- OCPP connectorId
    status            charge_mgt_connector_status NOT NULL DEFAULT 'Available',
    error_code        VARCHAR(32)     NOT NULL DEFAULT 'NoError',
    vendor_id         VARCHAR(128),
    vendor_error_code VARCHAR(128),
    info              TEXT,
    last_status_at    TIMESTAMPTZ,
    updated_at        TIMESTAMPTZ     NOT NULL DEFAULT now(),

    UNIQUE (charge_point_id, connector_id)
);

COMMENT ON TABLE  charge_mgt_connectors_ocpp_1_6 IS 'OCPP 1.6 充电连接器，StatusNotification 粒度';
COMMENT ON COLUMN charge_mgt_connectors_ocpp_1_6.connector_id IS 'OCPP connectorId，0=整个桩，1+=物理连接器';
COMMENT ON COLUMN charge_mgt_connectors_ocpp_1_6.status IS 'OCPP 1.6 ChargePointStatus 9 个枚举值之一';

CREATE INDEX idx_charge_mgt_conn_cp
    ON charge_mgt_connectors_ocpp_1_6(charge_point_id);

-- ============================================================
-- charge_mgt_sent_messages_ocpp_1_6（消息去重 / idempotency 屏障）
-- ============================================================
CREATE TABLE charge_mgt_sent_messages_ocpp_1_6 (
    unique_id        VARCHAR(64)     PRIMARY KEY,             -- OCPP messageId
    gateway_id       VARCHAR(64)     NOT NULL,
    charge_point_id  VARCHAR(64)     NOT NULL,
    direction        VARCHAR(8)      NOT NULL,               -- 'inbound' | 'outbound'
    action           VARCHAR(64)     NOT NULL,
    message_type     VARCHAR(16)     NOT NULL,               -- 'Call' | 'CallResult' | 'CallError'
    received_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),
    processed_at     TIMESTAMPTZ,

    CONSTRAINT charge_mgt_sm_dir_chk
        CHECK (direction IN ('inbound', 'outbound'))
);

COMMENT ON TABLE  charge_mgt_sent_messages_ocpp_1_6 IS '消息去重 / idempotency 屏障，Kafka 至少一次语义的防线';
COMMENT ON COLUMN charge_mgt_sent_messages_ocpp_1_6.unique_id IS 'OCPP messageId，PK 保证 INSERT ON CONFLICT DO NOTHING';

CREATE INDEX idx_charge_mgt_sm_cp_time
    ON charge_mgt_sent_messages_ocpp_1_6(charge_point_id, received_at DESC);

CREATE INDEX idx_charge_mgt_sm_recent
    ON charge_mgt_sent_messages_ocpp_1_6(received_at);

COMMIT;
