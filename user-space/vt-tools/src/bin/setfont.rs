use std::env;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::process;
use vt_core::font;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: setfont <font.psf>");
        process::exit(1);
    }

    let f = match font::load_from_file(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("setfont: failed to load '{}': {}", args[1], e);
            process::exit(1);
        }
    };

    let tty = match OpenOptions::new().write(true).open("/dev/tty0") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("setfont: cannot open /dev/tty0: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = font::apply(tty.as_raw_fd(), &f) {
        eprintln!("setfont: failed to apply font: {}", e);
        process::exit(1);
    }
}
