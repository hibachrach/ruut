use super::Error;
use itertools::interleave;

const BRACE_OPEN: char = '{';
const BRACE_CLOSE: char = '}';

#[derive(Debug, PartialEq)]
pub struct Template {
    unchanging_parts: Vec<String>,
    placeholder_names: Vec<String>,
}

impl Template {
    pub fn new(template_string: String) -> Result<Template, Error> {
        enum CurrentPart {
            UnchangingPart(String),
            PlaceholderPart(String),
        }
        use CurrentPart::*;

        let mut unchanging_parts = Vec::new();
        let mut placeholder_names = Vec::new();
        let mut cur_part = CurrentPart::UnchangingPart(String::new());
        for c in template_string.chars() {
            match c {
                BRACE_OPEN => match cur_part {
                    PlaceholderPart(_) => {
                        return Err(Error::FormatSpecificError(
                            "template placeholder cannot contain `{`".to_string(),
                        ));
                    }
                    UnchangingPart(cur_unchanging_part) => {
                        unchanging_parts.push(cur_unchanging_part);
                        cur_part = PlaceholderPart(String::new());
                    }
                },
                BRACE_CLOSE => {
                    if let PlaceholderPart(cur_pl_name) = cur_part {
                        placeholder_names.push(cur_pl_name);
                        cur_part = UnchangingPart(String::new());
                    }
                }
                non_brace_character => match &mut cur_part {
                    PlaceholderPart(cur_pl_name) => {
                        cur_pl_name.push(non_brace_character);
                    }
                    UnchangingPart(cur_unc_part) => {
                        cur_unc_part.push(non_brace_character);
                    }
                },
            }
        }

        match cur_part {
            PlaceholderPart(_) => {
                return Err(Error::FormatSpecificError(
                    "template placeholder missing closing `}`".to_string(),
                ));
            }
            UnchangingPart(cur_unchanging_part) => {
                unchanging_parts.push(cur_unchanging_part);
            }
        }

        Ok(Template {
            unchanging_parts,
            placeholder_names,
        })
    }

    pub fn fill<F>(&self, get_placeholder_value: F) -> Result<String, Error>
    where
        F: Fn(&String) -> Result<String, Error>,
    {
        let placeholder_values_iter = self
            .placeholder_names
            .iter()
            .map(get_placeholder_value)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(
            interleave(self.unchanging_parts.clone(), placeholder_values_iter)
                .collect::<Vec<_>>()
                .concat(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_new_good_template_str() {
        let expected = Ok(Template {
            unchanging_parts: vec!["that's my ".to_string(), ", by golly!".to_string()],
            placeholder_names: vec!["name".to_string()],
        });
        assert_eq!(
            Template::new("that's my {name}, by golly!".to_string()),
            expected
        );
    }
    #[test]
    fn template_new_good_template_str_ending_with_placeholder() {
        let expected = Ok(Template {
            unchanging_parts: vec!["that's my ".to_string(), "".to_string()],
            placeholder_names: vec!["name".to_string()],
        });
        assert_eq!(Template::new("that's my {name}".to_string()), expected);
    }
    #[test]
    fn template_new_good_template_str_beginning_with_placeholder() {
        let expected = Ok(Template {
            unchanging_parts: vec!["".to_string(), ", don't touch that!".to_string()],
            placeholder_names: vec!["name".to_string()],
        });
        assert_eq!(
            Template::new("{name}, don't touch that!".to_string()),
            expected
        );
    }
    #[test]
    fn template_new_good_template_str_with_only_placeholders() {
        let expected = Ok(Template {
            unchanging_parts: vec!["".to_string(), "".to_string(), "".to_string()],
            placeholder_names: vec!["name".to_string(), "id".to_string()],
        });
        assert_eq!(Template::new("{name}{id}".to_string()), expected);
    }
    #[test]
    fn fill_good_template_str() {
        let filled_template = Template::new("that's my {name}, by golly!".to_string())
            .unwrap()
            .fill(|_placeholder_name| Ok("Billingsby".to_string()))
            .unwrap();
        assert_eq!(
            filled_template,
            "that's my Billingsby, by golly!".to_string()
        )
    }
    #[test]
    fn fill_good_template_str_dynamic() {
        let filled_template = Template::new("{name}".to_string())
            .unwrap()
            .fill(|placeholder_name| Ok(format!("{}-123", placeholder_name)))
            .unwrap();
        assert_eq!(filled_template, "name-123".to_string())
    }
}
