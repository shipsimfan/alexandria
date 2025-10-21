use types::TypeManager;

pub mod types;

mod display;
mod new;

pub use types::Type;

/// A compiled ACSL program ready for analysis or lowering
#[derive(Debug)]
pub struct Program {
    /// The list of all types in the program
    types: TypeManager,
}
