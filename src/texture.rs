use crate::Window;
use alexandria_common::{
    Input, SampleType, Texture2D as CommonTexture2D, TextureFormat, UpdateRegion,
};
use ginger::Pixel;

#[cfg(target_os = "windows")]
type Texture2DType<F> = alexandria_dx11::Texture2D<F>;

#[cfg(target_os = "linux")]
type Texture2DType = alexandria_opengl::Texture2D;

pub struct Texture2D<F: TextureFormat = Pixel<f32>>(Texture2DType<F>);

impl<F: TextureFormat> Texture2D<F> {
    #[inline(always)]
    pub fn new<I: Input>(
        image: &[F],
        width: usize,
        height: usize,
        slot: usize,
        sample_type: SampleType,
        window: &mut Window<I>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Texture2D(<Texture2DType<F> as CommonTexture2D<F>>::new(
            image,
            width,
            height,
            slot,
            sample_type,
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

    #[inline(always)]
    pub fn update_region(&mut self, region: UpdateRegion, data: &[F]) {
        self.0.update_region(region, data)
    }
}
