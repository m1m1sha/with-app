use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_i32_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let num = i32::deserialize(deserializer)?;
    Ok(if num > 0 { true } else { false })
}
