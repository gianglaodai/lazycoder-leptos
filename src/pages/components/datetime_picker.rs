use leptos::prelude::*;
use time::PrimitiveDateTime;

// Keep the simple API, styled like shadcn, using a native input type="datetime-local"
#[component]
pub fn DateTimePicker(
    #[prop(optional, into)] selected: Option<Signal<Option<PrimitiveDateTime>>>,
    #[prop(optional)] default_selected: Option<PrimitiveDateTime>,
    #[prop(into, optional)] on_change: Option<Callback<PrimitiveDateTime, ()>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, default = 5)] _minute_step: u8,
    // legacy props kept for compatibility but ignored in this simplified version
    #[prop(into, optional)] _date_disabled: Option<Callback<time::Date, bool>>,
    #[prop(optional)] _caption_layout: (),
) -> impl IntoView {
    // Controlled/uncontrolled selected state handling
    let (sel_sig, set_sel) = match selected {
        Some(sig) => {
            let setter = Callback::new(move |dt: Option<PrimitiveDateTime>| {
                if let (Some(cb), Some(v)) = (&on_change, dt) {
                    cb.run(v);
                }
            });
            (sig, setter)
        }
        None => {
            let (s, set) = signal(default_selected);
            let cb = on_change.clone();
            let setter = Callback::new(move |dt: Option<PrimitiveDateTime>| {
                set.set(dt);
                if let (Some(cb), Some(v)) = (&cb, dt) {
                    cb.run(v);
                }
            });
            (s.into(), setter)
        }
    };

    // Text binding that mirrors the selected datetime in YYYY-MM-DDTHH:MM[:SS]
    let (text, set_text) = signal(String::new());
    Effect::new(move |_| {
        if let Some(pdt) = sel_sig.get() {
            // Prefer seconds when present
            let t = pdt.time();
            if t.second() > 0 {
                set_text.set(format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                    pdt.date().year(),
                    pdt.date().month() as u8,
                    pdt.date().day(),
                    t.hour(),
                    t.minute(),
                    t.second()
                ));
            } else {
                set_text.set(format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}",
                    pdt.date().year(),
                    pdt.date().month() as u8,
                    pdt.date().day(),
                    t.hour(),
                    t.minute()
                ));
            }
        } else {
            set_text.set(String::new());
        }
    });

    let placeholder_text = placeholder.unwrap_or_else(|| "Pick date & time".to_string());

    // shadcn-like input classes (mirroring our Input component styles)
    let input_classes = move || {
        crate::cn!(
            "flex w-full rounded-md border border-input bg-background px-3 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50 h-9 py-1",
            "appearance-none [&::-webkit-calendar-picker-indicator]:opacity-50",
            class.clone().unwrap_or_default()
        )
    };

    // Parse datetime-local string to PrimitiveDateTime
    fn parse_pdt(s: &str) -> Option<PrimitiveDateTime> {
        let f1 = time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]");
        let f2 = time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        if let Ok(v) = PrimitiveDateTime::parse(s, &f1) {
            return Some(v);
        }
        if let Ok(v) = PrimitiveDateTime::parse(s, &f2) {
            return Some(v);
        }
        None
    }

    view! {
        <input
            class=input_classes()
            r#type="datetime-local"
            step="1"
            placeholder=placeholder_text
            prop:value=text
            on:input=move |ev| {
                let v = event_target_value(&ev);
                set_text.set(v.clone());
                if let Some(pdt) = parse_pdt(&v) {
                    set_sel.run(Some(pdt));
                }
            }
        />
    }
}
