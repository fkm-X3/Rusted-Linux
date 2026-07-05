use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use vt_core::console;
use vt_core::font;
use vt_core::keymap;
use vt_core::notify;
use vt_core::vt;

fn setup_console() {
    let config = match console::read_vconsole_conf() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("vt-manager: cannot read /etc/vconsole.conf: {}", e);
            return;
        }
    };

    let tty = match OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty0")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("vt-manager: cannot open /dev/tty0: {}", e);
            return;
        }
    };
    let fd = tty.as_raw_fd();

    if let Some(fp) = &config.font {
        match font::load_from_file(fp) {
            Ok(f) => {
                if let Err(e) = font::apply(fd, &f) {
                    eprintln!("vt-manager: font apply failed '{}': {}", fp, e);
                } else {
                    let _ = notify::sd_notify_status(&format!("Font loaded: {}", fp));
                }
            }
            Err(e) => {
                eprintln!("vt-manager: font load failed '{}': {}", fp, e);
            }
        }
    }

    if config.keymap.is_some() {
        if let Err(e) = keymap::set_unicode(fd) {
            eprintln!("vt-manager: keyboard mode set failed: {}", e);
        } else {
            let _ = notify::sd_notify_status("Keyboard set to unicode mode");
        }
    }
}

fn main() {
    let _ = notify::sd_notify_status("Starting Rusted-Linux VT Manager");

    match vt::current() {
        Ok(n) => eprintln!("vt-manager: active VT is {}", n),
        Err(e) => eprintln!("vt-manager: not on VT: {}", e),
    }

    setup_console();

    let _ = notify::sd_notify_ready();

    loop {
        std::thread::sleep(Duration::from_secs(30));
        let _ = notify::sd_notify_watchdog();
    }
}
