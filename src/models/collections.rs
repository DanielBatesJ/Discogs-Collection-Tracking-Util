use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::convert::TryFrom;
use std::fmt;

// Auto-generated from JSON.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub pagination: Pagination,
    pub releases: Vec<Release>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub pages: i64,
    pub per_page: i64,
    pub items: i64,
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Urls {
    pub last: Option<String>,
    pub next: Option<String>,
    pub first: Option<String>,
    pub prev: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub id: i64,
    pub instance_id: i64,
    pub date_added: String, // TOOD: Make this a timestamp?
    pub rating: i64,
    pub basic_information: BasicInformation,
    pub folder_id: i64,
    pub notes: Option<Vec<Note>>,
    pub price: Option<f32>, // Custom field for this program.
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicInformation {
    pub id: i64,
    pub master_id: i64,
    pub master_url: Option<String>,
    pub resource_url: String,
    pub thumb: String,
    pub cover_image: String,
    pub title: String,
    pub year: i64,
    pub formats: Vec<Format>,
    pub labels: Vec<Label>,
    pub artists: Vec<Artist>,
    pub genres: Vec<String>,
    pub styles: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Format {
    pub name: String,
    pub qty: String,
    pub text: Option<String>,
    pub descriptions: Vec<String>,
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
pub struct Note {
    pub field_id: i64,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folders {
    pub folders: Vec<Folder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fields {
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub position: i64,
    pub public: bool,
    pub options: Option<Vec<String>>,
    pub lines: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionValue {
    pub minimum: String,
    pub median: String,
    pub maximum: String,
}

impl CollectionValue {
    pub fn convert_to_float(&self) -> CollectionValueFloat {
        CollectionValueFloat {
            minimum: self
                .minimum
                .replace(&['$', ','], "")
                .parse::<f32>()
                .unwrap(),
            median: self.median.replace(&['$', ','], "").parse::<f32>().unwrap(),
            maximum: self
                .maximum
                .replace(&['$', ','], "")
                .parse::<f32>()
                .unwrap(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionValueFloat {
    pub minimum: f32,
    pub median: f32,
    pub maximum: f32,
}

// Custom

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

pub enum Sort {
    Label,
    Artist,
    Title,
    Catno,
    Format,
    Rating,
    Added,
    Year,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sort::Label => write!(f, "label"),
            Sort::Artist => write!(f, "artist"),
            Sort::Title => write!(f, "title"),
            Sort::Catno => write!(f, "catno"),
            Sort::Format => write!(f, "format"),
            Sort::Rating => write!(f, "rating"),
            Sort::Added => write!(f, "added"),
            Sort::Year => write!(f, "year"),
        }
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}
