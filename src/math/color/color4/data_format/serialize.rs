use crate::math::{Color4, ColorSpace};
use data_format::{MapSerializer, Serialize};

impl<T: Serialize, Space: ColorSpace<T>> Serialize for Color4<T, Space> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;

        map.serialize_entry("r", &self.r)?;
        map.serialize_entry("g", &self.g)?;
        map.serialize_entry("b", &self.b)?;
        map.serialize_entry("a", &self.a)?;
        map.serialize_entry("space", Space::NAME)?;

        map.end()
    }
}
