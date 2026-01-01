use crate::UUID;
use data_format::{Deserialize, DeserializeError};

impl<'de> Deserialize<'de> for UUID {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let uuid = String::deserialize(deserializer)?;
        UUID::from_str(&uuid).map_err(|_| D::Error::invalid_value(uuid, "a UUID"))
    }
}
