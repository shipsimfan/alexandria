use crate::{Adapter, Display, Instance, RefreshRate, Resolution, Result, DISPLAY_FORMAT};
use common::DebugMessage;
use debug::Debug;
use win32::{string_to_utf16, D3D12Device, D3D12Object, DXGIFactory, DXGIFactory2, Interface};

pub struct Window {
    wnd: win32::HWnd,
    alive: bool,

    device: win32::ID3D12Device,
    command_queue: win32::ID3D12CommandQueue,

    swap_chain: win32::IDXGISwapChain4,

    debug: Option<Debug>,
}

mod create;
mod debug;
mod wnd_proc;

pub(crate) use wnd_proc::wnd_proc;

impl Window {
    pub fn new(
        title: &str,
        resolution: Option<Resolution>,
        refresh_rate: Option<RefreshRate>,
        instance: &mut Instance,
        adapter: Option<Adapter>,
        display: Option<Display>,
    ) -> Result<Self> {
        // Unwrap the adapter and display
        let mut adapter = match adapter {
            Some(adapter) => adapter,
            None => instance.default_adapter()?,
        };

        let display = match display {
            Some(display) => display,
            None => adapter.default_display()?,
        };

        // Convert the provided values
        let title = string_to_utf16!(title);
        let (resolution, refresh_rate) = display.find_closest_resolution(resolution, refresh_rate);
        let (x, y) = create::calculate_window_position(resolution, &display);

        // Create the window
        let wnd = create::create_window(
            instance,
            resolution.width(),
            resolution.height(),
            x,
            y,
            &title,
        )?;

        // Create the D3D12 Device
        let mut device: win32::ID3D12Device =
            win32::d3d12_create_device(adapter.inner(), win32::D3DFeatureLevel::_12_1)?;
        device.set_name(&string_to_utf16!("D3D12 Device"))?;

        let debug = if instance.debug_enabled() {
            Some(Debug::new(&mut device)?)
        } else {
            None
        };

        // Create the command queue
        let command_queue_desc = win32::D3D12CommandQueueDesc::new(
            win32::D3D12CommandListType::Direct,
            win32::D3D12CommandQueuePriority::Normal as i32,
            &[],
            0,
        );
        let mut command_queue: win32::ID3D12CommandQueue =
            device.create_command_queue(&command_queue_desc)?;
        command_queue.set_name(&string_to_utf16!("D3D12 Command Queue"))?;

        // Create the swap chain
        let swap_chain_desc = win32::DXGISwapChainDesc1::new(
            resolution.width() as u32,
            resolution.height() as u32,
            DISPLAY_FORMAT,
            false,
            win32::DXGISampleDesc::new(1, 0),
            win32::DXGIUsage::RenderTargetOutput,
            2,
            win32::DXGIScaling::Stretch,
            win32::DXGISwapEffect::FlipDiscard,
            win32::DXGIAlphaMode::Ignore,
            &[],
        );
        let swap_chain = instance
            .dxgi_factory()
            .create_swap_chain_for_hwnd(
                &mut command_queue,
                wnd,
                &swap_chain_desc,
                None,
                None::<&mut win32::IDXGIOutput>,
            )?
            .query_interface()?;

        Ok(Window {
            wnd,
            alive: true,

            device,
            command_queue,

            swap_chain,

            debug,
        })
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn poll_events(&mut self) {
        while let Some(msg) = win32::peek_message(None, 0, 0, &[win32::PeekMessage::Remove]) {
            if msg.message() == win32::WindowMessage::Quit as u32 {
                self.alive = false;
            }

            win32::translate_message(&msg);
            win32::dispatch_message(&msg);
        }
    }

    pub fn pop_debug_message(&mut self) -> Result<Option<DebugMessage>> {
        match &mut self.debug {
            Some(debug) => debug.pop_message(),
            None => Ok(None),
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        win32::destroy_window(self.wnd).unwrap();
    }
}

impl !Send for Window {}

fn print_debug_message(message: common::DebugMessage) {
    println!("[{}] {}", message.level(), message.message());
}
