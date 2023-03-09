use crate::{Adapter, Result};

mod adapter_iter;
mod class;
mod wnd_proc;

pub(crate) use class::WindowClass;

pub use adapter_iter::*;

pub struct Instance {
    dxgi_factory: win32::IDXGIFactory7,

    instance_handle: win32::HInstance,
    window_class: WindowClass,
}

impl Instance {
    pub fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
        let flags = &[win32::DXGICreateFactoryFlag::Debug];
        #[cfg(not(debug_assertions))]
        let flags = &[];

        let dxgi_factory = win32::create_factory2(flags)?;

        let instance_handle: win32::HInstance =
            win32::get_module_handle_ex(&[win32::GetModuleHandleExFlag::UnchangedRefCount], None)?
                .into();
        let window_class = WindowClass::new(instance_handle)?;

        Ok(Instance {
            dxgi_factory,
            instance_handle,
            window_class,
        })
    }

    pub fn enum_adapters<'a>(&'a mut self) -> Result<AdapterIter<'a>> {
        Ok(AdapterIter::new(&mut self.dxgi_factory))
    }

    pub fn default_adapter(&mut self) -> Result<Adapter> {
        self.enum_adapters()?.next().unwrap()
    }

    pub(crate) fn instance_handle(&self) -> win32::HInstance {
        self.instance_handle
    }

    pub(crate) fn window_class(&self) -> &WindowClass {
        &self.window_class
    }
}
