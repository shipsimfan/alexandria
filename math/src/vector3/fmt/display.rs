use crate::Vector3;

fn fmt<T: std::fmt::Display>(v: &Vector3<T>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("(")?;
    v.x.fmt(f)?;
    f.write_str(", ")?;
    v.y.fmt(f)?;
    f.write_str(", ")?;
    v.z.fmt(f)?;
    f.write_str(")")
}

impl<T: std::fmt::Display> std::fmt::Display for Vector3<T> {
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
