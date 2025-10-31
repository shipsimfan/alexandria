use crate::{D3DProgram, HlslProgram, InputLayout};
use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};
use win32::{d3dcompiler::D3DCompile, try_hresult, unknwn::IUnknownTrait, ComPtr};

/// A result the can occur when compiling HLSL
#[derive(Debug)]
pub enum D3DCompileError {
    /// The result is from the function call itself
    HResult(win32::Error),

    /// The result is in the provided code
    String(String),
}

/// Compile an [`HlslProgram`] into a [`D3DProgram`]
pub fn d3dcompile(program: &HlslProgram) -> Result<D3DProgram<'static>, D3DCompileError> {
    let vertex_content = do_compile(program.content(), program.vertex_entry(), c"vs_5_0")?;
    let pixel_content = do_compile(program.content(), program.pixel_entry(), c"ps_5_0")?;

    Ok(D3DProgram::new(
        vertex_content.into(),
        pixel_content.into(),
        InputLayout::new(),
    ))
}

fn do_compile(content: &str, entry: &str, target: &CStr) -> Result<Vec<u8>, D3DCompileError> {
    let entry = CString::new(entry).unwrap();
    let mut error_msgs = null_mut();
    let result = ComPtr::new_in(|code| {
        try_hresult!(D3DCompile(
            content.as_ptr().cast(),
            content.len() as _,
            null(),
            null(),
            null_mut(),
            entry.as_ptr(),
            target.as_ptr(),
            0,
            0,
            code,
            &mut error_msgs
        ))
    })
    .map(|mut blob| {
        unsafe {
            std::slice::from_raw_parts(
                blob.get_buffer_pointer().cast(),
                blob.get_buffer_size() as _,
            )
        }
        .to_vec()
    })
    .map_err(|error| {
        if error_msgs == null_mut() {
            D3DCompileError::HResult(error)
        } else {
            let error_msgs = unsafe { &mut *error_msgs };
            let msgs = unsafe {
                std::slice::from_raw_parts(
                    error_msgs.get_buffer_pointer().cast(),
                    error_msgs.get_buffer_size() as _,
                )
            };
            D3DCompileError::String(String::from_utf8_lossy(msgs).to_string())
        }
    });
    if error_msgs != null_mut() {
        let error_msgs = unsafe { &mut *error_msgs };
        error_msgs.release();
    }
    result
}

impl std::error::Error for D3DCompileError {}

impl std::fmt::Display for D3DCompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            D3DCompileError::HResult(error) => error.fmt(f),
            D3DCompileError::String(string) => string.fmt(f),
        }
    }
}
