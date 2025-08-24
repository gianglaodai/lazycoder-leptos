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
        "sm",
        "md",
        "lg",
        "xl",
        "2xl",
        "hover",
        "focus",
        "active",
        "disabled",
        "visited",
        "first",
        "last",
        "odd",
        "even",
        "checked",
        "open",
        "closed",
        "read-only",
        "required",
        "invalid",
        "dark",
        "rtl",
        "ltr",
        "motion-safe",
        "motion-reduce",
        "contrast-more",
        "contrast-less",
        "group-hover",
        "group-focus",
        "peer-hover",
        "peer-focus",
        "aria-checked",
        "aria-selected",
        "aria-expanded",
        "aria-current",
        "data-open",
        "data-closed",
    ]
}

fn split_variants(base: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut depth = 0i32;
    for ch in base.chars() {
        match ch {
            '[' => {
                depth += 1;
                buf.push(ch);
            }
            ']' => {
                depth -= 1;
                buf.push(ch);
            }
            ':' if depth == 0 => {
                parts.push(std::mem::take(&mut buf));
            }
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
    let mods: Vec<String> = parts[..parts.len() - 1]
        .iter()
        .map(|s| s.trim().to_string())
        .collect();

    let mut set: Vec<String> = mods;
    set.sort_unstable();
    set.dedup();
    set.join(":")
}

fn looks_like_width(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    let first = s.chars().next().unwrap();
    first.is_ascii_digit() || s.starts_with("px") || s == "0" || s.starts_with('[')
}

fn classify_text(after: &str) -> Group {
    const SIZES: &[&str] = &[
        "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl",
        "10xl",
    ];
    if SIZES.contains(&after)
        || after.starts_with('[') && !after.contains('#') && !after.starts_with("[color:")
    {
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
        return Some(Group::Coverage {
            family: "border-width",
            mask: TRBL,
        });
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
                return Some(Group::Coverage {
                    family: "border-width",
                    mask,
                });
            }
            match after {
                "solid" | "dashed" | "dotted" | "double" | "none" | "hidden" => {
                    return Some(Group::Coverage {
                        family: "border-style",
                        mask,
                    });
                }
                _ => {
                    return Some(Group::Coverage {
                        family: "border-color",
                        mask,
                    })
                }
            }
        }
    }
    if let Some(after) = s.strip_prefix("border-") {
        if looks_like_width(after) {
            return Some(Group::Coverage {
                family: "border-width",
                mask: TRBL,
            });
        }
        match after {
            "solid" | "dashed" | "dotted" | "double" | "none" | "hidden" => {
                return Some(Group::Coverage {
                    family: "border-style",
                    mask: TRBL,
                });
            }
            _ => {
                return Some(Group::Coverage {
                    family: "border-color",
                    mask: TRBL,
                })
            }
        }
    }
    None
}

fn classify_group(base: &str) -> Option<Group> {
    let s = base.strip_suffix('!').unwrap_or(base);
    let s = s.strip_prefix('!').unwrap_or(s);
    let s = s;

    // Display
    match s {
        "block" | "inline-block" | "inline" | "flex" | "inline-flex" | "grid" | "inline-grid"
        | "datatable" | "inline-datatable" | "contents" | "flow-root" | "hidden" => {
            return Some(Group::Simple("display"));
        }
        _ => {}
    }

    // Position
    match s {
        "static" | "fixed" | "absolute" | "relative" | "sticky" => {
            return Some(Group::Simple("position"));
        }
        _ => {}
    }

    // Visibility
    match s {
        "visible" | "invisible" | "collapse" => {
            return Some(Group::Simple("visibility"));
        }
        _ => {}
    }

    // Overflow
    if s.starts_with("overflow-") {
        if let Some(after) = s.strip_prefix("overflow-x-") {
            let _ = after;
            return Some(Group::Coverage {
                family: "overflow",
                mask: XY,
            });
        }
        if let Some(after) = s.strip_prefix("overflow-y-") {
            let _ = after;
            return Some(Group::Coverage {
                family: "overflow",
                mask: YX,
            });
        }
        return Some(Group::Coverage {
            family: "overflow",
            mask: TRBL,
        });
    }

    // Padding
    if let Some(after) = s.strip_prefix("p-") {
        let _ = after;
        return Some(Group::Coverage {
            family: "padding",
            mask: TRBL,
        });
    }
    if let Some(after) = s.strip_prefix("px-") {
        let _ = after;
        return Some(Group::Coverage {
            family: "padding",
            mask: XY,
        });
    }
    if let Some(after) = s.strip_prefix("py-") {
        let _ = after;
        return Some(Group::Coverage {
            family: "padding",
            mask: YX,
        });
    }
    if s.starts_with("pt-") {
        return Some(Group::Coverage {
            family: "padding",
            mask: T,
        });
    }
    if s.starts_with("pr-") {
        return Some(Group::Coverage {
            family: "padding",
            mask: R,
        });
    }
    if s.starts_with("pb-") {
        return Some(Group::Coverage {
            family: "padding",
            mask: B,
        });
    }
    if s.starts_with("pl-") {
        return Some(Group::Coverage {
            family: "padding",
            mask: L,
        });
    }

    // Margin (allow negative)
    let ms = s.strip_prefix('-').unwrap_or(s);
    if let Some(after) = ms.strip_prefix("m-") {
        let _ = after;
        return Some(Group::Coverage {
            family: "margin",
            mask: TRBL,
        });
    }
    if let Some(after) = ms.strip_prefix("mx-") {
        let _ = after;
        return Some(Group::Coverage {
            family: "margin",
            mask: XY,
        });
    }
    if let Some(after) = ms.strip_prefix("my-") {
        let _ = after;
        return Some(Group::Coverage {
            family: "margin",
            mask: YX,
        });
    }
    if ms.starts_with("mt-") {
        return Some(Group::Coverage {
            family: "margin",
            mask: T,
        });
    }
    if ms.starts_with("mr-") {
        return Some(Group::Coverage {
            family: "margin",
            mask: R,
        });
    }
    if ms.starts_with("mb-") {
        return Some(Group::Coverage {
            family: "margin",
            mask: B,
        });
    }
    if ms.starts_with("ml-") {
        return Some(Group::Coverage {
            family: "margin",
            mask: L,
        });
    }

    // Inset
    if ms.starts_with("inset-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: TRBL,
        });
    }
    if ms.starts_with("inset-x-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: XY,
        });
    }
    if ms.starts_with("inset-y-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: YX,
        });
    }
    if ms.starts_with("top-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: T,
        });
    }
    if ms.starts_with("right-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: R,
        });
    }
    if ms.starts_with("bottom-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: B,
        });
    }
    if ms.starts_with("left-") {
        return Some(Group::Coverage {
            family: "inset",
            mask: L,
        });
    }

    // Backgrounds: color vs other subproperties
    if let Some(after) = s.strip_prefix("bg-") {
        // Attachment
        match after {
            "fixed" | "local" | "scroll" => return Some(Group::Simple("bg-attachment")),
            _ => {}
        }
        // Repeat
        if after.starts_with("repeat") || matches!(after, "no-repeat" | "repeat-x" | "repeat-y") {
            return Some(Group::Simple("bg-repeat"));
        }
        // Size
        if matches!(after, "auto" | "cover" | "contain")
            || after.starts_with("[length")
            || after.starts_with("[size")
        {
            return Some(Group::Simple("bg-size"));
        }
        // Position or arbitrary value discrimination
        if matches!(after, "center" | "top" | "bottom" | "left" | "right")
            || after.starts_with("[position")
        {
            return Some(Group::Simple("bg-position"));
        }
        if after.starts_with('[') {
            // arbitrary value: detect color vs image vs position/size
            let lower = after.to_ascii_lowercase();
            let is_color = lower.contains('#')
                || lower.starts_with("[rgb")
                || lower.starts_with("[hsl")
                || lower.starts_with("[hwb")
                || lower.starts_with("[lch")
                || lower.starts_with("[oklch")
                || lower.starts_with("[oklab")
                || lower.starts_with("[var(")
                || lower.starts_with("[--")
                || lower.starts_with("[color:");
            if is_color {
                return Some(Group::Simple("bg-color"));
            }
            if lower.starts_with("[url(") {
                return Some(Group::Simple("bg-image"));
            }
            // otherwise treat as position
            return Some(Group::Simple("bg-position"));
        }
        // Clip/Origin (note: official classes are bg-clip-* and bg-origin-*)
        if after.starts_with("clip-") {
            return Some(Group::Simple("bg-clip"));
        }
        if after.starts_with("origin-") {
            return Some(Group::Simple("bg-origin"));
        }
        // Gradients
        if after.starts_with("gradient-to-") {
            return Some(Group::Simple("bg-gradient"));
        }
        if after == "none" {
            return Some(Group::Simple("bg-image"));
        }
        // Colors (default)
        return Some(Group::Simple("bg-color"));
    }
    // Gradient color stops
    if s.starts_with("from-") {
        return Some(Group::Simple("from"));
    }
    if s.starts_with("via-") {
        return Some(Group::Simple("via"));
    }
    if s.starts_with("to-") {
        return Some(Group::Simple("to"));
    }

    // Text
    if let Some(after) = s.strip_prefix("text-") {
        return Some(classify_text(after));
    }
    // Font weight
    if let Some(after) = s.strip_prefix("font-") {
        match after {
            "thin" | "extralight" | "light" | "normal" | "medium" | "semibold" | "bold"
            | "extrabold" | "black" => {
                return Some(Group::Simple("font-weight"));
            }
            _ => {}
        }
    }
    // Font style
    if s == "italic" || s == "not-italic" {
        return Some(Group::Simple("font-style"));
    }
    // Text decoration basics
    if matches!(
        s,
        "underline" | "overline" | "line-through" | "no-underline"
    ) {
        return Some(Group::Simple("text-decoration"));
    }
    // Text transform
    if matches!(s, "uppercase" | "lowercase" | "capitalize" | "normal-case") {
        return Some(Group::Simple("text-transform"));
    }
    // Leading/Tracking
    if s.starts_with("leading-") {
        return Some(Group::Simple("leading"));
    }
    if s.starts_with("tracking-") {
        return Some(Group::Simple("tracking"));
    }
    // Whitespace, break, truncate
    if s.starts_with("whitespace-") {
        return Some(Group::Simple("whitespace"));
    }
    if s.starts_with("break-") {
        return Some(Group::Simple("break"));
    }
    if matches!(s, "truncate" | "text-ellipsis" | "text-clip") {
        return Some(Group::Simple("truncate"));
    }

    // Rounding
    if s.starts_with("rounded-") || s == "rounded" {
        let (family, mask) = if s.starts_with("rounded-tl") {
            ("rounded", T | L)
        } else if s.starts_with("rounded-tr") {
            ("rounded", T | R)
        } else if s.starts_with("rounded-bl") {
            ("rounded", B | L)
        } else if s.starts_with("rounded-br") {
            ("rounded", B | R)
        } else if s.starts_with("rounded-t") {
            ("rounded", T)
        } else if s.starts_with("rounded-r") {
            ("rounded", R)
        } else if s.starts_with("rounded-b") {
            ("rounded", B)
        } else if s.starts_with("rounded-l") {
            ("rounded", L)
        } else {
            ("rounded", TRBL)
        };
        return Some(Group::Coverage { family, mask });
    }

    // Borders
    if let Some(g) = classify_border(s) {
        return Some(g);
    }

    // Shadows/opacity/z-index
    if s.starts_with("shadow") {
        return Some(Group::Simple("shadow"));
    }
    if s.starts_with("opacity-") {
        return Some(Group::Simple("opacity"));
    }
    if s.starts_with("z-") {
        return Some(Group::Simple("z-index"));
    }

    // Flexbox & Grid alignment
    if s.starts_with("flex-row") || s.starts_with("flex-col") {
        return Some(Group::Simple("flex-direction"));
    }
    if s.starts_with("flex-wrap") || s == "flex-nowrap" {
        return Some(Group::Simple("flex-wrap"));
    }
    if s.starts_with("justify-") {
        return Some(Group::Simple("justify"));
    }
    if s.starts_with("items-") {
        return Some(Group::Simple("items"));
    }
    if s.starts_with("content-") {
        return Some(Group::Simple("content"));
    }
    if s.starts_with("justify-items-") {
        return Some(Group::Simple("justify-items"));
    }
    if s.starts_with("justify-self-") {
        return Some(Group::Simple("justify-self"));
    }
    if s.starts_with("self-") {
        return Some(Group::Simple("self"));
    }
    if s.starts_with("place-items-") {
        return Some(Group::Simple("place-items"));
    }
    if s.starts_with("place-content-") {
        return Some(Group::Simple("place-content"));
    }
    if s.starts_with("place-self-") {
        return Some(Group::Simple("place-self"));
    }

    // Gap
    if s.starts_with("gap-") || s.starts_with("gap-x-") || s.starts_with("gap-y-") {
        return Some(Group::Simple("gap"));
    }

    // Sizing
    if s.starts_with("w-") {
        return Some(Group::Simple("width"));
    }
    if s.starts_with("h-") {
        return Some(Group::Simple("height"));
    }
    if s.starts_with("min-w-") {
        return Some(Group::Simple("min-width"));
    }
    if s.starts_with("min-h-") {
        return Some(Group::Simple("min-height"));
    }
    if s.starts_with("max-w-") {
        return Some(Group::Simple("max-width"));
    }
    if s.starts_with("max-h-") {
        return Some(Group::Simple("max-height"));
    }

    // Flex sizing
    if s == "grow" || s == "grow-0" {
        return Some(Group::Simple("flex-grow"));
    }
    if s == "shrink" || s == "shrink-0" {
        return Some(Group::Simple("flex-shrink"));
    }
    if s.starts_with("basis-") {
        return Some(Group::Simple("flex-basis"));
    }
    if s.starts_with("order-") {
        return Some(Group::Simple("order"));
    }

    // Grid
    if s.starts_with("grid-cols-") {
        return Some(Group::Simple("grid-cols"));
    }
    if s.starts_with("grid-rows-") {
        return Some(Group::Simple("grid-rows"));
    }
    if s.starts_with("grid-flow-") {
        return Some(Group::Simple("grid-flow"));
    }
    if s.starts_with("col-span-") || s.starts_with("col-start-") || s.starts_with("col-end-") {
        return Some(Group::Simple("col"));
    }
    if s.starts_with("row-span-") || s.starts_with("row-start-") || s.starts_with("row-end-") {
        return Some(Group::Simple("row"));
    }

    // Object fit/position
    if s.starts_with("object-") {
        if matches!(
            s,
            "object-contain" | "object-cover" | "object-fill" | "object-none" | "object-scale-down"
        ) {
            return Some(Group::Simple("object-fit"));
        }
        return Some(Group::Simple("object-position"));
    }

    // Aspect ratio
    if s.starts_with("aspect-") {
        return Some(Group::Simple("aspect-ratio"));
    }

    // Ring & outline
    if s == "ring" || s.starts_with("ring-") {
        if s == "ring-inset" {
            return Some(Group::Simple("ring-inset"));
        }
        if s.starts_with("ring-offset-") {
            if s.starts_with("ring-offset-")
                && (s == "ring-offset"
                    || s.starts_with("ring-offset-0")
                    || s.starts_with("ring-offset-"))
            {
                // differentiate width vs color by next piece
            }
        }
        // ring-offset-width
        if s == "ring-offset"
            || s.starts_with("ring-offset-0")
            || s.strip_prefix("ring-offset-")
                .map_or(false, |a| looks_like_width(a))
        {
            return Some(Group::Simple("ring-offset-width"));
        }
        // ring-offset-color
        if s.starts_with("ring-offset-") {
            return Some(Group::Simple("ring-offset-color"));
        }
        // ring width
        if s == "ring"
            || s.strip_prefix("ring-")
                .map_or(false, |a| looks_like_width(a))
        {
            return Some(Group::Simple("ring-width"));
        }
        // ring color
        return Some(Group::Simple("ring-color"));
    }
    if s == "outline" || s.starts_with("outline-") {
        return Some(Group::Simple("outline"));
    }

    // Divide & Space
    if s.starts_with("divide-x") {
        return Some(Group::Simple("divide-x"));
    }
    if s.starts_with("divide-y") {
        return Some(Group::Simple("divide-y"));
    }
    if s.starts_with("divide-") {
        return Some(Group::Simple("divide"));
    }
    if s.starts_with("space-x-") {
        return Some(Group::Simple("space-x"));
    }
    if s.starts_with("space-y-") {
        return Some(Group::Simple("space-y"));
    }

    // Transforms
    if s == "transform" || s == "transform-gpu" || s == "transform-none" {
        return Some(Group::Simple("transform"));
    }
    if s.starts_with("translate-") || s.starts_with("-translate-") {
        let t = s.strip_prefix('-').unwrap_or(s);
        if t.starts_with("translate-x-") {
            return Some(Group::Coverage {
                family: "translate",
                mask: XY,
            });
        }
        if t.starts_with("translate-y-") {
            return Some(Group::Coverage {
                family: "translate",
                mask: YX,
            });
        }
    }
    if s.starts_with("scale-") {
        if s.starts_with("scale-x-") {
            return Some(Group::Coverage {
                family: "scale",
                mask: XY,
            });
        }
        if s.starts_with("scale-y-") {
            return Some(Group::Coverage {
                family: "scale",
                mask: YX,
            });
        }
        return Some(Group::Simple("scale"));
    }
    if s.starts_with("rotate-") {
        return Some(Group::Simple("rotate"));
    }
    if s.starts_with("skew-") {
        if s.starts_with("skew-x-") {
            return Some(Group::Coverage {
                family: "skew",
                mask: XY,
            });
        }
        if s.starts_with("skew-y-") {
            return Some(Group::Coverage {
                family: "skew",
                mask: YX,
            });
        }
        return Some(Group::Simple("skew"));
    }

    // Transitions & animation
    if s == "transition" || s.starts_with("transition-") {
        return Some(Group::Simple("transition"));
    }
    if s.starts_with("duration-") {
        return Some(Group::Simple("duration"));
    }
    if s.starts_with("delay-") {
        return Some(Group::Simple("delay"));
    }
    if s.starts_with("ease-") {
        return Some(Group::Simple("ease"));
    }
    if s.starts_with("animate-") {
        return Some(Group::Simple("animation"));
    }

    // Interactivity
    if s.starts_with("cursor-") {
        return Some(Group::Simple("cursor"));
    }
    if s.starts_with("pointer-events-") {
        return Some(Group::Simple("pointer-events"));
    }
    if s.starts_with("select-") {
        return Some(Group::Simple("user-select"));
    }

    // Accessibility
    if s == "sr-only" || s == "not-sr-only" {
        return Some(Group::Simple("sr"));
    }

    None
}

fn parse_token(raw_piece: &str) -> Option<Token> {
    let raw_piece = raw_piece.trim();
    if raw_piece.is_empty() {
        return None;
    }

    let mut important = false;
    let mut s: Cow<str> = Cow::Borrowed(raw_piece);
    if s.ends_with('!') {
        important = true;
        s = Cow::Owned(s.trim_end_matches('!').to_string());
    }
    if s.starts_with('!') {
        important = true;
        s = Cow::Owned(s.trim_start_matches('!').to_string());
    }

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
    // Important rules: older important cannot be overridden by non-important newer
    if older.important && !newer.important {
        return false;
    }
    // Variants must match
    if older.variants_key != newer.variants_key {
        return false;
    }

    match (&older.group, &newer.group) {
        // Simple groups: same group => conflict, newer wins (including when newer is important)
        (Some(Group::Simple(a)), Some(Group::Simple(b))) if a == b => true,

        // Coverage groups: same family; newer covers older's mask fully
        (
            Some(Group::Coverage {
                family: f1,
                mask: m1,
            }),
            Some(Group::Coverage {
                family: f2,
                mask: m2,
            }),
        ) if f1 == f2 => (m2 & *m1) == *m1,

        // Special cross-family: border-width overrides border-color/border-style when mask covers
        (
            Some(Group::Coverage {
                family: of,
                mask: om,
            }),
            Some(Group::Coverage {
                family: nf,
                mask: nm,
            }),
        ) if *nf == "border-width" && (*of == "border-color" || *of == "border-style") => {
            (nm & *om) == *om
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

    for raw in raw_tokens {
        let Some(tok) = parse_token(&raw) else {
            continue;
        };
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

    out.iter()
        .map(|t| t.raw.as_str())
        .collect::<Vec<_>>()
        .join(" ")
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
        assert_eq!(
            tw_merge(["hover:focus:p-2", "focus:hover:p-4"]),
            "focus:hover:p-4"
        );
        assert_eq!(tw_merge(["md:block", "md:inline"]), "md:inline");
    }

    #[test]
    fn important_behavior() {
        // older has ! -> newer non-important cannot override
        assert_eq!(
            tw_merge(["!bg-red-500", "bg-blue-500"]),
            "!bg-red-500 bg-blue-500"
        );
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
        assert_eq!(
            tw_merge(["border-2", "border-red-500"]),
            "border-2 border-red-500"
        );
        assert_eq!(tw_merge(["border-red-500", "border-2"]), "border-2");
        assert_eq!(tw_merge(["border-x-2", "border-l-4", "border"]), "border");
    }

    #[test]
    fn rounded_coverage() {
        assert_eq!(tw_merge(["rounded-t-md", "rounded"]), "rounded");
        assert_eq!(
            tw_merge(["rounded", "rounded-tr-lg"]),
            "rounded rounded-tr-lg"
        );
    }

    #[test]
    fn sizing_and_bg_and_ring_and_transform() {
        // sizing
        assert_eq!(tw_merge(["w-4", "w-6"]), "w-6");
        assert_eq!(tw_merge(["h-10", "md:h-8", "h-8"]), "md:h-8 h-8");
        // background color vs repeat shouldn't conflict
        assert_eq!(
            tw_merge(["bg-red-500", "bg-no-repeat", "bg-blue-500"]),
            "bg-no-repeat bg-blue-500"
        );
        // ring width and color
        assert_eq!(tw_merge(["ring", "ring-2"]), "ring-2");
        assert_eq!(tw_merge(["ring-red-500", "ring-blue-500"]), "ring-blue-500");
        // translate axis coverage
        assert_eq!(
            tw_merge(["translate-x-2", "translate-x-4"]),
            "translate-x-4"
        );
        // gradient stops
        assert_eq!(tw_merge(["from-red-500", "from-blue-500"]), "from-blue-500");
    }
}
