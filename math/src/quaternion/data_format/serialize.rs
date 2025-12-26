use crate::Quaternion;
use data_format::{MapSerializer, Serialize};

impl<T: Serialize> Serialize for Quaternion<T> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;

        map.serialize_entry("x", &self.x)?;
        map.serialize_entry("y", &self.y)?;
        map.serialize_entry("z", &self.z)?;
        map.serialize_entry("w", &self.w)?;

        map.end()
    }
}
