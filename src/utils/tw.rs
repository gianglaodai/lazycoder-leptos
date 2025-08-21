use std::borrow::Cow;
use std::collections::HashSet;

const T: u8 = 1 << 0;
const R: u8 = 1 << 1;
const B: u8 = 1 << 2;
const L: u8 = 1 << 3;
const TRBL: u8 = T | R | B | L;
const XY: u8 = R | L; // dùng cho x
const YX: u8 = T | B; // dùng cho y

#[derive(Debug, Clone, PartialEq, Eq)]
enum Group {
    Simple(&'static str),
    Coverage { family: &'static str, mask: u8 },
}

#[derive(Debug, Clone)]
struct Token {
    raw: String,
    base: String,
    variants_key: String,
    important: bool,
    group: Option<Group>,
}

fn known_modifiers() -> &'static [&'static str] {
    &[
        "sm", "md", "lg", "xl", "2xl",
        "hover", "focus", "active", "disabled", "visited", "first", "last",
        "odd", "even", "checked", "open", "closed", "read-only", "required",
        "invalid",
        "dark", "rtl", "ltr",
        "motion-safe", "motion-reduce", "contrast-more", "contrast-less",
        "group-hover", "group-focus", "peer-hover", "peer-focus",
        "aria-checked", "aria-selected", "aria-expanded", "aria-current",
        "data-open", "data-closed",
    ]
}

fn split_variants(base: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut depth = 0i32;
    for ch in base.chars() {
        match ch {
            '[' => { depth += 1; buf.push(ch); }
            ']' => { depth -= 1; buf.push(ch); }
            ':' if depth == 0 => { parts.push(std::mem::take(&mut buf)); }
            _ => buf.push(ch),
        }
    }
    parts.push(buf);
    parts
}

fn canonicalize_variants(parts: &[String]) -> String {
    if parts.len() <= 1 {
        return String::new();
    }
    let mods: Vec<String> = parts[..parts.len()-1]
        .iter()
        .map(|s| s.trim().to_string())
        .collect();

    let mut set: Vec<String> = mods;
    set.sort_unstable();
    set.dedup();
    set.join(":")
}

fn looks_like_width(s: &str) -> bool {
    if s.is_empty() { return true; }
    let first = s.chars().next().unwrap();
    first.is_ascii_digit() || s.starts_with("px") || s == "0" || s.starts_with('[')
}

fn classify_text(after: &str) -> Group {
    const SIZES: &[&str] = &[
        "xs","sm","base","lg","xl","2xl","3xl","4xl","5xl","6xl","7xl","8xl","9xl","10xl"
    ];
    if SIZES.contains(&after) || after.starts_with('[') && !after.contains('#') && !after.starts_with("[color:") {
        return Group::Simple("text-size");
    }
    match after {
        "left" | "right" | "center" | "justify" | "start" | "end" => {
            return Group::Simple("text-align");
        }
        _ => {}
    }
    Group::Simple("text-color")
}

fn classify_border(base: &str) -> Option<Group> {
    let s = base.strip_prefix('-').unwrap_or(base);
    if s == "border" {
        return Some(Group::Coverage { family: "border-width", mask: TRBL });
    }
    let axis_side = [
        ("border-x-", XY),
        ("border-y-", YX),
        ("border-t-", T),
        ("border-r-", R),
        ("border-b-", B),
        ("border-l-", L),
    ];
    for (pref, mask) in axis_side {
        if let Some(after) = s.strip_prefix(pref) {
            if looks_like_width(after) {
                return Some(Group::Coverage { family: "border-width", mask });
            }
            match after {
                "solid" | "dashed" | "dotted" | "double" | "none" | "hidden" => {
                    return Some(Group::Coverage { family: "border-style", mask });
                }
                _ => return Some(Group::Coverage { family: "border-color", mask }),
            }
        }
    }
    if let Some(after) = s.strip_prefix("border-") {
        if looks_like_width(after) {
            return Some(Group::Coverage { family: "border-width", mask: TRBL });
        }
        match after {
            "solid" | "dashed" | "dotted" | "double" | "none" | "hidden" => {
                return Some(Group::Coverage { family: "border-style", mask: TRBL });
            }
            _ => return Some(Group::Coverage { family: "border-color", mask: TRBL }),
        }
    }
    None
}

fn classify_group(base: &str) -> Option<Group> {
    let s = base.strip_suffix('!').unwrap_or(base);
    let s = s.strip_prefix('!').unwrap_or(s);
    let s = s;

    match s {
        "block" | "inline-block" | "inline" | "flex" | "inline-flex" | "grid" | "inline-grid"
        | "table" | "inline-table" | "contents" | "flow-root" | "hidden" => {
            return Some(Group::Simple("display"));
        }
        _ => {}
    }

    match s {
        "static" | "fixed" | "absolute" | "relative" | "sticky" => {
            return Some(Group::Simple("position"));
        }
        _ => {}
    }

    if s.starts_with("overflow-") {
        if let Some(after) = s.strip_prefix("overflow-x-") {
            let _ = after; return Some(Group::Coverage { family: "overflow", mask: XY });
        }
        if let Some(after) = s.strip_prefix("overflow-y-") {
            let _ = after; return Some(Group::Coverage { family: "overflow", mask: YX });
        }
        return Some(Group::Coverage { family: "overflow", mask: TRBL });
    }

    if let Some(after) = s.strip_prefix("p-") { let _ = after; return Some(Group::Coverage { family: "padding", mask: TRBL }); }
    if let Some(after) = s.strip_prefix("px-") { let _ = after; return Some(Group::Coverage { family: "padding", mask: XY }); }
    if let Some(after) = s.strip_prefix("py-") { let _ = after; return Some(Group::Coverage { family: "padding", mask: YX }); }
    if s.starts_with("pt-") { return Some(Group::Coverage { family: "padding", mask: T }); }
    if s.starts_with("pr-") { return Some(Group::Coverage { family: "padding", mask: R }); }
    if s.starts_with("pb-") { return Some(Group::Coverage { family: "padding", mask: B }); }
    if s.starts_with("pl-") { return Some(Group::Coverage { family: "padding", mask: L }); }

    let ms = s.strip_prefix('-').unwrap_or(s);
    if let Some(after) = ms.strip_prefix("m-") { let _ = after; return Some(Group::Coverage { family: "margin", mask: TRBL }); }
    if let Some(after) = ms.strip_prefix("mx-") { let _ = after; return Some(Group::Coverage { family: "margin", mask: XY }); }
    if let Some(after) = ms.strip_prefix("my-") { let _ = after; return Some(Group::Coverage { family: "margin", mask: YX }); }
    if ms.starts_with("mt-") { return Some(Group::Coverage { family: "margin", mask: T }); }
    if ms.starts_with("mr-") { return Some(Group::Coverage { family: "margin", mask: R }); }
    if ms.starts_with("mb-") { return Some(Group::Coverage { family: "margin", mask: B }); }
    if ms.starts_with("ml-") { return Some(Group::Coverage { family: "margin", mask: L }); }

    if ms.starts_with("inset-") { return Some(Group::Coverage { family: "inset", mask: TRBL }); }
    if ms.starts_with("inset-x-") { return Some(Group::Coverage { family: "inset", mask: XY }); }
    if ms.starts_with("inset-y-") { return Some(Group::Coverage { family: "inset", mask: YX }); }
    if ms.starts_with("top-") { return Some(Group::Coverage { family: "inset", mask: T }); }
    if ms.starts_with("right-") { return Some(Group::Coverage { family: "inset", mask: R }); }
    if ms.starts_with("bottom-") { return Some(Group::Coverage { family: "inset", mask: B }); }
    if ms.starts_with("left-") { return Some(Group::Coverage { family: "inset", mask: L }); }

    if s.starts_with("bg-") {
        return Some(Group::Simple("bg-color"));
    }

    if let Some(after) = s.strip_prefix("text-") {
        return Some(classify_text(after));
    }

    if let Some(after) = s.strip_prefix("font-") {
        match after {
            "thin" | "extralight" | "light" | "normal" | "medium" |
            "semibold" | "bold" | "extrabold" | "black" => {
                return Some(Group::Simple("font-weight"));
            }
            _ => {}
        }
    }

    if s.starts_with("leading-") { return Some(Group::Simple("leading")); }
    if s.starts_with("tracking-") { return Some(Group::Simple("tracking")); }

    if s.starts_with("rounded-") || s == "rounded" {
        let (family, mask) = if s.starts_with("rounded-tl") { ("rounded", T|L) }
        else if s.starts_with("rounded-tr") { ("rounded", T|R) }
        else if s.starts_with("rounded-bl") { ("rounded", B|L) }
        else if s.starts_with("rounded-br") { ("rounded", B|R) }
        else if s.starts_with("rounded-t")  { ("rounded", T) }
        else if s.starts_with("rounded-r")  { ("rounded", R) }
        else if s.starts_with("rounded-b")  { ("rounded", B) }
        else if s.starts_with("rounded-l")  { ("rounded", L) }
        else { ("rounded", TRBL) };
        return Some(Group::Coverage { family, mask });
    }

    if let Some(g) = classify_border(s) { return Some(g); }

    if s.starts_with("shadow") { return Some(Group::Simple("shadow")); }
    if s.starts_with("opacity-") { return Some(Group::Simple("opacity")); }
    if s.starts_with("z-") { return Some(Group::Simple("z-index")); }

    if s.starts_with("flex-row") || s.starts_with("flex-col") {
        return Some(Group::Simple("flex-direction"));
    }
    if s.starts_with("flex-wrap") || s == "flex-nowrap" {
        return Some(Group::Simple("flex-wrap"));
    }
    if s.starts_with("justify-") { return Some(Group::Simple("justify")); }
    if s.starts_with("items-") { return Some(Group::Simple("items")); }
    if s.starts_with("content-") { return Some(Group::Simple("content")); }
    if s.starts_with("gap-") || s.starts_with("gap-x-") || s.starts_with("gap-y-") {
        return Some(Group::Simple("gap"));
    }

    None
}

fn parse_token(raw_piece: &str) -> Option<Token> {
    let raw_piece = raw_piece.trim();
    if raw_piece.is_empty() { return None; }

    let mut important = false;
    let mut s: Cow<str> = Cow::Borrowed(raw_piece);
    if s.ends_with('!') { important = true; s = Cow::Owned(s.trim_end_matches('!').to_string()); }
    if s.starts_with('!') { important = true; s = Cow::Owned(s.trim_start_matches('!').to_string()); }

    let parts = split_variants(&s);
    let base = parts.last().cloned().unwrap_or_default();
    let variants_key = canonicalize_variants(&parts);
    let group = classify_group(&base);

    Some(Token {
        raw: raw_piece.to_string(),
        base,
        variants_key,
        important,
        group,
    })
}

fn conflicts_and_newer_wins(older: &Token, newer: &Token) -> bool {
    if older.important && !newer.important {
        return false;
    }
    if older.variants_key != newer.variants_key || older.important != newer.important {
        return false;
    }

    match (&older.group, &newer.group) {
        (Some(Group::Simple(a)), Some(Group::Simple(b))) if a == b => true,

        (Some(Group::Coverage { family: f1, mask: m1 }),
            Some(Group::Coverage { family: f2, mask: m2 })) if f1 == f2 => {
            (m2 & *m1) == *m1
        }

        _ => false,
    }
}

pub fn tw_merge<I, S>(classes: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut raw_tokens: Vec<String> = Vec::new();
    for part in classes {
        for p in part.as_ref().split_whitespace() {
            raw_tokens.push(p.to_string());
        }
    }

    let mut out: Vec<Token> = Vec::new();

    'NEXT: for raw in raw_tokens {
        let Some(tok) = parse_token(&raw) else { continue };
        let mut to_remove: HashSet<usize> = HashSet::new();
        for (idx, old) in out.iter().enumerate() {
            if conflicts_and_newer_wins(old, &tok) {
                to_remove.insert(idx);
            }
        }
        if !to_remove.is_empty() {
            let mut kept = Vec::with_capacity(out.len() - to_remove.len());
            for (i, t) in out.into_iter().enumerate() {
                if !to_remove.contains(&i) {
                    kept.push(t);
                }
            }
            out = kept;
        }
        out.push(tok);
    }

    out.iter().map(|t| t.raw.as_str()).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::tw_merge;

    #[test]
    fn simple_last_wins() {
        assert_eq!(tw_merge(["block", "inline", "flex"]), "flex");
        assert_eq!(tw_merge(["bg-red-500", "bg-[#B91C1C]"]), "bg-[#B91C1C]");
    }

    #[test]
    fn variants_are_canonicalized() {
        assert_eq!(tw_merge(["hover:focus:p-2", "focus:hover:p-4"]), "focus:hover:p-4");
        assert_eq!(tw_merge(["md:block", "md:inline"]), "md:inline");
    }

    #[test]
    fn important_behavior() {
        // older has ! -> newer non-important cannot override
        assert_eq!(tw_merge(["!bg-red-500", "bg-blue-500"]), "!bg-red-500 bg-blue-500");
        // newer has ! -> can override
        assert_eq!(tw_merge(["bg-red-500", "bg-blue-500!"]), "bg-blue-500!");
    }

    #[test]
    fn padding_refinements() {
        assert_eq!(tw_merge(["pr-5", "p-4"]), "p-4");
        assert_eq!(tw_merge(["p-4", "pr-5"]), "p-4 pr-5");
        assert_eq!(tw_merge(["px-6", "pl-2", "p-4"]), "p-4");
    }

    #[test]
    fn border_width_vs_color() {
        assert_eq!(tw_merge(["border-2", "border-red-500"]), "border-2 border-red-500");
        assert_eq!(tw_merge(["border-red-500", "border-2"]), "border-2");
        assert_eq!(tw_merge(["border-x-2", "border-l-4", "border"]), "border");
    }

    #[test]
    fn rounded_coverage() {
        assert_eq!(tw_merge(["rounded-t-md", "rounded"]), "rounded");
        assert_eq!(tw_merge(["rounded", "rounded-tr-lg"]), "rounded rounded-tr-lg");
    }
}
