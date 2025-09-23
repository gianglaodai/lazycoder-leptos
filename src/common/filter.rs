#[derive(Clone, Debug, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    NotNull,
    Between,
    NotBetween,
}

#[derive(Clone, Debug, PartialEq)]
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
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarValue {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Date(time::Date),
    DateTime(time::OffsetDateTime),
    Time(time::Time),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FilterValue {
    Single(ScalarValue),
    List(Vec<ScalarValue>),
    Range((ScalarValue, ScalarValue)),
    None, // for is_null / not_null
}

impl FilterOperator {
    pub fn is_value_compatible(&self, value: &FilterValue) -> bool {
        use ScalarValue::*;
        match self {
            Self::Equal | Self::NotEqual => match value {
                FilterValue::Single(_) => true,
                _ => false,
            },
            Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::LessThan
            | Self::LessThanOrEqual => match value {
                FilterValue::Single(s) => {
                    matches!(s, Int(_) | Float(_) | Time(_) | Date(_) | DateTime(_))
                }
                _ => false,
            },
            Self::Like | Self::NotLike => match value {
                FilterValue::Single(String(_)) => true,
                _ => false,
            },
            Self::In | Self::NotIn => match value {
                FilterValue::List(vs) => {
                    if vs.is_empty() {
                        return false;
                    }
                    // allow homogeneous lists of Int, Float, or Str
                    let all_int = vs.iter().all(|v| matches!(v, Int(_)));
                    let all_float = vs.iter().all(|v| matches!(v, Float(_)));
                    let all_str = vs.iter().all(|v| matches!(v, String(_)));
                    all_int || all_float || all_str
                }
                _ => false,
            },
            Self::IsNull | Self::NotNull => matches!(value, FilterValue::None),
            Self::Between | Self::NotBetween => match value {
                FilterValue::Range((a, b)) => {
                    use std::mem::discriminant;
                    let same_kind = discriminant(a) == discriminant(b);
                    let comparable =
                        matches!(a, Int(_) | Float(_) | Time(_) | Date(_) | DateTime(_));
                    same_kind && comparable
                }
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_operator() {
        use ScalarValue::*;
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Single(Int(42))));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Single(Float(3.14))));
        assert!(FilterOperator::Equal
            .is_value_compatible(&FilterValue::Single(String("hello".to_string()))));
        assert!(FilterOperator::Equal.is_value_compatible(&FilterValue::Single(Bool(true))));
        assert!(
            FilterOperator::Equal.is_value_compatible(&FilterValue::Single(String("abc".into())))
        );
        assert!(
            FilterOperator::Equal.is_value_compatible(&FilterValue::Single(DateTime(
                time::OffsetDateTime::now_utc()
            )))
        );
        assert!(
            FilterOperator::Equal.is_value_compatible(&FilterValue::Single(Time(
                time::Time::from_hms(12, 0, 0).unwrap()
            )))
        );

        assert!(!FilterOperator::Equal.is_value_compatible(&FilterValue::Range((Int(1), Int(10)))));
        assert!(
            !FilterOperator::Equal.is_value_compatible(&FilterValue::List(vec![
                Int(1),
                Int(2),
                Int(3)
            ]))
        );
        assert!(
            !FilterOperator::Equal.is_value_compatible(&FilterValue::List(vec![
                String("a".to_string()),
                String("b".to_string())
            ]))
        );
    }

    #[test]
    fn test_greater_operator() {
        use ScalarValue::*;
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Single(Int(42))));
        assert!(FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Single(Float(3.14))));
        assert!(
            FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Single(Time(
                time::Time::from_hms(12, 0, 0).unwrap()
            )))
        );
        assert!(
            FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Single(Date(
                time::Date::from_calendar_date(2019, time::Month::January, 1).unwrap()
            )))
        );
        assert!(
            FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Single(DateTime(
                time::OffsetDateTime::now_utc()
            )))
        );

        assert!(!FilterOperator::GreaterThan
            .is_value_compatible(&FilterValue::Range((Int(1), Int(10)))));
        assert!(
            !FilterOperator::GreaterThan.is_value_compatible(&FilterValue::List(vec![
                Int(1),
                Int(2),
                Int(3)
            ]))
        );
        assert!(
            !FilterOperator::GreaterThan.is_value_compatible(&FilterValue::List(vec![
                String("a".to_string()),
                String("b".to_string())
            ]))
        );
        assert!(!FilterOperator::GreaterThan.is_value_compatible(&FilterValue::Single(Bool(true))));
    }
}
