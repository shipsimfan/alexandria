use crate::StateTrackingInput;
use alexandria_common::{Input, Window as CommonWindow};

#[cfg(target_os = "windows")]
type WindowType<I> = Box<alexandria_dx11::Window<I>>;

pub struct Window<I: Input = StateTrackingInput>(WindowType<I>);

impl<I: Input> Window<I> {
    #[inline(always)]
    pub fn new(
        title: &str,
        width: usize,
        height: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Window(<WindowType<I> as CommonWindow<I>>::new(
            title, width, height,
        )?))
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
    pub fn width(&self) -> usize {
        self.0.width()
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.0.height()
    }

    #[inline(always)]
    pub fn inner(&mut self) -> &mut WindowType<I> {
        &mut self.0
    }
}
