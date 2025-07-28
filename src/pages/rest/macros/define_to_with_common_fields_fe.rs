#[macro_export]
macro_rules! define_to_with_common_fields_fe {
    ($name:ident { $($field:tt)* }) => {
        #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
        pub struct $name {
            #[serde(default)]
            pub id: Option<i32>,
            #[serde(default)]
            pub uid: Option<String>,
            #[serde(default)]
            pub version: Option<i32>,
            #[serde(default, with = "time::serde::timestamp::option")]
            pub created_at: Option<time::OffsetDateTime>,
            #[serde(default, with = "time::serde::timestamp::option")]
            pub updated_at: Option<time::OffsetDateTime>,
            $($field)*
        }
    };
}
