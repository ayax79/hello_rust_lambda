use std::convert::Into;
use std::collections::HashMap;

use rusoto_core::{
    Region,
    RusotoFuture
};
use rusoto_dynamodb::{
    DynamoDb,
    DynamoDbClient,
    PutItemInput,
    AttributeValue,
    PutItemOutput,
    PutItemError
};
use model::CustomEvent;

const TABLE_NAME: &'static str = "hello_events";

pub struct HelloDAO {
    client: DynamoDbClient
}

impl HelloDAO {
    pub fn new(region: Region) -> Self {
        let client = DynamoDbClient::new(region);
        HelloDAO {
            client
        }
    }

    pub fn put(&mut self, event: &CustomEvent) -> RusotoFuture<PutItemOutput, PutItemError> {
        self.client.put_item(event.into())
    }
}


impl <'a> Into<PutItemInput> for &'a CustomEvent {
    fn into(self) -> PutItemInput {
        let item_map: HashMap<String, AttributeValue> =
            [("first_name".to_string(), from_string(self.first_name.clone())),
                ("last_name".to_string(), from_string(self.last_name.clone())),
                ("email".to_string(), from_string(self.email.clone()))]
                .iter()
                .cloned()
                .collect();

        let mut item = PutItemInput::default();
        item.item = item_map;
        item.table_name = TABLE_NAME.to_string();
        item
    }
}


fn from_string(s: String) -> AttributeValue {
    let mut att = AttributeValue::default();
    att.s = Some(s);
    att
}



