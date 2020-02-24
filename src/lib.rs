mod json;
mod json_properties;
mod lisplike;

use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            children: Vec::new(),
        }
    }
}

pub enum InputFormat {
    LispLike,
    Json,
    JsonProperties,
}

impl FromStr for InputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lisp" => Ok(InputFormat::LispLike),
            "json" => Ok(InputFormat::Json),
            "jsonprop" => Ok(InputFormat::JsonProperties),
            _ => Err("invalid format type"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyInputError,
    MissingNameError,
    MultipleRootsError,
    FormatSpecificError(String),
}

pub fn prettify(
    serialized: String,
    format: InputFormat,
    name_key: String,
    children_key: String,
) -> Result<String, Error> {
    let root = match format {
        InputFormat::LispLike => lisplike::deserialize(serialized),
        InputFormat::Json => json::deserialize(serialized),
        InputFormat::JsonProperties => {
            json_properties::deserialize(serialized, name_key, children_key)
        }
    }?;
    Ok(node_to_lines(&root).join("\n"))
}

pub fn node_to_lines(node: &Node) -> Vec<String> {
    let mut lines = vec![node.name.clone()];
    let children = &node.children[..];
    if let Some((last_child, non_last_children)) = children.split_last() {
        let child_node_lines = non_last_children.iter().flat_map(|child| {
            node_to_lines(child)
                .iter()
                .enumerate()
                .map(|(idx, child_line)| {
                    if idx == 0 {
                        format!("├── {}", child_line)
                    } else {
                        format!("│   {}", child_line)
                    }
                })
                .collect::<Vec<String>>()
        });
        let last_child_node_lines = node_to_lines(last_child);
        let formatted_last_child_node_lines_iter =
            last_child_node_lines
                .iter()
                .enumerate()
                .map(|(idx, child_line)| {
                    if idx == 0 {
                        format!("└── {}", child_line)
                    } else {
                        format!("    {}", child_line)
                    }
                });
        let children_lines = child_node_lines.chain(formatted_last_child_node_lines_iter);
        lines.extend(children_lines);
    }
    lines
}
