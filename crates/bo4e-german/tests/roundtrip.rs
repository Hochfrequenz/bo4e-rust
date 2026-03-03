use bo4e_german::prelude::*;

#[test]
fn test_zaehler_roundtrip() {
    let zaehler = Zaehler {
        zaehlernummer: Some("1EMH0012345678".to_string()),
        sparte: Some(Sparte::Strom),
        ..Default::default()
    };

    let json = serde_json::to_string(&zaehler).unwrap();
    let parsed: Zaehler = serde_json::from_str(&json).unwrap();
    assert_eq!(zaehler, parsed);
}

#[test]
fn test_zaehler_serializes_german_field_names() {
    let zaehler = Zaehler {
        zaehlernummer: Some("TEST123".to_string()),
        sparte: Some(Sparte::Gas),
        hersteller: Some("Acme".to_string()),
        herstellungsjahr: Some(2024),
        ..Default::default()
    };

    let json = serde_json::to_string(&zaehler).unwrap();
    assert!(json.contains("\"zaehlernummer\""));
    assert!(json.contains("\"sparte\""));
    assert!(json.contains("\"hersteller\""));
    assert!(json.contains("\"herstellungsjahr\""));
}

#[test]
fn test_adresse_roundtrip() {
    let adresse = Adresse {
        strasse: Some("Musterstra\u{00df}e".to_string()),
        hausnummer: Some("42".to_string()),
        postleitzahl: Some("50667".to_string()),
        ort: Some("K\u{00f6}ln".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&adresse).unwrap();
    let parsed: Adresse = serde_json::from_str(&json).unwrap();
    assert_eq!(adresse, parsed);
}

#[test]
fn test_adresse_serializes_german_field_names() {
    let adresse = Adresse {
        strasse: Some("Hauptstra\u{00df}e".to_string()),
        hausnummer: Some("1".to_string()),
        postleitzahl: Some("12345".to_string()),
        ort: Some("Berlin".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&adresse).unwrap();
    assert!(json.contains("\"strasse\""));
    assert!(json.contains("\"hausnummer\""));
    assert!(json.contains("\"postleitzahl\""));
    assert!(json.contains("\"ort\""));
}

#[test]
fn test_sparte_roundtrip() {
    for sparte in [
        Sparte::Strom,
        Sparte::Gas,
        Sparte::Fernwaerme,
        Sparte::Nahwaerme,
        Sparte::Wasser,
        Sparte::Abwasser,
        Sparte::StromUndGas,
    ] {
        let json = serde_json::to_string(&sparte).unwrap();
        let parsed: Sparte = serde_json::from_str(&json).unwrap();
        assert_eq!(sparte, parsed);
    }
}

#[test]
fn test_sparte_serialization_values() {
    assert_eq!(serde_json::to_string(&Sparte::Strom).unwrap(), r#""STROM""#);
    assert_eq!(serde_json::to_string(&Sparte::Gas).unwrap(), r#""GAS""#);
    assert_eq!(
        serde_json::to_string(&Sparte::Fernwaerme).unwrap(),
        r#""FERNWAERME""#
    );
    assert_eq!(
        serde_json::to_string(&Sparte::StromUndGas).unwrap(),
        r#""STROM_UND_GAS""#
    );
}

#[test]
fn test_zaehler_default_is_empty() {
    let zaehler = Zaehler::default();
    assert!(zaehler.zaehlernummer.is_none());
    assert!(zaehler.sparte.is_none());
    assert!(zaehler.zaehlertyp.is_none());
    assert!(zaehler.zaehlergroesse.is_none());
    assert!(zaehler.standort.is_none());
    assert!(zaehler.zaehlwerke.is_empty());
    assert!(zaehler.geraeteeigenschaften.is_empty());
    assert!(zaehler.marktlokations_id.is_none());
    assert!(zaehler.messlokations_id.is_none());
    assert!(zaehler.hersteller.is_none());
    assert!(zaehler.herstellungsjahr.is_none());
}

#[test]
fn test_zaehler_with_all_string_fields() {
    let zaehler = Zaehler {
        zaehlernummer: Some("Z001".to_string()),
        marktlokations_id: Some("MALO-001".to_string()),
        messlokations_id: Some("MELO-001".to_string()),
        eigentumsverhaeltnis: Some("Eigentum".to_string()),
        hersteller: Some("Siemens".to_string()),
        herstellungsjahr: Some(2023),
        ..Default::default()
    };

    let json = serde_json::to_string(&zaehler).unwrap();
    let parsed: Zaehler = serde_json::from_str(&json).unwrap();
    assert_eq!(zaehler, parsed);
}

#[test]
fn test_adresse_with_all_fields() {
    let adresse = Adresse {
        strasse: Some("Hauptweg".to_string()),
        hausnummer: Some("10a".to_string()),
        postleitzahl: Some("80331".to_string()),
        ort: Some("M\u{00fc}nchen".to_string()),
        ortsteil: Some("Altstadt".to_string()),
        postfach: Some("PF 1234".to_string()),
        adresszusatz: Some("Hinterhaus".to_string()),
        co_ergaenzung: Some("c/o Meier".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&adresse).unwrap();
    let parsed: Adresse = serde_json::from_str(&json).unwrap();
    assert_eq!(adresse, parsed);
}
