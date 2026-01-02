use win32::{ntddk::RtlGetVersion, wdm::RTL_OSVERSIONINFOEXW};

/// Get the current version of the running operating system
pub fn os_version() -> String {
    let mut version_information = RTL_OSVERSIONINFOEXW::default();
    unsafe { RtlGetVersion(&mut version_information as *mut _ as _) };

    format!(
        "{}.{} build {}",
        version_information.major_version,
        version_information.minor_version,
        version_information.build_number
    )
}
