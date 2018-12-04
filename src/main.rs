#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

mod dao;
mod model;

use std::error::Error;

use lambda::error::HandlerError;

use rusoto_core::Region;

use model::{
    CustomEvent,
    CustomOutput
};
use dao::HelloDAO;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(event: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    validate(&event, &c)
        .and_then(|e| {
            let region = Region::UsWest2; // todo - determine this dynamically
            let mut dao = HelloDAO::new(region);
            dao.put(e).sync()
                .map_err(|e| {
                    error!("Error putting item: {}", e);
                    c.new_error(e.description())
                })
                .map(|_| e)
        })
        .map(|e| CustomOutput::new(format!("Hello {}", e.first_name)))
}

fn validate<'a>(e: &'a CustomEvent, c: &lambda::Context) -> Result<&'a CustomEvent, HandlerError> {
    if e.email.is_empty() {
        error!("Empty email in request {}", c.aws_request_id);
        Err(c.new_error("Empty email"))
    } else if e.first_name.is_empty() {
        error!("Empty first name in request {}", c.aws_request_id);
        Err(c.new_error("Empty First Name"))
    } else if e.last_name.is_empty() {
        error!("Empty last name in request {}", c.aws_request_id);
        Err(c.new_error("Empty Last Name"))
    } else {
        Ok(e)
    }
}