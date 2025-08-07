#[macro_export]
macro_rules! define_struct_with_common_fields {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Debug, Clone)]
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
