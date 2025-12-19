use serde::{de::Deserializer, Deserialize};

/// Custom deserializer for the installed field that can handle both boolean and integer values
pub fn deserialize_installed<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    // Try to deserialize as a number first
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(num) => {
            if let Some(num) = num.as_u64() {
                Ok(num != 0)
            } else {
                Err(D::Error::custom(
                    "Expected a number that can be converted to u64",
                ))
            }
        }
        serde_json::Value::Bool(b) => Ok(b),
        _ => Err(D::Error::custom("Expected a boolean or number")),
    }
}
