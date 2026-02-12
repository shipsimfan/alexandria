use std::marker::PhantomData;

mod new;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Winodws
pub(in crate::window::subsystem) struct WindowSubsystemInner {
    /// A value to prevent this type from being made externally
    _priv: PhantomData<()>,
}
