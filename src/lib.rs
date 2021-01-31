use hyper::{body, header, Body, Method, Request, Uri};
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use serde::ser::Serializer;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};

mod auth;
pub mod batch;
pub mod cli;
pub use auth::*;

const DEFAULT_COUNT: u32 = 5000;

/// A Pocket item.
/// The official API docs state that all members are optional. However, empirically it seems safe
/// to assume that the ones that are not `Option`s are always present.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    /// A unique identifier matching the saved item. This id must be used to perform any actions
    /// through the v3/modify endpoint.
    pub item_id: String,

    /// A unique identifier similar to the item_id but is unique to the actual url of the saved
    /// item. The resolved_id identifies unique urls. For example a direct link to a New York Times
    /// article and a link that redirects (ex a shortened bit.ly url) to the same article will
    /// share the same resolved_id. If this value is 0, it means that Pocket has not processed the
    /// item. Normally this happens within seconds but is possible you may request the item before
    /// it has been resolved.
    pub resolved_id: String,

    /// The actual url that was saved with the item. This url should be used if the user wants to
    /// view the item.
    pub given_url: String,

    /// The final url of the item. For example if the item was a shortened bit.ly link, this will
    /// be the actual article the url linked to.
    pub resolved_url: String,

    /// The title that was saved along with the item.
    pub given_title: String,

    /// The title that Pocket found for the item when it was parsed.
    pub resolved_title: String,

    /// Whether the item is favorited or not.
    pub favorite: FavoriteStatus,

    /// Whether the item is unread or read (i.e. in the "Archive").
    pub status: Status,

    /// The first few lines of the item (articles only).
    pub excerpt: String,

    /// Whether the item is an article or not.
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    #[serde(serialize_with = "serialize_bool_to_string")]
    pub is_article: bool,

    /// Whether the item has/is an image.
    pub has_image: HasImage,

    /// Whether the item has/is a video.
    pub has_video: HasVideo,

    /// How many words are in the article.
    #[serde_as(as = "DisplayFromStr")]
    pub word_count: u64,

    // The following are not documented in the official API docs, but they are present in the
    // responses. The ones marked as Option are *sometimes* present in the responses. Use at your
    // own risk.
    /// UNIX timestamp when the item was added.
    #[serde_as(as = "DisplayFromStr")]
    pub time_added: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub time_updated: u64,

    /// UNIX timestamp when the item was read (i.e. moved to the "Archive"). Set to 0 if the item
    /// has not been read.
    #[serde_as(as = "DisplayFromStr")]
    pub time_read: u64,

    /// UNIX timestamp when the item was favorited. Set to 0 if the item has not been favorited.
    #[serde_as(as = "DisplayFromStr")]
    pub time_favorited: u64,

    pub sort_id: u32,

    #[serde(deserialize_with = "deserialize_string_to_bool")]
    #[serde(serialize_with = "serialize_bool_to_string")]
    pub is_index: bool,

    /// Language code. This is sometimes set to an empty string.
    pub lang: String,

    pub top_image_url: Option<String>,
    pub domain_metadata: Option<DomainMetadata>,
    pub listen_duration_estimate: u64,
    pub time_to_read: Option<u64>,
    pub amp_url: Option<String>,

    // The following fields are documented in the official API docs and only present when
    // detailType=complete.
    pub images: Option<BTreeMap<String, Image>>,
    pub videos: Option<BTreeMap<String, Video>>,
    pub authors: Option<BTreeMap<String, Author>>,
    pub tags: Option<BTreeMap<String, Tag>>,

    // The following are not documented in the official API docs, but they are present in responses
    // when detailType=complete.
    pub image: Option<MainImage>,
}

fn deserialize_string_to_bool<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(de::Error::invalid_value(Unexpected::Str(other), &"0 or 1")),
    }
}

fn serialize_bool_to_string<S>(b: &bool, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(if *b { "1" } else { "0" })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainMetadata {
    pub name: Option<String>,
    pub logo: String,
    pub greyscale_logo: String,
}

/// The main image associated with an `Item`.
/// Same as an `Image`, except the `image_id`, `credit`, and `caption` fields are not present.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct MainImage {
    /// The `Item`'s `item_id` this image is associated with.
    pub item_id: String,

    /// A URL where the image is found.
    pub src: String,

    /// Image width.
    #[serde_as(as = "DisplayFromStr")]
    pub width: u32,

    /// Image height.
    #[serde_as(as = "DisplayFromStr")]
    pub height: u32,
}

/// An image associated with an `Item`.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    /// The `Item`'s `item_id` this image is associated with.
    pub item_id: String,

    /// An id for the image. An incremental integer.
    pub image_id: String,

    /// A URL where the image is found.
    pub src: String,

    /// Image width. Caution: often set to zero.
    #[serde_as(as = "DisplayFromStr")]
    pub width: u32,

    /// Image height. Caution: often set to zero.
    #[serde_as(as = "DisplayFromStr")]
    pub height: u32,

    /// Image attribution. Caution: often set to an empty string.
    // TODO This field is set to an empty string instead of removed from the response. Change the
    // model to have it be of type Option<String>.
    pub credit: String,

    /// Image caption. Caution: often set to an empty string.
    // TODO This field is set to an empty string instead of removed from the response. Change the
    // model to have it be of type Option<String>.
    pub caption: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    /// The `Item`'s `item_id` this video is associated with.
    pub item_id: String,

    /// An id for the video. An incremental integer.
    pub video_id: String,

    /// A URL where the video is found.
    pub src: String,

    /// Image width. Caution: often set to zero.
    #[serde_as(as = "DisplayFromStr")]
    pub width: u32,

    /// Image height. Caution: often set to zero.
    #[serde_as(as = "DisplayFromStr")]
    pub height: u32,

    // TODO What is this? It seems to be set to 1, 2, 4, 5 or 7.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "type")]
    video_type: u32,

    /// Seems to be set to YouTube/Vimeo video id. Caution: often set to an empty string.
    // TODO This field is set to an empty string instead of removed from the response. Change the
    // model to have it be of type Option<String>.
    pub vid: String,

    /// Video length in seconds. Caution: often set to Some(0).
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub length: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    /// The `Item`'s `item_id` this author is associated with.
    pub item_id: String,

    /// An id for the author.
    pub author_id: String,

    /// Author's name.
    pub name: String,

    /// Author's URL. This may be the author's profile page in blogging platforms like e.g. Medium
    /// or social networks like Facebook/Google+. Caution: can be an empty string.
    // TODO This field is set to an empty string instead of removed from the response. Change the
    // model to have it be of type Option<String>.
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    /// The `Item`'s `item_id` this tag is applied to.
    pub item_id: String,

    /// Tag name.
    pub tag: String,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum FavoriteStatus {
    #[serde(rename = "0")]
    NotFavorited,
    #[serde(rename = "1")]
    Favorited,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Status {
    #[serde(rename = "0")]
    Unread,
    #[serde(rename = "1")]
    Read,
    #[serde(rename = "2")]
    ShouldBeDeleted,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum HasImage {
    #[serde(rename = "0")]
    No,
    #[serde(rename = "1")]
    Yes,
    #[serde(rename = "2")]
    IsImage,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum HasVideo {
    #[serde(rename = "0")]
    No,
    #[serde(rename = "1")]
    Yes,
    #[serde(rename = "2")]
    IsVideo,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Status::Read => "Read",
                Status::Unread => "Unread",
                Status::ShouldBeDeleted => "ShouldBeDeleted",
            }
        )
    }
}

impl Item {
    pub fn url(&self) -> &str {
        if !self.resolved_url.is_empty() {
            &self.resolved_url
        } else {
            &self.given_url
        }
    }

    pub fn title(&self) -> &str {
        if self.resolved_title.is_empty() {
            if self.given_title.is_empty() {
                self.url()
            } else {
                &self.given_title
            }
        } else {
            &self.resolved_title
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
    if let Ok(parsed) = url.parse::<Uri>() {
        let current_host = parsed.host().expect("Cleaned up an url without a host");
        let starts_from = start_domain_from(current_host);

        format!(
            "https://{}{}",
            fixup_blogspot(&current_host[starts_from..]),
            cleanup_path(parsed.path())
        )
    } else {
        url.into()
    }
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
        ResponseState::NoMore => (),
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
