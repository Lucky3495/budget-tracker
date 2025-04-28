use chrono::{DateTime, Local};
use crate::{Money, Args};

pub struct Row {
    timestamp: DateTime<Local>,
    category: String,
    money: Money,
    comment: Option<String>,
}

impl Row {
    pub fn from_input(input: Args) -> Row {
        Row {
            timestamp: Local::now(),
            category: input.category,
            money: Money::from_f64(input.money),
            comment: input.comment,
        }
    }

    pub fn to_string(self) -> String {
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