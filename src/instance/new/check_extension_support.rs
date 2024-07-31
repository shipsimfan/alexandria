use crate::InstanceCreateError;
use vk::{Global, InstanceExtension};

/// Checks if the required extensions are supported
pub(super) fn check_extension_support(
    platform_extensions: &[InstanceExtension],
    global: &Global,
    debug: bool,
) -> Result<Vec<InstanceExtension>, InstanceCreateError> {
    let mut extensions = get_required_extensions(debug).to_vec();
    extensions.extend_from_slice(platform_extensions);

    let supported_extensions = global
        .instance_extensions()
        .map_err(InstanceCreateError::EnumerateExtensionsFailed)?;

    let mut required_extensions = extensions.clone();
    for extension in supported_extensions {
        for i in 0..required_extensions.len() {
            if required_extensions[i] == extension {
                required_extensions.swap_remove(i);
                break;
            }
        }
    }

    if required_extensions.len() > 0 {
        return Err(InstanceCreateError::MissingExtensions(required_extensions));
    }

    Ok(extensions)
}

fn get_required_extensions(debug: bool) -> &'static [InstanceExtension] {
    const ALL_EXTENSIONS: &[InstanceExtension] = &[
        InstanceExtension::Surface,
        // The last `DEBUG_EXTENSIONS` extensions must be the extensions only to be enabled in
        // debug
        InstanceExtension::DebugUtils,
    ];
    const DEBUG_EXTENSIONS: usize = 1;

    if debug {
        ALL_EXTENSIONS
    } else {
        &ALL_EXTENSIONS[..ALL_EXTENSIONS.len() - DEBUG_EXTENSIONS]
    }
}
