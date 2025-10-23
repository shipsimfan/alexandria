use crate::{
    compile::ast::Item,
    pretty_print::{PrettyFormatter, PrettyPrint},
};

impl<'a> PrettyPrint for Item<'a> {
    fn fmt(&self, depth: usize, f: &mut PrettyFormatter) -> std::fmt::Result {
        match self {
            Item::Fn(r#fn) => r#fn.fmt(depth, f),
            Item::Struct(r#struct) => r#struct.fmt(depth, f),
        }
    }
}
