use crate::{
    Result,
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

impl VulkanDevice {
    /// Create a new graphics [`VulkanPipeline`]
    pub fn create_graphics_pipeline<
        'a,
        F: Into<VulkanPipelineCreateFlags>,
        I: IntoIterator<Item = &'a mut (dyn VulkanGraphicsPipelineExtendedCreateInfo + 'a)>,
    >(
        &self,
        extended: I,
        pipeline_cache: Option<&VulkanPipelineCache>,
        flags: F,
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
    ) -> Result<VulkanPipeline> {
        VulkanPipeline::new_graphics(
            extended,
            pipeline_cache,
            flags.into(),
            stages,
            vertex_input_state,
            input_assembly_state,
            tessellation_state,
            viewport_state,
            rasterization_state,
            multisample_state,
            depth_stencil_state,
            color_blend_state,
            dynamic_state,
            layout,
            render_pass,
            subpass,
            base_pipeline,
            base_pipeline_index,
            self,
        )
    }
}
