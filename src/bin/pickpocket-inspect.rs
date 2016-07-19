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

    for (_, reading_item) in reading_list {
        println!("{title} | {url}",
                 url = pickpocket::cleanup_url(reading_item.url()),
                 title = reading_item.title());
    }
}
