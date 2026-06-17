//! Procedural macros for alexandria

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(proc_macro_diagnostic)]

mod compile_shader;

proc_macro_util::proc_macro_function!(
    /// Compiles a shader at compile time and embeds the resulting SPIR-V binary into the compiled
    /// crate
    ///
    /// The first argument is the definition of the constant, with its type excluded and the path
    /// to the shader file set as its value. The remaining arguments are the entry point names to
    /// compile.
    ///
    /// # Examples
    /// ```rust
    /// use alexandria_macros::compile_shader;
    ///
    /// compile_shader!(
    ///     /// Docs about the shader
    ///     pub const SHADER = "shader.slang",
    ///     vert_main,
    ///     frag_main
    /// );
    /// ```
    compile_shader -> compile_shader::CompileShader
);
