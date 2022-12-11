use super::Node;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum NodeKind {
    File(usize),
    Dir(RefCell<Vec<Rc<Node>>>),
}

impl NodeKind {
    pub fn new_file(size: usize) -> Self {
        Self::File(size)
    }

    pub fn new_dir() -> Self {
        Self::Dir(RefCell::default())
    }
}

impl PartialEq for NodeKind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::File(_), Self::File(_)) | (Self::Dir(_), Self::Dir(_))
        )
    }
}

impl Eq for NodeKind {}
