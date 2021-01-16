use hyper::{body, header, Body, Method, Request, Uri};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};

mod auth;
pub mod batch;
pub mod cli;
pub use auth::*;

const DEFAULT_COUNT: u32 = 5000;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    given_url: String,
    resolved_url: Option<String>,
    given_title: String,
    resolved_title: Option<String>,
    favorite: String,
    status: String,
}

pub type ReadingList = BTreeMap<String, Item>;

#[derive(Deserialize)]
struct ReadingListResponse {
    list: ReadingList,
}

enum ResponseState {
    Parsed(ReadingListResponse),
    NoMore,
    Error(serde_json::Error),
}

enum Action {
    Archive,
    Favorite,
    Add,
}

#[derive(PartialEq)]
pub enum FavoriteStatus {
    Favorited,
    NotFavorited,
}

#[derive(PartialEq)]
pub enum Status {
    Read,
    Unread,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Status::Read => "Read",
                Status::Unread => "Unread",
            }
        )
    }
}

impl Item {
    pub fn url(&self) -> &str {
        self.resolved_url.as_ref().unwrap_or(&self.given_url)
    }

    pub fn title(&self) -> &str {
        let title = self.resolved_title.as_ref().unwrap_or(&self.given_title);
        if title.is_empty() {
            self.url()
        } else {
            title
        }
    }

    pub fn favorite(&self) -> FavoriteStatus {
        if &self.favorite == "1" {
            FavoriteStatus::Favorited
        } else {
            FavoriteStatus::NotFavorited
        }
    }

    pub fn status(&self) -> Status {
        if &self.status == "1" {
            Status::Read
        } else {
            Status::Unread
        }
    }
}

impl Client {
    pub async fn mark_as_read<'a, T>(&self, ids: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.modify(Action::Archive, ids).await;
    }

    pub async fn mark_as_favorite<'a, T>(&self, ids: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.modify(Action::Favorite, ids).await;
    }

    pub async fn add_urls<'a, T>(&self, urls: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.modify(Action::Add, urls).await;
    }

    pub async fn list_all(&self) -> ReadingList {
        let mut reading_list: ReadingList = Default::default();

        let mut offset = 0;

        loop {
            let method = url("/get");
            let payload = format!(
                r##"{{ "consumer_key":"{}",
                               "access_token":"{}",
                               "sort":"site",
                               "state":"all",
                               "detailType":"simple",
                               "count":"{}",
                               "offset":"{}"
                               }}"##,
                &self.consumer_key,
                &self.authorization_code,
                DEFAULT_COUNT,
                (offset * DEFAULT_COUNT)
            );

            let response = self.request(method, payload).await;
            match parse_all_response(&response) {
                ResponseState::NoMore => break,
                ResponseState::Parsed(parsed_response) => {
                    offset += 1;
                    reading_list.extend(parsed_response.list.into_iter())
                }
                ResponseState::Error(e) => panic!("Failed to parse the payload: {:?}", e),
            }
        }

        reading_list
    }

    async fn modify<'a, T>(&self, action: Action, ids: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        let method = url("/send");
        let action_verb = match action {
            Action::Favorite => "favorite",
            Action::Archive => "archive",
            Action::Add => "add",
        };
        let item_key = match action {
            Action::Add => "url",
            _ => "item_id",
        };
        let time = chrono::Utc::now().timestamp();
        let actions: Vec<String> = ids
            .into_iter()
            .map(|id| {
                format!(
                    r##"{{ "action": "{}", "{}": "{}", "time": "{}" }}"##,
                    action_verb, item_key, id, time
                )
            })
            .collect();
        let payload = format!(
            r##"{{ "consumer_key":"{}",
                               "access_token":"{}",
                               "actions": [{}]
                               }}"##,
            &self.consumer_key,
            &self.authorization_code,
            actions.join(", ")
        );

        self.request(method, payload).await;
    }

    async fn request(&self, uri: Uri, payload: String) -> String {
        let client = auth::https_client();

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONNECTION, "close")
            .body(Body::from(payload.clone()))
            .unwrap();

        let res = client
            .request(req)
            .await
            .unwrap_or_else(|_| panic!("Could not make request with payload: {}", &payload));

        let body_bytes = body::to_bytes(res.into_body())
            .await
            .expect("Could not read the HTTP request's body");

        String::from_utf8(body_bytes.to_vec()).expect("Response was not valid UTF-8")
    }
}

fn parse_all_response(response: &str) -> ResponseState {
    match serde_json::from_str::<ReadingListResponse>(response) {
        Ok(r) => ResponseState::Parsed(r),
        Err(e) => {
            if e.is_data() {
                ResponseState::NoMore
            } else {
                ResponseState::Error(e)
            }
        }
    }
}

fn fixup_blogspot(url: &str) -> String {
    let split: Vec<_> = url.split(".blogspot.").collect();
    if split.len() == 2 {
        format!("{}.blogspot.com", split[0])
    } else {
        url.into()
    }
}

fn start_domain_from(url: &str) -> usize {
    if url.starts_with("www.") {
        4
    } else {
        0
    }
}

fn cleanup_path(path: &str) -> &str {
    path.trim_end_matches("index.html")
        .trim_end_matches("index.php")
        .trim_end_matches('/')
}

pub fn cleanup_url(url: &str) -> String {
    let parsed: Uri = url.parse().expect("Could not parse cleanup url");
    let current_host = parsed.host().expect("Cleaned up an url without a host");
    let starts_from = start_domain_from(current_host);

    format!(
        "https://{}{}",
        fixup_blogspot(&current_host[starts_from..]),
        cleanup_path(parsed.path())
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clean_url_hash() {
        let url_ = "http://example.com#asdfas.fsa";
        assert_eq!(cleanup_url(url_), "https://example.com");
    }

    #[test]
    fn test_clean_url_query() {
        let url_ = "http://example.com?";
        assert_eq!(cleanup_url(url_), "https://example.com");
    }

    #[test]
    fn test_clean_url_keep_same_url() {
        let url_ = "http://another.example.com";
        assert_eq!(cleanup_url(url_), "https://another.example.com");
    }

    #[test]
    fn test_clean_url_keep_https() {
        let url = "https://another.example.com";
        assert_eq!(cleanup_url(url), "https://another.example.com");
    }

    #[test]
    fn test_cleanup_blogspot_first_tld() {
        let url = "https://this-is-a.blogspot.cl/asdf/asdf/asdf?asdf=1";
        assert_eq!(
            cleanup_url(url),
            "https://this-is-a.blogspot.com/asdf/asdf/asdf"
        );
    }

    #[test]
    fn test_cleanup_blogspot_second_tld() {
        let url = "https://this-is-a.blogspot.com.br/asdf/asdf/asdf?asdf=1";
        assert_eq!(
            cleanup_url(url),
            "https://this-is-a.blogspot.com/asdf/asdf/asdf"
        );
    }

    #[test]
    fn test_cleanup_www() {
        let url = "https://www.this-is-a.blogspot.com.br/asdf/asdf/asdf?asdf=1";
        assert_eq!(
            cleanup_url(url),
            "https://this-is-a.blogspot.com/asdf/asdf/asdf"
        );
    }

    #[test]
    fn test_cleanup_https_redirection() {
        let url = "http://www.this-is-a.blogspot.com.br/asdf/asdf/asdf?asdf=2";
        assert_eq!(
            cleanup_url(url),
            "https://this-is-a.blogspot.com/asdf/asdf/asdf"
        );
    }

    #[test]
    fn test_cleanup_urls_are_the_same() {
        let url1 = cleanup_url("https://example.com/hello");
        let url2 = cleanup_url("https://example.com/hello/");
        assert_eq!(url1, url2);
    }

    #[test]
    fn test_cleanup_urls_without_index() {
        let url = "https://example.com/index.php";
        assert_eq!(cleanup_url(url), "https://example.com");
    }

    #[test]
    fn test_cleanup_urls_without_index_html() {
        let url = "https://example.com/index.html";
        assert_eq!(cleanup_url(url), cleanup_url("https://example.com/"));
    }

    #[test]
    fn test_dot_on_files() {
        assert_eq!(
            cleanup_url("https://jenkins.io/2.0/index.html"),
            cleanup_url("https://jenkins.io/2.0/")
        );
    }
}

#[test]
fn test_decoding_empty_object_list() {
    let response = r#"{ "list": {}}"#;
    match parse_all_response(&response) {
        ResponseState::Parsed(_) => (),
        _ => panic!("This should have been parsed"),
    }
}

#[test]
fn test_decoding_empty_pocket_list() {
    let response = r#"{ "list": []}"#;
    match parse_all_response(&response) {
        ResponseState::Parsed(_) => (),
        _ => panic!("This should signal an empty list"),
    }
}

#[test]
fn test_decoding_error() {
    let response = r#"{ "list": "#;
    match parse_all_response(&response) {
        ResponseState::Error(_) => (),
        _ => panic!("This should fail to parse"),
    }
}
