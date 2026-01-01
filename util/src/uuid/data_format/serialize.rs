use crate::UUID;
use data_format::Serialize;

impl Serialize for UUID {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_string(&self.to_string())
    }
}
