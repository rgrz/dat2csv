use std::ffi::OsString;
use std::error::Error;
use std::env;

// First argument should be the dat file to parse
pub fn get_dat_filename() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}