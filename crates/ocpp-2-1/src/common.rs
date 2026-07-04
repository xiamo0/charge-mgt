//! OCPP 2.1 common types

pub mod authorization;
pub mod battery_swap;
pub mod boot;
pub mod certificate;
pub mod charging_profile;
pub mod component;
pub mod datetime;
pub mod der;
pub mod diagnostics;
pub mod display;
pub mod evse;
pub mod firmware;
pub mod id_token;
pub mod meter_value;
pub mod monitoring;
pub mod network;
pub mod periodic_event_stream;
pub mod response_status;
pub mod settlement;
pub mod status;
pub mod tariff;
pub mod transaction;
pub mod v2x;

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

pub use authorization::*;
pub use battery_swap::*;
pub use boot::*;
pub use certificate::*;
pub use charging_profile::*;
pub use component::*;
pub use der::*;
pub use diagnostics::*;
pub use display::*;
pub use evse::*;
pub use firmware::*;
pub use id_token::*;
pub use meter_value::*;
pub use monitoring::*;
pub use network::*;
pub use periodic_event_stream::*;
pub use response_status::*;
pub use settlement::*;
pub use status::*;
pub use tariff::*;
pub use transaction::*;
pub use v2x::*;
