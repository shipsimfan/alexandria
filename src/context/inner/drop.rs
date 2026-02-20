use crate::context::{AlexandriaContextInner, inner::ALEXANDRIA_CONTEXT_ACTIVE};
use std::sync::atomic::Ordering;

impl<UserEvent: 'static + Send> Drop for AlexandriaContextInner<UserEvent> {
    fn drop(&mut self) {
        let prev = ALEXANDRIA_CONTEXT_ACTIVE.swap(false, Ordering::Release);
        assert!(prev);
    }
}
