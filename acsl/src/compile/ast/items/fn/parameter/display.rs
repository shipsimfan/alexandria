use crate::compile::ast::items::FnParameter;

impl<'a> std::fmt::Display for FnParameter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.r#type)
    }
}
