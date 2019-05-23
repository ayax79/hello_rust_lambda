use std::collections::HashMap;
use std::convert::Into;
use std::error::Error;

use crate::errors::HelloError;
use crate::model::CustomEvent;
use log::{error, info};
use rusoto_core::RusotoError;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, PutItemOutput};

const TABLE_NAME: &'static str = "hello_events";

pub struct HelloDAO {
    client: DynamoDbClient,
}

impl HelloDAO {
    pub fn new(region: Region) -> Self {
        let client = match region {
            Region::Custom {
                name: ref n,
                endpoint: _,
            } if n == super::DEFAULT_REGION_NAME => build_local_dynamodb_client(&region),
            _ => DynamoDbClient::new(region),
        };

        HelloDAO { client }
    }

    pub fn put(&mut self, event: &CustomEvent) -> Result<PutItemOutput, HelloError> {
        self.client.put_item(event.into()).sync().map_err(|e| {
            log_rusoto_error(event, &e);
            HelloError::from(e)
        })
    }
}

impl<'a> Into<PutItemInput> for &'a CustomEvent {
    fn into(self) -> PutItemInput {
        let mut item = PutItemInput::default();
        item.item = [
            ("email".to_string(), from_string(self.email.clone())),
            (
                "first_name".to_string(),
                from_string(self.first_name.clone()),
            ),
            ("last_name".to_string(), from_string(self.last_name.clone())),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<String, AttributeValue>>();
        item.table_name = TABLE_NAME.to_string();
        item
    }
}

fn from_string(s: String) -> AttributeValue {
    let mut att = AttributeValue::default();
    att.s = Some(s);
    att
}

fn build_local_dynamodb_client(region: &Region) -> DynamoDbClient {
    let credentials_provider =
        StaticProvider::new("fakeKey".to_string(), "fakeSecret".to_string(), None, None);

    let dispatcher = HttpClient::new().expect("could not create http client");

    info!("Creating local connection with region {:#?}", region);
    DynamoDbClient::new_with(dispatcher, credentials_provider, region.clone())
}

fn log_rusoto_error<E: Error + 'static>(event: &CustomEvent, e: &RusotoError<E>) {
    match e {
        // if we received an unknown error, we will need to parse it to log it appropriately
        &RusotoError::Unknown(ref response) => {
            let body_as_string = response.body_as_str();
            error!(
                "Unknown error putting event {:?} with error response body of {:?}",
                event, body_as_string
            )
        }
        _ => error!("Error putting event: {:?} : error {}", event, e),
    }
}

#[cfg(test)]
mod tests {
    // extern crate dynamodb_testcontainer;
    use pretty_env_logger;
    use testcontainers;

    use rusoto_core::Region;
    use rusoto_dynamodb::{
        AttributeDefinition, CreateTableInput, DynamoDb, KeySchemaElement, ProvisionedThroughput,
    };

    use self::testcontainers::images::dynamodb_local::DynamoDb as DynamoDbImage;
    use self::testcontainers::*;
    use super::*;

    #[test]
    fn test_put_get() {
        let _ = pretty_env_logger::try_init();
        let docker = clients::Cli::default();
        let node = docker.run(DynamoDbImage::default());
        let host_port = node.get_host_port(8000).unwrap();

        let region = Region::Custom {
            name: "local".to_string(),
            endpoint: format!("http://localhost:{}", host_port),
        };
        let mut hello_dao = HelloDAO::new(region);
        create_table(&hello_dao.client);

        let event = CustomEvent {
            email: "foo@bar.com".to_string(),
            first_name: "Foo".to_string(),
            last_name: "Bar".to_string(),
        };

        let result = hello_dao.put(&event);
        if let &Err(ref e) = &result {
            error!("create error: {:#?}", e);
        }
        assert!(result.is_ok());
    }

    fn create_table(client: &DynamoDbClient) {
        let mut input = CreateTableInput::default();
        input.table_name = TABLE_NAME.to_string();

        let mut key_schema = KeySchemaElement::default();
        key_schema.attribute_name = "email".to_string();
        key_schema.key_type = "HASH".to_string();
        input.key_schema = vec![key_schema];

        let mut att_def = AttributeDefinition::default();
        att_def.attribute_name = "email".to_string();
        att_def.attribute_type = "S".to_string();
        input.attribute_definitions = vec![att_def];

        let mut provisioned_throughput = ProvisionedThroughput::default();
        provisioned_throughput.read_capacity_units = 5;
        provisioned_throughput.write_capacity_units = 5;
        input.provisioned_throughput = Some(provisioned_throughput);

        let result = client.create_table(input).sync();

        if let &Err(ref e) = &result {
            error!("table creation result: {:#?}", e);
        }
        assert!(result.is_ok());
    }
}
