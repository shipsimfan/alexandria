/// Prints out information about the graphics hardware on the system
fn main() {
    let context = alexandria::AlexandriaContext::builder().create().unwrap();

    // TODO: List graphics instance extensions

    // TODO: List graphics instance layers

    // TODO: List graphics adapters
    // TODO: List graphics adapter extensions
    // TODO: List graphics adapter queues

    drop(context);
}
