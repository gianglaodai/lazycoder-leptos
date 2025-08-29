use leptos::prelude::*;
use time::Date;

use crate::pages::components::Input;

#[component]
pub fn DatePicker(
    #[prop(optional, into)] selected: Option<Signal<Option<Date>>>,
    #[prop(optional)] default_selected: Option<Date>,
    #[prop(into, optional)] on_change: Option<Callback<Date, ()>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] _disabled: Option<Callback<Date, bool>>,
) -> impl IntoView {
    // Controlled/uncontrolled selected state handling
    let (sel_sig, set_sel) = match selected {
        Some(sig) => {
            let setter = Callback::new(move |d: Option<Date>| {
                if let (Some(cb), Some(dd)) = (&on_change, d) {
                    cb.run(dd);
                }
            });
            (sig, setter)
        }
        None => {
            let (s, set) = signal(default_selected);
            let cb = on_change.clone();
            let setter = Callback::new(move |d: Option<Date>| {
                set.set(d);
                if let (Some(cb), Some(dd)) = (&cb, d) {
                    cb.run(dd);
                }
            });
            (s.into(), setter)
        }
    };

    // Keep a text binding that mirrors the selected date in YYYY-MM-DD
    let (text, set_text) = signal(String::new());
    Effect::new(move |_| match sel_sig.get() {
        Some(d) => set_text.set(format!(
            "{:04}-{:02}-{:02}",
            d.year(),
            d.month() as u8,
            d.day()
        )),
        None => set_text.set(String::new()),
    });

    let placeholder_text = placeholder.unwrap_or_else(|| "Pick a date".to_string());

    // Explicit Signal<String> for Input value
    let value_sig: Signal<String> = text.into();

    // Parse YYYY-MM-DD to time::Date
    fn parse_date_str(s: &str) -> Option<Date> {
        let fmt = time::macros::format_description!("[year]-[month]-[day]");
        Date::parse(s, &fmt).ok()
    }

    view! {
        <Input
            r#type="date"
            class=class.clone().unwrap_or_default()
            placeholder=placeholder_text
            value=value_sig
            on_input=Callback::new(move |ev| {
                let v = event_target_value(&ev);
                set_text.set(v.clone());
                if let Some(d) = parse_date_str(&v) {
                    set_sel.run(Some(d));
                }
            })
        />
    }
}
