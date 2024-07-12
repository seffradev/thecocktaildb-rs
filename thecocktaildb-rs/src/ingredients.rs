use derive_more::Deref;
use serde::{Deserialize, Serialize};

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
pub(crate) struct IngredientsDto {
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
