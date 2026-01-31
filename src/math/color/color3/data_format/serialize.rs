use crate::math::{Color3, ColorSpace};
use data_format::{MapSerializer, Serialize};

impl<T: Serialize, Space: ColorSpace<T>> Serialize for Color3<T, Space> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;

        map.serialize_entry("r", &self.r)?;
        map.serialize_entry("g", &self.g)?;
        map.serialize_entry("b", &self.b)?;
        map.serialize_entry("space", Space::NAME)?;

        map.end()
    }
}
