use crate::InstanceExtension;

/// Gets the os specific instance extensions
pub fn instance_extensions() -> &'static [InstanceExtension] {
    &[InstanceExtension::Win32Surface]
}
