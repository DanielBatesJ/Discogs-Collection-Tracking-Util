use crate::models::collections::Pagination;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;
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
    pub rating: Option<Rating>,
    pub submitter: Option<Submitter>,
    pub contributors: Option<Vec<Contributor>>,
    pub data_quality: Option<String>,
    pub status: Option<String>,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserRating {
    pub release_id: i64,
    pub username: String,
    pub rating: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseRatings {
    pub release_id: i64,
    pub rating: Rating,
}

// TODO: This is weird, it should only return num_have and num_want but does not ever return it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    num_have: Option<i32>,
    num_want: Option<i32>,
    is_offensive: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasterRelease {
    pub id: i64,
    pub main_release: i64,
    pub most_recent_release: i64,
    pub resource_url: String,
    pub uri: String,
    pub versions_url: String,
    pub main_release_url: String,
    pub most_recent_release_url: String,
    pub num_for_sale: i64,
    pub lowest_price: f64,
    pub images: Vec<Image>,
    pub genres: Vec<String>,
    pub styles: Vec<String>,
    pub year: i64,
    pub tracklist: Vec<Tracklist>,
    pub artists: Vec<Artist>,
    pub title: String,
    pub data_quality: String,
    pub videos: Vec<Video>,
}

/// Master Versions

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasterVersions {
    pub pagination: Pagination,
    pub filters: Filters,
    pub filter_facets: Vec<FilterFacet>,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filters {
    pub applied: Applied,
    pub available: Available,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Applied {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Available {
    pub format: HashMap<String, i64>,
    pub label: HashMap<String, i64>,
    pub country: HashMap<String, i64>,
    pub released: HashMap<String, i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterFacet {
    pub title: String,
    pub id: String,
    pub values: Vec<DiscogsValue>,
    pub allows_multiple_values: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscogsValue {
    pub title: String,
    pub value: String,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub id: i64,
    pub label: String,
    pub country: String,
    pub title: String,
    pub major_formats: Vec<String>,
    pub format: String,
    pub catno: String,
    pub released: String,
    pub status: String,
    pub resource_url: String,
    pub thumb: String,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub user: Option<CollectionWantlist>,
    pub community: Option<CollectionWantlist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionWantlist {
    pub in_collection: i64,
    pub in_wantlist: i64,
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
pub enum SortMasterVersions {
    Released, // Year
    Title,
    Format,
    Label,
    Catno,
    Country,
}

impl fmt::Display for SortMasterVersions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortMasterVersions::Label => write!(f, "label"),
            SortMasterVersions::Title => write!(f, "title"),
            SortMasterVersions::Catno => write!(f, "catno"),
            SortMasterVersions::Format => write!(f, "format"),
            SortMasterVersions::Released => write!(f, "released"),
            SortMasterVersions::Country => write!(f, "country"),
        }
    }
}
