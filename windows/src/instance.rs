use crate::{window::wnd_proc, Result};

pub struct Instance {
    window_class: win32::Atom,
}

impl Instance {
    pub fn new(app_name: &str) -> Result<Self> {
        let class_name = win32::string_to_utf16!(app_name);

        let wnd_class_ex = win32::WndClassEx::new(
            &[
                win32::ClassStyle::OwnDC,
                win32::ClassStyle::VRedraw,
                win32::ClassStyle::HRedraw,
            ],
            wnd_proc,
            0,
            0,
            win32::get_module_handle(None)?,
            None,
            None,
            None,
            None,
            &class_name,
            None,
        );

        let window_class = win32::register_class_ex(&wnd_class_ex)?;

        Ok(Instance { window_class })
    }

    pub(crate) fn window_class(&self) -> win32::Atom {
        self.window_class
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        win32::unregister_class(
            win32::ClassName::Atom(self.window_class),
            win32::get_module_handle(None).ok(),
        )
        .ok();
    }
}
