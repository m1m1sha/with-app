use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_i32_to_bool<S>(num: i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let res = if num > 0 { true } else { false };
    serializer.serialize_bool(res)
}

pub fn deserialize_i32_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let num = i32::deserialize(deserializer)?;
    Ok(if num > 0 { true } else { false })
}
