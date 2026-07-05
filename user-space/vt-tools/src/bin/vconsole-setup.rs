use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::process;
use vt_core::console;
use vt_core::font;
use vt_core::keymap;

fn main() {
    let config = match console::read_vconsole_conf() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("vconsole-setup: cannot read /etc/vconsole.conf: {}", e);
            process::exit(1);
        }
    };

    let tty = match OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty0")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("vconsole-setup: cannot open /dev/tty0: {}", e);
            process::exit(1);
        }
    };
    let fd = tty.as_raw_fd();

    if let Some(fp) = &config.font {
        match font::load_from_file(fp) {
            Ok(f) => {
                if let Err(e) = font::apply(fd, &f) {
                    eprintln!("vconsole-setup: font apply failed '{}': {}", fp, e);
                }
            }
            Err(e) => {
                eprintln!("vconsole-setup: font load failed '{}': {}", fp, e);
            }
        }
    }

    if config.keymap.is_some() {
        if let Err(e) = keymap::set_unicode(fd) {
            eprintln!("vconsole-setup: keyboard mode set failed: {}", e);
        }
    }
}
