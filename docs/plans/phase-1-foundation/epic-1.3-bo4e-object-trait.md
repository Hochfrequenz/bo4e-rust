# Epic 1.3: Bo4eObject Trait System

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the core `Bo4eObject` trait, `Bo4eMeta` struct, and `AdditionalAttribute` types that all BO4E types will use.

**Architecture:** A trait that provides common metadata fields (`_typ`, `_version`, `_id`, `zusatz_attribute`) with a companion struct for storage.

**Tech Stack:** Rust traits, serde derive macros

---

## Task 1: Define AdditionalAttribute Types

**Files:**
- Create: `crates/bo4e-core/src/additional_attribute.rs`
- Modify: `crates/bo4e-core/src/lib.rs`

**Step 1: Write the test**

Create `crates/bo4e-core/src/additional_attribute.rs`:

```rust
//! Additional attributes for extensibility.

use serde::{Deserialize, Serialize};

/// Value type for additional attributes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    /// String value
    String(String),
    /// Numeric value (integer or float)
    Number(f64),
    /// Boolean value
    Boolean(bool),
    /// Nested object with more attributes
    Object(Vec<AdditionalAttribute>),
    /// Array of values
    Array(Vec<AttributeValue>),
    /// Null value
    Null,
}

/// Additional attribute for external system IDs and custom metadata.
///
/// This enables interoperability with external systems that need to attach
/// their own identifiers or metadata to BO4E objects.
///
/// # Example
///
/// ```rust
/// use bo4e_core::AdditionalAttribute;
/// use bo4e_core::additional_attribute::AttributeValue;
///
/// let attr = AdditionalAttribute {
///     name: "sap_id".to_string(),
///     value: Some(AttributeValue::String("SAP123".to_string())),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalAttribute {
    /// Name/key of the attribute
    pub name: String,
    /// Value of the attribute (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

impl AdditionalAttribute {
    /// Create a new additional attribute with a string value.
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(AttributeValue::String(value.into())),
        }
    }

    /// Create a new additional attribute with a numeric value.
    pub fn number(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value: Some(AttributeValue::Number(value)),
        }
    }

    /// Create a new additional attribute with a boolean value.
    pub fn boolean(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            value: Some(AttributeValue::Boolean(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_attribute() {
        let attr = AdditionalAttribute::string("system_id", "ABC123");
        assert_eq!(attr.name, "system_id");
        assert_eq!(attr.value, Some(AttributeValue::String("ABC123".to_string())));
    }

    #[test]
    fn test_number_attribute() {
        let attr = AdditionalAttribute::number("priority", 42.0);
        assert_eq!(attr.name, "priority");
        assert_eq!(attr.value, Some(AttributeValue::Number(42.0)));
    }

    #[test]
    fn test_serialize_attribute() {
        let attr = AdditionalAttribute::string("key", "value");
        let json = serde_json::to_string(&attr).unwrap();
        assert_eq!(json, r#"{"name":"key","value":"value"}"#);
    }

    #[test]
    fn test_deserialize_attribute() {
        let json = r#"{"name":"key","value":"value"}"#;
        let attr: AdditionalAttribute = serde_json::from_str(json).unwrap();
        assert_eq!(attr.name, "key");
        assert_eq!(attr.value, Some(AttributeValue::String("value".to_string())));
    }

    #[test]
    fn test_deserialize_nested_attribute() {
        let json = r#"{"name":"parent","value":[{"name":"child","value":123}]}"#;
        let attr: AdditionalAttribute = serde_json::from_str(json).unwrap();
        assert_eq!(attr.name, "parent");
        match attr.value {
            Some(AttributeValue::Object(children)) => {
                assert_eq!(children.len(), 1);
                assert_eq!(children[0].name, "child");
            }
            _ => panic!("Expected nested object"),
        }
    }
}
```

**Step 2: Run tests to verify they pass**

```bash
cargo test -p bo4e-core additional_attribute -- --nocapture
```

Expected: All tests pass

**Step 3: Update lib.rs to export**

Add to `crates/bo4e-core/src/lib.rs`:

```rust
pub mod additional_attribute;

pub use additional_attribute::AdditionalAttribute;
```

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/additional_attribute.rs crates/bo4e-core/src/lib.rs
git commit -m "feat(core): add AdditionalAttribute for extensibility"
```

---

## Task 2: Define Bo4eMeta Struct

**Files:**
- Modify: `crates/bo4e-core/src/traits.rs`

**Step 1: Write the test and implementation**

Replace `crates/bo4e-core/src/traits.rs`:

```rust
//! Core traits and types for BO4E objects.

use crate::AdditionalAttribute;
use serde::{Deserialize, Serialize};

/// Metadata common to all BO4E objects.
///
/// This struct holds the standard BO4E metadata fields:
/// - `_typ`: Type discriminator
/// - `_version`: BO4E schema version
/// - `_id`: External system ID
/// - `zusatzAttribute`: Additional attributes for extensibility
///
/// # Example
///
/// ```rust
/// use bo4e_core::Bo4eMeta;
///
/// let meta = Bo4eMeta {
///     typ: Some("Zaehler".to_string()),
///     version: Some("202401.0.1".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bo4eMeta {
    /// Type discriminator (maps to `_typ` in JSON)
    #[serde(rename = "_typ", skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,

    /// BO4E schema version (maps to `_version` in JSON)
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// External system ID (maps to `_id` in JSON)
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Additional attributes for extensibility
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zusatz_attribute: Vec<AdditionalAttribute>,
}

impl Bo4eMeta {
    /// Create metadata with type name.
    pub fn with_type(typ: impl Into<String>) -> Self {
        Self {
            typ: Some(typ.into()),
            ..Default::default()
        }
    }

    /// Set the version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the external ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add an additional attribute.
    pub fn with_attribute(mut self, attr: AdditionalAttribute) -> Self {
        self.zusatz_attribute.push(attr);
        self
    }
}

/// Trait implemented by all BO4E types.
///
/// This trait provides a common interface for accessing type metadata
/// and enables generic programming over BO4E types.
pub trait Bo4eObject {
    /// Returns the German type name as used in the `_typ` field.
    ///
    /// Example: `"Zaehler"` for Meter, `"Marktlokation"` for MarketLocation
    fn type_name_german() -> &'static str;

    /// Returns the English type name.
    ///
    /// Example: `"Meter"`, `"MarketLocation"`
    fn type_name_english() -> &'static str;

    /// Returns a reference to the metadata.
    fn meta(&self) -> &Bo4eMeta;

    /// Returns a mutable reference to the metadata.
    fn meta_mut(&mut self) -> &mut Bo4eMeta;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_default() {
        let meta = Bo4eMeta::default();
        assert!(meta.typ.is_none());
        assert!(meta.version.is_none());
        assert!(meta.id.is_none());
        assert!(meta.zusatz_attribute.is_empty());
    }

    #[test]
    fn test_meta_builder() {
        let meta = Bo4eMeta::with_type("Zaehler")
            .version("202401.0.1")
            .id("ext-123");

        assert_eq!(meta.typ, Some("Zaehler".to_string()));
        assert_eq!(meta.version, Some("202401.0.1".to_string()));
        assert_eq!(meta.id, Some("ext-123".to_string()));
    }

    #[test]
    fn test_meta_serialize() {
        let meta = Bo4eMeta::with_type("Zaehler").version("202401.0.1");
        let json = serde_json::to_string(&meta).unwrap();

        assert!(json.contains(r#""_typ":"Zaehler""#));
        assert!(json.contains(r#""_version":"202401.0.1""#));
        assert!(!json.contains("zusatzAttribute")); // Empty vec skipped
    }

    #[test]
    fn test_meta_deserialize() {
        let json = r#"{"_typ":"Zaehler","_version":"202401.0.1","_id":"123"}"#;
        let meta: Bo4eMeta = serde_json::from_str(json).unwrap();

        assert_eq!(meta.typ, Some("Zaehler".to_string()));
        assert_eq!(meta.version, Some("202401.0.1".to_string()));
        assert_eq!(meta.id, Some("123".to_string()));
    }

    #[test]
    fn test_meta_with_zusatz_attribute() {
        let meta = Bo4eMeta::with_type("Zaehler")
            .with_attribute(crate::AdditionalAttribute::string("sap_id", "SAP001"));

        assert_eq!(meta.zusatz_attribute.len(), 1);
        assert_eq!(meta.zusatz_attribute[0].name, "sap_id");
    }
}
```

**Step 2: Run tests to verify they pass**

```bash
cargo test -p bo4e-core traits -- --nocapture
```

Expected: All tests pass

**Step 3: Commit**

```bash
git add crates/bo4e-core/src/traits.rs
git commit -m "feat(core): implement Bo4eMeta and Bo4eObject trait"
```

---

## Task 3: Create Type Discriminator Enums

**Files:**
- Create: `crates/bo4e-core/src/enums/bo_type.rs`
- Create: `crates/bo4e-core/src/enums/com_type.rs`
- Modify: `crates/bo4e-core/src/enums/mod.rs`

**Step 1: Create BoType enum**

Create `crates/bo4e-core/src/enums/bo_type.rs`:

```rust
//! Business Object type discriminator.

use serde::{Deserialize, Serialize};

/// Type discriminator for Business Objects.
///
/// Used in the `_typ` field to identify the concrete type of a BO4E object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BoType {
    // Contracts
    #[serde(rename = "Angebot")]
    Offer,
    #[serde(rename = "Ausschreibung")]
    Tender,
    #[serde(rename = "Buendelvertrag")]
    BundleContract,
    #[serde(rename = "Vertrag")]
    Contract,

    // Locations
    #[serde(rename = "Lokationszuordnung")]
    LocationAssignment,
    #[serde(rename = "Marktlokation")]
    MarketLocation,
    #[serde(rename = "Messlokation")]
    MeteringLocation,
    #[serde(rename = "Netzlokation")]
    NetworkLocation,

    // Parties
    #[serde(rename = "Geschaeftspartner")]
    BusinessPartner,
    #[serde(rename = "Marktteilnehmer")]
    MarketParticipant,
    #[serde(rename = "Person")]
    Person,

    // Financial
    #[serde(rename = "Fremdkosten")]
    ExternalCosts,
    #[serde(rename = "Kosten")]
    Costs,
    #[serde(rename = "Rechnung")]
    Invoice,
    #[serde(rename = "Tarifkosten")]
    TariffCosts,

    // Pricing
    #[serde(rename = "Preisblatt")]
    PriceSheet,
    #[serde(rename = "PreisblattDienstleistung")]
    ServicePriceSheet,
    #[serde(rename = "PreisblattHardware")]
    HardwarePriceSheet,
    #[serde(rename = "PreisblattKonzessionsabgabe")]
    ConcessionFeePriceSheet,
    #[serde(rename = "PreisblattMessung")]
    MeteringPriceSheet,
    #[serde(rename = "PreisblattNetznutzung")]
    NetworkUsagePriceSheet,
    #[serde(rename = "Tarif")]
    Tariff,
    #[serde(rename = "Tarifinfo")]
    TariffInfo,
    #[serde(rename = "Tarifpreisblatt")]
    TariffPriceSheet,

    // Technical
    #[serde(rename = "Energiemenge")]
    EnergyAmount,
    #[serde(rename = "Geraet")]
    Device,
    #[serde(rename = "Lastgang")]
    LoadProfile,
    #[serde(rename = "SteuerbareRessource")]
    ControllableResource,
    #[serde(rename = "TechnischeRessource")]
    TechnicalResource,
    #[serde(rename = "Zaehler")]
    Meter,
    #[serde(rename = "Zeitreihe")]
    TimeSeries,

    // Other
    #[serde(rename = "Bilanzierung")]
    Balancing,
    #[serde(rename = "Region")]
    Region,
    #[serde(rename = "Regionaltarif")]
    RegionalTariff,
    #[serde(rename = "Standorteigenschaften")]
    LocationProperties,
}

impl BoType {
    /// Returns the German type name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Offer => "Angebot",
            Self::Tender => "Ausschreibung",
            Self::BundleContract => "Buendelvertrag",
            Self::Contract => "Vertrag",
            Self::LocationAssignment => "Lokationszuordnung",
            Self::MarketLocation => "Marktlokation",
            Self::MeteringLocation => "Messlokation",
            Self::NetworkLocation => "Netzlokation",
            Self::BusinessPartner => "Geschaeftspartner",
            Self::MarketParticipant => "Marktteilnehmer",
            Self::Person => "Person",
            Self::ExternalCosts => "Fremdkosten",
            Self::Costs => "Kosten",
            Self::Invoice => "Rechnung",
            Self::TariffCosts => "Tarifkosten",
            Self::PriceSheet => "Preisblatt",
            Self::ServicePriceSheet => "PreisblattDienstleistung",
            Self::HardwarePriceSheet => "PreisblattHardware",
            Self::ConcessionFeePriceSheet => "PreisblattKonzessionsabgabe",
            Self::MeteringPriceSheet => "PreisblattMessung",
            Self::NetworkUsagePriceSheet => "PreisblattNetznutzung",
            Self::Tariff => "Tarif",
            Self::TariffInfo => "Tarifinfo",
            Self::TariffPriceSheet => "Tarifpreisblatt",
            Self::EnergyAmount => "Energiemenge",
            Self::Device => "Geraet",
            Self::LoadProfile => "Lastgang",
            Self::ControllableResource => "SteuerbareRessource",
            Self::TechnicalResource => "TechnischeRessource",
            Self::Meter => "Zaehler",
            Self::TimeSeries => "Zeitreihe",
            Self::Balancing => "Bilanzierung",
            Self::Region => "Region",
            Self::RegionalTariff => "Regionaltarif",
            Self::LocationProperties => "Standorteigenschaften",
        }
    }

    /// Returns the English type name.
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Offer => "Offer",
            Self::Tender => "Tender",
            Self::BundleContract => "BundleContract",
            Self::Contract => "Contract",
            Self::LocationAssignment => "LocationAssignment",
            Self::MarketLocation => "MarketLocation",
            Self::MeteringLocation => "MeteringLocation",
            Self::NetworkLocation => "NetworkLocation",
            Self::BusinessPartner => "BusinessPartner",
            Self::MarketParticipant => "MarketParticipant",
            Self::Person => "Person",
            Self::ExternalCosts => "ExternalCosts",
            Self::Costs => "Costs",
            Self::Invoice => "Invoice",
            Self::TariffCosts => "TariffCosts",
            Self::PriceSheet => "PriceSheet",
            Self::ServicePriceSheet => "ServicePriceSheet",
            Self::HardwarePriceSheet => "HardwarePriceSheet",
            Self::ConcessionFeePriceSheet => "ConcessionFeePriceSheet",
            Self::MeteringPriceSheet => "MeteringPriceSheet",
            Self::NetworkUsagePriceSheet => "NetworkUsagePriceSheet",
            Self::Tariff => "Tariff",
            Self::TariffInfo => "TariffInfo",
            Self::TariffPriceSheet => "TariffPriceSheet",
            Self::EnergyAmount => "EnergyAmount",
            Self::Device => "Device",
            Self::LoadProfile => "LoadProfile",
            Self::ControllableResource => "ControllableResource",
            Self::TechnicalResource => "TechnicalResource",
            Self::Meter => "Meter",
            Self::TimeSeries => "TimeSeries",
            Self::Balancing => "Balancing",
            Self::Region => "Region",
            Self::RegionalTariff => "RegionalTariff",
            Self::LocationProperties => "LocationProperties",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bo_type_serialize() {
        let typ = BoType::Meter;
        let json = serde_json::to_string(&typ).unwrap();
        assert_eq!(json, r#""Zaehler""#);
    }

    #[test]
    fn test_bo_type_deserialize() {
        let typ: BoType = serde_json::from_str(r#""Zaehler""#).unwrap();
        assert_eq!(typ, BoType::Meter);
    }

    #[test]
    fn test_bo_type_names() {
        let typ = BoType::MarketLocation;
        assert_eq!(typ.german_name(), "Marktlokation");
        assert_eq!(typ.english_name(), "MarketLocation");
    }
}
```

**Step 2: Create ComType enum**

Create `crates/bo4e-core/src/enums/com_type.rs`:

```rust
//! Component type discriminator.

use serde::{Deserialize, Serialize};

/// Type discriminator for Components.
///
/// Used in the `_typ` field to identify the concrete type of a BO4E component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ComType {
    // Address related
    #[serde(rename = "Adresse")]
    Address,
    #[serde(rename = "Geokoordinaten")]
    GeoCoordinates,
    #[serde(rename = "Katasteradresse")]
    CadastralAddress,

    // Pricing
    #[serde(rename = "Preis")]
    Price,
    #[serde(rename = "Preisposition")]
    PricePosition,
    #[serde(rename = "Preisstaffel")]
    PriceTier,
    #[serde(rename = "Tarifpreis")]
    TariffPrice,
    #[serde(rename = "Tarifpreisposition")]
    TariffPricePosition,
    #[serde(rename = "RegionalePreisstaffel")]
    RegionalPriceTier,

    // Quantities
    #[serde(rename = "Betrag")]
    Amount,
    #[serde(rename = "Menge")]
    Quantity,
    #[serde(rename = "Messwert")]
    MeasuredValue,
    #[serde(rename = "Steuerbetrag")]
    TaxAmount,

    // Time related
    #[serde(rename = "Zeitraum")]
    TimePeriod,
    #[serde(rename = "Zaehlwerk")]
    MeterRegister,
    #[serde(rename = "Zaehlzeitregister")]
    TimeOfUseRegister,

    // Costs
    #[serde(rename = "Fremdkostenblock")]
    ExternalCostBlock,
    #[serde(rename = "Fremdkostenposition")]
    ExternalCostPosition,
    #[serde(rename = "Kostenblock")]
    CostBlock,
    #[serde(rename = "Kostenposition")]
    CostPosition,

    // Tariff
    #[serde(rename = "AufAbschlag")]
    Surcharge,
    #[serde(rename = "AufAbschlagProOrt")]
    SurchargePerLocation,
    #[serde(rename = "AufAbschlagRegional")]
    RegionalSurcharge,
    #[serde(rename = "PositionsAufAbschlag")]
    PositionSurcharge,
    #[serde(rename = "Tarifberechnungsparameter")]
    TariffCalculationParameter,
    #[serde(rename = "Tarifeinschraenkung")]
    TariffRestriction,

    // Contract
    #[serde(rename = "Vertragskonditionen")]
    ContractConditions,
    #[serde(rename = "Vertragsteil")]
    ContractPart,

    // Energy
    #[serde(rename = "Energieherkunft")]
    EnergySource,
    #[serde(rename = "Energiemix")]
    EnergyMix,

    // Invoice
    #[serde(rename = "Rechnungsposition")]
    InvoicePosition,

    // Offer
    #[serde(rename = "Angebotsposition")]
    OfferPosition,
    #[serde(rename = "Angebotsteil")]
    OfferPart,
    #[serde(rename = "Angebotsvariante")]
    OfferVariant,

    // Contact
    #[serde(rename = "Kontaktweg")]
    ContactMethod,
    #[serde(rename = "Unterschrift")]
    Signature,
    #[serde(rename = "Zustaendigkeit")]
    Responsibility,

    // Other
    #[serde(rename = "Preisgarantie")]
    PriceGuarantee,
    #[serde(rename = "Regionskriterium")]
    RegionCriterion,
    #[serde(rename = "Verbrauch")]
    Consumption,
}

impl ComType {
    /// Returns the German type name.
    pub fn german_name(&self) -> &'static str {
        // The serde rename value is already the German name
        match self {
            Self::Address => "Adresse",
            Self::GeoCoordinates => "Geokoordinaten",
            Self::CadastralAddress => "Katasteradresse",
            Self::Price => "Preis",
            Self::PricePosition => "Preisposition",
            Self::PriceTier => "Preisstaffel",
            Self::TariffPrice => "Tarifpreis",
            Self::TariffPricePosition => "Tarifpreisposition",
            Self::RegionalPriceTier => "RegionalePreisstaffel",
            Self::Amount => "Betrag",
            Self::Quantity => "Menge",
            Self::MeasuredValue => "Messwert",
            Self::TaxAmount => "Steuerbetrag",
            Self::TimePeriod => "Zeitraum",
            Self::MeterRegister => "Zaehlwerk",
            Self::TimeOfUseRegister => "Zaehlzeitregister",
            Self::ExternalCostBlock => "Fremdkostenblock",
            Self::ExternalCostPosition => "Fremdkostenposition",
            Self::CostBlock => "Kostenblock",
            Self::CostPosition => "Kostenposition",
            Self::Surcharge => "AufAbschlag",
            Self::SurchargePerLocation => "AufAbschlagProOrt",
            Self::RegionalSurcharge => "AufAbschlagRegional",
            Self::PositionSurcharge => "PositionsAufAbschlag",
            Self::TariffCalculationParameter => "Tarifberechnungsparameter",
            Self::TariffRestriction => "Tarifeinschraenkung",
            Self::ContractConditions => "Vertragskonditionen",
            Self::ContractPart => "Vertragsteil",
            Self::EnergySource => "Energieherkunft",
            Self::EnergyMix => "Energiemix",
            Self::InvoicePosition => "Rechnungsposition",
            Self::OfferPosition => "Angebotsposition",
            Self::OfferPart => "Angebotsteil",
            Self::OfferVariant => "Angebotsvariante",
            Self::ContactMethod => "Kontaktweg",
            Self::Signature => "Unterschrift",
            Self::Responsibility => "Zustaendigkeit",
            Self::PriceGuarantee => "Preisgarantie",
            Self::RegionCriterion => "Regionskriterium",
            Self::Consumption => "Verbrauch",
        }
    }

    /// Returns the English type name.
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Address => "Address",
            Self::GeoCoordinates => "GeoCoordinates",
            Self::CadastralAddress => "CadastralAddress",
            Self::Price => "Price",
            Self::PricePosition => "PricePosition",
            Self::PriceTier => "PriceTier",
            Self::TariffPrice => "TariffPrice",
            Self::TariffPricePosition => "TariffPricePosition",
            Self::RegionalPriceTier => "RegionalPriceTier",
            Self::Amount => "Amount",
            Self::Quantity => "Quantity",
            Self::MeasuredValue => "MeasuredValue",
            Self::TaxAmount => "TaxAmount",
            Self::TimePeriod => "TimePeriod",
            Self::MeterRegister => "MeterRegister",
            Self::TimeOfUseRegister => "TimeOfUseRegister",
            Self::ExternalCostBlock => "ExternalCostBlock",
            Self::ExternalCostPosition => "ExternalCostPosition",
            Self::CostBlock => "CostBlock",
            Self::CostPosition => "CostPosition",
            Self::Surcharge => "Surcharge",
            Self::SurchargePerLocation => "SurchargePerLocation",
            Self::RegionalSurcharge => "RegionalSurcharge",
            Self::PositionSurcharge => "PositionSurcharge",
            Self::TariffCalculationParameter => "TariffCalculationParameter",
            Self::TariffRestriction => "TariffRestriction",
            Self::ContractConditions => "ContractConditions",
            Self::ContractPart => "ContractPart",
            Self::EnergySource => "EnergySource",
            Self::EnergyMix => "EnergyMix",
            Self::InvoicePosition => "InvoicePosition",
            Self::OfferPosition => "OfferPosition",
            Self::OfferPart => "OfferPart",
            Self::OfferVariant => "OfferVariant",
            Self::ContactMethod => "ContactMethod",
            Self::Signature => "Signature",
            Self::Responsibility => "Responsibility",
            Self::PriceGuarantee => "PriceGuarantee",
            Self::RegionCriterion => "RegionCriterion",
            Self::Consumption => "Consumption",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_type_serialize() {
        let typ = ComType::Address;
        let json = serde_json::to_string(&typ).unwrap();
        assert_eq!(json, r#""Adresse""#);
    }

    #[test]
    fn test_com_type_deserialize() {
        let typ: ComType = serde_json::from_str(r#""Adresse""#).unwrap();
        assert_eq!(typ, ComType::Address);
    }

    #[test]
    fn test_com_type_names() {
        let typ = ComType::PriceGuarantee;
        assert_eq!(typ.german_name(), "Preisgarantie");
        assert_eq!(typ.english_name(), "PriceGuarantee");
    }
}
```

**Step 3: Update enums/mod.rs**

Replace `crates/bo4e-core/src/enums/mod.rs`:

```rust
//! Enumerations for BO4E type-safe values.
//!
//! This module contains all the enum types used in BO4E, organized by category.

mod bo_type;
mod com_type;

pub use bo_type::BoType;
pub use com_type::ComType;
```

**Step 4: Run tests**

```bash
cargo test -p bo4e-core -- --nocapture
```

Expected: All tests pass

**Step 5: Commit**

```bash
git add crates/bo4e-core/src/enums/
git commit -m "feat(core): add BoType and ComType discriminator enums"
```

---

## Verification

Run full test suite:

```bash
cargo test --workspace
cargo clippy --workspace
```

Expected: All tests pass, no clippy warnings.

## Test Summary

| Metric | Value |
|--------|-------|
| Tests | 16 |
| Passed | 16 |
| Failed | 0 |
| Skipped | 0 |
| Doc Tests | 3 |

Files tested:
- crates/bo4e-core/src/additional_attribute.rs
- crates/bo4e-core/src/traits.rs
- crates/bo4e-core/src/enums/bo_type.rs
- crates/bo4e-core/src/enums/com_type.rs
- crates/bo4e-core/src/bo/meter.rs (updated to implement new trait)
