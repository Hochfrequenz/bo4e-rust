use bo4e_core::bo::Meter;
use bo4e_core::com::Address;
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
fn test_meter_zaehler_roundtrip_conversion() {
    let original = Meter {
        meter_number: Some("ROUND".to_string()),
        division: Some(Division::Electricity),
        manufacturer: Some("TestCorp".to_string()),
        manufacturing_year: Some(2024),
        ..Default::default()
    };

    let zaehler: Zaehler = original.clone().into();
    let back: Meter = zaehler.into();
    assert_eq!(original, back);
}

#[test]
fn test_zaehler_meter_roundtrip_conversion() {
    let original = Zaehler {
        zaehlernummer: Some("RUND".to_string()),
        sparte: Some(Sparte::Fernwaerme),
        hersteller: Some("Fabrikant".to_string()),
        herstellungsjahr: Some(2023),
        ..Default::default()
    };

    let meter: Meter = original.clone().into();
    let back: Zaehler = meter.into();
    assert_eq!(original, back);
}

#[test]
fn test_division_to_sparte_conversion() {
    assert_eq!(Sparte::from(Division::Electricity), Sparte::Strom);
    assert_eq!(Sparte::from(Division::Gas), Sparte::Gas);
    assert_eq!(Sparte::from(Division::DistrictHeating), Sparte::Fernwaerme);
    assert_eq!(Sparte::from(Division::LocalHeating), Sparte::Nahwaerme);
    assert_eq!(Sparte::from(Division::Water), Sparte::Wasser);
    assert_eq!(Sparte::from(Division::Wastewater), Sparte::Abwasser);
    assert_eq!(
        Sparte::from(Division::ElectricityAndGas),
        Sparte::StromUndGas
    );
}

#[test]
fn test_sparte_to_division_conversion() {
    assert_eq!(Division::from(Sparte::Strom), Division::Electricity);
    assert_eq!(Division::from(Sparte::Gas), Division::Gas);
    assert_eq!(
        Division::from(Sparte::Fernwaerme),
        Division::DistrictHeating
    );
    assert_eq!(Division::from(Sparte::Nahwaerme), Division::LocalHeating);
    assert_eq!(Division::from(Sparte::Wasser), Division::Water);
    assert_eq!(Division::from(Sparte::Abwasser), Division::Wastewater);
    assert_eq!(
        Division::from(Sparte::StromUndGas),
        Division::ElectricityAndGas
    );
}

#[test]
fn test_sparte_division_roundtrip() {
    for division in [
        Division::Electricity,
        Division::Gas,
        Division::DistrictHeating,
        Division::LocalHeating,
        Division::Water,
        Division::Wastewater,
        Division::ElectricityAndGas,
    ] {
        let sparte = Sparte::from(division);
        let back = Division::from(sparte);
        assert_eq!(division, back);
    }
}

#[test]
fn test_address_to_adresse() {
    let address = Address {
        street: Some("Hauptstra\u{00df}e".to_string()),
        house_number: Some("1".to_string()),
        postal_code: Some("10115".to_string()),
        city: Some("Berlin".to_string()),
        ..Default::default()
    };

    let adresse: Adresse = address.into();
    assert_eq!(adresse.strasse, Some("Hauptstra\u{00df}e".to_string()));
    assert_eq!(adresse.hausnummer, Some("1".to_string()));
    assert_eq!(adresse.postleitzahl, Some("10115".to_string()));
    assert_eq!(adresse.ort, Some("Berlin".to_string()));
}

#[test]
fn test_adresse_to_address() {
    let adresse = Adresse {
        strasse: Some("Marktplatz".to_string()),
        hausnummer: Some("5".to_string()),
        postleitzahl: Some("50667".to_string()),
        ort: Some("K\u{00f6}ln".to_string()),
        ..Default::default()
    };

    let address: Address = adresse.into();
    assert_eq!(address.street, Some("Marktplatz".to_string()));
    assert_eq!(address.house_number, Some("5".to_string()));
    assert_eq!(address.postal_code, Some("50667".to_string()));
    assert_eq!(address.city, Some("K\u{00f6}ln".to_string()));
}

#[test]
fn test_address_adresse_roundtrip() {
    let original = Address {
        street: Some("Testweg".to_string()),
        house_number: Some("42a".to_string()),
        postal_code: Some("80331".to_string()),
        city: Some("M\u{00fc}nchen".to_string()),
        district: Some("Altstadt".to_string()),
        po_box: Some("PF 999".to_string()),
        address_addition: Some("Etage 3".to_string()),
        co_ergaenzung: Some("c/o Schmidt".to_string()),
        ..Default::default()
    };

    let adresse: Adresse = original.clone().into();
    let back: Address = adresse.into();
    assert_eq!(original, back);
}

#[test]
fn test_default_meter_to_zaehler() {
    let meter = Meter::default();
    let zaehler: Zaehler = meter.into();

    assert!(zaehler.zaehlernummer.is_none());
    assert!(zaehler.sparte.is_none());
    assert!(zaehler.zaehlertyp.is_none());
    assert!(zaehler.zaehlwerke.is_empty());
}

#[test]
fn test_default_zaehler_to_meter() {
    let zaehler = Zaehler::default();
    let meter: Meter = zaehler.into();

    assert!(meter.meter_number.is_none());
    assert!(meter.division.is_none());
    assert!(meter.meter_type.is_none());
    assert!(meter.registers.is_empty());
}
