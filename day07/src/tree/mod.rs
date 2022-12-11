use crate::prelude::*;
use std::{iter::Peekable, str::SplitWhitespace};

mod node;
pub use node::Node;

mod nodekind;
pub use nodekind::NodeKind;

fn parse_output_line(
    current: &mut Option<Rc<Node>>,
    words: &mut Peekable<SplitWhitespace>,
) -> Result<()> {
    let number = {
        let Some(word) = words.next() else {
            return Err("A command output was expected!")
        };

        word.parse::<usize>().ok()
    };

    let name = words
        .next()
        .map(str::to_string)
        .ok_or("No name was provided for the node!")?;

    let mut parent_childs = current
        .as_ref()
        .and_then(|rc| rc.children_mut())
        .ok_or("Pointer to current directory missing!")?;

    if let Err(position) = parent_childs.binary_search_by(|node| node.name().cmp(&name)) {
        let node_kind = match number {
            // It's file
            Some(size) => NodeKind::new_file(size),
            // It's directory
            None => NodeKind::new_dir(),
        };

        let node = Node::new(name, current.as_ref(), node_kind);
        parent_childs.insert(position, Rc::new(node));
    }

    Ok(())
}

fn parse_command_line(
    current: &mut Option<Rc<Node>>,
    words: &mut Peekable<SplitWhitespace>,
) -> Result<()> {
    if words.next().filter(|&str_| str_ == "cd").is_some() {
        let directory = words.next().ok_or("Directory name missing!")?;

        if current.is_none() {
            if directory == ".." {
                return Err("Attempted to go back up from the root directory!");
            }

            let root = Node::new(directory.to_string(), None, NodeKind::new_dir());
            *current = Some(Rc::new(root));
            return Ok(());
        }

        match directory {
            ".." => {
                *current = current
                    .as_ref()
                    .map(|rc| rc.parent().upgrade())
                    .ok_or("Pointer to current directory missing!")?
            }
            _ => {
                let node_rc = {
                    let mut parent_childs = current
                        .as_ref()
                        .and_then(|rc| rc.children_mut())
                        .ok_or("Pointer to current directory missing!")?;

                    let node_exists =
                        parent_childs.binary_search_by(|node| node.name().cmp(directory));

                    match node_exists {
                        Ok(index) => Rc::clone(&parent_childs[index]),
                        Err(position) => {
                            let node = Node::new(
                                directory.to_string(),
                                current.as_ref(),
                                NodeKind::new_dir(),
                            );
                            let node_rc = Rc::new(node);
                            parent_childs.insert(position, Rc::clone(&node_rc));
                            node_rc
                        }
                    }
                };

                *current = Some(node_rc);
            }
        }
    }

    Ok(())
}

pub fn parse_tree(input: &str) -> Result<Rc<Node>> {
    // Both of these should (and must) always be directories
    let mut root: Result<Rc<Node>> = Err("The given input is malformed!");
    let mut current: Option<Rc<Node>> = None;

    for line in input.lines() {
        let mut words = line.split_whitespace().peekable();

        while let Some(&word) = words.peek() {
            match word {
                "$" => {
                    words.next();
                    parse_command_line(&mut current, &mut words)?;

                    if root.is_err() {
                        root = current
                            .as_ref()
                            .map(Rc::clone)
                            .ok_or("The current position is None!");
                    }
                }
                _ => parse_output_line(&mut current, &mut words)?,
            }
        }
    }

    root
}
