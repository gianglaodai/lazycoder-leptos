use leptos::ev;
use leptos::prelude::*;
use std::collections::HashMap;

use crate::utils::tv::{Tv, TvConfig, TvProps, TvResult, VariantClass, VariantDef};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectSize {
    Default,
    Sm,
    Lg,
}

impl Default for SelectSize {
    fn default() -> Self { SelectSize::Default }
}

fn select_tv() -> Tv {
    let base = VariantClass::All(
        "w-full rounded-md border border-input bg-background px-3 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50".to_string(),
    );

    let mut variants: HashMap<String, VariantDef> = HashMap::new();

    let mut size = VariantDef::new();
    size.values.insert("default".into(), VariantClass::All("h-9".into()));
    size.values.insert("sm".into(), VariantClass::All("h-8".into()));
    size.values.insert("lg".into(), VariantClass::All("h-10".into()));
    variants.insert("size".into(), size);

    let mut defaults = HashMap::new();
    defaults.insert("size".into(), "default".into());

    let cfg = TvConfig { base, variants, default_variants: defaults, compound_variants: vec![], slots: None };
    Tv::new(cfg)
}

#[component]
pub fn Select(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(into, optional, default = Signal::from(String::new()))] value: Signal<String>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event, ()>>,
    #[prop(optional)] size: SelectSize,
    children: Children,
) -> impl IntoView {
    let tv = select_tv();

    let mut vmap: HashMap<String, String> = HashMap::new();
    vmap.insert(
        "size".into(),
        match size { SelectSize::Default => "default", SelectSize::Sm => "sm", SelectSize::Lg => "lg" }.into(),
    );

    let props = TvProps { variants: vmap, class: class.clone(), slot_classes: HashMap::new() };
    let classes = match tv.build(&props) { TvResult::Single(s) => s, TvResult::Slots(_) => String::new() };

    view! {
        <select
            class=classes
            id=id
            name=name
            disabled=disabled
            required=required
            prop:value=value
            on:change=move |ev| if let Some(cb) = on_change { cb.run(ev) }
        >
            {children()}
        </select>
    }
}
