use super::{device::Device, Debug};
use crate::{map_3d_error, Result};
use std::sync::{Arc, Mutex};
use win32::{string_to_utf16, D3D12Object};

pub(super) struct CommandQueue {
    command_queue: win32::ID3D12CommandQueue,

    debug: Option<Arc<Mutex<Debug>>>,
}

impl CommandQueue {
    pub(super) fn new(device: &mut Device, debug: Option<Arc<Mutex<Debug>>>) -> Result<Self> {
        let command_queue_desc = win32::D3D12CommandQueueDesc::new(
            win32::D3D12CommandListType::Direct,
            win32::D3D12CommandQueuePriority::Normal as i32,
            &[],
            0,
        );
        let mut command_queue = map_3d_error!(
            device.create_command_queue(&command_queue_desc),
            CreateCommandQueue,
            &debug
        )?;
        map_3d_error!(
            command_queue.set_name(&string_to_utf16!("D3D12 Command Queue")),
            CreateCommandQueue,
            &debug
        )?;

        Ok(CommandQueue {
            command_queue,
            debug,
        })
    }

    pub(super) fn inner(&mut self) -> &mut win32::ID3D12CommandQueue {
        &mut self.command_queue
    }
}
