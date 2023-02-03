use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub pagination: Pagination,
    pub releases: Vec<Release>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: i64,
    pub pages: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    pub items: i64,
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    pub last: Option<String>,
    pub next: Option<String>,
    pub first: Option<String>,
    pub prev: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub id: i64,
    #[serde(rename = "instance_id")]
    pub instance_id: i64,
    #[serde(rename = "date_added")]
    pub date_added: String, // TOOD: Make this a timestamp?
    pub rating: i64,
    #[serde(rename = "basic_information")]
    pub basic_information: BasicInformation,
    #[serde(rename = "folder_id")]
    pub folder_id: i64,
    pub notes: Option<Vec<Note>>,
    pub price: Option<f32>, // Custom field for this program.
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicInformation {
    pub id: i64,
    #[serde(rename = "master_id")]
    pub master_id: i64,
    #[serde(rename = "master_url")]
    pub master_url: Option<String>,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
    pub thumb: String,
    #[serde(rename = "cover_image")]
    pub cover_image: String,
    pub title: String,
    pub year: i64,
    pub formats: Vec<Format>,
    pub labels: Vec<Label>,
    pub artists: Vec<Artist>,
    pub genres: Vec<String>,
    pub styles: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub name: String,
    pub qty: String,
    pub text: Option<String>,
    pub descriptions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub name: String,
    pub catno: String,
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    #[serde(rename = "entity_type_name")]
    pub entity_type_name: String,
    pub id: i64,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub name: String,
    pub anv: String,
    pub join: String,
    pub role: String,
    pub tracks: String,
    pub id: i64,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "field_id")]
    pub field_id: i64,
    pub value: String,
}

// Notes to denote the field numbers for the "notes" on Discogs, I have custom notes on here as well.
#[derive(Eq, PartialEq)]
pub enum Notes {
    Null, // Discogs starts the field numbers at 1.
    MediaCondition,
    SleveCondition,
    FreeformNotes,
    PurchaseLocation,
    DateOrdered,
    DateRecieved,
    Price,
}

use std::convert::TryFrom;

impl TryFrom<i64> for Notes {
    type Error = ();

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            x if x == Notes::Null as i64 => Ok(Notes::Null),
            x if x == Notes::MediaCondition as i64 => Ok(Notes::MediaCondition),
            x if x == Notes::SleveCondition as i64 => Ok(Notes::SleveCondition),
            x if x == Notes::FreeformNotes as i64 => Ok(Notes::FreeformNotes),
            x if x == Notes::PurchaseLocation as i64 => Ok(Notes::PurchaseLocation),
            x if x == Notes::DateOrdered as i64 => Ok(Notes::DateOrdered),
            x if x == Notes::DateRecieved as i64 => Ok(Notes::DateRecieved),
            x if x == Notes::Price as i64 => Ok(Notes::Price),
            _ => Err(()),
        }
    }
}
