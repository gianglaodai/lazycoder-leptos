use leptos::prelude::*;
use time::{Date, Duration, Month, Weekday};
use crate::cn;
use crate::pages::components::button::{Button, ButtonSize, ButtonVariant};

fn always_enabled() -> Callback<Date, bool> {
    Callback::new(|_| false)
}

fn calendar_base_cls() -> &'static str {
    // shadcn/ui Calendar container baseline
    "p-3"
}

fn header_cls() -> &'static str {
    "flex items-center justify-between mb-4"
}

fn nav_btn_cls() -> &'static str {
    // mimic shadcn button-ghost small square icon button
    "inline-flex h-8 w-8 items-center justify-center rounded-md border border-transparent hover:bg-accent hover:text-accent-foreground text-sm"
}

fn month_title_cls() -> &'static str {
    "text-sm font-medium"
}

fn weekdays_row_cls() -> &'static str {
    "grid grid-cols-7 gap-1 mb-1"
}

fn weekday_cell_cls() -> &'static str {
    "text-muted-foreground text-[0.8rem] font-normal w-8 h-8 inline-flex items-center justify-center"
}

fn days_grid_cls() -> &'static str {
    "grid grid-cols-7 gap-1"
}

fn day_btn_base_cls() -> &'static str {
    // baseline for each day button
    "relative inline-flex h-8 w-8 items-center justify-center rounded-md text-sm font-normal focus:outline-none focus:ring-2 focus:ring-ring"
}

fn weekday_labels() -> [&'static str; 7] {
    // Sunday-first, typical shadcn default
    ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]
}

fn month_name(month: Month) -> &'static str {
    match month {
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
    }
}

fn month_from_u8(n: u8) -> Month {
    match n {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => Month::January,
    }
}

fn weekday_sunday_index(w: Weekday) -> u8 {
    match w {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    }
}

fn days_in_month(year: i32, month: Month) -> u8 {
    // Try from 31 down to 28
    for d in (28..=31).rev() {
        if Date::from_calendar_date(year, month, d as u8).is_ok() {
            return d as u8;
        }
    }
    30 // fallback (should never hit)
}

fn add_month(year: i32, month: Month, delta: i32) -> (i32, Month) {
    let mut y = year;
    // Month is handled via explicit mapping; avoid relying on discriminant values.
    // Convert month -> number 1..12
    let mut mi = match month {
        Month::January => 1,
        Month::February => 2,
        Month::March => 3,
        Month::April => 4,
        Month::May => 5,
        Month::June => 6,
        Month::July => 7,
        Month::August => 8,
        Month::September => 9,
        Month::October => 10,
        Month::November => 11,
        Month::December => 12,
    } as i32;
    mi += delta;
    while mi > 12 {
        mi -= 12;
        y += 1;
    }
    while mi < 1 {
        mi += 12;
        y -= 1;
    }
    let m2 = match mi {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        _ => Month::December,
    };
    (y, m2)
}

fn is_same_date(a: &Date, b: &Date) -> bool {
    a == b
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CaptionLayout {
    Label,
    Dropdown,
}

/// Calendar component, shadcn-style
///
/// Props:
/// - selected: controlled selection (optional). If provided, component is controlled.
/// - default_selected: initial value when uncontrolled.
/// - on_change: callback when a day is selected.
/// - disabled: predicate to disable specific dates.
/// - class: additional classes for container.
/// - show_outside_days: whether to render outside-month days (muted). Default true.
#[component]
pub fn Calendar(
    #[prop(optional, into)] selected: Option<Signal<Option<Date>>>,
    #[prop(optional)] default_selected: Option<Date>,
    #[prop(into, optional)] on_change: Option<Callback<Date, ()>>,
    #[prop(into, optional, default = always_enabled())] disabled: Callback<Date, bool>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, default = true)] show_outside_days: bool,
    #[prop(optional, default = CaptionLayout::Label)] caption_layout: CaptionLayout,
    #[prop(optional, default = true)] show_navigation: bool,
) -> impl IntoView {
    // Controlled/uncontrolled selected state
    let (sel_signal, set_sel) = {
        if let Some(sig) = selected {
            let setter = Callback::new(move |d: Option<Date>| {
                if let Some(cb) = on_change {
                    if let Some(dd) = d {
                        cb.run(dd);
                    }
                }
            });
            (sig, setter)
        } else {
            let (s, set) = signal(default_selected);
            let cb = on_change.clone();
            let setter = Callback::new(move |d: Option<Date>| {
                set.set(d);
                if let Some(cb) = cb {
                    if let Some(dd) = d {
                        cb.run(dd);
                    }
                }
            });
            (s.into(), setter)
        }
    };

    // Current visible month
    // Initial visible month: selected -> default_selected -> 1970-01-01
    let initial = sel_signal
        .get_untracked()
        .or(default_selected)
        .unwrap_or_else(|| Date::from_calendar_date(1970, Month::January, 1).unwrap());

    let (cur_year, cur_month) = (initial.year(), initial.month());
    let (year, set_year) = signal(cur_year);
    let (month, set_month) = signal(cur_month);


    let go_prev = move |_| {
        let (y, m) = add_month(year.get_untracked(), month.get_untracked(), -1);
        set_year.set(y);
        set_month.set(m);
    };
    let go_next = move |_| {
        let (y, m) = add_month(year.get_untracked(), month.get_untracked(), 1);
        set_year.set(y);
        set_month.set(m);
    };

    // Build a vector of 42 dates (6 weeks) to render
    let weeks = Memo::new(move |_| {
        let y = year.get();
        let m = month.get();
        let first = Date::from_calendar_date(y, m, 1).unwrap();
        let first_wi = weekday_sunday_index(first.weekday());
        let start = first - Duration::days(first_wi as i64);
        // 6*7 days
        let mut days = Vec::with_capacity(42);
        for i in 0..42i64 {
            days.push(start + Duration::days(i));
        }
        days
    });

    let class = class.unwrap_or_default();

    // Determine today (best effort: we won't use JS Date; optional: highlight if matches)
    // For correctness within Rust `time` only, we skip real today and only highlight selected.

    let disabled_cb = disabled.clone();

    let render_day = move |d: Date| {
        let in_month = d.month() == month.get() && d.year() == year.get();
        let selected_opt = sel_signal.get();
        let is_selected = selected_opt.map(|s| is_same_date(&s, &d)).unwrap_or(false);
        let is_disabled = disabled_cb.run(d);

        let mut classes = crate::cn!(day_btn_base_cls());
        if is_selected {
            classes = crate::cn!(
                classes,
                "bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground"
            );
        } else {
            classes = crate::cn!(classes, "hover:bg-accent hover:text-accent-foreground");
        }
        if !in_month {
            classes = crate::cn!(classes, "text-muted-foreground opacity-50");
        }
        if is_disabled {
            classes = crate::cn!(classes, "opacity-50 pointer-events-none");
        }

        let day_num = d.day() as i32;
        view! {
            <Button
                class=classes.clone()
                variant=crate::pages::components::button::ButtonVariant::Ghost
                size=crate::pages::components::button::ButtonSize::Icon
                disabled=is_disabled
                on_click=Callback::new(move |_| {
                    if !is_disabled {
                        set_sel.run(Some(d));
                    }
                })
            >{day_num}</Button>
        }
    };

    view! {
        <div class=move || cn!(calendar_base_cls(), class.clone())>
            // Header
            <div class=header_cls()>
                <Show when=move || show_navigation>
                    <Button
                        class=nav_btn_cls().to_string()
                        variant=crate::pages::components::button::ButtonVariant::Ghost
                        size=crate::pages::components::button::ButtonSize::Icon
                        on_click=Callback::new(move |_| go_prev(()))
                    >
                        {"\u{2039}"}
                    </Button>
                </Show>
                <div class=month_title_cls()>
                    {move || {
                        if matches!(caption_layout, CaptionLayout::Dropdown) {
                            // Render month/year dropdowns
                            let y = year.get();
                            let m = month.get() as u8;
                            view! {
                                <div class="flex items-center justify-center gap-1.5 text-sm font-medium">
                                    <select
                                        class="h-8 rounded-md border border-input bg-background px-2 text-sm"
                                        prop:value=move || format!("{}", m)
                                        on:change=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<u8>() {
                                                set_month.set(month_from_u8(v));
                                            }
                                        }
                                    >
                                        {(1u8..=12u8).map(|i| {
                                            let label = month_name(month_from_u8(i));
                                            view! { <option value=move || format!("{}", i) selected=move || i == month.get() as u8>{label}</option> }
                                        }).collect_view()}
                                    </select>
                                    <select
                                        class="h-8 rounded-md border border-input bg-background px-2 text-sm"
                                        prop:value=move || format!("{}", y)
                                        on:change=move |ev| {
                                            if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                                                set_year.set(v);
                                            }
                                        }
                                    >
                                        {let yc = year.get();
                                        ((yc-50)..=(yc+50)).map(|yy| {
                                            view! { <option value=move || format!("{}", yy) selected=move || yy == year.get()>{yy}</option> }
                                        }).collect_view()}
                                    </select>
                                </div>
                            }.into_any()
                        } else {
                            let y = year.get();
                            let m = month.get();
                            format!("{} {}", month_name(m), y).into_any()
                        }
                    }}
                </div>
                <Show when=move || show_navigation>
                    <Button
                        class=nav_btn_cls().to_string()
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Icon
                        on_click=Callback::new(move |_| go_next(()))
                    >
                        {"\u{203A}"}
                    </Button>
                </Show>
            </div>

            // Weekdays
            <div class=weekdays_row_cls()>
                {weekday_labels().into_iter().map(|w| view!{ <div class=weekday_cell_cls()>{w}</div> }).collect_view()}
            </div>

            // Days
            <div class=days_grid_cls()>
                {move || weeks.get().into_iter().map(|d| {
                    let in_month = d.month() == month.get() && d.year() == year.get();
                    view! {
                        <Show
                            when=move || show_outside_days || in_month
                            fallback=move || view! { <div class="h-8 w-8"></div> }
                        >
                            { render_day(d) }
                        </Show>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
