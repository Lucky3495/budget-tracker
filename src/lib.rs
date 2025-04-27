pub mod core;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub use core::money::Money;
pub use core::row::Row;
pub use core::input::Input;
pub use core::config::Config;

const HEADER: &str = "DATE,MONEY,CATEGORY,COMMENT";

pub fn log(input: Input, config: &Config) -> Result<(), std::io::Error> {
    let path = &config.file_path;

    let row = Row::from_input(input);

    let file_exists = Path::new(path).exists();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    if !file_exists {
        writeln!(file, "{}", HEADER)?;
    }

    writeln!(file, "{}",row.to_string())?;
    Ok(())
}