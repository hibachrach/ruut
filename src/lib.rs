mod lisplike;

use std::vec::Vec;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new<'a>(name: String) -> Node {
        Node {
            name: name,
            children: vec![],
        }
    }
}

pub enum InputFormat {
    LispLike,
}

impl FromStr for InputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lisp" => Ok(InputFormat::LispLike),
            _ => Err("invalid format type"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyInputError,
    MissingNameError,
    MultipleRootsError,
}

pub fn prettify(serialized: String, format: InputFormat) -> Result<String, Error> {
    let root = match format {
        InputFormat::LispLike => lisplike::deserialize(serialized)
    }?;
    Ok(node_to_lines(&root).join("\n"))
}

pub fn node_to_lines(node: &Node) -> Vec<String> {
    let mut lines = vec![node.name.clone()];
    let children = &node.children;
    let last_child_idx = if children.len() > 1 {
        children.len() - 1
    } else {
        0
    };
    for child in children[0..last_child_idx].iter() {
        let child_node_lines = node_to_lines(child);
        if let Some(first_line) = child_node_lines.first() {
            lines.push(format!("├── {}", first_line));
        }
        for line in child_node_lines[1..].iter() {
            lines.push(format!("│   {}", line));
        }
    }
    if let Some(last_child) = children.last() {
        let last_child_node_lines = node_to_lines(last_child);
        if let Some(first_line) = last_child_node_lines.first() {
            lines.push(format!("└── {}", first_line));
        }
        for line in last_child_node_lines[1..].iter() {
            lines.push(format!("    {}", line));
        }
    }
    lines
}
