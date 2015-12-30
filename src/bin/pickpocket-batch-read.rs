extern crate hyper;
extern crate rustc_serialize;
extern crate pickpocket;

use std::collections::HashMap;
use std::env;
use std::io::{BufReader, BufRead};

use pickpocket::Client;

fn main() {
    let consumer_env_key = "POCKET_CONSUMER_KEY";
    let consumer_key = env::var(consumer_env_key)
                           .expect(&format!("Consumer key should be available on the \
                                             environment variable {}",
                                            consumer_env_key));

    let auth_env_code = "POCKET_AUTHORIZATION_CODE";
    let authorization_code = env::var(auth_env_code)
                                 .expect(&format!("Authorization code should be available on \
                                                   the environment variable {}",
                                                  auth_env_code));
    let file_name = env::args()
                        .skip(1)
                        .next()
                        .expect("Expected an file as argument");

    let file = std::fs::File::open(&file_name).expect(&format!("Couldn't open {}", &file_name));

    let auth = Client {
        consumer_key: consumer_key,
        authorization_code: authorization_code,
    };

    let reading_list = auth.list_all();

    let mut url_id: HashMap<&str, &str> = HashMap::new();
    for (id, reading_item) in reading_list.list.iter() {
        url_id.insert(&reading_item.url(), id);
    }

    let mut ids: Vec<&str> = Vec::new();

    for line in BufReader::new(file).lines() {
        let url = line.expect("Couldn't read line from Buffered Reader");
        match url_id.get(&url as &str) {
            Some(id) => ids.push(id),
            None => println!("Url {} did not match", &url),
        }
    }

    auth.mark_as_read(&ids);
}
