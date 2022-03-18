extern crate suds;
use serde::Deserialize;
use serde_json::{Result, Value};
use std::collections::HashMap;
pub use suds::{methods, rules};

type JsonMap = HashMap<String, Value>;

#[test]
#[ignore]
fn load_rules() {
    let rules_config = rules::Config::new();
    for rules in &rules_config {
        println!("Device: {}", &rules.device_name);
        for var in &rules.variables {
            println!("Variable: {}", &var.name);
            println!("Upper: {}\tLower: {}", var.rules.upper, var.rules.lower);
        }
    }
}

#[test]
fn load_message() {
    let rules_config = rules::Config::new();

    let data = r#"
    {
        "RTC": 16356445,
        "airTemperature": 17.3,
        "atmosphericPressure": 914.1,
        "battery": 4.147,
        "command": 0,
        "gustSpeed": 1.01,
        "precipitation": 0,
        "relativeHumidity": 82,
        "solar": 0,
        "solarPanel": 0,
        "strikes": 0,
        "vapourPressure": 1620,
        "windDirection": 118.3,
        "windSpeed": 0.43
      }"#
    .to_lowercase();

    let msg: JsonMap = serde_json::from_str(&data).expect("Unpack error");

    for (key, value) in msg.iter() {
        methods::bounds::upper(&key, &value, &rules_config);
    }
}
