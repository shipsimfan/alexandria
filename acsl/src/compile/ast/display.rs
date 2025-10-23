use crate::{compile::Ast, pretty_print::PrettyPrint};

impl<'a> std::fmt::Display for Ast<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print().fmt(f)
    }
}
