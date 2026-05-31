mod trampolines;

/// An item which can be used at the callback to XDG toplevel decoration events
pub(in crate::window) trait XdgTopLevelDecorationListener: Sized {
    /// Called when the compositor changes the decoration mode for this surface
    fn configure(&mut self, server_decorations: bool);
}
