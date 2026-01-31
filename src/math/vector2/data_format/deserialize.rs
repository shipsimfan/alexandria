use crate::math::Vector2;
use data_format::{Converter, Deserialize, DeserializeError, ListDeserializer};
use std::marker::PhantomData;

/// The converter for [`Vector2`]
struct Vector2Converter<'de, T: Deserialize<'de>> {
    /// A marker for the element type
    _type: PhantomData<&'de T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vector2<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Vector2Converter::default())
    }
}

impl<'de, T: Deserialize<'de>> Default for Vector2Converter<'de, T> {
    fn default() -> Self {
        Vector2Converter { _type: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>> Converter<'de> for Vector2Converter<'de, T> {
    type Value = Vector2<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a vector 2")
    }

    fn convert_list<L: ListDeserializer<'de>>(self, mut list: L) -> Result<Self::Value, L::Error> {
        let x = list.next_item()?.ok_or(L::Error::missing_field("x"))?;
        let y = list.next_item()?.ok_or(L::Error::missing_field("y"))?;

        if list.next_item::<T>()?.is_some() {
            return Err(L::Error::invalid_length(3, "2"));
        }

        Ok(Vector2::new(x, y))
    }
}
