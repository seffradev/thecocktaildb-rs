use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;
use url::Url;

#[derive(Debug, Display, Error)]
pub enum Error {
    Url(#[from] url::ParseError),
}

#[derive(Debug, Serialize, Deserialize)]
struct DrinksDto {
    drinks: Vec<DrinkDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrinkDto {
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
struct IngredientsDto {
    #[serde(rename = "strIngredient1")]
    first: Option<String>,
    #[serde(rename = "strIngredient2")]
    second: Option<String>,
    #[serde(rename = "strIngredient3")]
    third: Option<String>,
    #[serde(rename = "strIngredient4")]
    fourth: Option<String>,
    #[serde(rename = "strIngredient5")]
    fifth: Option<String>,
    #[serde(rename = "strIngredient6")]
    sixth: Option<String>,
    #[serde(rename = "strIngredient7")]
    seventh: Option<String>,
    #[serde(rename = "strIngredient8")]
    eighth: Option<String>,
    #[serde(rename = "strIngredient9")]
    ninth: Option<String>,
    #[serde(rename = "strIngredient10")]
    tenth: Option<String>,
    #[serde(rename = "strIngredient11")]
    eleventh: Option<String>,
    #[serde(rename = "strIngredient12")]
    twelfth: Option<String>,
    #[serde(rename = "strIngredient13")]
    thirteenth: Option<String>,
    #[serde(rename = "strIngredient14")]
    fourteenth: Option<String>,
    #[serde(rename = "strIngredient15")]
    fifteenth: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeasureDto {
    #[serde(rename = "strMeasure1")]
    first: Option<String>,
    #[serde(rename = "strMeasure2")]
    second: Option<String>,
    #[serde(rename = "strMeasure3")]
    third: Option<String>,
    #[serde(rename = "strMeasure4")]
    fourth: Option<String>,
    #[serde(rename = "strMeasure5")]
    fifth: Option<String>,
    #[serde(rename = "strMeasure6")]
    sixth: Option<String>,
    #[serde(rename = "strMeasure7")]
    seventh: Option<String>,
    #[serde(rename = "strMeasure8")]
    eighth: Option<String>,
    #[serde(rename = "strMeasure9")]
    ninth: Option<String>,
    #[serde(rename = "strMeasure10")]
    tenth: Option<String>,
    #[serde(rename = "strMeasure11")]
    eleventh: Option<String>,
    #[serde(rename = "strMeasure12")]
    twelfth: Option<String>,
    #[serde(rename = "strMeasure13")]
    thirteenth: Option<String>,
    #[serde(rename = "strMeasure14")]
    fourteenth: Option<String>,
    #[serde(rename = "strMeasure15")]
    fifteenth: Option<String>,
}

pub struct Drinks {
    pub drinks: Vec<Drink>,
}

impl From<DrinksDto> for Drinks {
    fn from(value: DrinksDto) -> Self {
        Self {
            drinks: value.drinks.into_iter().map(Into::into).collect(),
        }
    }
}

pub struct Drink {
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

impl From<DrinkDto> for Drink {
    fn from(value: DrinkDto) -> Self {
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

#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
    pub measure: String,
}

impl From<(String, String)> for Ingredient {
    fn from(value: (String, String)) -> Self {
        Self {
            name: value.0,
            measure: value.1,
        }
    }
}

#[derive(Debug, Deref)]
pub struct Ingredients(Vec<Ingredient>);

impl From<(IngredientsDto, MeasureDto)> for Ingredients {
    fn from(value: (IngredientsDto, MeasureDto)) -> Self {
        let mut ingredients = vec![];

        let val = value.0.first.zip(value.1.first);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.second.zip(value.1.second);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.third.zip(value.1.third);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.fourth.zip(value.1.fourth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.fifth.zip(value.1.fifth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.sixth.zip(value.1.sixth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.seventh.zip(value.1.seventh);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.eighth.zip(value.1.eighth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.ninth.zip(value.1.ninth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.tenth.zip(value.1.tenth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.eleventh.zip(value.1.eleventh);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.twelfth.zip(value.1.twelfth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.thirteenth.zip(value.1.thirteenth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.fourteenth.zip(value.1.fourteenth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        let val = value.0.fifteenth.zip(value.1.fifteenth);
        if val.is_some() {
            ingredients.push(val.unwrap().into());
        }

        Self(ingredients)
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(false);
    }
}
