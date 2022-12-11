use super::NodeKind;
use std::{
    cell::{Ref, RefMut},
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub struct Node {
    name: String,
    parent: Weak<Node>,
    kind: NodeKind,
}

impl Node {
    pub fn new(name: String, parent: Option<&Rc<Node>>, kind: NodeKind) -> Self {
        let parent = parent.map(Rc::downgrade).unwrap_or_default();
        Self { name, parent, kind }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    //pub fn set_parent(&mut self, parent: &Rc<Node>) {
    //    self.parent = Rc::downgrade(parent);
    //}

    pub fn parent(&self) -> Weak<Node> {
        self.parent.clone()
    }

    //pub fn kind(&self) -> &NodeKind {
    //    &self.kind
    //}

    //pub fn is_file(&self) -> bool {
    //    matches!(self.kind, NodeKind::File(_))
    //}

    pub fn is_dir(&self) -> bool {
        matches!(self.kind, NodeKind::Dir(_))
    }

    pub fn size(&self) -> usize {
        match self.kind {
            NodeKind::File(size) => size,
            NodeKind::Dir(ref list) => list.borrow().iter().map(|node| node.size()).sum::<usize>(),
        }
    }

    pub fn children(&self) -> Option<Ref<Vec<Rc<Node>>>> {
        match self.kind {
            NodeKind::Dir(ref vec) => Some(vec.borrow()),
            NodeKind::File(_) => None,
        }
    }

    pub fn children_mut(&self) -> Option<RefMut<Vec<Rc<Node>>>> {
        match self.kind {
            NodeKind::Dir(ref vec) => Some(vec.borrow_mut()),
            NodeKind::File(_) => None,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            //.field("parent", &self.parent.as_ref().map(|node| node.name.clone()))
            .field("type", &self.kind)
            .finish()
    }
}
