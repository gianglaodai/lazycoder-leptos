use leptos::prelude::*;
use time::{Date, Month};

use crate::pages::components::button::{Button, ButtonVariant};
use crate::pages::components::calendar::Calendar;
use crate::pages::components::popover::{Popover, PopoverContent, PopoverTrigger};

fn calendar_icon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="16" y1="2" x2="16" y2="6"></line>
            <line x1="8" y1="2" x2="8" y2="6"></line>
            <line x1="3" y1="10" x2="21" y2="10"></line>
        </svg>
    }
}

fn format_date(d: Date) -> String {
    let month = match d.month() {
        Month::January => "January",
        Month::February => "February",
        Month::March => "March",
        Month::April => "April",
        Month::May => "May",
        Month::June => "June",
        Month::July => "July",
        Month::August => "August",
        Month::September => "September",
        Month::October => "October",
        Month::November => "November",
        Month::December => "December",
    };
    format!("{} {}, {}", month, d.day(), d.year())
}

#[component]
pub fn DatePicker(
    #[prop(optional, into)] selected: Option<Signal<Option<Date>>>,
    #[prop(optional)] default_selected: Option<Date>,
    #[prop(into, optional)] on_change: Option<Callback<Date, ()>>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] disabled: Option<Callback<Date, bool>>,
) -> impl IntoView {
    // Controlled/uncontrolled selected state handling
    let (sel_sig, set_sel) = match selected {
        Some(sig) => {
            // In controlled mode we do not own the state; call on_change only
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

    // Popover open state
    let (open, set_open) = signal(false);

    let placeholder_text = placeholder.unwrap_or_else(|| "Pick a date".to_string());
    let trigger_classes = move || {
        // shadcn example uses w-[280px] justify-start text-left font-normal
        crate::cn!(
            "w-[280px] justify-start text-left font-normal",
            class.clone().unwrap_or_default()
        )
    };

    // Handle selection from Calendar: set and close popover
    let on_select = {
        let set_open = set_open.clone();
        Callback::new(move |d: Date| {
            set_sel.run(Some(d));
            set_open.set(false);
        })
    };

    // Build disabled predicate: default is all enabled if not provided
    let disabled_fn = {
        let disabled_opt = disabled.clone();
        Callback::new(move |d: Date| disabled_opt.as_ref().map(|cb| cb.run(d)).unwrap_or(false))
    };

    view! {
        <Popover open=open.into() on_open_change=Callback::new(move |v| set_open.set(v))>
            <PopoverTrigger>
                <Button variant=ButtonVariant::Outline class=trigger_classes()>
                    {calendar_icon()}
                    <span class="ml-2">
                        {move || match sel_sig.get() {
                            Some(d) => format_date(d),
                            None => placeholder_text.clone(),
                        }}
                    </span>
                </Button>
            </PopoverTrigger>
            <PopoverContent class="p-0">
                <Calendar selected=sel_sig on_change=on_select disabled=disabled_fn />
            </PopoverContent>
        </Popover>
    }
}
