use crate::math::Matrix3x3;
use data_format::{Converter, Deserialize, DeserializeError, ListDeserializer};
use std::marker::PhantomData;

/// The converter for [`Matrix3x3`]
struct Matrix3x3Converter<'de, T: Deserialize<'de>> {
    /// A marker for the element type
    _type: PhantomData<&'de T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Matrix3x3<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Matrix3x3Converter::default())
    }
}

impl<'de, T: Deserialize<'de>> Default for Matrix3x3Converter<'de, T> {
    fn default() -> Self {
        Matrix3x3Converter { _type: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>> Converter<'de> for Matrix3x3Converter<'de, T> {
    type Value = Matrix3x3<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a 3x3 matrix")
    }

    fn convert_list<L: ListDeserializer<'de>>(self, mut list: L) -> Result<Self::Value, L::Error> {
        let r0 = list.next_item()?.ok_or(L::Error::missing_field("r0"))?;
        let r1 = list.next_item()?.ok_or(L::Error::missing_field("r1"))?;
        let r2 = list.next_item()?.ok_or(L::Error::missing_field("r2"))?;

        if list.next_item::<T>()?.is_some() {
            return Err(L::Error::invalid_length(4, "3"));
        }

        Ok(Matrix3x3::new_rows(r0, r1, r2))
    }
}
