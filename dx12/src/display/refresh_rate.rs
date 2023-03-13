#[derive(Clone, Copy)]
pub struct RefreshRate {
    numerator: usize,
    denominator: usize,
}

impl RefreshRate {
    pub(crate) fn new(numerator: usize, denominator: usize) -> Self {
        RefreshRate {
            numerator,
            denominator,
        }
    }

    pub fn parse(refresh_rate: &str) -> Option<Self> {
        let mut parts = refresh_rate.split('/');

        let numerator = match parts.next() {
            Some(numerator) => match numerator.parse() {
                Ok(numerator) => numerator,
                Err(_) => return None,
            },
            None => return None,
        };

        let denominator = match parts.next() {
            Some(denominator) => match denominator.parse() {
                Ok(denominator) => denominator,
                Err(_) => return None,
            },
            None => return None,
        };

        match parts.next() {
            Some(_) => None,
            None => Some(RefreshRate::new(numerator, denominator)),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }

    pub fn as_f32(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    pub(crate) fn into_rational(&self) -> win32::DXGIRational {
        win32::DXGIRational::new(self.numerator as u32, self.denominator as u32)
    }
}

impl std::fmt::Display for RefreshRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let refresh_rate = format!("{:.2}", self.as_f32());
        write!(
            f,
            "{} Hz",
            refresh_rate.trim_end_matches('0').trim_end_matches('.')
        )
    }
}

impl PartialEq for RefreshRate {
    fn eq(&self, other: &Self) -> bool {
        self.as_f32().eq(&other.as_f32())
    }
}

impl Eq for RefreshRate {}

impl PartialOrd for RefreshRate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RefreshRate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_f32().total_cmp(&other.as_f32())
    }
}
