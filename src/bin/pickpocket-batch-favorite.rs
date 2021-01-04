extern crate pickpocket;

use std::collections::BTreeSet;

use pickpocket::batch::BatchApp;
use pickpocket::FavoriteStatus;

#[tokio::main]
async fn main() {
    let app = BatchApp::default();

    let mut ids: BTreeSet<&str> = BTreeSet::new();

    let cache_reading_list = app.cache_client.list_all();

    for line in app.file_lines() {
        let url = line.expect("Could not read line");
        match app.get(&url as &str) {
            Some(id) => {
                let item = cache_reading_list.get(id).expect("cant locate id");
                if item.favorite() == FavoriteStatus::NotFavorited {
                    ids.insert(id);
                } else {
                    println!("Url {} already marked as favorite", url);
                }
                ids.insert(id);
            }
            None => println!("Url {} did not match", &url),
        }
    }

    app.client.mark_as_favorite(ids).await;
}
