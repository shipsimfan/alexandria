use crate::math::{ColorHsva, ColorSpace};
use data_format::{MapSerializer, Serialize};

impl<T: Serialize, Space: ColorSpace<T>> Serialize for ColorHsva<T, Space> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;

        map.serialize_entry("h", &self.h)?;
        map.serialize_entry("s", &self.s)?;
        map.serialize_entry("v", &self.v)?;
        map.serialize_entry("a", &self.a)?;
        map.serialize_entry("space", Space::NAME)?;

        map.end()
    }
}
