use derive_more::Deref;
use serde::{Deserialize, Serialize};

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
    pub ingredients: Vec<Ingredient>,
    pub image_source: Option<String>,
    pub image_attribution: Option<String>,
    pub creative_commons_confirmed: Option<String>,
    pub date_modified: Option<String>,
}

#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
    pub measure: String,
}

#[derive(Debug, Deref)]
pub struct Ingredients(Vec<Ingredient>);

impl From<(IngredientsDto, MeasureDto)> for Ingredients {
    fn from(_value: (IngredientsDto, MeasureDto)) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(false);
    }
}
