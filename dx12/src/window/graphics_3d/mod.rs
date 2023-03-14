use crate::{map_raw_error, Adapter, Instance, Resolution, Result};
use command_queue::CommandQueue;
use common::DebugMessage;
use device::Device;
use render_target_view::RenderTargetView;
use std::sync::{Arc, Mutex};
use swap_chain::SwapChain;

mod command_queue;
mod debug;
mod device;
mod heap;
mod render_target_view;
mod swap_chain;

pub(crate) use debug::Debug;

const FRAME_COUNT: u32 = 2;

pub struct Graphics3D {
    device: Device,
    command_queue: CommandQueue,
    swap_chain: SwapChain,
    render_target_view: RenderTargetView,

    debug: Option<Arc<Mutex<Debug>>>,
}

impl Graphics3D {
    pub(super) fn new(
        instance: &mut Instance,
        adapter: &mut Adapter,
        resolution: Resolution,
        wnd: win32::HWnd,
    ) -> Result<Self> {
        let mut device = map_raw_error!(Device::new(adapter), CreateD3D12Device)?;

        let debug = if instance.debug_enabled() {
            Some(Debug::new(&mut device)?)
        } else {
            None
        };

        let mut command_queue = CommandQueue::new(&mut device, debug.clone())?;

        let mut swap_chain = SwapChain::new(resolution, &mut command_queue, wnd, instance)?;

        let render_target_view = RenderTargetView::new(
            &mut device,
            &mut swap_chain,
            debug.clone(),
            FRAME_COUNT as usize,
        )?;

        Ok(Graphics3D {
            device,
            command_queue,
            swap_chain,
            render_target_view,

            debug,
        })
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
}
