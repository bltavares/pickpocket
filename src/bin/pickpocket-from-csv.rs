extern crate csv;
extern crate pickpocket;

use std::env;
use std::collections::BTreeSet;

use pickpocket::batch::BatchApp;
use pickpocket::Status;

fn main() {
    let app = BatchApp::default();

    let csv_file_name = env::args()
        .nth(1)
        .expect("Expected an csv file as argument");

    let csv_reader = csv::Reader::from_path(csv_file_name);

    let mut read_ids: BTreeSet<&str> = BTreeSet::new();
    let mut missing_urls: BTreeSet<String> = BTreeSet::new();

    let cache_reading_list = app.cache_client.list_all();

    for item in csv_reader.expect("couldnt read csv").records() {
        let item = item.expect("coudltn read line");

        let url = item.get(0).unwrap();
        let folder = item.get(3).unwrap();

        match app.get(&url) {
            None => {
                missing_urls.insert(url.into());
            }
            Some(id) => {
                let pocket_item = cache_reading_list.get(id).expect("cant locate id");
                if pocket_item.status() == Status::Unread
                    && (folder == "Archive" || folder == "Done")
                {
                    read_ids.insert(id);
                }
            }
        }
    }

    println!("Missing: {}", missing_urls.len());
    println!("Marking as read: {}", read_ids.len());

    for url in missing_urls {
        println!("{}", url);
    }

    app.client.mark_as_read(read_ids);
}
