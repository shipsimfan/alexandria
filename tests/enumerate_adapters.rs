#[test]
pub fn enumerate_adapters() {
    let adapters = alexandria::Adapter::enumerate().unwrap();
    for adapter in adapters {
        println!("Adapter: {}", adapter.name());
    }
}
