#[cfg(feature = "hlsl")]
use lct_diagnostics::{emitters::HumanReadableEmitter, DiagCtxt, SourceMap, SourceName};
#[cfg(feature = "hlsl")]
use std::io::stdout;

#[cfg(feature = "hlsl")]
#[macro_export]
macro_rules! hlsl_test {
    ($name: ident) => {
        #[test]
        fn $name() {
            const INPUT_NAME: &str = concat!("hlsl/input/", stringify!($name), ".acsl");
            const INPUT: &[u8] = include_bytes!(concat!("hlsl/input/", stringify!($name), ".acsl"));
            const TARGET: &str = include_str!(concat!("hlsl/output/", stringify!($name), ".hlsl"));

            $crate::hlsl::run_hlsl_test(INPUT_NAME, INPUT, TARGET);
        }
    };
}

#[cfg(not(feature = "hlsl"))]
#[macro_export]
macro_rules! hlsl_test {
    ($name: ident) => {};
}

#[cfg(feature = "hlsl")]
pub fn run_hlsl_test(input_name: &str, input: &[u8], target: &str) {
    let source_map: SourceMap = SourceMap::new();
    let diag = DiagCtxt::new(&source_map, HumanReadableEmitter::new(stdout().lock()));
    source_map.push(SourceName::Real(input_name.into()), input.to_vec(), ());

    let program = match acsl::compile(&source_map.sources()[0], &diag) {
        Ok(program) => program,
        Err(error) => {
            error.emit();
            panic!("An error occurred during compilation");
        }
    };

    let hlsl = acsl::HlslProgram::lower(program);
    println!();
    println!("\x1B[1;36mHLSL:\x1B[0m");
    print!("{}", hlsl.content());
    assert_eq!(hlsl.content(), target);
}
