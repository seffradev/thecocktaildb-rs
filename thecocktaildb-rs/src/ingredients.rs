use derive_more::Deref;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{Client, Error};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MeasureDto {
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StaticIngredientsDto {
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
pub(crate) struct DynamicIngredientsDto {
    ingredients: Vec<IngredientDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IngredientDto {
    #[serde(rename = "idIngredient")]
    id: Option<String>,
    #[serde(rename = "strIngredient")]
    ingredient: Option<String>,
    #[serde(rename = "strDescription")]
    description: Option<String>,
    #[serde(rename = "strType")]
    kind: Option<String>,
    #[serde(rename = "strAlcohol")]
    alcohol: Option<String>,
    #[serde(rename = "strABV")]
    abv: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IngredientListDto {
    #[serde(rename = "strIngredient1")]
    ingredient: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IngredientsListDto {
    drinks: Vec<IngredientListDto>,
}

#[derive(Debug)]
pub struct Ingredient {
    pub ingredient: Option<String>,
    pub measure: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub kind: Option<String>,
    pub alcohol: Option<String>,
    pub abv: Option<String>,
}

impl From<(String, String)> for Ingredient {
    fn from(value: (String, String)) -> Self {
        Self {
            ingredient: Some(value.0),
            measure: Some(value.1),
            id: None,
            description: None,
            kind: None,
            alcohol: None,
            abv: None,
        }
    }
}

impl From<IngredientListDto> for Ingredient {
    fn from(value: IngredientListDto) -> Self {
        Self {
            ingredient: value.ingredient,
            measure: None,
            id: None,
            description: None,
            kind: None,
            alcohol: None,
            abv: None,
        }
    }
}

impl From<IngredientDto> for Ingredient {
    fn from(value: IngredientDto) -> Self {
        Self {
            ingredient: value.ingredient,
            measure: None,
            id: value.id,
            description: value.description,
            kind: value.kind,
            alcohol: value.alcohol,
            abv: value.abv,
        }
    }
}

#[derive(Debug, Deref)]
pub struct Ingredients(Vec<Ingredient>);

impl Ingredients {
    /// Search ingredient by name
    #[instrument]
    pub async fn by_name(client: &Client, name: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("search.php")?;
        url.set_query(Some(&format!("i={}", name)));
        Ok(reqwest::get(url.to_string()).await?.json::<DynamicIngredientsDto>().await?.into())
    }

    /// Lookup ingredient by ID
    #[instrument]
    pub async fn by_id(client: &Client, id: &str) -> Result<Self, Error> {
        let mut url = client.base_url.join("lookup.php")?;
        url.set_query(Some(&format!("iid={}", id)));
        Ok(reqwest::get(url.to_string()).await?.json::<DynamicIngredientsDto>().await?.into())
    }

    /// List the ingredients
    #[instrument]
    pub async fn list(client: &Client) -> Result<Self, Error> {
        Ok(reqwest::get(client.base_url.join("list.php?i=list")?.to_string())
            .await?
            .json::<IngredientsListDto>()
            .await?
            .into())
    }
}

impl From<DynamicIngredientsDto> for Ingredients {
    fn from(value: DynamicIngredientsDto) -> Self {
        Self(value.ingredients.into_iter().map(Into::into).collect())
    }
}

impl From<IngredientsListDto> for Ingredients {
    fn from(value: IngredientsListDto) -> Self {
        Self(value.drinks.into_iter().map(Into::into).collect())
    }
}

impl From<(StaticIngredientsDto, MeasureDto)> for Ingredients {
    fn from(value: (StaticIngredientsDto, MeasureDto)) -> Self {
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
