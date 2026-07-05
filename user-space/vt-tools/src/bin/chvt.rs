use std::process;
use vt_core::vt;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: chvt <VT number>");
        process::exit(1);
    }

    let num: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("chvt: invalid VT number: {}", args[1]);
            process::exit(1);
        }
    };

    if let Err(e) = vt::activate(num) {
        eprintln!("chvt: cannot switch to VT {}: {}", num, e);
        process::exit(1);
    }
}
