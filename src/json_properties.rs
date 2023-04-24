mod template;

use super::{Error, Node};
use serde_json::Map;
use serde_json::Value as JsonValue;
use template::Template;

pub fn deserialize(
    serialized: String,
    template_str: String,
    children_key: String,
    default: Option<String>,
) -> Result<Node, Error> {
    if serialized.trim().is_empty() {
        return Err(Error::EmptyInputError);
    }
    let root_value: JsonValue = json5::from_str(&serialized)?;
    let template = Template::new(template_str)?;
    match root_value {
        JsonValue::Array(vec) => {
            if vec.len() > 1 {
                Err(Error::MultipleRootsError)
            } else if vec.is_empty() {
                Err(Error::EmptyInputError)
            } else {
                let root_obj = vec.first().unwrap();
                json_value_to_node(root_obj, &template, &children_key, &default)?
                    .ok_or(Error::EmptyInputError)
            }
        }
        root_obj @ JsonValue::Object(_) => {
            json_value_to_node(&root_obj, &template, &children_key, &default)?
                .ok_or(Error::EmptyInputError)
        }
        _ => Err(Error::FormatSpecificError(
            "root item must be a root object or an array containing a root object".to_string(),
        )),
    }
}

fn json_value_to_node(
    value: &JsonValue,
    template: &Template,
    children_key: &str,
    default: &Option<String>,
) -> Result<Option<Node>, Error> {
    match value {
        JsonValue::Object(map) => {
            let name = get_name(template, map, default)?;
            let children = match map.get(children_key) {
                Some(JsonValue::Object(children_json_values)) => children_json_values
                    .values()
                    .flat_map(|value| {
                        Result::transpose(json_value_to_node(
                            value,
                            template,
                            children_key,
                            default,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>(),
                Some(JsonValue::Array(children_json_values)) => children_json_values
                    .iter()
                    .flat_map(|value| {
                        Result::transpose(json_value_to_node(
                            value,
                            template,
                            children_key,
                            default,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>(),
                None => Ok(Vec::new()),
                _ => Ok(Vec::new()),
            }?;
            Ok(Some(Node { name, children }))
        }
        _ => Ok(None),
    }
}

fn get_name(
    template: &Template,
    map: &Map<String, JsonValue>,
    default: &Option<String>,
) -> Result<String, Error> {
    // Option<&String> -> Option<&JsonValue::Value<String>>
    template.fill(|placeholder_name| {
        map.get(placeholder_name)
            .map(|v| {
                if let JsonValue::String(s) = v {
                    s.to_string()
                } else {
                    v.to_string()
                }
            })
            .or_else(|| default.clone())
            .ok_or(Error::FormatSpecificError(
                "missing template value".to_string(),
            ))
    })
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
        let deserialization_err = deserialize(
            json.to_string(),
            "{name}".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap_err();
        let is_format_error = matches!(deserialization_err, Error::FormatSpecificError(_));
        assert!(is_format_error);
    }

    #[test]
    fn zero_length_json() {
        let json = r#""#;
        let deserialization_err = deserialize(
            json.to_string(),
            "{name}".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap_err();
        assert_eq!(deserialization_err, Error::EmptyInputError);
    }

    #[test]
    fn empty_object_json() {
        let json = r#"{}"#;
        let deserialization_err = deserialize(
            json.to_string(),
            "{name}".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap_err();
        assert_eq!(
            deserialization_err,
            Error::FormatSpecificError("missing template value".to_string())
        );
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
        let deserialization_err = deserialize(
            json.to_string(),
            "name".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap_err();
        assert_eq!(deserialization_err, Error::MultipleRootsError);
    }

    #[test]
    fn good_json() {
        let json = r#"
            {
                "name": "big root boy",
                "children": [
                    {
                        "name": "me, the bean man"
                    },
                    {
                        "name": "another child of beans"
                    }
                ]
            }
        "#;
        let root_node = deserialize(
            json.to_string(),
            "{name}".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap();
        assert_eq!(
            root_node,
            Node {
                name: "big root boy".to_string(),
                children: vec![
                    Node {
                        name: "me, the bean man".to_string(),
                        children: Vec::new()
                    },
                    Node {
                        name: "another child of beans".to_string(),
                        children: Vec::new()
                    }
                ]
            }
        );
    }

    #[test]
    fn good_json_arr() {
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
        let root_node = deserialize(
            json.to_string(),
            "{name}".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap();
        assert_eq!(
            root_node,
            Node {
                name: "big root boy".to_string(),
                children: vec![
                    Node {
                        name: "me, the bean man".to_string(),
                        children: Vec::new()
                    },
                    Node {
                        name: "another child of beans".to_string(),
                        children: Vec::new()
                    }
                ]
            }
        );
    }

    #[test]
    fn good_json_diff_json_prop_names() {
        let json = r#"
            {
                "moniker": "big root boy",
                "progeny": [
                    {
                        "moniker": "me, the bean man"
                    },
                    {
                        "moniker": "another child of beans"
                    }
                ]
            }
        "#;
        let root_node = deserialize(
            json.to_string(),
            "{moniker}".to_string(),
            "progeny".to_string(),
            None,
        )
        .unwrap();
        assert_eq!(
            root_node,
            Node {
                name: "big root boy".to_string(),
                children: vec![
                    Node {
                        name: "me, the bean man".to_string(),
                        children: Vec::new()
                    },
                    Node {
                        name: "another child of beans".to_string(),
                        children: Vec::new()
                    }
                ]
            }
        );
    }
    #[test]
    fn json_with_missing_prop_no_default() {
        let json = r#"
            {
                "other_thing": "other thing value",
                "not_in_all_of_them": "sassy kid",
                "children": [
                    {
                        "other_thing": "other thing value 2",
                        "not_in_all_of_them": "sassy kid 2"
                    },
                    {
                        "other_thing": "other thing value 3"
                    }
                ]
            }
        "#;
        let deserialization_err = deserialize(
            json.to_string(),
            "{other_thing}: {not_in_all_of_them}".to_string(),
            "children".to_string(),
            None,
        )
        .unwrap_err();
        assert_eq!(
            deserialization_err,
            Error::FormatSpecificError("missing template value".to_string())
        );
    }
    #[test]
    fn json_with_missing_prop_with_default() {
        let json = r#"
            {
                "other_thing": "other thing value",
                "not_in_all_of_them": "sassy kid",
                "children": [
                    {
                        "other_thing": "other thing value 2",
                        "not_in_all_of_them": "sassy kid 2"
                    },
                    {
                        "other_thing": "other thing value 3"
                    }
                ]
            }
        "#;
        let root_node = deserialize(
            json.to_string(),
            "{other_thing}: {not_in_all_of_them}".to_string(),
            "children".to_string(),
            Some("<u fucked up>".to_string()),
        )
        .unwrap();
        let expected_root_node = Node {
            name: "other thing value: sassy kid".to_string(),
            children: vec![
                Node {
                    name: "other thing value 2: sassy kid 2".to_string(),
                    children: Vec::new(),
                },
                Node {
                    name: "other thing value 3: <u fucked up>".to_string(),
                    children: Vec::new(),
                },
            ],
        };
        assert_eq!(root_node, expected_root_node);
    }
}
