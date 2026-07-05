use std::env;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::process;
use vt_core::keymap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: loadkeys <keymap-file>");
        process::exit(1);
    }

    let tty = match OpenOptions::new().write(true).open("/dev/tty0") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("loadkeys: cannot open /dev/tty0: {}", e);
            process::exit(1);
        }
    };

    let fd = tty.as_raw_fd();

    let old = match keymap::current_mode(fd) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("loadkeys: cannot get keyboard mode: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = keymap::set_unicode(fd) {
        eprintln!("loadkeys: cannot set keyboard to unicode mode: {}", e);
        process::exit(1);
    }

    eprintln!("Switched keyboard from mode {} to unicode (K_UNICODE)", old);
    eprintln!("Note: full keymap table loading not yet implemented in Rust");
}
