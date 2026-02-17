use crate::{EventKind, EventQueue, PackedMap, Result, window::display::DisplayInner};
use win32::dxgi::IDXGIFactory;

/// Re-enumerate the displays in the system, emitting the appropriate events for changes
pub(in crate::window::subsystem::windows::message_only_wnd_proc) fn re_enumerate_displays<
    UserEvent: Send,
>(
    pump: &EventQueue<UserEvent>,
    displays: &mut PackedMap<DisplayInner>,
    dxgi_factory: &mut IDXGIFactory,
) -> Result<()> {
    let mut new_displays = DisplayInner::enumerate(dxgi_factory)?;

    // Check for display updates
    let mut to_remove = Vec::new();
    for (id, display) in displays.key_value_iter_mut() {
        // Find a display with the same ID
        let mut found_display = None;
        for i in 0..new_displays.len() {
            if new_displays[i].id() == display.id() {
                found_display = Some(new_displays.swap_remove(i));
                break;
            }
        }

        // Queue this display to be removed if no match was found
        let found_display = match found_display {
            Some(found_display) => found_display,
            None => {
                to_remove.push(id);
                continue;
            }
        };

        // Update the display if a match was found and emit appropriate events
        if display.position() != found_display.position() {
            pump.push(EventKind::DisplayMoved {
                id: unsafe { id.cast() },
                new_position: found_display.position(),
            })?;
        }
        if display.size() != found_display.size() {
            pump.push(EventKind::DisplayResized {
                id: unsafe { id.cast() },
                new_size: found_display.size(),
            })?;
        }
        if display.work_area() != found_display.work_area() {
            pump.push(EventKind::DisplayWorkAreaChanged {
                id: unsafe { id.cast() },
                new_work_area: found_display.work_area(),
            })?;
        }
        if display.refresh_rate() != found_display.refresh_rate() {
            pump.push(EventKind::DisplayRefreshRateChanged {
                id: unsafe { id.cast() },
                new_refresh_rate: found_display.refresh_rate(),
            })?;
        }
        if display.current_orientation() != found_display.current_orientation() {
            pump.push(EventKind::DisplayRotated {
                id: unsafe { id.cast() },
                new_orientation: found_display.current_orientation(),
            })?;
        }
        if display.dpi() != found_display.dpi() {
            pump.push(EventKind::DisplayDpiChanged {
                id: unsafe { id.cast() },
                new_dpi: found_display.dpi(),
            })?;
        }

        *display = found_display;
    }

    // Remove each excess display and emit events
    for id in to_remove {
        displays.remove(id);
        pump.push(EventKind::DisplayRemoved {
            id: unsafe { id.cast() },
        })?;
    }

    // Add each new display and emit events
    for new_display in new_displays {
        let id = displays.insert(new_display);
        pump.push(EventKind::DisplayAdded {
            id: unsafe { id.cast() },
        })?;
    }

    Ok(())
}
