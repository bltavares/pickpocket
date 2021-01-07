use std::env;
use std::io;
use std::io::Write;

use pickpocket::{BeginAuthentication, Client};

fn consumer_key() -> String {
    let key = "POCKET_CONSUMER_KEY";
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            print!("Please, type in your consumer key: ");
            io::stdout()
                .flush()
                .expect("Could not write message to terminal");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read consumer key from terminal");
            input
        }
    }
}

#[tokio::main]
async fn main() {
    let authorization_request = BeginAuthentication {
        consumer_key: consumer_key(),
    }
    .request_authorization_code()
    .await;

    println!("Please visit {}", authorization_request.authorization_url());
    println!("Press enter after authorizing with Pocket");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read authorizing code from terminal");

    let auth = authorization_request.request_authorized_code().await;
    print_auth_as_env_variables(&auth);
}

fn print_auth_as_env_variables(auth: &Client) {
    println!(
        "export POCKET_AUTHORIZATION_CODE=\"{}\"",
        &auth.authorization_code
    );
    println!("export POCKET_CONSUMER_KEY=\"{}\"", &auth.consumer_key);
}
