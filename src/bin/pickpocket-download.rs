extern crate pickpocket;

use std::env;

use pickpocket::cli::*;

fn main() {
    let file_name = env::args()
        .nth(2)
        .expect("Expected an file as argument");

    let client = match client_from_env_vars() {
        Ok(client) => client,
        Err(e) => panic!("It wasn't possible to initialize a Pocket client\n{}", e),
    };

    let cli_client = FileClient::from_online(client.list_all());
    cli_client.write_cache(&file_name).expect("Could not write to cache");
}
