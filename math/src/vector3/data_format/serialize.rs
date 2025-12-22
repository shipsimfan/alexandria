use crate::Vector3;
use data_format::{ListSerializer, Serialize};

impl<T: Serialize> Serialize for Vector3<T> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut list = serializer.serialize_list(Some(3))?;
        list.serialize_item(&self.x)?;
        list.serialize_item(&self.y)?;
        list.serialize_item(&self.z)?;
        list.end()
    }
}
