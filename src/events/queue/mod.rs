use crate::define_handle;
use inner::EventQueueInner;

mod inner;
mod iter;

mod new;

pub use iter::EventIter;

define_handle! {
    /// A queue of events that can be pushed from any thread
   pub EventQueue<UserEvent: Send> -> EventQueueInner<UserEvent>
}
