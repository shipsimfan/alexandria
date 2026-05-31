use crate::math::{ColorHsv, ColorSpace};
use data_format::{Converter, Deserialize, DeserializeError};
use std::{borrow::Cow, marker::PhantomData};

/// The converter for [`ColorHsv`]
struct ColorHsvConverter<'de, T: Deserialize<'de>, Space: ColorSpace<T>> {
    /// A marker for the element type
    _type: PhantomData<T>,

    /// A marker for the color space type
    _space: PhantomData<&'de Space>,
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Deserialize<'de> for ColorHsv<T, Space> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ColorHsvConverter::default())
    }
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Default for ColorHsvConverter<'de, T, Space> {
    fn default() -> Self {
        ColorHsvConverter {
            _type: PhantomData,
            _space: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de>, Space: ColorSpace<T>> Converter<'de>
    for ColorHsvConverter<'de, T, Space>
{
    type Value = ColorHsv<T, Space>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a HSV color")
    }

    fn convert_map<M: data_format::MapDeserializer<'de>>(
        self,
        mut map: M,
    ) -> Result<Self::Value, M::Error> {
        let mut h = None;
        let mut s = None;
        let mut v = None;
        let mut space: Option<Cow<'de, str>> = None;

        while let Some(key) = map.next_key::<Cow<'de, str>>()? {
            match key.as_ref() {
                "h" => {
                    if h.is_some() {
                        return Err(M::Error::duplicate_field("h"));
                    }

                    h = Some(map.next_value()?);
                }
                "s" => {
                    if s.is_some() {
                        return Err(M::Error::duplicate_field("s"));
                    }

                    s = Some(map.next_value()?);
                }
                "v" => {
                    if v.is_some() {
                        return Err(M::Error::duplicate_field("v"));
                    }

                    v = Some(map.next_value()?);
                }
                "space" => {
                    if space.is_some() {
                        return Err(M::Error::duplicate_field("space"));
                    }

                    space = Some(map.next_value()?);
                }
                _ => return Err(M::Error::unknown_field(key, &["h", "s", "v", "space"])),
            }
        }

        let h = h.ok_or(M::Error::missing_field("h"))?;
        let s = s.ok_or(M::Error::missing_field("s"))?;
        let v = v.ok_or(M::Error::missing_field("v"))?;
        if let Some(space) = space {
            if space != Space::NAME {
                return Err(M::Error::invalid_value(space, Space::NAME));
            }
        }

        Ok(ColorHsv::new(h, s, v))
    }
}
