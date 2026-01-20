//! Enumerations for BO4E type-safe values.
//!
//! This module contains all the enum types used in BO4E, organized by category.

// Type discriminators
mod bo_type;
mod com_type;

pub use bo_type::BoType;
pub use com_type::ComType;

// Energy sector and direction
mod division;
mod energy_direction;
mod generation_type;
mod medium;

pub use division::Division;
pub use energy_direction::EnergyDirection;
pub use generation_type::GenerationType;
pub use medium::Medium;

// Measurement related
mod measured_quantity;
mod measured_value_status;
mod measurement_price_type;
mod measurement_type;

pub use measured_quantity::MeasuredQuantity;
pub use measured_value_status::MeasuredValueStatus;
pub use measurement_price_type::MeasurementPriceType;
pub use measurement_type::MeasurementType;

// Meter related
mod meter_category;
mod meter_size;
mod meter_type;
mod reading_type;

pub use meter_category::MeterCategory;
pub use meter_size::MeterSize;
pub use meter_type::MeterType;
pub use reading_type::ReadingType;

// Network related
mod network_level;
mod voltage_level;

pub use network_level::NetworkLevel;
pub use voltage_level::VoltageLevel;

// Location and usage
mod location_type;
mod usage_type;

pub use location_type::LocationType;
pub use usage_type::UsageType;

// Device related
mod device_category;
mod device_type;

pub use device_category::DeviceCategory;
pub use device_type::DeviceType;

// Technical specifications
mod phase_type;
mod register_type;
mod tariff_type;

pub use phase_type::PhaseType;
pub use register_type::RegisterType;
pub use tariff_type::TariffType;

// Units and measurements
mod currency;
mod time_unit;
mod unit;
mod unit_prefix;

pub use currency::Currency;
pub use time_unit::TimeUnit;
pub use unit::Unit;
pub use unit_prefix::UnitPrefix;

// Calculation and operations
mod arithmetic_operation;
mod calculation_formula;
mod rounding_mode;

pub use arithmetic_operation::ArithmeticOperation;
pub use calculation_formula::CalculationFormula;
pub use rounding_mode::RoundingMode;

// Technical resources
mod controllable_resource_type;
mod technical_resource_usage;

pub use controllable_resource_type::ControllableResourceType;
pub use technical_resource_usage::TechnicalResourceUsage;

// Business partner and market roles
mod business_partner_role;
mod market_role;
mod organization_type;

pub use business_partner_role::BusinessPartnerRole;
pub use market_role::MarketRole;
pub use organization_type::OrganizationType;

// Contact and person related
mod contact_type;
mod salutation;
mod title;

pub use contact_type::ContactType;
pub use salutation::Salutation;
pub use title::Title;

// Contract related
mod contract_form;
mod contract_status;
mod contract_type;

pub use contract_form::ContractForm;
pub use contract_status::ContractStatus;
pub use contract_type::ContractType;

// Customer related
mod customer_group;
mod customer_type;

pub use customer_group::CustomerGroup;
pub use customer_type::CustomerType;

// Invoice and payment related
mod invoice_status;
mod invoice_type;
mod payment_method;

pub use invoice_status::InvoiceStatus;
pub use invoice_type::InvoiceType;
pub use payment_method::PaymentMethod;

// Offer and tender related
mod offer_status;
mod tender_status;
mod tender_type;

pub use offer_status::OfferStatus;
pub use tender_status::TenderStatus;
pub use tender_type::TenderType;

// Service and area related
mod area_type;
mod service_type;

pub use area_type::AreaType;
pub use service_type::ServiceType;

// Geographic
mod country;

pub use country::Country;
