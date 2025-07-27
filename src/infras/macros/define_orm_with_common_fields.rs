#![cfg(feature = "ssr")]
#[macro_export]
macro_rules! define_orm_with_common_fields {
    (
        $name:ident {
            $(
                $field:tt
            )*
        }) => {
        #[derive(sqlx::FromRow, Debug)]
        pub struct $name {
            pub id: Option<i32>,
            pub uid: Option<uuid::Uuid>,
            pub created_at: Option<time::OffsetDateTime>,
            pub updated_at: Option<time::OffsetDateTime>,
            $($field)*
        }

        impl $name {
            pub fn columns() -> Vec<&'static str> {
                vec!["id", "uid", "created_at", "updated_at", $(stringify!($field),)*]
            }
        }
    };
}
