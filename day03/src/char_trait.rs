pub trait AodCharTrait {
    fn priority(&self) -> u32;
}

impl AodCharTrait for char {
    fn priority(&self) -> u32 {
        let value: u32 = *self as u32;

        match self {
            'a'..='z' => value - 96,
            'A'..='Z' => value - 38,
            _ => unreachable!(),
        }
    }
}
