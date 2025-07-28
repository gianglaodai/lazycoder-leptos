#[macro_export]
macro_rules! define_struct_with_common_fields {
    ($name:ident { $($field:tt)* }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<String>,
            pub version: Option<i32>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $($field)*
        }
    };
}
