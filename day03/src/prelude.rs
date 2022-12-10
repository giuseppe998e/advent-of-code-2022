pub use crate::char_trait::AodCharTrait;

pub type Result<T> = std::result::Result<T, &'static str>;
pub type Rucksack<'a> = (&'a str, &'a str);
