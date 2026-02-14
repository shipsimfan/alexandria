use crate::Id;

mod key_value_iters;

mod at_index;
mod convert_index;
mod from_iter;
mod get;
mod index;
mod insert;
mod into_iter;
mod into_key_values;
mod len;
mod new;
mod remove;

#[cfg(test)]
mod tests;

pub use key_value_iters::*;

/// A list of elements accessible by stable ID, tailored for iteration
///
/// This structure stores the values in a dense packed array. This favor iteration speed at a
/// slight slowdown in access due to indirection. If you plan to do more general access than
/// iteration, consider using a [`SlotMap`](crate::SlotMap)
pub struct PackedMap<T> {
    /// The packed values
    values: Vec<T>,

    /// The indices into `indices` for a given index in `values`
    ids: Vec<Id<T>>,

    /// The indices into `values` of the value identified by a given index
    indices: Vec<usize>,
}
