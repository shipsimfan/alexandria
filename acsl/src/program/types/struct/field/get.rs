use crate::program::{
    types::{StructField, StructFieldMeta},
    Type,
};
use std::rc::Rc;

impl StructField {
    /// Gets the name of this field
    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the type of the field
    pub const fn r#type(&self) -> &Rc<Type> {
        &self.r#type
    }

    /// Get the metadata effecting this field
    pub const fn meta(&self) -> Option<StructFieldMeta> {
        self.meta
    }
}
