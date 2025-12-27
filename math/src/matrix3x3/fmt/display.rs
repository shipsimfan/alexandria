use crate::Matrix3x3;

impl<T: std::fmt::Display> std::fmt::Display for Matrix3x3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let mut values = Vec::new();
            let mut longest_value = 0;
            for row in self {
                for value in row {
                    let mut string = String::new();
                    value.fmt(&mut std::fmt::Formatter::new(&mut string, f.options()))?;
                    if string.len() > longest_value {
                        longest_value = string.len();
                    }
                    values.push(string);
                }
            }

            for r in 0..3 {
                match r {
                    0 => write!(f, "⎡"),
                    1 => write!(f, "⎢"),
                    2 => write!(f, "⎣"),
                    _ => unreachable!(),
                }?;

                for c in 0..3 {
                    write!(f, " ")?;
                    let value_str = &values[c + r * 3];
                    for _ in 0..longest_value - value_str.len() {
                        write!(f, " ")?;
                    }
                    write!(f, "{} ", value_str)?;
                }

                match r {
                    0 => writeln!(f, "⎤"),
                    1 => writeln!(f, "⎥"),
                    2 => writeln!(f, "⎦"),
                    _ => unreachable!(),
                }?;
            }

            Ok(())
        } else {
            write!(f, "[")?;
            for (r, row) in self.row_iter().enumerate() {
                write!(f, "[")?;
                for (c, value) in row.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if c < 2 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")?;
                if r < 2 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")
        }
    }
}
