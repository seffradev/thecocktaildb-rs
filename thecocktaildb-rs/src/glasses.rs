use derive_more::Deref;
use serde::Deserialize;
use tracing::instrument;

use crate::{Client, Error};

#[derive(Debug, Deserialize)]
pub(crate) struct GlassDto {
    #[serde(rename = "strGlass")]
    alcoholic: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GlassesDto {
    drinks: Vec<GlassDto>,
}

#[derive(Debug, Deref)]
pub struct Glass(String);

impl Glasses {
    /// List the glasses
    #[instrument]
    pub async fn list(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get(client.base_url.join("list.php?g=list")?.to_string())
            .await?
            .json::<GlassesDto>()
            .await?
            .into())
    }
}

impl From<GlassDto> for Glass {
    fn from(value: GlassDto) -> Self {
        Self(value.alcoholic)
    }
}

#[derive(Debug, Deref)]
pub struct Glasses(Vec<Glass>);

impl From<GlassesDto> for Glasses {
    fn from(value: GlassesDto) -> Self {
        Self(value.drinks.into_iter().map(Into::into).collect())
    }
}
