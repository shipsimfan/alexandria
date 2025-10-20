use crate::Program;
use ast::Ast;
use lct_diagnostics::{Diag, DiagCtxt, Source};
use lexer::Lexer;
use tokens::Token;

mod ast;
mod lexer;
mod tokens;

/// Compile ACSL `source` into a [`Program`]
pub fn compile<'a, 'b>(source: &Source, diag: &'b DiagCtxt<'a>) -> Result<Program, Diag<'a, 'b>> {
    let ast = Ast::parse(source, diag)?;

    println!("{}", ast);

    let program = Program::new();
    println!("{:#?}", program);

    Ok(program)
}
