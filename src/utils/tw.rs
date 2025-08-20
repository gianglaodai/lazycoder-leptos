use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    variants: String,
    important: bool,
    group: &'static str,
}

#[derive(Clone)]
struct Rule {
    group: &'static str,
    regex: Regex,
}

static CONFLICTS: Lazy<HashMap<&'static str, HashSet<&'static str>>> = Lazy::new(|| {
    let mut margin_related = HashSet::from([
        "m", "mx", "my", "mt", "mr", "mb", "ml"
    ]);
    let mut padding_related = HashSet::from([
        "p", "px", "py", "pt", "pr", "pb", "pl"
    ]);

    let mut m = HashMap::new();
    for g in &["m","mx","my","mt","mr","mb","ml"] {
        m.insert(*g, margin_related.clone());
    }
    for g in &["p","px","py","pt","pr","pb","pl"] {
        m.insert(*g, padding_related.clone());
    }

    m.insert("bg-color",    HashSet::from(["bg-color"]));
    m.insert("text-color",  HashSet::from(["text-color"]));
    m.insert("border-color",HashSet::from(["border-color"]));

    m.insert("bg-opacity",  HashSet::from(["bg-opacity"]));
    m.insert("text-opacity",HashSet::from(["text-opacity"]));
    m.insert("border-opacity",HashSet::from(["border-opacity"]));

    m.insert("text-size",   HashSet::from(["text-size"]));
    m.insert("text-align",  HashSet::from(["text-align"]));

    let br = HashSet::from(["rounded","rounded-t","rounded-r","rounded-b","rounded-l","rounded-tl","rounded-tr","rounded-br","rounded-bl"]);
    for g in &["rounded","rounded-t","rounded-r","rounded-b","rounded-l","rounded-tl","rounded-tr","rounded-br","rounded-bl"] {
        m.insert(*g, br.clone());
    }

    m.insert("display", HashSet::from(["display"]));

    m.insert("position", HashSet::from(["position"]));

    let inset = HashSet::from(["inset","inset-x","inset-y","top","right","bottom","left"]);
    for g in &["inset","inset-x","inset-y","top","right","bottom","left"] {
        m.insert(*g, inset.clone());
    }

    m
});

static RULES: Lazy<Vec<Rule>> = Lazy::new(build_rules);

fn build_rules() -> Vec<Rule> {
    let mut v = vec![];

    v.push(r("mx", r"^mx-"));
    v.push(r("my", r"^my-"));
    v.push(r("mt", r"^mt-"));
    v.push(r("mr", r"^mr-"));
    v.push(r("mb", r"^mb-"));
    v.push(r("ml", r"^ml-"));
    v.push(r("m",  r"^m-"));

    v.push(r("px", r"^px-"));
    v.push(r("py", r"^py-"));
    v.push(r("pt", r"^pt-"));
    v.push(r("pr", r"^pr-"));
    v.push(r("pb", r"^pb-"));
    v.push(r("pl", r"^pl-"));
    v.push(r("p",  r"^p-"));

    v.push(r("display", r"^(hidden|block|inline|inline-block|flex|inline-flex|grid|inline-grid|table|inline-table|flow-root)$"));

    v.push(r("position", r"^(static|fixed|absolute|relative|sticky)$"));

    v.push(r("inset-x", r"^inset-x-"));
    v.push(r("inset-y", r"^inset-y-"));
    v.push(r("top",     r"^top-"));
    v.push(r("right",   r"^right-"));
    v.push(r("bottom",  r"^bottom-"));
    v.push(r("left",    r"^left-"));
    v.push(r("inset",   r"^inset-"));

    v.push(r("rounded-tl", r"^rounded-tl-"));
    v.push(r("rounded-tr", r"^rounded-tr-"));
    v.push(r("rounded-br", r"^rounded-br-"));
    v.push(r("rounded-bl", r"^rounded-bl-"));
    v.push(r("rounded-t",  r"^rounded-t-"));
    v.push(r("rounded-r",  r"^rounded-r-"));
    v.push(r("rounded-b",  r"^rounded-b-"));
    v.push(r("rounded-l",  r"^rounded-l-"));
    v.push(r("rounded",    r"^rounded(-\[[^\]]+\]|-[a-z0-9]+)?$"));

    v.push(r("text-size",  r"^text-(xs|sm|base|lg|xl|2xl|3xl|4xl|5xl|6xl|7xl|8xl|9xl)$"));
    v.push(r("text-align", r"^text-(left|center|right|justify)$"));

    v.push(r("text-opacity", r"^text-opacity-"));
    v.push(r("text-color",   r"^text-(?:\[.+\]|(black|white|transparent|current|inherit)|([a-z]+)(?:-\d{1,3})?)$"));

    v.push(r("bg-opacity", r"^bg-opacity-"));
    v.push(r("bg-color",   r"^bg-(?:\[.+\]|(black|white|transparent|current|inherit)|([a-z]+)(?:-\d{1,3})?)$"));

    v.push(r("border-opacity", r"^border-opacity-"));
    v.push(r("border-color",   r"^border-(?:\[.+\]|(black|white|transparent|current|inherit)|([a-z]+)(?:-\d{1,3})?)$"));

    v
}

fn r(group: &'static str, re: &str) -> Rule {
    Rule {
        group,
        regex: Regex::new(re).unwrap(),
    }
}

#[derive(Debug)]
struct Parsed<'a> {
    variants: String,
    core: &'a str,
    important: bool,
}

fn parse_token(token: &str) -> Parsed<'_> {
    let mut parts = token.split(':').collect::<Vec<_>>();
    let last = parts.pop().unwrap_or(token);

    let (important, core) = if let Some(rest) = last.strip_prefix('!') {
        (true, rest)
    } else {
        (false, last)
    };

    let variants = if parts.is_empty() {
        "".to_string()
    } else {
        parts.join(":")
    };

    Parsed { variants, core, important }
}

fn find_group(core: &str) -> Option<&'static str> {
    for rule in RULES.iter() {
        if rule.regex.is_match(core) {
            return Some(rule.group);
        }
    }
    None
}

pub fn tw_merge(input: &str) -> String {
    let mut last_kept: HashMap<Key, usize> = HashMap::new();
    let mut out: Vec<&str> = Vec::new();

    let mut active: Vec<bool> = Vec::new();

    'outer: for raw in input.split_whitespace() {
        if raw.is_empty() {
            continue;
        }
        let parsed = parse_token(raw);
        let Some(group) = find_group(parsed.core) else {
            // Không nhận diện nhóm => không hợp nhất, cứ giữ nguyên
            active.push(true);
            out.push(raw);
            continue;
        };

        let key = Key {
            variants: parsed.variants.clone(),
            important: parsed.important,
            group,
        };

        let conflicts = CONFLICTS.get(group).cloned().unwrap_or_else(|| HashSet::from([group]));

        for other_group in conflicts {
            let victim_key = Key {
                variants: parsed.variants.clone(),
                important: parsed.important,
                group: other_group,
            };
            if let Some(&victim_idx) = last_kept.get(&victim_key) {
                active[victim_idx] = false; // đánh dấu xóa
                last_kept.remove(&victim_key);
            }
        }

        let idx = out.len();
        out.push(raw);
        active.push(true);
        last_kept.insert(key, idx);
    }

    let mut result = Vec::new();
    for (i, &tok) in out.iter().enumerate() {
        if active[i] {
            result.push(tok);
        }
    }
    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::tw_merge;

    #[test]
    fn merge_margin_axes() {
        assert_eq!(tw_merge("mx-2 ml-4"), "mx-2 ml-4");
        assert_eq!(tw_merge("mx-2 ml-4"), "ml-4");
    }

    #[test]
    fn merge_simple_margin() {
        assert_eq!(tw_merge("m-2 m-4"), "m-4");
        assert_eq!(tw_merge("mt-2 m-4"), "m-4");
        assert_eq!(tw_merge("m-4 mt-2"), "mt-2");
    }

    #[test]
    fn variants_isolated() {
        assert_eq!(tw_merge("hover:m-2 hover:m-4"), "hover:m-4");
        assert_eq!(tw_merge("hover:m-4 focus:m-2"), "hover:m-4 focus:m-2");
    }

    #[test]
    fn important_isolated() {
        assert_eq!(tw_merge("m-2 !m-4"), "!m-4");
        assert_eq!(tw_merge("!m-4 m-2"), "m-2");
    }

    #[test]
    fn colors_and_opacity() {
        assert_eq!(tw_merge("bg-red-500 bg-blue-500"), "bg-blue-500");
        assert_eq!(tw_merge("text-[#123456] text-red-500"), "text-red-500");
        assert_eq!(tw_merge("border-opacity-50 border-opacity-20"), "border-opacity-20");
    }

    #[test]
    fn text_size_vs_color() {
        assert_eq!(tw_merge("text-sm text-lg"), "text-lg");
        assert_eq!(tw_merge("text-sm text-red-500"), "text-sm text-red-500");
    }

    #[test]
    fn display_position() {
        assert_eq!(tw_merge("block inline-block"), "inline-block");
        assert_eq!(tw_merge("static absolute"), "absolute");
    }

    #[test]
    fn inset_conflicts() {
        assert_eq!(tw_merge("inset-0 top-4"), "top-4");
        assert_eq!(tw_merge("top-4 inset-x-0"), "inset-x-0");
    }

    #[test]
    fn preserve_unknown_classes() {
        assert_eq!(tw_merge("prose prose-lg custom-class"), "prose prose-lg custom-class");
    }

    #[test]
    fn variants_and_important_together() {
        assert_eq!(tw_merge("sm:hover:bg-red-500 sm:hover:bg-blue-500"), "sm:hover:bg-blue-500");
        assert_eq!(tw_merge("sm:hover:!bg-red-500 sm:hover:bg-blue-500"), "sm:hover:!bg-red-500 sm:hover:bg-blue-500");
    }
}
