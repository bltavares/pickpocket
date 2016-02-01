extern crate hyper;
extern crate rustc_serialize;
extern crate pickpocket;

use pickpocket::{Status, FavoriteStatus};

fn main() {
    let client = match pickpocket::cli::client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let reading_list = client.list_all();
    let mut favorites : Vec<&str> = vec!();
    let mut read : Vec<&str> = vec!();

    for (id, reading_item) in reading_list.list.iter() {
        if reading_item.favorite() == FavoriteStatus::Favorited {
            favorites.push(id)
        }

        if reading_item.status() == Status::Read {
            read.push(id)
        }
    }

    client.mark_as_favorite(&favorites);
    client.mark_as_read(&read);
}
