extern crate pickpocket;

use std::collections::BTreeSet;

use pickpocket::batch::BatchApp;
use pickpocket::Status;

fn main() {
    let app = BatchApp::default();

    let mut ids: BTreeSet<&str> = BTreeSet::new();

    let cache_reading_list = app.cache_client.list_all();

    for line in app.file_lines() {
        let url = line.expect("Could not read line");
        match app.get(&url as &str) {
            Some(id) => {
                let item = cache_reading_list.get(id).expect("cant locate id");
                if item.status() == Status::Unread {
                    ids.insert(id);
                } else {
                    println!("Url {} already marked as read", url);
                }
            }
            None => println!("Url {} did not match", &url),
        }
    }

    app.client.mark_as_read(ids);
}
