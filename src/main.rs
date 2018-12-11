#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

mod dao;
mod model;

use std::error::Error;
use std::env;
use std::str::FromStr;
use lambda::error::HandlerError;
use rusoto_core::Region;

use model::{
    CustomEvent,
    CustomOutput,
};
use dao::HelloDAO;

const DYNAMO_REGION_ENV_KEY: &'static str = "DYNAMO_REGION";
const DEFAULT_REGION_NAME: &'static str = "local";
const DEFAULT_REGION_ENDPOINT: &'static str = "http://localhost:8000";

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(event: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    validate(&event, &c)
        .and_then(|e| {
            let mut dao = HelloDAO::new(determine_region());
            dao.put(e).sync()
                .map_err(|e| {
                    error!("Error putting item: {:?}", e);
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

fn determine_region() -> Region {
    env::var(DYNAMO_REGION_ENV_KEY)
        .map(|reg| {
            let trimmed = reg.trim();
            let result = Region::from_str(trimmed.as_ref());
            if let &Err(_) = &result {
                warn!("Could not parse dynamo region of {} returning local.", reg);
            }
            result.unwrap_or(default_region())
        })
        .unwrap_or({
            warn!("No dynamo region was specified in env variable {} returning default", DYNAMO_REGION_ENV_KEY);
            default_region()
        })
}

fn default_region() -> Region {
    Region::Custom {
        name: DEFAULT_REGION_NAME.to_string(),
        endpoint: DEFAULT_REGION_ENDPOINT.to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_validate() {
        let context = lambda::Context::default();
        let e0 = CustomEvent::new("sldkfj@sldfkj.com", "", "Blah");
        assert!(validate(&e0, &context).is_err());
        let e1 = CustomEvent::new("", "sdfl", "slkjdf");
        assert!(validate(&e1, &context).is_err());
        let e2 = CustomEvent::new("asdf", "sdfl", "");
        assert!(validate(&e2, &context).is_err());
        let e2 = CustomEvent::new("asdf", "sdfl", "asdf");
        assert!(validate(&e2, &context).is_ok());
    }

    #[test]
    fn test_determine_region() {
        let default_region = default_region();

        env::remove_var(DYNAMO_REGION_ENV_KEY);
        let r0 = determine_region();
        assert_eq!(default_region, r0);

        env::set_var(DYNAMO_REGION_ENV_KEY, "us-west-2");
        let r1 = determine_region();
        assert_eq!(Region::UsWest2, r1);

        env::set_var(DYNAMO_REGION_ENV_KEY, "  us-west-2");
        let r2 = determine_region();
        assert_eq!(Region::UsWest2, r2)
    }


}