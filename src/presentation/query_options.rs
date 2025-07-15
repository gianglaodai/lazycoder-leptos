use crate::business::filter::Filter;
use crate::business::repository::SortCriterion;
use serde::Deserialize;
use std::str::FromStr;

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
                if let Some(f) = QueryOptions::parse_single_filter(raw, true) {
                    filters.push(f);
                }
            }
        }

        if let Some(a_filters) = &self.a_filters {
            for raw in a_filters {
                if let Some(f) = QueryOptions::parse_single_filter(raw, false) {
                    filters.push(f);
                }
            }
        }
        filters
    }
    fn parse_single_filter(raw: &str, is_property: bool) -> Option<Filter> {
        use crate::business::filter::{FilterOperator, FilterValue};

        let parts: Vec<&str> = raw.split("|").collect();
        if parts.len() < 3 {
            return None;
        }

        let key = parts[0].to_string();
        let operator_str = parts[1];
        let value_str = parts[2];

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
            "?" | "search" => FilterOperator::Search,
            _ => return None,
        };

        let value = if let Ok(int_val) = value_str.parse::<i32>() {
            FilterValue::Int(int_val)
        } else {
            FilterValue::String(value_str.to_string())
        };

        Some(if is_property {
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
}
