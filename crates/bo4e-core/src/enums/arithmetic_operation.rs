//! Arithmetic operation (ArithmetischeOperation) enumeration.

use serde::{Deserialize, Serialize};

/// Arithmetic operation type.
///
/// Defines arithmetic operations that can be applied.
///
/// German: ArithmetischeOperation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ArithmeticOperation {
    /// Addition
    #[serde(rename = "ADDITION")]
    Addition,

    /// Subtraction
    #[serde(rename = "SUBTRAKTION")]
    Subtraction,

    /// Multiplication
    #[serde(rename = "MULTIPLIKATION")]
    Multiplication,

    /// Division
    #[serde(rename = "DIVISION")]
    Division,
}

impl ArithmeticOperation {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Addition => "Addition",
            Self::Subtraction => "Subtraktion",
            Self::Multiplication => "Multiplikation",
            Self::Division => "Division",
        }
    }

    /// Returns the mathematical symbol for this operation.
    pub fn symbol(&self) -> char {
        match self {
            Self::Addition => '+',
            Self::Subtraction => '-',
            Self::Multiplication => '*',
            Self::Division => '/',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ArithmeticOperation::Addition).unwrap(),
            r#""ADDITION""#
        );
        assert_eq!(
            serde_json::to_string(&ArithmeticOperation::Division).unwrap(),
            r#""DIVISION""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for op in [
            ArithmeticOperation::Addition,
            ArithmeticOperation::Subtraction,
            ArithmeticOperation::Multiplication,
            ArithmeticOperation::Division,
        ] {
            let json = serde_json::to_string(&op).unwrap();
            let parsed: ArithmeticOperation = serde_json::from_str(&json).unwrap();
            assert_eq!(op, parsed);
        }
    }

    #[test]
    fn test_symbol() {
        assert_eq!(ArithmeticOperation::Addition.symbol(), '+');
        assert_eq!(ArithmeticOperation::Subtraction.symbol(), '-');
        assert_eq!(ArithmeticOperation::Multiplication.symbol(), '*');
        assert_eq!(ArithmeticOperation::Division.symbol(), '/');
    }
}
