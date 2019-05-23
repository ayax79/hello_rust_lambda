mod dao;
mod errors;
mod model;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{debug, error, warn};
use rusoto_core::Region;
use simple_error::bail;
use std::env;
use std::error::Error;
use std::str::FromStr;

use crate::dao::HelloDAO;
use crate::model::{CustomEvent, CustomOutput};

const DYNAMO_REGION_ENV_KEY: &'static str = "DYNAMO_REGION";
const DEFAULT_REGION_NAME: &'static str = "local";
const DEFAULT_REGION_ENDPOINT: &'static str = "http://localhost:8000";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::try_init()?;

    lambda!(my_handler);
    Ok(())
}

fn my_handler(event: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
    validate(&event, &c).and_then(|e| {
        let region = determine_region();
        debug!("Configuring Dynamo client with region {:?}", &region);
        let mut dao = HelloDAO::new(region);
        dao.put(e)
            .map_err(|e| e.into())
            .map(|_| CustomOutput::new(format!("Hello {}", e.first_name)))
    })
}

fn validate<'a>(e: &'a CustomEvent, c: &Context) -> Result<&'a CustomEvent, HandlerError> {
    if e.email.is_empty() {
        error!("Empty email in request {}", c.aws_request_id);
        bail!("Empty email")
    } else if e.first_name.is_empty() {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty First Name")
    } else if e.last_name.is_empty() {
        error!("Empty last name in request {}", c.aws_request_id);
        bail!("Empty Last Name")
    } else {
        Ok(e)
    }
}

fn determine_region() -> Region {
    env::var(DYNAMO_REGION_ENV_KEY)
        .map(|reg| {
            let trimmed = reg.trim();
            debug!("Attempting to acquire region for string {} ", trimmed);
            let result = Region::from_str(trimmed);
            if let &Err(_) = &result {
                warn!("Could not parse dynamo region of {} returning local.", reg);
            }
            result.unwrap_or(default_region())
        })
        .unwrap_or_else(|_| {
            warn!(
                "No dynamo region was specified in env variable {} returning default",
                DYNAMO_REGION_ENV_KEY
            );
            default_region()
        })
}

fn default_region() -> Region {
    Region::Custom {
        name: DEFAULT_REGION_NAME.to_string(),
        endpoint: DEFAULT_REGION_ENDPOINT.to_string(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_validate() {
        let context = Context::default();
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
