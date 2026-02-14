fn main() {
    let (context, _) = alexandria::AlexandriaContext::<()>::builder()
        .window()
        .create()
        .unwrap();

    println!("     --- Display Information ---");
    for (id, display) in context.window().displays() {
        println!("{}: {}", id, display.name());
        println!("  - ID: {}", display.id());
        println!("  - Position: {}", display.position());
        println!("  - Size: {}x{}", display.width(), display.height());
        println!("  - Work Area Position: {}", display.work_area_position());
        println!(
            "  - Work Area Size: {}x{}",
            display.work_area_width(),
            display.work_area_height()
        );
        println!(
            "  - DPI: {} ({}%)",
            display.dpi(),
            (display.dpi() * 100) / 96
        );
        println!(
            "  - Is Primary? {}",
            if display.is_primary() { "Yes" } else { "No" }
        );
    }
}
