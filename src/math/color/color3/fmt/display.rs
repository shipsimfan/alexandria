use crate::math::{Color3, ColorSpace};

fn fmt<T: std::fmt::Display, Space: ColorSpace<T>>(
    c: &Color3<T, Space>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    f.write_str("(")?;
    c.r.fmt(f)?;
    f.write_str(", ")?;
    c.g.fmt(f)?;
    f.write_str(", ")?;
    c.b.fmt(f)?;
    f.write_str(")")
}

impl<T: std::fmt::Display, Space: ColorSpace<T>> std::fmt::Display for Color3<T, Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.align().is_none() && f.width().is_none() {
            return fmt(self, f);
        }

        let mut string = String::new();
        let mut options = f.options();
        options.align(None).width(None);
        let mut string_f = std::fmt::Formatter::new(&mut string, options);

        fmt(self, &mut string_f)?;
        string.fmt(f)
    }
}
