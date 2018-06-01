extern crate pickpocket;
extern crate csv;

use std::env;
use pickpocket::ByUrl;

fn main() {
    let file_name = env::args()
        .nth(1)
        .expect("Expected an pokcet file as argument");

    let client = match pickpocket::cli::FileClient::from_cache(&file_name) {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let csv_file_name = env::args()
        .nth(2)
        .expect("Expected an csv file as argument");

    let csv_reader = csv::Reader::from_path(csv_file_name);

    let reading_list = client.list_all().by_url();

    for item in csv_reader.expect("couldnt read csv").records() {
        let item = item.expect("coudltn read line");

        let url = item.get(0).unwrap();
        let folder = item.get(3).unwrap();

        match reading_list.get(&url as &str) {
            None => {
                println!("{}, {}", url, folder);
            }
            _ => {}
        }
    }
}
