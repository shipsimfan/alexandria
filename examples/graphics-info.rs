use alexandria::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures, VulkanDeviceVulkan13Features,
};

/// Prints out information about the graphics hardware on the system
fn main() {
    let (context, _) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .create()
        .unwrap();

    println!("     --- Graphics Information ---");

    // Get Vulkan version
    let version = context.gpu().version().unwrap();
    println!("Vulkan Version: {}", version);

    // List graphics instance extensions
    let extensions = context.gpu().all_extensions(None).unwrap();
    print!("Vulkan Instance Extensions:");
    if extensions.len() == 0 {
        println!(" No Vulkan instance extensions supported");
    } else {
        println!();
        for extension in extensions {
            println!(" - {}", extension);
        }
    }

    // List graphics instance layers
    let layers = context.gpu().layers().unwrap();
    print!("Vulkan Instance Layers:");
    if layers.len() == 0 {
        println!(" No Vulkan instance layers supported");
    } else {
        println!();
        for layer in layers {
            println!(" - {} (Driver: v{})", layer, layer.version());
            if layer.description().len() > 0 {
                println!("     {}", layer.description());
            }

            let extensions = context.gpu().all_extensions(Some(&layer)).unwrap();

            if extensions.len() == 0 {
                continue;
            }

            println!("     Extensions:");
            for extension in extensions {
                println!("      - {}", extension);
            }
        }
    }

    // Create an instance for enumerating adapters
    let instance = context
        .gpu()
        .instance_builder(version)
        .application(
            "Enumerate Adapters Example",
            alexandria::gpu::VulkanVersion::new(0, 1, 0, 0),
        )
        .create()
        .unwrap();

    // List graphics adapters
    let adapters = instance.enumerate_adapters().unwrap();
    print!("Adapters:");
    if adapters.len() == 0 {
        println!(" No Vulkan adapters available");
    } else {
        println!();
        for adapter in adapters {
            println!(" - {}", adapter.name());
            println!("   - Kind: {:?}", adapter.kind());
            println!("   - Memory: {}", adapter.vram());
            println!("   - UUID: {}", adapter.uuid());
            println!("   - Vulkan Version: v{}", adapter.api_version());
            println!("   - Driver Version: v{}", adapter.driver_version());
            println!("   - Queues Families:");
            for queue in adapter.queue_families() {
                print!("     {}. {} queues (", queue.index(), queue.count());
                let mut first = true;
                if queue.graphics() {
                    print!("graphics");
                    first = false;
                }
                if queue.compute() {
                    if first {
                        first = false;
                    } else {
                        print!(", ");
                    }
                    print!("compute");
                }
                if queue.transfer() {
                    if !first {
                        print!(", ");
                    }
                    print!("transfer");
                }
                println!(")");
            }

            let extensions = adapter.enumerate_all_extensions(None).unwrap();
            if extensions.len() == 0 {
                println!("   - No extensions");
            } else {
                println!("   - Extensions:");
                for extension in extensions {
                    println!("     - {}", extension);
                }
            }

            println!("   - Features:");
            let mut features = VulkanDeviceFeatures::default();
            let mut vulkan_13_features = VulkanDeviceVulkan13Features::default();
            let mut extended_dynamic_state_features =
                VulkanDeviceExtendedDynamicStateFeatures::default();
            adapter.get_features([
                &mut features as &mut _,
                &mut vulkan_13_features as _,
                &mut extended_dynamic_state_features as _,
            ]);

            print!("     - Vulkan 1.3 Features: ");
            if vulkan_13_features.robust_image_access() {
                print!("robust_image_access ");
            }
            if vulkan_13_features.inline_uniform_block() {
                print!("inline_uniform_block ");
            }
            if vulkan_13_features.descriptor_binding_inline_uniform_block_update_after_bind() {
                print!("descriptor_binding_inline_uniform_block_update_after_bind ");
            }
            if vulkan_13_features.pipeline_creation_cache_control() {
                print!("pipeline_creation_cache_control ");
            }
            if vulkan_13_features.private_data() {
                print!("private_data ");
            }
            if vulkan_13_features.shader_demote_to_helper_invocation() {
                print!("shader_demote_to_helper_invocation ");
            }
            if vulkan_13_features.shader_terminate_invocation() {
                print!("shader_terminate_invocation ");
            }
            if vulkan_13_features.subgroup_size_control() {
                print!("subgroup_size_control ");
            }
            if vulkan_13_features.compute_full_subgroups() {
                print!("compute_full_subgroups ");
            }
            if vulkan_13_features.synchronization2() {
                print!("synchronization2 ");
            }
            if vulkan_13_features.texture_compression_astc_hdr() {
                print!("texture_compression_astc_hdr ");
            }
            if vulkan_13_features.shader_zero_initialize_workgroup_memory() {
                print!("shader_zero_initialize_workgroup_memory ");
            }
            if vulkan_13_features.dynamic_rendering() {
                print!("dynamic_rendering ");
            }
            if vulkan_13_features.shader_integer_dot_product() {
                print!("shader_integer_dot_product ");
            }
            if vulkan_13_features.maintenance4() {
                print!("maintenance4");
            }
            println!();

            print!("     - Extended Dynamic State Features: ");
            if extended_dynamic_state_features.extended_dynamic_state() {
                print!("extended_dynamic_state");
            }
            println!();
        }
    }
}
