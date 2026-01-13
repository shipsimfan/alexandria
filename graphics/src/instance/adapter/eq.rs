use crate::GraphicsAdapter;

impl<'instance> PartialEq for GraphicsAdapter<'instance> {
    fn eq(&self, other: &Self) -> bool {
        self.api_version == other.api_version
            && self.kind == other.kind
            && self.name == other.name
            && self.uuid == other.uuid
            && self.vram == other.vram
            && self.queue_families == other.queue_families
    }
}

impl<'instance> Eq for GraphicsAdapter<'instance> {}
