//! Components (COMs) - composite types used within Business Objects.
//!
//! Components are reusable data structures that appear within multiple
//! Business Objects. Unlike BOs, they cannot exist independently.
//!
//! # Epic 3.1: Address & Contact Components
//!
//! This module contains address, contact, and person-related components:
//!
//! - [`Address`] - Physical or postal address
//! - [`GeoCoordinates`] - Geographic coordinates (latitude/longitude)
//! - [`CadastralAddress`] - Cadastral/land registry address
//! - [`ContactMethod`] - Contact method (email, phone, etc.)
//! - [`Signature`] - Digital signature for contracts/offers
//! - [`ExternalReference`] - Reference to external systems
//! - [`Responsibility`] - Area of responsibility for a contact
//! - [`Hardware`] - Hardware component information
//! - [`OfferPosition`] - Position within an offer
//! - [`OfferPart`] - Part of an offer variant
//! - [`OfferVariant`] - Variant of an offer
//! - [`ContractPart`] - Part of a contract
//! - [`ContractConditions`] - Contract conditions
//! - [`InvoicePosition`] - Position within an invoice
//! - [`RegionCriterion`] - Regional criterion for delimitation
//!
//! # Epic 3.2: Pricing & Cost Components
//!
//! Pricing, cost, and tariff-related components:
//!
//! - [`Price`] - A price with value and unit
//! - [`Amount`] - Monetary amount
//! - [`PriceTier`] - Price tier/bracket
//! - [`PricePosition`] - Position in a price sheet
//! - [`TariffPrice`] - Tariff price
//! - [`TariffPricePosition`] - Tariff price position
//! - [`RegionalPriceTier`] - Regional price tier
//! - [`Surcharge`] - Surcharge or discount
//! - [`SurchargePerLocation`] - Surcharge per location
//! - [`RegionalSurcharge`] - Regional surcharge
//! - [`PositionSurcharge`] - Position-specific surcharge
//! - [`TaxAmount`] - Tax amount
//! - [`CostBlock`] - Block of costs
//! - [`CostPosition`] - Cost position
//! - [`ExternalCostBlock`] - External cost block
//! - [`ExternalCostPosition`] - External cost position
//! - [`TariffCalculationParameter`] - Tariff calculation parameters
//! - [`TariffRestriction`] - Tariff restriction
//! - [`PriceGuarantee`] - Price guarantee
//! - [`EnergySource`] - Energy source/origin
//! - [`EnergyMix`] - Energy mix composition
//! - [`Consumption`] - Consumption data
//! - [`ConsumedQuantity`] - Consumed quantity
//! - [`ConcessionFee`] - Concession fee
//! - [`NetworkCharge`] - Network charge
//! - [`Levy`] - Levy (EEG, KWK, etc.)
//! - [`Bonus`] - Bonus/incentive
//! - [`Discount`] - Discount
//! - [`MarginPrice`] - Margin price
//! - [`ServicePrice`] - Service price
//!
//! # Epic 3.3: Measurement & Time Components
//!
//! Measurement, quantity, and time-related components:
//!
//! - [`TimePeriod`] - A time period with start and end timestamps
//! - [`MeasuredValue`] - A measured value at a specific timestamp
//! - [`Quantity`] - A quantity with value and unit
//! - [`MeterRegister`] - A register on a meter
//! - [`TimeOfUseRegister`] - Time-of-use register
//! - [`MeterReading`] - Meter reading at a point in time
//! - [`LoadProfileValue`] - Load profile value
//! - [`TimeSeriesValue`] - Time series value
//! - [`Interval`] - Time interval with duration and unit
//! - [`DateRange`] - Date range with start and end dates
//! - [`SeasonalTariff`] - Seasonal tariff period
//! - [`BillingPeriodData`] - Billing period data
//! - [`MeteringPointStatus`] - Metering point status
//! - [`ValidationResult`] - Validation result
//! - [`QualityIndicator`] - Quality indicator for measured data
//! - [`SubstitutionValue`] - Substituted/replacement value
//! - [`AggregatedValue`] - Aggregated value
//! - [`ProfileData`] - Profile data (standard load profiles)
//! - [`LoadCurveData`] - Load curve data

// Epic 3.1: Address & Contact Components
mod address;
mod cadastral_address;
mod contact_method;
mod contract_conditions;
mod contract_part;
mod external_reference;
mod geo_coordinates;
mod hardware;
mod invoice_position;
mod offer_part;
mod offer_position;
mod offer_variant;
mod region_criterion;
mod responsibility;
mod signature;

// Epic 3.2: Pricing & Cost Components
mod amount;
mod bonus;
mod concession_fee;
mod consumed_quantity;
mod consumption;
mod cost_block;
mod cost_position;
mod discount;
mod energy_mix;
mod energy_source;
mod external_cost_block;
mod external_cost_position;
mod levy;
mod margin_price;
mod network_charge;
mod position_surcharge;
mod price;
mod price_guarantee;
mod price_position;
mod price_tier;
mod regional_price_tier;
mod regional_surcharge;
mod service_price;
mod surcharge;
mod surcharge_per_location;
mod tariff_calculation_parameter;
mod tariff_price;
mod tariff_price_position;
mod tariff_restriction;
mod tax_amount;

// Epic 3.3: Measurement & Time Components
mod aggregated_value;
mod billing_period_data;
mod date_range;
mod interval;
mod load_curve_data;
mod load_profile_value;
mod measured_value;
mod meter_reading;
mod meter_register;
mod metering_point_status;
mod profile_data;
mod quality_indicator;
mod quantity;
mod seasonal_tariff;
mod substitution_value;
mod time_of_use_register;
mod time_period;
mod time_series_value;
mod validation_result;

// Epic 3.1 exports
pub use address::Address;
pub use cadastral_address::CadastralAddress;
pub use contact_method::ContactMethod;
pub use contract_conditions::ContractConditions;
pub use contract_part::ContractPart;
pub use external_reference::ExternalReference;
pub use geo_coordinates::GeoCoordinates;
pub use hardware::Hardware;
pub use invoice_position::InvoicePosition;
pub use offer_part::OfferPart;
pub use offer_position::OfferPosition;
pub use offer_variant::OfferVariant;
pub use region_criterion::RegionCriterion;
pub use responsibility::Responsibility;
pub use signature::Signature;

// Epic 3.2 exports
pub use amount::Amount;
pub use bonus::Bonus;
pub use concession_fee::ConcessionFee;
pub use consumed_quantity::ConsumedQuantity;
pub use consumption::Consumption;
pub use cost_block::CostBlock;
pub use cost_position::CostPosition;
pub use discount::Discount;
pub use energy_mix::EnergyMix;
pub use energy_source::EnergySource;
pub use external_cost_block::ExternalCostBlock;
pub use external_cost_position::ExternalCostPosition;
pub use levy::Levy;
pub use margin_price::MarginPrice;
pub use network_charge::NetworkCharge;
pub use position_surcharge::PositionSurcharge;
pub use price::Price;
pub use price_guarantee::PriceGuarantee;
pub use price_position::PricePosition;
pub use price_tier::PriceTier;
pub use regional_price_tier::RegionalPriceTier;
pub use regional_surcharge::RegionalSurcharge;
pub use service_price::ServicePrice;
pub use surcharge::Surcharge;
pub use surcharge_per_location::SurchargePerLocation;
pub use tariff_calculation_parameter::TariffCalculationParameter;
pub use tariff_price::TariffPrice;
pub use tariff_price_position::TariffPricePosition;
pub use tariff_restriction::TariffRestriction;
pub use tax_amount::TaxAmount;

// Epic 3.3 exports
pub use aggregated_value::AggregatedValue;
pub use billing_period_data::BillingPeriodData;
pub use date_range::DateRange;
pub use interval::Interval;
pub use load_curve_data::LoadCurveData;
pub use load_profile_value::LoadProfileValue;
pub use measured_value::MeasuredValue;
pub use meter_reading::MeterReading;
pub use meter_register::MeterRegister;
pub use metering_point_status::MeteringPointStatus;
pub use profile_data::ProfileData;
pub use quality_indicator::QualityIndicator;
pub use quantity::Quantity;
pub use seasonal_tariff::SeasonalTariff;
pub use substitution_value::SubstitutionValue;
pub use time_of_use_register::TimeOfUseRegister;
pub use time_period::TimePeriod;
pub use time_series_value::TimeSeriesValue;
pub use validation_result::ValidationResult;
