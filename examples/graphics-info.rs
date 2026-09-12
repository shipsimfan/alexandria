use alexandria::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures, VulkanDeviceVulkan12Features,
    VulkanDeviceVulkan13Features, VulkanInstanceExtension,
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
        .extension(VulkanInstanceExtension::Surface)
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
            let properties = adapter.get_properties();
            println!(" - {}", properties.device_name());
            println!("   - Kind: {:?}", properties.device_type());
            println!("   - UUID: {}", properties.pipeline_cache_uuid());
            println!("   - Vulkan Version: v{}", properties.api_version());
            println!("   - Driver Version: v{}", properties.driver_version());

            println!("   - Limits:");
            println!(
                "      - Max Push Constants Size: {} bytes",
                properties.max_push_constants_size()
            );
            println!(
                "      - Max Memory Allocations: {}",
                properties.max_memory_allocations()
            );
            println!(
                "      - Max Sampler Allocations: {}",
                properties.max_sampler_allocation_count()
            );
            println!(
                "      - Buffer Image Granularity: {} bytes",
                properties.buffer_image_granularity()
            );
            println!(
                "      - Max Bound Descriptor Sets: {}",
                properties.max_bound_descriptor_sets()
            );
            println!(
                "      - Max Per Stage Descriptor Samplers: {}",
                properties.max_per_stage_descriptor_samplers()
            );
            println!(
                "      - Max Per Stage Descriptor Uniform Buffers: {}",
                properties.max_per_stage_descriptor_uniform_buffers()
            );
            println!(
                "      - Max Per Stage Descriptor Storage Buffers: {}",
                properties.max_per_stage_descriptor_storage_buffers()
            );
            println!(
                "      - Max Per Stage Descriptor Sampled Images: {}",
                properties.max_per_stage_descriptor_sampled_images()
            );
            println!(
                "      - Max Per Stage Descriptor Storage Images: {}",
                properties.max_per_stage_descriptor_storage_images()
            );
            println!(
                "      - Max Per Stage Descriptor Input Attachments: {}",
                properties.max_per_stage_descriptor_input_attachments()
            );
            println!(
                "      - Max Per Stage Resources: {}",
                properties.max_per_stage_resources()
            );
            println!(
                "      - Max Descriptor Set Samplers: {}",
                properties.max_descriptor_set_samplers()
            );
            println!(
                "      - Max Descriptor Set Uniform Buffers: {}",
                properties.max_descriptor_set_uniform_buffers()
            );
            println!(
                "      - Max Descriptor Set Uniform Buffers Dynamic: {}",
                properties.max_descriptor_set_uniform_buffers_dynamic()
            );
            println!(
                "      - Max Descriptor Set Storage Buffers: {}",
                properties.max_descriptor_set_storage_buffers()
            );
            println!(
                "      - Max Descriptor Set Storage Buffers Dynamic: {}",
                properties.max_descriptor_set_storage_buffers_dynamic()
            );
            println!(
                "      - Max Descriptor Set Sampled Images: {}",
                properties.max_descriptor_set_sampled_images()
            );
            println!(
                "      - Max Descriptor Set Storage Images: {}",
                properties.max_descriptor_set_storage_images()
            );
            println!(
                "      - Max Descriptor Set Input Attachments: {}",
                properties.max_descriptor_set_input_attachments()
            );
            println!(
                "      - Max Vertex Input Attributes: {}",
                properties.max_vertex_input_attributes()
            );
            println!(
                "      - Max Vertex Input Bindings: {}",
                properties.max_vertex_input_bindings()
            );
            println!(
                "      - Max Vertex Input Attribute Offset: {}",
                properties.max_vertex_input_attribute_offset()
            );
            println!(
                "      - Max Vertex Input Binding Stride: {}",
                properties.max_vertex_input_binding_stride()
            );
            println!(
                "      - Max Vertex Output Components: {}",
                properties.max_vertex_output_components()
            );
            println!(
                "      - Max Tessellation Generation Level: {}",
                properties.max_tessellation_generation_level()
            );
            println!(
                "      - Max Tessellation Patch Size: {}",
                properties.max_tessellation_patch_size()
            );
            println!(
                "      - Max Tessellation Control Per Vertex Input Components: {}",
                properties.max_tessellation_control_per_vertex_input_components()
            );
            println!(
                "      - Max Tessellation Control Per Vertex Output Components: {}",
                properties.max_tessellation_control_per_vertex_output_components()
            );
            println!(
                "      - Max Tessellation Control Per Patch Output Components: {}",
                properties.max_tessellation_control_per_patch_output_components()
            );
            println!(
                "      - Max Tessellation Control Total Output Components: {}",
                properties.max_tessellation_control_total_output_components()
            );
            println!(
                "      - Max Tessellation Evaluation Input Components: {}",
                properties.max_tessellation_evaluation_input_components()
            );
            println!(
                "      - Max Tessellation Evaluation Output Components: {}",
                properties.max_tessellation_evaluation_output_components()
            );
            println!(
                "      - Max Geometry Shader Invocations: {}",
                properties.max_geometry_shader_invocations()
            );
            println!(
                "      - Max Geometry Input Components: {}",
                properties.max_geometry_input_components()
            );
            println!(
                "      - Max Geometry Output Components: {}",
                properties.max_geometry_output_components()
            );
            println!(
                "      - Max Geometry Output Vertices: {}",
                properties.max_geometry_output_vertices()
            );
            println!(
                "      - Max Geometry Total Output Components: {}",
                properties.max_geometry_total_output_components()
            );
            println!(
                "      - Max Fragment Input Components: {}",
                properties.max_fragment_input_components()
            );
            println!(
                "      - Max Fragment Output Attachments: {}",
                properties.max_fragment_output_attachments()
            );
            println!(
                "      - Max Fragment Dual Source Attachments: {}",
                properties.max_fragment_dual_src_attachments()
            );
            println!(
                "      - Max Fragment Combined Output Resources: {}",
                properties.max_fragment_combined_output_resources()
            );
            println!(
                "      - Max Compute Shared Memory Size: {} bytes",
                properties.max_compute_shared_memory_size()
            );
            println!(
                "      - Max Compute Work Group Count: {:?}",
                properties.max_compute_work_group_count()
            );
            println!(
                "      - Max Compute Work Group Invocations: {}",
                properties.max_compute_work_group_invocations()
            );
            println!(
                "      - Max Compute Work Group Size: {:?}",
                properties.max_compute_work_group_size()
            );
            println!(
                "      - Sub-Pixel Precision Bits: {}",
                properties.sub_pixel_precision_bits()
            );
            println!(
                "      - Sub-Texel Precision Bits: {}",
                properties.sub_texel_precision_bits()
            );
            println!(
                "      - Mipmap Precision Bits: {}",
                properties.mipmap_precision_bits()
            );
            println!(
                "      - Max Draw Indexed Index Value: {}",
                properties.max_draw_indexed_index_value()
            );
            println!(
                "      - Max Draw Indirect Count: {}",
                properties.max_draw_indirect_count()
            );
            println!(
                "      - Timestamp Compute And Graphics: {}",
                properties.timestamp_compute_and_graphics()
            );
            println!(
                "      - Timestamp Period: {} ns",
                properties.timestamp_period()
            );
            println!(
                "      - Max Clip Distances: {}",
                properties.max_clip_distances()
            );
            println!(
                "      - Max Cull Distances: {}",
                properties.max_cull_distances()
            );
            println!(
                "      - Max Combined Clip And Cull Distances: {}",
                properties.max_combined_clip_and_cull_distances()
            );
            println!(
                "      - Discrete Queue Priorities: {}",
                properties.discrete_queue_priorities()
            );
            println!(
                "      - Point Size Range: {:?}",
                properties.point_size_range()
            );
            println!(
                "      - Line Width Range: {:?}",
                properties.line_width_range()
            );
            println!(
                "      - Point Size Granularity: {}",
                properties.point_size_granularity()
            );
            println!(
                "      - Line Width Granularity: {}",
                properties.line_width_granularity()
            );
            println!("      - Strict Lines: {}", properties.strict_lines());
            println!(
                "      - Standard Sample Locations: {}",
                properties.standard_sample_locations()
            );
            println!(
                "      - Optimal Buffer Copy Offset Alignment: {} bytes",
                properties.optimal_buffer_copy_offset_alignment()
            );
            println!(
                "      - Optimal Buffer Copy Row Pitch Alignment: {} bytes",
                properties.optimal_buffer_copy_row_pitch_alignment()
            );
            println!(
                "      - Non-Coherent Atom Size: {} bytes",
                properties.non_coherent_atom_size()
            );
            println!(
                "      - Max Image Dimension 1D: {}",
                properties.max_image_dimension_1d()
            );
            println!(
                "      - Max Image Dimension 2D: {}",
                properties.max_image_dimension_2d()
            );
            println!(
                "      - Max Image Dimension 3D: {}",
                properties.max_image_dimension_3d()
            );
            println!(
                "      - Max Image Dimension Cube: {}",
                properties.max_image_dimension_cube()
            );
            println!(
                "      - Max Image Array Layers: {}",
                properties.max_image_array_layers()
            );
            println!(
                "      - Max Texel Buffer Elements: {}",
                properties.max_texel_buffer_elements()
            );
            println!(
                "      - Max Uniform Buffer Range: {} bytes",
                properties.max_uniform_buffer_range()
            );
            println!(
                "      - Max Storage Buffer Range: {} bytes",
                properties.max_storage_buffer_range()
            );
            println!(
                "      - Sparse Address Space Size: {} bytes",
                properties.sparse_address_space_size()
            );
            println!("      - Max Viewports: {}", properties.max_viewports());
            println!(
                "      - Max Sampler LOD Bias: {}",
                properties.max_sampler_lod_bias()
            );
            println!(
                "      - Max Sampler Anisotropy: {}",
                properties.max_sampler_anisotropy()
            );
            println!(
                "      - Max Viewport Dimensions: {}",
                properties.max_viewport_dimensions()
            );
            println!(
                "      - Viewport Bounds Range: {:?}",
                properties.viewport_bounds_range()
            );
            println!(
                "      - Viewport Sub-Pixel Bits: {}",
                properties.viewport_sub_pixel_bits()
            );
            println!(
                "      - Min Memory Map Alignment: {} bytes",
                properties.min_memory_map_alignment()
            );
            println!(
                "      - Min Texel Buffer Offset Alignment: {} bytes",
                properties.min_texel_buffer_offset_alignment()
            );
            println!(
                "      - Min Uniform Buffer Offset Alignment: {} bytes",
                properties.min_uniform_buffer_offset_alignment()
            );
            println!(
                "      - Min Storage Buffer Offset Alignment: {} bytes",
                properties.min_storage_buffer_offset_alignment()
            );
            println!(
                "      - Min Texel Offset: {}",
                properties.min_texel_offset()
            );
            println!(
                "      - Max Texel Offset: {}",
                properties.max_texel_offset()
            );
            println!(
                "      - Min Texel Gather Offset: {}",
                properties.min_texel_gather_offset()
            );
            println!(
                "      - Max Texel Gather Offset: {}",
                properties.max_texel_gather_offset()
            );
            println!(
                "      - Min Interpolation Offset: {}",
                properties.min_interpolation_offset()
            );
            println!(
                "      - Max Interpolation Offset: {}",
                properties.max_interpolation_offset()
            );
            println!(
                "      - Sub-Pixel Interpolation Offset Bits: {}",
                properties.sub_pixel_interpolation_offset_bits()
            );
            println!(
                "      - Max Framebuffer Width: {}",
                properties.max_framebuffer_width()
            );
            println!(
                "      - Max Framebuffer Height: {}",
                properties.max_framebuffer_height()
            );
            println!(
                "      - Max Framebuffer Layers: {}",
                properties.max_framebuffer_layers()
            );
            println!(
                "      - Framebuffer Color Sample Counts: {:?}",
                properties.framebuffer_color_sample_counts()
            );
            println!(
                "      - Framebuffer Depth Sample Counts: {:?}",
                properties.framebuffer_depth_sample_counts()
            );
            println!(
                "      - Framebuffer Stencil Sample Counts: {:?}",
                properties.framebuffer_stencil_sample_counts()
            );
            println!(
                "      - Framebuffer No-Attachments Sample Counts: {:?}",
                properties.framebuffer_no_attachments_sample_counts()
            );
            println!(
                "      - Max Color Attachments: {}",
                properties.max_color_attachments()
            );
            println!(
                "      - Sampled Image Color Sample Counts: {:?}",
                properties.sampled_image_color_sample_counts()
            );
            println!(
                "      - Sampled Image Integer Sample Counts: {:?}",
                properties.sampled_image_integer_sample_counts()
            );
            println!(
                "      - Sampled Image Depth Sample Counts: {:?}",
                properties.sampled_image_depth_sample_counts()
            );
            println!(
                "      - Sampled Image Stencil Sample Counts: {:?}",
                properties.sampled_image_stencil_sample_counts()
            );
            println!(
                "      - Storage Image Sample Counts: {:?}",
                properties.storage_image_sample_counts()
            );
            println!(
                "      - Max Sample Mask Words: {}",
                properties.max_sample_mask_words()
            );

            println!("   - Memory Heaps:");
            let memory_properties = adapter.get_memory_properties();
            for (index, memory_type) in memory_properties.memory_types().into_iter().enumerate() {
                let heap = memory_properties
                    .memory_heap(memory_type.heap_index())
                    .unwrap();
                print!("      {}. {} bytes (", index, heap.size());

                let mut first = true;
                if memory_type.device_local() {
                    first = false;
                    print!("device_local");
                }
                if memory_type.host_visible() {
                    if first {
                        first = false;
                    } else {
                        print!(", ");
                    }
                    print!("host_visible");
                }
                if memory_type.host_coherent() {
                    if !first {
                        print!(", ");
                    }
                    print!("host_coherent");
                }
                println!(")");
            }

            println!("   - Queues Families:");
            for (index, queue) in adapter
                .get_queue_family_properties()
                .into_iter()
                .enumerate()
            {
                print!("      {}. {} queues (", index, queue.count());
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
            let mut vulkan_11_features = alexandria::gpu::VulkanDeviceVulkan11Features::default();
            let mut vulkan_12_features = VulkanDeviceVulkan12Features::default();
            let mut vulkan_13_features = VulkanDeviceVulkan13Features::default();
            let mut buffer_device_address_features =
                alexandria::gpu::VulkanDeviceBufferDeviceAddressFeatures::default();
            let mut extended_dynamic_state_features =
                VulkanDeviceExtendedDynamicStateFeatures::default();
            adapter.get_features([
                &mut features as &mut _,
                &mut vulkan_11_features as _,
                &mut vulkan_12_features as _,
                &mut vulkan_13_features as _,
                &mut extended_dynamic_state_features as _,
                &mut buffer_device_address_features as _,
            ]);

            print!("     - Vulkan 1.0 Features: ");
            if features.robust_buffer_access() {
                print!("robust_buffer_access ");
            }
            if features.full_draw_index_uint32() {
                print!("full_draw_index_uint32 ");
            }
            if features.image_cube_array() {
                print!("image_cube_array ");
            }
            if features.independent_blend() {
                print!("independent_blend ");
            }
            if features.geometry_shader() {
                print!("geometry_shader ");
            }
            if features.tessellation_shader() {
                print!("tessellation_shader ");
            }
            if features.sample_rate_shading() {
                print!("sample_rate_shading ");
            }
            if features.dual_src_blend() {
                print!("dual_src_blend ");
            }
            if features.logic_op() {
                print!("logic_op ");
            }
            if features.multi_draw_indirect() {
                print!("multi_draw_indirect ");
            }
            if features.draw_indirect_first_instance() {
                print!("draw_indirect_first_instance ");
            }
            if features.depth_clamp() {
                print!("depth_clamp ");
            }
            if features.depth_bias_clamp() {
                print!("depth_bias_clamp ");
            }
            if features.fill_mode_non_solid() {
                print!("fill_mode_non_solid ");
            }
            if features.depth_bounds() {
                print!("depth_bounds ");
            }
            if features.wide_lines() {
                print!("wide_lines ");
            }
            if features.large_points() {
                print!("large_points ");
            }
            if features.alpha_to_one() {
                print!("alpha_to_one ");
            }
            if features.multi_viewport() {
                print!("multi_viewport ");
            }
            if features.sampler_anisotropy() {
                print!("sampler_anisotropy ");
            }
            if features.texture_compression_etc2() {
                print!("texture_compression_etc2 ");
            }
            if features.texture_compression_astcldr() {
                print!("texture_compression_astcldr ");
            }
            if features.texture_compression_bc() {
                print!("texture_compression_bc ");
            }
            if features.occlusion_query_precise() {
                print!("occlusion_query_precise ");
            }
            if features.pipeline_statistics_query() {
                print!("pipeline_statistics_query ");
            }
            if features.vertex_pipeline_stores_and_atomics() {
                print!("vertex_pipeline_stores_and_atomics ");
            }
            if features.fragment_stores_and_atomics() {
                print!("fragment_stores_and_atomics ");
            }
            if features.shader_tessellation_and_geometry_point_size() {
                print!("shader_tessellation_and_geometry_point_size ");
            }
            if features.shader_image_gather_extended() {
                print!("shader_image_gather_extended ");
            }
            if features.shader_storage_image_extended_formats() {
                print!("shader_storage_image_extended_formats ");
            }
            if features.shader_storage_image_multisample() {
                print!("shader_storage_image_multisample ");
            }
            if features.shader_storage_image_read_without_format() {
                print!("shader_storage_image_read_without_format ");
            }
            if features.shader_storage_image_write_without_format() {
                print!("shader_storage_image_write_without_format ");
            }
            if features.shader_uniform_buffer_array_dynamic_indexing() {
                print!("shader_uniform_buffer_array_dynamic_indexing ");
            }
            if features.shader_sampled_image_array_dynamic_indexing() {
                print!("shader_sampled_image_array_dynamic_indexing ");
            }
            if features.shader_storage_buffer_array_dynamic_indexing() {
                print!("shader_storage_buffer_array_dynamic_indexing ");
            }
            if features.shader_storage_image_array_dynamic_indexing() {
                print!("shader_storage_image_array_dynamic_indexing ");
            }
            if features.shader_clip_distance() {
                print!("shader_clip_distance ");
            }
            if features.shader_cull_distance() {
                print!("shader_cull_distance ");
            }
            if features.shader_float64() {
                print!("shader_float64 ");
            }
            if features.shader_int64() {
                print!("shader_int64 ");
            }
            if features.shader_int16() {
                print!("shader_int16 ");
            }
            if features.shader_resource_residency() {
                print!("shader_resource_residency ");
            }
            if features.shader_resource_min_lod() {
                print!("shader_resource_min_lod ");
            }
            if features.sparse_binding() {
                print!("sparse_binding ");
            }
            if features.sparse_residency_buffer() {
                print!("sparse_residency_buffer ");
            }
            if features.sparse_residency_image_2d() {
                print!("sparse_residency_image_2d ");
            }
            if features.sparse_residency_image_3d() {
                print!("sparse_residency_image_3d ");
            }
            if features.sparse_residency_2_samples() {
                print!("sparse_residency_2_samples ");
            }
            if features.sparse_residency_4_samples() {
                print!("sparse_residency_4_samples ");
            }
            if features.sparse_residency_8_samples() {
                print!("sparse_residency_8_samples ");
            }
            if features.sparse_residency_16_samples() {
                print!("sparse_residency_16_samples ");
            }
            if features.sparse_residency_aliased() {
                print!("sparse_residency_aliased ");
            }
            if features.variable_multisample_rate() {
                print!("variable_multisample_rate ");
            }
            if features.inherited_queries() {
                print!("inherited_queries");
            }
            println!();

            print!("     - Vulkan 1.1 Features: ");
            if vulkan_11_features.storage_buffer_16_bit_access() {
                print!("storage_buffer_16_bit_access ");
            }
            if vulkan_11_features.uniform_and_storage_buffer_16_bit_access() {
                print!("uniform_and_storage_buffer_16_bit_access ");
            }
            if vulkan_11_features.storage_push_constant_16() {
                print!("storage_push_constant_16 ");
            }
            if vulkan_11_features.storage_input_output_16() {
                print!("storage_input_output_16 ");
            }
            if vulkan_11_features.multiview() {
                print!("multiview ");
            }
            if vulkan_11_features.multiview_geometry_shader() {
                print!("multiview_geometry_shader ");
            }
            if vulkan_11_features.multiview_tessellation_shader() {
                print!("multiview_tessellation_shader ");
            }
            if vulkan_11_features.variable_pointers_storage_buffer() {
                print!("variable_pointers_storage_buffer ");
            }
            if vulkan_11_features.variable_pointers() {
                print!("variable_pointers ");
            }
            if vulkan_11_features.protected_memory() {
                print!("protected_memory ");
            }
            if vulkan_11_features.sampler_ycbcr_conversion() {
                print!("sampler_ycbcr_conversion ");
            }
            if vulkan_11_features.shader_draw_parameters() {
                print!("shader_draw_parameters");
            }
            println!();

            print!("     - Vulkan 1.2 Features: ");
            if vulkan_12_features.sampler_mirror_clamp_to_edge() {
                print!("sampler_mirror_clamp_to_edge ");
            }
            if vulkan_12_features.draw_indirect_count() {
                print!("draw_indirect_count ");
            }
            if vulkan_12_features.storage_buffer_8_bit_access() {
                print!("storage_buffer_8_bit_access ");
            }
            if vulkan_12_features.uniform_and_storage_buffer_8_bit_access() {
                print!("uniform_and_storage_buffer_8_bit_access ");
            }
            if vulkan_12_features.storage_push_constant_8() {
                print!("storage_push_constant_8 ");
            }
            if vulkan_12_features.shader_buffer_int64_atomics() {
                print!("shader_buffer_int64_atomics ");
            }
            if vulkan_12_features.shader_shared_int64_atomics() {
                print!("shader_shared_int64_atomics ");
            }
            if vulkan_12_features.shader_float_16() {
                print!("shader_float_16 ");
            }
            if vulkan_12_features.shader_int_8() {
                print!("shader_int_8 ");
            }
            if vulkan_12_features.descriptor_indexing() {
                print!("descriptor_indexing ");
            }
            if vulkan_12_features.shader_input_attachment_array_dynamic_indexing() {
                print!("shader_input_attachment_array_dynamic_indexing ");
            }
            if vulkan_12_features.shader_uniform_texel_buffer_array_dynamic_indexing() {
                print!("shader_uniform_texel_buffer_array_dynamic_indexing ");
            }
            if vulkan_12_features.shader_storage_texel_buffer_array_dynamic_indexing() {
                print!("shader_storage_texel_buffer_array_dynamic_indexing ");
            }
            if vulkan_12_features.shader_uniform_buffer_array_non_uniform_indexing() {
                print!("shader_uniform_buffer_array_non_uniform_indexing ");
            }
            if vulkan_12_features.shader_sampled_image_array_non_uniform_indexing() {
                print!("shader_sampled_image_array_non_uniform_indexing ");
            }
            if vulkan_12_features.shader_storage_buffer_array_non_uniform_indexing() {
                print!("shader_storage_buffer_array_non_uniform_indexing ");
            }
            if vulkan_12_features.shader_storage_image_array_non_uniform_indexing() {
                print!("shader_storage_image_array_non_uniform_indexing ");
            }
            if vulkan_12_features.shader_input_attachment_array_non_uniform_indexing() {
                print!("shader_input_attachment_array_non_uniform_indexing ");
            }
            if vulkan_12_features.shader_uniform_texel_buffer_array_non_uniform_indexing() {
                print!("shader_uniform_texel_buffer_array_non_uniform_indexing ");
            }
            if vulkan_12_features.shader_storage_texel_buffer_array_non_uniform_indexing() {
                print!("shader_storage_texel_buffer_array_non_uniform_indexing ");
            }
            if vulkan_12_features.descriptor_binding_uniform_buffer_update_after_bind() {
                print!("descriptor_binding_uniform_buffer_update_after_bind ");
            }
            if vulkan_12_features.descriptor_binding_sampled_image_update_after_bind() {
                print!("descriptor_binding_sampled_image_update_after_bind ");
            }
            if vulkan_12_features.descriptor_binding_storage_image_update_after_bind() {
                print!("descriptor_binding_storage_image_update_after_bind ");
            }
            if vulkan_12_features.descriptor_binding_storage_buffer_update_after_bind() {
                print!("descriptor_binding_storage_buffer_update_after_bind ");
            }
            if vulkan_12_features.descriptor_binding_uniform_texel_buffer_update_after_bind() {
                print!("descriptor_binding_uniform_texel_buffer_update_after_bind ");
            }
            if vulkan_12_features.descriptor_binding_storage_texel_buffer_update_after_bind() {
                print!("descriptor_binding_storage_texel_buffer_update_after_bind ");
            }
            if vulkan_12_features.descriptor_binding_update_unused_while_pending() {
                print!("descriptor_binding_update_unused_while_pending ");
            }
            if vulkan_12_features.descriptor_binding_partially_bound() {
                print!("descriptor_binding_partially_bound ");
            }
            if vulkan_12_features.descriptor_binding_variable_descriptor_count() {
                print!("descriptor_binding_variable_descriptor_count ");
            }
            if vulkan_12_features.runtime_descriptor_array() {
                print!("runtime_descriptor_array ");
            }
            if vulkan_12_features.sampler_filter_minmax() {
                print!("sampler_filter_minmax ");
            }
            if vulkan_12_features.scalar_block_layout() {
                print!("scalar_block_layout ");
            }
            if vulkan_12_features.imageless_framebuffer() {
                print!("imageless_framebuffer ");
            }
            if vulkan_12_features.uniform_buffer_standard_layout() {
                print!("uniform_buffer_standard_layout ");
            }
            if vulkan_12_features.shader_subgroup_extended_types() {
                print!("shader_subgroup_extended_types ");
            }
            if vulkan_12_features.separate_depth_stencil_layouts() {
                print!("separate_depth_stencil_layouts ");
            }
            if vulkan_12_features.host_query_reset() {
                print!("host_query_reset ");
            }
            if vulkan_12_features.timeline_semaphore() {
                print!("timeline_semaphore ");
            }
            if vulkan_12_features.buffer_device_address() {
                print!("buffer_device_address ");
            }
            if vulkan_12_features.buffer_device_address_capture_replay() {
                print!("buffer_device_address_capture_replay ");
            }
            if vulkan_12_features.buffer_device_address_multi_device() {
                print!("buffer_device_address_multi_device ");
            }
            if vulkan_12_features.vulkan_memory_model() {
                print!("vulkan_memory_model ");
            }
            if vulkan_12_features.vulkan_memory_model_device_scope() {
                print!("vulkan_memory_model_device_scope ");
            }
            if vulkan_12_features.vulkan_memory_model_availability_visibility_chains() {
                print!("vulkan_memory_model_availability_visibility_chains ");
            }
            if vulkan_12_features.shader_output_viewport_index() {
                print!("shader_output_viewport_index ");
            }
            if vulkan_12_features.shader_output_layer() {
                print!("shader_output_layer ");
            }
            if vulkan_12_features.subgroup_broadcast_dynamic_id() {
                print!("subgroup_broadcast_dynamic_id");
            }
            println!();

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

            print!("     - Buffer Device Address Features: ");
            if buffer_device_address_features.buffer_device_address() {
                print!("buffer_device_address ");
            }
            if buffer_device_address_features.buffer_device_address_capture_replay() {
                print!("buffer_device_address_capture_replay ");
            }
            if buffer_device_address_features.buffer_device_address_multi_device() {
                print!("buffer_device_address_multi_device");
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
