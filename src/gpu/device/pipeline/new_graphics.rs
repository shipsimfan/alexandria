use crate::{
    Error, Result,
    gpu::{
        VulkanDevice, VulkanGraphicsPipelineExtendedCreateInfo, VulkanPipeline,
        VulkanPipelineCache, VulkanPipelineColorBlendStateCreateInfo, VulkanPipelineCreateFlags,
        VulkanPipelineDepthStencilStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
        VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineLayout,
        VulkanPipelineMultisampleStateCreateInfo, VulkanPipelineRasterizationStateCreateInfo,
        VulkanPipelineShaderStageCreateInfo, VulkanPipelineTessellationStateCreateInfo,
        VulkanPipelineVertexInputStateCreateInfo, VulkanPipelineViewportStateCreateInfo,
        VulkanRenderPass,
    },
};
use std::ptr::null;
use vulkan::{
    VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineCache, VkPipelineLayout, VkRenderPass,
    try_vulkan, util::create_next_chain,
};

impl VulkanPipeline {
    /// Create a new graphics [`VulkanPipeline`]
    pub(in crate::gpu::device) fn new_graphics<
        'a,
        I: IntoIterator<Item = &'a mut (dyn VulkanGraphicsPipelineExtendedCreateInfo + 'a)>,
    >(
        extended: I,
        pipeline_cache: Option<&VulkanPipelineCache>,
        flags: VulkanPipelineCreateFlags,
        stages: &[VulkanPipelineShaderStageCreateInfo],
        vertex_input_state: Option<&VulkanPipelineVertexInputStateCreateInfo>,
        input_assembly_state: Option<&VulkanPipelineInputAssemblyStateCreateInfo>,
        tessellation_state: Option<&VulkanPipelineTessellationStateCreateInfo>,
        viewport_state: Option<&VulkanPipelineViewportStateCreateInfo>,
        rasterization_state: Option<&VulkanPipelineRasterizationStateCreateInfo>,
        multisample_state: Option<&VulkanPipelineMultisampleStateCreateInfo>,
        depth_stencil_state: Option<&VulkanPipelineDepthStencilStateCreateInfo>,
        color_blend_state: Option<&VulkanPipelineColorBlendStateCreateInfo>,
        dynamic_state: Option<&VulkanPipelineDynamicStateCreateInfo>,
        layout: Option<&VulkanPipelineLayout>,
        render_pass: Option<&VulkanRenderPass>,
        subpass: u32,
        base_pipeline: Option<&VulkanPipeline>,
        base_pipeline_index: i32,
        device: &VulkanDevice,
    ) -> Result<VulkanPipeline> {
        let mut create_info = VkGraphicsPipelineCreateInfo {
            flags,
            stage_count: stages.len() as u32,
            stages: stages.as_ptr().cast(),
            vertex_input_state: vertex_input_state.map_or(null(), |state| state.as_ptr()),
            input_assembly_state: input_assembly_state.map_or(null(), |state| state.as_ptr()),
            tessellation_state: tessellation_state.map_or(null(), |state| state.as_ptr()),
            viewport_state: viewport_state.map_or(null(), |state| state.as_ptr()),
            rasterization_state: rasterization_state.map_or(null(), |state| state.as_ptr()),
            multisample_state: multisample_state.map_or(null(), |state| state.as_ptr()),
            depth_stencil_state: depth_stencil_state.map_or(null(), |state| state.as_ptr()),
            color_blend_state: color_blend_state.map_or(null(), |state| state.as_ptr()),
            dynamic_state: dynamic_state.map_or(null(), |state| state.as_ptr()),
            layout: layout.map_or(VkPipelineLayout::null(), |layout| layout.handle()),
            render_pass: render_pass.map_or(VkRenderPass::null(), |pass| pass.handle()),
            subpass,
            base_pipeline_handle: base_pipeline
                .map_or(VkPipeline::null(), |pipeline| pipeline.handle),
            base_pipeline_index,
            ..Default::default()
        };

        create_next_chain(
            [&mut create_info as _]
                .into_iter()
                .chain(extended.into_iter().map(|info| info as _)),
        );

        let mut handle = VkPipeline::null();
        try_vulkan!((device.functions().pipeline.create_graphics_pipelines)(
            device.handle(),
            pipeline_cache.map_or(VkPipelineCache::null(), |cache| cache.handle()),
            1,
            &create_info,
            null(),
            &mut handle
        ))
        .map_err(|vk| Error::new_with("unable to create a graphics pipeline", vk))?;

        Ok(VulkanPipeline {
            handle,
            device: device.clone(),
        })
    }
}
