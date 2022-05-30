use crate::{Input, Window};
use ginger::{Image, Pixel};
use win32::{D3D11SubresourceData, DXGIFormat};

pub struct Texture {
    texture: win32::ID3D11Texture2D,
    srv: win32::ID3D11ShaderResourceView,
    uav: win32::ID3D11UnorderedAccessView,
    slot: usize,
}

impl Texture {
    pub fn new_1f<I: Input>(
        image: &[f32],
        width: usize,
        slot: usize,
        window: &mut Window<I>,
    ) -> Self {
        let initial_data =
            win32::D3D11SubresourceData::new(image, (std::mem::size_of::<f32>() * width) as u32, 0);

        Self::create(
            initial_data,
            width,
            image.len() / width,
            slot,
            DXGIFormat::R32Float,
            window,
        )
    }

    pub fn new<I: Input>(image: &Image<f32>, slot: usize, window: &mut Window<I>) -> Self {
        let initial_data = win32::D3D11SubresourceData::new(
            image.pixels(),
            (std::mem::size_of::<Pixel<f32>>() * image.width()) as u32,
            0,
        );

        Self::create(
            initial_data,
            image.width(),
            image.height(),
            slot,
            DXGIFormat::R32G32B32A32Float,
            window,
        )
    }

    pub fn set_slot(&mut self, slot: usize) {
        self.slot = slot
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .vs_set_shader_resources(self.slot as u32, &mut [&mut self.srv]);
        window
            .device_context()
            .ps_set_shader_resources(self.slot as u32, &mut [&mut self.srv]);
    }

    pub fn set_active_compute<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .cs_set_shader_resources(self.slot as u32, &mut [Some(&mut self.srv)])
    }

    pub fn set_active_compute_rw<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .cs_set_unordered_access_views(self.slot as u32, &mut [Some(&mut self.uav)]);
    }

    pub fn inner_mut(&mut self) -> &mut win32::ID3D11Texture2D {
        &mut self.texture
    }

    fn create<I: Input>(
        initial_data: D3D11SubresourceData,
        width: usize,
        height: usize,
        slot: usize,
        format: DXGIFormat,
        window: &mut Window<I>,
    ) -> Self {
        let desc = win32::D3D11Texture2DDesc::new(
            width as u32,
            height as u32,
            1,
            1,
            format,
            1,
            0,
            win32::D3D11Usage::Default,
            &[
                win32::D3D11BindFlag::ShaderResource,
                win32::D3D11BindFlag::UnorderedAccess,
            ],
            &[],
            &[],
        );

        let mut texture = window
            .device()
            .create_texture_2d(&desc, Some(&initial_data))
            .unwrap();

        let srv_desc = win32::D3D11ShaderResourceViewDesc::new(format, &mut texture);

        let srv = window
            .device()
            .create_shader_resource_view(&mut texture, &srv_desc)
            .unwrap();

        let uav_desc = win32::D3D11UnorderedAccessViewDesc::new(format, &mut texture);

        let uav = window
            .device()
            .create_unordered_access_view(&mut texture, &uav_desc)
            .unwrap();

        Texture {
            texture,
            srv,
            uav,
            slot,
        }
    }
}
