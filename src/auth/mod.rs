use hyper;
use hyper::header::{Connection, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Url;
use std::io::Read;

const ENDPOINT: &'static str = "https://getpocket.com/v3";
const REDIRECT_URL: &'static str = "https://getpocket.com";

pub fn url(method: &str) -> Url {
    let url = format!("{}{}", ENDPOINT, method);
    Url::parse(&url).expect(&format!("Could not parse url: {}", url))
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

pub fn https_client() -> hyper::Client {
    let ssl = NativeTlsClient::new().expect("Could not aquire a Native TLS connector");
    let connector = HttpsConnector::new(ssl);
    hyper::Client::with_connector(connector)
}

impl BeginAuthentication {
    pub fn request_authorization_code(self) -> AuthorizationRequest {
        let body = self.request();
        let code = body.split('=')
            .skip(1)
            .next()
            .expect("Could not retrieve the authorization code from the authentication request");

        AuthorizationRequest {
            consumer_key: self.consumer_key,
            request_code: code.to_owned(),
        }
    }

    fn request(&self) -> String {
        let client = https_client();

        let method = url("/oauth/request");
        let mut res = client.post(method)
            .body(&format!("consumer_key={}&redirect_uri={}",
                           &self.consumer_key,
                           REDIRECT_URL))
            .header(ContentType::form_url_encoded())
            .header(Connection::close())
            .send()
            .expect("Could not request OAuth authorization");

        let mut body = String::new();
        res.read_to_string(&mut body).expect("Could not read OAuth request body");
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
        let first_value = body.split('=')
            .skip(1)
            .next()
            .expect("Could not extract authorization line from response");
        let code = first_value.split('&')
            .next()
            .expect("Could not extract authorization code from response")
            .to_owned();

        Client {
            consumer_key: self.consumer_key,
            authorization_code: code,
        }
    }

    fn request(&self) -> String {
        let client = https_client();

        let method = url("/oauth/authorize");
        let mut res = client.post(method)
            .body(&format!("consumer_key={}&code={}",
                           &self.consumer_key,
                           &self.request_code))
            .header(ContentType::form_url_encoded())
            .header(Connection::close())
            .send()
            .expect("Could not make authorization request");

        let mut body = String::new();
        res.read_to_string(&mut body).expect("Could not read authorization body response");
        body
    }
}
