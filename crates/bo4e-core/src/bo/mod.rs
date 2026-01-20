//! Business Objects (BOs) - top-level entities in BO4E.
//!
//! # Epic 4.1: Locations & Technical Business Objects
//!
//! This module contains location and technical resource business objects:
//!
//! - [`Meter`] - A metering device for energy measurement
//! - [`MarketLocation`] - Market location (MaLo) - point of energy delivery/receipt
//! - [`MeteringLocation`] - Metering location (MeLo) - where measurement takes place
//! - [`NetworkLocation`] - Network location in the grid infrastructure
//! - [`LocationAssignment`] - Assignment between different location types
//! - [`Device`] - A technical device in the energy infrastructure
//! - [`TechnicalResource`] - Technical resource (generation, consumption, storage)
//! - [`ControllableResource`] - Resource controllable for demand response
//! - [`EnergyAmount`] - Amount of energy with time series data
//! - [`LoadProfile`] - Load profile (time series of power data)
//! - [`TimeSeries`] - Generic time series of data values
//! - [`LocationProperties`] - Properties of a physical location
//!
//! # Epic 4.2: Parties & Contracts Business Objects
//!
//! This module contains business partner, contract, and offer business objects:
//!
//! - [`BusinessPartner`] - A business partner (company or organization)
//! - [`Person`] - A natural person
//! - [`MarketParticipant`] - A market participant in the energy market
//! - [`Contract`] - A contract between parties
//! - [`BundleContract`] - A bundle contract combining multiple contracts
//! - [`Offer`] - An offer/quote for energy supply or services
//! - [`Tender`] - A tender/RFP for energy supply
//! - [`Balancing`] - Balance group data
//! - [`Region`] - A geographical region
//! - [`RegionalTariff`] - A regional tariff definition

// Epic 4.1: Locations & Technical Business Objects
mod controllable_resource;
mod device;
mod energy_amount;
mod load_profile;
mod location_assignment;
mod location_properties;
mod market_location;
mod meter;
mod metering_location;
mod network_location;
mod technical_resource;
mod time_series;

// Epic 4.2: Parties & Contracts Business Objects
mod balancing;
mod bundle_contract;
mod business_partner;
mod contract;
mod market_participant;
mod offer;
mod person;
mod region;
mod regional_tariff;
mod tender;

// Epic 4.1 exports
pub use controllable_resource::ControllableResource;
pub use device::Device;
pub use energy_amount::EnergyAmount;
pub use load_profile::LoadProfile;
pub use location_assignment::LocationAssignment;
pub use location_properties::LocationProperties;
pub use market_location::MarketLocation;
pub use meter::Meter;
pub use metering_location::MeteringLocation;
pub use network_location::NetworkLocation;
pub use technical_resource::TechnicalResource;
pub use time_series::TimeSeries;

// Epic 4.2 exports
pub use balancing::Balancing;
pub use bundle_contract::BundleContract;
pub use business_partner::BusinessPartner;
pub use contract::Contract;
pub use market_participant::MarketParticipant;
pub use offer::Offer;
pub use person::Person;
pub use region::Region;
pub use regional_tariff::RegionalTariff;
pub use tender::Tender;
