use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

#[component]
pub fn Separator(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] decorative: bool,
) -> impl IntoView {
    let o = match orientation {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    };
    let classes = move || {
        let extra = class.clone().unwrap_or_default();
        let base = "bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px";
        if extra.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, extra)
        }
    };
    view! {
        <div role=move || if decorative { Some("none") } else { Some("separator") }
             aria-orientation=o
             data-slot="separator"
             class=classes />
    }
}
