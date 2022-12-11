use crate::prelude::*;

const MAX_SIZE: usize = 100_000;

pub fn part_one(node: &Rc<Node>) -> Result<usize> {
    let mut sum = 0usize;

    let node_size = node.size();
    if node_size <= MAX_SIZE {
        sum += node_size;
    }

    //let childs = node
    //    .children()
    //    .ok_or("The given Node is not a directory!")?;
    //
    //for subnode in childs.iter().filter(|node| node.is_dir()) {
    //    sum += part_one(subnode)?;
    //}

    sum += node
        .children()
        .ok_or("The given Node is not a directory!")?
        .iter()
        .filter(|node| node.is_dir())
        .map(part_one)
        .sum::<Result<usize>>()?;

    Ok(sum)
}

/* Using a "loop" ::

pub fn part_one(node: Rc<Node>) -> Result<usize> {
    let mut sum = 0usize;

    let mut nodes = vec![node];
    let mut index = 0usize;

    loop {
        let node = {
            let Some(node) = nodes.get(index) else {
                break;
            };

            index += 1;
            Rc::clone(node)
        };

        let node_size = node.size();
        if node_size <= MAX_SIZE {
            sum += node_size;
        }

        let childs = node
            .children()
            .ok_or("The given Node is not a directory!")?;

        for child in childs.iter().filter(|node| node.is_dir()) {
            nodes.push(Rc::clone(child));
        }
    }

    Ok(sum)
}

*/
