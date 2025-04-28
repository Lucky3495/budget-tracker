use crate::Money;

#[derive(Debug)]
pub struct Args {
    pub money: f64,
    pub category: String,
    pub comment: Option<String>
}

impl Args {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Args, String> {
        // Ignore the first argument (usually the file name or something)
        args.next();

        // check that a value is given
        let money = match args.next() {
            Some(v) => v.parse().unwrap(),
            None => Err("Expected a number as money but got nothing.")?,
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

        Ok(Args {
            money,
            category,
            comment,
        })
    }
}