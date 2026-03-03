# German Crate Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Generate a `bo4e-german` crate with full German type names, field names, and enum variant names from existing `bo4e-core` English sources.

**Architecture:** A Rust code generator binary parses `bo4e-core` source files using `syn`, maps English→German names using a TOML mapping + serde/schemars attributes, and writes generated German types into `crates/bo4e-german/src/generated/`. The generated code is committed to the repo.

**Tech Stack:** `syn` (parsing), `quote` (codegen), `prettyplease` (formatting), `toml` (config), `serde_json` for tests

---

### Task 1: Create the generator binary crate

**Files:**
- Create: `crates/generate-german/Cargo.toml`
- Create: `crates/generate-german/src/main.rs`
- Modify: `Cargo.toml` (workspace members)

**Step 1: Add generator crate to workspace**

In `Cargo.toml`, add `"crates/generate-german"` to workspace members:

```toml
[workspace]
resolver = "2"
members = [
    "crates/bo4e-core",
    "crates/bo4e-serde",
    "crates/bo4e",
    "crates/generate-german",
]
```

**Step 2: Create generator Cargo.toml**

```toml
[package]
name = "generate-german"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
syn = { version = "2", features = ["full", "parsing", "visit"] }
quote = "1"
proc-macro2 = "1"
prettyplease = "0.2"
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
walkdir = "2"
```

**Step 3: Create minimal main.rs**

```rust
fn main() {
    println!("generate-german: starting code generation");
}
```

**Step 4: Verify it compiles**

Run: `cargo build -p generate-german`
Expected: Compiles successfully

**Step 5: Commit**

```
feat: add generate-german crate skeleton
```

---

### Task 2: Create the type mapping TOML

This file maps English type names to German type names, and English enum variant names to German variant names. Field-level German names are extracted from `schemars(rename)` attributes in the source.

**Files:**
- Create: `crates/generate-german/mapping.toml`

**Step 1: Create the mapping file**

The mapping contains three sections: `[bo]` for business objects, `[com]` for components, `[enums]` for enumerations. Each maps `EnglishTypeName = "DeutscherTypName"`.

For enum variants, use `[enum_variants.EnglishEnumName]` sections mapping `EnglishVariant = "DeutscheVariante"`.

This file must cover all 35 BOs, 64 COMs, and 73 enums. The German names come from:
- `Bo4eObject::type_name_german()` implementations
- `#[schemars(rename = "...")]` struct-level attributes
- Doc comments `/// German: ...`

The enum variant German names come from doc comments like `/// Electricity (Strom)` → variant `Strom`, and from `#[serde(rename = "STROM")]` values.

**Important:** This is a large file (~500+ lines). Create it with all types. Reference every source file in `crates/bo4e-core/src/` to get the correct German names.

The file format:

```toml
# Type name mappings: English = "German"

[bo]
Balancing = "Bilanzierung"
BundleContract = "Buendelvertrag"
BusinessPartner = "Geschaeftspartner"
ConcessionFeePriceSheet = "KonzessionsabgabePreisblatt"
Contract = "Vertrag"
ControllableResource = "SteuerbarRessource"
Costs = "Kosten"
Device = "Geraet"
EnergyAmount = "Energiemenge"
ExternalCosts = "FremdKosten"
HardwarePriceSheet = "HardwarePreisblatt"
Invoice = "Rechnung"
LoadProfile = "Lastprofil"
LocationAssignment = "Zuordnung"
LocationProperties = "Standorteigenschaften"
MarketLocation = "Marktlokation"
MarketParticipant = "Marktteilnehmer"
Meter = "Zaehler"
MeteringLocation = "Messlokation"
MeteringPriceSheet = "MessstellenbetriebPreisblatt"
NetworkLocation = "Netzlokation"
NetworkUsagePriceSheet = "NetznutzungPreisblatt"
Offer = "Angebot"
Person = "Person"
PriceSheet = "Preisblatt"
Region = "Region"
RegionalTariff = "RegionalerTarif"
ServicePriceSheet = "DienstleistungPreisblatt"
Tariff = "Tarif"
TariffCosts = "TarifKosten"
TariffInfo = "TarifInfo"
TariffPriceSheet = "TarifPreisblatt"
TechnicalResource = "TechnischeRessource"
Tender = "Ausschreibung"
TimeSeries = "Zeitreihe"

[com]
Address = "Adresse"
AggregatedValue = "AggregierterWert"
Amount = "Betrag"
BillingPeriodData = "Abrechnungszeitraumdaten"
Bonus = "Bonus"
CadastralAddress = "Katasteradresse"
ConcessionFee = "Konzessionsabgabe"
ConsumptionQuantity = "Verbrauchsmenge"  # Note: check actual name
ConsumedQuantity = "VerbrauchteQuanti"
Consumption = "Verbrauch"
ContactMethod = "Kontaktweg"
ContractConditions = "Vertragskonditionen"
ContractPart = "Vertragsteil"
CostBlock = "Kostenblock"
CostPosition = "Kostenposition"
DateRange = "Zeitspanne"
Discount = "Rabatt"
EnergyMix = "Energiemix"
EnergySource = "Energiequelle"
ExternalCostBlock = "FremdKostenblock"
ExternalCostPosition = "FremdKostenposition"
ExternalReference = "ExterneReferenz"
GeoCoordinates = "Geokoordinaten"
Hardware = "Hardware"
Interval = "Intervall"
InvoicePosition = "Rechnungsposition"
Levy = "Abgabe"
LoadCurveData = "Lastkurvendaten"
LoadProfileValue = "Lastprofilwert"
MarginPrice = "Marge"
MeasuredValue = "Messwert"
MeterReading = "Zaehlerstand"
MeterRegister = "Zaehlwerk"
MeteringPointStatus = "Messpunktstatus"
NetworkCharge = "Netzentgelt"
OfferPart = "Angebotsteil"
OfferPosition = "Angebotsposition"
OfferVariant = "Angebotsvariante"
PositionSurcharge = "PositionsAufAbschlag"
Price = "Preis"
PriceGuarantee = "Preisgarantie"
PricePosition = "Preisposition"
PriceTier = "Preisstaffel"
ProfileData = "Profildaten"
QualityIndicator = "Qualitaetsindikator"
Quantity = "Menge"
RegionCriterion = "Regionskriterium"
RegionalPriceTier = "RegionalePreisstaffel"
RegionalSurcharge = "RegionalerAufAbschlag"
Responsibility = "Zustaendigkeit"
SeasonalTariff = "SaisonTarif"
ServicePrice = "Dienstleistungspreis"
Signature = "Unterschrift"
SubstitutionValue = "Ersatzwert"
Surcharge = "AufAbschlag"
SurchargePerLocation = "AufAbschlagProOrt"
TariffCalculationParameter = "TarifBerechnungsparameter"
TariffPrice = "TarifPreis"
TariffPricePosition = "TarifPreisposition"
TariffRestriction = "TarifEinschraenkung"
TaxAmount = "Steuerbetrag"
TimePeriod = "Zeitraum"
TimeOfUseRegister = "ZeitZaehlwerk"
TimeSeriesValue = "Zeitreihenwert"
ValidationResult = "Validierungsergebnis"

[enums]
AreaType = "Gebietstyp"
ArithmeticOperation = "ArithmetischeOperation"
BoType = "BoTyp"
BusinessPartnerRole = "Geschaeftspartnerrolle"
CalculationFormula = "Berechnungsformel"
CalculationMethod = "Berechnungsmethode"
ComType = "ComTyp"
ConcessionFeeCustomerGroup = "KonzessionsabgabeKundengruppe"
ConcessionFeeType = "KonzessionsabgabeArt"
ContactType = "Kontaktart"
ContractForm = "Vertragsform"
ContractStatus = "Vertragsstatus"
ContractType = "Vertragsart"
ControllableResourceType = "SteuerbarRessourceTyp"
CostClass = "Kostenklasse"
Country = "Landescode"
Currency = "Waehrung"
CustomerGroup = "Kundengruppe"
CustomerType = "Kundentyp"
DeviceCategory = "Geraetekategorie"
DeviceType = "Geraetetyp"
Division = "Sparte"
EcoCertificate = "OekoZertifikat"
EcoLabel = "OekoLabel"
EnergyDirection = "Energierichtung"
GenerationType = "Erzeugungsart"
InvoiceStatus = "Rechnungsstatus"
InvoiceType = "Rechnungstyp"
LocationType = "Standorttyp"
MarketRole = "Marktrolle"
MeasuredQuantity = "Messgrösse"
MeasuredValueStatus = "Messwertstatus"
MeasurementPriceType = "Messpreistyp"
MeasurementType = "Messart"
Medium = "Medium"
MeterCategory = "Zaehlerkategorie"
MeterSize = "Zaehlergroesse"
MeterType = "Zaehlertyp"
NetworkLevel = "Netzebene"
OfferStatus = "Angebotsstatus"
OrganizationType = "Organisationstyp"
PaymentMethod = "Zahlungsmethode"
PhaseType = "Phasentyp"
PriceGuaranteeType = "Preisgarantietyp"
PriceModel = "Preismodell"
PriceStatus = "Preisstatus"
PriceType = "Preistyp"
ReadingType = "Ableseart"
RegionCriterionType = "RegionskriteriumTyp"
RegionType = "Regionstyp"
RegisterType = "Registerart"
RoundingMode = "Rundungsmodus"
Salutation = "Anrede"
ServiceType = "Dienstleistungstyp"
SubjectArea = "Themengebiet"
SurchargeTarget = "AufAbschlagZiel"
SurchargeType = "AufAbschlagTyp"
TariffCalculationMethod = "TarifBerechnungsmethode"
TariffFeature = "TarifMerkmal"
TariffRegionCriterion = "TarifRegionskriterium"
TariffTime = "TarifZeit"
TariffType = "Tariftyp"
TaxType = "Steuerart"
TechnicalResourceUsage = "TechnischeRessourceNutzung"
TenderStatus = "Ausschreibungsstatus"
TenderType = "Ausschreibungstyp"
TimeUnit = "Zeiteinheit"
Title = "Titel"
Unit = "Mengeneinheit"
UnitPrefix = "Mengeneinheitpraefix"
UsageType = "Verwendungsart"
ValidityType = "Gueltigkeitstyp"
VoltageLevel = "Spannungsebene"
```

**Note:** The enum variant mappings will be derived automatically from the `#[serde(rename = "VARIANT_NAME")]` attributes and doc comments during generation. We don't need to manually map every variant — the generator will parse the doc comment pattern `/// English Name (DeutscherName)` to extract German variant names.

**Step 2: Verify TOML parses**

Add a quick test in `main.rs`:

```rust
fn main() {
    let toml_str = include_str!("../mapping.toml");
    let mapping: toml::Value = toml::from_str(toml_str).unwrap();
    println!("Loaded {} BO mappings", mapping["bo"].as_table().unwrap().len());
    println!("Loaded {} COM mappings", mapping["com"].as_table().unwrap().len());
    println!("Loaded {} enum mappings", mapping["enums"].as_table().unwrap().len());
}
```

Run: `cargo run -p generate-german`
Expected: Prints counts matching actual type counts

**Step 3: Commit**

```
feat: add English→German type mapping for code generator
```

---

### Task 3: Implement source file parser

Parse bo4e-core `.rs` files to extract struct/enum definitions with all their serde attributes.

**Files:**
- Create: `crates/generate-german/src/parser.rs`
- Modify: `crates/generate-german/src/main.rs`

**Step 1: Define parsed data structures**

```rust
// crates/generate-german/src/parser.rs

use std::collections::HashMap;
use std::path::Path;

/// Parsed information about a struct field
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,                    // English field name (snake_case)
    pub german_name: Option<String>,     // From serde(alias) or schemars(rename)
    pub type_str: String,                // Full type as string (e.g. "Option<String>")
    pub is_vec: bool,                    // Whether it's a Vec<T>
    pub is_option: bool,                 // Whether it's Option<T>
    pub is_box: bool,                    // Whether it contains Box<T>
    pub inner_type: Option<String>,      // Inner type name for BO4E types (e.g. "Address", "Division")
    pub is_flatten: bool,               // #[serde(flatten)] — skip for German struct (meta field)
    pub serde_rename: Option<String>,    // #[serde(rename = "...")] for explicit renames
}

/// Parsed information about an enum variant
#[derive(Debug, Clone)]
pub struct VariantInfo {
    pub name: String,                    // English variant name (PascalCase)
    pub german_name: Option<String>,     // From doc comment pattern "English (German)"
    pub serde_rename: Option<String>,    // #[serde(rename = "WIRE_VALUE")]
}

/// Parsed information about a struct
#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,                    // English struct name
    pub german_type_name: Option<String>, // From schemars(rename) or Bo4eObject impl
    pub fields: Vec<FieldInfo>,
    pub source_file: String,             // Relative path
    pub module_path: String,             // e.g. "bo" or "com"
}

/// Parsed information about an enum
#[derive(Debug, Clone)]
pub struct EnumInfo {
    pub name: String,
    pub german_type_name: Option<String>,
    pub variants: Vec<VariantInfo>,
    pub source_file: String,
    pub module_path: String,
    pub has_copy: bool,                  // Whether it derives Copy
}

/// All parsed types from bo4e-core
#[derive(Debug, Default)]
pub struct ParsedCrate {
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
}
```

**Step 2: Implement the file parser**

The parser needs to:
1. Walk `crates/bo4e-core/src/bo/`, `com/`, `enums/` directories
2. Parse each `.rs` file with `syn::parse_file`
3. For each `struct` item, extract fields and their attributes
4. For each `enum` item, extract variants and their attributes
5. Extract German names from `schemars(rename = "...")` and `serde(alias = "...")`

Key parsing logic for extracting German field names from attributes:
- Look for `#[serde(alias = "germanName")]` — parse the `alias = "..."` from the attribute
- Look for `#[cfg_attr(feature = "json-schema", schemars(rename = "germanName"))]` — parse the `rename = "..."` value
- Prefer `serde(alias)` if present, fall back to `schemars(rename)`

For enum variants, extract German names from doc comments:
- Pattern: `/// English description (GermanName)` — extract the parenthesized German name

For struct-level German type names:
- Look for `#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehler"))]`
- Or parse `Bo4eObject` impl for `type_name_german()`

**Step 3: Write a test that parses meter.rs**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_meter() {
        let source = std::fs::read_to_string("../bo4e-core/src/bo/meter.rs").unwrap();
        let parsed = parse_file(&source, "bo/meter.rs", "bo");

        assert_eq!(parsed.structs.len(), 1);
        let meter = &parsed.structs[0];
        assert_eq!(meter.name, "Meter");
        assert_eq!(meter.german_type_name.as_deref(), Some("Zaehler"));

        // Check field: meter_number → zaehlernummer
        let field = meter.fields.iter().find(|f| f.name == "meter_number").unwrap();
        assert_eq!(field.german_name.as_deref(), Some("zaehlernummer"));
        assert!(field.is_option);
        assert!(!field.is_vec);
    }

    #[test]
    fn test_parse_division_enum() {
        let source = std::fs::read_to_string("../bo4e-core/src/enums/division.rs").unwrap();
        let parsed = parse_file(&source, "enums/division.rs", "enums");

        assert_eq!(parsed.enums.len(), 1);
        let div = &parsed.enums[0];
        assert_eq!(div.name, "Division");
        assert_eq!(div.german_type_name.as_deref(), Some("Sparte"));

        let electricity = div.variants.iter().find(|v| v.name == "Electricity").unwrap();
        assert_eq!(electricity.serde_rename.as_deref(), Some("STROM"));
        assert_eq!(electricity.german_name.as_deref(), Some("Strom"));
    }
}
```

**Step 4: Run tests**

Run: `cargo test -p generate-german`
Expected: Both tests pass

**Step 5: Implement full crate parsing**

Add a function that walks all source directories:

```rust
pub fn parse_bo4e_core(core_path: &Path) -> ParsedCrate {
    let mut result = ParsedCrate::default();

    for (dir, module) in [("bo", "bo"), ("com", "com"), ("enums", "enums")] {
        let dir_path = core_path.join("src").join(dir);
        for entry in walkdir::WalkDir::new(&dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| e.file_name() != "mod.rs")
        {
            let source = std::fs::read_to_string(entry.path()).unwrap();
            let rel_path = format!("{}/{}", dir, entry.file_name().to_string_lossy());
            let parsed = parse_file(&source, &rel_path, module);
            result.structs.extend(parsed.structs);
            result.enums.extend(parsed.enums);
        }
    }

    result
}
```

**Step 6: Add integration test**

```rust
#[test]
fn test_parse_full_crate() {
    let parsed = parse_bo4e_core(Path::new("../bo4e-core"));
    // Should find all 35 BOs + 64 COMs = 99 structs (some COMs may not have Bo4eObject)
    assert!(parsed.structs.len() >= 90, "Found {} structs", parsed.structs.len());
    // Should find all 73 enums
    assert!(parsed.enums.len() >= 70, "Found {} enums", parsed.enums.len());
}
```

**Step 7: Commit**

```
feat: implement source file parser for bo4e-core types
```

---

### Task 4: Implement enum code generator

Generate German enum files from parsed EnumInfo.

**Files:**
- Create: `crates/generate-german/src/codegen.rs`
- Modify: `crates/generate-german/src/main.rs`

**Step 1: Implement enum generation**

```rust
// crates/generate-german/src/codegen.rs

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::EnumInfo;

/// Generate a German enum from parsed EnumInfo
pub fn generate_german_enum(info: &EnumInfo, german_name: &str) -> TokenStream {
    let german_ident = format_ident!("{}", german_name);
    let english_path = match info.module_path.as_str() {
        "enums" => {
            let english_ident = format_ident!("{}", info.name);
            quote! { bo4e_core::enums::#english_ident }
        }
        _ => unreachable!("Enums only live in enums module"),
    };

    let mut variant_defs = Vec::new();
    let mut to_german_arms = Vec::new();
    let mut to_english_arms = Vec::new();

    for variant in &info.variants {
        let german_variant_name = variant.german_name.as_deref()
            .unwrap_or(&variant.name); // Fall back to English if no German name
        let german_var = format_ident!("{}", german_variant_name);
        let english_var = format_ident!("{}", variant.name);

        let serde_attr = if let Some(ref rename) = variant.serde_rename {
            quote! { #[serde(rename = #rename)] }
        } else {
            quote! {}
        };

        variant_defs.push(quote! {
            #serde_attr
            #german_var
        });

        to_german_arms.push(quote! {
            #english_path::#english_var => #german_ident::#german_var
        });
        to_english_arms.push(quote! {
            #german_ident::#german_var => #english_path::#english_var
        });
    }

    let copy_derive = if info.has_copy {
        quote! { Copy, }
    } else {
        quote! {}
    };

    quote! {
        // Auto-generated — do not edit manually
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, #copy_derive PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum #german_ident {
            #(#variant_defs,)*
        }

        impl From<#english_path> for #german_ident {
            fn from(v: #english_path) -> Self {
                match v {
                    #(#to_german_arms,)*
                    // Handle non_exhaustive: unknown variants fall through
                    _ => panic!("Unknown variant in {}", stringify!(#english_path)),
                }
            }
        }

        impl From<#german_ident> for #english_path {
            fn from(v: #german_ident) -> Self {
                match v {
                    #(#to_english_arms,)*
                }
            }
        }
    }
}
```

**Step 2: Write a test for enum generation**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::VariantInfo;

    #[test]
    fn test_generate_division_enum() {
        let info = EnumInfo {
            name: "Division".to_string(),
            german_type_name: Some("Sparte".to_string()),
            variants: vec![
                VariantInfo {
                    name: "Electricity".to_string(),
                    german_name: Some("Strom".to_string()),
                    serde_rename: Some("STROM".to_string()),
                },
                VariantInfo {
                    name: "Gas".to_string(),
                    german_name: Some("Gas".to_string()),
                    serde_rename: Some("GAS".to_string()),
                },
            ],
            source_file: "enums/division.rs".to_string(),
            module_path: "enums".to_string(),
            has_copy: true,
        };

        let tokens = generate_german_enum(&info, "Sparte");
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());

        assert!(code.contains("pub enum Sparte"));
        assert!(code.contains("Strom"));
        assert!(code.contains("Gas"));
        assert!(code.contains("impl From<bo4e_core::enums::Division> for Sparte"));
    }
}
```

**Step 3: Run tests**

Run: `cargo test -p generate-german`
Expected: PASS

**Step 4: Commit**

```
feat: implement German enum code generator
```

---

### Task 5: Implement struct code generator

Generate German struct files from parsed StructInfo.

**Files:**
- Modify: `crates/generate-german/src/codegen.rs`

**Step 1: Implement the type resolver**

The type resolver maps English type names to German type names in field types.

```rust
use std::collections::HashMap;

pub struct TypeResolver {
    /// Maps English type name → German type name
    type_map: HashMap<String, String>,
}

impl TypeResolver {
    pub fn new(type_map: HashMap<String, String>) -> Self {
        Self { type_map }
    }

    /// Resolve a field's type string, replacing BO4E types with German equivalents.
    /// E.g., "Option<Box<BusinessPartner>>" → "Option<Box<Geschaeftspartner>>"
    pub fn resolve_type(&self, type_str: &str) -> String {
        let mut result = type_str.to_string();
        for (english, german) in &self.type_map {
            // Replace type name when it appears as a standalone identifier
            // (not as part of a longer name)
            result = result.replace(english.as_str(), german.as_str());
        }
        result
    }
}
```

**Note:** The type resolution needs to be more sophisticated than simple string replacement to avoid replacing substrings (e.g., `MeterType` shouldn't partially match `Meter`). Sort replacements by length (longest first) and use word-boundary matching.

**Step 2: Implement struct generation**

```rust
pub fn generate_german_struct(
    info: &StructInfo,
    german_name: &str,
    resolver: &TypeResolver,
) -> TokenStream {
    let german_ident = format_ident!("{}", german_name);
    let english_module = match info.module_path.as_str() {
        "bo" => quote! { bo4e_core::bo },
        "com" => quote! { bo4e_core::com },
        _ => unreachable!(),
    };
    let english_ident = format_ident!("{}", info.name);

    let mut field_defs = Vec::new();
    let mut to_german_fields = Vec::new();
    let mut to_english_fields = Vec::new();

    for field in &info.fields {
        if field.is_flatten {
            // Meta field — keep as Bo4eMeta
            field_defs.push(quote! {
                #[serde(flatten)]
                pub meta: bo4e_core::Bo4eMeta
            });
            to_german_fields.push(quote! { meta: v.meta });
            to_english_fields.push(quote! { meta: v.meta });
            continue;
        }

        let german_field_name = field.german_name.as_deref()
            .unwrap_or(&field.name);
        // Convert camelCase german name to snake_case
        let german_snake = camel_to_snake(german_field_name);
        let german_field_ident = format_ident!("{}", german_snake);
        let english_field_ident = format_ident!("{}", field.name);

        // Resolve the type — replace BO4E types with German equivalents
        let resolved_type = resolver.resolve_type(&field.type_str);
        let type_tokens: TokenStream = resolved_type.parse().unwrap();

        // Generate serde attributes
        // The English camelCase name becomes the alias for interop
        let english_camel = snake_to_camel(&field.name);

        let serde_attr = if field.is_vec {
            quote! {
                #[serde(default, skip_serializing_if = "Vec::is_empty", alias = #english_camel)]
            }
        } else {
            quote! {
                #[serde(skip_serializing_if = "Option::is_none", alias = #english_camel)]
            }
        };

        field_defs.push(quote! {
            #serde_attr
            pub #german_field_ident: #type_tokens
        });

        // From/Into field mapping
        // Need to handle Option<T>, Vec<T>, Option<Box<T>> with .map(Into::into) etc.
        let (to_german_expr, to_english_expr) = generate_field_conversion(
            &english_field_ident,
            &german_field_ident,
            field,
        );
        to_german_fields.push(quote! { #german_field_ident: #to_german_expr });
        to_english_fields.push(quote! { #english_field_ident: #to_english_expr });
    }

    quote! {
        // Auto-generated — do not edit manually
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #german_ident {
            #(#field_defs,)*
        }

        impl From<#english_module::#english_ident> for #german_ident {
            fn from(v: #english_module::#english_ident) -> Self {
                Self {
                    #(#to_german_fields,)*
                }
            }
        }

        impl From<#german_ident> for #english_module::#english_ident {
            fn from(v: #german_ident) -> Self {
                Self {
                    #(#to_english_fields,)*
                }
            }
        }
    }
}
```

**Step 3: Implement field conversion helpers**

For `From`/`Into` implementations, handle these patterns:
- `Option<String>` → direct assignment (`.field`)
- `Option<EnumType>` → `.field.map(Into::into)`
- `Option<ComType>` → `.field.map(Into::into)`
- `Option<Box<BoType>>` → `.field.map(|b| Box::new((*b).into()))`
- `Vec<ComType>` → `.field.into_iter().map(Into::into).collect()`
- Primitive types (`String`, `i32`, `f64`, `bool`, `DateTime`) → direct assignment

**Step 4: Implement helper functions**

```rust
/// Convert camelCase to snake_case
fn camel_to_snake(s: &str) -> String { /* ... */ }

/// Convert snake_case to camelCase
fn snake_to_camel(s: &str) -> String { /* ... */ }
```

**Step 5: Write test for struct generation**

Test with the Meter struct info and verify the output contains correct German field names, types, and From/Into impls.

**Step 6: Commit**

```
feat: implement German struct code generator with type resolution
```

---

### Task 6: Implement file writer and main orchestration

Wire everything together in main.rs.

**Files:**
- Create: `crates/generate-german/src/writer.rs`
- Modify: `crates/generate-german/src/main.rs`

**Step 1: Implement the file writer**

```rust
// crates/generate-german/src/writer.rs

use std::path::Path;

/// Write generated code to the bo4e-german crate directory
pub fn write_generated_files(
    output_dir: &Path,        // crates/bo4e-german/src/generated/
    bo_files: Vec<(String, String)>,    // (filename, code)
    com_files: Vec<(String, String)>,
    enum_files: Vec<(String, String)>,
) -> std::io::Result<()> {
    // Create directories
    std::fs::create_dir_all(output_dir.join("bo"))?;
    std::fs::create_dir_all(output_dir.join("com"))?;
    std::fs::create_dir_all(output_dir.join("enums"))?;

    // Write files
    for (filename, code) in &bo_files {
        std::fs::write(output_dir.join("bo").join(filename), code)?;
    }
    for (filename, code) in &com_files {
        std::fs::write(output_dir.join("com").join(filename), code)?;
    }
    for (filename, code) in &enum_files {
        std::fs::write(output_dir.join("enums").join(filename), code)?;
    }

    // Generate mod.rs files
    write_mod_file(output_dir.join("bo").join("mod.rs"), &bo_files)?;
    write_mod_file(output_dir.join("com").join("mod.rs"), &com_files)?;
    write_mod_file(output_dir.join("enums").join("mod.rs"), &enum_files)?;

    Ok(())
}
```

**Step 2: Wire up main.rs**

```rust
mod parser;
mod codegen;
mod writer;

use std::collections::HashMap;
use std::path::Path;

fn main() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let core_path = workspace_root.join("crates/bo4e-core");
    let output_dir = workspace_root.join("crates/bo4e-german/src/generated");

    // Load mapping
    let mapping_str = include_str!("../mapping.toml");
    let mapping: toml::Value = toml::from_str(mapping_str).unwrap();

    // Build type map from all sections
    let mut type_map = HashMap::new();
    for section in ["bo", "com", "enums"] {
        if let Some(table) = mapping[section].as_table() {
            for (english, german) in table {
                type_map.insert(english.clone(), german.as_str().unwrap().to_string());
            }
        }
    }

    // Parse source
    let parsed = parser::parse_bo4e_core(&core_path);
    println!("Parsed {} structs, {} enums", parsed.structs.len(), parsed.enums.len());

    // Generate code
    let resolver = codegen::TypeResolver::new(type_map.clone());

    let mut bo_files = Vec::new();
    let mut com_files = Vec::new();
    let mut enum_files = Vec::new();

    for info in &parsed.structs {
        let german_name = type_map.get(&info.name)
            .or(info.german_type_name.as_ref())
            .expect(&format!("No German name for struct {}", info.name));

        let tokens = codegen::generate_german_struct(info, german_name, &resolver);
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());

        let filename = camel_to_snake_filename(german_name) + ".rs";
        match info.module_path.as_str() {
            "bo" => bo_files.push((filename, code)),
            "com" => com_files.push((filename, code)),
            _ => {}
        }
    }

    for info in &parsed.enums {
        let german_name = type_map.get(&info.name)
            .or(info.german_type_name.as_ref())
            .expect(&format!("No German name for enum {}", info.name));

        let tokens = codegen::generate_german_enum(info, german_name);
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());

        let filename = camel_to_snake_filename(german_name) + ".rs";
        enum_files.push((filename, code));
    }

    // Write output
    writer::write_generated_files(&output_dir, bo_files, com_files, enum_files).unwrap();
    println!("Generated files written to {}", output_dir.display());
}
```

**Step 3: Commit**

```
feat: wire up generator main with file writer
```

---

### Task 7: Create bo4e-german crate skeleton

**Files:**
- Create: `crates/bo4e-german/Cargo.toml`
- Create: `crates/bo4e-german/src/lib.rs`
- Modify: `Cargo.toml` (add workspace member + dependency)

**Step 1: Add to workspace**

In root `Cargo.toml`, add member and dependency:

```toml
[workspace]
members = [
    "crates/bo4e-core",
    "crates/bo4e-serde",
    "crates/bo4e",
    "crates/generate-german",
    "crates/bo4e-german",
]

[workspace.dependencies]
# ... existing deps ...
bo4e-german = { path = "crates/bo4e-german", version = "0.1.1" }
```

**Step 2: Create Cargo.toml**

```toml
[package]
name = "bo4e-german"
description = "German API for BO4E (Business Objects for Energy) - Deutsche Feldnamen"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
keywords = ["bo4e", "energy", "edi", "bdew", "deutsch"]
categories = ["data-structures", "encoding"]

[dependencies]
serde = { workspace = true }
chrono = { workspace = true }
bo4e-core = { workspace = true }

[dev-dependencies]
serde_json = { workspace = true }
```

**Step 3: Create lib.rs**

```rust
//! # BO4E German - Deutsche Feldnamen
//!
//! German API for BO4E (Business Objects for Energy).
//!
//! This crate provides German type names, field names, and enum variant names
//! for the BO4E standard. All types have bidirectional `From`/`Into`
//! conversions with their English counterparts in `bo4e-core`.
//!
//! ## Example
//!
//! ```rust
//! use bo4e_german::prelude::*;
//!
//! let zaehler = Zaehler {
//!     zaehlernummer: Some("1EMH0012345678".to_string()),
//!     sparte: Some(Sparte::Strom),
//!     ..Default::default()
//! };
//! ```

pub mod generated;

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::generated::bo::*;
    pub use crate::generated::com::*;
    pub use crate::generated::enums::*;
}
```

**Step 4: Create placeholder generated module**

Create `crates/bo4e-german/src/generated/mod.rs`:

```rust
pub mod bo;
pub mod com;
pub mod enums;
```

And empty sub-modules:
- `crates/bo4e-german/src/generated/bo/mod.rs` → `// Generated — do not edit`
- `crates/bo4e-german/src/generated/com/mod.rs` → `// Generated — do not edit`
- `crates/bo4e-german/src/generated/enums/mod.rs` → `// Generated — do not edit`

**Step 5: Verify skeleton compiles**

Run: `cargo build -p bo4e-german`
Expected: Compiles (empty crate)

**Step 6: Commit**

```
feat: add bo4e-german crate skeleton
```

---

### Task 8: Run generator and iterate until compilation succeeds

**Step 1: Run the generator**

Run: `cargo run -p generate-german`

This will populate `crates/bo4e-german/src/generated/` with all German types.

**Step 2: Try to compile**

Run: `cargo build -p bo4e-german`

This will likely produce compilation errors. Iterate:
- Fix type resolution issues (wrong type names, missing imports)
- Fix camelCase→snake_case conversion edge cases
- Fix From/Into conversion code for complex types
- Fix module path references

**Step 3: Fix issues in the generator, re-run, repeat**

Common issues to expect:
- `super::` type references in bo4e-core (e.g., `Box<super::BusinessPartner>`) need to be resolved to full paths
- German names that conflict with Rust keywords
- Complex nested types like `Option<Box<T>>` needing correct conversion code
- Some types may not have German names in the mapping

**Step 4: Verify full build**

Run: `cargo build --workspace`
Expected: All crates compile

**Step 5: Commit**

```
feat: generate all German types for bo4e-german
```

---

### Task 9: Add tests

**Files:**
- Create: `crates/bo4e-german/tests/roundtrip.rs`
- Create: `crates/bo4e-german/tests/conversion.rs`

**Step 1: Add serialization roundtrip tests**

```rust
// crates/bo4e-german/tests/roundtrip.rs

use bo4e_german::prelude::*;

#[test]
fn test_zaehler_roundtrip() {
    let zaehler = Zaehler {
        zaehlernummer: Some("1EMH0012345678".to_string()),
        sparte: Some(Sparte::Strom),
        ..Default::default()
    };

    let json = serde_json::to_string(&zaehler).unwrap();
    // German field names in JSON
    assert!(json.contains("zaehlernummer") || json.contains("Zaehlernummer"));

    let parsed: Zaehler = serde_json::from_str(&json).unwrap();
    assert_eq!(zaehler, parsed);
}

#[test]
fn test_adresse_roundtrip() {
    let adresse = Adresse {
        strasse: Some("Musterstraße".to_string()),
        hausnummer: Some("42".to_string()),
        postleitzahl: Some("50667".to_string()),
        ort: Some("Köln".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&adresse).unwrap();
    let parsed: Adresse = serde_json::from_str(&json).unwrap();
    assert_eq!(adresse, parsed);
}

#[test]
fn test_sparte_roundtrip() {
    for sparte in [Sparte::Strom, Sparte::Gas, Sparte::Fernwaerme] {
        let json = serde_json::to_string(&sparte).unwrap();
        let parsed: Sparte = serde_json::from_str(&json).unwrap();
        assert_eq!(sparte, parsed);
    }
}
```

**Step 2: Add English↔German conversion tests**

```rust
// crates/bo4e-german/tests/conversion.rs

use bo4e_core::bo::Meter;
use bo4e_core::enums::Division;
use bo4e_german::prelude::*;

#[test]
fn test_meter_to_zaehler() {
    let meter = Meter {
        meter_number: Some("TEST123".to_string()),
        division: Some(Division::Electricity),
        ..Default::default()
    };

    let zaehler: Zaehler = meter.into();
    assert_eq!(zaehler.zaehlernummer, Some("TEST123".to_string()));
    // Enum should also convert
    assert_eq!(zaehler.sparte, Some(Sparte::Strom));
}

#[test]
fn test_zaehler_to_meter() {
    let zaehler = Zaehler {
        zaehlernummer: Some("TEST456".to_string()),
        sparte: Some(Sparte::Gas),
        ..Default::default()
    };

    let meter: Meter = zaehler.into();
    assert_eq!(meter.meter_number, Some("TEST456".to_string()));
    assert_eq!(meter.division, Some(Division::Gas));
}

#[test]
fn test_roundtrip_conversion() {
    let original = Meter {
        meter_number: Some("ROUND".to_string()),
        division: Some(Division::Electricity),
        ..Default::default()
    };

    let zaehler: Zaehler = original.clone().into();
    let back: Meter = zaehler.into();
    assert_eq!(original, back);
}

#[test]
fn test_division_sparte_conversion() {
    assert_eq!(Sparte::from(Division::Electricity), Sparte::Strom);
    assert_eq!(Division::from(Sparte::Strom), Division::Electricity);
    assert_eq!(Sparte::from(Division::Gas), Sparte::Gas);
}
```

**Step 3: Run tests**

Run: `cargo test -p bo4e-german`
Expected: All pass

**Step 4: Commit**

```
test: add roundtrip and conversion tests for bo4e-german
```

---

### Task 10: Run full CI checks and clean up

**Step 1: Format**

Run: `cargo fmt --all`

**Step 2: Clippy**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Fix any warnings.

**Step 3: Full test suite**

Run: `cargo test --workspace`
Expected: All pass (existing tests unaffected, new tests pass)

**Step 4: Add freshness check script**

Create a simple script or test that verifies generated code matches what the generator would produce. This can be a test in generate-german:

```rust
#[test]
fn test_generated_code_is_fresh() {
    // Run generator to temp dir, diff against committed files
    // This ensures the committed generated code is up-to-date
}
```

**Step 5: Final commit**

```
chore: clean up and add CI checks for bo4e-german
```

---

## Notes for Implementation

### Key Edge Cases

1. **Fields without serde alias:** Some types (like `Contract`, `MeterRegister`) don't have `#[serde(alias = "...")]` on all fields. The generator must fall back to `#[schemars(rename = "...")]` for the German name. Parse the `cfg_attr` wrapping to extract it.

2. **`super::` type references:** The `Contract` struct references `Box<super::BusinessPartner>`. The parser needs to resolve `super::` to the correct module path.

3. **camelCase to snake_case:** German camelCase names like `marktlokationsId` → `marktlokations_id`. Handle edge cases like consecutive uppercase, numbers, and German compound words.

4. **`name1`/`name2`/`name3` fields:** In `BusinessPartner`, these field names are the same in both languages. The generator should detect when a field's German name matches its English name (after serde rename_all camelCase) and handle it correctly.

5. **`Bo4eMeta` is shared:** Don't generate a German version of `Bo4eMeta` or `AdditionalAttribute`. They're used directly from `bo4e_core`.

6. **Enum variant German names from doc comments:** Parse `/// English desc (GermanName)` pattern. Some variants may not follow this pattern — use the serde rename value as a fallback for the German name by converting `SCREAMING_SNAKE` to PascalCase.

### Type Resolution Order

When resolving field types, replace BO4E type names with German names:
1. Sort type names by length (longest first) to avoid substring matches
2. Only replace whole identifiers, not substrings
3. Standard library types (`String`, `i32`, `f64`, `bool`, `DateTime<Utc>`) → keep as-is
4. `Bo4eMeta`, `AdditionalAttribute` → keep as-is
