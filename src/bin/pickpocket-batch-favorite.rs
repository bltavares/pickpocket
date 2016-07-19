extern crate pickpocket;

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use pickpocket::cli::*;

fn main() {
    let readlist_file_name = env::args()
        .skip(1)
        .next()
        .expect("Expected an reading list file as argument");

    let cache_file_name = env::args()
        .skip(2)
        .next()
        .expect("Expected an cache as argument");

    let readlist_file = File::open(&readlist_file_name)
        .expect(&format!("Couldn't open {}", &readlist_file_name));

    let client = match client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let cache_client = match FileClient::from_cache(&cache_file_name) {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket cache\n{}", e),
    };

    let reading_list = cache_client.list_all();

    let mut url_id: BTreeMap<&str, &str> = BTreeMap::new();
    let mut cleanurl_id: BTreeMap<String, &str> = BTreeMap::new();
    for (id, reading_item) in reading_list {
        url_id.insert(reading_item.url(), id);
        cleanurl_id.insert(pickpocket::cleanup_url(reading_item.url()), id);
    }

    let mut ids: BTreeSet<&str> = BTreeSet::new();

    for line in BufReader::new(readlist_file).lines() {
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
                    None => println!("Url {} did not match", &url),
                }
            }
        }
    }

    client.mark_as_favorite(ids);
}
