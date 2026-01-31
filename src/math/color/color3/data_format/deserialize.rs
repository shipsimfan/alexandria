use crate::math::{Color3, ColorSpace};
use data_format::{Converter, Deserialize, DeserializeError};
use std::{borrow::Cow, marker::PhantomData};

/// The converter for [`Color3`]
struct Color3Converter<'de, T: Deserialize<'de>, Space: ColorSpace<T>> {
    /// A marker for the element type
    _type: PhantomData<T>,

    /// A marker for the color space type
    _space: PhantomData<&'de Space>,
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Deserialize<'de> for Color3<T, Space> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Color3Converter::default())
    }
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Default for Color3Converter<'de, T, Space> {
    fn default() -> Self {
        Color3Converter {
            _type: PhantomData,
            _space: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Converter<'de>
    for Color3Converter<'de, T, Space>
{
    type Value = Color3<T, Space>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a RGB color")
    }

    fn convert_map<M: data_format::MapDeserializer<'de>>(
        self,
        mut map: M,
    ) -> Result<Self::Value, M::Error> {
        let mut r = None;
        let mut g = None;
        let mut b = None;
        let mut space: Option<Cow<'de, str>> = None;

        while let Some(key) = map.next_key::<Cow<'de, str>>()? {
            match key.as_ref() {
                "r" => {
                    if r.is_some() {
                        return Err(M::Error::duplicate_field("r"));
                    }

                    r = Some(map.next_value()?);
                }
                "g" => {
                    if g.is_some() {
                        return Err(M::Error::duplicate_field("g"));
                    }

                    g = Some(map.next_value()?);
                }
                "b" => {
                    if b.is_some() {
                        return Err(M::Error::duplicate_field("b"));
                    }

                    b = Some(map.next_value()?);
                }
                "space" => {
                    if space.is_some() {
                        return Err(M::Error::duplicate_field("space"));
                    }

                    space = Some(map.next_value()?);
                }
                _ => return Err(M::Error::unknown_field(key, &["r", "g", "b", "space"])),
            }
        }

        let r = r.ok_or(M::Error::missing_field("r"))?;
        let g = g.ok_or(M::Error::missing_field("g"))?;
        let b = b.ok_or(M::Error::missing_field("b"))?;
        if let Some(space) = space {
            if space != Space::NAME {
                return Err(M::Error::invalid_value(space, Space::NAME));
            }
        }

        Ok(Color3::new(r, g, b))
    }
}
