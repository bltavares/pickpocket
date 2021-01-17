use std::io::{BufReader, BufWriter};

use bincode::{deserialize_from, serialize_into};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

pub use crate::auth::*;
use crate::ReadingList;

use std::env;
use std::fs::File;

pub fn client_from_env_vars() -> Result<Client, String> {
    let consumer_env_key = "POCKET_CONSUMER_KEY";
    let consumer_key = env::var(consumer_env_key).map_err(|_| {
        format!(
            "Consumer key should be available on the environment variable {}",
            consumer_env_key
        )
    })?;

    let auth_env_code = "POCKET_AUTHORIZATION_CODE";
    let authorization_code = env::var(auth_env_code).map_err(|_| {
        format!(
            "Authorization code should be available on the environment variable {}",
            auth_env_code
        )
    })?;

    Ok(Client {
        consumer_key,
        authorization_code,
    })
}

pub struct FileClient {
    list: ReadingList,
}

impl FileClient {
    pub fn from_online(list: ReadingList) -> Self {
        FileClient { list }
    }

    pub fn from_cache(file_name: &str) -> Result<Self, String> {
        let file = File::open(&file_name).map_err(|_| format!("Couldn't open {}", &file_name))?;

        let reader = BufReader::new(file);
        let mut decoder = ZlibDecoder::new(reader);

        let parsed = deserialize_from(&mut decoder)
            .map_err(|_| format!("Could not read content from file: {}", &file_name))?;

        Ok(FileClient { list: parsed })
    }

    pub fn list_all(&self) -> &ReadingList {
        &self.list
    }

    pub fn write_cache(&self, file_name: &str) -> Result<(), String> {
        let file = File::create(&file_name).map_err(|_| format!("Couldn't open {}", &file_name))?;

        let writer = BufWriter::new(file);
        let mut encoder = ZlibEncoder::new(writer, Compression::best());
        serialize_into(&mut encoder, &self.list).map_err(|_| "Failed to encode the content".into())
    }
}
