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
