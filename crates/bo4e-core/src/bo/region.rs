//! Region (Region) business object.

use serde::{Deserialize, Serialize};

use crate::com::RegionCriterion;
use crate::enums::RegionType;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A geographical region in the energy market.
///
/// German: Region
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Region;
/// use bo4e_core::enums::RegionType;
///
/// let region = Region {
///     region_code: Some("DE-BY".to_string()),
///     name: Some("Bavaria".to_string()),
///     region_type: Some(RegionType::SupplyArea),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Region code (Regionscode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,

    /// Region name (Name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of region (Gebietstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<RegionType>,

    /// Criteria that define this region (Regionskriterien)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<RegionCriterion>,

    /// Parent region (Uebergeordnete Region)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_region: Option<Box<Region>>,

    /// Sub-regions (Unterregionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_regions: Vec<Box<Region>>,
}

impl Bo4eObject for Region {
    fn type_name_german() -> &'static str {
        "Region"
    }

    fn type_name_english() -> &'static str {
        "Region"
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
    fn test_region_creation() {
        let region = Region {
            region_code: Some("DE-BY".to_string()),
            name: Some("Bavaria".to_string()),
            region_type: Some(RegionType::SupplyArea),
            ..Default::default()
        };

        assert_eq!(region.region_code, Some("DE-BY".to_string()));
        assert_eq!(region.region_type, Some(RegionType::SupplyArea));
    }

    #[test]
    fn test_market_area_region() {
        let region = Region {
            region_code: Some("TRADING-HUB-EUROPE".to_string()),
            name: Some("Trading Hub Europe".to_string()),
            region_type: Some(RegionType::MarketArea),
            ..Default::default()
        };

        assert_eq!(region.region_type, Some(RegionType::MarketArea));
    }

    #[test]
    fn test_nested_regions() {
        let sub_region = Box::new(Region {
            region_code: Some("DE-BY-M".to_string()),
            name: Some("Munich Area".to_string()),
            ..Default::default()
        });

        let region = Region {
            region_code: Some("DE-BY".to_string()),
            name: Some("Bavaria".to_string()),
            sub_regions: vec![sub_region],
            ..Default::default()
        };

        assert_eq!(region.sub_regions.len(), 1);
        assert_eq!(region.sub_regions[0].name, Some("Munich Area".to_string()));
    }

    #[test]
    fn test_serialize() {
        let region = Region {
            meta: Bo4eMeta::with_type("Region"),
            region_code: Some("DE-BY".to_string()),
            name: Some("Bavaria".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&region).unwrap();
        assert!(json.contains(r#""regionCode":"DE-BY""#));
        assert!(json.contains(r#""name":"Bavaria""#));
    }

    #[test]
    fn test_roundtrip() {
        let region = Region {
            meta: Bo4eMeta::with_type("Region"),
            region_code: Some("DE-BY".to_string()),
            name: Some("Bavaria".to_string()),
            description: Some("State of Bavaria".to_string()),
            region_type: Some(RegionType::SupplyArea),
            ..Default::default()
        };

        let json = serde_json::to_string(&region).unwrap();
        let parsed: Region = serde_json::from_str(&json).unwrap();
        assert_eq!(region, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Region::type_name_german(), "Region");
        assert_eq!(Region::type_name_english(), "Region");
    }
}
