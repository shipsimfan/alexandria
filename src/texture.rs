use crate::{Input, Window};
use ginger::{Image, Pixel};

pub struct Texture {
    _texture: win32::ID3D11Texture2D,
    view: win32::ID3D11ShaderResourceView,
}

impl Texture {
    pub fn new<I: Input>(image: &Image<f32>, window: &mut Window<I>) -> Self {
        let desc = win32::D3D11Texture2DDesc::new(
            image.width() as u32,
            image.height() as u32,
            1,
            1,
            win32::DXGIFormat::R32G32B32A32Float,
            1,
            0,
            win32::D3D11Usage::Default,
            &[win32::D3D11BindFlag::ShaderResource],
            &[],
            &[],
        );

        let initial_data = win32::D3D11SubresourceData::new(
            image.pixels(),
            (std::mem::size_of::<Pixel<f32>>() * image.width()) as u32,
            0,
        );

        let mut texture = window
            .device()
            .create_texture_2d(&desc, Some(&initial_data))
            .unwrap();

        let srv_desc = win32::D3D11ShaderResourceViewDesc::new(
            win32::DXGIFormat::R32G32B32A32Float,
            &mut texture,
        );

        let view = window
            .device()
            .create_shader_resource_view(&mut texture, &srv_desc)
            .unwrap();

        Texture {
            _texture: texture,
            view,
        }
    }

    pub fn set_active<I: Input>(&mut self, window: &mut Window<I>) {
        window
            .device_context()
            .ps_set_shader_resources(0, &mut [&mut self.view]);
    }
}
