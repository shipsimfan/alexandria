use crate::program::{
    types::{StructField, StructFieldMeta},
    Type,
};
use std::rc::Rc;

impl StructField {
    /// Create a new [`StructField`]
    pub(crate) const fn new(meta: Option<StructFieldMeta>, name: String, r#type: Rc<Type>) -> Self {
        StructField { meta, name, r#type }
    }
}
