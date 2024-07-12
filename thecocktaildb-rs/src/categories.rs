use derive_more::Deref;
use serde::Deserialize;
use tracing::instrument;

use crate::{Client, Error};

#[derive(Debug, Deserialize)]
pub(crate) struct CategoryDto {
    #[serde(rename = "strCategory")]
    alcoholic: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CategoriesDto {
    drinks: Vec<CategoryDto>,
}

#[derive(Debug, Deref)]
pub struct Category(String);

impl From<CategoryDto> for Category {
    fn from(value: CategoryDto) -> Self {
        Self(value.alcoholic)
    }
}

#[derive(Debug, Deref)]
pub struct Categories(Vec<Category>);

impl Categories {
    /// List the categories
    #[instrument]
    pub async fn list(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get(client.base_url.join("list.php?c=list")?.to_string())
            .await?
            .json::<CategoriesDto>()
            .await?
            .into())
    }
}

impl From<CategoriesDto> for Categories {
    fn from(value: CategoriesDto) -> Self {
        Self(value.drinks.into_iter().map(Into::into).collect())
    }
}
