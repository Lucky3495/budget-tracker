use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Money {
    pub whole: u32,
    pub fraction: u8,
}

impl Money {
    fn from_str(s: &String) -> Result<Money, String> {
        let value: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => Err(&format!("The value {} is not a valid number.", s))?,
        };

        Ok(Money {
            whole: value.trunc() as u32,
            fraction: ((value.fract() * 100.0).round()) as u8,
        })
    }
}

#[derive(Debug)]
pub struct Input {
    pub money: Money,
    pub category: String,
    pub comment: Option<String>
}

impl Input {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Input, String> {
        // Ignore the first argument (usually the file name or something)
        args.next();

        // check that a value is given
        let money_str = match args.next() {
            Some(v) => v,
            None => Err("Expected a number as money but got nothing.")?,
        };

        let money = match Money::from_str(&money_str) {
            Ok(v) => v,
            Err(_) => Err(format!("Expected a number as money but got {}.", money_str))?
        };

        // check that a category is given
        let category = match args.next() {
            Some(v) => v,
            None => Err("Expected a string but got nothing.")?,
        };

        // check that the given category is valid
        // if !Category::str_list().contains(&category.as_str()) {
        //     Err(format!("Category {} is not a valid category.", category))?
        // }

        // capture the comment if any
        let comment = args.next();

        Ok(Input {
            money,
            category,
            comment,
        })
    }
}

pub struct Config {
    pub file_path: String,
}

struct Row {
    timestamp: DateTime<Local>,
    category: String,
    money: Money,
    comment: Option<String>,
}

impl Row {
    fn from_input(input: Input) -> Row {
        Row {
            timestamp: Local::now(),
            category: input.category,
            money: input.money,
            comment: input.comment,
        }
    }

    fn to_string(self) -> String {
        return format!(
            "{},{}.{},{},{}",
            self.timestamp.format("%Y-%m-%d"),
            self.money.whole,
            self.money.fraction,
            self.category,
            match self.comment {
                Some(v) => v,
                None => "".to_string(),
            }
        )
    }
}

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