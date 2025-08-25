// render/table.rs
use crate::pages::components::datatable::core::state::TableState;
use crate::pages::components::datatable::render::body::VirtualizedBody;
use crate::pages::components::datatable::render::footer::StatusBar;
use crate::pages::components::datatable::render::header::HeaderRow;
use crate::pages::components::datatable::render::overlay::{
    EmptyOverlay, ErrorOverlay, LoadingOverlay,
};
use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct DataTableProperties<T: 'static + Send + Sync> {
    pub state: Arc<TableState<T>>,
    pub height: Option<String>,
    pub row_height: Option<i32>,
}

#[component]
pub fn DataTable<T: Clone + Send + Sync + 'static>(
    #[prop(into)] state: Arc<TableState<T>>,
    #[prop(optional)] height: Option<String>,
    #[prop(optional)] row_height: Option<i32>,
) -> impl IntoView {
    let height = height.unwrap_or_else(|| "auto".to_string());
    let row_height = row_height.unwrap_or(36);

    // Basic AG Grid-inspired container with sticky header and scrollable body
    let loading = state.loading;
    let error = state.error;
    let rows = state.rows;

    // Clone state for multiple component uses to satisfy move semantics in closures
    let s_head = state.clone();
    let s_body = state.clone();
    let s_load = state.clone();
    let s_err = state.clone();
    let s_empty = state.clone();
    let s_foot = state.clone();

    view! {
        <div class="lc-datatable relative border border-gray-200 rounded-md overflow-hidden text-sm bg-white">
            <div class="lc-dt-body overflow-auto" style=move || format!("height:{};", height)>
                <div class="lc-dt-header sticky top-0 z-10 bg-gray-50 border-b border-gray-200">
                    <HeaderRow state=s_head />
                </div>
                <VirtualizedBody state=s_body row_height=row_height />
            </div>
            <Show when=move || loading.get()>
                <LoadingOverlay _state=s_load.clone() />
            </Show>
            <Show when=move || error.get().is_some()>
                <ErrorOverlay state=s_err.clone() />
            </Show>
            <Show when=move || !loading.get() && error.get().is_none() && rows.with(|v| v.is_empty())>
                <EmptyOverlay _state=s_empty.clone() />
            </Show>
            <div class="lc-dt-footer border-t border-gray-200 bg-gray-50">
                <StatusBar state=s_foot />
            </div>
        </div>
    }
}
