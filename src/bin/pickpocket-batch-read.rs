extern crate hyper;
extern crate rustc_serialize;
extern crate pickpocket;

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::io::{BufReader, BufRead};

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("Expected an file as argument");

    let file = std::fs::File::open(&file_name).expect(&format!("Couldn't open {}", &file_name));

    let client = match pickpocket::cli::client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let reading_list = client.list_all();

    let mut url_id: BTreeMap<&str, &str> = BTreeMap::new();
    let mut cleanurl_id: BTreeMap<String, &str> = BTreeMap::new();

    for (id, reading_item) in &reading_list.list {
        url_id.insert(reading_item.url(), id);
        cleanurl_id.insert(pickpocket::cleanup_url(reading_item.url()), id);
    }

    let mut ids: BTreeSet<&str> = BTreeSet::new();

    for line in BufReader::new(file).lines() {
        let url = line.expect("Couldn't read line from Buffered Reader");
        match url_id.get(&url as &str) {
            Some(id) => {
                ids.insert(id);
            }
            None => {
                match cleanurl_id.get(&pickpocket::cleanup_url(&url)) {
                    Some(id) => {
                        ids.insert(id);
                    }
                    None => println!("Url {} did not match", &url)
                }
            }
        }
    }

    client.mark_as_read(ids);
}
