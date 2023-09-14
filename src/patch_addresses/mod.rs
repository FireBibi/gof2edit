use std::io;
use std::io::Write;

pub mod binary_version;
pub mod station;
pub mod system;
mod write_value;

pub type WriteValueFn = dyn Fn(&mut dyn Write, u8) -> io::Result<()>;
