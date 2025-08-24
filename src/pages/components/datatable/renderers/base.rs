use leptos::attr::Scope;
use leptos::prelude::*;
use crate::pages::components::datatable::core::render_value::Value;

/// Trait cho tất cả cell renderers (inspired by AG Grid)
pub trait ICellRenderer<T> {
    /// Trả về View để gắn vào ô
    fn view(&self, _cx: Scope, _value: &Value, _row: &T) -> impl IntoView {
        unimplemented!()
    }
}


/// Tham số khởi tạo (tuỳ chọn)
#[derive(Clone, Default)]
pub struct RendererParams {
    pub class: Option<String>,
}