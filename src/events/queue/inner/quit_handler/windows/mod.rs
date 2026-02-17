mod clear;
mod set;

pub(in crate::events::queue::inner::quit_handler) use clear::clear_quit_handler;
pub(in crate::events::queue::inner::quit_handler) use set::set_quit_handler;
