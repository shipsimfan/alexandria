use render_context::{RenderContext, SWAPCHAIN_FORMAT, Swapchain};

mod render_context;

alexandria::gpu::compile_shader!(const SHADER = "triangle.slang", vert_main, frag_main);

const VERTICES: &[Vertex] = &[
    Vertex {
        position: alexandria::math::Vector2f::new(0.0, -0.5),
        color: alexandria::math::Color3f::<alexandria::math::Linear>::new(1.0, 0.0, 0.0),
    },
    Vertex {
        position: alexandria::math::Vector2f::new(0.5, 0.5),
        color: alexandria::math::Color3f::<alexandria::math::Linear>::new(0.0, 1.0, 0.0),
    },
    Vertex {
        position: alexandria::math::Vector2f::new(-0.5, 0.5),
        color: alexandria::math::Color3f::<alexandria::math::Linear>::new(0.0, 0.0, 1.0),
    },
];

const INDICES: &[u32] = &[0, 1, 2];

fn main() {
    // Create the Alexandria context with GPU and window support
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .gpu()
        .window()
        .create()
        .expect("unable to create Alexandria context");

    // Create a window
    let window = context
        .window()
        .create_window("Blank Window")
        .resizable()
        .create()
        .expect("unable to create window");
    println!(
        "Window created at {} with size {}x{}",
        window.position(),
        window.width(),
        window.height()
    );

    // Create the render context
    let (mut render_context, mut surface) = RenderContext::new(&context, &window);

    // Create the shader
    let shader = render_context.create_shader_module(&SHADER).unwrap();

    // Create the pipeline layout
    let pipeline_layout = render_context.create_pipeline_layout(0, None, &[]).unwrap();

    // Create the graphics pipeline
    let graphics_pipeline = create_graphics_pipeline(&render_context, &pipeline_layout, &shader);

    // Create the vertex buffer
    let vertex_buffer = create_device_buffer(
        &mut render_context,
        alexandria::gpu::VulkanBufferUsageFlag::VertexBuffer,
        VERTICES,
    );
    let index_buffer = create_device_buffer(
        &mut render_context,
        alexandria::gpu::VulkanBufferUsageFlag::IndexBuffer,
        INDICES,
    );

    // Create the swapchain and image views
    let mut swapchain = Swapchain::new(&mut render_context, &mut surface, &window);

    // Run the main event loop
    let mut running = true;
    let mut should_recreate_swapchain = false;
    while running {
        // Render a frame if possible
        let rendered = if !window.is_minimized() {
            let rendered = swapchain.render_frame(
                &mut render_context,
                alexandria::math::Color3f::<alexandria::math::Linear>::new(0.0, 0.0, 0.0),
                |swapchain_size, command_buffer| {
                    let viewport = alexandria::gpu::VulkanViewport::new(
                        0.0,
                        0.0,
                        swapchain_size.x as _,
                        swapchain_size.y as _,
                        0.0,
                        1.0,
                    );
                    let scissor = alexandria::math::Recti::new(
                        alexandria::math::Vector2::ZERO,
                        swapchain_size,
                    );
                    command_buffer.cmd_set_viewport(0, &[viewport]);
                    command_buffer.cmd_set_scissor(0, &[scissor]);

                    command_buffer.cmd_bind_pipeline(
                        alexandria::gpu::VulkanPipelineBindPoint::Graphics,
                        &graphics_pipeline,
                    );
                    command_buffer.cmd_bind_vertex_buffer(0, &vertex_buffer.buffer, 0);
                    command_buffer.cmd_bind_index_buffer(
                        &index_buffer.buffer,
                        0,
                        alexandria::gpu::VulkanIndexType::Uint32,
                    );

                    command_buffer.cmd_draw_indexed(INDICES.len() as _, 1, 0, 0, 0);
                },
            );
            if !rendered {
                should_recreate_swapchain = true;
            }
            rendered
        } else {
            false
        };

        if rendered || window.is_minimized() {
            // Wait for the next event and handle it
            let event = pump.wait().expect("unable to wait for event");
            running &= handle_event(&event, &mut should_recreate_swapchain);
        }

        // Poll for any additional events that may have occurred while handling the previous event
        while let Some(event) = pump.poll().expect("unable to poll for event") {
            running &= handle_event(&event, &mut should_recreate_swapchain);
        }

        // Recreate the swapchain when appropriate
        if should_recreate_swapchain && !window.is_minimized() {
            should_recreate_swapchain = false;

            render_context.wait_idle().unwrap();

            drop(swapchain);
            swapchain = Swapchain::new(&mut render_context, &mut surface, &window);
        }
    }

    render_context.wait_idle().unwrap();

    drop(vertex_buffer);

    window.destroy().expect("unable to destroy window");
}

/// Handles an event and returns whether the application should continue running
fn handle_event(event: &alexandria::Event<()>, should_recreate_swapchain: &mut bool) -> bool {
    match event.kind {
        alexandria::EventKind::Quit | alexandria::EventKind::WindowCloseRequest { .. } => false,
        alexandria::EventKind::WindowResized { .. } => {
            *should_recreate_swapchain = true;
            true
        }
        _ => true,
    }
}

fn create_graphics_pipeline(
    render_context: &RenderContext,
    pipeline_layout: &alexandria::gpu::VulkanPipelineLayout,
    shader: &alexandria::gpu::VulkanShaderModule,
) -> alexandria::gpu::VulkanPipeline {
    let stages = [
        alexandria::gpu::VulkanPipelineShaderStageCreateInfo::new(
            0,
            alexandria::gpu::VulkanShaderStageFlag::Vertex,
            shader,
            SHADER.entry_points()[0],
            None,
        ),
        alexandria::gpu::VulkanPipelineShaderStageCreateInfo::new(
            0,
            alexandria::gpu::VulkanShaderStageFlag::Fragment,
            shader,
            SHADER.entry_points()[1],
            None,
        ),
    ];

    let dynamic_state = alexandria::gpu::VulkanPipelineDynamicStateCreateInfo::new(&[
        alexandria::gpu::VulkanDynamicState::Viewport,
        alexandria::gpu::VulkanDynamicState::Scissor,
    ]);

    let attribute_descriptions = Vertex::attribute_descriptions();
    let binding_descriptions = Vertex::binding_descriptions();
    let vertex_input_state = alexandria::gpu::VulkanPipelineVertexInputStateCreateInfo::new(
        &attribute_descriptions,
        &binding_descriptions,
    );

    let input_assembly_state = alexandria::gpu::VulkanPipelineInputAssemblyStateCreateInfo::new(
        alexandria::gpu::VulkanPrimitiveTopology::TriangleList,
        false,
    );

    let viewport_state = alexandria::gpu::VulkanPipelineViewportStateCreateInfo::new_dynamic(1, 1);

    let rasterization_state = alexandria::gpu::VulkanPipelineRasterizationStateCreateInfo::new(
        false,
        false,
        alexandria::gpu::VulkanPolygonMode::Fill,
        alexandria::gpu::VulkanCullModeFlag::Back,
        alexandria::gpu::VulkanFrontFace::Clockwise,
        false,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    let multisample_state = alexandria::gpu::VulkanPipelineMultisampleStateCreateInfo::new(
        alexandria::gpu::VulkanSampleCountFlag::_1,
        false,
        0.0,
        None,
        false,
        false,
    );

    let color_blend_attachment = [
        alexandria::gpu::VulkanPipelineColorBlendAttachmentState::new(
            false,
            alexandria::gpu::VulkanBlendFactor::Zero,
            alexandria::gpu::VulkanBlendFactor::Zero,
            alexandria::gpu::VulkanBlendOp::Add,
            alexandria::gpu::VulkanBlendFactor::Zero,
            alexandria::gpu::VulkanBlendFactor::Zero,
            alexandria::gpu::VulkanBlendOp::Add,
            alexandria::gpu::VulkanColorComponentFlag::R
                | alexandria::gpu::VulkanColorComponentFlag::G
                | alexandria::gpu::VulkanColorComponentFlag::B
                | alexandria::gpu::VulkanColorComponentFlag::A,
        ),
    ];
    let color_blend_state = alexandria::gpu::VulkanPipelineColorBlendStateCreateInfo::new(
        0,
        false,
        alexandria::gpu::VulkanLogicOp::Copy,
        &color_blend_attachment,
        alexandria::math::Color4f::<alexandria::math::Linear>::CLEAR,
    );

    render_context
        .create_graphics_pipeline(
            [
                &mut alexandria::gpu::VulkanPipelineRenderingCreateInfo::new(
                    0,
                    &[SWAPCHAIN_FORMAT],
                    alexandria::gpu::VulkanFormat::Undefined,
                    alexandria::gpu::VulkanFormat::Undefined,
                ) as _,
            ],
            None,
            0,
            &stages,
            Some(&vertex_input_state),
            Some(&input_assembly_state),
            None,
            Some(&viewport_state),
            Some(&rasterization_state),
            Some(&multisample_state),
            None,
            Some(&color_blend_state),
            Some(&dynamic_state),
            Some(pipeline_layout),
            None,
            0,
            None,
            0,
        )
        .unwrap()
}

struct Buffer {
    buffer: alexandria::gpu::VulkanBuffer,

    #[allow(unused)]
    memory: alexandria::gpu::VulkanDeviceMemory,
}

fn create_device_buffer<T: Copy, U: Into<alexandria::gpu::VulkanBufferUsageFlags>>(
    render_context: &mut RenderContext,
    properties: U,
    data: &[T],
) -> Buffer {
    let buffer_size = data.len() * std::mem::size_of::<T>();

    // Create the staging buffer
    let staging_buffer = create_buffer(
        render_context,
        buffer_size,
        alexandria::gpu::VulkanBufferUsageFlag::TransferSrc,
        alexandria::gpu::VulkanMemoryPropertyFlag::HostVisible
            | alexandria::gpu::VulkanMemoryPropertyFlag::HostCoherent,
    );

    // Fill the staging buffer with vertex data
    let mut mapped_memory = staging_buffer
        .memory
        .map(0, buffer_size, 0)
        .map_err(|(error, _)| error)
        .unwrap();
    mapped_memory.copy_from_slice(data);
    let staging_buffer_memory = mapped_memory.unmap();

    // Create the vertex buffer
    let vertex_buffer = create_buffer(
        render_context,
        buffer_size,
        properties.into() | alexandria::gpu::VulkanBufferUsageFlag::TransferDst,
        alexandria::gpu::VulkanMemoryPropertyFlag::DeviceLocal,
    );

    // Copy the data from the staging buffer to the vertex buffer
    copy_buffer(
        render_context,
        &staging_buffer.buffer,
        &vertex_buffer.buffer,
        buffer_size,
    );

    drop(staging_buffer_memory);

    vertex_buffer
}

fn create_buffer<
    U: Into<alexandria::gpu::VulkanBufferUsageFlags>,
    P: Into<alexandria::gpu::VulkanMemoryPropertyFlags>,
>(
    render_context: &RenderContext,
    size: usize,
    usage: U,
    properties: P,
) -> Buffer {
    // Create the buffer
    let mut buffer = render_context
        .create_buffer(
            0,
            size as _,
            usage,
            alexandria::gpu::VulkanSharingMode::Exclusive,
            &[],
        )
        .unwrap();

    // Allocate memory for the buffer
    let memory_requirements = buffer.get_memory_requirements();
    let memory_type = find_memory_type(
        render_context.memory_properties(),
        memory_requirements.memory_type_bits(),
        properties,
    );
    let memory = render_context
        .allocate_memory(memory_requirements.size(), memory_type)
        .unwrap();

    // Bind the buffer and memory
    buffer.bind_memory(&memory, 0).unwrap();

    Buffer { buffer, memory }
}

fn find_memory_type<F: Into<alexandria::gpu::VulkanMemoryPropertyFlags>>(
    memory_properties: &alexandria::gpu::VulkanAdapterMemoryProperties,
    type_filter: u32,
    properties: F,
) -> usize {
    let properties = properties.into();

    for (i, memory_type) in memory_properties.memory_types().iter().enumerate() {
        if (type_filter & (1 << i)) != 0 && memory_type.flags().contains(properties) {
            return i;
        }
    }

    panic!("Failed to find suitable memory type");
}

fn copy_buffer(
    render_context: &mut RenderContext,
    src_buffer: &alexandria::gpu::VulkanBuffer,
    dst_buffer: &alexandria::gpu::VulkanBuffer,
    size: usize,
) {
    let (command_pool, queue) = render_context.command_pool();

    // Allocate a command buffer for the copy operation
    let command_buffer_id = command_pool
        .allocate_command_buffer(alexandria::gpu::VulkanCommandBufferLevel::Primary)
        .unwrap();

    // Record the copy command in the command buffer
    let command_buffer = &mut command_pool[command_buffer_id];
    command_buffer.begin().unwrap();
    command_buffer.cmd_copy_buffer(
        src_buffer,
        dst_buffer,
        &[alexandria::gpu::VulkanBufferCopy::new(0, 0, size as _)],
    );
    command_buffer.end().unwrap();

    // Submit the command buffer to the queue and wait for it to finish
    queue
        .submit(
            &[alexandria::gpu::VulkanSubmitInfo::new(
                0,
                &[],
                &[alexandria::gpu::VulkanCommandBufferSubmitInfo::new(
                    command_buffer,
                    0,
                )],
                &[],
            )],
            None,
        )
        .unwrap();

    queue.wait_idle().unwrap();

    // Free the command buffer after the copy operation is complete
    command_pool.free_command_buffer(command_buffer_id);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: alexandria::math::Vector2f,
    color: alexandria::math::Color3f<alexandria::math::Linear>,
}

impl Vertex {
    /// Returns the vertex input binding description for this vertex type
    fn binding_descriptions() -> [alexandria::gpu::VulkanVertexInputBindingDescription; 1] {
        [alexandria::gpu::VulkanVertexInputBindingDescription::new(
            0,
            std::mem::size_of::<Vertex>() as _,
            alexandria::gpu::VulkanVertexInputRate::Vertex,
        )]
    }

    /// Returns the vertex input attribute descriptions for this vertex type
    fn attribute_descriptions() -> [alexandria::gpu::VulkanVertexInputAttributeDescription; 2] {
        [
            alexandria::gpu::VulkanVertexInputAttributeDescription::new(
                0,
                0,
                alexandria::gpu::VulkanFormat::R32G32SFloat,
                std::mem::offset_of!(Vertex, position) as _,
            ),
            alexandria::gpu::VulkanVertexInputAttributeDescription::new(
                1,
                0,
                alexandria::gpu::VulkanFormat::R32G32B32SFloat,
                std::mem::offset_of!(Vertex, color) as _,
            ),
        ]
    }
}
