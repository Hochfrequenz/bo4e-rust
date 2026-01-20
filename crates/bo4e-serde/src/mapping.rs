//! Field name mappings between English (Rust) and German (BO4E JSON).

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Mapping from English field names to German field names.
static ENGLISH_TO_GERMAN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Meter fields
    map.insert("meter_number", "zaehlernummer");
    map.insert("meter_type", "zaehlertyp");
    map.insert("meter_size", "zaehlergroesse");
    map.insert("registers", "zaehlwerke");

    // MarketLocation fields
    map.insert("market_location_id", "marktlokationsId");
    map.insert("energy_direction", "energierichtung");
    map.insert("annual_consumption", "jahresverbrauchsprognose");
    map.insert("supply_start", "lieferbeginn");
    map.insert("supply_end", "lieferende");
    map.insert("network_operator_code", "netzbetreiberCodenummer");

    // MeteringLocation fields
    map.insert("metering_location_id", "messlokationsId");
    map.insert("metering_operator_code", "messstellenbetreiberCodenummer");

    // BusinessPartner fields
    map.insert("partner_id", "geschaeftspartnerId");
    map.insert("commercial_register_number", "handelsregisternummer");
    map.insert("tax_id", "steuernummer");
    map.insert("vat_id", "umsatzsteuerId");

    // Contract fields
    map.insert("contract_number", "vertragsnummer");
    map.insert("contract_type", "vertragsart");
    map.insert("contract_start", "vertragsbeginn");
    map.insert("contract_end", "vertragsende");
    map.insert("signing_date", "unterzeichnungsdatum");
    map.insert("validity_period", "gueltigkeitszeitraum");
    map.insert("contract_partner", "vertragspartner");

    // Invoice fields
    map.insert("invoice_number", "rechnungsnummer");
    map.insert("invoice_type", "rechnungstyp");
    map.insert("invoice_date", "rechnungsdatum");
    map.insert("due_date", "faelligkeitsdatum");
    map.insert("billing_period", "abrechnungszeitraum");
    map.insert("net_amount", "nettobetrag");
    map.insert("tax_amount", "steuerbetrag");
    map.insert("gross_amount", "bruttobetrag");
    map.insert("positions", "rechnungspositionen");

    // Address fields
    map.insert("street", "strasse");
    map.insert("house_number", "hausnummer");
    map.insert("house_number_addition", "hausnummernzusatz");
    map.insert("postal_code", "postleitzahl");
    map.insert("city", "ort");
    map.insert("district", "ortsteil");
    map.insert("country_code", "landescode");
    map.insert("po_box", "postfach");

    // Common fields
    map.insert("division", "sparte");
    map.insert("description", "beschreibung");
    map.insert("status", "status");
    map.insert("value", "wert");
    map.insert("unit", "einheit");
    map.insert("currency", "waehrung");
    map.insert("timestamp", "zeitpunkt");
    map.insert("start", "startdatum");
    map.insert("end", "enddatum");

    // Price fields
    map.insert("price_type", "preistyp");
    map.insert("reference_unit", "bezugsgroesse");
    map.insert("base_price", "grundpreis");
    map.insert("working_price", "arbeitspreis");
    map.insert("price_tiers", "preisstaffeln");

    // Additional attribute fields
    map.insert("additional_attributes", "zusatzAttribute");

    map
});

/// Mapping from German field names to English field names.
static GERMAN_TO_ENGLISH: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    ENGLISH_TO_GERMAN
        .iter()
        .map(|(en, de)| (*de, *en))
        .collect()
});

/// Get the German field name for an English field name.
pub fn to_german(english: &str) -> Option<&'static str> {
    ENGLISH_TO_GERMAN.get(english).copied()
}

/// Get the English field name for a German field name.
pub fn to_english(german: &str) -> Option<&'static str> {
    GERMAN_TO_ENGLISH.get(german).copied()
}

/// Check if a field name is a known English field.
pub fn is_english_field(name: &str) -> bool {
    ENGLISH_TO_GERMAN.contains_key(name)
}

/// Check if a field name is a known German field.
pub fn is_german_field(name: &str) -> bool {
    GERMAN_TO_ENGLISH.contains_key(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_german() {
        assert_eq!(to_german("meter_number"), Some("zaehlernummer"));
        assert_eq!(to_german("market_location_id"), Some("marktlokationsId"));
        assert_eq!(to_german("unknown_field"), None);
    }

    #[test]
    fn test_to_english() {
        assert_eq!(to_english("zaehlernummer"), Some("meter_number"));
        assert_eq!(to_english("marktlokationsId"), Some("market_location_id"));
        assert_eq!(to_english("unknown_field"), None);
    }

    #[test]
    fn test_bidirectional() {
        // Every English -> German should have German -> English
        for (en, de) in ENGLISH_TO_GERMAN.iter() {
            assert_eq!(
                to_english(de),
                Some(*en),
                "Missing reverse mapping for {} -> {}",
                en,
                de
            );
        }
    }

    #[test]
    fn test_field_detection() {
        assert!(is_english_field("meter_number"));
        assert!(!is_english_field("zaehlernummer"));

        assert!(is_german_field("zaehlernummer"));
        assert!(!is_german_field("meter_number"));
    }
}
