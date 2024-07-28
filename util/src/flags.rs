/// Checks if `flags` contains `bit`
#[macro_export]
macro_rules! flags_contains {
    ($flags: expr, $bit: expr) => {
        $flags as u32 & $bit as u32 != 0
    };
}
