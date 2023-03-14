use crate::{map_instance_error, map_raw_error, Adapter, Result};
use common::DebugMessage;
use std::sync::{Arc, Mutex};

mod adapter_iter;
mod class;
mod debug;

pub(crate) use class::WindowClass;
pub(crate) use debug::Debug;

pub use adapter_iter::*;

pub struct Instance {
    dxgi_factory: win32::IDXGIFactory7,

    instance_handle: win32::HInstance,
    window_class: WindowClass,

    debug: Option<Arc<Mutex<Debug>>>,
}

impl Instance {
    pub fn new(enable_debugging: bool) -> Result<Self> {
        let debug = if enable_debugging {
            Some(map_raw_error!(Debug::new(), CreateInstanceDebug)?)
        } else {
            None
        };

        #[cfg(debug_assertions)]
        let flags = &[win32::DXGICreateFactoryFlag::Debug];
        #[cfg(not(debug_assertions))]
        let flags = &[];

        let dxgi_factory =
            map_instance_error!(win32::create_factory2(flags), CreateInstanceFactory, debug)?;

        let instance_handle: win32::HInstance = map_instance_error!(
            win32::get_module_handle_ex(&[win32::GetModuleHandleExFlag::UnchangedRefCount], None),
            CreateWindowClass,
            debug
        )?
        .into();
        let window_class =
            map_instance_error!(WindowClass::new(instance_handle), CreateWindowClass, debug)?;

        Ok(Instance {
            dxgi_factory,
            instance_handle,
            window_class,

            debug,
        })
    }

    pub fn enum_adapters<'a>(&'a mut self) -> Result<AdapterIter<'a>> {
        Ok(AdapterIter::new(
            &mut self.dxgi_factory,
            self.debug.as_ref(),
        ))
    }

    pub fn default_adapter(&mut self) -> Result<Adapter> {
        self.enum_adapters()?.next().unwrap()
    }

    pub fn get_debug_messages(&self) -> Result<Vec<DebugMessage>> {
        let mut debug = match self.debug.as_ref() {
            Some(debug) => debug.lock().unwrap(),
            None => return Ok(Vec::new()),
        };

        let mut messages = Vec::new();
        while let Some(message) = debug.pop_message() {
            messages.push(message);
        }
        Ok(messages)
    }

    pub(crate) fn debug_enabled(&self) -> bool {
        self.debug.is_some()
    }

    pub(crate) fn get_debugger(&self) -> Option<Arc<Mutex<Debug>>> {
        self.debug.clone()
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
