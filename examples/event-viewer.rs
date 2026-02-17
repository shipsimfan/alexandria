use std::time::Instant;

fn handle_event(event: &alexandria::Event<()>, start: Instant) -> bool {
    let time = event.time - start;
    print!("{:10.03} ", time.as_secs_f64());

    match event.kind {
        alexandria::EventKind::Quit => {
            println!("[QUIT]");
            return true;
        }
        alexandria::EventKind::User(_) => println!("[USER]"),
    }

    false
}

fn main() {
    let start = Instant::now();
    let (context, mut pump) = alexandria::AlexandriaContext::<()>::builder()
        .window()
        .create()
        .unwrap();

    loop {
        let event = pump.wait().unwrap();
        if handle_event(&event, start) {
            break;
        }

        while let Some(event) = pump.poll().unwrap() {
            if handle_event(&event, start) {
                break;
            }
        }
    }

    drop(context);
}
