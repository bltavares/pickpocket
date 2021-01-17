use hyper::{body, header, Body, Method, Request, Uri};
use hyper_rustls::HttpsConnector;

const ENDPOINT: &str = "https://getpocket.com/v3";
const REDIRECT_URL: &str = "https://getpocket.com";

pub fn url(method: &str) -> Uri {
    let url = format!("{}{}", ENDPOINT, method);
    url.parse()
        .unwrap_or_else(|_| panic!("Could not parse url: {}", url))
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

pub fn https_client() -> hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
    let https = HttpsConnector::with_native_roots();
    hyper::Client::builder().build::<_, hyper::Body>(https)
}

impl BeginAuthentication {
    pub async fn request_authorization_code(self) -> AuthorizationRequest {
        let body = self.request().await;
        let code = body
            .split('=')
            .nth(1)
            .expect("Could not retrieve the authorization code from the authentication request");

        AuthorizationRequest {
            consumer_key: self.consumer_key,
            request_code: code.to_owned(),
        }
    }

    async fn request(&self) -> String {
        let client = https_client();

        let req = Request::builder()
            .method(Method::POST)
            .uri(url("/oauth/request"))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::CONNECTION, "close")
            .body(Body::from(format!(
                "consumer_key={}&redirect_uri={}",
                &self.consumer_key, REDIRECT_URL
            )))
            .unwrap();

        let res = client
            .request(req)
            .await
            .expect("Could not request OAuth authorization");

        let body_bytes = body::to_bytes(res.into_body())
            .await
            .expect("Could not read OAuth response body");

        String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8")
    }
}

impl AuthorizationRequest {
    pub fn authorization_url(&self) -> String {
        format!(
            "https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
            &self.request_code, REDIRECT_URL
        )
    }

    pub async fn request_authorized_code(self) -> Client {
        let body = self.request().await;
        let first_value = body
            .split('=')
            .nth(1)
            .expect("Could not extract authorization line from response");
        let code = first_value
            .split('&')
            .next()
            .expect("Could not extract authorization code from response")
            .to_owned();

        Client {
            consumer_key: self.consumer_key,
            authorization_code: code,
        }
    }

    async fn request(&self) -> String {
        let client = https_client();

        let req = Request::builder()
            .method(Method::POST)
            .uri(url("/oauth/authorize"))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::CONNECTION, "close")
            .body(Body::from(format!(
                "consumer_key={}&code={}",
                &self.consumer_key, &self.request_code
            )))
            .unwrap();

        let res = client
            .request(req)
            .await
            .expect("Could not make authorization request");

        let body_bytes = body::to_bytes(res.into_body())
            .await
            .expect("Could not read authorization response body");

        String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8")
    }
}
