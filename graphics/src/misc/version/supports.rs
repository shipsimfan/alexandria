use crate::GraphicsVersion;

impl GraphicsVersion {
    /// Does this version support `version`
    pub const fn supports(&self, version: &GraphicsVersion) -> bool {
        let self_variant = self.variant();
        let self_major = self.major();
        if self_variant != version.variant() || self_major != version.major() {
            return false;
        }

        let self_minor = self.minor();
        let other_minor = version.minor();
        if self_minor < other_minor {
            return false;
        }

        if self_minor > other_minor {
            return true;
        }

        if self_variant == 0 && self_major == 0 && self_minor == 0 {
            self.patch() == version.patch()
        } else {
            self.patch() >= version.patch()
        }
    }
}
