use crate::MemorySize;

const KILO: u64 = 1024;
const MEGA: u64 = KILO * 1024;
const GIGA: u64 = MEGA * 1024;

impl std::fmt::Display for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < KILO {
            return write!(f, "{}B", self.0);
        }

        let (divisor, suffix) = if self.0 < MEGA {
            (KILO, "KiB")
        } else if self.0 < GIGA {
            (MEGA, "MiB")
        } else {
            (GIGA, "GiB")
        };

        let integer = self.0 / divisor;

        let precision = f.precision().unwrap_or(2);
        if precision == 0 {
            return write!(f, "{}{}", integer, suffix);
        }

        let multiplier = 10u64.pow(precision as u32);
        let fract = (multiplier * (self.0 % divisor)) / divisor;

        write!(
            f,
            "{}.{:0width$}{}",
            integer,
            fract,
            suffix,
            width = precision
        )
    }
}
