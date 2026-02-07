use std::marker::PhantomData;

mod display;
mod get;
mod new;

/// An ID pointing at a stable value in a [`SlotMap`](crate::SlotMap)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T> {
    /// The index into the slot map
    index: u32,

    /// The generation of the slot
    generation: u32,

    /// The type the ID points to
    _type: PhantomData<T>,
}
