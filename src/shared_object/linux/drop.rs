use crate::shared_object::SharedObjectInner;
use linux::dlfcn::dlclose;

impl Drop for SharedObjectInner {
    fn drop(&mut self) {
        unsafe { dlclose(self.handle) };
    }
}
