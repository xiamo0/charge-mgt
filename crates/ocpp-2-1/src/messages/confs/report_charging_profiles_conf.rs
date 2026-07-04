//! ReportChargingProfiles Confirmation (Block K)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportChargingProfilesConfirmation {}

impl ReportChargingProfilesConfirmation {
    pub fn new() -> Self { Self {} }
}
