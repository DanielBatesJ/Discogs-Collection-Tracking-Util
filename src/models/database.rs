use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::fmt;

// Auto-generated

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseRelease {
    pub id: i64,
    pub status: String,
    pub year: i64,
    pub resource_url: String,
    pub uri: String,
    pub artists: Vec<Artist>,
    pub artists_sort: String,
    pub labels: Vec<Label>,
    pub series: Vec<Value>,
    pub companies: Vec<Company>,
    pub formats: Vec<Format>,
    pub data_quality: String,
    pub community: Community,
    pub format_quantity: i64,
    pub date_added: String,
    pub date_changed: String,
    pub num_for_sale: i64,
    pub lowest_price: Option<f64>,
    pub master_id: Option<i64>,
    pub master_url: Option<String>,
    pub title: String,
    pub country: Option<String>,
    pub released: Option<String>,
    pub notes: Option<String>,
    pub released_formatted: Option<String>,
    pub identifiers: Vec<Identifier>,
    pub videos: Option<Vec<Video>>,
    pub genres: Vec<String>,
    pub styles: Option<Vec<String>>,
    pub tracklist: Vec<Tracklist>,
    pub extraartists: Vec<Extraartist>,
    pub images: Vec<Image>,
    pub thumb: String,
    pub estimated_weight: i64,
    pub blocked_from_sale: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub anv: String,
    pub join: String,
    pub role: String,
    pub tracks: String,
    pub id: i64,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: i64,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub catno: String,
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: i64,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    pub name: String,
    pub qty: String,
    pub text: Option<String>,
    pub descriptions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Community {
    pub have: i64,
    pub want: i64,
    pub rating: Rating,
    pub submitter: Submitter,
    pub contributors: Vec<Contributor>,
    pub data_quality: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    pub count: i64,
    pub average: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Submitter {
    pub username: String,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    pub username: String,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub uri: String,
    pub title: String,
    pub description: String,
    pub duration: i64,
    pub embed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tracklist {
    pub position: String,
    #[serde(rename = "type_")]
    pub type_field: String,
    pub title: String,
    pub extraartists: Option<Vec<Extraartist>>,
    pub duration: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extraartist {
    pub name: String,
    pub anv: String,
    pub join: String,
    pub role: String,
    pub tracks: String,
    pub id: i64,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub resource_url: String,
    pub uri150: String,
    pub width: i64,
    pub height: i64,
}

// Custom

pub enum CurrAbbr {
    USD,
    GBP,
    EUR,
    CAD,
    AUD,
    JPY,
    CHF,
    MXN,
    BRL,
    NZD,
    SEK,
    ZAR,
}

impl fmt::Display for CurrAbbr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CurrAbbr::USD => write!(f, "USD"),
            CurrAbbr::GBP => write!(f, "GBP"),
            CurrAbbr::EUR => write!(f, "EUR"),
            CurrAbbr::CAD => write!(f, "CAD"),
            CurrAbbr::AUD => write!(f, "AUD"),
            CurrAbbr::JPY => write!(f, "JPY"),
            CurrAbbr::CHF => write!(f, "CHF"),
            CurrAbbr::MXN => write!(f, "MXN"),
            CurrAbbr::BRL => write!(f, "BRL"),
            CurrAbbr::NZD => write!(f, "NZD"),
            CurrAbbr::SEK => write!(f, "SEK"),
            CurrAbbr::ZAR => write!(f, "ZAR"),
        }
    }
}
