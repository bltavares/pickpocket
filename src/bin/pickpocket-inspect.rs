extern crate bincode;
extern crate flate2;
extern crate hyper;
extern crate rustc_serialize;

extern crate pickpocket;

use std::io::BufReader;
use std::env;

use bincode::rustc_serialize::decode_from;

use flate2::read::ZlibDecoder;

use pickpocket::ReadingList;

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("Expected an file as argument");

    let file = std::fs::File::open(&file_name).expect(&format!("Couldn't open {}", &file_name));

    let reader = BufReader::new(file);
    let mut decoder = ZlibDecoder::new(reader);
    let reading_list: ReadingList = decode_from(&mut decoder, bincode::SizeLimit::Infinite)
        .expect("Could not read content from file");

    for (id, reading_item) in &reading_list {
        println!("Id:\t{id}
Reading Item:\t{reading_item:?}
Used url:\t{url}
Cleaned url:\t{clean}
---
",
                 id = id,
                 reading_item = reading_item,
                 url = reading_item.url(),
                 clean = pickpocket::cleanup_url(reading_item.url()));
    }
}
