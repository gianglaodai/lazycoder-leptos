use std::str::FromStr;

pub struct SortCriterion {
    pub field: String,
    pub ascending: bool,
}

impl FromStr for SortCriterion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Ok(SortCriterion {
                field: "".to_string(),
                ascending: true,
            });
        }
        let (ascending, field) = match s.chars().next().unwrap() {
            '+' => (true, &s[1..].trim()),
            '-' => (false, &s[1..].trim()),
            _ => (true, &s.trim()),
        };
        Ok(SortCriterion {
            field: field.to_string(),
            ascending,
        })
    }
}
