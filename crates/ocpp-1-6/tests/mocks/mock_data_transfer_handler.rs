//! Mock DataTransfer Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use serde_json::Value as JsonValue;
use ocpp_1_6::{
    messages::calls::data_transfer::{DataTransferHandler, DataTransferRequest, DataTransferError},
    messages::confs::data_transfer_conf::DataTransferConfirmation,
    common::status::DataTransferStatus,
};

#[derive(Debug, Clone)]
pub struct MockDataTransferHandler {
    pub vendor_id: String,
    pub message_id: Option<String>,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<DataTransferRequest>>>,
}

impl MockDataTransferHandler {
    pub fn new(vendor_id: &str, message_id: Option<&str>) -> Self {
        Self {
            vendor_id: vendor_id.to_string(),
            message_id: message_id.map(|s| s.to_string()),
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl DataTransferHandler for MockDataTransferHandler {
    fn vendor_id(&self) -> &'static str {
        Box::leak(self.vendor_id.clone().into_boxed_str())
    }

    fn handle(&self, message_id: Option<&str>, data: &Option<JsonValue>) -> Result<JsonValue, DataTransferError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        if self.message_id.as_deref() != message_id {
            return Err(DataTransferError::Rejected);
        }
        Ok(data.clone().unwrap_or(JsonValue::Null))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_data_transfer() {
        let handler = MockDataTransferHandler::new("vendor_x", Some("msg_1"));
        let req = DataTransferRequest {
            vendor_id: "vendor_x".to_string(),
            message_id: Some("msg_1".to_string()),
            data: Some(JsonValue::String("test".to_string())),
        };
        let conf = DataTransferConfirmation::from_handler_result(handler.handle(req.message_id.as_deref(), &req.data));
        assert_eq!(conf.status, DataTransferStatus::Accepted);
    }
}
