#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use std::error::Error;

use lambda::error::HandlerError;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    first_name: String
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    message: String
}

impl CustomOutput {
    fn new(message: String) -> CustomOutput {
        CustomOutput {
            message
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        Err(c.new_error("Empty First Name"))
    }
    else {
        Ok(CustomOutput::new(format!("Hello, {}", e.first_name)))
    }
}