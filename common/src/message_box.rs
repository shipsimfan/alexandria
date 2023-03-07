pub enum MessageBoxClass {
    Error,
}

pub trait MessageBox {
    fn run(class: MessageBoxClass, title: &str, message: &str);
}
