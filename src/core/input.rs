use crate::Money;

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