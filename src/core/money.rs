#[derive(Debug)]
pub struct Money {
    pub whole: u32,
    pub fraction: u8,
}

impl Money {
    pub fn from_str(s: &String) -> Result<Money, String> {
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