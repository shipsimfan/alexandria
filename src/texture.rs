use crate::Window;
use alexandria_common::Texture as CommonTexture;
use ginger::Image;

#[cfg(target_os = "windows")]
type TextureType = alexandria_dx11::Texture;

pub struct Texture(TextureType);

impl Texture {
    #[inline(always)]
    pub fn new(
        image: &Image<f32>,
        slot: usize,
        window: &mut Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Texture(<TextureType as CommonTexture>::new(
            image,
            slot,
            window.inner(),
        )?))
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
        self.0.set_active()
    }

    #[inline(always)]
    pub fn set_active_compute_rw(&mut self) {
        self.0.set_active()
    }
}
