//! GetVariables Request (Functional Block B)
//! 读取配置变量（替代 OCPP 1.6 的 GetConfiguration）

use serde::{Deserialize, Serialize};
use crate::common::GetVariableDataType;

/// GetVariables 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    /// 要获取的变量列表
    pub get_variable_data: Vec<GetVariableDataType>,
}

impl GetVariablesRequest {
    pub fn new(get_variable_data: Vec<GetVariableDataType>) -> Self {
        Self {
            get_variable_data,
        }
    }
}

pub const ACTION: &str = "GetVariables";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ComponentType, VariableType};

    #[test]
    fn test_get_variables_request_serialization() {
        let req = GetVariablesRequest::new(vec![
            GetVariableDataType::new(
                ComponentType::new("HeartbeatCtrlr"),
                VariableType::new("Interval"),
            ),
        ]);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("getVariableData"));
        assert!(json.contains("HeartbeatCtrlr"));
        assert!(json.contains("Interval"));
        
        let de: GetVariablesRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, de);
    }
}