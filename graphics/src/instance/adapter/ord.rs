use crate::GraphicsAdapter;
use std::cmp::Ordering;

impl<'instance> PartialOrd for GraphicsAdapter<'instance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'instance> Ord for GraphicsAdapter<'instance> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.vram.cmp(&other.vram) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.api_version.cmp(&other.api_version) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.queue_families.len().cmp(&other.queue_families.len()) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.name.cmp(&other.name) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        match self.driver_version.cmp(&other.driver_version) {
            Ordering::Equal => {}
            ordering => return ordering,
        }

        self.uuid.cmp(&other.uuid)
    }
}
