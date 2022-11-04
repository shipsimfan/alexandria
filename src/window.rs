use crate::StateTrackingInput;
use alexandria_common::{
    FixedAspectUpdater, Input, Vector2, ViewportUpdater, Window as CommonWindow,
};

#[cfg(target_os = "windows")]
type WindowType<I> = Box<alexandria_dx11::Window<I>>;

#[cfg(target_os = "linux")]
type WindowType<I> = alexandria_opengl::Window<I>;

pub struct Window<I: Input = StateTrackingInput>(WindowType<I>);

impl<I: Input> Window<I> {
    #[inline(always)]
    pub fn new(
        title: &str,
        width: usize,
        height: usize,
        debug_logging: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window(<WindowType<I> as CommonWindow<I>>::new(
            title,
            width,
            height,
            debug_logging,
        )?);

        let viewport = window.create_viewport(
            Vector2::ZERO,
            Vector2::new(width as f32, height as f32),
            Some(FixedAspectUpdater::new((width as f32) / (height as f32))),
        );
        window.set_default_viewport(viewport);

        Ok(window)
    }

    #[inline(always)]
    pub fn poll_events(&mut self) -> bool {
        self.0.poll_events()
    }

    #[inline(always)]
    pub fn begin_render(&mut self, clear_color: [f32; 4]) {
        self.0.begin_render(clear_color)
    }

    #[inline(always)]
    pub fn end_render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.0.end_render()
    }

    #[inline(always)]
    pub fn input(&self) -> &I {
        self.0.input()
    }

    #[inline(always)]
    pub fn input_mut(&mut self) -> &mut I {
        self.0.input_mut()
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.0.width()
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.0.height()
    }

    #[inline(always)]
    pub fn create_viewport(
        &mut self,
        top_left: Vector2,
        size: Vector2,
        updater: Option<Box<dyn ViewportUpdater>>,
    ) -> usize {
        self.0.create_viewport(top_left, size, updater)
    }

    #[inline(always)]
    pub fn set_default_viewport(&mut self, viewport: usize) {
        self.0.set_default_viewport(viewport);
    }

    #[inline(always)]
    pub fn update_viewport(&mut self, viewport: usize, top_left: Vector2, size: Vector2) {
        self.0.update_viewport(viewport, top_left, size);
    }

    #[inline(always)]
    pub fn set_active_viewport(&mut self, viewport: usize) {
        self.0.set_active_viewport(viewport);
    }

    #[inline(always)]
    pub fn remove_viewport(&mut self, viewport: usize) {
        self.0.remove_viewport(viewport);
    }

    #[inline(always)]
    pub fn inner(&mut self) -> &mut WindowType<I> {
        &mut self.0
    }

    #[inline(always)]
    pub fn set_debug_logging(&mut self, enable: bool) {
        self.0.set_debug_logging(enable)
    }
}
