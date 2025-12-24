use crate::Matrix4x4;
use data_format::{Converter, Deserialize, DeserializeError, ListDeserializer};
use std::marker::PhantomData;

/// The converter for [`Matrix4x4`]
struct Matrix4x4Converter<'de, T: Deserialize<'de>> {
    /// A marker for the element type
    _type: PhantomData<&'de T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Matrix4x4<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Matrix4x4Converter::default())
    }
}

impl<'de, T: Deserialize<'de>> Default for Matrix4x4Converter<'de, T> {
    fn default() -> Self {
        Matrix4x4Converter { _type: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>> Converter<'de> for Matrix4x4Converter<'de, T> {
    type Value = Matrix4x4<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a 4x4 matrix")
    }

    fn convert_list<L: ListDeserializer<'de>>(self, mut list: L) -> Result<Self::Value, L::Error> {
        let r0 = list.next_item()?.ok_or(L::Error::missing_field("r0"))?;
        let r1 = list.next_item()?.ok_or(L::Error::missing_field("r1"))?;
        let r2 = list.next_item()?.ok_or(L::Error::missing_field("r2"))?;
        let r3 = list.next_item()?.ok_or(L::Error::missing_field("r3"))?;

        if list.next_item::<T>()?.is_some() {
            return Err(L::Error::invalid_length(5, "4"));
        }

        Ok(Matrix4x4::new_rows(r0, r1, r2, r3))
    }
}
