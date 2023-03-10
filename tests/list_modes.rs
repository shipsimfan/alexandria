#[test]
fn list_modes() -> Result<(), alexandria::Error> {
    let mut instance = alexandria::Instance::new(false)?;

    println!("Adapters & Displays:");
    for adapter in instance.enum_adapters()? {
        let mut adapter = adapter?;
        println!("  - {}", adapter.name());

        for display in adapter.enum_displays()? {
            let display = display?;
            println!("      - {}", display.name());

            for available_resolution in display.available_resolutions() {
                print!("          - {} (", available_resolution.resolution());

                let refresh_rates = available_resolution.refresh_rates();
                for i in 0..refresh_rates.len() {
                    print!("{}", refresh_rates[i]);

                    if i != refresh_rates.len() - 1 {
                        print!(", ");
                    }
                }
                println!(")");
            }
        }
    }

    let mut default_adapter = instance.default_adapter()?;
    let default_display = default_adapter.default_display()?;

    println!(
        "Default: {} - {} ({} @ {})",
        default_adapter.name(),
        default_display.name(),
        default_display.available_resolutions()[0].resolution(),
        default_display.available_resolutions()[0].refresh_rates()[0],
    );

    Ok(())
}
