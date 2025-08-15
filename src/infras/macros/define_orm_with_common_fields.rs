#![cfg(feature = "ssr")]
#[macro_export]
macro_rules! define_orm_with_common_fields {
    ($name:ident {$($field:tt)*}) => {
        paste::paste! {
            #[derive(sqlx::FromRow, Debug)]
            pub struct [<$name Orm>] {
                pub id: i32,
                pub uid: uuid::Uuid,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $($field)*
            }

            impl [<$name Orm>] {
                pub fn columns() -> Vec<&'static str> {
                    vec!["id", "uid", "version", "created_at", "updated_at", $(stringify!($field),)*]
                }
            }

            pub struct [<$name CreateOrm>] {
                $($field)*
            }
        }
    };
}
#[macro_export]
macro_rules! define_readonly_orm_with_common_fields {
    ($name:ident {$($field:tt)*}) => {
        paste::paste! {
            #[derive(sqlx::FromRow, Debug)]
            pub struct [<$name Orm>] {
                pub id: i32,
                pub uid: uuid::Uuid,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $($field)*
            }

            impl [<$name Orm>] {
                pub fn columns() -> Vec<&'static str> {
                    vec!["id", "uid", "version", "created_at", "updated_at", $(stringify!($field),)*]
                }
            }
        }
    };
}
