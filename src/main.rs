#![allow(unused)]
use budget_tracker::{Args, Config, log};
use std::{env, process};
use clap::{crate_name, crate_version, value_parser, Arg, Command};

fn main() {

    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .about("Logs expenses in a CSV file for personal analysis.")
        .arg(
            Arg::new("money")
            .value_name("MONEY")
            .value_parser(value_parser!(f64))
            .help("The value of the expense as a number. Will be truncated to 2 decimal places.")
            .required(true)
        )
        // TODO: Make the possible categories a config for the app, validate
        // them, and display them in the --help output.
        // For now the user is free to sepcify any one they want
        .arg(
            Arg::new("category")
            .value_name("CATEGORY")
            .help("The category of the expense.")
            .required(true)
        )
        .arg(
            Arg::new("comment")
            .value_name("COMMENT")
            .help("Comment that describes the expense.")
        ).get_matches();

        let args = Args {
            money: *matches.get_one("money").expect("Money is required"),
            category: matches.get_one::<String>("category").expect("Category is required").clone(),
            comment: match matches.get_one::<String>("comment") {
                Some(v) => Some(v.clone()),
                None => None,
            }
        };

        let config = Config { file_path: "test.csv".to_string(), };

        log(args, &config).unwrap()

}
