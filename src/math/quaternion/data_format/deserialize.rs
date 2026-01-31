use crate::math::Quaternion;
use data_format::{Converter, Deserialize, DeserializeError};
use std::{borrow::Cow, marker::PhantomData};

/// The converter for [`Quaternion`]
struct QuaternionConverter<'de, T: Deserialize<'de>> {
    /// A marker for the element type
    _type: PhantomData<&'de T>,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Quaternion<T> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(QuaternionConverter::default())
    }
}

impl<'de, T: Deserialize<'de>> Default for QuaternionConverter<'de, T> {
    fn default() -> Self {
        QuaternionConverter { _type: PhantomData }
    }
}

impl<'de, T: Deserialize<'de>> Converter<'de> for QuaternionConverter<'de, T> {
    type Value = Quaternion<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a quaternion")
    }

    fn convert_map<M: data_format::MapDeserializer<'de>>(
        self,
        mut map: M,
    ) -> Result<Self::Value, M::Error> {
        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut w = None;

        while let Some(key) = map.next_key::<Cow<'de, str>>()? {
            match key.as_ref() {
                "x" => {
                    if x.is_some() {
                        return Err(M::Error::duplicate_field("x"));
                    }

                    x = Some(map.next_value()?);
                }
                "y" => {
                    if y.is_some() {
                        return Err(M::Error::duplicate_field("y"));
                    }

                    y = Some(map.next_value()?);
                }
                "z" => {
                    if z.is_some() {
                        return Err(M::Error::duplicate_field("z"));
                    }

                    z = Some(map.next_value()?);
                }
                "w" => {
                    if w.is_some() {
                        return Err(M::Error::duplicate_field("w"));
                    }

                    w = Some(map.next_value()?);
                }
                _ => return Err(M::Error::unknown_field(key, &["x", "y", "z", "w"])),
            }
        }

        let x = x.ok_or(M::Error::missing_field("x"))?;
        let y = y.ok_or(M::Error::missing_field("y"))?;
        let z = z.ok_or(M::Error::missing_field("z"))?;
        let w = w.ok_or(M::Error::missing_field("w"))?;

        Ok(Quaternion::new(x, y, z, w))
    }
}
