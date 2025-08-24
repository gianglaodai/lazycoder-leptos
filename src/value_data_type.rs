#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueDataType {
    String,
    Int,
    Float,
    Bool,
    Date,
    DateTime,
    Time,
}

impl ValueDataType {
    pub fn to_code(self) -> u8 {
        match self {
            ValueDataType::String => 0,
            ValueDataType::Int => 1,
            ValueDataType::Float => 2,
            ValueDataType::Bool => 3,
            ValueDataType::Date => 4,
            ValueDataType::DateTime => 5,
            ValueDataType::Time => 6,
        }
    }
}
