use crate::Matrix4x4;
use data_format::{ListSerializer, Serialize};

impl<T: Serialize> Serialize for Matrix4x4<T> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut list = serializer.serialize_list(Some(4))?;
        list.serialize_item(&self.r0)?;
        list.serialize_item(&self.r1)?;
        list.serialize_item(&self.r2)?;
        list.serialize_item(&self.r3)?;
        list.end()
    }
}
