#[derive(Debug, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    Is,
    In,
    NotIn,
    IsNull,
    NotNull,
    Between,
    NotBetween,
}

#[derive(Debug, PartialEq)]
pub enum Filter {
    Property {
        property_name: String,
        operator: FilterOperator,
        value: FilterValue,
    },
    Attribute {
        attr_name: String,
        operator: FilterOperator,
        value: FilterValue,
    },
    Search {
        value: String,
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FilterValue {
    Int(i32),
    IntRange(i32, i32),
    ListInt(Vec<i32>),
    String(String),
    ListString(Vec<String>),
    Float(f64),
    FloatRange(f64, f64),
    ListFloat(Vec<f64>),
    Bool(bool),
    Date(time::Date),
    DateRange(time::Date, time::Date),
    DateTime(time::OffsetDateTime),
    DateTimeRange(time::OffsetDateTime, time::OffsetDateTime),
    Time(time::Time),
    TimeRange(time::Time, time::Time),
}

impl FilterOperator {
    pub fn is_value_compatible(&self, value: &FilterValue) -> bool {
        match self {
            Self::Equal | Self::NotEqual => matches!(
                value,
                FilterValue::Int(_)
                    | FilterValue::Float(_)
                    | FilterValue::String(_)
                    | FilterValue::Bool(_)
                    | FilterValue::Date(_)
                    | FilterValue::DateTime(_)
                    | FilterValue::Time(_)
            ),
            Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::LessThan
            | Self::LessThanOrEqual => matches!(
                value,
                FilterValue::Int(_)
                    | FilterValue::Float(_)
                    | FilterValue::Time(_)
                    | FilterValue::Date(_)
                    | FilterValue::DateTime(_)
            ),
            Self::Like | Self::NotLike => matches!(value, FilterValue::String(_)),
            Self::Is => matches!(value, FilterValue::Bool(_)),
            Self::In | Self::NotIn => matches!(
                value,
                FilterValue::ListInt(_)
                    | FilterValue::ListFloat(_)
                    | FilterValue::ListString(_)
            ),
            Self::IsNull | Self::NotNull => true,
            Self::Between | Self::NotBetween => matches!(
                value,
                FilterValue::IntRange(_, _)
                    | FilterValue::FloatRange(_, _)
                    | FilterValue::TimeRange(_, _)
                    | FilterValue::DateRange(_, _)
                    | FilterValue::DateTimeRange(_, _)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_operator() {
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Int(42)));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Float(3.14)));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::String("hello".to_string())));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Bool(true)));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Date(time::Date::from_calendar_date(2019,time::Month::January, 1).unwrap())));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::DateTime(time::OffsetDateTime::now_utc())));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Time(time::Time::from_hms(12, 0, 0).unwrap())));

        assert!(!FilterOperator::Equal.is_value_compatible(&FilterValue::IntRange(1, 10)));
        assert!(!FilterOperator::Equal.is_value_compatible(&FilterValue::ListInt(vec![1, 2, 3])));
        assert!(!FilterOperator::Equal.is_value_compatible(&FilterValue::ListString(vec!["a".to_string(), "b".to_string()])));
        assert!(!FilterOperator::Equal.is_value_compatible(&FilterValue::ListFloat(vec![1.0, 2.0, 3.0])));
    }

    #[test]
    fn test_greater_operator() {
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Int(42)));
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Float(3.14)));
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Time(time::Time::from_hms(12, 0, 0).unwrap())));
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Date(time::Date::from_calendar_date(2019,time::Month::January, 1).unwrap())));
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::DateTime(time::OffsetDateTime::now_utc())));

        assert!(!FilterOperator::GreaterThan.is_value_compatible(&FilterValue::IntRange(1, 10)));
        assert!(!FilterOperator::GreaterThan.is_value_compatible(&FilterValue::ListInt(vec![1, 2, 3])));
        assert!(!FilterOperator::GreaterThan.is_value_compatible(&FilterValue::ListString(vec!["a".to_string(), "b".to_string()])));
        assert!(!FilterOperator::GreaterThan.is_value_compatible(&FilterValue::ListFloat(vec![1.0, 2.0, 3.0])));
        assert!(!FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Bool(true)));
    }

}