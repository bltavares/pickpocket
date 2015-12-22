extern crate hyper;

use std::io::Read;
use std::env;

use hyper::{Client, Url};
use hyper::header::{Connection, ContentType};

const ENDPOINT: &'static str = "https://getpocket.com/v3";
fn url(method: &str) -> Url {
    Url::parse(&format!("{}{}", ENDPOINT, method)).unwrap()
}

fn get(consumer_key: &str, auth_code: &str) {
    let client = Client::new();

    let method = url("/get");
    let payload = format!(r##"{{ "consumer_key":"{}",
                           "access_token":"{}",
                           "state":"unread",
                           "detailType":"simple"
                           }}"##,
                          consumer_key,
                          auth_code);
    println!("{}", payload);
    let mut res = client.post(method)
                        .body(&payload)
                        .header(ContentType::json())
                        .header(Connection::close())
                        .send()
                        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{:?}", res);
    println!("{}", body);
}

fn main() {
    let consumer_env_key = "POCKET_CONSUMER_KEY";
    let consumer_key = env::var(consumer_env_key)
                           .expect(&format!("Consumer key should be available on the environment \
                                            variable {}",
                                           consumer_env_key));

    let auth_env_key = "POCKET_AUTHORIZATION_CODE";
    let authorization_key = env::var(auth_env_key)
                                .expect(&format!("Authorization key should be available on the \
                                                 environment variable {}",
                                                auth_env_key));

    get(&consumer_key, &authorization_key);
}
