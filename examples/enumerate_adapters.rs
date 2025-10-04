//! Uses alexandria to enumerate the adapters in the system

fn main() {
    let adapters = alexandria::Adapter::enumerate().unwrap();
    if adapters.len() == 0 {
        println!("No adapters on the system!");
        return;
    }

    println!("Adapters:");
    for (i, adapter) in adapters.into_iter().enumerate() {
        println!(
            " {}. {} ({})",
            i + 1,
            adapter.name(),
            ByteSize(adapter.video_memory())
        );

        for output in adapter.outputs() {
            println!("   - {} (@ {})", output.name(), output.position());
            for resolution in output.resolutions() {
                print!("     - {}x{} (@ ", resolution.width(), resolution.height());
                let mut first = true;
                for refresh_rate in resolution.refresh_rates() {
                    if first {
                        first = false;
                    } else {
                        print!(", ");
                    }
                    print!("{:.02}Hz", refresh_rate);
                }
                println!(")");
            }
        }
    }
}

struct ByteSize(u64);

const KILO: u64 = 1024;
const MEGA: u64 = KILO * 1024;

impl std::fmt::Display for ByteSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < KILO {
            write!(f, "{}b", self.0)
        } else if self.0 < MEGA {
            let kilo = self.0 / KILO;
            write!(f, "{kilo}")?;

            let frac = ((self.0 * 1000) / KILO) % 1000;
            if frac != 0 {
                write!(f, ".{frac:03}")?;
            }

            write!(f, "Kb")
        } else {
            let mega = self.0 / MEGA;
            write!(f, "{mega}")?;

            let frac = ((self.0 * 1000) / MEGA) % 1000;
            if frac != 0 {
                write!(f, ".{frac:03}")?;
            }

            write!(f, "Mb")
        }
    }
}
