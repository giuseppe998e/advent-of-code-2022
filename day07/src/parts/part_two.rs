use crate::prelude::*;

const SPACE_AVAILABLE: usize = 70_000_000;
const SPACE_REQUIRED_TOTAL: usize = 30_000_000;

fn find_best_match(node: &Rc<Node>, needed_space: usize) -> Result<usize> {
    let mut best_match = SPACE_AVAILABLE;

    let node_size = node.size();
    if node_size >= needed_space {
        best_match = node_size;
    }

    let childs = node
        .children()
        .ok_or("The given Node is not a directory!")?;

    for subnode in childs.iter().filter(|node| node.is_dir()) {
        let subnode_best_match = find_best_match(subnode, needed_space)?;
        best_match = best_match.min(subnode_best_match)
    }

    Ok(best_match)
}

pub fn part_two(root: &Rc<Node>) -> Result<usize> {
    let free_space = SPACE_AVAILABLE - root.size();
    let needed_space = SPACE_REQUIRED_TOTAL - free_space;
    find_best_match(root, needed_space)
}
