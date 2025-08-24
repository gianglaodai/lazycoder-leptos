use leptos::prelude::*;
use time::Time;

#[component]
pub fn TimePicker(
    #[prop(optional, into)] selected: Option<Signal<Option<Time>>>,
    #[prop(optional)] default_selected: Option<Time>,
    #[prop(into, optional)] on_change: Option<Callback<Time, ()>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, default = 5)] _minute_step: u8,
) -> impl IntoView {
    // Controlled/uncontrolled selected state handling
    let (sel_sig, set_sel) = match selected {
        Some(sig) => {
            let setter = Callback::new(move |t: Option<Time>| {
                if let (Some(cb), Some(tt)) = (&on_change, t) {
                    cb.run(tt);
                }
            });
            (sig, setter)
        }
        None => {
            let (s, set) = signal(default_selected);
            let cb = on_change.clone();
            let setter = Callback::new(move |t: Option<Time>| {
                set.set(t);
                if let (Some(cb), Some(tt)) = (&cb, t) {
                    cb.run(tt);
                }
            });
            (s.into(), setter)
        }
    };

    // Text value bound to the input (HH:MM:SS)
    let (text, set_text) = signal(String::new());
    create_effect(move |_| {
        let v = sel_sig.get();
        if let Some(t) = v {
            set_text.set(format!(
                "{:02}:{:02}:{:02}",
                t.hour(),
                t.minute(),
                t.second()
            ));
        } else {
            set_text.set(String::new());
        }
    });

    let placeholder_text = placeholder.unwrap_or_else(|| "Pick a time".to_string());

    // Parse input "HH:MM" or "HH:MM:SS"
    fn parse_time_str(s: &str) -> Option<Time> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 2 {
            return None;
        }
        let h: u8 = parts.get(0)?.parse().ok()?;
        let m: u8 = parts.get(1)?.parse().ok()?;
        let sec: u8 = if let Some(p) = parts.get(2) {
            p.parse().ok()?
        } else {
            0
        };
        Time::from_hms(h, m, sec).ok()
    }

    let input_classes = move || {
        crate::cn!(
            // base Input styles from our Input component + shadcn additions
            "flex w-full rounded-md border border-input bg-background px-3 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50 h-9 py-1",
            "appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none",
            class.clone().unwrap_or_default()
        )
    };

    view! {
        <div class="flex flex-col gap-3">
            <crate::pages::components::Label class="px-1">{"Time"}</crate::pages::components::Label>
            <input
                class=input_classes()
                r#type="time"
                step="1"
                placeholder=placeholder_text
                prop:value=text
                on:input=move |ev| {
                    let v = event_target_value(&ev);
                    set_text.set(v.clone());
                    if let Some(t) = parse_time_str(&v) {
                        set_sel.run(Some(t));
                    }
                }
            />
        </div>
    }
}
