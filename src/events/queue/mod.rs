use crate::define_handle;
use inner::EventQueueInner;

mod inner;
mod iter;

mod get;
mod new;
mod pop;
mod push;

pub use iter::EventIter;

pub(in crate::events) use inner::pump_quit_event;

define_handle! {
    /// A queue of events that can be pushed from any thread
   pub EventQueue<UserEvent: Send> -> EventQueueInner<UserEvent>
}
