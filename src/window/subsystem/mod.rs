use std::marker::PhantomData;

mod new;

/// Allows interaction with the platform windowing system
pub struct WindowSubsystem {
    /// A value to prevent this from being made externally
    _priv: PhantomData<()>,
}
