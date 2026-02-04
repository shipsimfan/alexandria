use crate::{AlexandriaContextInner, context::inner::ALEXANDRIA_CONTEXT_ACTIVE};

impl Drop for AlexandriaContextInner {
    fn drop(&mut self) {
        ALEXANDRIA_CONTEXT_ACTIVE.with_borrow_mut(|context_active| {
            assert_eq!(*context_active, true);
            *context_active = false;
        });
    }
}
