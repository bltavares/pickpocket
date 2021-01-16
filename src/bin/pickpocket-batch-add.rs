use std::collections::BTreeSet;

use pickpocket::batch::BatchApp;

#[tokio::main]
async fn main() {
    let app = BatchApp::default();

    let mut urls: BTreeSet<String> = BTreeSet::new();

    for line in app.file_lines() {
        let url = line.expect("Could not read line");
        match app.get(&url as &str) {
            None => {
                urls.insert(url);
            }
            Some(_) => println!("Url {} already there. Not adding.", &url),
        }
    }

    app.client.add_urls(urls.iter().map(AsRef::as_ref)).await;
}
