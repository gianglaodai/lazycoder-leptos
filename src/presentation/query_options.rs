use crate::business::error::CoreError;
use crate::business::filter::{Filter, FilterOperator, FilterValue};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Date, OffsetDateTime, Time};
use crate::business::sort::SortCriterion;

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub first_result: Option<i32>,
    pub max_results: Option<i32>,
    pub sort: Option<String>,
    pub p_filters: Option<Vec<String>>,
    pub a_filters: Option<Vec<String>>,
}

impl QueryOptions {
    pub fn to_sort_criteria(&self) -> Vec<SortCriterion> {
        self.sort
            .as_deref()
            .map(|s| {
                s.split('|')
                    .filter_map(|item| SortCriterion::from_str(item).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    pub fn to_filters(&self) -> Vec<Filter> {
        let mut filters = vec![];

        if let Some(p_filters) = &self.p_filters {
            for raw in p_filters {
                if let Ok(f) = QueryOptions::parse_single_filter(raw, true) {
                    filters.push(f);
                }
            }
        }

        if let Some(a_filters) = &self.a_filters {
            for raw in a_filters {
                if let Ok(f) = QueryOptions::parse_single_filter(raw, false) {
                    filters.push(f);
                }
            }
        }
        filters
    }
    fn parse_single_filter(raw: &str, is_property: bool) -> Result<Filter, CoreError> {
        use crate::business::filter::FilterOperator;

        let parts: Vec<&str> = raw.split(":").collect();
        if parts.len() < 2 {
            return Err(CoreError::UnprocessableEntity(
                "invalid_filter".to_string(),
                HashMap::from([("filter".to_string(), raw.to_string())]),
            ));
        }

        let key = parts[0].to_string();
        let operator_str = parts[1];
        let value_str_opt = parts.get(2).copied();

        let operator = match operator_str {
            "=" | "eq" | "" => FilterOperator::Equal,
            "!=" | "ne" => FilterOperator::NotEqual,
            ">" | "gt" => FilterOperator::GreaterThan,
            ">=" | "gte" => FilterOperator::GreaterThanOrEqual,
            "<" | "lt" => FilterOperator::LessThan,
            "<=" | "lte" => FilterOperator::LessThanOrEqual,
            "~" | "like" => FilterOperator::Like,
            "!~" | "not_like" | "!like" | "nlike" => FilterOperator::NotLike,
            "is" => FilterOperator::Is,
            "[]" | "in" => FilterOperator::In,
            "![]" | "not_in" | "nin" => FilterOperator::NotIn,
            "=null" | "is_null" => FilterOperator::IsNull,
            "!null" | "not_null" => FilterOperator::NotNull,
            ".." | "bw" | "between" => FilterOperator::Between,
            "!.." | "!bw" | "not_between" => FilterOperator::NotBetween,
            _ => {
                return Err(CoreError::UnprocessableEntity(
                    "invalid_filter_operator".into(),
                    HashMap::from([("operator".into(), operator_str.into())]),
                ));
            }
        };

        let value = match operator {
            FilterOperator::IsNull | FilterOperator::NotNull => FilterValue::Bool(true),
            _ => {
                let value_str = value_str_opt.unwrap();
                match Self::parse_value(value_str, &operator) {
                    Ok(v) => v,
                    Err(_e) => {
                        return Err(CoreError::UnprocessableEntity(
                            "invalid_filter_value".into(),
                            HashMap::from([
                                ("operator".into(), operator_str.into()),
                                ("value".into(), value_str.into()),
                            ]),
                        ))
                    }
                }
            }
        };

        Ok(if is_property {
            Filter::Property {
                property_name: key,
                operator,
                value,
            }
        } else {
            Filter::Attribute {
                attr_name: key,
                operator,
                value,
            }
        })
    }

    fn parse_value(value_str: &str, operator: &FilterOperator) -> Result<FilterValue, CoreError> {
        match operator {
            FilterOperator::In | FilterOperator::NotIn => {
                let items: Vec<&str> = value_str.split('|').collect();
                if let Ok(int_list) = items
                    .iter()
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(FilterValue::ListInt(int_list))
                } else if let Ok(float_list) = items
                    .iter()
                    .map(|s| s.parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(FilterValue::ListFloat(float_list))
                } else {
                    Ok(FilterValue::ListString(
                        items.iter().map(|s| s.to_string()).collect(),
                    ))
                }
            }
            FilterOperator::Between | FilterOperator::NotBetween => {
                let items: Vec<&str> = value_str.split('|').collect();
                if items.len() != 2 {
                    return Err(CoreError::UnprocessableEntity(
                        "invalid_range_value_format".into(),
                        HashMap::from([("value".into(), value_str.into())]),
                    ));
                }
                let (start, end) = (items[0], items[1]);

                if let (Ok(s), Ok(e)) = (start.parse::<i32>(), end.parse::<i32>()) {
                    Ok(FilterValue::IntRange(s, e))
                } else if let (Ok(s), Ok(e)) = (start.parse::<f64>(), end.parse::<f64>()) {
                    Ok(FilterValue::FloatRange(s, e))
                } else if let (Ok(s), Ok(e)) = (
                    Date::parse(start, format_description!("[year]-[month]-[day]")),
                    Date::parse(end, format_description!("[year]-[month]-[day]")),
                ) {
                    Ok(FilterValue::DateRange(s, e))
                } else if let (Ok(s), Ok(e)) = (
                    OffsetDateTime::parse(start, &Rfc3339),
                    OffsetDateTime::parse(end, &Rfc3339),
                ) {
                    Ok(FilterValue::DateTimeRange(s, e))
                } else if let (Ok(s), Ok(e)) = (
                    Time::parse(start, format_description!("[hour]:[minute]:[second]")),
                    Time::parse(end, format_description!("[hour]:[minute]:[second]")),
                ) {
                    Ok(FilterValue::TimeRange(s, e))
                } else {
                    Err(CoreError::UnprocessableEntity(
                        "invalid_range_value".into(),
                        HashMap::from([("value".into(), value_str.into())],
                    )))
                }
            }
            FilterOperator::Is => {
                if let Ok(b) = value_str.parse::<bool>() {
                    Ok(FilterValue::Bool(b))
                } else {
                    Err(CoreError::UnprocessableEntity(
                        "invalid_bool_value".into(),
                        HashMap::from([("value".into(), value_str.into())],
                    )))
                }
            }
            _ => {
                if let Ok(i) = value_str.parse::<i32>() {
                    Ok(FilterValue::Int(i))
                } else if let Ok(f) = value_str.parse::<f64>() {
                    Ok(FilterValue::Float(f))
                } else if let Ok(d) =
                    Date::parse(value_str, format_description!("[year]-[month]-[day]"))
                {
                    Ok(FilterValue::Date(d))
                } else if let Ok(dt) = OffsetDateTime::parse(value_str, &Rfc3339) {
                    Ok(FilterValue::DateTime(dt))
                } else if let Ok(t) =
                    Time::parse(value_str, format_description!("[hour]:[minute]:[second]"))
                {
                    Ok(FilterValue::Time(t))
                } else {
                    Ok(FilterValue::String(value_str.into()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::business::filter::{FilterOperator, FilterValue};
    use time::macros::{datetime, time};
    use time::Month;

    #[test]
    fn test_parse_single_value() {
        assert_eq!(
            QueryOptions::parse_value("42", &FilterOperator::Equal).unwrap(),
            FilterValue::Int(42)
        );
        assert_eq!(
            QueryOptions::parse_value("3.14", &FilterOperator::Equal).unwrap(),
            FilterValue::Float(3.14)
        );
        assert_eq!(
            QueryOptions::parse_value("true", &FilterOperator::Equal).unwrap(),
            FilterValue::String("true".to_string())
        );
        assert_eq!(
            QueryOptions::parse_value("true", &FilterOperator::Is).unwrap(),
            FilterValue::Bool(true)
        );
        assert_eq!(
            QueryOptions::parse_value("2025-07-16", &FilterOperator::Equal).unwrap(),
            FilterValue::Date(Date::from_calendar_date(2025, Month::July, 16).unwrap())
        );
        assert_eq!(
            QueryOptions::parse_value("2025-07-16T15:30:01Z", &FilterOperator::Equal).unwrap(),
            FilterValue::DateTime(datetime!(2025-07-16 15:30:01 UTC))
        );
        assert_eq!(
            QueryOptions::parse_value("15:30:01", &FilterOperator::Equal).unwrap(),
            FilterValue::Time(time!(15:30:01))
        );
    }

    #[test]
    fn test_parse_range_values() {
        assert_eq!(
            QueryOptions::parse_value("42|50", &FilterOperator::Between).unwrap(),
            FilterValue::IntRange(42, 50)
        );
        assert_eq!(
            QueryOptions::parse_value("3.14|4.14", &FilterOperator::Between).unwrap(),
            FilterValue::FloatRange(3.14, 4.14)
        );
        assert_eq!(
            QueryOptions::parse_value("2025-07-16|2025-07-18", &FilterOperator::Between).unwrap(),
            FilterValue::DateRange(
                Date::from_calendar_date(2025, Month::July, 16).unwrap(),
                Date::from_calendar_date(2025, Month::July, 18).unwrap()
            )
        );
        assert_eq!(
            QueryOptions::parse_value(
                "2025-07-16T15:30:01Z|2025-07-18T15:30:11Z",
                &FilterOperator::NotBetween
            )
            .unwrap(),
            FilterValue::DateTimeRange(
                datetime!(2025-07-16 15:30:01 UTC),
                datetime!(2025-07-18 15:30:11 UTC)
            )
        );
        assert_eq!(
            QueryOptions::parse_value("15:30:01|23:10:11", &FilterOperator::NotBetween).unwrap(),
            FilterValue::TimeRange(time!(15:30:01), time!(23:10:11))
        );
    }

    #[test]
    fn test_parse_list_values() {
        assert_eq!(
            QueryOptions::parse_value("42|50", &FilterOperator::In).unwrap(),
            FilterValue::ListInt(vec![42, 50])
        );
        assert_eq!(
            QueryOptions::parse_value("3.14|4.14", &FilterOperator::In).unwrap(),
            FilterValue::ListFloat(vec![3.14, 4.14])
        );
        assert_eq!(
            QueryOptions::parse_value("abc|xyz|rst", &FilterOperator::NotIn).unwrap(),
            FilterValue::ListString(vec!["abc".into(), "xyz".into(), "rst".into()])
        );
    }

    #[test]
    fn test_parse_single_filter() {
        let f = QueryOptions::parse_single_filter("age:=:32", true).unwrap();
        assert_eq!(
            f,
            Filter::Property {
                property_name: "age".to_string(),
                operator: FilterOperator::Equal,
                value: FilterValue::Int(32)
            }
        );
        let f = QueryOptions::parse_single_filter("age:is:true", false).unwrap();
        assert_eq!(
            f,
            Filter::Attribute {
                attr_name: "age".to_string(),
                operator: FilterOperator::Is,
                value: FilterValue::Bool(true)
            }
        );
        let f = QueryOptions::parse_single_filter("status:in:active|pending|done", true).unwrap();
        assert_eq!(
            f,
            Filter::Property {
                property_name: "status".to_string(),
                operator: FilterOperator::In,
                value: FilterValue::ListString(vec![
                    "active".into(),
                    "pending".into(),
                    "done".into()
                ])
            }
        );
        let f = QueryOptions::parse_single_filter("create_at:between:2025-01-01|2025-12-31", false)
            .unwrap();
        assert_eq!(
            f,
            Filter::Attribute {
                attr_name: "create_at".to_string(),
                operator: FilterOperator::Between,
                value: FilterValue::DateRange(
                    Date::parse("2025-01-01", format_description!("[year]-[month]-[day]")).unwrap(),
                    Date::parse("2025-12-31", format_description!("[year]-[month]-[day]")).unwrap()
                )
            }
        );
        let f = QueryOptions::parse_single_filter("name:=null", true).unwrap();
        assert_eq!(
            f,
            Filter::Property {
                property_name: "name".to_string(),
                operator: FilterOperator::IsNull,
                value: FilterValue::Bool(true)
            }
        );
    }
}
