use derive_more::Deref;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{Client, Error};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AlcoholicDto {
    #[serde(rename = "strAlcoholic")]
    alcoholic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AlcoholicsDto {
    drinks: Vec<AlcoholicDto>,
}

#[derive(Debug, Deref)]
pub struct Alcoholic(String);

impl Alcoholic {
    /// List the alcoholics
    #[instrument]
    pub async fn list(client: &Client) -> Result<Alcoholics, Error> {
        Ok(reqwest::get(client.base_url.join("list.php?a=list")?.to_string())
            .await?
            .json::<AlcoholicsDto>()
            .await?
            .into())
    }
}

impl From<AlcoholicDto> for Alcoholic {
    fn from(value: AlcoholicDto) -> Self {
        Self(value.alcoholic)
    }
}

#[derive(Debug, Deref)]
pub struct Alcoholics(Vec<Alcoholic>);

impl From<AlcoholicsDto> for Alcoholics {
    fn from(value: AlcoholicsDto) -> Self {
        Self(value.drinks.into_iter().map(Into::into).collect())
    }
}