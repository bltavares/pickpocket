extern crate hyper;

use std::io::{Read, Write};
use std::io;
use std::env;

use hyper::{Client, Url};
use hyper::header::{Connection, ContentType};

const ENDPOINT: &'static str = "https://getpocket.com/v3";
const REDIRECT_URL: &'static str = "https://getpocket.com";

fn url(method: &str) -> Url {
    Url::parse(&format!("{}{}", ENDPOINT, method)).unwrap()
}

fn begin_auth(consumer_key: &str) -> String {
    let client = Client::new();

    let method = url("/oauth/request");
    let mut res = client.post(method)
                        .body(&format!("consumer_key={}&redirect_uri={}",
                                       consumer_key,
                                       REDIRECT_URL))
                        .header(ContentType::form_url_encoded())
                        .header(Connection::close())
                        .send()
                        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let code = body.split("=").skip(1).next().unwrap();
    code.to_string()
}

fn finish_auth(consumer_key: &str, code: &str) -> String {
    let client = Client::new();

    let method = url("/oauth/authorize");
    let mut res = client.post(method)
                        .body(&format!("consumer_key={}&code={}", consumer_key, code))
                        .header(ContentType::form_url_encoded())
                        .header(Connection::close())
                        .send()
                        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let code = body.split("=").skip(1).next().unwrap();
    code.split("&").next().unwrap().to_string()
}


fn main() {
    let key = "POCKET_CONSUMER_KEY";
    let consumer_key: String = match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            print!("Please, type in your consumer key: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input
        }
    };

    let code = begin_auth(&consumer_key);
    println!("Please visit https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
             code,
             REDIRECT_URL);
    print!("Press enter after authorizing with Pocket");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let authorized_code = finish_auth(&consumer_key, &code);
    println!("export POCKET_AUTHORIZATION_CODE=\"{}\"", authorized_code);
}
