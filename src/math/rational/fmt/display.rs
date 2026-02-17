use crate::math::Rational;

fn fmt(v: &Rational, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use std::fmt::Display;

    v.numerator.fmt(f)?;
    f.write_str(" / ")?;
    v.denominator.fmt(f)
}

impl std::fmt::Display for Rational {
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
