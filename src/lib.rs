extern crate hyper;
extern crate rustc_serialize;
extern crate chrono;

use self::hyper::header::{Connection, ContentType};
use self::hyper::Url;
use self::rustc_serialize::json;
use self::rustc_serialize::Decoder;
use std::collections::BTreeMap;
use std::io::Read;

mod auth;
pub mod cli;
pub use auth::*;

#[derive(RustcDecodable)]
pub struct Item {
    given_url: String,
    resolved_url: Option<String>,
    favorite: String,
    status: String,
}

#[derive(RustcDecodable)]
pub struct ReadingListResponse {
    pub list: BTreeMap<String, Item>,
}

enum Action {
    Archive,
    Favorite,
    Add,
}

#[derive(PartialEq)]
pub enum FavoriteStatus {
    Favorited,
    NotFavorited
}

#[derive(PartialEq)]
pub enum Status {
    Read,
    Unread
}

impl Item {
    pub fn url(&self) -> &str {
        &self.resolved_url.as_ref().unwrap_or(&self.given_url)
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
    pub fn mark_as_read<'a, T>(&self, ids: T)
        where T: IntoIterator<Item = &'a str>
    {
        self.modify(Action::Archive, ids);
    }

    pub fn mark_as_favorite<'a, T>(&self, ids: T)
        where T: IntoIterator<Item = &'a str>
    {
        self.modify(Action::Favorite, ids);
    }

    pub fn add_urls<'a, T>(&self, urls: T)
        where T: IntoIterator<Item = &'a str>
    {
        self.modify(Action::Add, urls);
    }

    pub fn list_all(&self) -> ReadingListResponse {
        let method = url("/get");
        let payload = format!(r##"{{ "consumer_key":"{}",
                               "access_token":"{}",
                               "state":"all",
                               "detailType":"simple"
                               }}"##,
                              &self.consumer_key,
                              &self.authorization_code);

        let response = self.request(method, payload);
        json::decode(&response).expect("Couldn't parse /get response")
    }

    fn modify<'a, T>(&self, action: Action, ids: T)
        where T: IntoIterator<Item = &'a str>
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
        let time = chrono::UTC::now().timestamp();
        let actions: Vec<String> = ids.iter()
                                      .map(|id| {
                                          format!(r##"{{ "action": "{}", "{}": "{}", "time": "{}" }}"##,
                                                  action_verb,
                                                  item_key,
                                                  id,
                                                  time)
                                      })
                                      .collect();
        let payload = format!(r##"{{ "consumer_key":"{}",
                               "access_token":"{}",
                               "actions": [{}]
                               }}"##,
                              &self.consumer_key,
                              &self.authorization_code,
                              actions.join(", "));

        self.request(method, payload);
    }


    fn request(&self, method: Url, payload: String) -> String {
        let client = hyper::Client::new();

        let mut res = client.post(method)
                            .body(&payload)
                            .header(ContentType::json())
                            .header(Connection::close())
                            .send()
                            .expect(&format!("Coulnd't make request with payload: {}", &payload));

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }
}
