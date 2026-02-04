use crate::{SharedObject, Symbol};
use std::ffi::CStr;

impl SharedObject {
    /// Load the symbol named `symbol` from this shared object
    pub fn load_symbol<T>(&self, symbol: &CStr) -> Symbol<T> {
        Symbol::new(
            unsafe { self.inner.load_raw_symbol(symbol) }.cast(),
            self.clone(),
        )
    }
}
