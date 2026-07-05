use std::os::unix::io::RawFd;
use std::io;

use crate::error::{Error, Result};

const KDSKBMODE: libc::c_ulong = 0x4B45;
const KDGKBMODE: libc::c_ulong = 0x4B44;

const K_UNICODE: libc::c_int = 0x03;

pub fn set_mode(fd: RawFd, mode: libc::c_int) -> Result<libc::c_int> {
    let old = unsafe { libc::ioctl(fd, KDGKBMODE) };
    if old < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }

    let ret = unsafe { libc::ioctl(fd, KDSKBMODE, mode) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }

    Ok(old)
}

pub fn set_unicode(fd: RawFd) -> Result<()> {
    set_mode(fd, K_UNICODE)?;
    Ok(())
}

pub fn current_mode(fd: RawFd) -> Result<libc::c_int> {
    let mode = unsafe { libc::ioctl(fd, KDGKBMODE) };
    if mode < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(mode)
}
