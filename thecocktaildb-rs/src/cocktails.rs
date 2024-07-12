use derive_more::Deref;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    ingredients::{Ingredients, MeasureDto, StaticIngredientsDto},
    Client, Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Instructions {
    /// English translation
    #[serde(rename = "strInstructions")]
    pub en: Option<String>,
    /// Spanish translation
    #[serde(rename = "strInstructionsES")]
    pub es: Option<String>,
    /// German translation
    #[serde(rename = "strInstructionsDE")]
    pub de: Option<String>,
    /// French translation
    #[serde(rename = "strInstructionsFR")]
    pub fr: Option<String>,
    /// Italian translation
    #[serde(rename = "strInstructionsIT")]
    pub it: Option<String>,
    /// Simplified Chinese translation
    #[serde(rename = "strInstructionsZH-HANS")]
    pub zh_hans: Option<String>,
    /// Traditional Chinese translation
    #[serde(rename = "strInstructionsZH-HANT")]
    pub zh_hant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CocktailsDto {
    drinks: Vec<CocktailDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CocktailDto {
    id_drink: Option<String>,
    #[serde(rename = "strDrink")]
    drink: Option<String>,
    #[serde(rename = "strDrinkAlternate")]
    drink_alternate: Option<String>,
    #[serde(rename = "strTags")]
    tags: Option<String>,
    #[serde(rename = "strVideo")]
    video: Option<String>,
    #[serde(rename = "strCategory")]
    category: Option<String>,
    #[serde(rename = "strIBA")]
    iba: Option<String>,
    #[serde(rename = "strAlcoholic")]
    alcoholic: Option<String>,
    #[serde(rename = "strGlass")]
    glass: Option<String>,
    #[serde(flatten)]
    instructions: Instructions,
    #[serde(rename = "strDrinkThumb")]
    drink_thumb: Option<String>,
    #[serde(flatten)]
    ingredients: StaticIngredientsDto,
    #[serde(flatten)]
    measure: MeasureDto,
    #[serde(rename = "strImageSource")]
    image_source: Option<String>,
    #[serde(rename = "strImageAttribution")]
    image_attribution: Option<String>,
    #[serde(rename = "strCreativeCommonsConfirmed")]
    creative_commons_confirmed: Option<String>,
    date_modified: Option<String>,
}

#[derive(Debug)]
pub struct Cocktail {
    pub id_drink: Option<String>,
    pub drink: Option<String>,
    pub drink_alternate: Option<String>,
    pub tags: Option<String>,
    pub video: Option<String>,
    pub category: Option<String>,
    pub iba: Option<String>,
    pub alcoholic: Option<String>,
    pub glass: Option<String>,
    pub instructions: Instructions,
    pub drink_thumb: Option<String>,
    pub ingredients: Ingredients,
    pub image_source: Option<String>,
    pub image_attribution: Option<String>,
    pub creative_commons_confirmed: Option<String>,
    pub date_modified: Option<String>,
}

impl From<CocktailDto> for Cocktail {
    fn from(value: CocktailDto) -> Self {
        Self {
            id_drink: value.id_drink,
            drink: value.drink,
            drink_alternate: value.drink_alternate,
            tags: value.tags,
            video: value.video,
            category: value.category,
            iba: value.iba,
            alcoholic: value.alcoholic,
            glass: value.glass,
            instructions: value.instructions,
            drink_thumb: value.drink_thumb,
            ingredients: (value.ingredients, value.measure).into(),
            image_source: value.image_source,
            image_attribution: value.image_attribution,
            creative_commons_confirmed: value.creative_commons_confirmed,
            date_modified: value.date_modified,
        }
    }
}

#[derive(Debug, Deref)]
pub struct Cocktails(Vec<Cocktail>);

impl Cocktails {
    /// Lookup a random cocktail
    #[instrument]
    pub async fn random(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get((&client.base_url.join("random.php")?).to_string())
            .await?
            .json::<CocktailsDto>()
            .await?
            .into())
    }

    /// Lookup a selection of 10 random cocktails
    ///
    /// *Note*: Only available to $2+ [Patreon Supporters](https://www.patreon.com/thedatadb)
    #[instrument]
    pub async fn multiple_random(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get((&client.base_url.join("randomselection.php")?).to_string())
            .await?
            .json::<CocktailsDto>()
            .await?
            .into())
    }

    /// List popular cocktails
    ///
    /// *Note*: Only available to $2+ [Patreon Supporters](https://www.patreon.com/thedatadb)
    #[instrument]
    pub async fn popular(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get((&client.base_url.join("popular.php")?).to_string())
            .await?
            .json::<CocktailsDto>()
            .await?
            .into())
    }

    /// List mot latest cocktails
    ///
    /// *Note*: Only available to $2+ [Patreon Supporters](https://www.patreon.com/thedatadb)
    #[instrument]
    pub async fn latest(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get((&client.base_url.join("popular.php")?).to_string())
            .await?
            .json::<CocktailsDto>()
            .await?
            .into())
    }

    /// Search cocktail by name
    #[instrument]
    pub async fn by_name(client: &Client, name: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("search.php")?;
        url.set_query(Some(&format!("s={}", name)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// List all cocktails by first letter
    #[instrument]
    pub async fn by_first_letter(client: &Client, letter: char) -> Result<Self, Error> {
        let mut url = client.base_url.join("search.php")?;
        url.set_query(Some(&format!("f={}", letter)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// Lookup full cocktail details by id
    #[instrument]
    pub async fn by_id(client: &Client, id: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("search.php")?;
        url.set_query(Some(&format!("i={}", id)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// Search by ingredient
    #[instrument]
    pub async fn by_ingredient(client: &Client, ingredient: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("filter.php")?;
        url.set_query(Some(&format!("i={}", ingredient)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// Filter by multi-ingredient
    ///
    /// *Note*: Only available to $2+ [Patreon Supporters](https://www.patreon.com/thedatadb)
    #[instrument]
    pub async fn by_ingredients(client: &Client, ingredients: Vec<&str>) -> Result<Self, Error> {
        let mut url = client.base_url.join("filter.php")?;
        url.set_query(Some(&format!("i={}", ingredients.join(","))));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// Filter by alcoholic
    #[instrument]
    pub async fn by_alcoholic(client: &Client, alcoholic: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("filter.php")?;
        url.set_query(Some(&format!("a={}", alcoholic)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// Filter by category
    #[instrument]
    pub async fn by_category(client: &Client, category: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("filter.php")?;
        url.set_query(Some(&format!("c={}", category)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }

    /// Filter by glass
    #[instrument]
    pub async fn by_glass(client: &Client, glass: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("filter.php")?;
        url.set_query(Some(&format!("g={}", glass)));
        Ok(reqwest::get(url.to_string()).await?.json::<CocktailsDto>().await?.into())
    }
}

impl From<CocktailsDto> for Cocktails {
    fn from(value: CocktailsDto) -> Self {
        Self(value.drinks.into_iter().map(Into::into).collect())
    }
}
