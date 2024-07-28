use crate::EventCallback;
use std::{
    ffi::{c_void, CStr},
    ptr::{null, null_mut},
};
use util::{flags_contains, Severity};
use vulkan::{
    VkBool32, VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VK_FALSE,
};

#[allow(improper_ctypes_definitions)]
pub(super) extern "system" fn log_callback(
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    _: VkDebugUtilsMessageTypeFlagsEXT,
    message_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    event_callback: *mut c_void,
) -> VkBool32 {
    if event_callback == null_mut() || message_data == null() {
        return VK_FALSE;
    }

    let message_data = unsafe { &*message_data };
    let event_callback: &Box<dyn EventCallback> = unsafe { &*(event_callback as *const _) };

    if message_data.message == null() {
        return VK_FALSE;
    }

    let severity = convert_severity(message_severity);
    let message = unsafe { CStr::from_ptr(message_data.message) }.to_string_lossy();

    let mut objects = Vec::with_capacity(
        message_data.queue_label_count as usize
            + message_data.cmd_buf_label_count as usize
            + message_data.object_count as usize,
    );

    if message_data.queue_labels != null() {
        for queue_label in unsafe {
            std::slice::from_raw_parts(
                message_data.queue_labels,
                message_data.queue_label_count as _,
            )
        } {
            if queue_label.label_name == null() {
                continue;
            }

            objects.push(unsafe { CStr::from_ptr(queue_label.label_name).to_string_lossy() });
        }
    }

    if message_data.cmd_buf_labels != null() {
        for cmd_buf_label in unsafe {
            std::slice::from_raw_parts(
                message_data.cmd_buf_labels,
                message_data.cmd_buf_label_count as _,
            )
        } {
            if cmd_buf_label.label_name == null() {
                continue;
            }

            objects.push(unsafe { CStr::from_ptr(cmd_buf_label.label_name).to_string_lossy() });
        }
    }

    if message_data.objects != null() {
        for object in unsafe {
            std::slice::from_raw_parts(message_data.objects, message_data.object_count as _)
        } {
            if object.object_name == null() {
                continue;
            }

            objects.push(unsafe { CStr::from_ptr(object.object_name).to_string_lossy() });
        }
    }

    event_callback.callback(severity, &message, objects);

    VK_FALSE
}

const fn convert_severity(message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT) -> Severity {
    if flags_contains!(
        message_severity,
        VkDebugUtilsMessageSeverityFlagBitsEXT::ErrorBitEXT
    ) {
        return Severity::Error;
    }

    if flags_contains!(
        message_severity,
        VkDebugUtilsMessageSeverityFlagBitsEXT::WarningBitEXT
    ) {
        return Severity::Warning;
    }

    if flags_contains!(
        message_severity,
        VkDebugUtilsMessageSeverityFlagBitsEXT::InfoBitEXT
    ) {
        return Severity::Info;
    }

    Severity::Debug
}
