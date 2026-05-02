use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ChargePointInfo {
    pub id: String,
    pub vendor: String,
    pub protocol_version: String,
}

pub struct AppState {
    pub charge_points: Arc<RwLock<HashMap<String, ChargePointInfo>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            charge_points: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_charge_point(
        &self,
        id: String,
        vendor: String,
        protocol_version: String,
    ) {
        let info = ChargePointInfo {
            id: id.clone(),
            vendor,
            protocol_version,
        };
        let mut points = self.charge_points.write().await;
        points.insert(id, info);
    }

    pub async fn get_vendor(&self, charge_point_id: &str) -> Option<String> {
        let points = self.charge_points.read().await;
        points.get(charge_point_id).map(|p| p.vendor.clone())
    }

    pub async fn remove_charge_point(&self, charge_point_id: &str) {
        let mut points = self.charge_points.write().await;
        points.remove(charge_point_id);
    }

    pub async fn list_charge_points(&self) -> Vec<ChargePointInfo> {
        let points = self.charge_points.read().await;
        points.values().cloned().collect()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_and_get_vendor() {
        let state = AppState::new();
        state
            .register_charge_point(
                "cp-001".to_string(),
                "alphas".to_string(),
                "OCPP-1.6".to_string(),
            )
            .await;

        assert_eq!(state.get_vendor("cp-001").await, Some("alphas".to_string()));
        assert_eq!(state.get_vendor("cp-002").await, None);
    }

    #[tokio::test]
    async fn test_remove_charge_point() {
        let state = AppState::new();
        state
            .register_charge_point(
                "cp-001".to_string(),
                "alphas".to_string(),
                "OCPP-1.6".to_string(),
            )
            .await;
        state.remove_charge_point("cp-001").await;

        assert_eq!(state.get_vendor("cp-001").await, None);
    }

    #[tokio::test]
    async fn test_list_charge_points() {
        let state = AppState::new();
        state
            .register_charge_point(
                "cp-001".to_string(),
                "alphas".to_string(),
                "OCPP-1.6".to_string(),
            )
            .await;
        state
            .register_charge_point(
                "cp-002".to_string(),
                "wz".to_string(),
                "OCPP-1.6".to_string(),
            )
            .await;

        let points = state.list_charge_points().await;
        assert_eq!(points.len(), 2);
    }
}
