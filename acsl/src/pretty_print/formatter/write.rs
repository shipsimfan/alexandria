use crate::pretty_print::PrettyFormatter;

impl<'a, 'b> PrettyFormatter<'a, 'b> {
    #[inline]
    pub fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.f.write_fmt(fmt)
    }
}

impl<'a, 'b> std::fmt::Write for PrettyFormatter<'a, 'b> {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.f.write_char(c)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.write_fmt(args)
    }

    fn write_str(&mut self, data: &str) -> std::fmt::Result {
        self.f.write_str(data)
    }
}
