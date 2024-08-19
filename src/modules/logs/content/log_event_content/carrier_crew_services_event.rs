use crate::station::CarrierCrewRole;
use serde::{Deserialize, Serialize};

/// Fired when changes were made to a fleet carrier service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierCrewServicesEvent {
    /// The ID of the carrier of which the crew was changed. This is functionally the same as the
    /// market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The service that this event applies to.
    pub crew_role: CarrierCrewRole,

    /// The current operational status of the target service.
    pub operation: CarrierCrewServicesEventOperation,

    /// The name of the crew member for the `crew_role`.
    pub crew_name: String,
}

// /// The given service the event is for.
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
// pub enum CarrierCrewServicesEventCrewRole {
//     Captain,
//     CarrierFuel,
//     Refuel,
//     Repair,
//     Rearm,
//     Shipyard,
//
//     #[serde(rename = "Commodities")]
//     Market,
//
//     BlackMarket,
//
//     #[serde(rename = "Exploration")]
//     UniversalCartographers,
//     VistaGenomics,
//     PioneerSupplies,
//     Bartender,
//     Outfitting,
//
//     #[serde(rename = "VoucherRedemption")]
//     RedemptionOffice,
// }

/// The operation for this event. Note that some of these variants are for changes and some are
/// for the current status. This is because sometimes the [CarrierCrewServicesEvent] is fired
/// without a change in operation status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierCrewServicesEventOperation {
    /// The service is currently active.
    Active,

    /// The service is getting activated.
    Activate,

    /// The service is getting deactivated.
    Deactivate,

    /// The service is currently suspended.
    #[serde(rename = "Pause")]
    Suspended,

    // TODO check this.
    /// The service is getting replaced.
    Replace,
}
