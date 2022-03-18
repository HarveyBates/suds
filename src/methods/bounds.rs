use crate::rules;
use serde_json::Value;

/// Error code 2 if exceeded
pub fn upper(key: &String, value: &Value, rules_config: &Vec<rules::Config>) {
    // Check if value is a number (either float or int)
    if value.is_number() {
        for c in rules_config {
            for v in &c.variables {
                if &v.name == key {
                    if value.as_f64().expect("Conversion error") > v.rules.upper {
                        println!("{}: {}", key, value);
                    }
                }
            }
        }
    }
}

/// Error code 3 if exceeded
pub fn lower(key: &String, value: &Value, rules_config: &Vec<rules::Config>) {
    // Check if value is a number (either float or int)
    if value.is_number() {
        for c in rules_config {
            for v in &c.variables {
                if &v.name == key {
                    if value.as_f64().expect("Conversion error") < v.rules.lower {
                        println!("{}: {}", key, value);
                    }
                }
            }
        }
    }
}
