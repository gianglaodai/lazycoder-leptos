use leptos::ev;
use leptos::prelude::*;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::utils::tv::{Tv, TvConfig, TvProps, TvResult, VariantClass, VariantDef};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectSize {
    Default,
    Sm,
    Lg,
}

impl Default for SelectSize {
    fn default() -> Self {
        SelectSize::Default
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: false }
    }
}

fn select_tv() -> Tv {
    let base = VariantClass::All(
        "w-full rounded-md border border-input bg-background px-3 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50".to_string(),
    );

    let mut variants: HashMap<String, VariantDef> = HashMap::new();

    let mut size = VariantDef::new();
    size.values
        .insert("default".into(), VariantClass::All("h-9".into()));
    size.values
        .insert("sm".into(), VariantClass::All("h-8".into()));
    size.values
        .insert("lg".into(), VariantClass::All("h-10".into()));
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
pub fn Select(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(into, optional, default = Signal::from(String::new()))] value: Signal<String>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event, ()>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent, ()>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent, ()>>,
    // New: lazy-loaded options
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] load_options: Option<Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<Vec<SelectOption>, String>> + Send>> + Send + Sync>>,
    #[prop(optional)] size: SelectSize,
    children: Children,
) -> impl IntoView {
    let tv = select_tv();

    let mut vmap: HashMap<String, String> = HashMap::new();
    vmap.insert(
        "size".into(),
        match size {
            SelectSize::Default => "default",
            SelectSize::Sm => "sm",
            SelectSize::Lg => "lg",
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

    // Internal lazy-load trigger if a loader is provided
    // Changed: use a tick counter so options are reloaded on every interaction
    let has_loader = load_options.is_some();
    let tick = RwSignal::new(0u32);

    let options_res = Resource::new(
        {
            let tick = tick.clone();
            move || tick.get()
        },
        {
            let load_options = load_options.clone();
            move |t| {
                let load_options = load_options.clone();
                async move {
                    if t > 0 {
                        if let Some(loader) = load_options {
                            loader().await
                        } else {
                            Ok(vec![])
                        }
                    } else {
                        Ok(vec![])
                    }
                }
            }
        },
    );

    let placeholder_text = placeholder.unwrap_or_else(|| "Select an option".to_string());

    // Cache children view to avoid moving `children()` into reactive closures
    let children_any = children().into_any();

    if has_loader {
        view! {
            <select
                class=classes
                id=id
                name=name
                disabled=disabled
                required=required
                prop:value=value
                on:change=move |ev| if let Some(cb) = on_change { cb.run(ev) }
                on:focus=move |ev| {
                    tick.update(|v| *v = v.saturating_add(1));
                    if let Some(cb) = on_focus { cb.run(ev) }
                }
                on:click=move |ev| {
                    tick.update(|v| *v = v.saturating_add(1));
                    if let Some(cb) = on_click { cb.run(ev) }
                }
            >
                {move || {
                    if tick.get() == 0 {
                        view! {
                            <>
                                <option value="" selected=move || value.get().is_empty() disabled=true hidden=true>{placeholder_text.clone()}</option>
                                <option value="" disabled=true>{"Click to load options"}</option>
                            </>
                        }.into_any()
                    } else {
                        match options_res.get() {
                            Some(Ok(items)) => {
                                view! {
                                    <>
                                        <option value="" selected=move || value.get().is_empty() disabled=true hidden=true>{placeholder_text.clone()}</option>
                                        {items.into_iter().map(|opt| {
                                            let v = opt.value;
                                            let l = opt.label;
                                            let dis = opt.disabled;
                                            view! { <option value={v} disabled=dis>{l}</option> }
                                        }).collect_view()}
                                    </>
                                }.into_any()
                            }
                            Some(Err(_e)) => {
                                view! {
                                    <>
                                        <option value="" selected=move || value.get().is_empty() disabled=true hidden=true>{placeholder_text.clone()}</option>
                                        <option value="" disabled=true>{"Failed to load options"}</option>
                                    </>
                                }.into_any()
                            }
                            None => {
                                view! {
                                    <>
                                        <option value="" selected=move || value.get().is_empty() disabled=true hidden=true>{placeholder_text.clone()}</option>
                                        <option value="" disabled=true>{"Loading..."}</option>
                                    </>
                                }.into_any()
                            }
                        }
                    }
                }}
            </select>
        }.into_any()
    } else {
        view! {
            <select
                class=classes
                id=id
                name=name
                disabled=disabled
                required=required
                prop:value=value
                on:change=move |ev| if let Some(cb) = on_change { cb.run(ev) }
                on:focus=move |ev| if let Some(cb) = on_focus { cb.run(ev) }
                on:click=move |ev| if let Some(cb) = on_click { cb.run(ev) }
            >
                {children_any}
            </select>
        }.into_any()
    }
}
