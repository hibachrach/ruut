use super::{Error, Node};
use serde_json;
use serde_json::Value as JsonValue;

pub fn deserialize(serialized: String) -> Result<Node, Error> {
    if serialized.trim().len() == 0 {
        return Err(Error::EmptyInputError);
    }
    let root_value: JsonValue = serde_json::from_str(&serialized)?;
    match root_value {
        JsonValue::Array(vec) => {
            if vec.len() > 1 {
                Err(Error::MultipleRootsError)
            } else if vec.len() == 0 {
                Err(Error::EmptyInputError)
            } else {
                let root_obj = vec.iter().next().unwrap();
                json_value_to_node(root_obj)?.ok_or(Error::EmptyInputError)
            }
        },
        root_obj @ JsonValue::Object(_) => {
            json_value_to_node(&root_obj)?.ok_or(Error::EmptyInputError)
        },
        _ => Err(Error::FormatSpecificError(
            "root item must be a root object or an array containing a root object".to_string(),
        )),
    }
}

fn json_value_to_node(value: &JsonValue) -> Result<Option<Node>, Error> {
    match value {
        JsonValue::Object(map) => match map.get("name") {
            Some(name_value) => match name_value {
                JsonValue::String(name) => {
                    let children = match map.get("children") {
                        Some(
                            JsonValue::Object(children_json_values),
                        ) => children_json_values
                            .values()
                            .map(|value| json_value_to_node(value))
                            .collect::<Result<Vec<_>, _>>(),
                        Some(
                            JsonValue::Array(children_json_values)
                        ) => children_json_values
                            .iter()
                            .map(|value| json_value_to_node(value))
                            .collect::<Result<Vec<_>, _>>(),
                        None => Ok(vec![]),
                        _ => Ok(vec![]),
                    }?;
                    Ok(Some(Node {
                        name: name.to_string(),
                        children: children.into_iter().filter_map(|node| node).collect(),
                    }))
                }
                _ => Err(Error::FormatSpecificError("`name` must be a string".to_string())),
            },
            None => Err(Error::MissingNameError)
        },
        _ => Ok(None),
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
        assert_eq!(deserialization_err, Error::MissingNameError);
    }

    #[test]
    fn multiple_roots_arr_json() {
        let json = r#"
            [
                {
                    "name": "first root"
                },
                {
                    "name": "second root",
                    "children": {
                        "beans": {
                            "name": "me, the bean man"
                        }
                    }
                }
            ]
        "#;
        let deserialization_err = deserialize(json.to_string()).unwrap_err();
        assert_eq!(deserialization_err, Error::MultipleRootsError);
    }

    #[test]
    fn good_json() {
        let json = r#"
            [{
                "name": "big root boy",
                "children": [
                    {
                        "name": "me, the bean man"
                    },
                    {
                        "name": "another child of beans"
                    }
                ]
            }]
        "#;
        let root_node = deserialize(json.to_string()).unwrap();
        assert_eq!(
            root_node,
            Node {
                name: "big root boy".to_string(),
                children: vec![
                    Node {
                        name: "me, the bean man".to_string(),
                        children: vec![]
                    },
                    Node {
                        name: "another child of beans".to_string(),
                        children: vec![]
                    }
                ]
            }
        );
    }
}
