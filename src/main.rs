use budget_tracker::{Input, Config, log};
use std::{env, process};

fn main() {
    let input_data = Input::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Cannot parse arguments: {err}");
        process::exit(1);
    });

    println!("{:?}", input_data);

    let config = Config {
        file_path: "test.csv".to_string()
    };

    log(input_data, &config).unwrap();

}
