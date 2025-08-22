use leptos::ev;
use leptos::prelude::*;
use std::collections::HashMap;

use crate::utils::tv::{Tv, TvConfig, TvProps, TvResult, VariantClass};

fn textarea_tv() -> Tv {
    let base = VariantClass::All(
        "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50".to_string(),
    );
    let cfg = TvConfig {
        base,
        variants: HashMap::new(),
        default_variants: HashMap::new(),
        compound_variants: vec![],
        slots: None,
    };
    Tv::new(cfg)
}

#[component]
pub fn Textarea(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] rows: Option<u32>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional, default = Signal::from(String::new()))] value: Signal<String>,
    #[prop(optional)] on_input: Option<Callback<ev::Event, ()>>,
    #[prop(optional)] on_blur: Option<Callback<ev::FocusEvent, ()>>,
) -> impl IntoView {
    let tv = textarea_tv();
    let props = TvProps { variants: HashMap::new(), class: class.clone(), slot_classes: HashMap::new() };
    let classes = match tv.build(&props) { TvResult::Single(s) => s, TvResult::Slots(_) => String::new() };

    view! {
        <textarea
            class=classes
            id=id
            name=name
            disabled=disabled
            required=required
            rows=rows
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| if let Some(cb) = on_input { cb.run(ev) }
            on:blur=move |ev| if let Some(cb) = on_blur { cb.run(ev) }
        />
    }
}
