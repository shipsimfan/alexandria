use crate::{
    EventKind, Result,
    window::{
        display::DisplayInner,
        subsystem::linux::wayland::{WaylandGlobals, WlRegistryRef},
    },
};
use std::{ffi::CStr, rc::Rc};

impl<UserEvent: 'static + Send> WaylandGlobals<UserEvent> {
    /// Insert a new global into the globals list
    pub(in crate::window::subsystem::linux::wayland::globals) fn add_global(
        &mut self,
        registry: &mut WlRegistryRef,
        name: u32,
        interface: &CStr,
        version: u32,
    ) -> Result<()> {
        if interface == self.wl_output_manager_name {
            let display = DisplayInner::new_wayland(
                registry,
                name,
                version,
                self.event_queue.clone(),
                self.xdg_output_manager.as_ref(),
            )?;

            let id = self.displays.insert(display);
            let cast_id = unsafe { id.cast() };

            self.displays[id].set_display_id(cast_id);

            if self.events_enabled {
                self.event_queue
                    .push(EventKind::DisplayAdded { id: cast_id })?;
            }
        } else if interface == self.xdg_output_name {
            let xdg_output_manager = Rc::new(registry.bind(name, version)?);
            for display in &mut self.displays {
                display.wayland_upgrade(&xdg_output_manager, self.events_enabled)?;
            }
            self.xdg_output_manager = Some(xdg_output_manager);
        }

        /*
        else if self.compositor.is_none() && interface == self.compositor_name {
             self.compositor = Some( registry.bind(name, version)? )
        } else if self.xdg_wm_base.is_none() && interface == self.xdg_wm_base_name {
             self.xdg_wm_base = Some(Rc::new(
                registry
                    .bind::<XdgWmBase>(name, version)?
                    .register_ping_handler()?
             ));
        }
        */

        Ok(())
    }
}
