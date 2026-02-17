use crate::{AlexandriaContextInner, context::inner::ALEXANDRIA_CONTEXT_ACTIVE};
use std::sync::atomic::Ordering;

impl<UserEvent: Send> Drop for AlexandriaContextInner<UserEvent> {
    fn drop(&mut self) {
        let prev = ALEXANDRIA_CONTEXT_ACTIVE.swap(false, Ordering::Release);
        assert!(prev);
    }
}
