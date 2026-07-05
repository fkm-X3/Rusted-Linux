use std::env;
use std::os::unix::net::UnixDatagram;

use crate::error::{Error, Result};

pub fn sd_notify(state: &str) -> Result<()> {
    let sock = match env::var("NOTIFY_SOCKET") {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };

    let path = if sock.starts_with('@') {
        format!("\0{}", &sock[1..])
    } else {
        sock
    };

    let socket = match UnixDatagram::unbound() {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };

    socket.send_to(state.as_bytes(), &path).map_err(Error::Io)?;
    Ok(())
}

pub fn sd_notify_ready() -> Result<()> {
    sd_notify("READY=1\nSTATUS=Rusted-Linux VT Manager ready\nMAINPID=1")
}

pub fn sd_notify_status(status: &str) -> Result<()> {
    sd_notify(&format!("STATUS={}\n", status))
}

pub fn sd_notify_watchdog() -> Result<()> {
    sd_notify("WATCHDOG=1")
}

pub fn sd_notify_stopping() -> Result<()> {
    sd_notify("STOPPING=1\nSTATUS=Shutting down")
}

pub fn sd_listen_fds(unset: bool) -> Result<u32> {
    let pid = match env::var("LISTEN_PID") {
        Ok(v) => v.parse::<u32>().map_err(|_| Error::SystemdSocket)?,
        Err(_) => return Ok(0),
    };

    let n = match env::var("LISTEN_FDS") {
        Ok(v) => v.parse::<u32>().map_err(|_| Error::SystemdSocket)?,
        Err(_) => return Ok(0),
    };

    if pid != std::process::id() {
        return Ok(0);
    }

    if unset {
        env::remove_var("LISTEN_PID");
        env::remove_var("LISTEN_FDS");
        env::remove_var("LISTEN_FDNAMES");
    }

    Ok(n)
}
