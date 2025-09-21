use crate::business::filter::{FilterOperator, FilterValue, ScalarValue};
use crate::pages::components::datatable::core::column::DataType;
use crate::pages::components::datatable::core::data_source::{SortModel, SortOrder};
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SyncOptions {
    pub include_sort: bool,
    pub include_p_filters: bool,
    pub include_a_filters: bool, // reserved for future
    pub include_search: bool,    // reserved for future
    pub first_result_key: &'static str,
    pub max_results_key: &'static str,
    pub sort_key: &'static str,
    pub p_filters_key: &'static str,
    pub a_filters_key: &'static str,
    pub search_key: &'static str,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            include_sort: true,
            include_p_filters: true,
            include_a_filters: false,
            include_search: false,
            first_result_key: "first_result",
            max_results_key: "max_results",
            sort_key: "sort",
            p_filters_key: "p_filters",
            a_filters_key: "a_filters",
            search_key: "search",
        }
    }
}

pub fn sync_table_query_to_url<T: Send + Sync + 'static, F>(
    state: Arc<TableState<T>>,
    mut navigate_with_query: F,
    opts: SyncOptions,
) where
    F: FnMut(String) + 'static + Clone,
{
    let st = state.clone();
    Effect::new(move |_| {
        let _ = st.query_version.get();
        // paging
        let ps = st.page_size.get_untracked().max(1);
        let cp = st.current_page.get_untracked().max(1);
        let first_result = (cp - 1) * ps;
        let max_results = ps;

        let mut parts: Vec<String> = vec![
            format!("{}={}", opts.first_result_key, first_result),
            format!("{}={}", opts.max_results_key, max_results),
        ];

        if opts.include_sort {
            if let Some(s) = build_sort_param(&st) {
                if !s.is_empty() {
                    parts.push(format!("{}={}", opts.sort_key, s));
                }
            }
        }
        if opts.include_p_filters {
            for pf in build_p_filters(&st).into_iter() {
                parts.push(format!("{}={}", opts.p_filters_key, pf));
            }
        }
        // a_filters/search placeholders for future extension
        // if opts.include_a_filters { }
        // if opts.include_search { }

        let qs = parts.join("&");
        navigate_with_query(format!("?{}", qs));
    });
}

pub fn build_sort_param<T: Send + Sync + 'static>(state: &Arc<TableState<T>>) -> Option<String> {
    let mut v: Vec<SortModel> = state.sort_model.get_untracked();
    if v.is_empty() {
        return Some(String::new());
    }
    v.sort_by(|a, b| match (a.sort_index, b.sort_index) {
        (Some(x), Some(y)) => x.cmp(&y),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.col_id.cmp(&b.col_id),
    });
    let cols = state.columns.read_untracked();
    let mut items: Vec<String> = vec![];
    for sm in v.into_iter() {
        let field = cols
            .iter()
            .find(|c| c.id == sm.col_id)
            .and_then(|c| c.field)
            .unwrap_or_else(|| sm.col_id.as_str());
        let prefix = match sm.sort {
            SortOrder::Asc => "+",
            SortOrder::Desc => "-",
        };
        items.push(format!("{}{}", prefix, field));
    }
    Some(items.join("|"))
}

pub fn build_p_filters<T: Send + Sync + 'static>(state: &Arc<TableState<T>>) -> Vec<String> {
    let cols = state.columns.read_untracked();
    let to_dtype_code = |c: &DataType| -> u8 {
        match c {
            DataType::Text => 0,
            DataType::Int => 1,
            DataType::Float => 2,
            DataType::Boolean => 3,
            DataType::Date => 4,
            DataType::DateTime => 5,
            DataType::Time => 6,
        }
    };
    let op_str = |op: &FilterOperator| -> &'static str {
        match op {
            FilterOperator::Equal => "=",
            FilterOperator::NotEqual => "!=",
            FilterOperator::GreaterThan => ">",
            FilterOperator::GreaterThanOrEqual => ">=",
            FilterOperator::LessThan => "<",
            FilterOperator::LessThanOrEqual => "<=",
            FilterOperator::Like => "~",
            FilterOperator::NotLike => "!~",
            FilterOperator::In => "[]",
            FilterOperator::NotIn => "![]",
            FilterOperator::IsNull => "=null",
            FilterOperator::NotNull => "!null",
            FilterOperator::Between => "..",
            FilterOperator::NotBetween => "!..",
        }
    };
    let val_to_s = |v: &FilterValue| -> (String, u8) {
        match v {
            FilterValue::Single(sv) => match sv {
                ScalarValue::Int(i) => (i.to_string(), 1),
                ScalarValue::Float(f) => (f.to_string(), 2),
                ScalarValue::String(s) => (s.clone(), 0),
                ScalarValue::Bool(b) => (b.to_string(), 3),
                ScalarValue::Date(d) => (
                    d.format(&time::macros::format_description!("[year]-[month]-[day]"))
                        .unwrap_or_default(),
                    4,
                ),
                ScalarValue::DateTime(dt) => (
                    dt.format(&time::format_description::well_known::Rfc3339)
                        .unwrap_or_default(),
                    5,
                ),
                ScalarValue::Time(t) => (
                    t.format(&time::macros::format_description!(
                        "[hour]:[minute]:[second]"
                    ))
                    .unwrap_or_default(),
                    6,
                ),
            },
            FilterValue::Range((a, b)) => match (a, b) {
                (ScalarValue::Int(x), ScalarValue::Int(y)) => (format!("{}|{}", x, y), 1),
                (ScalarValue::Float(x), ScalarValue::Float(y)) => (format!("{}|{}", x, y), 2),
                (ScalarValue::Date(x), ScalarValue::Date(y)) => (
                    format!(
                        "{}|{}",
                        x.format(&time::macros::format_description!("[year]-[month]-[day]"))
                            .unwrap_or_default(),
                        y.format(&time::macros::format_description!("[year]-[month]-[day]"))
                            .unwrap_or_default()
                    ),
                    4,
                ),
                (ScalarValue::DateTime(x), ScalarValue::DateTime(y)) => (
                    format!(
                        "{}|{}",
                        x.format(&time::format_description::well_known::Rfc3339)
                            .unwrap_or_default(),
                        y.format(&time::format_description::well_known::Rfc3339)
                            .unwrap_or_default()
                    ),
                    5,
                ),
                (ScalarValue::Time(x), ScalarValue::Time(y)) => (
                    format!(
                        "{}|{}",
                        x.format(&time::macros::format_description!(
                            "[hour]:[minute]:[second]"
                        ))
                        .unwrap_or_default(),
                        y.format(&time::macros::format_description!(
                            "[hour]:[minute]:[second]"
                        ))
                        .unwrap_or_default()
                    ),
                    6,
                ),
                _ => (String::new(), 0),
            },
            FilterValue::List(vs) => {
                if let Some(first) = vs.first() {
                    match first {
                        ScalarValue::Int(_) => (
                            vs.iter()
                                .filter_map(|s| match s { ScalarValue::Int(i) => Some(i.to_string()), _ => None })
                                .collect::<Vec<_>>()
                                .join("|"),
                            1,
                        ),
                        ScalarValue::Float(_) => (
                            vs.iter()
                                .filter_map(|s| match s { ScalarValue::Float(f) => Some(f.to_string()), _ => None })
                                .collect::<Vec<_>>()
                                .join("|"),
                            2,
                        ),
                        ScalarValue::String(_) => (
                            vs.iter()
                                .filter_map(|s| match s { ScalarValue::String(st) => Some(st.clone()), _ => None })
                                .collect::<Vec<_>>()
                                .join("|"),
                            0,
                        ),
                        _ => (String::new(), 0),
                    }
                } else {
                    (String::new(), 0)
                }
            }
            FilterValue::None => (String::new(), 0),
        }
    };

    let mut out: Vec<String> = vec![];
    // simple per-column text contains (only sensible for text dtype)
    let col_text = state.filter_model.get_untracked().column_text;
    for (cid, needle) in col_text.into_iter() {
        if let Some(col) = cols.iter().find(|c| c.id == cid) {
            let field = col.field.unwrap_or(col.id);
            let dtype = col.data_type.unwrap_or(DataType::Text);
            let code = to_dtype_code(&dtype);
            if !needle.trim().is_empty() {
                out.push(format!("{}:{}:{}:{}", field, "~", needle, code));
            }
        }
    }

    let col_adv = state.filter_model.get_untracked().column_advanced;
    for (cid, adv) in col_adv.into_iter() {
        if let Some(col) = cols.iter().find(|c| c.id == cid) {
            let field = col.field.unwrap_or(col.id);
            let op = op_str(&adv.operator);
            match adv.operator {
                FilterOperator::IsNull | FilterOperator::NotNull => {
                    out.push(format!("{}:{}", field, op));
                }
                _ => {
                    if let Some(val) = adv.value {
                        let (vs, code) = val_to_s(&val);
                        out.push(format!("{}:{}:{}:{}", field, op, vs, code));
                    }
                }
            }
        }
    }
    out
}
