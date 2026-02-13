use crate::Id;

mod convert_index;
mod get;
mod index;
mod insert;
mod into_iter;
mod len;
mod new;
mod remove;

#[cfg(test)]
mod tests;

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
