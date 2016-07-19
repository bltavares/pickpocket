extern crate bincode;
extern crate flate2;
extern crate hyper;
extern crate rustc_serialize;

extern crate pickpocket;

use std::io::BufWriter;
use std::env;

use bincode::rustc_serialize::encode_into;

use flate2::write::ZlibEncoder;
use flate2::Compression;

fn main() {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("Expected an file as argument");

    let client = match pickpocket::cli::client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let file = std::fs::File::create(&file_name).expect(&format!("Couldn't open {}", &file_name));

    let reading_list = client.list_all();
    let writer = BufWriter::new(file);
    let mut encoder = ZlibEncoder::new(writer, Compression::Best);
    encode_into(&reading_list, &mut encoder, bincode::SizeLimit::Infinite)
        .expect("Failed to encode the content");
}
