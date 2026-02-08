use slot::Slot;

mod id;
mod iters;
mod slot;

mod get;
mod index;
mod insert;
mod into_iter;
mod len;
mod new;
mod remove;

pub use id::Id;
pub use iters::*;

/// A list of elements accessible by stable ID
#[derive(Clone)]
pub struct SlotMap<T> {
    /// The slots that make up this map
    slots: Vec<Slot<T>>,

    /// The number of elements in the slot map
    len: usize,

    /// The first free slot in the map
    first_free: Option<usize>,
}
