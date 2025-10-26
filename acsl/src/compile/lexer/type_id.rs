use crate::compile::Lexer;

impl<'a> Lexer<'a> {
    /// Allocate a new AST type ID
    pub fn allocate_type_id(&mut self) -> u32 {
        let ret = self.next_type_id;
        self.next_type_id += 1;
        ret
    }
}
