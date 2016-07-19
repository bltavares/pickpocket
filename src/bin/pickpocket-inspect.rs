extern crate pickpocket;

use std::env;

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("Expected an file as argument");

    let client = match pickpocket::cli::FileClient::from_cache(&file_name) {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let reading_list = client.list_all();

    for (id, reading_item) in reading_list {
        println!("Id:\t{id}
Reading Item:\t{reading_item:?}
Used url:\t{url}
Cleaned url:\t{clean}
---
",
                 id = id,
                 reading_item = reading_item,
                 url = reading_item.url(),
                 clean = pickpocket::cleanup_url(reading_item.url()));
    }
}
