use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

pub struct BatchApp {
    pub client: Client,
    url_id: BTreeMap<String, String>,
    clean_url_id: BTreeMap<String, String>,
    file_name: String,
}

use cleanup_url;
use cli::*;

impl Default for BatchApp {
    fn default() -> Self {
        let readlist_file_name = env::args()
            .skip(1)
            .next()
            .expect("Expected an reading list file as argument");

        let cache_file_name = env::args()
            .skip(2)
            .next()
            .expect("Expected an cache as argument");

        let client = match client_from_env_vars() {
            Ok(client) => client,
            Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
        };

        let cache_client = match FileClient::from_cache(&cache_file_name) {
            Ok(client) => client,
            Err(e) => panic!("It wasn't possible to initialize a Pocket cache\n{}", e),
        };

        let reading_list = cache_client.list_all();

        let mut url_id = BTreeMap::new();
        let mut cleanurl_id = BTreeMap::new();
        for (id, reading_item) in reading_list {
            url_id.insert(reading_item.url().into(), id.clone());
            cleanurl_id.insert(cleanup_url(reading_item.url()), id.clone());
        }

        BatchApp {
            url_id: url_id,
            clean_url_id: cleanurl_id,
            client: client,
            file_name: readlist_file_name,
        }
    }
}

impl BatchApp {
    pub fn get(&self, url: &str) -> Option<&str> {
        match self.url_id.get(url) {
            Some(x) => Some(x),
            None => {
                match self.clean_url_id.get(&cleanup_url(url)) {
                    Some(x) => Some(x),
                    None => None,
                }
            }
        }
    }

    pub fn file_lines(&self) -> Lines<BufReader<File>> {
        let file = File::open(&self.file_name)
            .expect(&format!("Couldn't open {}", &self.file_name));

        BufReader::new(file).lines()
    }
}
