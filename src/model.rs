#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomEvent {
    pub email: String,
    pub first_name: String,
    pub last_name: String
}

impl CustomEvent {

    pub fn new(email: &str, first_name: &str, last_name: &str) -> CustomEvent {
        CustomEvent {
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string()
        }
    }

}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomOutput {
    pub message: String
}

impl CustomOutput {
    pub fn new(message: String) -> CustomOutput {
        CustomOutput {
            message
        }
    }
}