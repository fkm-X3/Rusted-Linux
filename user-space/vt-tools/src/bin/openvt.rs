use std::env;
use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::process;
use vt_core::vt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: openvt [-c <VT>] <command> [args...]");
        eprintln!("       openvt <command> [args...]");
        process::exit(1);
    }

    let (vt_num, cmd_start) = if args[1] == "-c" && args.len() > 2 {
        match args[2].parse::<u32>() {
            Ok(n) => (n, 3),
            Err(_) => {
                eprintln!("openvt: invalid VT number: {}", args[2]);
                process::exit(1);
            }
        }
    } else {
        (0, 1)
    };

    let vt_num = if vt_num == 0 {
        match vt::open_free() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("openvt: no free VT available: {}", e);
                process::exit(1);
            }
        }
    } else {
        vt_num
    };

    let tty_path = format!("/dev/tty{}", vt_num);

    match unsafe { libc::fork() } {
        -1 => {
            eprintln!("openvt: fork failed");
            process::exit(1);
        }
        0 => {
            if let Ok(file) = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&tty_path)
            {
                let fd = file.as_raw_fd();
                unsafe {
                    libc::ioctl(fd, 0x5409, 0);
                    libc::dup2(fd, 0);
                    libc::dup2(fd, 1);
                    libc::dup2(fd, 2);
                    if fd > 2 { libc::close(fd); }
                }
            }

            let _ = vt::activate(vt_num);
            let _ = vt::wait_active(vt_num);

            let cmd = CString::new(args[cmd_start].as_bytes()).unwrap();
            let mut cargs: Vec<CString> = args[cmd_start..]
                .iter()
                .map(|a| CString::new(a.as_bytes()).unwrap())
                .collect();
            cargs.push(CString::new("").unwrap());

            unsafe {
                libc::execvp(cmd.as_ptr(), cargs.as_ptr() as *const *const libc::c_char);
            }

            eprintln!("openvt: exec failed: {}", args[cmd_start]);
            unsafe { libc::_exit(1); }
        }
        pid => {
            let mut status: i32 = 0;
            unsafe { libc::waitpid(pid, &mut status, 0); }
            let code = libc::WEXITSTATUS(status);
            if code != 0 {
                process::exit(code);
            }
        }
    }
}
