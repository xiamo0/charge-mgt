//! Component and Variable Types (Functional Block B — Provisioning)

use crate::common::{EVSEType, StatusInfoType};
use serde::{Deserialize, Serialize};

/// 组件类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

impl ComponentType {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instance: None,
            evse: None,
        }
    }
}

/// 变量类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl VariableType {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instance: None,
        }
    }
}

/// 属性类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AttributeEnumType {
    Actual,
    Target,
    MinSet,
    MaxSet,
}

/// 可变性枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MutabilityEnumType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// 变量属性类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<MutabilityEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
}

/// 数据类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DataEnumType {
    String,
    Decimal,
    Integer,
    DateTime,
    Boolean,
    OptionList,
    SequenceList,
    MemberList,
}

/// 变量特征类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    pub data_type: DataEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_monitoring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
}

/// 获取变量数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
}

/// 获取变量状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
}

/// 获取变量结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType {
    pub attribute_status: GetVariableStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

/// 设置变量数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    pub attribute_value: String,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
}

/// 设置变量状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
    RebootRequired,
    InvalidValue,
    OutOfRange,
}

/// 设置变量结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    pub attribute_status: SetVariableStatusEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
}

/// 组件标准枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ComponentCriterionEnumType {
    Active,
    Available,
    Enabled,
    Problem,
}

/// 报告数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_attribute: Vec<VariableAttributeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}

/// 报告基础枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReportBaseEnumType {
    ConfigurationInventory,
    FullInventory,
    SummaryInventory,
}

/// 组件变量类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableType>,
}
