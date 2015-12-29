extern crate pickpocket;

use std::env;
use std::io::Write;
use std::io;

use pickpocket::{BeginAuthentication, Auth};

fn consumer_key() -> String {
    let key = "POCKET_CONSUMER_KEY";
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            print!("Please, type in your consumer key: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input
        }
    }
}

fn main() {
    let authorization_request = BeginAuthentication { consumer_key: consumer_key() }
                                    .request_authorization_code();

    println!("Please visit {}", authorization_request.authorization_url());
    println!("Press enter after authorizing with Pocket");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let auth = authorization_request.request_authorized_code();
    print_auth_as_env_variables(&auth);
}

fn print_auth_as_env_variables(auth: &Auth) {
    println!("export POCKET_AUTHORIZATION_CODE=\"{}\"",
             &auth.authorization_code);
    println!("export POCKET_CONSUMER_KEY=\"{}\"", &auth.consumer_key);
}
