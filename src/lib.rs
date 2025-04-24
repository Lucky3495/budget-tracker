#[derive(Debug)]
pub struct Input {
    pub money: Money,
    pub category: String,
    pub comment: Option<String>
}

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

// enum Category {
//     Food,
//     Gas,
//     HomeSupplies,
//     Games,
//     Utilities,
//     Health,
//     Donations,
//     Misc
// }

// impl Category {
//     fn str_list() -> [&'static str; 8] {
//         [
//             "food",
//             "gas",
//             "homeSupplies",
//             "games",
//             "utilities",
//             "health",
//             "donations",
//             "misc",
//         ]
//     }

//     fn to_str(&self) -> &str {
//         match self {
//             Category::Food => "food",
//             Category::Gas => "gas",
//             Category::HomeSupplies => "homeSupplies",
//             Category::Games => "games",
//             Category::Utilities => "utilities",
//             Category::Health => "health",
//             Category::Donations => "donations",
//             Category::Misc => "misc",
//         }
//     }
// }