use crate::math::{Color4, ColorSpace};
use data_format::{Converter, Deserialize, DeserializeError};
use std::{borrow::Cow, marker::PhantomData};

/// The converter for [`Color4`]
struct Color4Converter<'de, T: Deserialize<'de>, Space: ColorSpace<T>> {
    /// A marker for the element type
    _type: PhantomData<T>,

    /// A marker for the color space type
    _space: PhantomData<&'de Space>,
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Deserialize<'de> for Color4<T, Space> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Color4Converter::default())
    }
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Default for Color4Converter<'de, T, Space> {
    fn default() -> Self {
        Color4Converter {
            _type: PhantomData,
            _space: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Converter<'de>
    for Color4Converter<'de, T, Space>
{
    type Value = Color4<T, Space>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a RGBA color")
    }

    fn convert_map<M: data_format::MapDeserializer<'de>>(
        self,
        mut map: M,
    ) -> Result<Self::Value, M::Error> {
        let mut r = None;
        let mut g = None;
        let mut b = None;
        let mut a = None;
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
                "a" => {
                    if a.is_some() {
                        return Err(M::Error::duplicate_field("a"));
                    }

                    a = Some(map.next_value()?);
                }
                "space" => {
                    if space.is_some() {
                        return Err(M::Error::duplicate_field("space"));
                    }

                    space = Some(map.next_value()?);
                }
                _ => return Err(M::Error::unknown_field(key, &["r", "g", "b", "a", "space"])),
            }
        }

        let r = r.ok_or(M::Error::missing_field("r"))?;
        let g = g.ok_or(M::Error::missing_field("g"))?;
        let b = b.ok_or(M::Error::missing_field("b"))?;
        let a = a.ok_or(M::Error::missing_field("a"))?;
        if let Some(space) = space {
            if space != Space::NAME {
                return Err(M::Error::invalid_value(space, Space::NAME));
            }
        }

        Ok(Color4::new(r, g, b, a))
    }
}
