use super::command_queue::CommandQueue;
use crate::{instance::Debug, map_instance_error, Instance, Resolution, Result, DISPLAY_FORMAT};
use std::sync::{Arc, Mutex};
use win32::{DXGIFactory2, Interface};

pub(super) struct SwapChain {
    swap_chain: win32::IDXGISwapChain4,

    debug: Option<Arc<Mutex<Debug>>>,
}

impl SwapChain {
    pub(super) fn new(
        resolution: Resolution,
        command_queue: &mut CommandQueue,
        wnd: win32::HWnd,
        instance: &mut Instance,
    ) -> Result<Self> {
        let debug = instance.get_debugger();

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

        let swap_chain = map_instance_error!(
            map_instance_error!(
                instance.dxgi_factory().create_swap_chain_for_hwnd(
                    command_queue.inner(),
                    wnd,
                    &swap_chain_desc,
                    None,
                    None::<&mut win32::IDXGIOutput>,
                ),
                CreateSwapChain,
                debug
            )?
            .query_interface(),
            CreateSwapChain,
            debug
        )?;

        Ok(SwapChain { swap_chain, debug })
    }
}
