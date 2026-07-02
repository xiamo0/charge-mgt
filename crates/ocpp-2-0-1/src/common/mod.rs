//! OCPP 2.0.1 common types

pub mod id_token;
pub mod evse;
pub mod status;
pub mod authorization;
pub mod transaction;
pub mod meter_value;
pub mod boot;
pub mod charging_profile;
pub mod component;
pub mod datetime;
pub mod network;
pub mod response_status;

pub mod uuid {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    pub fn generate_uuid() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("{:016x}-{:08x}", timestamp, counter)
    }
}

pub use id_token::*;
pub use evse::*;
pub use status::*;
pub use authorization::*;
pub use transaction::*;
pub use meter_value::*;
pub use boot::*;
pub use charging_profile::*;
pub use component::*;
pub use network::*;
pub use response_status::*;
