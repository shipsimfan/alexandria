use crate::program::types::StructFieldMeta;

impl std::fmt::Display for StructFieldMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructFieldMeta::Position => "position",
        }
        .fmt(f)
    }
}
