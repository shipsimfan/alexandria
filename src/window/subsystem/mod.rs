use std::{marker::PhantomData, rc::Rc};

mod new;
mod pump_events;
mod wait_for_event;

/// Allows interaction with the platform windowing system
pub struct WindowSubsystem {
    /// A value to prevent this from being made externally and making it `!Send + !Sync`
    _priv: PhantomData<Rc<()>>,
}
