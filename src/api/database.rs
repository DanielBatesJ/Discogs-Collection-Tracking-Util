use crate::models::{client::DiscogsClient, database::*};
use reqwest;
use serde_derive::Deserialize;

/// GET requests do NOT require a token for authentication for these endpoints.

/// GET /releases/{release_id}{?curr_abbr}
/// Endpoint shows lowest current price of sale in a given currency (defaults to USD)
///
pub async fn get_release_database(
    client: &DiscogsClient,
    release: i64,
    currency: Option<CurrAbbr>,
) -> Result<DatabaseRelease, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/releases/{}?curr_abbr={}",
            &client.api_endpoint,
            &release,
            &currency.unwrap_or(CurrAbbr::USD).to_string(),
        ))
        .await?
        .json::<DatabaseRelease>()
        .await
}

/// GET /releases/{release_id}/rating/{username}
pub async fn get_release_rating_by_user(
    client: &DiscogsClient,
    release: i64,
    username: &str,
) -> Result<UserRating, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/releases/{}/rating/{}",
            &client.api_endpoint, &release, &username,
        ))
        .await?
        .json::<UserRating>()
        .await
}

/// GET /releases/{release_id}/rating
pub async fn get_community_release_rating(
    client: &DiscogsClient,
    release: i64,
) -> Result<DatabaseRatings, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/releases/{}/rating",
            &client.api_endpoint, &release
        ))
        .await?
        .json::<DatabaseRatings>()
        .await
}

/// GET /releases/{release_id}/stats
pub async fn get_release_stats(
    client: &DiscogsClient,
    release: i64,
) -> Result<DatabaseStats, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/releases/{}/stats",
            &client.api_endpoint, &release
        ))
        .await?
        .json::<DatabaseStats>()
        .await
}

/// GET /masters/{master_id}
pub async fn get_master_release(
    client: &DiscogsClient,
    master_id: i64,
) -> Result<MasterRelease, reqwest::Error> {
    client
        .api_call(&format!("{}/masters/{}", &client.api_endpoint, &master_id))
        .await?
        .json::<MasterRelease>()
        .await
}
