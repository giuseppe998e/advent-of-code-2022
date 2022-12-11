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

    let name = words.next().ok_or("No name was provided for the node!")?;

    let mut current_nodes = current
        .as_ref()
        .and_then(|rc| rc.children_mut())
        .ok_or("The pointer to current directory is missing!")?;

    // If the "current" directory doesn't contains this Node name, add the new Node to the recommended "position"
    if let Err(position) = current_nodes.binary_search_by(|node| node.name().cmp(name)) {
        let node_kind = match number {
            // Case when the new node is a file
            Some(size) => NodeKind::new_file(size),
            // Case when the new node is a directory
            None => NodeKind::new_dir(),
        };

        let node = Node::new(name.to_string(), current.as_ref(), node_kind);
        current_nodes.insert(position, Rc::new(node));
    }

    Ok(())
}

fn parse_command_line(
    current: &mut Option<Rc<Node>>,
    words: &mut Peekable<SplitWhitespace>,
) -> Result<()> {
    let directory = {
        if words.next().filter(|&str_| str_ == "cd").is_none() {
            return Ok(()); // All commands other than "cd" simply return
        }

        words
            .next()
            .ok_or("The command has no directory name argument!")?
    };

    // Update the "current" directory pointer
    *current = match directory {
        // The "current" directory pointer needs to go up
        ".." => current
            .as_ref()
            .ok_or("Attempted to go back up from the root directory!")
            .and_then(|rc| {
                rc.parent()
                    .upgrade()
                    .ok_or("The pointer to parent directory is missing!")
            })
            .map(Some)?,
        // The "current" directory pointer is None == Root directory
        // Should occur just once
        _ if current.is_none() => {
            let root = Node::new(directory.to_string(), None, NodeKind::new_dir());
            Some(Rc::new(root))
        }
        // The "current" directory pointer is Some(_)
        _ => {
            let node_rc = {
                let mut current_nodes = current
                    .as_ref()
                    .and_then(|rc| rc.children_mut())
                    .ok_or("The pointer to current directory is missing!")?;

                let node_exists = current_nodes.binary_search_by(|node| node.name().cmp(directory));

                match node_exists {
                    Ok(index) => Rc::clone(&current_nodes[index]),
                    Err(position) => {
                        let node =
                            Node::new(directory.to_string(), current.as_ref(), NodeKind::new_dir());

                        let node_rc = Rc::new(node);
                        current_nodes.insert(position, Rc::clone(&node_rc));
                        node_rc
                    }
                }
            };

            Some(node_rc)
        }
    };

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
                // Parse command lines
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
                // Command output lines
                _ => parse_output_line(&mut current, &mut words)?,
            }
        }
    }

    root
}
