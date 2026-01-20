//! Location assignment (Lokationszuordnung) business object.
//!
//! Represents the assignment/relationship between different location types.

use serde::{Deserialize, Serialize};

use crate::com::TimePeriod;
use crate::enums::{ArithmeticOperation, LocationType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An assignment between locations in the energy market.
///
/// German: Lokationszuordnung
///
/// This business object represents the relationship between different
/// types of locations (market, metering, network).
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::LocationAssignment;
///
/// let assignment = LocationAssignment {
///     market_location_id: Some("12345678901".to_string()),
///     metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAssignment {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Market location ID (Marktlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location_id: Option<String>,

    /// Metering location ID (Messlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_location_id: Option<String>,

    /// Network location ID (Netzlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_location_id: Option<String>,

    /// Technical resource ID (Technische-Ressource-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_resource_id: Option<String>,

    /// Controllable resource ID (Steuerbare-Ressource-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controllable_resource_id: Option<String>,

    /// Location type (Lokationstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<LocationType>,

    /// Arithmetic operation for combination (Rechenoperation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arithmetic_operation: Option<ArithmeticOperation>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Sequence/order number (Reihenfolge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

impl Bo4eObject for LocationAssignment {
    fn type_name_german() -> &'static str {
        "Lokationszuordnung"
    }

    fn type_name_english() -> &'static str {
        "LocationAssignment"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_creation() {
        let assignment = LocationAssignment {
            market_location_id: Some("12345678901".to_string()),
            metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
            ..Default::default()
        };

        assert!(assignment.market_location_id.is_some());
        assert!(assignment.metering_location_id.is_some());
    }

    #[test]
    fn test_serialize() {
        let assignment = LocationAssignment {
            meta: Bo4eMeta::with_type("Lokationszuordnung"),
            market_location_id: Some("12345678901".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&assignment).unwrap();
        assert!(json.contains(r#""_typ":"Lokationszuordnung""#));
    }

    #[test]
    fn test_roundtrip() {
        let assignment = LocationAssignment {
            meta: Bo4eMeta::with_type("Lokationszuordnung"),
            market_location_id: Some("12345678901".to_string()),
            metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
            arithmetic_operation: Some(ArithmeticOperation::Addition),
            sequence: Some(1),
            ..Default::default()
        };

        let json = serde_json::to_string(&assignment).unwrap();
        let parsed: LocationAssignment = serde_json::from_str(&json).unwrap();
        assert_eq!(assignment, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(LocationAssignment::type_name_german(), "Lokationszuordnung");
        assert_eq!(
            LocationAssignment::type_name_english(),
            "LocationAssignment"
        );
    }
}
