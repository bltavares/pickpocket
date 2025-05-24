const ENDPOINT: &str = "https://getpocket.com/v3";
const REDIRECT_URL: &str = "https://getpocket.com";

pub fn url(method: &str) -> reqwest::Url {
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

pub fn https_client() -> reqwest::Client {
    reqwest::Client::new()
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
        https_client()
            .post(url("/oauth/request"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Connection", "close")
            .body(format!(
                "consumer_key={}&redirect_uri={}",
                &self.consumer_key, REDIRECT_URL
            ))
            .send()
            .await
            .expect("Could not request OAuth authorization")
            .text()
            .await
            .expect("Response was not valid UTF-8")
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
        https_client()
            .post(url("/oauth/authorize"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Connection", "close")
            .body(format!(
                "consumer_key={}&code={}",
                &self.consumer_key, &self.request_code
            ))
            .send()
            .await
            .expect("Could not make authorization request")
            .text()
            .await
            .expect("Response was not valid UTF-8")
    }
}
