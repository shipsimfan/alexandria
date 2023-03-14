use super::{device::Device, Debug};
use crate::{map_3d_error, Result};
use std::sync::{Arc, Mutex};
use win32::D3D12Object;

pub(super) struct Heap {
    heap: win32::ID3D12DescriptorHeap,
    handle: win32::CD3DX12CPUDescriptorHandle,
    increment_size: usize,

    remaining: usize,
}

impl Heap {
    pub(super) fn new(
        name: &[u16],
        r#type: win32::D3D12DescriptorHeapType,
        num_descriptors: u32,
        device: &mut Device,
        debug: Option<&Arc<Mutex<Debug>>>,
    ) -> Result<Self> {
        let desc = win32::D3D12DescriptorHeapDesc::new(r#type, num_descriptors, &[], 0);
        let (mut heap, increment_size) =
            map_3d_error!(device.create_heap(&desc), CreateHeap, debug)?;
        map_3d_error!(heap.set_name(name), CreateHeap, debug)?;

        let handle = heap.get_cpu_descriptor_handle_for_heap_start().into();

        Ok(Heap {
            heap,
            handle,
            increment_size,
            remaining: num_descriptors as usize,
        })
    }

    pub(super) fn allocate(&mut self) -> win32::D3D12CPUDescriptorHandle {
        assert_ne!(self.remaining, 0);

        let handle: win32::D3D12CPUDescriptorHandle = self.handle.into();

        self.handle.offset(self.increment_size as isize);
        self.remaining -= 1;

        handle
    }
}
