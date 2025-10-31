//! Macros to help Alexandria

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(proc_macro_diagnostic)]

mod compile_hlsl;
mod compile_hlsl_file;

proc_macro_util::proc_macro_function!(
    /// Compiles an HLSL file into a CompiledShader, producing the struct in place of this macro.
    ///
    /// # Format
    /// ```ignore
    /// compile_hlsl!(content: literal, vertex_main: literal, pixel_main: literal, input_layout: expr);
    /// ```
    ///
    /// # Parameters
    ///  * `content` - String literal containing the code content
    ///  * `vertex_main` - The name of the main vertex function
    ///  * `pixel_main` - The name of the main pixel function
    ///  * `input_layout` - The input layout to use for the shader
    compile_hlsl -> compile_hlsl::CompileHlsl
);

proc_macro_util::proc_macro_function!(
    /// Compiles an HLSL file into a CompiledShader, producing the struct in place of this macro.
    ///
    /// # Format
    /// ```ignore
    /// compile_hlsl_file!(file_name: literal, vertex_main: literal, pixel_main: literal, input_layout: expr);
    /// ```
    ///
    /// # Parameters
    ///  * `file_name` - The name of the file, relative to the defining module file
    ///  * `vertex_main` - The name of the main vertex function
    ///  * `pixel_main` - The name of the main pixel function
    ///  * `input_layout` - The input layout to use for the shader
    compile_hlsl_file -> compile_hlsl_file::CompileHlslFile
);
