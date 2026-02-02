use crate::Uuid;
use data_format::{Deserialize, DeserializeError};

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let uuid = String::deserialize(deserializer)?;
        Uuid::from_str(&uuid).map_err(|_| D::Error::invalid_value(uuid, "a UUID"))
    }
}
