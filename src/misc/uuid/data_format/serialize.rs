use crate::Uuid;
use data_format::Serialize;

impl Serialize for Uuid {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_string(&self.to_string())
    }
}
