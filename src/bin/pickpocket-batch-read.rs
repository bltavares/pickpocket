extern crate pickpocket;

use std::collections::BTreeSet;

use pickpocket::batch::BatchApp;

fn main() {
    let app = BatchApp::default();

    let mut ids: BTreeSet<&str> = BTreeSet::new();

    for line in app.file_lines() {
        let url = line.expect("Could not read line");
        match app.get(&url as &str) {
            Some(id) => {
                ids.insert(id);
            }
            None => println!("Url {} did not match", &url),
        }
    }

    app.client.mark_as_read(ids);
}
