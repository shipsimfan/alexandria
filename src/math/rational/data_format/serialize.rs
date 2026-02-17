use crate::math::Rational;
use data_format::{ListSerializer, Serialize};

impl Serialize for Rational {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut list = serializer.serialize_list(Some(2))?;
        list.serialize_item(&self.numerator)?;
        list.serialize_item(&self.denominator)?;
        list.end()
    }
}
