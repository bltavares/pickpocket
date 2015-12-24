extern crate hyper;
extern crate rustc_serialize;

use hyper::header::{Connection, ContentType};
use hyper::{Client, Url};
use rustc_serialize::json::Json;
use std::collections::HashMap;
use std::env;
use std::io::{Read, BufReader, BufRead};

const ENDPOINT: &'static str = "https://getpocket.com/v3";
fn url(method: &str) -> Url {
    Url::parse(&format!("{}{}", ENDPOINT, method)).unwrap()
}

fn mark_as_read(consumer_key: &str, auth_code: &str, ids: &Vec<&String>) {
    let client = Client::new();

    let method = url("/send");
    let actions: Vec<String> = ids.iter()
                                  .map(|id| {
                                      format!(r##"{{ "action": "archive", "item_id": "{}" }}"##, id)
                                  })
                                  .collect();
    let payload = format!(r##"{{ "consumer_key":"{}",
                           "access_token":"{}",
                           "actions": [{}]
                           }}"##,
                          consumer_key,
                          auth_code,
                          actions.join(", "));
    println!("Payload: {}", payload);
    let mut res = client.post(method)
                        .body(&payload)
                        .header(ContentType::json())
                        .header(Connection::close())
                        .send()
                        .unwrap();
}

fn get(consumer_key: &str, auth_code: &str) -> Json {
    let client = Client::new();

    let method = url("/get");
    let payload = format!(r##"{{ "consumer_key":"{}",
                           "access_token":"{}",
                           "state":"unread",
                           "detailType":"simple"
                           }}"##,
                          consumer_key,
                          auth_code);
    let mut res = client.post(method)
                        .body(&payload)
                        .header(ContentType::json())
                        .header(Connection::close())
                        .send()
                        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Json::from_str(&body).unwrap()
}

fn main() {
    let consumer_env_key = "POCKET_CONSUMER_KEY";
    let consumer_key = env::var(consumer_env_key)
                           .expect(&format!("Consumer key should be available on the \
                                             environment variable {}",
                                            consumer_env_key));

    let auth_env_key = "POCKET_AUTHORIZATION_CODE";
    let authorization_key = env::var(auth_env_key)
                                .expect(&format!("Authorization key should be available on the \
                                                  environment variable {}",
                                                 auth_env_key));
    let file_name = env::args()
                        .skip(1)
                        .next()
                        .expect("Expected an file as argument");

    let file = std::fs::File::open(file_name).unwrap();

    let reading_list = get(&consumer_key, &authorization_key);

    let mut url_id: HashMap<String, String> = HashMap::new();

    for (id, reading_item) in reading_list.as_object()
                                          .unwrap()
                                          .get("list")
                                          .unwrap()
                                          .as_object()
                                          .unwrap()
                                          .iter() {
        let item = reading_item.as_object()
                               .unwrap();
        let url = match item.get("resolved_url") {
            Some(x) => x,
            None => item.get("given_url").unwrap(),
        };

        url_id.insert(String::from(url.as_string().unwrap()), id.clone());
    }

    println!("debug: References\n{:?}", &url_id);
    let mut ids = Vec::new();

    for line in BufReader::new(file).lines() {
        let url = line.unwrap();
        match url_id.get(&url) {
            Some(id) => ids.push(id),
            None => println!("Url {} did not match", &url),
        }
    }

    println!("debug: Ids\n{:?}", &ids);

    mark_as_read(&consumer_key, &authorization_key, &ids);
}
