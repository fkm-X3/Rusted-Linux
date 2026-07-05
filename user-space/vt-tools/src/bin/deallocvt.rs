use std::process;
use vt_core::vt;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        for arg in &args[1..] {
            let num: u32 = match arg.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("deallocvt: invalid VT number: {}", arg);
                    process::exit(1);
                }
            };
            if let Err(e) = vt::deallocate(num) {
                eprintln!("deallocvt: cannot deallocate VT {}: {}", num, e);
            }
        }
    } else {
        let current = vt::current().unwrap_or(1);
        for n in 2..=63u32 {
            if n != current {
                let _ = vt::deallocate(n);
            }
        }
    }
}
