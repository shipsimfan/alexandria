use crate::Window;
use alexandria_common::ConstantBuffer as CommonConstantBuffer;

#[cfg(target_os = "windows")]
type ConstantBufferType<T> = alexandria_dx11::ConstantBuffer<T>;

pub struct ConstantBuffer<T>(ConstantBufferType<T>);

impl<T> ConstantBuffer<T> {
    #[inline(always)]
    pub fn new(
        initial_data: T,
        slot: usize,
        window: &mut Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ConstantBuffer(
            <ConstantBufferType<T> as CommonConstantBuffer<T>>::new(
                initial_data,
                slot,
                window.inner(),
            )?,
        ))
    }

    #[inline(always)]
    pub fn set_data(&mut self, new_data: T) -> Result<(), Box<dyn std::error::Error>> {
        self.0.set_data(new_data)
    }

    #[inline(always)]
    pub fn set_slot(&mut self, new_slot: usize) {
        self.0.set_slot(new_slot)
    }

    #[inline(always)]
    pub fn set_active(&mut self) {
        self.0.set_active()
    }

    #[inline(always)]
    pub fn set_active_compute(&mut self) {
        self.0.set_active_compute()
    }
}
