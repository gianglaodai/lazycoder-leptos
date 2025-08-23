use leptos::prelude::*;
use time::{Date, Duration, Month, Weekday};

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
    #[prop(optional)] show_outside_days: bool,
) -> impl IntoView {
    // Controlled/uncontrolled selected state
    let (sel_signal, set_sel) = {
        if let Some(sig) = selected {
            let setter = Callback::new(move |d: Option<Date>| {
                if let Some(cb) = on_change { if let Some(dd) = d { cb.run(dd); } }
            });
            (sig, setter)
        } else {
            let (s, set) = signal(default_selected);
            let cb = on_change.clone();
            let setter = Callback::new(move |d: Option<Date>| {
                set.set(d);
                if let Some(cb) = cb { if let Some(dd) = d { cb.run(dd); } }
            });
            (s.into(), setter)
        }
    };

    // Current visible month
    let today = Date::from_calendar_date(1970, Month::January, 1)
        .ok()
        .and_then(|_| Date::from_calendar_date(2025, Month::January, 1).ok());
    // Fallback to system date via JS Date if we wanted, but keep simple: use selected or default or 1970-01-01?
    let initial = sel_signal
        .get_untracked()
        .or(default_selected)
        .unwrap_or_else(|| Date::from_calendar_date(1970, Month::January, 1).unwrap());

    let (cur_year, cur_month) = (initial.year(), initial.month());
    let (year, set_year) = signal(cur_year);
    let (month, set_month) = signal(cur_month);

    let show_outside_days = if show_outside_days { true } else { true }; // default true

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
            classes = crate::cn!(classes, "bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground");
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
            <button
                class=move || classes.clone()
                disabled=is_disabled
                on:click=move |_| {
                    if !is_disabled {
                        set_sel.run(Some(d));
                    }
                }
            >{day_num}</button>
        }
    };

    view! {
        <div class=move || crate::cn!(calendar_base_cls(), class.clone())>
            // Header
            <div class=header_cls()>
                <button class=nav_btn_cls() on:click=go_prev aria-label="Previous month">{"\u{2039}"}</button>
                <div class=month_title_cls()>
                    {move || {
                        let y = year.get();
                        let m = month.get();
                        format!("{} {}", month_name(m), y)
                    }}
                </div>
                <button class=nav_btn_cls() on:click=go_next aria-label="Next month">{"\u{203A}"}</button>
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
