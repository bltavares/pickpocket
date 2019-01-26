use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use cleanup_url;
use cli::*;

pub struct BatchApp {
    pub client: Client,
    pub cache_client: FileClient,
    url_id: BTreeMap<String, String>,
    clean_url_id: BTreeMap<String, String>,
    file_name: String,
}

impl Default for BatchApp {
    fn default() -> Self {
        let readlist_file_name = env::args()
            .nth(1)
            .expect("Expected an reading list file as argument");

        let cache_file_name = env::args().nth(2).expect("Expected an cache as argument");

        let client = match client_from_env_vars() {
            Ok(client) => client,
            Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
        };

        let cache_client = match FileClient::from_cache(&cache_file_name) {
            Ok(client) => client,
            Err(e) => panic!("It wasn't possible to initialize a Pocket cache\n{}", e),
        };

        let mut url_id = BTreeMap::new();
        let mut clean_url_id = BTreeMap::new();

        {
            let reading_list = cache_client.list_all();
            for (id, reading_item) in reading_list {
                url_id.insert(reading_item.url().into(), id.clone());
                clean_url_id.insert(cleanup_url(reading_item.url()), id.clone());
            }
        }

        BatchApp {
            url_id,
            clean_url_id,
            client,
            cache_client,
            file_name: readlist_file_name,
        }
    }
}

impl BatchApp {
    pub fn get(&self, url: &str) -> Option<&str> {
        match self.url_id.get(url) {
            Some(x) => Some(x),
            None => match self.clean_url_id.get(&cleanup_url(url)) {
                Some(x) => Some(x),
                None => None,
            },
        }
    }

    pub fn file_lines(&self) -> Lines<BufReader<File>> {
        let file = File::open(&self.file_name)
            .unwrap_or_else(|_| panic!("Couldn't open {}", &self.file_name));

        BufReader::new(file).lines()
    }
}
