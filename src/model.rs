#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomEvent {
    pub email: String,
    pub first_name: String,
    pub last_name: String
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