-- charge-mgt-cloud Phase 0 初始化迁移
--
-- 创建 OCPP 协议绑定的最小表集合（Phase 0）：
--   - charge_mgt_charge_points_ocpp_1_6       （BootNotification 注册/更新）
--   - charge_mgt_connectors_ocpp_1_6          （StatusNotification 状态）
--   - charge_mgt_sent_messages_ocpp_1_6       （idempotency 屏障）
--
-- 后续阶段会添加：transactions, meter_values, pending_commands 等

BEGIN;
-- 充电桩设备信息表
CREATE TABLE charge_point_ocpp16 (
                                     charge_point_id VARCHAR(64) NOT NULL,
                                     station_id BIGINT NOT NULL,
                                     charge_point_vendor VARCHAR(64),
                                     charge_point_model VARCHAR(64),
                                     charge_box_serial_number VARCHAR(64),
                                     charge_point_serial_number VARCHAR(64),
                                     firmware_version VARCHAR(64),
                                     iccid VARCHAR(64),
                                     imsi VARCHAR(64),
                                     meter_type VARCHAR(64),
                                     meter_serial_number VARCHAR(64),
                                     status VARCHAR(64) NOT NULL,
                                     error_code VARCHAR(64),

    -- 运维与系统字段
                                     install_date DATE,
                                     is_deleted SMALLINT NOT NULL DEFAULT 0,
                                     create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                     update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- 设置主键（OCPP协议中充电桩唯一标识是核心主键）
                                     CONSTRAINT pk_charge_point PRIMARY KEY (charge_point_id)
);

COMMENT ON TABLE charge_point_ocpp16 IS '充电桩设备信息表';

COMMENT ON COLUMN charge_point_ocpp16.charge_point_id IS '充电桩唯一标识(ws请求地址中夹带)';
COMMENT ON COLUMN charge_point_ocpp16.station_id IS '所属充电站ID';
COMMENT ON COLUMN charge_point_ocpp16.charge_point_vendor IS '厂商名称';
COMMENT ON COLUMN charge_point_ocpp16.charge_point_model IS '型号';
COMMENT ON COLUMN charge_point_ocpp16.charge_box_serial_number IS '充电盒序列号';
COMMENT ON COLUMN charge_point_ocpp16.charge_point_serial_number IS '充电桩序列号';
COMMENT ON COLUMN charge_point_ocpp16.firmware_version IS '固件版本';
COMMENT ON COLUMN charge_point_ocpp16.iccid IS 'SIM卡ICCID';
COMMENT ON COLUMN charge_point_ocpp16.imsi IS 'SIM卡IMSI';
COMMENT ON COLUMN charge_point_ocpp16.meter_type IS '电能表型号';
COMMENT ON COLUMN charge_point_ocpp16.meter_serial_number IS '电能表序列号';
COMMENT ON COLUMN charge_point_ocpp16.status IS '工作状态(对应OCPP ChargePointStatus枚举)';
COMMENT ON COLUMN charge_point_ocpp16.error_code IS '错误码(对应OCPP ChargePointErrorCode枚举)';
COMMENT ON COLUMN charge_point_ocpp16.install_date IS '安装投运日期';
COMMENT ON COLUMN charge_point_ocpp16.is_deleted IS '逻辑删除: 0-正常, 1-已删除';
COMMENT ON COLUMN charge_point_ocpp16.create_time IS '创建时间';
COMMENT ON COLUMN charge_point_ocpp16.update_time IS '更新时间';

-- 3. 创建常用查询索引
CREATE INDEX idx_charge_point_station_id ON charge_point_ocpp16 (station_id);
-- 充电枪(连接器)表
CREATE TABLE charge_connector_ocpp16 (
                                         charge_point_id VARCHAR(64) NOT NULL,
                                         connector_id VARCHAR(32) NOT NULL,
                                         connector_type SMALLINT NOT NULL,

    -- 枪的独立状态
                                         status VARCHAR(64) NOT NULL,
                                         error_code VARCHAR(64),
                                         last_heartbeat_time TIMESTAMP,

                                         create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                         update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- 设置联合主键（一个桩的某把枪是唯一的）
                                         CONSTRAINT pk_charge_connector PRIMARY KEY (charge_point_id, connector_id)
);

-- 添加表和字段注释
COMMENT ON TABLE charge_connector_ocpp16 IS '充电枪(连接器)信息表';

COMMENT ON COLUMN charge_connector_ocpp16.charge_point_id IS '充电桩唯一标识(ws请求地址中夹带)';
COMMENT ON COLUMN charge_connector_ocpp16.connector_id IS '枪编号(如: 1, 2 或 OCPP中的ConnectorId)';
COMMENT ON COLUMN charge_connector_ocpp16.connector_type IS '接口类型: 1-国标直流, 2-国标交流';
COMMENT ON COLUMN charge_connector_ocpp16.status IS '枪状态(对应OCPP ChargePointStatus枚举)';
COMMENT ON COLUMN charge_connector_ocpp16.error_code IS '错误码(对应OCPP ChargePointErrorCode枚举)';
COMMENT ON COLUMN charge_connector_ocpp16.last_heartbeat_time IS '枪级状态最后更新时间';
COMMENT ON COLUMN charge_connector_ocpp16.create_time IS '记录创建时间';
COMMENT ON COLUMN charge_connector_ocpp16.update_time IS '记录更新时间';


-- 用户身份标签/鉴权表
CREATE TABLE identity_info (
                               id BIGSERIAL PRIMARY KEY,
                               user_id BIGINT,
                               tag_id VARCHAR(64) NOT NULL,
                               tag_type SMALLINT NOT NULL,

    -- 鉴权与状态控制
                               status SMALLINT NOT NULL DEFAULT 1,
                               expire_time TIMESTAMP,

    -- 运维与系统字段
                               create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                               update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. 添加表和字段注释
COMMENT ON TABLE identity_info IS '用户身份标签/鉴权表';

COMMENT ON COLUMN identity_info.id IS '主键ID';
COMMENT ON COLUMN identity_info.user_id IS '关联的用户ID(可为空，支持未绑定或共享Tag)';
COMMENT ON COLUMN identity_info.tag_id IS '标签值(如RFID卡号、虚拟Token字符串)';
COMMENT ON COLUMN identity_info.tag_type IS '标签类型: 1-RFID卡, 2-二维码, 3-车牌号, 4-App虚拟Token';

COMMENT ON COLUMN identity_info.status IS '鉴权状态: 1-有效(Accepted), 2-无效/挂失(Blocked), 3-已过期';
COMMENT ON COLUMN identity_info.expire_time IS '标签有效期(用于临时卡或月卡管理)';

COMMENT ON COLUMN identity_info.create_time IS '创建时间';
COMMENT ON COLUMN identity_info.update_time IS '更新时间';

-- 3. 创建核心索引
CREATE UNIQUE INDEX uk_identity_info_tag_id ON identity_info (tag_id);



-- 充电事务表
CREATE TABLE charge_transaction_ocpp16 (
                                           id BIGSERIAL PRIMARY KEY,
                                           transaction_id VARCHAR(64) NOT NULL,

    -- 1. 核心关联实体
                                           user_id BIGINT,
                                           tag_id VARCHAR(64) NOT NULL,
                                           charge_point_id VARCHAR(32) NOT NULL,
                                           connector_id VARCHAR(32) NOT NULL,

    -- 2. 核心业务状态机
                                           status SMALLINT NOT NULL DEFAULT 0,
                                           stop_reason VARCHAR(32),

    -- 3. 时间与计量数据
                                           start_time TIMESTAMP NOT NULL,
                                           end_time TIMESTAMP,
                                           meter_start DECIMAL(10,3) NOT NULL,
                                           meter_stop DECIMAL(10,3),
                                           total_energy DECIMAL(10,3),

    -- 4. 计费与财务结算
                                           total_amount DECIMAL(10,2) DEFAULT 0.00,
                                           electricity_fee DECIMAL(10,2) DEFAULT 0.00,
                                           service_fee DECIMAL(10,2) DEFAULT 0.00,
                                           payment_status SMALLINT NOT NULL DEFAULT 0,

    -- 5. 离线与数据同步控制
                                           is_offline_sync SMALLINT NOT NULL DEFAULT 0,
                                           sync_attempts INT DEFAULT 0,
                                           last_sync_time TIMESTAMP,

    -- 6. 系统基础字段
                                           create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                           update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. 添加表和字段注释 (PostgreSQL 语法)
COMMENT ON TABLE charge_transaction_ocpp16 IS '充电事务/订单核心表';

COMMENT ON COLUMN charge_transaction_ocpp16.id IS '主键ID';
COMMENT ON COLUMN charge_transaction_ocpp16.transaction_id IS 'OCPP事务ID(桩端生成，全局唯一)';

COMMENT ON COLUMN charge_transaction_ocpp16.user_id IS '关联的用户ID';
COMMENT ON COLUMN charge_transaction_ocpp16.tag_id IS '触发充电的身份标签(RFID/Token)';
COMMENT ON COLUMN charge_transaction_ocpp16.charge_point_id IS '充电桩唯一标识';
COMMENT ON COLUMN charge_transaction_ocpp16.connector_id IS '充电枪编号(如: 1, 2)';

COMMENT ON COLUMN charge_transaction_ocpp16.status IS '事务状态: 0-进行中, 1-正常结束, 2-异常中断, 3-离线补传待处理';
COMMENT ON COLUMN charge_transaction_ocpp16.stop_reason IS '结束原因(EVDisconnected, HardReset, Local, Remote, DeAuthorized等)';

COMMENT ON COLUMN charge_transaction_ocpp16.start_time IS '充电开始时间(桩端上报)';
COMMENT ON COLUMN charge_transaction_ocpp16.end_time IS '充电结束时间';
COMMENT ON COLUMN charge_transaction_ocpp16.meter_start IS '起始电表读数(kWh)';
COMMENT ON COLUMN charge_transaction_ocpp16.meter_stop IS '结束电表读数(kWh)';
COMMENT ON COLUMN charge_transaction_ocpp16.total_energy IS '总充电量(kWh)';

COMMENT ON COLUMN charge_transaction_ocpp16.total_amount IS '总费用(元)';
COMMENT ON COLUMN charge_transaction_ocpp16.electricity_fee IS '电费(元)';
COMMENT ON COLUMN charge_transaction_ocpp16.service_fee IS '服务费(元)';
COMMENT ON COLUMN charge_transaction_ocpp16.payment_status IS '支付状态: 0-未支付, 1-已支付, 2-支付失败, 3-已退款';

COMMENT ON COLUMN charge_transaction_ocpp16.is_offline_sync IS '是否为离线补传数据: 0-实时上报, 1-离线补传';
COMMENT ON COLUMN charge_transaction_ocpp16.sync_attempts IS '数据同步重试次数';
COMMENT ON COLUMN charge_transaction_ocpp16.last_sync_time IS '最后一次同步/更新时间';

COMMENT ON COLUMN charge_transaction_ocpp16.create_time IS '记录创建时间';
COMMENT ON COLUMN charge_transaction_ocpp16.update_time IS '记录更新时间';

-- 3. 创建核心索引
CREATE UNIQUE INDEX uk_charge_transaction_txn_id ON charge_transaction_ocpp16 (transaction_id);
CREATE INDEX idx_charge_transaction_user_id ON charge_transaction_ocpp16 (user_id);
CREATE INDEX idx_charge_transaction_cp_connector ON charge_transaction_ocpp16 (charge_point_id, connector_id);
CREATE INDEX idx_charge_transaction_start_time ON charge_transaction_ocpp16 (start_time);




-- 充电预约表
CREATE TABLE charge_reservation_ocpp16 (
    -- 基础标识
                                           reservation_id BIGSERIAL PRIMARY KEY,
                                           user_id BIGINT NOT NULL,                -- 关联用户表
                                           charge_point_id VARCHAR(32) NOT NULL,         -- 关联充电桩唯一编码
                                           connector_id VARCHAR(32),                  -- 关联充电枪编号(可选，精确到枪则必填)
                                           tag_id VARCHAR(64),                     -- 预约时绑定的RFID/Token(用于到达后直接启动)

    -- 核心预约时间窗口
                                           start_time TIMESTAMP NOT NULL,          -- 预约开始时间
                                           end_time TIMESTAMP NOT NULL,            -- 预约结束时间

    -- 业务状态与履约
                                           status SMALLINT NOT NULL DEFAULT 0,     -- 预约状态: 0-待履约, 1-进行中(已扫码), 2-已完成, 3-已取消, 4-已违约
                                           transaction_id VARCHAR(64),             -- 关联实际的充电事务ID(充电启动后回写此字段)
                                           cancel_reason VARCHAR(255),             -- 取消原因(用户主动取消或系统超时释放)

    -- 系统审计字段
                                           created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                           updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);




-- 智能充电配置表
CREATE TABLE smart_charge_profile_ocpp16 (
                                             id BIGSERIAL PRIMARY KEY,
                                             charge_point_id VARCHAR(64) NOT NULL,      -- 关联充电桩唯一标识
                                             connector_id VARCHAR(32),                     -- 关联充电枪编号(0代表整个桩)

    -- OCPP 1.6 核心协议字段
                                             charging_profile_id INT NOT NULL,          -- 桩端生成的Profile唯一ID
                                             stack_level SMALLINT NOT NULL,             -- 策略优先级(数字越大优先级越高)
                                             charging_profile_purpose VARCHAR(32) NOT NULL, -- 策略目的: ChargePointMaxProfile, TxDefaultProfile, TxProfile
                                             charging_profile_kind VARCHAR(32) NOT NULL,    -- 策略类型: Absolute, Recurring, Relative

    -- 业务调度字段
                                             start_time TIMESTAMP,                      -- 计划生效时间
                                             duration INT,                              -- 计划持续时间(秒)
                                             max_power_kw DECIMAL(10,3),                -- 限制的最大功率(kW)
                                             max_current_a DECIMAL(10,3),               -- 限制的最大电流(A)

    -- 状态与审计
                                             status SMALLINT NOT NULL DEFAULT 0,        -- 下发状态: 0-待下发, 1-已接受(Accepted), 2-已拒绝(Rejected)
                                             create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                             update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. 添加表和字段注释
COMMENT ON TABLE smart_charge_profile_ocpp16 IS '智能充电配置/策略表';

COMMENT ON COLUMN smart_charge_profile_ocpp16.charge_point_id IS '充电桩唯一标识';
COMMENT ON COLUMN smart_charge_profile_ocpp16.connector_id IS '充电枪编号(0代表整个桩)';
COMMENT ON COLUMN smart_charge_profile_ocpp16.charging_profile_id IS '桩端生成的Profile唯一ID';
COMMENT ON COLUMN smart_charge_profile_ocpp16.stack_level IS '策略优先级(数字越大优先级越高)';
COMMENT ON COLUMN smart_charge_profile_ocpp16.charging_profile_purpose IS '策略目的: ChargePointMaxProfile, TxDefaultProfile, TxProfile';
COMMENT ON COLUMN smart_charge_profile_ocpp16.charging_profile_kind IS '策略类型: Absolute(绝对时间), Recurring(周期), Relative(相对开始时间)';
COMMENT ON COLUMN smart_charge_profile_ocpp16.start_time IS '计划生效时间';
COMMENT ON COLUMN smart_charge_profile_ocpp16.duration IS '计划持续时间(秒)';
COMMENT ON COLUMN smart_charge_profile_ocpp16.max_power_kw IS '限制的最大功率(kW)';
COMMENT ON COLUMN smart_charge_profile_ocpp16.max_current_a IS '限制的最大电流(A)';
COMMENT ON COLUMN smart_charge_profile_ocpp16.status IS '下发状态: 0-待下发, 1-已接受, 2-已拒绝';
COMMENT ON COLUMN smart_charge_profile_ocpp16.create_time IS '创建时间';
COMMENT ON COLUMN smart_charge_profile_ocpp16.update_time IS '更新时间';
COMMIT;
