use slot::Slot;

mod id;
mod slot;

mod get;
mod insert;
mod new;
mod remove;

pub use id::Id;

/// A list of elements accessible by stable ID
#[derive(Clone)]
pub struct SlotMap<T> {
    /// The slots that make up this map
    slots: Vec<Slot<T>>,

    /// The first free slot in the map
    first_free: Option<usize>,
}
