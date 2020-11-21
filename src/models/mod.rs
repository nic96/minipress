mod post;
pub mod user;

pub use post::Post;
pub use post::PostRequest;
pub use user::User;
pub use user::UserRequest;
pub use uuid as uuid_serializer;

pub mod uuid {
    use serde::{Deserialize, Deserializer, Serializer};
    use sqlx::types::Uuid;

    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = uuid.to_simple().to_string();
        serializer.serialize_str(s.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}
