use crate::window::DisplayMode;
use std::cmp::Ordering;

impl PartialOrd for DisplayMode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DisplayMode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.size.x.cmp(&other.size.x) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.size.y.cmp(&other.size.y) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        self.refresh_rate.cmp(&other.refresh_rate)
    }
}
