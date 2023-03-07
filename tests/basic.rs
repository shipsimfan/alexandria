#[test]
fn basic() -> Result<(), alexandria::Error> {
    let mut instance = alexandria::Instance::new()?;

    println!("Adapters & Displays:");
    for adapter in instance.enum_adapters()? {
        let mut adapter = adapter?;
        println!("  - {}", adapter.name());

        for display in adapter.enum_displays()? {
            let display = display?;
            println!("      - {}", display.name());
        }
    }

    let mut default_adapter = instance.default_adapter()?;
    let default_display = default_adapter.default_display()?;

    println!(
        "Default: {} - {}",
        default_adapter.name(),
        default_display.name()
    );

    Ok(())
}
