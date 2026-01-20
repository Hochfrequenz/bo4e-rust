//! Example: Creating and serializing a meter.
//!
//! Run with: `cargo run --example meter_roundtrip`

use bo4e::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a smart meter
    let meter = Meter {
        meta: Bo4eMeta::with_type("Zaehler").version("202401.0.1"),
        meter_number: Some("1EMH0012345678".to_string()),
        division: Some(Division::Electricity),
        meter_type: Some(MeterType::IntelligentMeasuringSystem),
        registers: vec![
            MeterRegister {
                obis_code: Some("1-0:1.8.0".to_string()),
                energy_direction: Some(EnergyDirection::FeedOut),
                unit: Some(Unit::KilowattHour),
                description: Some("Bezug".to_string()),
                ..Default::default()
            },
            MeterRegister {
                obis_code: Some("1-0:2.8.0".to_string()),
                energy_direction: Some(EnergyDirection::FeedIn),
                unit: Some(Unit::KilowattHour),
                description: Some("Einspeisung".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Serialize to German JSON
    let json = to_json_german(&meter)?;
    println!("German JSON:\n{}\n", json);

    // Serialize to English JSON
    let english = to_json_english(&meter)?;
    println!("English JSON:\n{}\n", english);

    // Parse back
    let mut bytes = json.into_bytes();
    let parsed: Meter = from_json(&mut bytes)?;

    assert_eq!(parsed.meter_number, Some("1EMH0012345678".to_string()));
    println!("Roundtrip successful!");

    Ok(())
}
