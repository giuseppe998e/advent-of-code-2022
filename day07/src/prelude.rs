pub use crate::tree::{Node, NodeKind};
pub use std::rc::Rc;

pub type Result<T> = std::result::Result<T, &'static str>;
