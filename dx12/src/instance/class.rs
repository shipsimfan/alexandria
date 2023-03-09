use super::wnd_proc::wnd_proc;
use crate::Result;

pub struct WindowClass {
    atom: win32::Atom,
}

const CLASS_NAME: &[u16] = &[
    0x41, 0x6C, 0x65, 0x78, 0x61, 0x6E, 0x64, 0x72, 0x69, 0x61, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F,
    0x77, 0x20, 0x43, 0x6C, 0x61, 0x73, 0x73, 0x00,
]; // "Alexandria Window Class"

impl WindowClass {
    pub(super) fn new(instance: win32::HInstance) -> Result<Self> {
        let wnd_class = win32::WndClassEx::new(
            &[
                win32::ClassStyle::HRedraw,
                win32::ClassStyle::VRedraw,
                win32::ClassStyle::OwnDC,
            ],
            wnd_proc,
            0,
            0,
            instance,
            None,
            None,
            None,
            None,
            CLASS_NAME,
            None,
        );
        let atom = win32::register_class_ex(&wnd_class)?;

        Ok(WindowClass { atom })
    }

    pub(crate) fn as_class_name(&self) -> win32::ClassName {
        win32::ClassName::Atom(self.atom)
    }
}
