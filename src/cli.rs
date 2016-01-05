pub use auth::*;
use std::env;

pub fn client_from_env_vars() -> Result<Client, String> {
    let consumer_env_key = "POCKET_CONSUMER_KEY";
    let consumer_key = try!(env::var(consumer_env_key).map_err(|_| {
        format!("Consumer key should be available on the environment variable {}",
                consumer_env_key)
    }));

    let auth_env_code = "POCKET_AUTHORIZATION_CODE";
    let authorization_code = try!(env::var(auth_env_code).map_err(|_| {
        format!("Authorization code should be available on the environment variable {}",
                auth_env_code)
    }));

    Ok(Client {
        consumer_key: consumer_key,
        authorization_code: authorization_code,
    })
}
