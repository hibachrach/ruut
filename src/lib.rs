mod json;
mod json_properties;
mod parens;

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

impl render_as_tree::Node for Node {
    type Iter<'a> = std::slice::Iter<'a, Self>;
    fn name(&self) -> &str {
        &self.name
    }
    fn children(&self) -> Self::Iter<'_> {
        self.children.iter()
    }
}

pub enum InputFormat {
    Parens,
    Json,
    JsonProperties,
}

impl FromStr for InputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parens" => Ok(InputFormat::Parens),
            "json" => Ok(InputFormat::Json),
            "jsonprop" => Ok(InputFormat::JsonProperties),
            _ => Err("invalid format type"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyInputError,
    MissingPropError,
    MultipleRootsError,
    FormatSpecificError(String),
}

impl From<json5::Error> for Error {
    fn from(serde_error: json5::Error) -> Error {
        Error::FormatSpecificError(format!("{}", serde_error))
    }
}

pub fn prettify(
    serialized: String,
    format: InputFormat,
    template: String,
    children_key: String,
    default: Option<String>,
) -> Result<String, Error> {
    let root = match format {
        InputFormat::Parens => parens::deserialize(serialized),
        InputFormat::Json => json::deserialize(serialized),
        InputFormat::JsonProperties => {
            json_properties::deserialize(serialized, template, children_key, default)
        }
    }?;
    Ok(render_as_tree::render(&root).join("\n"))
}
