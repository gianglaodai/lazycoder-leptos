#[macro_export]
macro_rules! define_struct_with_common_fields {
    ($name:ident {
        req { $($req:tt)* }
        opt { $($opt:tt)* }
    }) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct $name {
                pub id: i32,
                pub uid: String,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $($req)*
                $($opt)*
            }

            #[derive(Debug, Clone)]
            pub struct [<$name Create>] {
                $($req)*
            }

            impl crate::common::repository::Creatable for [<$name Create>] {
                type Entity = $name;
            }

            impl crate::common::service::Entity for $name {
                fn id(&self) -> i32 { self.id }
                fn uid(&self) -> &str { &self.uid }
                fn version(&self) -> i32 { self.version }
                fn created_at(&self) -> time::OffsetDateTime { self.created_at }
                fn updated_at(&self) -> time::OffsetDateTime { self.updated_at }
            }
        }
    };
}

#[macro_export]
macro_rules! define_readonly_struct_with_common_fields {
    ($name:ident { $($field:tt)* }) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct $name {
                pub id: i32,
                pub uid: String,
                pub version: i32,
                pub created_at: time::OffsetDateTime,
                pub updated_at: time::OffsetDateTime,
                $($field)*
            }

            impl crate::common::service::Entity for $name {
                fn id(&self) -> i32 { self.id }
                fn uid(&self) -> &str { &self.uid }
                fn version(&self) -> i32 { self.version }
                fn created_at(&self) -> time::OffsetDateTime { self.created_at }
                fn updated_at(&self) -> time::OffsetDateTime { self.updated_at }
            }
        }
    };
}
