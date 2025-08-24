use leptos::prelude::*;

use crate::pages::components::{DatePicker, DateTimePicker, Input, Select, TimePicker};
use crate::value_data_type::ValueDataType;

/// ColumnFilter renders operator Select + value Input for a given field
/// Now a dumb component: parent provides current operator/value signals and change callbacks
#[component]
pub fn ColumnFilter(
    field_name: String,
    field_datatype: ValueDataType,
    // provided by parent (DataTable)
    operator: Signal<String>,
    value: Signal<String>,
    on_operator_change: Callback<String, ()>,
    on_value_change: Callback<String, ()>,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "flex items-center gap-1".to_string());

    // Operator options by datatype (minimal set for now)
    let op_options = move || -> Vec<(&'static str, &'static str)> {
        match field_datatype {
            ValueDataType::String => vec![("=", "="), ("~", "~")],
            _ => vec![("=", "=")],
        }
    };

    view! {
        <div class=class>
            <Select
                class="h-7 text-xs w-16"
                value=operator
                on_change=Callback::new({
                    let on_op = on_operator_change.clone();
                    move |ev: leptos::ev::Event| {
                        let op = event_target_value(&ev);
                        on_op.run(op);
                    }
                })
            >
                {view! {{
                    let opts = op_options();
                    let mut vs: Vec<AnyView> = Vec::new();
                    for (label, val) in opts.into_iter() {
                        vs.push(view!{ <option value=val.to_string()>{label.to_string()}</option> }.into_any());
                    }
                    vs.into_view()
                }}}
            </Select>
            { // render value input based on datatype
                match field_datatype {
                    ValueDataType::String => {
                        view! {
                            <Input
                                placeholder="Filter..."
                                class="h-7 text-xs"
                                value=value
                                on_input=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |ev: leptos::ev::Event| {
                                        on_val.run(event_target_value(&ev));
                                    }
                                })
                            />
                        }.into_any()
                    }
                    ValueDataType::Int => {
                        view! {
                            <Input
                                placeholder="0"
                                class="h-7 text-xs w-24"
                                r#type="number"
                                value=value
                                on_input=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |ev: leptos::ev::Event| {
                                        on_val.run(event_target_value(&ev));
                                    }
                                })
                            />
                        }.into_any()
                    }
                    ValueDataType::Float => {
                        view! {
                            <Input
                                placeholder="0.0"
                                class="h-7 text-xs w-28"
                                r#type="number"
                                value=value
                                on_input=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |ev: leptos::ev::Event| {
                                        on_val.run(event_target_value(&ev));
                                    }
                                })
                            />
                        }.into_any()
                    }
                    ValueDataType::Bool => {
                        view! {
                            <Select
                                class="h-7 text-xs w-20"
                                value=value
                                on_change=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |ev: leptos::ev::Event| {
                                        on_val.run(event_target_value(&ev));
                                    }
                                })
                            >
                                <option value="">{""}</option>
                                <option value="true">{"true"}</option>
                                <option value="false">{"false"}</option>
                            </Select>
                        }.into_any()
                    }
                    ValueDataType::Date => {
                        view! {
                            <DatePicker
                                on_change=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |d: time::Date| {
                                        let v = format!("{:04}-{:02}-{:02}", d.year(), d.month() as u8, d.day());
                                        on_val.run(v);
                                    }
                                })
                                placeholder="Pick a date"
                                class="h-7 text-xs"
                            />
                        }.into_any()
                    }
                    ValueDataType::DateTime => {
                        view! {
                            <DateTimePicker
                                caption_layout=crate::pages::components::calendar::CaptionLayout::Dropdown
                                on_change=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |dt: time::PrimitiveDateTime| {
                                        let d = dt.date();
                                        let t = dt.time();
                                        let v = format!(
                                            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                                            d.year(), d.month() as u8, d.day(), t.hour(), t.minute(), t.second()
                                        );
                                        on_val.run(v);
                                    }
                                })
                                placeholder="Pick date & time"
                                class="h-7 text-xs"
                            />
                        }.into_any()
                    }
                    ValueDataType::Time => {
                        view! {
                            <TimePicker
                                on_change=Callback::new({
                                    let on_val = on_value_change.clone();
                                    move |t: time::Time| {
                                        let v = format!("{:02}:{:02}:{:02}", t.hour(), t.minute(), t.second());
                                        on_val.run(v);
                                    }
                                })
                                placeholder="Pick time"
                                class="h-7 text-xs"
                            />
                        }.into_any()
                    }
                }
            }
        </div>
    }
}
