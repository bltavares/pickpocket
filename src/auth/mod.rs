extern crate hyper;
extern crate rustc_serialize;

use self::hyper::header::{Connection, ContentType};
use self::hyper::Url;
use std::io::Read;

const ENDPOINT: &'static str = "https://getpocket.com/v3";
const REDIRECT_URL: &'static str = "https://getpocket.com";

pub fn url(method: &str) -> Url {
    Url::parse(&format!("{}{}", ENDPOINT, method)).unwrap()
}

pub struct Client {
    pub consumer_key: String,
    pub authorization_code: String,
}

pub struct BeginAuthentication {
    pub consumer_key: String,
}

pub struct AuthorizationRequest {
    consumer_key: String,
    request_code: String,
}

impl BeginAuthentication {
    pub fn request_authorization_code(self) -> AuthorizationRequest {
        let body = self.request();
        let code = body.split("=").skip(1).next().unwrap();

        AuthorizationRequest {
            consumer_key: self.consumer_key,
            request_code: code.to_string(),
        }
    }

    fn request(&self) -> String {
        let client = hyper::Client::new();

        let method = url("/oauth/request");
        let mut res = client.post(method)
                            .body(&format!("consumer_key={}&redirect_uri={}",
                                           &self.consumer_key,
                                           REDIRECT_URL))
                            .header(ContentType::form_url_encoded())
                            .header(Connection::close())
                            .send()
                            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }
}

impl AuthorizationRequest {
    pub fn authorization_url(&self) -> String {
        format!("https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
                &self.request_code,
                REDIRECT_URL)
    }

    pub fn request_authorized_code(self) -> Client {
        let body = self.request();
        let first_value = body.split("=").skip(1).next().unwrap();
        let code = first_value.split("&").next().unwrap().to_string();

        Client {
            consumer_key: self.consumer_key,
            authorization_code: code,
        }
    }

    fn request(&self) -> String {
        let client = hyper::Client::new();

        let method = url("/oauth/authorize");
        let mut res = client.post(method)
                            .body(&format!("consumer_key={}&code={}",
                                           &self.consumer_key,
                                           &self.request_code))
                            .header(ContentType::form_url_encoded())
                            .header(Connection::close())
                            .send()
                            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }
}
