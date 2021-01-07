use pickpocket::{FavoriteStatus, Status};
use std::collections::BTreeSet;

#[tokio::main]
async fn main() {
    let client = match pickpocket::cli::client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let reading_list = client.list_all().await;
    let mut favorites: BTreeSet<&str> = BTreeSet::new();
    let mut read: BTreeSet<&str> = BTreeSet::new();

    for (id, reading_item) in &reading_list {
        if reading_item.favorite() == FavoriteStatus::Favorited {
            favorites.insert(id);
        }

        if reading_item.status() == Status::Read {
            read.insert(id);
        }
    }

    client.mark_as_favorite(favorites).await;
    client.mark_as_read(read).await;
}
