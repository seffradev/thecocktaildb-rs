use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    ingredients::{Ingredients, IngredientsDto, MeasureDto},
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
    ingredients: IngredientsDto,
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
pub struct Cocktails {
    pub cocktails: Vec<Cocktail>,
}

impl From<CocktailsDto> for Cocktails {
    fn from(value: CocktailsDto) -> Self {
        Self {
            cocktails: value.drinks.into_iter().map(Into::into).collect(),
        }
    }
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

impl Cocktail {
    #[instrument]
    pub async fn random(client: &Client) -> Result<Cocktails, Error> {
        Ok(reqwest::get((&client.base_url.join("random.php")?).to_string())
            .await?
            .json::<CocktailsDto>()
            .await?
            .into())
    }
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
