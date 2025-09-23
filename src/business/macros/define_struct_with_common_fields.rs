#[macro_export]
macro_rules! define_struct_with_common_fields {
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

            #[derive(Debug, Clone)]
            pub struct [<$name Create>] {
                $($field)*
            }

            impl crate::common::repository::Creatable for [<$name Create>] {
                type Entity = $name;
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
        }
    };
}
