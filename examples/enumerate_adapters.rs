//! Uses alexandria to enumerate the adapters in the system

pub fn main() {
    let adapters = alexandria::Adapter::enumerate().unwrap();
    if adapters.len() == 0 {
        println!("No adapters on the system!");
        return;
    }

    println!("Adapters:");
    for (i, adapter) in adapters.into_iter().enumerate() {
        println!(" {}. {}", i + 1, adapter.name());
    }
}
