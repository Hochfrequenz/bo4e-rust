# Epic 3.1: Address & Contact Components

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement address, contact, and person-related components (~15 components).

**Architecture:** Each component is a struct implementing `Bo4eObject` with optional fields.

**Tech Stack:** Rust structs, serde, Option<T> for all fields

---

## Components in This Epic

| Rust Name | German Name | Description |
|-----------|-------------|-------------|
| `Address` | `Adresse` | Physical or postal address |
| `GeoCoordinates` | `Geokoordinaten` | GPS coordinates |
| `CadastralAddress` | `Katasteradresse` | Cadastral/land registry address |
| `ContactMethod` | `Kontaktweg` | Contact method (email, phone) |
| `Signature` | `Unterschrift` | Digital signature |
| `Responsibility` | `Zustaendigkeit` | Area of responsibility |
| `ExternalReference` | `ExterneReferenz` | External system reference |
| `Hardware` | `Hardware` | Hardware component |
| `OfferPosition` | `Angebotsposition` | Position in an offer |
| `OfferPart` | `Angebotsteil` | Part of an offer |
| `OfferVariant` | `Angebotsvariante` | Variant of an offer |
| `ContractPart` | `Vertragsteil` | Part of a contract |
| `ContractConditions` | `Vertragskonditionen` | Contract conditions |
| `InvoicePosition` | `Rechnungsposition` | Position in an invoice |
| `RegionCriterion` | `Regionskriterium` | Regional criterion |

---

## Task 1: Create Address Component

**Files:**
- Create: `crates/bo4e-core/src/com/address.rs`
- Modify: `crates/bo4e-core/src/com/mod.rs`

**Step 1: Write the failing test**

Add to `crates/bo4e-core/src/com/address.rs`:

```rust
//! Address (Adresse) component.

use serde::{Deserialize, Serialize};
use crate::enums::Country;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Physical or postal address.
///
/// German: Adresse
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Address;
///
/// let address = Address {
///     street: Some("Musterstraße".to_string()),
///     house_number: Some("42".to_string()),
///     postal_code: Some("50667".to_string()),
///     city: Some("Köln".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Street name (Strasse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    /// House number (Hausnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,

    /// House number addition (Hausnummernzusatz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number_addition: Option<String>,

    /// Postal code (Postleitzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// City/town (Ort)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// District (Ortsteil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,

    /// Country code (Landescode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Country>,

    /// PO Box number (Postfach)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub po_box: Option<String>,

    /// Co-location info (c/o)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co_ergaenzung: Option<String>,
}

impl Bo4eObject for Address {
    fn type_name_german() -> &'static str {
        "Adresse"
    }

    fn type_name_english() -> &'static str {
        "Address"
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
    fn test_address_default() {
        let address = Address::default();
        assert!(address.street.is_none());
        assert!(address.city.is_none());
    }

    #[test]
    fn test_address_serialize() {
        let address = Address {
            street: Some("Hauptstraße".to_string()),
            house_number: Some("1".to_string()),
            postal_code: Some("12345".to_string()),
            city: Some("Berlin".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains(r#""street":"Hauptstraße""#));
        assert!(json.contains(r#""houseNumber":"1""#));
        assert!(json.contains(r#""postalCode":"12345""#));
        assert!(json.contains(r#""city":"Berlin""#));
    }

    #[test]
    fn test_address_deserialize() {
        let json = r#"{
            "street": "Musterweg",
            "houseNumber": "42a",
            "postalCode": "50667",
            "city": "Köln"
        }"#;

        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(address.street, Some("Musterweg".to_string()));
        assert_eq!(address.house_number, Some("42a".to_string()));
        assert_eq!(address.postal_code, Some("50667".to_string()));
        assert_eq!(address.city, Some("Köln".to_string()));
    }

    #[test]
    fn test_address_roundtrip() {
        let address = Address {
            meta: Bo4eMeta::with_type("Adresse"),
            street: Some("Teststraße".to_string()),
            house_number: Some("123".to_string()),
            postal_code: Some("99999".to_string()),
            city: Some("Teststadt".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&address).unwrap();
        let parsed: Address = serde_json::from_str(&json).unwrap();
        assert_eq!(address, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Address::type_name_german(), "Adresse");
        assert_eq!(Address::type_name_english(), "Address");
    }
}
```

**Step 2: Run tests**

```bash
cargo test -p bo4e-core address -- --nocapture
```

Expected: All tests pass

**Step 3: Update mod.rs**

Replace `crates/bo4e-core/src/com/mod.rs`:

```rust
//! Components (COMs) - composite types used within Business Objects.
//!
//! Components are reusable data structures that appear within multiple
//! Business Objects. Unlike BOs, they cannot exist independently.

mod address;

pub use address::Address;
```

**Step 4: Commit**

```bash
git add crates/bo4e-core/src/com/
git commit -m "feat(core): add Address (Adresse) component"
```

---

## Task 2: Create GeoCoordinates Component

**Files:**
- Create: `crates/bo4e-core/src/com/geo_coordinates.rs`

**Step 1: Write the implementation**

```rust
//! Geographic coordinates (Geokoordinaten) component.

use serde::{Deserialize, Serialize};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Geographic coordinates (latitude/longitude).
///
/// German: Geokoordinaten
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoCoordinates {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Latitude in decimal degrees (Breitengrad)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude in decimal degrees (Laengengrad)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

impl Bo4eObject for GeoCoordinates {
    fn type_name_german() -> &'static str { "Geokoordinaten" }
    fn type_name_english() -> &'static str { "GeoCoordinates" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cologne_coordinates() {
        let coords = GeoCoordinates {
            latitude: Some(50.9375),
            longitude: Some(6.9603),
            ..Default::default()
        };

        let json = serde_json::to_string(&coords).unwrap();
        assert!(json.contains("50.9375"));
        assert!(json.contains("6.9603"));

        let parsed: GeoCoordinates = serde_json::from_str(&json).unwrap();
        assert_eq!(coords, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core geo_coordinates
git add crates/bo4e-core/src/com/geo_coordinates.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add GeoCoordinates (Geokoordinaten) component"
```

---

## Task 3: Create ContactMethod Component

**Files:**
- Create: `crates/bo4e-core/src/com/contact_method.rs`

**Step 1: Write the implementation**

```rust
//! Contact method (Kontaktweg) component.

use serde::{Deserialize, Serialize};
use crate::enums::ContactType;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Method of contact (email, phone, etc.).
///
/// German: Kontaktweg
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactMethod {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of contact (Kontaktart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<ContactType>,

    /// Contact value (phone number, email, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_value: Option<String>,

    /// Description/note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this is the primary contact method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}

impl Bo4eObject for ContactMethod {
    fn type_name_german() -> &'static str { "Kontaktweg" }
    fn type_name_english() -> &'static str { "ContactMethod" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_contact() {
        let contact = ContactMethod {
            contact_value: Some("test@example.com".to_string()),
            is_primary: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&contact).unwrap();
        let parsed: ContactMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(contact, parsed);
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core contact_method
git add crates/bo4e-core/src/com/contact_method.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add ContactMethod (Kontaktweg) component"
```

---

## Task 4: Create ExternalReference Component

**Files:**
- Create: `crates/bo4e-core/src/com/external_reference.rs`

**Step 1: Write the implementation**

```rust
//! External reference (ExterneReferenz) component.

use serde::{Deserialize, Serialize};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Reference to an external system.
///
/// Used to link BO4E objects to identifiers in other systems.
///
/// German: ExterneReferenz
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalReference {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name of the external system (ExRefName)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref_name: Option<String>,

    /// Value/ID in the external system (ExRefWert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref_value: Option<String>,
}

impl Bo4eObject for ExternalReference {
    fn type_name_german() -> &'static str { "ExterneReferenz" }
    fn type_name_english() -> &'static str { "ExternalReference" }
    fn meta(&self) -> &Bo4eMeta { &self.meta }
    fn meta_mut(&mut self) -> &mut Bo4eMeta { &mut self.meta }
}

impl ExternalReference {
    /// Create a new external reference.
    pub fn new(system: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            external_ref_name: Some(system.into()),
            external_ref_value: Some(value.into()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sap_reference() {
        let ext_ref = ExternalReference::new("SAP", "4711");
        assert_eq!(ext_ref.external_ref_name, Some("SAP".to_string()));
        assert_eq!(ext_ref.external_ref_value, Some("4711".to_string()));
    }

    #[test]
    fn test_serialize() {
        let ext_ref = ExternalReference::new("CRM", "CUST-12345");
        let json = serde_json::to_string(&ext_ref).unwrap();
        assert!(json.contains(r#""externalRefName":"CRM""#));
        assert!(json.contains(r#""externalRefValue":"CUST-12345""#));
    }
}
```

**Step 2: Run tests and commit**

```bash
cargo test -p bo4e-core external_reference
git add crates/bo4e-core/src/com/external_reference.rs crates/bo4e-core/src/com/mod.rs
git commit -m "feat(core): add ExternalReference (ExterneReferenz) component"
```

---

## Tasks 5-15: Remaining Address/Contact Components

Follow the established pattern for:
- `CadastralAddress`
- `Signature`
- `Responsibility`
- `Hardware`
- `OfferPosition`
- `OfferPart`
- `OfferVariant`
- `ContractPart`
- `ContractConditions`
- `InvoicePosition`
- `RegionCriterion`

Reference `../BO4E-python/src/bo4e/com/` for field definitions.

---

## Verification

```bash
cargo test -p bo4e-core com -- --nocapture
cargo clippy -p bo4e-core
```

Expected: All tests pass, no warnings.
