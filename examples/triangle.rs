use render_context::{RenderContext, Swapchain};

use crate::render_context::SWAPCHAIN_FORMAT;

mod render_context;

alexandria::gpu::compile_shader!(const SHADER = "triangle.slang", vert_main, frag_main);

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
                    command_buffer.cmd_draw(3, 1, 0, 0);
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

    drop(graphics_pipeline);

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

    let vertex_input_state =
        alexandria::gpu::VulkanPipelineVertexInputStateCreateInfo::new(&[], &[]);

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
