use crate::Program;

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r#type in &self.types {
            writeln!(f, "{}", r#type)?;
        }

        Ok(())
    }
}
