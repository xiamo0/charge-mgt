//! Component and Variable Types (Functional Block B - Provisioning)
//! OCPP 2.0.1 使用 Component/Variable 模型替代 1.6 的 Key-Value 配置

use serde::{Deserialize, Serialize};
use crate::common::EVSEType;

/// 组件类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    /// 组件名称 (max 50 chars)
    pub name: String,
    /// 组件实例 (max 50 chars, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// EVSE (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

impl ComponentType {
    /// 创建新的组件
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instance: None,
            evse: None,
        }
    }

    /// 设置实例
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// 设置 EVSE
    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }
}

/// 变量类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    /// 变量名称 (max 50 chars)
    pub name: String,
    /// 变量实例 (max 50 chars, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl VariableType {
    /// 创建新的变量
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instance: None,
        }
    }

    /// 设置实例
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }
}

/// 属性类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AttributeEnumType {
    /// 实际值
    Actual,
    /// 目标值
    Target,
    /// 最小设置值
    MinSet,
    /// 最大设置值
    MaxSet,
}

/// 变量属性类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    /// 属性类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// 属性值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 可变性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<MutabilityEnumType>,
    /// 是否持久化
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    /// 是否常量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
}

/// 可变性枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MutabilityEnumType {
    /// 只读
    ReadOnly,
    /// 只写
    WriteOnly,
    /// 读写
    ReadWrite,
}

/// 变量特征类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    /// 数据类型
    pub data_type: DataEnumType,
    /// 单位 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 最小值 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,
    /// 最大值 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,
    /// 可选值列表 (逗号分隔, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    /// 是否支持监控
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_monitoring: Option<bool>,
    /// 最大长度 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
}

/// 数据类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DataEnumType {
    /// 字符串
    String,
    /// 十进制数
    Decimal,
    /// 整数
    Integer,
    /// 日期时间
    DateTime,
    /// 布尔值
    Boolean,
    /// 选项列表
    OptionList,
    /// 序列列表
    SequenceList,
    /// 成员列表
    MemberList,
}

/// 获取变量数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableDataType {
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 属性类型 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
}

impl GetVariableDataType {
    /// 创建新的获取变量数据
    pub fn new(component: ComponentType, variable: VariableType) -> Self {
        Self {
            component,
            variable,
            attribute_type: None,
        }
    }

    /// 设置属性类型
    pub fn with_attribute_type(mut self, attribute_type: AttributeEnumType) -> Self {
        self.attribute_type = Some(attribute_type);
        self
    }
}

/// 获取变量状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetVariableStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 未知组件
    UnknownComponent,
    /// 未知变量
    UnknownVariable,
    /// 不支持的属性类型
    NotSupportedAttributeType,
}

/// 获取变量结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType {
    /// 属性状态
    pub attribute_status: GetVariableStatusEnumType,
    /// 属性值 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 属性类型 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// 状态信息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<crate::common::StatusInfoType>,
}

/// 设置变量数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    /// 属性值 (max 1000 chars)
    pub attribute_value: String,
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 属性类型 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
}

impl SetVariableDataType {
    /// 创建新的设置变量数据
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        value: impl Into<String>,
    ) -> Self {
        Self {
            attribute_value: value.into(),
            component,
            variable,
            attribute_type: None,
        }
    }

    /// 设置属性类型
    pub fn with_attribute_type(mut self, attribute_type: AttributeEnumType) -> Self {
        self.attribute_type = Some(attribute_type);
        self
    }
}

/// 设置变量状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetVariableStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 未知组件
    UnknownComponent,
    /// 未知变量
    UnknownVariable,
    /// 不支持的属性类型
    NotSupportedAttributeType,
    /// 需要重启
    RebootRequired,
    /// 无效值
    InvalidValue,
    /// 超出范围
    OutOfRange,
}

/// 设置变量结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    /// 属性状态
    pub attribute_status: SetVariableStatusEnumType,
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 属性类型 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// 状态信息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<crate::common::StatusInfoType>,
}

/// 组件标准枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ComponentCriterionEnumType {
    /// 活动组件
    Active,
    /// 可用组件
    Available,
    /// 启用组件
    Enabled,
    /// 问题组件
    Problem,
}

/// 报告数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 变量属性列表
    pub variable_attribute: Vec<VariableAttributeType>,
    /// 变量特征 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}

/// 报告基础枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReportBaseEnumType {
    /// 配置清单
    ConfigurationInventory,
    /// 完整清单
    FullInventory,
    /// 摘要清单
    SummaryInventory,
}

/// 组件变量类型 (用于 GetReport)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    /// 组件
    pub component: ComponentType,
    /// 变量 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}

impl ComponentVariableType {
    /// 创建新的组件变量
    pub fn new(component: ComponentType) -> Self {
        Self {
            component,
            variable: None,
        }
    }

    /// 设置变量
    pub fn with_variable(mut self, variable: VariableType) -> Self {
        self.variable = Some(variable);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_type() {
        let component = ComponentType::new("HeartbeatCtrlr");
        let json = serde_json::to_string(&component).unwrap();
        let de: ComponentType = serde_json::from_str(&json).unwrap();
        assert_eq!(component, de);
    }

    #[test]
    fn test_variable_type() {
        let variable = VariableType::new("Interval");
        let json = serde_json::to_string(&variable).unwrap();
        let de: VariableType = serde_json::from_str(&json).unwrap();
        assert_eq!(variable, de);
    }

    #[test]
    fn test_attribute_enum() {
        let variants = [
            AttributeEnumType::Actual,
            AttributeEnumType::Target,
            AttributeEnumType::MinSet,
            AttributeEnumType::MaxSet,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: AttributeEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_get_variable_data() {
        let data = GetVariableDataType::new(
            ComponentType::new("HeartbeatCtrlr"),
            VariableType::new("Interval"),
        )
        .with_attribute_type(AttributeEnumType::Actual);
        let json = serde_json::to_string(&data).unwrap();
        let de: GetVariableDataType = serde_json::from_str(&json).unwrap();
        assert_eq!(data, de);
    }

    #[test]
    fn test_set_variable_data() {
        let data = SetVariableDataType::new(
            ComponentType::new("HeartbeatCtrlr"),
            VariableType::new("Interval"),
            "60",
        );
        let json = serde_json::to_string(&data).unwrap();
        let de: SetVariableDataType = serde_json::from_str(&json).unwrap();
        assert_eq!(data, de);
    }

    #[test]
    fn test_get_variable_status() {
        let variants = [
            GetVariableStatusEnumType::Accepted,
            GetVariableStatusEnumType::Rejected,
            GetVariableStatusEnumType::UnknownComponent,
            GetVariableStatusEnumType::UnknownVariable,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: GetVariableStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_set_variable_status() {
        let variants = [
            SetVariableStatusEnumType::Accepted,
            SetVariableStatusEnumType::Rejected,
            SetVariableStatusEnumType::RebootRequired,
            SetVariableStatusEnumType::InvalidValue,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: SetVariableStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }
}