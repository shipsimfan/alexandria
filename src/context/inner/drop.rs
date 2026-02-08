use crate::{AlexandriaContextInner, context::inner::ALEXANDRIA_CONTEXT_ACTIVE};

impl<UserEvent: Send> Drop for AlexandriaContextInner<UserEvent> {
    fn drop(&mut self) {
        ALEXANDRIA_CONTEXT_ACTIVE.with_borrow_mut(|context_active| {
            assert_eq!(*context_active, true);
            *context_active = false;
        });
    }
}
