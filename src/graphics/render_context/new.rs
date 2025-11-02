#[cfg(debug_assertions)]
use crate::graphics::render_context::info_queue::InfoQueue;
use crate::{
    graphics::{render_context::SWAP_CHAIN_FLAGS, Adapter, GraphicsContext, RenderContext},
    math::Vector2u,
    window::WindowHandle,
    Error, LogCallbacks, Result, BUFFER_COUNT, FORMAT,
};
use std::ptr::null_mut;
use win32::{
    d3d11::{
        D3D11CreateDeviceAndSwapChain, D3D11_BLEND_DESC, D3D11_CREATE_DEVICE_FLAG,
        D3D11_DEPTH_STENCIL_DESC, D3D11_RASTERIZER_DESC, D3D11_SDK_VERSION,
    },
    d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL},
    dxgi::{
        DXGI_MODE_DESC, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT, DXGI_USAGE_RENDER_TARGET_OUTPUT,
    },
    try_hresult, ComPtr, TRUE, UINT,
};

const BASE_DEVICE_FLAGS: UINT = D3D11_CREATE_DEVICE_FLAG::BgraSupport as _;

#[cfg(debug_assertions)]
const DEVICE_FLAGS: UINT = BASE_DEVICE_FLAGS | D3D11_CREATE_DEVICE_FLAG::Debug as UINT;
#[cfg(not(debug_assertions))]
const DEVICE_FLAGS: UINT = BASE_DEVICE_FLAGS;

const FEATURE_LEVELS: &[D3D_FEATURE_LEVEL] = &[D3D_FEATURE_LEVEL::_11_0, D3D_FEATURE_LEVEL::_11_1];

impl RenderContext {
    /// Creates a new [`GraphicsContext`] given the options
    pub(crate) fn new(
        window: &WindowHandle,
        adapter: &mut Adapter,
        width: u32,
        height: u32,
        log_callbacks: &mut dyn LogCallbacks,
    ) -> Result<(Self, GraphicsContext)> {
        // Prepare swapchain description
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            buffer_desc: DXGI_MODE_DESC {
                width,
                height,
                format: FORMAT,
                ..Default::default()
            },
            buffer_usage: DXGI_USAGE_RENDER_TARGET_OUTPUT as _,
            buffer_count: BUFFER_COUNT,
            output_window: **window,
            windowed: TRUE,
            swap_effect: DXGI_SWAP_EFFECT::FlipDiscard,
            flags: SWAP_CHAIN_FLAGS,
            ..Default::default()
        };

        // Create device, device context, and swapchain
        let mut device = null_mut();
        let mut device_context = null_mut();
        let mut swap_chain = null_mut();
        try_hresult!(D3D11CreateDeviceAndSwapChain(
            adapter.handle() as *mut _ as _,
            D3D_DRIVER_TYPE::Unknown,
            null_mut(),
            DEVICE_FLAGS,
            FEATURE_LEVELS.as_ptr(),
            2,
            D3D11_SDK_VERSION,
            &swap_chain_desc,
            &mut swap_chain,
            &mut device,
            null_mut(),
            &mut device_context
        ))
        .map_err(|os| Error::new_os("unable to start D3D11", os))?;

        // Convert raw pointers into `ComPtr`s
        let mut device = ComPtr::new(device);
        let device_context = ComPtr::new(device_context);
        let swapchain = ComPtr::new(swap_chain);

        // Create info queue
        #[cfg(debug_assertions)]
        let info_queue = InfoQueue::new(&mut device)?;

        // Create rasterizer state
        let rasterizer_desc = D3D11_RASTERIZER_DESC {
            front_counter_clockwise: TRUE,
            ..Default::default()
        };
        let rasterizer_state = ComPtr::new_in(|rasterizer_state| {
            try_hresult!(device.create_rasterizer_state(&rasterizer_desc, rasterizer_state))
        })
        .map_err(|error| Error::new_os("unable to create rasterizer state", error))?;

        // Create blend state
        let blend_state_desc = D3D11_BLEND_DESC::default();
        let blend_state = ComPtr::new_in(|blend_state| {
            try_hresult!(device.create_blend_state(&blend_state_desc, blend_state))
        })
        .map_err(|error| Error::new_os("unable to create blend state", error))?;

        // Create depth stencil state
        let depth_stencil_desc = D3D11_DEPTH_STENCIL_DESC::default();
        let depth_stencil_state =
            ComPtr::new_in(|depth_stencil_state| {
                try_hresult!(
                    device.create_depth_stencil_state(&depth_stencil_desc, depth_stencil_state)
                )
            })
            .map_err(|error| Error::new_os("unable to create depth stencil state", error))?;

        // Create render context and graphics context
        let mut render_context = RenderContext {
            current_shader: None,
            swapchain_size: Vector2u::new(width, height),
            swapchain_objects: None,
            swapchain,
            depth_stencil_state,
            rasterizer_state,
            blend_state,
            device_context,
            #[cfg(debug_assertions)]
            info_queue,
        };
        let graphics_context = GraphicsContext::new(device);

        // Force a resize
        render_context
            .force_resize(
                &&graphics_context,
                render_context.swapchain_size,
                log_callbacks,
            )
            .map_err(|error| {
                #[cfg(debug_assertions)]
                render_context
                    .info_queue
                    .empty_queue(log_callbacks)
                    .unwrap();
                error
            })?;

        Ok((render_context, graphics_context))
    }
}
