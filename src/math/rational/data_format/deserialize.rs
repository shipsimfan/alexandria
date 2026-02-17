use crate::math::Rational;
use data_format::{Converter, Deserialize, DeserializeError, ListDeserializer};

/// The converter for [`Rational`]
struct RationalConverter;

impl<'de> Deserialize<'de> for Rational {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_list(RationalConverter::default())
    }
}

impl Default for RationalConverter {
    fn default() -> Self {
        RationalConverter
    }
}

impl<'de> Converter<'de> for RationalConverter {
    type Value = Rational;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a rational")
    }

    fn convert_list<L: ListDeserializer<'de>>(self, mut list: L) -> Result<Self::Value, L::Error> {
        let x = list
            .next_item()?
            .ok_or(L::Error::missing_field("numerator"))?;
        let y = list
            .next_item()?
            .ok_or(L::Error::missing_field("denominator"))?;

        if list.next_item::<i32>()?.is_some() {
            return Err(L::Error::invalid_length(3, "2"));
        }

        Ok(Rational::new(x, y))
    }
}
