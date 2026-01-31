use crate::math::Vector2;
use data_format::{ListSerializer, Serialize};

impl<T: Serialize> Serialize for Vector2<T> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut list = serializer.serialize_list(Some(2))?;
        list.serialize_item(&self.x)?;
        list.serialize_item(&self.y)?;
        list.end()
    }
}
