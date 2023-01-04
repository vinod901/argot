#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Operator not supported!!!"),
        }
    }
}

#[cfg(test)]
mod tests {}
