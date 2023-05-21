use win32::string_to_utf16;

pub struct MessageBox;

const fn convert_class_to_type(class: common::MessageBoxClass) -> win32::MessageBoxType {
    match class {
        common::MessageBoxClass::Error => win32::MessageBoxType::new(
            Some(win32::MessageBoxButtons::Ok),
            Some(win32::MessageBoxIcon::Error),
            None,
            None,
            &[win32::MessageBoxFlags::SetForeground],
        ),
    }
}

impl MessageBox {
    pub fn run(class: common::MessageBoxClass, title: &str, message: &str) {
        win32::message_box_ex_w(
            None,
            Some(&string_to_utf16!(message)),
            Some(&string_to_utf16!(title)),
            convert_class_to_type(class),
            0,
        )
        .unwrap();
    }
}
