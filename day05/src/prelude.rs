pub type Result<T> = std::result::Result<T, &'static str>;

pub type Procedures = Box<[Procedure]>;
pub type Procedure = (u32, usize, usize);

pub type Stacks = Vec<Stack>;
pub type Stack = Vec<char>;
