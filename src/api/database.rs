use crate::models::{client::DiscogsClient, collections::SortOrder, database::*};
use reqwest;

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

/// GET /masters/{master_id}/versions{?page,per_page}
///
/// TODO: Bring in SortMasterVersions.
/// TODO: Build better mechanisms around pagination.
///
/// Returns information about the authenticated user's wantlist/collection.
///
/// Default is page=1, per_page=50
pub async fn get_master_release_versions(
    client: &DiscogsClient,
    master_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    sort: Option<SortMasterVersions>,
    sort_order: Option<SortOrder>,
) -> Result<MasterVersions, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/masters/{}/versions?page={}&per_page={}&sort={}&sort_order={}&token={}",
            &client.api_endpoint,
            &master_id,
            page.unwrap_or(1),
            per_page.unwrap_or(50),
            sort.unwrap_or(SortMasterVersions::Label).to_string(),
            sort_order.unwrap_or(SortOrder::Asc).to_string(),
            client.user_token,
        ))
        .await?
        .json::<MasterVersions>()
        .await
}
