#[derive(Debug)]
pub struct Money {
    pub whole: u32,
    pub fraction: u8,
}

impl Money {
    pub fn from_f64(value: f64) -> Money {
        Money {
            whole: value.trunc() as u32,
            fraction: ((value.fract() * 100.0).round()) as u8,
        }
    }
}