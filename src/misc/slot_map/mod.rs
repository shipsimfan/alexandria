use slot::Slot;

mod iters;
mod slot;

mod from_iter;
mod get;
mod index;
mod insert;
mod into_iter;
mod len;
mod new;
mod remove;

pub use iters::*;

/// A list of elements accessible by stable ID, tailored for access
///
/// The structure uses slots which may be used or free, which favors general access but is slower
/// for iteration. If you plan to do more iteration than general access, consider using a
/// [`PackedMap`](crate::PackedMap)
#[derive(Clone)]
pub struct SlotMap<T> {
    /// The slots that make up this map
    slots: Vec<Slot<T>>,

    /// The number of elements in the slot map
    len: usize,

    /// The first free slot in the map
    first_free: Option<usize>,
}
