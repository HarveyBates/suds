use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct Response {
    pub device_name: String,
    pub variables: Vec<Variable>,
}

#[derive(Default, Debug, Serialize)]
pub struct Variable {
    pub name: String,
    pub value: Option<f64>,
    pub error_code: i32,
}

impl Response {
    pub fn new(device_name: &String) -> Self {
        Response {
            device_name: device_name.to_string(),
            variables: Vec::new(),
        }
    }
}
