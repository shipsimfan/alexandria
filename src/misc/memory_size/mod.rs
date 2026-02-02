mod clone;
mod deref;
mod display;
mod eq;
mod from;
mod into;
mod new;
mod ord;
mod size_of;

/// The size of something in memory
#[derive(Debug)]
pub struct MemorySize(pub u64);
