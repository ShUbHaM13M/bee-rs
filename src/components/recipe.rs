
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeData {
    pub id: i64,
    pub name: String,
    pub tagline: String,
    #[serde(rename = "first_brewed")]
    pub first_brewed: String,
    pub description: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    pub abv: f64,
    pub ibu: Option<f64>,
    #[serde(rename = "target_fg")]
    pub target_fg: i64,
    #[serde(rename = "target_og")]
    pub target_og: f32,
    pub ebc: Option<i64>,
    pub srm: Option<f32>,
    pub ph: Option<f64>,
    #[serde(rename = "attenuation_level")]
    pub attenuation_level: f32,
    pub volume: Volume,
    #[serde(rename = "boil_volume")]
    pub boil_volume: BoilVolume,
    pub method: Method,
    pub ingredients: Ingredients,
    #[serde(rename = "food_pairing")]
    pub food_pairing: Vec<String>,
    #[serde(rename = "brewers_tips")]
    pub brewers_tips: String,
    #[serde(rename = "contributed_by")]
    pub contributed_by: String,
}

impl RecipeData {
    pub fn get_string(&self) -> String {
        return serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub value: i64,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoilVolume {
    pub value: i64,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    #[serde(rename = "mash_temp")]
    pub mash_temp: Vec<MashTemp>,
    pub fermentation: Fermentation,
    pub twist: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MashTemp {
    pub temp: Temp,
    pub duration: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temp {
    pub value: i64,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fermentation {
    pub temp: Temp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredients {
    pub malt: Vec<Malt>,
    pub hops: Vec<Hop>,
    pub yeast: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Malt {
    pub name: String,
    pub amount: Amount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub value: f64,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hop {
    pub name: String,
    pub amount: Amount,
    pub add: String,
    pub attribute: String,
}

