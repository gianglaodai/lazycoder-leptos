#![cfg(feature = "ssr")]
#[macro_export]
macro_rules! define_orm_with_common_fields {
    // Unified fields arm: do not separate req/opt
    ($name:ident { $(pub $field:ident : $ty:ty,)* }) => {
        paste::paste! {
            #[derive(sqlx::FromRow, Debug)]
            pub struct [<$name Orm>] {
                pub id: i32,
                pub uid: uuid::Uuid,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $(pub $field: $ty,)*
            }

            impl [<$name Orm>] {
                pub fn columns() -> Vec<&'static str> {
                    vec!["id", "uid", "version", "created_at", "updated_at", $(stringify!($field),)*]
                }
            }

            impl crate::infras::sqlx_repository::OrmMeta for [<$name Orm>] {
                fn columns() -> Vec<&'static str> {
                    vec!["id", "uid", "version", "created_at", "updated_at", $(stringify!($field),)*]
                }
            }

            impl crate::infras::sqlx_repository::OrmBind for [<$name Orm>] {
                fn bind_column<'q>(&'q self, col: &str, qb: &mut sqlx::QueryBuilder<'q, sqlx::Postgres>) {
                    match col {
                        "id" => { qb.push_bind(self.id); },
                        "uid" => { qb.push_bind(self.uid); },
                        "version" => { qb.push_bind(self.version); },
                        "created_at" => { qb.push_bind(self.created_at); },
                        "updated_at" => { qb.push_bind(self.updated_at); },
                        $(stringify!($field) => { qb.push_bind(&self.$field); },)*
                        other => panic!("Unknown column '{}' for {}", other, stringify!([<$name Orm>])),
                    }
                }
                fn bind_update_pairs<'q>(&'q self, cols: &[&str], qb: &mut sqlx::QueryBuilder<'q, sqlx::Postgres>) {
                    let mut first = true;
                    for col in cols.iter() {
                        if !first { qb.push(", "); } else { first = false; }
                        qb.push(*col).push(" = ");
                        match *col {
                            "id" => { qb.push_bind(self.id); },
                            "uid" => { qb.push_bind(self.uid); },
                            "version" => { qb.push_bind(self.version); },
                            "created_at" => { qb.push_bind(self.created_at); },
                            "updated_at" => { qb.push_bind(self.updated_at); },
                            $(stringify!($field) => { qb.push_bind(&self.$field); },)*
                            other => panic!("Unknown column '{}' for {}", other, stringify!([<$name Orm>])),
                        }
                    }
                }
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
