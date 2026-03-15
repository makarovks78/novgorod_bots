use serde::{Deserialize, Deserializer, de};
use std::collections::HashMap;

pub fn deserialize_u8_key_map<'de, D>(deserializer: D) -> Result<HashMap<u8, u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: HashMap<String, u8> = HashMap::deserialize(deserializer)?;
    raw.into_iter()
        .map(|(k, v)| {
            k.parse::<u8>()
                .map(|parsed| (parsed, v))
                .map_err(|_| de::Error::custom(format!("invalid u8 key: {}", k)))
        })
        .collect()
}
