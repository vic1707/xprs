#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Operator {
    // Factorial,
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    Modulo,
}

impl TryFrom<u8> for Operator {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // b'!' => Ok(Self::Factorial),
            b'+' => Ok(Self::Plus),
            b'-' => Ok(Self::Minus),
            b'*' => Ok(Self::Times),
            b'/' => Ok(Self::Divide),
            b'^' => Ok(Self::Power),
            b'%' => Ok(Self::Modulo),
            _ => Err("Operator not found"),
        }
    }
}
