use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::components::A;
use std::collections::HashMap;

use crate::utils::tv::{
    CompoundVariant, Tv, TvConfig, TvProps, TvResult, VariantClass, VariantDef,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Primary,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonIntent {
    Default,
    Primary,
    Destructive,
    Secondary,
}

impl Default for ButtonIntent {
    fn default() -> Self {
        ButtonIntent::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}

fn button_tv() -> Tv {
    // Base (close to shadcn/ui)
    let base = VariantClass::All(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50".to_string(),
    );

    // Variants definitions
    let mut variants: HashMap<String, VariantDef> = HashMap::new();

    // appearance
    let mut appearance = VariantDef::new();
    appearance.values.insert(
        "default".into(),
        VariantClass::All("bg-primary text-primary-foreground hover:bg-primary/90".into()),
    );
    appearance.values.insert(
        "outline".into(),
        VariantClass::All(
            "border border-input bg-background text-foreground hover:bg-accent hover:text-accent-foreground".into(),
        ),
    );
    appearance.values.insert(
        "ghost".into(),
        VariantClass::All("hover:bg-accent hover:text-accent-foreground".into()),
    );
    appearance.values.insert(
        "link".into(),
        VariantClass::All("text-primary underline-offset-4 hover:underline".into()),
    );
    variants.insert("appearance".into(), appearance);

    // intent (tone)
    let mut intent = VariantDef::new();
    intent
        .values
        .insert("default".into(), VariantClass::All("".into()));
    // Tone by itself does not apply filled styles; those are added via compound variants
    intent
        .values
        .insert("primary".into(), VariantClass::All("".into()));
    intent
        .values
        .insert("destructive".into(), VariantClass::All("".into()));
    intent
        .values
        .insert("secondary".into(), VariantClass::All("".into()));
    variants.insert("intent".into(), intent);

    // size
    let mut size = VariantDef::new();
    size.values
        .insert("default".into(), VariantClass::All("h-9 px-4 py-2".into()));
    size.values
        .insert("sm".into(), VariantClass::All("h-8 rounded-md px-3".into()));
    size.values.insert(
        "lg".into(),
        VariantClass::All("h-10 rounded-md px-8".into()),
    );
    size.values
        .insert("icon".into(), VariantClass::All("h-9 w-9".into()));
    variants.insert("size".into(), size);

    // Defaults
    let mut defaults = HashMap::new();
    defaults.insert("appearance".into(), "default".into());
    defaults.insert("intent".into(), "default".into());
    defaults.insert("size".into(), "default".into());

    // Compound variants to override by intent on non-default appearances
    let mut compound: Vec<CompoundVariant> = Vec::new();
    // default (filled) + destructive
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "default".into()),
            ("intent".into(), "destructive".into()),
        ]),
        class: VariantClass::All(
            "bg-destructive text-destructive-foreground hover:bg-destructive/90".into(),
        ),
    });
    // default (filled) + secondary
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "default".into()),
            ("intent".into(), "secondary".into()),
        ]),
        class: VariantClass::All(
            "bg-secondary text-secondary-foreground hover:bg-secondary/80".into(),
        ),
    });
    // outline + destructive
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "outline".into()),
            ("intent".into(), "destructive".into()),
        ]),
        class: VariantClass::All(
            "border-destructive text-destructive hover:bg-destructive/10 hover:text-destructive"
                .into(),
        ),
    });
    // outline + secondary
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "outline".into()),
            ("intent".into(), "secondary".into()),
        ]),
        class: VariantClass::All(
            "border-secondary text-secondary hover:bg-secondary/10 hover:text-secondary".into(),
        ),
    });
    // outline + primary
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "outline".into()),
            ("intent".into(), "primary".into()),
        ]),
        class: VariantClass::All(
            "border-primary text-primary hover:bg-primary/10 hover:text-primary".into(),
        ),
    });
    // ghost + destructive
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "ghost".into()),
            ("intent".into(), "destructive".into()),
        ]),
        class: VariantClass::All("text-destructive hover:bg-destructive/10".into()),
    });
    // ghost + primary
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "ghost".into()),
            ("intent".into(), "primary".into()),
        ]),
        class: VariantClass::All("text-primary hover:bg-primary/10".into()),
    });
    // link + destructive
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "link".into()),
            ("intent".into(), "destructive".into()),
        ]),
        class: VariantClass::All("text-destructive".into()),
    });
    // link + secondary
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "link".into()),
            ("intent".into(), "secondary".into()),
        ]),
        class: VariantClass::All("text-secondary".into()),
    });
    // link + primary (redundant with link base but explicit for completeness)
    compound.push(CompoundVariant {
        when: HashMap::from([
            ("appearance".into(), "link".into()),
            ("intent".into(), "primary".into()),
        ]),
        class: VariantClass::All("text-primary".into()),
    });

    let cfg = TvConfig {
        base,
        variants,
        default_variants: defaults,
        compound_variants: compound,
        slots: None,
    };

    Tv::new(cfg)
}

#[component]
pub fn Button(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] intent: ButtonIntent,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    #[prop(optional)] disabled_signal: Option<Signal<bool>>,
    #[prop(optional)] loading_signal: Option<Signal<bool>>,
    #[prop(into, optional)] href: Option<String>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent, ()>>,
    #[prop(optional)] r#type: Option<String>,
    children: Children,
) -> impl IntoView {
    // Map legacy variant into appearance + intent to keep backward compatibility
    let mut appearance = match variant {
        ButtonVariant::Outline => "outline",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Link => "link",
        _ => "default",
    };
    let tone = match variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Destructive => "destructive",
        ButtonVariant::Secondary => "secondary",
        _ => "default",
    };

    // Allow explicit intent prop to override the mapped tone
    let tone = match intent {
        ButtonIntent::Primary => "primary",
        ButtonIntent::Destructive => "destructive",
        ButtonIntent::Secondary => "secondary",
        ButtonIntent::Default => tone,
    };

    // If variant is Link and we don't have href, we still render a button with link styles
    if matches!(variant, ButtonVariant::Link) {
        appearance = "link";
    }

    let tv = button_tv();

    let mut vmap: HashMap<String, String> = HashMap::new();
    vmap.insert("appearance".into(), appearance.into());
    vmap.insert("intent".into(), tone.into());
    vmap.insert(
        "size".into(),
        match size {
            ButtonSize::Default => "default",
            ButtonSize::Sm => "sm",
            ButtonSize::Lg => "lg",
            ButtonSize::Icon => "icon",
        }
        .into(),
    );

    let props = TvProps {
        variants: vmap,
        class: class.clone(),
        slot_classes: HashMap::new(),
    };

    let classes = match tv.build(&props) {
        TvResult::Single(s) => s,
        TvResult::Slots(_) => String::new(),
    };

    let content = move || {
        let l = loading || loading_signal.as_ref().map(|s| s.get()).unwrap_or(false);
        if l {
            view! {
                <span class="mr-2 inline-block h-4 w-4 animate-spin rounded-full border-2 border-current border-r-transparent"></span>
                {children()}
            }.into_any()
        } else {
            children().into_any()
        }
    };

    match href {
        Some(url) => view! {
            <A href=url attr:class=classes>
                {content()}
            </A>
        }
        .into_any(),
        None => {
            let t = r#type.unwrap_or_else(|| "button".to_string());
            view! {
                <button
                    class=classes
                    disabled=move || {
                        let l = loading_signal.as_ref().map(|s| s.get()).unwrap_or(false);
                        let d = disabled_signal.as_ref().map(|s| s.get()).unwrap_or(false);
                        disabled || loading || l || d
                    }
                    r#type=t
                    on:click=move |ev| if let Some(cb) = on_click { cb.run(ev) }
                >
                    {content()}
                </button>
            }
            .into_any()
        }
    }
}
