#[derive(PartialEq, Clone)]
pub enum Calculation {
    Add,
    Multiply,
}

impl Calculation {
    pub fn parse(char: &str) -> Self {
        match char {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => unreachable!(),
        }
    }
}
