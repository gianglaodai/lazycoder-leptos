use crate::utils::tw::tw_merge;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum VariantClass {
    All(String),
    PerSlot(HashMap<String, String>),
}

impl VariantClass {
    fn empty() -> Self {
        VariantClass::All(String::new())
    }

    fn add_to(
        &self,
        acc: &mut HashMap<String, Vec<String>>,
        slots: Option<&HashMap<String, String>>,
    ) {
        match self {
            VariantClass::All(s) => {
                if let Some(slots_map) = slots {
                    for slot in slots_map.keys() {
                        acc.entry(slot.clone()).or_default().push(s.clone());
                    }
                } else {
                    acc.entry("_".into()).or_default().push(s.clone());
                }
            }
            VariantClass::PerSlot(map) => {
                if let Some(slots_map) = slots {
                    for (slot, cls) in map {
                        if slots_map.contains_key(slot) {
                            acc.entry(slot.clone()).or_default().push(cls.clone());
                        }
                    }
                } else {
                    // nếu không có slots mà lại PerSlot thì bỏ qua (không áp dụng)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariantDef {
    pub values: HashMap<String, VariantClass>,
    pub boolean: bool,
}

impl VariantDef {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            boolean: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CompoundVariant {
    pub when: HashMap<String, String>,
    pub class: VariantClass,
}

#[derive(Clone, Debug)]
pub struct TvConfig {
    pub base: VariantClass,
    pub variants: HashMap<String, VariantDef>,
    pub default_variants: HashMap<String, String>,
    pub compound_variants: Vec<CompoundVariant>,
    pub slots: Option<HashMap<String, String>>,
}

impl TvConfig {
    pub fn new() -> Self {
        Self {
            base: VariantClass::All(String::new()),
            variants: HashMap::new(),
            default_variants: HashMap::new(),
            compound_variants: Vec::new(),
            slots: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TvProps {
    pub variants: HashMap<String, String>,
    pub class: Option<String>,
    pub slot_classes: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum TvResult {
    Single(String),
    Slots(HashMap<String, String>),
}

pub struct Tv {
    cfg: TvConfig,
}

impl Tv {
    pub fn new(cfg: TvConfig) -> Self {
        Self { cfg }
    }

    pub fn build(&self, props: &TvProps) -> TvResult {
        let slots_defined = self.cfg.slots.as_ref();

        let mut acc: HashMap<String, Vec<String>> = HashMap::new();

        match &self.cfg.base {
            VariantClass::All(s) => {
                if let Some(slots) = slots_defined {
                    for slot in slots.keys() {
                        acc.entry(slot.clone()).or_default().push(s.clone());
                    }
                } else {
                    acc.entry("_".into()).or_default().push(s.clone());
                }
            }
            VariantClass::PerSlot(map) => {
                if let Some(slots) = slots_defined {
                    for (slot, s) in map {
                        if slots.contains_key(slot) {
                            acc.entry(slot.clone()).or_default().push(s.clone());
                        }
                    }
                } else {
                    let mut merged = String::new();
                    for s in map.values() {
                        if !merged.is_empty() {
                            merged.push(' ');
                        }
                        merged.push_str(s);
                    }
                    acc.entry("_".into()).or_default().push(merged);
                }
            }
        }

        let mut chosen: HashMap<&str, &str> = HashMap::new();
        for (k, def) in &self.cfg.variants {
            if let Some(v) = props.variants.get(k) {
                chosen.insert(k.as_str(), v.as_str());
            } else if let Some(d) = self.cfg.default_variants.get(k) {
                chosen.insert(k.as_str(), d.as_str());
            } else if def.boolean {
                // nếu boolean mà không set -> coi như "false" (không áp)
            }
        }

        for (k, def) in &self.cfg.variants {
            if def.boolean {
                if let Some(v) = chosen.get(k.as_str()) {
                    if *v == "true" {
                        if let Some(vc) = def.values.get("true") {
                            vc.add_to(&mut acc, slots_defined);
                        }
                    }
                }
            } else if let Some(val) = chosen.get(k.as_str()) {
                if let Some(vc) = def.values.get(*val) {
                    vc.add_to(&mut acc, slots_defined);
                }
            }
        }

        'cv_loop: for cv in &self.cfg.compound_variants {
            for (vk, vv) in &cv.when {
                match chosen.get(vk.as_str()) {
                    Some(ch) if ch == &vv.as_str() => {}
                    _ => {
                        continue 'cv_loop;
                    }
                }
            }
            cv.class.add_to(&mut acc, slots_defined);
        }

        if let Some(slots) = slots_defined {
            if let Some(extra) = &props.class {
                for slot in slots.keys() {
                    if !extra.is_empty() {
                        acc.entry(slot.clone()).or_default().push(extra.clone());
                    }
                }
            }
            for (slot, extra) in &props.slot_classes {
                if !extra.is_empty() && slots.contains_key(slot) {
                    acc.entry(slot.clone()).or_default().push(extra.clone());
                }
            }
        } else if let Some(extra) = &props.class {
            if !extra.is_empty() {
                acc.entry("_".into()).or_default().push(extra.clone());
            }
        }

        if let Some(slots) = slots_defined {
            let mut out: HashMap<String, String> = HashMap::new();
            for slot in slots.keys() {
                let pieces = acc.remove(slot).unwrap_or_default();
                out.insert(slot.clone(), tw_merge(pieces));
            }
            TvResult::Slots(out)
        } else {
            let pieces = acc.remove("_").unwrap_or_default();
            TvResult::Single(tw_merge(pieces))
        }
    }
}
