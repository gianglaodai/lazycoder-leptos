use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// This module provides shadcn/ui-like form primitives for Leptos.
// Now with lightweight contexts to approximate shadcn's FormField wiring
// (ids, labels, aria attributes, and error styling).

fn form_item_cls() -> &'static str {
    "space-y-2"
}

fn form_description_cls() -> &'static str {
    "text-sm text-muted-foreground"
}

fn form_message_cls() -> &'static str {
    "text-sm font-medium text-destructive"
}

fn form_control_wrapper_cls() -> &'static str {
    "relative"
}

static FORM_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
fn next_form_id() -> String {
    let n = FORM_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("form-item-{}", n)
}

#[derive(Clone)]
struct FormItemContext {
    id: String,
}

#[derive(Clone)]
struct FormFieldContext {
    name: Option<String>,
    // Optional reactive error coming from the form layer
    error: Option<Signal<Option<String>>>,
}

pub struct FormFieldInfo {
    pub id: String,
    pub name: Option<String>,
    pub form_item_id: String,
    pub form_description_id: String,
    pub form_message_id: String,
    pub error: Option<String>,
}

pub fn use_form_field() -> FormFieldInfo {
    let item = use_context::<FormItemContext>()
        .expect("use_form_field should be used within <FormItem>");
    let field = use_context::<FormFieldContext>().unwrap_or(FormFieldContext {
        name: None,
        error: None,
    });

    let id = item.id.clone();
    let name = field.name.clone();
    let form_item_id = format!("{}-form-item", id);
    let form_description_id = format!("{}-form-item-description", id);
    let form_message_id = format!("{}-form-item-message", id);

    let error = field
        .error
        .and_then(|sig| sig.get());

    FormFieldInfo {
        id,
        name,
        form_item_id,
        form_description_id,
        form_message_id,
        error,
    }
}

#[component]
pub fn Form(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(optional)] prevent_default: bool,
    #[prop(into, optional)] on_submit: Option<Callback<leptos::ev::SubmitEvent, ()>>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    let on_submit_handler = move |ev: leptos::ev::SubmitEvent| {
        if prevent_default {
            ev.prevent_default();
        }
        if let Some(cb) = on_submit.clone() {
            cb.run(ev);
        }
    };

    view! {
        <form class=move || crate::cn!(class.clone()) id=id on:submit=on_submit_handler>
            {children()}
        </form>
    }
}

// Provider for field-level metadata, similar to shadcn's <FormField>
#[component]
pub fn FormField(
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] error: Option<Signal<Option<String>>>,
    children: Children,
) -> impl IntoView {
    provide_context(FormFieldContext { name, error });
    view! { {children()} }
}

#[component]
pub fn FormItem(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = next_form_id();
    provide_context(FormItemContext { id: id.clone() });

    view! {
        <div class=move || crate::cn!(form_item_cls(), class.clone())>
            {children()}
        </div>
    }
}

#[component]
pub fn FormLabel(
    #[prop(into, optional)] class: Option<String>,
    // Overridable, but usually bound to the computed form_item_id
    #[prop(into, optional)] html_for: Option<String>,
    children: Children,
) -> impl IntoView {
    let info = use_form_field();
    let class = class.unwrap_or_default();

    let computed_for = html_for.unwrap_or_else(|| info.form_item_id.clone());
    let final_class = crate::cn!(
        if info.error.is_some() { "text-destructive" } else { "" },
        class.clone()
    );

    view! {
        <super::label::Label
            class=final_class
            html_for=computed_for
        >
            {children()}
        </super::label::Label>
    }
}

#[component]
pub fn FormControl(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    // NOTE: In React, shadcn uses a Slot to apply id/aria to the actual control.
    // In Leptos we can't inject props into an arbitrary child, so we place them on a wrapper.
    // Consumers can still read/use computed ids via use_form_field() if needed.
    let info = use_form_field();
    let class = class.unwrap_or_default();

    let describedby = if info.error.is_some() {
        format!("{} {}", info.form_description_id, info.form_message_id)
    } else {
        info.form_description_id.clone()
    };
    let invalid = if info.error.is_some() { "true" } else { "false" };

    view! {
        <div
            id=info.form_item_id.clone()
            class=move || crate::cn!(form_control_wrapper_cls(), class.clone())
            attr:aria-describedby=describedby
            attr:aria-invalid=invalid
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FormDescription(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let info = use_form_field();
    let class = class.unwrap_or_default();

    view! {
        <p id=info.form_description_id.clone() class=move || crate::cn!(form_description_cls(), class.clone())>
            {children()}
        </p>
    }
}

#[component]
pub fn FormMessage(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] message: Option<String>,
    children: Children,
) -> impl IntoView {
    let info = use_form_field();
    let class = class.unwrap_or_default();
    let classes = crate::cn!(form_message_cls(), class.clone());

    if let Some(m) = message {
        return view! { <p id=info.form_message_id class=classes.clone()>{m}</p> }.into_any();
    }

    view! { <p id=info.form_message_id class=classes>{children()}</p> }.into_any()
}
