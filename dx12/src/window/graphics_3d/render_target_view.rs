use super::{device::Device, heap::Heap, swap_chain::SwapChain, Debug};
use crate::Result;
use std::sync::{Arc, Mutex};
use win32::string_to_utf16;

pub(super) struct RenderTargetView {
    heap: Heap,
    render_target_views: Vec<win32::ID3D12Resource>,

    debug: Option<Arc<Mutex<Debug>>>,
}

impl RenderTargetView {
    pub(super) fn new(
        device: &mut Device,
        swap_chain: &mut SwapChain,
        debug: Option<Arc<Mutex<Debug>>>,
        frame_count: usize,
    ) -> Result<Self> {
        let mut heap = Heap::new(
            &string_to_utf16!("Render Target View Heap"),
            win32::D3D12DescriptorHeapType::RTV,
            frame_count as u32,
            device,
            debug.as_ref(),
        )?;

        let mut render_target_views = Vec::with_capacity(frame_count);
        for i in 0..frame_count {
            let mut render_target_view = swap_chain.get_buffer(i)?;
            device.create_render_target_view(&mut render_target_view, heap.allocate());
            render_target_views.push(render_target_view);
        }

        Ok(Self {
            heap,
            render_target_views,
            debug,
        })
    }
}
