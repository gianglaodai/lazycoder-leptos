use crate::business::error::CoreError;
use crate::business::filter::{Filter, FilterOperator, FilterValue};
use crate::business::sort::SortCriterion;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Date, OffsetDateTime, Time};

#[derive(Debug, Clone, Copy)]
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
    fn from_code(code_str: &str) -> Result<Self, CoreError> {
        let code: u8 = code_str.parse().map_err(|_| {
            CoreError::UnprocessableEntity(
                "error.filters.invalid.datatype".into(),
                HashMap::from([("datatype".into(), code_str.into())]),
            )
        })?;
        match code {
            0 => Ok(ValueDataType::String),
            1 => Ok(ValueDataType::Int),
            2 => Ok(ValueDataType::Float),
            3 => Ok(ValueDataType::Bool),
            4 => Ok(ValueDataType::Date),
            5 => Ok(ValueDataType::DateTime),
            6 => Ok(ValueDataType::Time),
            _ => Err(CoreError::UnprocessableEntity(
                "error.filters.invalid.datatype".into(),
                HashMap::from([("datatype".into(), code_str.into())]),
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub first_result: Option<i32>,
    pub max_results: Option<i32>,
    pub sort: Option<String>,
    pub p_filters: Option<Vec<String>>,
    pub a_filters: Option<Vec<String>>,
    pub search: Option<String>,
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

        if let Some(search_value) = &self.search {
            let keyword = search_value.trim();
            if !keyword.is_empty() {
                filters.push(Filter::Search {
                    value: keyword.to_string(),
                });
            }
        }
        filters
    }
    fn parse_single_filter(raw: &str, is_property: bool) -> Result<Filter, CoreError> {
        use crate::business::filter::FilterOperator;

        let parts: Vec<&str> = raw.split(":").collect();
        if parts.len() < 2 {
            return Err(CoreError::UnprocessableEntity(
                "error.filters.invalid.format",
                HashMap::from([("filter".to_string(), raw.to_string())]),
            ));
        }

        let key = parts[0].to_string();
        let operator_str = parts[1];
        let value_str_opt = parts.get(2).copied();
        let dtype_opt = parts.get(3).copied();

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
                let value_str = value_str_opt.ok_or_else(|| {
                    CoreError::UnprocessableEntity(
                        "error.filters.invalid.format",
                        HashMap::from([("filter".to_string(), raw.to_string())]),
                    )
                })?;
                let dtype = dtype_opt
                    .ok_or_else(|| {
                        CoreError::UnprocessableEntity(
                            "error.filters.missing.datatype".into(),
                            HashMap::from([("filter".into(), raw.into())]),
                        )
                    })
                    .and_then(ValueDataType::from_code)?;
                match Self::parse_value(value_str, &operator, dtype) {
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

    fn parse_value(
        value_str: &str,
        operator: &FilterOperator,
        data_type: ValueDataType,
    ) -> Result<FilterValue, CoreError> {
        match operator {
            FilterOperator::In | FilterOperator::NotIn => {
                let items: Vec<&str> = value_str.split('|').collect();
                match data_type {
                    ValueDataType::Int => {
                        let parsed = items
                            .iter()
                            .map(|s| s.parse::<i32>())
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|_| {
                                CoreError::UnprocessableEntity(
                                    "error.invalid.list.value".into(),
                                    HashMap::from([("value".into(), value_str.into())]),
                                )
                            })?;
                        Ok(FilterValue::ListInt(parsed))
                    }
                    ValueDataType::Float => {
                        let parsed = items
                            .iter()
                            .map(|s| s.parse::<f64>())
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|_| {
                                CoreError::UnprocessableEntity(
                                    "error.invalid.list.value".into(),
                                    HashMap::from([("value".into(), value_str.into())]),
                                )
                            })?;
                        Ok(FilterValue::ListFloat(parsed))
                    }
                    ValueDataType::String => Ok(FilterValue::ListString(
                        items.into_iter().map(|s| s.to_string()).collect(),
                    )),
                    // Not supported list datatypes in current FilterValue
                    _ => Err(CoreError::UnprocessableEntity(
                        "error.invalid.list.datatype".into(),
                        HashMap::from([("value".into(), value_str.into())]),
                    )),
                }
            }
            FilterOperator::Between | FilterOperator::NotBetween => {
                let items: Vec<&str> = value_str.split('|').collect();
                if items.len() != 2 {
                    return Err(CoreError::UnprocessableEntity(
                        "error.invalid.range.value.format".into(),
                        HashMap::from([("value".into(), value_str.into())]),
                    ));
                }
                let (start, end) = (items[0], items[1]);
                match data_type {
                    ValueDataType::Int => {
                        let s = start.parse::<i32>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        let e = end.parse::<i32>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::IntRange(s, e))
                    }
                    ValueDataType::Float => {
                        let s = start.parse::<f64>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        let e = end.parse::<f64>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::FloatRange(s, e))
                    }
                    ValueDataType::Date => {
                        let s = Date::parse(start, format_description!("[year]-[month]-[day]"))
                            .map_err(|_| {
                                CoreError::UnprocessableEntity(
                                    "error.invalid.range.value".into(),
                                    HashMap::from([("value".into(), value_str.into())]),
                                )
                            })?;
                        let e = Date::parse(end, format_description!("[year]-[month]-[day]"))
                            .map_err(|_| {
                                CoreError::UnprocessableEntity(
                                    "error.invalid.range.value".into(),
                                    HashMap::from([("value".into(), value_str.into())]),
                                )
                            })?;
                        Ok(FilterValue::DateRange(s, e))
                    }
                    ValueDataType::DateTime => {
                        let s = OffsetDateTime::parse(start, &Rfc3339).map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        let e = OffsetDateTime::parse(end, &Rfc3339).map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::DateTimeRange(s, e))
                    }
                    ValueDataType::Time => {
                        let s = Time::parse(start, format_description!("[hour]:[minute]:[second]"))
                            .map_err(|_| {
                                CoreError::UnprocessableEntity(
                                    "error.invalid.range.value".into(),
                                    HashMap::from([("value".into(), value_str.into())]),
                                )
                            })?;
                        let e = Time::parse(end, format_description!("[hour]:[minute]:[second]"))
                            .map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.range.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::TimeRange(s, e))
                    }
                    _ => Err(CoreError::UnprocessableEntity(
                        "error.invalid.range.datatype".into(),
                        HashMap::from([("value".into(), value_str.into())]),
                    )),
                }
            }
            FilterOperator::Is => match data_type {
                ValueDataType::Bool => {
                    let b = value_str.parse::<bool>().map_err(|_| {
                        CoreError::UnprocessableEntity(
                            "error.invalid.bool.value".into(),
                            HashMap::from([("value".into(), value_str.into())]),
                        )
                    })?;
                    Ok(FilterValue::Bool(b))
                }
                _ => Err(CoreError::UnprocessableEntity(
                    "error.invalid.bool.datatype".into(),
                    HashMap::from([("value".into(), value_str.into())]),
                )),
            },
            _ => {
                // Single value based on dtype
                match data_type {
                    ValueDataType::Int => {
                        let v = value_str.parse::<i32>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.int.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::Int(v))
                    }
                    ValueDataType::Float => {
                        let v = value_str.parse::<f64>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.float.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::Float(v))
                    }
                    ValueDataType::String => Ok(FilterValue::String(value_str.into())),
                    ValueDataType::Bool => {
                        let v = value_str.parse::<bool>().map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.bool.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::Bool(v))
                    }
                    ValueDataType::Date => {
                        let v = Date::parse(value_str, format_description!("[year]-[month]-[day]"))
                            .map_err(|_| {
                                CoreError::UnprocessableEntity(
                                    "error.invalid.date.value".into(),
                                    HashMap::from([("value".into(), value_str.into())]),
                                )
                            })?;
                        Ok(FilterValue::Date(v))
                    }
                    ValueDataType::DateTime => {
                        let v = OffsetDateTime::parse(value_str, &Rfc3339).map_err(|_| {
                            CoreError::UnprocessableEntity(
                                "error.invalid.datetime.value".into(),
                                HashMap::from([("value".into(), value_str.into())]),
                            )
                        })?;
                        Ok(FilterValue::DateTime(v))
                    }
                    ValueDataType::Time => {
                        let v =
                            Time::parse(value_str, format_description!("[hour]:[minute]:[second]"))
                                .map_err(|_| {
                                    CoreError::UnprocessableEntity(
                                        "error.invalid.time.value".into(),
                                        HashMap::from([("value".into(), value_str.into())]),
                                    )
                                })?;
                        Ok(FilterValue::Time(v))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::business::filter::{FilterOperator, FilterValue};
    use leptos::prelude::RenderHtml;
    use time::macros::{datetime, time};
    use time::Month;

    #[test]
    fn test_parse_single_value() {
        assert_eq!(
            QueryOptions::parse_value("42", &FilterOperator::Equal, ValueDataType::Int).unwrap(),
            FilterValue::Int(42)
        );
        assert_eq!(
            QueryOptions::parse_value("3.14", &FilterOperator::Equal, ValueDataType::Float)
                .unwrap(),
            FilterValue::Float(3.14)
        );
        assert_eq!(
            QueryOptions::parse_value("true", &FilterOperator::Equal, ValueDataType::String)
                .unwrap(),
            FilterValue::String("true".to_string())
        );
        assert_eq!(
            QueryOptions::parse_value("true", &FilterOperator::Is, ValueDataType::Bool).unwrap(),
            FilterValue::Bool(true)
        );
        assert_eq!(
            QueryOptions::parse_value("2025-07-16", &FilterOperator::Equal, ValueDataType::Date)
                .unwrap(),
            FilterValue::Date(Date::from_calendar_date(2025, Month::July, 16).unwrap())
        );
        assert_eq!(
            QueryOptions::parse_value(
                "2025-07-16T15:30:01Z",
                &FilterOperator::Equal,
                ValueDataType::DateTime
            )
            .unwrap(),
            FilterValue::DateTime(datetime!(2025-07-16 15:30:01 UTC))
        );
        assert_eq!(
            QueryOptions::parse_value("15:30:01", &FilterOperator::Equal, ValueDataType::Time)
                .unwrap(),
            FilterValue::Time(time!(15:30:01))
        );
    }

    #[test]
    fn test_parse_range_values() {
        assert_eq!(
            QueryOptions::parse_value("42|50", &FilterOperator::Between, ValueDataType::Int)
                .unwrap(),
            FilterValue::IntRange(42, 50)
        );
        assert_eq!(
            QueryOptions::parse_value("3.14|4.14", &FilterOperator::Between, ValueDataType::Float)
                .unwrap(),
            FilterValue::FloatRange(3.14, 4.14)
        );
        assert_eq!(
            QueryOptions::parse_value(
                "2025-07-16|2025-07-18",
                &FilterOperator::Between,
                ValueDataType::Date
            )
            .unwrap(),
            FilterValue::DateRange(
                Date::from_calendar_date(2025, Month::July, 16).unwrap(),
                Date::from_calendar_date(2025, Month::July, 18).unwrap()
            )
        );
        assert_eq!(
            QueryOptions::parse_value(
                "2025-07-16T15:30:01Z|2025-07-18T15:30:11Z",
                &FilterOperator::NotBetween,
                ValueDataType::DateTime
            )
            .unwrap(),
            FilterValue::DateTimeRange(
                datetime!(2025-07-16 15:30:01 UTC),
                datetime!(2025-07-18 15:30:11 UTC)
            )
        );
        assert_eq!(
            QueryOptions::parse_value(
                "15:30:01|23:10:11",
                &FilterOperator::NotBetween,
                ValueDataType::Time
            )
            .unwrap(),
            FilterValue::TimeRange(time!(15:30:01), time!(23:10:11))
        );
    }

    #[test]
    fn test_parse_list_values() {
        assert_eq!(
            QueryOptions::parse_value("42|50", &FilterOperator::In, ValueDataType::Int).unwrap(),
            FilterValue::ListInt(vec![42, 50])
        );
        assert_eq!(
            QueryOptions::parse_value("3.14|4.14", &FilterOperator::In, ValueDataType::Float)
                .unwrap(),
            FilterValue::ListFloat(vec![3.14, 4.14])
        );
        assert_eq!(
            QueryOptions::parse_value("abc|xyz|rst", &FilterOperator::NotIn, ValueDataType::String)
                .unwrap(),
            FilterValue::ListString(vec!["abc".into(), "xyz".into(), "rst".into()])
        );
    }

    #[test]
    fn test_parse_single_filter() {
        let f = QueryOptions::parse_single_filter("age:=:32:1", true).unwrap();
        assert_eq!(
            f,
            Filter::Property {
                property_name: "age".to_string(),
                operator: FilterOperator::Equal,
                value: FilterValue::Int(32)
            }
        );
        let f = QueryOptions::parse_single_filter("age:is:true:3", false).unwrap();
        assert_eq!(
            f,
            Filter::Attribute {
                attr_name: "age".to_string(),
                operator: FilterOperator::Is,
                value: FilterValue::Bool(true)
            }
        );
        let f = QueryOptions::parse_single_filter("status:in:active|pending|done:0", true).unwrap();
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
        let f =
            QueryOptions::parse_single_filter("create_at:between:2025-01-01|2025-12-31:4", false)
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

    #[test]
    fn test_to_filter() {
        let query = QueryOptions {
            sort: Some("+name|-age".into_owned()),
            first_result: Some(0),
            max_results: Some(5),
            a_filters: Some(vec!["p_name:=:giang:0".to_owned()]),
            p_filters: Some(vec!["name:=:hoang:0".to_owned(), "age:<=:5:1".to_owned()]),
            search: Some("abc xyz".to_owned()),
        };
        assert_eq!(
            query.to_filters(),
            vec![
                Filter::Property {
                    property_name: "name".to_owned(),
                    operator: FilterOperator::Equal,
                    value: FilterValue::String("hoang".to_owned())
                },
                Filter::Property {
                    property_name: "age".to_owned(),
                    operator: FilterOperator::LessThanOrEqual,
                    value: FilterValue::Int(5)
                },
                Filter::Attribute {
                    attr_name: "p_name".to_owned(),
                    operator: FilterOperator::Equal,
                    value: FilterValue::String("giang".to_owned())
                },
                Filter::Search {
                    value: "abc xyz".to_owned()
                }
            ]
        );
    }
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
