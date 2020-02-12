use super::{Error, Node};
use serde_json;
use serde_json::Value as JsonValue;
use std::convert::From;

pub fn deserialize(serialized: String) -> Result<Node, Error> {
    if serialized.trim().len() == 0 {
        return Err(Error::EmptyInputError);
    }
    let root_value: JsonValue = serde_json::from_str(&serialized)?;
    match root_value {
        JsonValue::Object(map) => {
            if map.len() > 1 {
                Err(Error::MultipleRootsError)
            } else if map.len() == 0 {
                Err(Error::EmptyInputError)
            } else {
                let root_entry = map.iter().next().unwrap();
                Ok(json_value_to_node(root_entry.0.to_string(), root_entry.1))
            }
        }
        _ => Err(Error::FormatSpecificError("root item must be an object".to_string())),
    }
}

fn json_value_to_node(name: String, value: &JsonValue) -> Node {
    match value {
        JsonValue::Object(map) => Node {
            name: name,
            children: map
                .iter()
                .map(|(name, value)| json_value_to_node(name.to_string(), value))
                .collect(),
        },
        _ => Node::new(name),
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::error::Error) -> Error {
        Error::FormatSpecificError(format!("{}", serde_error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_json() {
        let json = r#"
            {{{
                        ---
                    "beans": {
                        "man": null
                    },
                    "wow": null
                },
                "another one": null
            }
        "#;
        let deserialization_err = deserialize(json.to_string()).unwrap_err();
        let is_format_error = if let Error::FormatSpecificError(_) = deserialization_err {
            true
        } else {
            false
        };
        assert!(is_format_error);
    }

    #[test]
    fn zero_length_json() {
        let json = r#""#;
        let deserialization_err = deserialize(json.to_string()).unwrap_err();
        assert_eq!(deserialization_err, Error::EmptyInputError);
    }

    #[test]
    fn empty_object_json() {
        let json = r#"{}"#;
        let deserialization_err = deserialize(json.to_string()).unwrap_err();
        assert_eq!(deserialization_err, Error::EmptyInputError);
    }

    #[test]
    fn multiple_roots_json() {
        let json = r#"
            {
                "cool": {
                    "beans": {
                        "man": null
                    },
                    "wow": null
                },
                "another one": null
            }
        "#;
        let deserialization_err = deserialize(json.to_string()).unwrap_err();
        assert_eq!(deserialization_err, Error::MultipleRootsError);
    }

    #[test]
    fn good_json() {
        let json = r#"
            {
                "cool": {
                    "beans": {
                        "man": null
                    },
                    "wow": null
                }
            }
        "#;
        let root_node = deserialize(json.to_string()).unwrap();
        assert_eq!(
            root_node,
            Node {
                name: "cool".to_string(),
                children: vec![
                    Node {
                        name: "beans".to_string(),
                        children: vec![Node {
                            name: "man".to_string(),
                            children: vec![]
                        }]
                    },
                    Node {
                        name: "wow".to_string(),
                        children: vec![]
                    }
                ]
            }
        );
    }
}
