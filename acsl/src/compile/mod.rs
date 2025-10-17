use crate::Program;
use lct_diagnostics::{Diag, DiagCtxt, Source};

/// Compile ACSL `source` into a [`Program`]
pub fn compile<'a, 'b>(source: &Source, diag: &'b DiagCtxt<'a>) -> Result<Program, Diag<'a, 'b>> {
    todo!()
}
