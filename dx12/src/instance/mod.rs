use crate::{Adapter, Result};
use common::DebugMessage;
use debug::Debug;

mod adapter_iter;
mod class;
mod debug;

pub(crate) use class::WindowClass;

pub use adapter_iter::*;

pub struct Instance {
    dxgi_factory: win32::IDXGIFactory7,

    instance_handle: win32::HInstance,
    window_class: WindowClass,

    debug: Option<Debug>,
}

impl Instance {
    pub fn new(enable_debugging: bool) -> Result<Self> {
        let debug = if enable_debugging {
            Some(Debug::new()?)
        } else {
            None
        };

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

            debug,
        })
    }

    pub fn enum_adapters<'a>(&'a mut self) -> Result<AdapterIter<'a>> {
        Ok(AdapterIter::new(&mut self.dxgi_factory))
    }

    pub fn default_adapter(&mut self) -> Result<Adapter> {
        self.enum_adapters()?.next().unwrap()
    }

    pub fn pop_debug_message(&mut self) -> Result<Option<DebugMessage>> {
        match &mut self.debug {
            Some(debug) => debug.pop_message(),
            None => Ok(None),
        }
    }

    pub(crate) fn debug_enabled(&self) -> bool {
        self.debug.is_some()
    }

    pub(crate) fn instance_handle(&self) -> win32::HInstance {
        self.instance_handle
    }

    pub(crate) fn window_class(&self) -> &WindowClass {
        &self.window_class
    }

    pub(crate) fn dxgi_factory(&mut self) -> &mut win32::IDXGIFactory7 {
        &mut self.dxgi_factory
    }
}
