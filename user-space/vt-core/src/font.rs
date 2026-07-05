use std::fs::File;
use std::io::Read;
use std::mem;
use std::os::unix::io::RawFd;
use std::io;

use crate::error::{Error, Result};

const PSF1_MAGIC: [u8; 2] = [0x36, 0x04];

const PSF2_MAGIC: [u8; 4] = [0x72, 0xb5, 0x4a, 0x86];

#[repr(C)]
struct Psf1Header {
    magic: [u8; 2],
    mode: u8,
    charsize: u8,
}

#[repr(C)]
struct Psf2Header {
    magic: [u8; 4],
    version: u32,
    headersize: u32,
    flags: u32,
    length: u32,
    charsize: u32,
    height: u32,
    width: u32,
}

#[derive(Clone)]
pub struct Font {
    pub width: u32,
    pub height: u32,
    pub charcount: u32,
    pub data: Vec<u8>,
    pub bytes_per_glyph: usize,
}

pub fn load_from_file(path: &str) -> Result<Font> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    parse(&data)
}

pub fn parse(data: &[u8]) -> Result<Font> {
    if data.len() < 2 {
        return Err(Error::FontParse("file too short"));
    }

    if data[0..2] == PSF1_MAGIC {
        parse_psf1(data)
    } else if data.len() >= 4 && data[0..4] == PSF2_MAGIC {
        parse_psf2(data)
    } else {
        Err(Error::FontParse("unknown font format (not PSF1 or PSF2)"))
    }
}

fn parse_psf1(data: &[u8]) -> Result<Font> {
    if data.len() < mem::size_of::<Psf1Header>() {
        return Err(Error::FontParse("PSF1 file too short"));
    }

    let header = unsafe { &*(data.as_ptr() as *const Psf1Header) };
    let charsize = header.charsize as u32;
    let charcount: u32 = 256;
    let width: u32 = 8;
    let height = charsize;
    let bpg = charsize as usize;

    let glyph_bytes = (charcount as usize) * bpg;
    if data.len() < mem::size_of::<Psf1Header>() + glyph_bytes {
        return Err(Error::FontParse("PSF1 glyph data truncated"));
    }

    let glyph_start = mem::size_of::<Psf1Header>();
    let glyph_data = data[glyph_start..glyph_start + glyph_bytes].to_vec();

    Ok(Font { width, height, charcount, data: glyph_data, bytes_per_glyph: bpg })
}

fn parse_psf2(data: &[u8]) -> Result<Font> {
    if data.len() < mem::size_of::<Psf2Header>() {
        return Err(Error::FontParse("PSF2 file too short"));
    }

    let header = unsafe { &*(data.as_ptr() as *const Psf2Header) };

    if header.version != 0 {
        return Err(Error::FontParse("PSF2 unsupported version"));
    }

    let width = header.width;
    let height = header.height;
    let charcount = header.length;
    let bpg = header.charsize as usize;
    let headersize = header.headersize as usize;

    let glyph_bytes = charcount as usize * bpg;
    if data.len() < headersize + glyph_bytes {
        return Err(Error::FontParse("PSF2 glyph data truncated"));
    }

    let glyph_data = data[headersize..headersize + glyph_bytes].to_vec();

    Ok(Font { width, height, charcount, data: glyph_data, bytes_per_glyph: bpg })
}

#[repr(C)]
struct ConsoleFontOp {
    op: u32,
    flags: u32,
    width: u32,
    height: u32,
    charcount: u32,
    data: *mut u8,
}

fn kd_font_op_req() -> libc::c_ulong {
    const DIR: u32 = 1 | 2;
    const TYP: u32 = 0x4B;
    const NR: u32 = 0x71;
    const SZ: u32 = mem::size_of::<ConsoleFontOp>() as u32;
    ((DIR as libc::c_ulong) << 30)
        | ((TYP as libc::c_ulong) << 8)
        | (NR as libc::c_ulong)
        | ((SZ as libc::c_ulong) << 16)
}

pub fn apply(fd: RawFd, font: &Font) -> Result<()> {
    let op = ConsoleFontOp {
        op: 0,
        flags: 0,
        width: font.width,
        height: font.height,
        charcount: font.charcount,
        data: font.data.as_ptr() as *mut u8,
    };

    let ret = unsafe {
        libc::ioctl(fd, kd_font_op_req(), &op as *const _ as *const libc::c_void)
    };

    if ret < 0 {
        return Err(Error::Io(io::Error::last_os_error()));
    }
    Ok(())
}
