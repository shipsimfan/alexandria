use crate::math::Vector3;
use data_format::{Converter, Deserialize, DeserializeError, ListDeserializer};
use std::marker::PhantomData;

/// The converter for [`Vector3`]
struct Vector3Converter<'de, T: Deserialize<'de>> {
    /// A marker for the element type
    _type: PhantomData<&'de T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vector3<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Vector3Converter::default())
    }
}

impl<'de, T: Deserialize<'de>> Default for Vector3Converter<'de, T> {
    fn default() -> Self {
        Vector3Converter { _type: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>> Converter<'de> for Vector3Converter<'de, T> {
    type Value = Vector3<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a vector 3")
    }

    fn convert_list<L: ListDeserializer<'de>>(self, mut list: L) -> Result<Self::Value, L::Error> {
        let x = list.next_item()?.ok_or(L::Error::missing_field("x"))?;
        let y = list.next_item()?.ok_or(L::Error::missing_field("y"))?;
        let z = list.next_item()?.ok_or(L::Error::missing_field("z"))?;

        if list.next_item::<T>()?.is_some() {
            return Err(L::Error::invalid_length(4, "3"));
        }

        Ok(Vector3::new(x, y, z))
    }
}
