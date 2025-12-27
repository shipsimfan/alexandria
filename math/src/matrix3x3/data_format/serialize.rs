use crate::Matrix3x3;
use data_format::{ListSerializer, Serialize};

impl<T: Serialize> Serialize for Matrix3x3<T> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut list = serializer.serialize_list(Some(3))?;
        list.serialize_item(&self.r0)?;
        list.serialize_item(&self.r1)?;
        list.serialize_item(&self.r2)?;
        list.end()
    }
}
