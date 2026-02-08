use std::marker::PhantomData;

mod clone;
mod debug;
mod display;
mod eq;
mod get;
mod hash;
mod new;
mod ord;

/// An ID pointing at a stable value in a [`SlotMap`](crate::SlotMap)
pub struct Id<T> {
    /// The index into the slot map
    index: u32,

    /// The generation of the slot
    generation: u32,

    /// The type the ID points to
    _type: PhantomData<T>,
}
