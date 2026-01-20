//! Example: Working with market locations.
//!
//! Run with: `cargo run --example market_location`

use bo4e::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a market location for electricity consumption
    let malo = MarketLocation {
        meta: Bo4eMeta::with_type("Marktlokation"),
        market_location_id: Some("12345678901".to_string()),
        division: Some(Division::Electricity),
        energy_direction: Some(EnergyDirection::FeedOut),
        customer_type: Some(CustomerType::Private),
        address: Some(Address {
            street: Some("Musterstraße".to_string()),
            house_number: Some("42".to_string()),
            postal_code: Some("50667".to_string()),
            city: Some("Köln".to_string()),
            ..Default::default()
        }),
        annual_consumption: Some(3500.0), // kWh
        ..Default::default()
    };

    let json = to_json_german(&malo)?;
    println!("Market Location JSON:\n{}", json);

    // Parse back
    let mut bytes = json.into_bytes();
    let parsed: MarketLocation = from_json(&mut bytes)?;

    assert_eq!(parsed.market_location_id, Some("12345678901".to_string()));
    println!("\nRoundtrip successful!");

    Ok(())
}
