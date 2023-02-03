use crate::models::{Collection, Notes, Release};
use dotenv::dotenv;
use reqwest;
use std::error::Error;
use tokio;
extern crate serde_derive;
use std::time::Instant;

mod models;

const BASE_URL: &str = "https://api.discogs.com";
const DEFAULT_FOLDER_ID: isize = 0;
const USERNAME: &str = "DanielBatesJ";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let user_token = dotenv::var("DISCOGS_USER_TOKEN").unwrap();
    let start = Instant::now();
    let mut collection = collections(&user_token).await?;
    let duration = start.elapsed();
    add_price(&mut collection).await;
    // display(&collection).await;
    sort_by_price(&mut collection).await;
    display(&collection).await;
    let total_spend = sum_spend(&collection).await;
    println!("{total_spend}");
    println!(
        "Time elapsed collecting the data from the API is: {:?}",
        duration
    );
    Ok(())
}

async fn collections(user_token: &str) -> Result<Vec<Release>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut response;
    let mut current = format!(
        "{BASE_URL}/users/{USERNAME}/collection/folders/{DEFAULT_FOLDER_ID}/releases?token={user_token}");
    let mut collection: Vec<Release> = Vec::new();
    loop {
        response = client
            .get(&current)
            .header(
                reqwest::header::USER_AGENT,
                "Rust script test DanielBatesJ)",
            )
            .send()
            .await?
            .json::<Collection>()
            .await?;
        collection.append(&mut response.releases); // push them here
        if response.pagination.page != response.pagination.pages {
            current = response.pagination.urls.next.unwrap();
        } else {
            break Ok(collection);
        }
    }
}

async fn sort_by_price(collection: &mut Vec<Release>) {
    collection.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
}

async fn display(collection: &Vec<Release>) {
    for record in collection {
        println!(
            "${:06.2?} {:?} - {:}",
            record.price.unwrap_or(0.0),
            record.basic_information.artists[0].name,
            record.basic_information.title
        );
    }
}

async fn sum_spend(collection: &Vec<Release>) -> f32 {
    collection.iter().map(|r| r.price.unwrap_or(0.0)).sum()
}

async fn add_price(collection: &mut Vec<Release>) {
    for record in collection {
        if let Some(notes) = &record.notes {
            record.price = if let Some(note) = notes
                .into_iter()
                .find(|x| x.field_id.try_into() == Ok(Notes::Price))
            {
                Some(
                    note.value.split("$").collect::<Vec<&str>>()[1]
                        .trim()
                        .parse::<f32>()
                        .unwrap(),
                )
            } else {
                None
            }
        }
    }
}
