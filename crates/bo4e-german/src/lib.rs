pub mod generated;

// Re-export all types at crate root for crate::TypeName paths in generated code
pub use generated::bo::*;
pub use generated::com::*;
pub use generated::enums::*;

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::generated::bo::*;
    pub use crate::generated::com::*;
    pub use crate::generated::enums::*;
}
