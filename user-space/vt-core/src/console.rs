use std::fs;
use std::io::{self, BufRead, BufReader};

use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct Config {
    pub keymap: Option<String>,
    pub font: Option<String>,
    pub font_map: Option<String>,
    pub font_unimap: Option<String>,
}

pub fn read_vconsole_conf() -> Result<Config> {
    let path = "/etc/vconsole.conf";
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Config::default()),
        Err(e) => return Err(Error::Io(e)),
    };

    let reader = BufReader::new(file);
    let mut config = Config::default();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let k = key.trim();
            let v = value.trim().trim_matches('"').trim_matches('\'').to_string();
            match k {
                "KEYMAP" => config.keymap = Some(v),
                "FONT" => config.font = Some(v),
                "FONT_MAP" => config.font_map = Some(v),
                "FONT_UNIMAP" => config.font_unimap = Some(v),
                _ => {}
            }
        }
    }

    Ok(config)
}
