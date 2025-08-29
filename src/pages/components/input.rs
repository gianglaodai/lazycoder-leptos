use leptos::ev;
use leptos::prelude::*;
use std::collections::HashMap;

use crate::utils::tv::{Tv, TvConfig, TvProps, TvResult, VariantClass, VariantDef};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputSize {
    Default,
    Sm,
    Lg,
}

impl Default for InputSize {
    fn default() -> Self {
        InputSize::Default
    }
}

fn input_tv() -> Tv {
    // Base based on shadcn/ui input styles
    let base = VariantClass::All(
        "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive".to_string(),
    );

    let mut variants: HashMap<String, VariantDef> = HashMap::new();

    // size variant adjusts height and vertical padding
    let mut size = VariantDef::new();
    size.values
        .insert("default".into(), VariantClass::All("h-9 py-1".into()));
    size.values
        .insert("sm".into(), VariantClass::All("h-8 py-1".into()));
    size.values
        .insert("lg".into(), VariantClass::All("h-10 py-2".into()));
    variants.insert("size".into(), size);

    let mut defaults = HashMap::new();
    defaults.insert("size".into(), "default".into());

    let cfg = TvConfig {
        base,
        variants,
        default_variants: defaults,
        compound_variants: vec![],
        slots: None,
    };

    Tv::new(cfg)
}

#[component]
pub fn Input(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] r#type: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional, default = Signal::from(String::new()))] value: Signal<String>,
    #[prop(into, optional)] on_input: Option<Callback<ev::Event, ()>>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event, ()>>,
    #[prop(optional)] size: InputSize,
) -> impl IntoView {
    let tv = input_tv();

    let mut vmap: HashMap<String, String> = HashMap::new();
    vmap.insert(
        "size".into(),
        match size {
            InputSize::Default => "default",
            InputSize::Sm => "sm",
            InputSize::Lg => "lg",
        }
        .into(),
    );

    let props = TvProps {
        variants: vmap,
        class: class.clone(),
        slot_classes: HashMap::new(),
    };
    let classes = match tv.build(&props) {
        TvResult::Single(s) => s,
        TvResult::Slots(_) => String::new(),
    };

    let t = r#type.unwrap_or_else(|| "text".to_string());

    view! {
        <input
            class=classes
            id=id
            name=name
            r#type=t
            disabled=disabled
            required=required
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| if let Some(cb) = on_input { cb.run(ev) }
            on:change=move |ev| if let Some(cb) = on_change { cb.run(ev) }
        />
    }
}
