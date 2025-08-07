#[macro_export]
macro_rules! define_to_with_common_fields_be {
    ($name:ident { $($field:tt)* }) => {
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
        pub struct $name {
            pub id: i32,
            pub uid: String,
            pub version: i32,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            $($field)*
        }
    };
}
