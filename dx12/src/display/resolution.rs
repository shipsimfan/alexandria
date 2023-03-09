#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resolution {
    width: usize,
    height: usize,
}

impl Resolution {
    pub fn new(width: usize, height: usize) -> Self {
        Resolution { width, height }
    }

    pub fn parse(resolution: &str) -> Option<Self> {
        let mut parts = resolution.split('x');

        let width = match parts.next() {
            Some(width) => match width.parse() {
                Ok(width) => width,
                Err(_) => return None,
            },
            None => return None,
        };

        let height = match parts.next() {
            Some(height) => match height.parse() {
                Ok(height) => height,
                Err(_) => return None,
            },
            None => return None,
        };

        match parts.next() {
            Some(_) => None,
            None => Some(Resolution::new(width, height)),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} x {}", self.width, self.height)
    }
}
