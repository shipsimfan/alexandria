use crate::program::{types::TypeManager, Type};
use std::rc::Rc;

impl TypeManager {
    /// Register a new [`Type`] into the manager
    pub(crate) fn add<T: Into<Type>>(&mut self, r#type: T) -> Rc<Type> {
        let r#type = r#type.into();
        assert!(
            !r#type.is_builtin(),
            "Cannot add raw built-in types to the type manager"
        );

        self.inner_add(r#type)
    }

    /// Register a new [`Type`] into the manager without verifying it isn't a raw built-in type
    pub(in crate::program::types) fn inner_add<T: Into<Type>>(&mut self, r#type: T) -> Rc<Type> {
        let mut r#type = r#type.into();
        unsafe { r#type.set_id(self.types.len() as _) };

        let r#type = Rc::new(r#type);
        self.types.push(r#type.clone());
        r#type
    }
}
