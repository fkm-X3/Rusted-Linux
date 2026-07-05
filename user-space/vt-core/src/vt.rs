use std::fs::OpenOptions;
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

use crate::error::{Error, Result};

const VT_ACTIVATE: libc::c_ulong = 0x5606;
const VT_WAITACTIVE: libc::c_ulong = 0x5607;
const VT_OPENQRY: libc::c_ulong = 0x5600;
const VT_DISALLOCATE: libc::c_ulong = 0x5608;
const VT_GETSTATE: libc::c_ulong = 0x5603;

const VT_PROCESS: libc::c_int = 0x0002;
const VT_ACKACQ: libc::c_int = 0x0004;

#[repr(C)]
struct VtStat {
    v_active: u16,
    v_signal: u16,
    v_state: u16,
    _pad: u16,
}

pub fn current() -> Result<u32> {
    let fd = console_fd()?;
    let mut stat = VtStat { v_active: 0, v_signal: 0, v_state: 0, _pad: 0 };
    let ret = unsafe { libc::ioctl(fd, VT_GETSTATE, &mut stat as *mut _ as *mut libc::c_void) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(stat.v_active as u32)
}

pub fn activate(num: u32) -> Result<()> {
    let fd = console_fd()?;
    let ret = unsafe { libc::ioctl(fd, VT_ACTIVATE, num as libc::c_int) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(())
}

pub fn wait_active(num: u32) -> Result<()> {
    let fd = console_fd()?;
    let ret = unsafe { libc::ioctl(fd, VT_WAITACTIVE, num as libc::c_int) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(())
}

pub fn open_free() -> Result<u32> {
    let fd = console_fd()?;
    let mut num: libc::c_int = 0;
    let ret = unsafe { libc::ioctl(fd, VT_OPENQRY, &mut num as *mut _ as *mut libc::c_void) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    if num < 0 {
        return Err(Error::VtInUse);
    }
    Ok(num as u32)
}

pub fn deallocate(num: u32) -> Result<()> {
    let fd = console_fd()?;
    let ret = unsafe { libc::ioctl(fd, VT_DISALLOCATE, num as libc::c_int) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(())
}

pub fn set_process_mode() -> Result<()> {
    use crate::error::Error;
    let fd = console_fd()?;
    let mode: libc::c_int = VT_PROCESS | VT_ACKACQ;
    let ret = unsafe { libc::ioctl(fd, 0x5602, mode) };
    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(())
}

pub fn console_fd() -> Result<RawFd> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty0")
        .or_else(|_| OpenOptions::new().read(true).write(true).open("/dev/console"))
        .map_err(|e| Error::Io(e))?;
    Ok(file.as_raw_fd())
}
