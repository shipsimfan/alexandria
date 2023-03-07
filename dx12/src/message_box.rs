use win32::string_to_utf16;

pub struct MessageBox;

impl common::MessageBox for MessageBox {
    fn run(class: common::MessageBoxClass, title: &str, message: &str) {
        let flags = match class {
            common::MessageBoxClass::Error => &[
                win32::MessageBoxFlag::Ok,
                win32::MessageBoxFlag::Error,
                win32::MessageBoxFlag::SetForeground,
            ],
        };

        win32::message_box_ex(
            None,
            Some(&string_to_utf16!(message)),
            Some(&string_to_utf16!(title)),
            flags,
        );
    }
}
