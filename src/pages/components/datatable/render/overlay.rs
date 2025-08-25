// render/overlay.rs
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::*;
use std::sync::Arc;

#[component]
pub fn LoadingOverlay<T: Send + Sync + 'static>(
    #[prop(into)] _state: Arc<TableState<T>>,
) -> impl IntoView {
    view! {
        <div class="lc-dt-overlay absolute inset-0 flex items-center justify-center bg-white/70 text-gray-600">
            <span class="animate-pulse">Loading...</span>
        </div>
    }
}

#[component]
pub fn EmptyOverlay<T: Send + Sync + 'static>(
    #[prop(into)] _state: Arc<TableState<T>>,
) -> impl IntoView {
    view! {
        <div class="lc-dt-overlay absolute inset-0 flex items-center justify-center text-gray-400">
            <span>No rows to display</span>
        </div>
    }
}

#[component]
pub fn ErrorOverlay<T: Send + Sync + 'static>(
    #[prop(into)] state: Arc<TableState<T>>,
) -> impl IntoView {
    let msg = state.error.get_untracked().unwrap_or_else(|| "Unknown error".to_string());
    view! {
        <div class="lc-dt-overlay absolute inset-0 flex items-center justify-center bg-red-50 text-red-700">
            <span>{msg}</span>
        </div>
    }
}
