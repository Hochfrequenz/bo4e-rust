//! Schema generator for drift detection.
//!
//! Outputs JSON schemas for all BO4E types.
//!
//! Run with: cargo run --bin generate_schema --features json-schema

fn main() {
    #[cfg(feature = "json-schema")]
    {
        use bo4e_core::com::*;
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

        // Build component schemas map
        let mut com_schemas = Map::new();

        // Address & Contact components
        com_schemas.insert("Adresse".into(), json!(schema_for!(Address)));
        com_schemas.insert(
            "Katasteradresse".into(),
            json!(schema_for!(CadastralAddress)),
        );
        com_schemas.insert("Geokoordinaten".into(), json!(schema_for!(GeoCoordinates)));
        com_schemas.insert("Kontaktweg".into(), json!(schema_for!(ContactMethod)));
        com_schemas.insert("Unterschrift".into(), json!(schema_for!(Signature)));
        com_schemas.insert(
            "ExterneReferenz".into(),
            json!(schema_for!(ExternalReference)),
        );

        // Pricing components
        com_schemas.insert("Preis".into(), json!(schema_for!(Price)));
        com_schemas.insert("Preisstufe".into(), json!(schema_for!(PriceTier)));
        com_schemas.insert("Preisposition".into(), json!(schema_for!(PricePosition)));
        com_schemas.insert("Preisgarantie".into(), json!(schema_for!(PriceGuarantee)));
        com_schemas.insert(
            "RegionalePreisstufe".into(),
            json!(schema_for!(RegionalPriceTier)),
        );
        com_schemas.insert("Margenpreis".into(), json!(schema_for!(MarginPrice)));
        com_schemas.insert("Aufschlag".into(), json!(schema_for!(Surcharge)));
        com_schemas.insert(
            "PositionsAufschlag".into(),
            json!(schema_for!(PositionSurcharge)),
        );
        com_schemas.insert(
            "RegionalerAufschlag".into(),
            json!(schema_for!(RegionalSurcharge)),
        );
        com_schemas.insert(
            "AufschlagProOrt".into(),
            json!(schema_for!(SurchargePerLocation)),
        );
        com_schemas.insert("Abgabe".into(), json!(schema_for!(Levy)));
        com_schemas.insert("Netzentgelt".into(), json!(schema_for!(NetworkCharge)));

        // Tariff components
        com_schemas.insert("Tarifpreis".into(), json!(schema_for!(TariffPrice)));
        com_schemas.insert(
            "Tarifpreisposition".into(),
            json!(schema_for!(TariffPricePosition)),
        );
        com_schemas.insert(
            "Tarifberechnungsparameter".into(),
            json!(schema_for!(TariffCalculationParameter)),
        );
        com_schemas.insert(
            "Tarifeinschraenkung".into(),
            json!(schema_for!(TariffRestriction)),
        );
        com_schemas.insert("Saisontarif".into(), json!(schema_for!(SeasonalTariff)));
        com_schemas.insert(
            "Zaehlzeitregister".into(),
            json!(schema_for!(TimeOfUseRegister)),
        );
        com_schemas.insert("Energiemix".into(), json!(schema_for!(EnergyMix)));
        com_schemas.insert("Energieherkunft".into(), json!(schema_for!(EnergySource)));

        // Cost & Invoice components
        com_schemas.insert("Kostenblock".into(), json!(schema_for!(CostBlock)));
        com_schemas.insert("Kostenposition".into(), json!(schema_for!(CostPosition)));
        com_schemas.insert(
            "Fremdkostenblock".into(),
            json!(schema_for!(ExternalCostBlock)),
        );
        com_schemas.insert(
            "Fremdkostenposition".into(),
            json!(schema_for!(ExternalCostPosition)),
        );
        com_schemas.insert("Steuerbetrag".into(), json!(schema_for!(TaxAmount)));
        com_schemas.insert(
            "Konzessionsabgabe".into(),
            json!(schema_for!(ConcessionFee)),
        );
        com_schemas.insert(
            "Rechnungsposition".into(),
            json!(schema_for!(InvoicePosition)),
        );
        com_schemas.insert(
            "Abrechnungsperiodendaten".into(),
            json!(schema_for!(BillingPeriodData)),
        );
        com_schemas.insert("Bonus".into(), json!(schema_for!(Bonus)));
        com_schemas.insert("Rabatt".into(), json!(schema_for!(Discount)));

        // Measurement components
        com_schemas.insert("Menge".into(), json!(schema_for!(Quantity)));
        com_schemas.insert("Betrag".into(), json!(schema_for!(Amount)));
        com_schemas.insert("Messwert".into(), json!(schema_for!(MeasuredValue)));
        com_schemas.insert("Zaehlwerksstand".into(), json!(schema_for!(MeterReading)));
        com_schemas.insert("Zaehlwerk".into(), json!(schema_for!(MeterRegister)));
        com_schemas.insert(
            "Messstellenstatus".into(),
            json!(schema_for!(MeteringPointStatus)),
        );
        com_schemas.insert("Verbrauch".into(), json!(schema_for!(Consumption)));
        com_schemas.insert(
            "VerbrauchteQuantitaet".into(),
            json!(schema_for!(ConsumedQuantity)),
        );
        com_schemas.insert("Aggregiertwert".into(), json!(schema_for!(AggregatedValue)));
        com_schemas.insert("Ersatzwert".into(), json!(schema_for!(SubstitutionValue)));

        // Time & Profile components
        com_schemas.insert("Zeitraum".into(), json!(schema_for!(TimePeriod)));
        com_schemas.insert("Datumsbereich".into(), json!(schema_for!(DateRange)));
        com_schemas.insert("Intervall".into(), json!(schema_for!(Interval)));
        com_schemas.insert("Lastkurvendaten".into(), json!(schema_for!(LoadCurveData)));
        com_schemas.insert(
            "Lastprofilwert".into(),
            json!(schema_for!(LoadProfileValue)),
        );
        com_schemas.insert("Profildaten".into(), json!(schema_for!(ProfileData)));
        com_schemas.insert("Zeitreihenwert".into(), json!(schema_for!(TimeSeriesValue)));
        com_schemas.insert(
            "Qualitaetsindikator".into(),
            json!(schema_for!(QualityIndicator)),
        );

        // Contract & Offer components
        com_schemas.insert(
            "Vertragskonditionen".into(),
            json!(schema_for!(ContractConditions)),
        );
        com_schemas.insert("Vertragsteil".into(), json!(schema_for!(ContractPart)));
        com_schemas.insert("Angebotsteil".into(), json!(schema_for!(OfferPart)));
        com_schemas.insert("Angebotsposition".into(), json!(schema_for!(OfferPosition)));
        com_schemas.insert("Angebotsvariante".into(), json!(schema_for!(OfferVariant)));
        com_schemas.insert(
            "Regionskriterium".into(),
            json!(schema_for!(RegionCriterion)),
        );
        com_schemas.insert("Zustaendigkeit".into(), json!(schema_for!(Responsibility)));
        com_schemas.insert(
            "Dienstleistungspreis".into(),
            json!(schema_for!(ServicePrice)),
        );
        com_schemas.insert("Hardware".into(), json!(schema_for!(Hardware)));
        com_schemas.insert(
            "Validierungsergebnis".into(),
            json!(schema_for!(ValidationResult)),
        );

        let schemas = json!({
            "bo": {},
            "com": Value::Object(com_schemas),
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
