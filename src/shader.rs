use crate::Window;
use alexandria_common::{Format, Shader as CommonShader};

#[cfg(target_os = "windows")]
type ShaderType = alexandria_dx11::Shader;

#[cfg(target_os = "linux")]
type ShaderType = alexandria_opengl::Shader;

pub struct Shader(ShaderType);

impl Shader {
    #[inline(always)]
    pub fn new<S: AsRef<str>>(
        code: S,
        vertex_layout: &[(&str, Format)],
        window: &mut Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Shader(<ShaderType as CommonShader>::new(
            code,
            vertex_layout,
            window.inner(),
        )?))
    }

    pub fn set_active(&mut self) {
        self.0.set_active()
    }
}
