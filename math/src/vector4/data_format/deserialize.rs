use crate::Vector4;
use data_format::{Converter, Deserialize, DeserializeError, ListDeserializer};
use std::marker::PhantomData;

/// The converter for [`Vector4`]
struct Vector4Converter<'de, T: Deserialize<'de>> {
    /// A marker for the element type
    _type: PhantomData<&'de T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vector4<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Vector4Converter::default())
    }
}

impl<'de, T: Deserialize<'de>> Default for Vector4Converter<'de, T> {
    fn default() -> Self {
        Vector4Converter { _type: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>> Converter<'de> for Vector4Converter<'de, T> {
    type Value = Vector4<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a vector 4")
    }

    fn convert_list<L: ListDeserializer<'de>>(self, mut list: L) -> Result<Self::Value, L::Error> {
        let x = list.next_item()?.ok_or(L::Error::missing_field("x"))?;
        let y = list.next_item()?.ok_or(L::Error::missing_field("y"))?;
        let z = list.next_item()?.ok_or(L::Error::missing_field("z"))?;
        let w = list.next_item()?.ok_or(L::Error::missing_field("w"))?;

        if list.next_item::<T>()?.is_some() {
            return Err(L::Error::invalid_length(5, "4"));
        }

        Ok(Vector4::new(x, y, z, w))
    }
}
