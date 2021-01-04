extern crate pickpocket;

use std::env;

#[tokio::main]
async fn main() {
    let url = env::args().nth(1).expect("Expected an needle as argument");

    let client = match pickpocket::cli::client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let reading_list = client.list_all().await;
    for (id, reading_item) in &reading_list {
        if reading_item.url().contains(&url) {
            println!(
                "Id:\t{id}
Reading Item:\t{reading_item:?}
Used url:\t{url}
Cleaned url:\t{clean}
",
                id = id,
                reading_item = reading_item,
                url = reading_item.url(),
                clean = pickpocket::cleanup_url(reading_item.url())
            );
        }
    }
}
