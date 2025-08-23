#[macro_export]
macro_rules! define_to_with_common_fields_fe {
    ($name:ident { $($field:tt)* }) => {
        paste::paste! {
            #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
            pub struct [<$name TO>] {
                pub id: i32,
                pub uid: String,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $($field)*
            }
        }
    };
}

pub trait AsCell {
    fn as_cell(&self) -> String;
}

// Narrow, non-overlapping implementations to avoid coherence conflicts
impl AsCell for String {
    fn as_cell(&self) -> String {
        self.clone()
    }
}
impl AsCell for i32 {
    fn as_cell(&self) -> String {
        self.to_string()
    }
}
impl AsCell for time::OffsetDateTime {
    fn as_cell(&self) -> String {
        let fmt =
            time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        self.format(&fmt).unwrap_or_else(|_| self.to_string())
    }
}
impl AsCell for bool {
    fn as_cell(&self) -> String {
        if *self {
            "✓".into()
        } else {
            "✗".into()
        }
    }
}
impl<T: AsCell> AsCell for Option<T> {
    fn as_cell(&self) -> String {
        self.as_ref().map(|v| v.as_cell()).unwrap_or_default()
    }
}

#[macro_export]
macro_rules! define_readonly_to_with_common_fields_fe {
    ($name:ident { $( pub $fname:ident : $fty:ty , )* }) => {
        paste::paste! {
            #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
            pub struct [<$name TO>] {
                pub id: i32,
                pub uid: String,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $( pub $fname : $fty, )*
            }

            impl [<$name TO>] {
                pub fn columns() -> Vec<&'static str> {
                    vec!["id", "uid", "version", "created_at", "updated_at", $(stringify!($fname),)*]
                }

                pub fn to_field_map(&self) -> std::collections::HashMap<&'static str, String> {
                    use crate::pages::rest::macros::define_to_with_common_fields_fe::AsCell;
                    let mut m = std::collections::HashMap::new();
                    m.insert("id", self.id.as_cell());
                    m.insert("uid", self.uid.as_cell());
                    m.insert("version", self.version.as_cell());
                    m.insert("created_at", self.created_at.as_cell());
                    m.insert("updated_at", self.updated_at.as_cell());
                    $( m.insert(stringify!($fname), self.$fname.as_cell()); )*
                    m
                }

                pub fn get(&self, name: &str) -> Option<String> {
                    use crate::pages::rest::macros::define_to_with_common_fields_fe::AsCell;
                    match name {
                        "id" => Some(self.id.as_cell()),
                        "uid" => Some(self.uid.as_cell()),
                        "version" => Some(self.version.as_cell()),
                        "created_at" => Some(self.created_at.as_cell()),
                        "updated_at" => Some(self.updated_at.as_cell()),
                        $( stringify!($fname) => Some(self.$fname.as_cell()), )*
                        _ => None
                    }
                }

                pub fn to_cells(&self, cols: &[&str]) -> Vec<String> {
                    cols.iter().map(|&c| self.get(c).unwrap_or_default()).collect()
                }
            }
        }
    };
}
