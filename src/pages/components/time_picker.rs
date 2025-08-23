use leptos::prelude::*;
use crate::pages::components::button::{Button, ButtonVariant, ButtonSize};
use crate::pages::components::popover::{Popover, PopoverContent, PopoverTrigger};
use time::Time;

fn clock_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <circle cx="12" cy="12" r="10"></circle>
            <polyline points="12 6 12 12 16 14"></polyline>
        </svg>
    }
}

fn format_time(t: Time) -> String {
    format!("{:02}:{:02}", t.hour(), t.minute())
}

fn col_container_cls() -> &'static str {
    "max-h-48 overflow-auto pr-1"
}

fn item_btn_cls(active: bool) -> String {
    let base = "w-14 rounded-sm px-2 py-1 text-sm text-left hover:bg-accent hover:text-accent-foreground";
    if active {
        crate::cn!(base, "bg-primary text-primary-foreground hover:bg-primary/90")
    } else {
        base.to_string()
    }
}

#[component]
pub fn TimePicker(
    #[prop(optional, into)] selected: Option<Signal<Option<Time>>>,
    #[prop(optional)] default_selected: Option<Time>,
    #[prop(into, optional)] on_change: Option<Callback<Time, ()>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, default = 5)] minute_step: u8,
) -> impl IntoView {
    // Controlled/uncontrolled selected state handling
    let (sel_sig, set_sel) = match selected {
        Some(sig) => {
            // In controlled mode we do not own the state; call on_change only
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

    // Popover open state
    let (open, set_open) = signal(false);

    let placeholder_text = placeholder.unwrap_or_else(|| "Pick a time".to_string());
    let trigger_classes = move || {
        crate::cn!(
            "w-[180px] justify-start text-left font-normal",
            class.clone().unwrap_or_default()
        )
    };

    let on_select_time = {
        let set_open = set_open.clone();
        Callback::new(move |t: Time| {
            set_sel.run(Some(t));
            set_open.set(false);
        })
    };

    // Build hours 0..=23 and minutes in step
    let hours: Vec<u8> = (0..=23).collect();
    let step = if minute_step == 0 { 1 } else { minute_step.min(30) }; // sane bounds
    let minutes: Vec<u8> = (0..60).step_by(step as usize).map(|m| m as u8).collect();

    let selected_hour = Memo::new(move |_| sel_sig.get().map(|t| t.hour()));
    let selected_min = Memo::new(move |_| sel_sig.get().map(|t| t.minute()));

    view! {
        <Popover open=open.into() on_open_change=Callback::new(move |v| set_open.set(v))>
            <PopoverTrigger>
                <Button variant=ButtonVariant::Outline class=trigger_classes() size=ButtonSize::Default>
                    {clock_icon()}
                    <span class="ml-2">
                        {move || match sel_sig.get() {
                            Some(t) => format_time(t),
                            None => placeholder_text.clone(),
                        }}
                    </span>
                </Button>
            </PopoverTrigger>
            <PopoverContent class="p-3">
                <div class="flex gap-3">
                    <div>
                        <div class="mb-2 text-xs text-muted-foreground">{"Hour"}</div>
                        <div class=col_container_cls()>
                            {hours.into_iter().map(|h| {
                                let is_active = move || selected_hour.get().map(|v| v == h).unwrap_or(false);
                                view!{
                                    <button
                                        class=move || item_btn_cls(is_active())
                                        on:click=move |_| {
                                            let min = selected_min.get().unwrap_or(0);
                                            let t = Time::from_hms(h, min, 0).unwrap_or(Time::from_hms(0,0,0).unwrap());
                                            on_select_time.run(t);
                                        }
                                    >{format!("{:02}", h)}</button>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                    <div>
                        <div class="mb-2 text-xs text-muted-foreground">{"Minute"}</div>
                        <div class=col_container_cls()>
                            {minutes.into_iter().map(|m| {
                                let is_active = move || selected_min.get().map(|v| v == m).unwrap_or(false);
                                view!{
                                    <button
                                        class=move || item_btn_cls(is_active())
                                        on:click=move |_| {
                                            let hour = selected_hour.get().unwrap_or(0);
                                            let t = Time::from_hms(hour, m, 0).unwrap_or(Time::from_hms(0,0,0).unwrap());
                                            on_select_time.run(t);
                                        }
                                    >{format!("{:02}", m)}</button>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </PopoverContent>
        </Popover>
    }
}
