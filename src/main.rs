#![allow(dead_code)]
use crate::api::collections::*;
use crate::models::client::DiscogsClient;
use crate::models::collections::*;
use dotenv::dotenv;
use std::error::Error;
use tokio;
extern crate serde_derive;

mod api;
mod models;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let user_token = dotenv::var("DISCOGS_USER_TOKEN")
        .expect("Ensure you have a .env in the project root with a valid \"DISCOGS_USER_TOKEN\"");
    let client = DiscogsClient::new(&user_token, "Rust Script DanielBatesJ", "DanielBatesJ");
    let mut collection = get_collection_folder(&client, None).await?;
    add_price(&mut collection).await;
    // sort_by_price(&mut collection).await;
    // display(&collection).await;
    let total_spend = sum_spend(&collection).await;
    println!("Total spent according to price tracking: ${total_spend}");

    let collection_value = get_collection_value(&client).await?.convert_to_float();
    println!("Maximum collection value: ${}", collection_value.maximum);
    println!("Median collection value:  ${}", collection_value.median);
    println!("Minimum collection value: ${}", collection_value.minimum);
    Ok(())
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
                    note.value
                        .replace(&['$', ','], "")
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
