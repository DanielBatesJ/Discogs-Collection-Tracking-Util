use crate::models::{client::*, collections::*};
use reqwest;

//
//
// User Collection
//
//

/// GET /users/{username}/collection/folders
///
pub async fn get_collection_folders(client: &DiscogsClient) -> Result<Vec<Folder>, reqwest::Error> {
    let url = format!(
        "{}/users/{}/collection/folders?token={}",
        &client.api_endpoint, &client.username, &client.user_token,
    );
    let f = client
        .http_client
        .get(&url)
        .header(reqwest::header::USER_AGENT, &client.user_agent)
        .send()
        .await?
        .json::<Folders>()
        .await?;
    Ok(f.folders)
}

/// GET /users/{username}/collection/folders/{folder_id}
///
/// Optional field for folder_id, will default to `0` if not supplied (all items in collection).
pub async fn get_collection_folder(
    client: &DiscogsClient,
    folder_id: Option<i32>,
) -> Result<Vec<Release>, reqwest::Error> {
    let url = format!(
        "{}/users/{}/collection/folders/{}/releases?token={}",
        &client.api_endpoint,
        &client.username,
        folder_id.unwrap_or(0),
        &client.user_token,
    );
    pagination_releases_loop(&client, url).await
}

/// GET users/{username}/collection/releases/{release_id}
///
pub async fn get_collection_items_by_release(
    client: &DiscogsClient,
    release_id: i64,
) -> Result<Vec<Release>, reqwest::Error> {
    let url = format!(
        "{}/users/{}/collection/releases/{}?token={}",
        &client.api_endpoint, &client.username, release_id, &client.user_token,
    );
    pagination_releases_loop(&client, url).await
}

/// GET /users/{username}/collection/folders/{folder_id}/releases
///
/// Sort defaults to 'label', and SortOrder defaults to 'asc'.
pub async fn get_collection_items_by_folder(
    client: &DiscogsClient,
    folder_id: Option<i32>,
    sort: Option<Sort>,
    sort_order: Option<SortOrder>,
) -> Result<Vec<Release>, reqwest::Error> {
    let url = format!(
        "{}/users/{}/collection/folders/{}/releases?sort={}&sort_order={}&token={}",
        &client.api_endpoint,
        &client.username,
        folder_id.unwrap_or(0),
        sort.unwrap_or(Sort::Label).to_string(),
        sort_order.unwrap_or(SortOrder::Asc).to_string(),
        &client.user_token,
    );
    pagination_releases_loop(&client, url).await
}

/// GET /users/{username}/collection/fields
pub async fn get_list_custom_fields(client: &DiscogsClient) -> Result<Vec<Field>, reqwest::Error> {
    let url = format!(
        "{}/users/{}/collection/fields?token={}",
        &client.api_endpoint, &client.username, &client.user_token,
    );
    let f = client
        .http_client
        .get(&url)
        .header(reqwest::header::USER_AGENT, &client.user_agent)
        .send()
        .await?
        .json::<Fields>()
        .await?;
    Ok(f.fields)
}

// Helper function

/// Collects multiple pages of releases by using the pagination data returned by the Discogs API.
async fn pagination_releases_loop(
    client: &DiscogsClient,
    mut current: String,
) -> Result<Vec<Release>, reqwest::Error> {
    let mut response;
    let mut collection: Vec<Release> = Vec::new();
    loop {
        response = client
            .http_client
            .get(current)
            .header(reqwest::header::USER_AGENT, &client.user_agent)
            .send()
            .await?
            .json::<Collection>()
            .await?;
        collection.append(&mut response.releases);
        if response.pagination.page != response.pagination.pages {
            current = response.pagination.urls.next.unwrap();
        } else {
            return Ok(collection);
        }
    }
}

/// GET /users/{username}/collection/value
pub async fn get_collection_value(
    client: &DiscogsClient,
) -> Result<CollectionValue, reqwest::Error> {
    let url = format!(
        "{}/users/{}/collection/value?token={}",
        &client.api_endpoint, &client.username, &client.user_token,
    );
    client
        .http_client
        .get(&url)
        .header(reqwest::header::USER_AGENT, &client.user_agent)
        .send()
        .await?
        .json::<CollectionValue>()
        .await
}
