//! Meter Value Types (Functional Block J)

use serde::{Deserialize, Serialize};

/// 测量量枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MeasurandEnumType {
    /// 有功电能-输入（最常用）
    EnergyActiveImportRegister,
    /// 有功电能-输出
    EnergyActiveExportRegister,
    /// 无功电能-输入
    EnergyReactiveImportRegister,
    /// 无功电能-输出
    EnergyReactiveExportRegister,
    /// 有功电能-输入(间隔)
    EnergyActiveImportInterval,
    /// 有功电能-输出(间隔)
    EnergyActiveExportInterval,
    /// 净有功电能
    EnergyActiveNet,
    /// 无功电能-输入(间隔)
    EnergyReactiveImportInterval,
    /// 无功电能-输出(间隔)
    EnergyReactiveExportInterval,
    /// 净无功电能
    EnergyReactiveNet,
    /// 净视在电能
    EnergyApparentNet,
    /// 视在电能-输入
    EnergyApparentImport,
    /// 视在电能-输出
    EnergyApparentExport,
    /// 频率
    Frequency,
    /// 有功功率-输入
    PowerActiveImport,
    /// 有功功率-输出
    PowerActiveExport,
    /// 功率因数
    PowerFactor,
    /// 功率-供给
    PowerOffered,
    /// 无功功率-输入
    PowerReactiveImport,
    /// 无功功率-输出
    PowerReactiveExport,
    /// SoC (荷电状态)
    SoC,
    /// 电流-输入
    CurrentImport,
    /// 电流-输出
    CurrentExport,
    /// 电流-供给
    CurrentOffered,
    /// 电压
    Voltage,
}

/// 读数上下文枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReadingContextEnumType {
    /// 中断开始
    InterruptionBegin,
    /// 中断结束
    InterruptionEnd,
    /// 其他
    Other,
    /// 时钟采样
    SampleClock,
    /// 周期采样（最常用）
    SamplePeriodic,
    /// 事务开始
    TransactionBegin,
    /// 事务结束
    TransactionEnd,
    /// 触发
    Trigger,
}

/// 相位枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PhaseEnumType {
    /// L1相
    L1,
    /// L2相
    L2,
    /// L3相
    L3,
    /// 中性线
    N,
    /// L1-中性线
    #[serde(rename = "L1-N")]
    L1N,
    /// L2-中性线
    #[serde(rename = "L2-N")]
    L2N,
    /// L3-中性线
    #[serde(rename = "L3-N")]
    L3N,
    /// L1-L2
    #[serde(rename = "L1-L2")]
    L1L2,
    /// L2-L3
    #[serde(rename = "L2-L3")]
    L2L3,
    /// L3-L1
    #[serde(rename = "L3-L1")]
    L3L1,
}

/// 位置枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LocationEnumType {
    /// 充电桩体
    Body,
    /// 线缆
    Cable,
    /// 电动汽车
    EV,
    /// 入口
    Inlet,
    /// 出口（默认）
    Outlet,
}

/// 计量单位类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    /// 单位 (max 20 chars, 默认 Wh)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 倍数 (默认 0, 表示 10^0 = 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
}

impl Default for UnitOfMeasureType {
    fn default() -> Self {
        Self {
            unit: Some("Wh".to_string()),
            multiplier: Some(0),
        }
    }
}

/// 签名计量值类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    /// 签名数据 (Base64, max 2500)
    pub signed_meter_data: String,
    /// 签名方法 (max 50)
    pub signing_method: String,
    /// 编码方法 (max 50)
    pub encoding_method: String,
    /// 公钥 (Base64, max 2500)
    pub public_key: String,
}

/// 采样值类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    /// 采样值
    pub value: f64,
    /// 读数上下文 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,
    /// 测量量 (可选, 默认 Energy.Active.Import.Register)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,
    /// 相位 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,
    /// 位置 (可选, 默认 Outlet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,
    /// 签名计量值 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_meter_value: Option<SignedMeterValueType>,
    /// 计量单位 (可选, 默认 Wh)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasureType>,
}

impl SampledValueType {
    /// 创建新的采样值
    pub fn new(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: None,
        }
    }

    /// 创建电能采样值 (默认 Energy.Active.Import.Register)
    pub fn energy(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: Some(MeasurandEnumType::EnergyActiveImportRegister),
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: Some(UnitOfMeasureType::default()),
        }
    }

    /// 创建功率采样值
    pub fn power_active_import(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: Some(MeasurandEnumType::PowerActiveImport),
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: Some(UnitOfMeasureType {
                unit: Some("W".to_string()),
                multiplier: Some(0),
            }),
        }
    }

    /// 创建电流采样值
    pub fn current_import(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: Some(MeasurandEnumType::CurrentImport),
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: Some(UnitOfMeasureType {
                unit: Some("A".to_string()),
                multiplier: Some(0),
            }),
        }
    }

    /// 创建 SoC 采样值
    pub fn soc(value: f64) -> Self {
        Self {
            value,
            context: None,
            measurand: Some(MeasurandEnumType::SoC),
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: Some(UnitOfMeasureType {
                unit: Some("%".to_string()),
                multiplier: Some(0),
            }),
        }
    }

    /// 设置采样上下文
    pub fn with_context(mut self, context: ReadingContextEnumType) -> Self {
        self.context = Some(context);
        self
    }

    /// 设置相位
    pub fn with_phase(mut self, phase: PhaseEnumType) -> Self {
        self.phase = Some(phase);
        self
    }

    /// 设置位置
    pub fn with_location(mut self, location: LocationEnumType) -> Self {
        self.location = Some(location);
        self
    }
}

/// 计量值类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    /// 时间戳
    pub timestamp: String,
    /// 采样值列表
    pub sampled_value: Vec<SampledValueType>,
}

impl MeterValueType {
    /// 创建新的计量值
    pub fn new(timestamp: impl Into<String>, sampled_values: Vec<SampledValueType>) -> Self {
        Self {
            timestamp: timestamp.into(),
            sampled_value: sampled_values,
        }
    }

    /// 创建单值计量值
    pub fn single(timestamp: impl Into<String>, value: SampledValueType) -> Self {
        Self {
            timestamp: timestamp.into(),
            sampled_value: vec![value],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurand_enum() {
        let variants = [
            MeasurandEnumType::EnergyActiveImportRegister,
            MeasurandEnumType::EnergyActiveExportRegister,
            MeasurandEnumType::PowerActiveImport,
            MeasurandEnumType::PowerReactiveImport,
            MeasurandEnumType::CurrentImport,
            MeasurandEnumType::Voltage,
            MeasurandEnumType::SoC,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: MeasurandEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_reading_context_enum() {
        let variants = [
            ReadingContextEnumType::SamplePeriodic,
            ReadingContextEnumType::SampleClock,
            ReadingContextEnumType::TransactionBegin,
            ReadingContextEnumType::TransactionEnd,
            ReadingContextEnumType::Trigger,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ReadingContextEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_phase_enum() {
        let variants = [
            PhaseEnumType::L1,
            PhaseEnumType::L2,
            PhaseEnumType::L3,
            PhaseEnumType::L1N,
            PhaseEnumType::L2L3,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: PhaseEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_sampled_value_type() {
        let sv = SampledValueType::new(15500.0);
        let json = serde_json::to_string(&sv).unwrap();
        let de: SampledValueType = serde_json::from_str(&json).unwrap();
        assert_eq!(sv, de);
        assert_eq!(sv.value, 15500.0);
    }

    #[test]
    fn test_sampled_value_energy() {
        let sv = SampledValueType::energy(15500.0);
        assert_eq!(sv.measurand, Some(MeasurandEnumType::EnergyActiveImportRegister));
        assert!(sv.unit_of_measure.is_some());
    }

    #[test]
    fn test_meter_value_type() {
        let mv = MeterValueType::new(
            "2024-01-15T10:30:00Z",
            vec![SampledValueType::energy(15500.0)],
        );
        let json = serde_json::to_string(&mv).unwrap();
        let de: MeterValueType = serde_json::from_str(&json).unwrap();
        assert_eq!(mv, de);
    }
}