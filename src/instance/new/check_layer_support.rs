use super::InstanceCreateError;
use vk::{Global, InstanceLayer};

/// Checks if the required layers are supported
pub(super) fn check_layer_support(
    global: &Global,
    debug: bool,
) -> Result<&'static [InstanceLayer], InstanceCreateError> {
    let layers = get_required_layers(debug);

    let supported_layers = global
        .instance_layers()
        .map_err(InstanceCreateError::EnumerateLayersFailed)?;

    let mut required_layers = layers.to_vec();
    for layer in supported_layers {
        for i in 0..required_layers.len() {
            if required_layers[i] == layer {
                required_layers.swap_remove(i);
                break;
            }
        }
    }

    if required_layers.len() > 0 {
        return Err(InstanceCreateError::MissingLayers(required_layers));
    }

    Ok(layers)
}

fn get_required_layers(debug: bool) -> &'static [InstanceLayer] {
    const ALL_LAYERS: &[InstanceLayer] = &[
        // The last `DEBUG_LAYERS` layers must be the layers only to be enabled in debug
        InstanceLayer::Validation,
    ];
    const DEBUG_LAYERS: usize = 1;

    if debug {
        ALL_LAYERS
    } else {
        &ALL_LAYERS[..ALL_LAYERS.len() - DEBUG_LAYERS]
    }
}
