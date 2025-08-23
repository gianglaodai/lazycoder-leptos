use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};

use crate::pages::components::{Input, Select};
use crate::presentation::query_options::ValueDataType;

/// ColumnFilter renders operator Select + value Input for a given field
/// It synchronizes with URL query params and preserves sort/search when updating p_filters
#[component]
pub fn ColumnFilter(
    field_name: String,
    field_datatype: ValueDataType,
    max_results: i64,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "flex items-center gap-1".to_string());

    let query = use_query_map();
    let navigate = use_navigate();

    // Read helpers from query
    let current_sort =
        move || query.with(|q| q.get("sort").map(|s| s.to_string()).unwrap_or_default());
    let current_search =
        move || query.with(|q| q.get("search").map(|s| s.to_string()).unwrap_or_default());
    let current_pfilter_raw = move || {
        query.with(|q| {
            q.get("p_filters")
                .map(|s| s.to_string())
                .unwrap_or_default()
        })
    };
    // helpers: none; we'll parse directly where needed to avoid move issues

    // Operator options by datatype (minimal set for now)
    let op_options = move || -> Vec<(&'static str, &'static str)> {
        match field_datatype {
            ValueDataType::String => vec![("=", "="), ("~", "~")],
            _ => vec![("=", "=")],
        }
    };

    view! {
        <div class=class>
            <Select
                class="h-7 text-xs w-16"
                value=Signal::derive({
                    let field_for_op = field_name.clone();
                    move || {
                        let raw = current_pfilter_raw();
                        if raw.is_empty() { return "=".to_string(); }
                        let parts: Vec<&str> = raw.splitn(4, ':').collect();
                        if parts.len() >= 2 && parts[0] == field_for_op { parts[1].to_string() } else { "=".to_string() }
                    }
                })
                on_change=Callback::new({
                    let navigate5 = navigate.clone();
                    let field = field_name.clone();
                    move |ev: leptos::ev::Event| {
                        let op = event_target_value(&ev);
                        let raw = current_pfilter_raw();
                        let vcur = if raw.is_empty() { String::new() } else {
                            let parts: Vec<&str> = raw.splitn(4, ':').collect();
                            if parts.len() >= 3 && parts[0] == field { parts[2].to_string() } else { String::new() }
                        };
                        let mut url = format!("?first_result=0&max_results={}", max_results);
                        let s = current_sort();
                        if !s.is_empty() { url.push_str(&format!("&sort={}", s)); }
                        let q = current_search();
                        if !q.is_empty() { url.push_str(&format!("&search={}", q)); }
                        if !vcur.is_empty() { url.push_str(&format!("&p_filters={}:{}:{}:{}", field, op, vcur, field_datatype.to_code())); }
                        let _ = navigate5(&url, Default::default());
                    }
                })
            >
                {view! {{
                    let opts = op_options();
                    let mut vs: Vec<AnyView> = Vec::new();
                    for (label, val) in opts.into_iter() {
                        vs.push(view!{ <option value=val.to_string()>{label.to_string()}</option> }.into_any());
                    }
                    vs.into_view()
                }}}
            </Select>
            <Input
                placeholder="Filter..."
                class="h-7 text-xs"
                value=Signal::derive({
                                    let field_for_val = field_name.clone();
                                    move || {
                                        let raw = current_pfilter_raw();
                                        if raw.is_empty() { return String::new(); }
                                        let parts: Vec<&str> = raw.splitn(4, ':').collect();
                                        if parts.len() >= 3 && parts[0] == field_for_val { parts[2].to_string() } else { String::new() }
                                    }
                                })
                on_input=Callback::new({
                    let navigate4 = navigate.clone();
                    let field = field_name.clone();
                    move |ev: leptos::ev::Event| {
                        let v = event_target_value(&ev);
                        let mut url = format!("?first_result=0&max_results={}", max_results);
                        let s = current_sort();
                        if !s.is_empty() { url.push_str(&format!("&sort={}", s)); }
                        let q = current_search();
                        if !q.is_empty() { url.push_str(&format!("&search={}", q)); }
                        if !v.is_empty() {
                            let raw = current_pfilter_raw();
                            let op = if raw.is_empty() { "=".to_string() } else {
                                let parts: Vec<&str> = raw.splitn(4, ':').collect();
                                if parts.len() >= 2 && parts[0] == field { parts[1].to_string() } else { "=".to_string() }
                            };
                            url.push_str(&format!("&p_filters={}:{}:{}:{}", field, op, v, field_datatype.to_code()));
                        }
                        let _ = navigate4(&url, Default::default());
                    }
                })
            />
        </div>
    }
}
