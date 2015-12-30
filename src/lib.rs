extern crate hyper;
extern crate rustc_serialize;

use self::hyper::header::{Connection, ContentType};
use self::hyper::Url;
use self::rustc_serialize::json;
use self::rustc_serialize::Decoder;
use std::collections::HashMap;
use std::io::Read;

mod auth;
pub use auth::*;

#[derive(RustcDecodable)]
pub struct Item {
    given_url: String,
    resolved_url: Option<String>,
}

#[derive(RustcDecodable)]
pub struct ReadingListResponse {
    pub list: HashMap<String, Item>,
}

impl Item {
    pub fn url(&self) -> &str {
        &self.resolved_url.as_ref().unwrap_or(&self.given_url)
    }
}

impl Client {
    pub fn mark_as_read(&self, ids: &Vec<&str>) {
        let method = url("/send");
        let actions: Vec<String> = ids.iter()
                                      .map(|id| {
                                          format!(r##"{{ "action": "archive", "item_id": "{}" }}"##,
                                                  id)
                                      })
                                      .collect();
        let payload = format!(r##"{{ "consumer_key":"{}",
                               "access_token":"{}",
                               "actions": [{}]
                               }}"##,
                              &self.consumer_key,
                              &self.authorization_code,
                              actions.join(", "));

        self.request(method, payload);
    }

    pub fn list_all(&self) -> ReadingListResponse {
        let method = url("/get");
        let payload = format!(r##"{{ "consumer_key":"{}",
                               "access_token":"{}",
                               "state":"all",
                               "detailType":"simple"
                               }}"##,
                              &self.consumer_key,
                              &self.authorization_code);

        let response = self.request(method, payload);
        json::decode(&response).expect("Couldn't parse /get response")
    }

    fn request(&self, method: Url, payload: String) -> String {
        let client = hyper::Client::new();

        let mut res = client.post(method)
                            .body(&payload)
                            .header(ContentType::json())
                            .header(Connection::close())
                            .send()
                            .expect(&format!("Coulnd't make request with payload: {}", &payload));

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }
}
