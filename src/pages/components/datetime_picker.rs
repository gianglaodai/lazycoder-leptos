use leptos::prelude::*;
use time::{Date, PrimitiveDateTime, Time};

use crate::pages::components::button::{Button, ButtonVariant};
use crate::pages::components::calendar::{Calendar, CaptionLayout};
use crate::pages::components::popover::{Popover, PopoverContent, PopoverTrigger};

fn calendar_clock_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="16" y1="2" x2="16" y2="6"></line>
            <line x1="8" y1="2" x2="8" y2="6"></line>
            <line x1="3" y1="10" x2="21" y2="10"></line>
            <circle cx="18" cy="18" r="4"></circle>
            <path d="M18 16v2l1 1"></path>
        </svg>
    }
}

fn format_datetime(dt: PrimitiveDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        dt.date().year(),
        dt.date().month() as u8,
        dt.date().day(),
        dt.time().hour(),
        dt.time().minute()
    )
}

fn col_container_cls() -> &'static str {
    "max-h-48 overflow-auto pr-1"
}

fn item_btn_cls(active: bool) -> String {
    let base =
        "w-14 rounded-sm px-2 py-1 text-sm text-left hover:bg-accent hover:text-accent-foreground";
    if active {
        crate::cn!(
            base,
            "bg-primary text-primary-foreground hover:bg-primary/90"
        )
    } else {
        base.to_string()
    }
}

#[component]
pub fn DateTimePicker(
    #[prop(optional, into)] selected: Option<Signal<Option<PrimitiveDateTime>>>,
    #[prop(optional)] default_selected: Option<PrimitiveDateTime>,
    #[prop(into, optional)] on_change: Option<Callback<PrimitiveDateTime, ()>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, default = 5)] minute_step: u8,
    #[prop(into, optional)] date_disabled: Option<Callback<Date, bool>>,
    #[prop(optional, default = CaptionLayout::Label)] caption_layout: CaptionLayout,
) -> impl IntoView {
    // Internal state: separate date and time selections
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

    let (open, set_open) = signal(false);

    // Derived date and time pieces
    let (date_sig, set_date) = signal::<Option<Date>>(sel_sig.get_untracked().map(|d| d.date()));
    let (time_sig, set_time) = signal::<Option<Time>>(sel_sig.get_untracked().map(|d| d.time()));

    // When either part changes, push combined selection
    let propagate = move || match (date_sig.get_untracked(), time_sig.get_untracked()) {
        (Some(d), Some(t)) => set_sel.run(Some(PrimitiveDateTime::new(d, t))),
        _ => set_sel.run(None),
    };

    let on_select_date = {
        let set_date = set_date.clone();
        let propagate = propagate.clone();
        Callback::new(move |d: Date| {
            set_date.set(Some(d));
            propagate();
        })
    };

    let on_select_time = {
        let set_time = set_time.clone();
        let propagate = propagate.clone();
        Callback::new(move |t: Time| {
            set_time.set(Some(t));
            propagate();
        })
    };

    let placeholder_text = placeholder.unwrap_or_else(|| "Pick date & time".to_string());
    let class_for_trigger = class.clone();
    let trigger_classes = move || {
        crate::cn!(
            "w-[300px] justify-start text-left font-normal",
            class_for_trigger.clone().unwrap_or_default()
        )
    };

    // Build hours/mins just like TimePicker
    let hours: Vec<u8> = (0..=23).collect();
    let step = if minute_step == 0 {
        1
    } else {
        minute_step.min(30)
    };
    let minutes: Vec<u8> = (0..60).step_by(step as usize).map(|m| m as u8).collect();

    let selected_hour = Memo::new(move |_| time_sig.get().map(|t| t.hour()));
    let selected_min = Memo::new(move |_| time_sig.get().map(|t| t.minute()));

    // Build disabled predicate: default is all enabled if not provided
    let disabled_fn = {
        let disabled_opt = date_disabled.clone();
        Callback::new(move |d: Date| disabled_opt.as_ref().map(|cb| cb.run(d)).unwrap_or(false))
    };

    // Time input text binding (HH:MM:SS)
    let (time_text, set_time_text) = signal(String::new());
    Effect::new(move |_| {
        if let Some(t) = time_sig.get() {
            set_time_text.set(format!(
                "{:02}:{:02}:{:02}",
                t.hour(),
                t.minute(),
                t.second()
            ));
        } else {
            set_time_text.set(String::new());
        }
    });

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

    let class_for_input = class.clone();
    let input_classes = move || {
        crate::cn!(
            "flex w-full rounded-md border border-input bg-background px-3 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50 h-9 py-1",
            "appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none",
            class_for_input.clone().unwrap_or_default()
        )
    };

    let date_button_label = move || {
        let date_part = date_sig.get();
        let time_part = time_sig.get();
        match (date_part, time_part) {
            (Some(d), Some(t)) => {
                if t.second() > 0 {
                    format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                        d.year(),
                        d.month() as u8,
                        d.day(),
                        t.hour(),
                        t.minute(),
                        t.second()
                    )
                } else {
                    format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}",
                        d.year(),
                        d.month() as u8,
                        d.day(),
                        t.hour(),
                        t.minute()
                    )
                }
            }
            (Some(d), None) => format!("{:04}-{:02}-{:02}", d.year(), d.month() as u8, d.day()),
            (None, Some(t)) => {
                if t.second() > 0 {
                    format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second())
                } else {
                    format!("{:02}:{:02}", t.hour(), t.minute())
                }
            }
            (None, None) => "Pick date & time".to_string(),
        }
    };

    view! {
        <Popover open=open.into() on_open_change=Callback::new(move |v| set_open.set(v))>
            <PopoverTrigger>
                {let _class_for_button = class.clone();
                view!{<Button variant=ButtonVariant::Outline class=trigger_classes()>
                    {move || date_button_label()}
                    <span class="ml-2">{"▾"}</span>
                </Button>}}
            </PopoverTrigger>
            <PopoverContent class="w-auto overflow-hidden p-0">
                <div class="flex gap-3 p-3">
                    <Calendar
                        selected=date_sig
                        on_change={
                            Callback::new(move |d: Date| { set_date.set(Some(d)); propagate(); })
                        }
                        disabled=disabled_fn
                        caption_layout=caption_layout
                        show_navigation=matches!(caption_layout, CaptionLayout::Label)
                    />
                    <div class="flex flex-col gap-2 min-w-[140px]">
                        <label class="text-xs text-muted-foreground">Time</label>
                        <input
                            class=input_classes()
                            r#type="time"
                            step="1"
                            placeholder=placeholder_text
                            prop:value=time_text
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                set_time_text.set(v.clone());
                                if let Some(t) = parse_time_str(&v) {
                                    on_select_time.run(t);
                                }
                            }
                        />
                    </div>
                </div>
            </PopoverContent>
        </Popover>
    }
}
