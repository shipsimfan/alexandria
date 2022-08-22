use crate::Window;
use alexandria_common::Texture2D as CommonTexture2D;
use ginger::Image;

#[cfg(target_os = "windows")]
type Texture2DType = alexandria_dx11::Texture2D;

#[cfg(target_os = "linux")]
type Texture2DType = alexandria_opengl::Texture2D;

pub struct Texture2D(Texture2DType);

impl Texture2D {
    #[inline(always)]
    pub fn new(
        image: &Image<f32>,
        slot: usize,
        window: &mut Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Texture2D(<Texture2DType as CommonTexture2D>::new(
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
    pub fn clear_active(&mut self) {
        self.0.clear_active()
    }
}
