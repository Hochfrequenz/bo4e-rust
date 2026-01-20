//! Schema generator for drift detection.
//!
//! Outputs JSON schemas for all BO4E types.
//!
//! Run with: cargo run --bin generate_schema --features json-schema

fn main() {
    #[cfg(feature = "json-schema")]
    {
        use bo4e_core::enums::*;
        use schemars::schema_for;
        use serde_json::{json, Map, Value};

        // Build enum schemas map incrementally to avoid recursion limit
        let mut enum_schemas = Map::new();

        // Type discriminators
        enum_schemas.insert("BoType".into(), json!(schema_for!(BoType)));
        enum_schemas.insert("ComType".into(), json!(schema_for!(ComType)));

        // Energy sector and direction
        enum_schemas.insert("Division".into(), json!(schema_for!(Division)));
        enum_schemas.insert(
            "EnergyDirection".into(),
            json!(schema_for!(EnergyDirection)),
        );
        enum_schemas.insert("GenerationType".into(), json!(schema_for!(GenerationType)));
        enum_schemas.insert("Medium".into(), json!(schema_for!(Medium)));

        // Measurement related
        enum_schemas.insert(
            "MeasuredQuantity".into(),
            json!(schema_for!(MeasuredQuantity)),
        );
        enum_schemas.insert(
            "MeasuredValueStatus".into(),
            json!(schema_for!(MeasuredValueStatus)),
        );
        enum_schemas.insert(
            "MeasurementPriceType".into(),
            json!(schema_for!(MeasurementPriceType)),
        );
        enum_schemas.insert(
            "MeasurementType".into(),
            json!(schema_for!(MeasurementType)),
        );

        // Meter related
        enum_schemas.insert("MeterCategory".into(), json!(schema_for!(MeterCategory)));
        enum_schemas.insert("MeterSize".into(), json!(schema_for!(MeterSize)));
        enum_schemas.insert("MeterType".into(), json!(schema_for!(MeterType)));
        enum_schemas.insert("ReadingType".into(), json!(schema_for!(ReadingType)));

        // Network related
        enum_schemas.insert("NetworkLevel".into(), json!(schema_for!(NetworkLevel)));
        enum_schemas.insert("VoltageLevel".into(), json!(schema_for!(VoltageLevel)));

        // Location and usage
        enum_schemas.insert("LocationType".into(), json!(schema_for!(LocationType)));
        enum_schemas.insert("UsageType".into(), json!(schema_for!(UsageType)));

        // Device related
        enum_schemas.insert("DeviceCategory".into(), json!(schema_for!(DeviceCategory)));
        enum_schemas.insert("DeviceType".into(), json!(schema_for!(DeviceType)));

        // Technical specifications
        enum_schemas.insert("PhaseType".into(), json!(schema_for!(PhaseType)));
        enum_schemas.insert("RegisterType".into(), json!(schema_for!(RegisterType)));
        enum_schemas.insert("TariffType".into(), json!(schema_for!(TariffType)));

        // Units and measurements
        enum_schemas.insert("Currency".into(), json!(schema_for!(Currency)));
        enum_schemas.insert("TimeUnit".into(), json!(schema_for!(TimeUnit)));
        enum_schemas.insert("Unit".into(), json!(schema_for!(Unit)));
        enum_schemas.insert("UnitPrefix".into(), json!(schema_for!(UnitPrefix)));

        // Calculation and operations
        enum_schemas.insert(
            "ArithmeticOperation".into(),
            json!(schema_for!(ArithmeticOperation)),
        );
        enum_schemas.insert(
            "CalculationFormula".into(),
            json!(schema_for!(CalculationFormula)),
        );
        enum_schemas.insert("RoundingMode".into(), json!(schema_for!(RoundingMode)));

        // Technical resources
        enum_schemas.insert(
            "ControllableResourceType".into(),
            json!(schema_for!(ControllableResourceType)),
        );
        enum_schemas.insert(
            "TechnicalResourceUsage".into(),
            json!(schema_for!(TechnicalResourceUsage)),
        );

        // Business partner and market roles
        enum_schemas.insert(
            "BusinessPartnerRole".into(),
            json!(schema_for!(BusinessPartnerRole)),
        );
        enum_schemas.insert("MarketRole".into(), json!(schema_for!(MarketRole)));
        enum_schemas.insert(
            "OrganizationType".into(),
            json!(schema_for!(OrganizationType)),
        );

        // Contact and person related
        enum_schemas.insert("ContactType".into(), json!(schema_for!(ContactType)));
        enum_schemas.insert("Salutation".into(), json!(schema_for!(Salutation)));
        enum_schemas.insert("Title".into(), json!(schema_for!(Title)));

        // Contract related
        enum_schemas.insert("ContractForm".into(), json!(schema_for!(ContractForm)));
        enum_schemas.insert("ContractStatus".into(), json!(schema_for!(ContractStatus)));
        enum_schemas.insert("ContractType".into(), json!(schema_for!(ContractType)));

        // Customer related
        enum_schemas.insert("CustomerGroup".into(), json!(schema_for!(CustomerGroup)));
        enum_schemas.insert("CustomerType".into(), json!(schema_for!(CustomerType)));

        // Invoice and payment related
        enum_schemas.insert("InvoiceStatus".into(), json!(schema_for!(InvoiceStatus)));
        enum_schemas.insert("InvoiceType".into(), json!(schema_for!(InvoiceType)));
        enum_schemas.insert("PaymentMethod".into(), json!(schema_for!(PaymentMethod)));

        // Offer and tender related
        enum_schemas.insert("OfferStatus".into(), json!(schema_for!(OfferStatus)));
        enum_schemas.insert("TenderStatus".into(), json!(schema_for!(TenderStatus)));
        enum_schemas.insert("TenderType".into(), json!(schema_for!(TenderType)));

        // Service and area related
        enum_schemas.insert("AreaType".into(), json!(schema_for!(AreaType)));
        enum_schemas.insert("ServiceType".into(), json!(schema_for!(ServiceType)));

        // Geographic
        enum_schemas.insert("Country".into(), json!(schema_for!(Country)));

        // Pricing related
        enum_schemas.insert(
            "CalculationMethod".into(),
            json!(schema_for!(CalculationMethod)),
        );
        enum_schemas.insert(
            "PriceGuaranteeType".into(),
            json!(schema_for!(PriceGuaranteeType)),
        );
        enum_schemas.insert("PriceModel".into(), json!(schema_for!(PriceModel)));
        enum_schemas.insert("PriceStatus".into(), json!(schema_for!(PriceStatus)));
        enum_schemas.insert("PriceType".into(), json!(schema_for!(PriceType)));
        enum_schemas.insert(
            "SurchargeTarget".into(),
            json!(schema_for!(SurchargeTarget)),
        );
        enum_schemas.insert("SurchargeType".into(), json!(schema_for!(SurchargeType)));
        enum_schemas.insert("TaxType".into(), json!(schema_for!(TaxType)));

        // Cost related
        enum_schemas.insert("CostClass".into(), json!(schema_for!(CostClass)));

        // Tariff related
        enum_schemas.insert(
            "TariffCalculationMethod".into(),
            json!(schema_for!(TariffCalculationMethod)),
        );
        enum_schemas.insert("TariffFeature".into(), json!(schema_for!(TariffFeature)));
        enum_schemas.insert(
            "TariffRegionCriterion".into(),
            json!(schema_for!(TariffRegionCriterion)),
        );
        enum_schemas.insert("TariffTime".into(), json!(schema_for!(TariffTime)));

        // Regional related
        enum_schemas.insert(
            "RegionCriterionType".into(),
            json!(schema_for!(RegionCriterionType)),
        );
        enum_schemas.insert("RegionType".into(), json!(schema_for!(RegionType)));

        // Concession fee related
        enum_schemas.insert(
            "ConcessionFeeCustomerGroup".into(),
            json!(schema_for!(ConcessionFeeCustomerGroup)),
        );
        enum_schemas.insert(
            "ConcessionFeeType".into(),
            json!(schema_for!(ConcessionFeeType)),
        );

        // Eco/certificate related
        enum_schemas.insert("EcoCertificate".into(), json!(schema_for!(EcoCertificate)));
        enum_schemas.insert("EcoLabel".into(), json!(schema_for!(EcoLabel)));

        // Subject area and validity
        enum_schemas.insert("SubjectArea".into(), json!(schema_for!(SubjectArea)));
        enum_schemas.insert("ValidityType".into(), json!(schema_for!(ValidityType)));

        let schemas = json!({
            "bo": {},
            "com": {},
            "enum": Value::Object(enum_schemas)
        });

        println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
    }

    #[cfg(not(feature = "json-schema"))]
    {
        eprintln!("Error: This binary requires the 'json-schema' feature.");
        eprintln!("Run with: cargo run --bin generate_schema --features json-schema");
        std::process::exit(1);
    }
}
