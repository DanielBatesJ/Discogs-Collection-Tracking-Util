use crate::api::{collections::*, database::*};
use crate::models::{client::*, collections::*, database::*};

#[tokio::test]
async fn collections_endpoints() {
    let user_token = dotenv::var("DISCOGS_USER_TOKEN")
        .expect("Ensure you have a .env in the project root with a valid \"DISCOGS_USER_TOKEN\"");
    let cli = DiscogsClient::new(&user_token, "Rust Script DanielBatesJ", "DanielBatesJ");
    let a = get_collection_folders(&cli).await;
    //println!("{a:#?}");
    assert!(a.is_ok());

    assert!(get_collection_folder(&cli, None).await.is_ok());

    let b = get_collection_items_by_release(&cli, 25982725).await;
    assert!(b.is_ok());
    //println!("{b:#?}");

    let c = get_list_custom_fields(&cli).await;
    assert!(c.is_ok());
    //println!("{c:#?}");

    let d = get_collection_value(&cli).await;
    assert!(d.is_ok());
    //println!("{d:#?}");
}

#[tokio::test]
async fn database_endpoints() {
    let user_token = dotenv::var("DISCOGS_USER_TOKEN")
        .expect("Ensure you have a .env in the project root with a valid \"DISCOGS_USER_TOKEN\"");
    let cli = DiscogsClient::new(&user_token, "Rust Script DanielBatesJ", "DanielBatesJ");
    let a = get_release_database(&cli, 1097885, Some(CurrAbbr::USD)).await;
    assert!(a.is_ok());
    // println!("{a:#?}");
    let b = get_release_rating_by_user(&cli, 26048068, "DanielBatesJ").await;
    assert!(b.is_ok());
    // println!("{b:#?}");
    let c = get_community_release_rating(&cli, 26048068).await;
    assert!(c.is_ok());
    // println!("{c:#?}");
    let d = get_release_stats(&cli, 1097885).await;
    assert!(d.is_ok());
    // println!("{d:#?}");
    let e = get_master_release(&cli, 484590).await;
    assert!(e.is_ok());
    // println!("{e:#?}");
    let f = get_master_release_versions(
        &cli,
        9478,
        Some(1),
        Some(55),
        Some(SortMasterVersions::Released),
        Some(SortOrder::Asc),
    )
    .await;
    assert!(f.is_ok());
    // println!("{f:#?}");
    let g = get_artist(&cli, 101028).await;
    // println!("{:#?}", g);
    assert!(g.is_ok());
    let h = get_artist_releases(&cli, 101028, SortAndPaginationArtistReleases::default()).await;
    // println!("{:#?}", h);
    assert!(h.is_ok());
    let i = get_label(&cli, 681).await;
    // println!("{:#?}", i);
    assert!(i.is_ok());
    let j = get_label_releases(&cli, 681, PaginationParams::default()).await;
    // println!("{:#?}", j);
    assert!(j.is_ok());
}

#[tokio::test]
async fn database_search() {
    let user_token = dotenv::var("DISCOGS_USER_TOKEN")
        .expect("Ensure you have a .env in the project root with a valid \"DISCOGS_USER_TOKEN\"");
    let cli = DiscogsClient::new(&user_token, "Rust Script DanielBatesJ", "DanielBatesJ");
    let mut params = SearchParams::default();
    params.artist = Some("Rush".to_string());
    params.catno = Some("54437-0".to_string());
    let _a = build_parameters_format(&cli, params.clone(), "/database/search").await;
    // println!("{:#?}", a);
    let b = search(&cli, params).await;
    // println!("{:#?}", b);
    assert!(b.is_ok());
}

//
// This code was used to run through all of my personal collection and validate that the db releases page would parse correctly. I'm sure it's not perfect as of rn, but my 270+ records all work.
//

// use std::collections::HashSet;
// use std::fs::File;
// use std::io::prelude::*;

// #[tokio::test]
// async fn validate_db_endpoints() {
//     let user_token = dotenv::var("DISCOGS_USER_TOKEN")
//         .expect("Ensure you have a .env in the project root with a valid \"DISCOGS_USER_TOKEN\"");
//     let cli = DiscogsClient::new(&user_token, "Rust Script DanielBatesJ", "DanielBatesJ");
//     let collection = get_collection_folder(&cli, None).await.unwrap();

//     // Read in old IDs
//     let mut rfile = File::open("results.txt").unwrap();
//     let mut buff = String::new();
//     let _ = rfile.read_to_string(&mut buff).unwrap();
//     let old_ids: HashSet<i64> = serde_json::from_str(&buff).unwrap_or(HashSet::new());
//     println!("Skipping {} results.", old_ids.len());
//     // New IDs
//     let wfile = File::create("results.txt").unwrap();
//     let mut new_ids: HashSet<i64> = HashSet::new();
//     for rel in collection {
//         if !old_ids.contains(&rel.id) {
//             match get_release_database(&cli, rel.id, Some(CurrAbbr::USD)).await {
//                 Ok(_) => {
//                     new_ids.insert(rel.id);
//                     ()
//                 }
//                 Err(e) => {
//                     println!("Failed on ID: {}", rel.id);
//                     eprintln!("{e:#?}");
//                     new_ids.extend(old_ids.iter());
//                     serde_json::to_writer(&wfile, &new_ids).unwrap();
//                     assert!(false);
//                     ()
//                 }
//             }
//         }
//     }
// }
