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

#[macro_export]
macro_rules! define_readonly_to_with_common_fields_fe {
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