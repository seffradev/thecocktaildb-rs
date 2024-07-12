use tracing::instrument;
use url::Url;

use crate::Error;

#[derive(Debug)]
pub struct Client {
    base_url: Url,
}

impl Client {
    #[instrument]
    pub fn new(api_key: &str) -> Result<Self, Error> {
        Ok(Self {
            base_url: Url::parse("https://www.thecocktaildb.com/api/json/v1/")?.join(&(api_key.to_string() + &String::from("/")))?,
        })
    }
}
