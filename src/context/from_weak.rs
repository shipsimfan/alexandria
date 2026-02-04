use crate::{AlexandriaContext, AlexandriaContextInner};
use std::{mem::MaybeUninit, sync::Weak};

impl AlexandriaContext {
    /// Convert a `weak` reference to an [`AlexandriaContext`] into a strong one
    pub(crate) fn from_weak(weak: &Weak<MaybeUninit<AlexandriaContextInner>>) -> AlexandriaContext {
        AlexandriaContext {
            inner: unsafe { weak.upgrade().unwrap().assume_init() },
        }
    }
}
