use crate::{compile::Ast, Program};
use lct_diagnostics::{Diag, DiagCtxt};

mod type_solving;

/// Analyzes the semantics of an [`Ast`] and converts it into a [`Program`]
pub(in crate::compile) fn semantic_analysis<'a, 'b>(
    ast: Ast,
    diag: &'b DiagCtxt<'a>,
) -> Result<Program, Diag<'a, 'b>> {
    let mut program = Program::new();

    type_solving::solve_types(program.types_mut(), &ast, diag)?;

    Ok(program)
}
