use crate::{common, rules};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

/// Error code 2 if exceeded
pub fn upper(variables: &HashMap<String, Value>, rules_config: &rules::Config) {
    // Check if value is a number (either float or int)
    for (key, value) in variables.iter() {
        if value.is_number() {
            for v in &rules_config.variables {
                if &v.name == key {
                    if value.as_f64().expect("Conversion error") > v.rules.bounds.upper {
                        println!("{}: {}", key, value);
                    }
                }
            }
        }
    }
}

/// Error code 3 if exceeded
pub fn lower(variables: &HashMap<String, Value>, rules_config: &rules::Config) {
    // Check if value is a number (either float or int)
    for (key, value) in variables.iter() {
        if value.is_number() {
            for v in &rules_config.variables {
                if &v.name == key {
                    if value.as_f64().expect("Conversion error") < v.rules.bounds.lower {
                        println!("{}: {}", key, value);
                    }
                }
            }
        }
    }
}

/// Combined upper and lower as limits.
/// Error code 2 if upper is exceeded.
/// Error code 3 if lower is exceeded.
pub fn limits(
    device_name: &String,
    variables: &HashMap<String, Value>,
    rules_config: &rules::Config,
) -> Result<common::Response, Box<dyn Error>> {
    let mut response = common::Response::new(&device_name);
    for (key, value) in variables.iter() {
        if value.is_number() {
            for v in &rules_config.variables {
                if &v.name == key {
                    match value.as_f64() {
                        Some(val) => {
                            if val < v.rules.bounds.lower {
                                response.variables.push(common::Variable {
                                    name: (&key).to_string(),
                                    value: Some(val),
                                    error_code: 3,
                                });
                                break;
                            }
                            if val > v.rules.bounds.upper {
                                response.variables.push(common::Variable {
                                    name: (&key).to_string(),
                                    value: Some(val),
                                    error_code: 2,
                                });
                                break;
                            }
                            response.variables.push(common::Variable {
                                name: (&key).to_string(),
                                value: Some(val),
                                error_code: 1,
                            });
                        }
                        None => response.variables.push(common::Variable {
                            name: (&key).to_string(),
                            value: None,
                            error_code: 99,
                        }),
                    }
                };
            }
        }
    }
    Ok(response)
}
