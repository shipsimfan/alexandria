use std::marker::PhantomData;

mod get;

/// The Linux-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) struct WindowInner<UserEvent: 'static + Send> {
    _user_event: PhantomData<UserEvent>,
}
