#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ArithmetischeOperation {
    #[serde(rename = "ADDITION")]
    Addition,
    #[serde(rename = "SUBTRAKTION")]
    Subtraction,
    #[serde(rename = "MULTIPLIKATION")]
    Multiplication,
    #[serde(rename = "DIVISION")]
    Division,
}
impl From<bo4e_core::enums::ArithmeticOperation> for ArithmetischeOperation {
    fn from(v: bo4e_core::enums::ArithmeticOperation) -> Self {
        match v {
            bo4e_core::enums::ArithmeticOperation::Addition => {
                ArithmetischeOperation::Addition
            }
            bo4e_core::enums::ArithmeticOperation::Subtraction => {
                ArithmetischeOperation::Subtraction
            }
            bo4e_core::enums::ArithmeticOperation::Multiplication => {
                ArithmetischeOperation::Multiplication
            }
            bo4e_core::enums::ArithmeticOperation::Division => {
                ArithmetischeOperation::Division
            }
            _ => panic!("Unknown {} variant", stringify!(ArithmeticOperation)),
        }
    }
}
impl From<ArithmetischeOperation> for bo4e_core::enums::ArithmeticOperation {
    fn from(v: ArithmetischeOperation) -> Self {
        match v {
            ArithmetischeOperation::Addition => {
                bo4e_core::enums::ArithmeticOperation::Addition
            }
            ArithmetischeOperation::Subtraction => {
                bo4e_core::enums::ArithmeticOperation::Subtraction
            }
            ArithmetischeOperation::Multiplication => {
                bo4e_core::enums::ArithmeticOperation::Multiplication
            }
            ArithmetischeOperation::Division => {
                bo4e_core::enums::ArithmeticOperation::Division
            }
            _ => panic!("Unknown {} variant", stringify!(ArithmetischeOperation)),
        }
    }
}
