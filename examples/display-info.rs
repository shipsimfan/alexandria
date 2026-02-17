fn main() {
    let (context, _) = alexandria::AlexandriaContext::<()>::builder()
        .window()
        .create()
        .unwrap();

    println!("     --- Display Information ---");
    for (id, display) in context.window().displays() {
        println!("{}: {}", id, display.name());
        println!("  ID: {}", display.id());
        println!("  Position: {}", display.position());
        println!("  Size: {}x{}", display.width(), display.height());
        println!("  Work Area Position: {}", display.work_area_position());
        println!(
            "  Work Area Size: {}x{}",
            display.work_area_width(),
            display.work_area_height()
        );
        println!("  Refresh Rate: {:.02}Hz", display.refresh_rate().as_f32());
        println!(
            "  DPI: {} ({}%)",
            display.dpi(),
            display.content_scale() * 100.0
        );
        if let Some(physical_size) = display.physical_size() {
            println!(
                "  Physical Size: {}mm x {}mm ({:.1}\")",
                physical_size.x,
                physical_size.y,
                display.physical_diagonal_inches().unwrap()
            );
        }
        println!("  Orientation: {:?}", display.current_orientation());
        println!(
            "  Is Primary? {}",
            if display.is_primary() { "Yes" } else { "No" }
        );
        println!("  Display Modes:");
        for mode in display.modes() {
            println!(
                "    - {}x{} @ {:.02}Hz",
                mode.width(),
                mode.height(),
                mode.refresh_rate.as_f32()
            );
        }
    }
}
