use std::collections::HashMap;

use serde_json::{json, Value};
use tree_sitter::Node;

use crate::{
    parsers::{
        json_schema::{
            errors::additional_properties_error,
            json_schema_parser::JSONSchemaValidator,
            utils::{ir::to_diagnostic, object::Properties},
        },
        parser::ParseResult,
    },
    utils::tree_sitter::{end_position, start_position},
};

pub fn validate_additional_properties<'a>(
    validate: &'a JSONSchemaValidator,
    available_keys: &HashMap<&'a str, Node>,
    sub_schema: &Value,
    contents: &str,
) -> Option<Properties<'a>> {
    let properties = sub_schema.get("additionalProperties")?;

    let mut validations: Vec<ParseResult> = Vec::new();
    let mut keys_used = Vec::new();

    match properties {
        Value::Bool(boo) => {
            if boo == &true {
                // re-evaluate all the property nodes that haven't been visited yet against the schema

                for (key, value) in available_keys {
                    keys_used.push(key.to_owned());
                    validations.push(validate.validate_root(value.walk(), sub_schema, contents));
                }
            } else {
                let mut validation = ParseResult::default();
                // properties/patternProperties have already removed all their matching nodes, these are additional properties that shouldn't be here
                for (additional_property, value) in available_keys {
                    keys_used.push(additional_property.to_owned());
                    validation.errors.push(to_diagnostic(
                        start_position(value),
                        end_position(value),
                        additional_properties_error(additional_property),
                    ));
                }
                validations.push(validation);
            }

            Some(Properties {
                keys_used,
                validation: validations,
            })
        }
        Value::Object(obj) => {
            for (key, value) in available_keys {
                keys_used.push(key.to_owned());
                validations.push(validate.validate_root(value.walk(), &json!(obj), contents));
            }

            Some(Properties {
                keys_used,
                validation: validations,
            })
        }
        _ => Some(Properties {
            keys_used,
            validation: validations,
        }),
    }
}
