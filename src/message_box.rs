use crate::MessageBoxClass;
use common::MessageBox as CommonMessageBox;

#[cfg(target_os = "windows")]
type InnerMessageBox = dx12::MessageBox;

pub struct MessageBox;

impl MessageBox {
    #[inline]
    pub fn run(class: MessageBoxClass, title: &str, message: &str) {
        InnerMessageBox::run(class, title, message)
    }
}
