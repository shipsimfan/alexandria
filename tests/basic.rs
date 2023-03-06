#[test]
fn basic() -> Result<(), alexandria::Error> {
    let mut instance = alexandria::Instance::new()?;

    println!("Adapters:");
    for adapter in instance.enum_adapters()? {
        let adapter = adapter?;
        println!("  - {}", adapter.name());
    }

    println!("Default: {}", instance.default_adapter()?.name());

    Ok(())
}
