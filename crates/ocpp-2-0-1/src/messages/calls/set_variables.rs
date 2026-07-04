//! SetVariables Request (Functional Block B)
//! 写入配置变量（替代 OCPP 1.6 的 ChangeConfiguration）

use crate::common::SetVariableDataType;
use serde::{Deserialize, Serialize};

/// SetVariables 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    /// 要设置的变量列表
    pub set_variable_data: Vec<SetVariableDataType>,
}

impl SetVariablesRequest {
    pub fn new(set_variable_data: Vec<SetVariableDataType>) -> Self {
        Self { set_variable_data }
    }
}

pub const ACTION: &str = "SetVariables";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ComponentType, VariableType};

    #[test]
    fn test_set_variables_request_serialization() {
        let req = SetVariablesRequest::new(vec![SetVariableDataType::new(
            ComponentType::new("HeartbeatCtrlr"),
            VariableType::new("Interval"),
            "60",
        )]);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("setVariableData"));
        assert!(json.contains("HeartbeatCtrlr"));
        assert!(json.contains("Interval"));
        assert!(json.contains("60"));

        let de: SetVariablesRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, de);
    }
}
