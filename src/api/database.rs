use crate::models::{client::DiscogsClient, database::*};
use reqwest;

/// GET /releases/{release_id}{?curr_abbr}
/// Endpoint shows lowest current price of sale in a given currency (defaults to USD)
///
pub async fn get_release_database(
    client: &DiscogsClient,
    release: i64,
    currency: Option<CurrAbbr>,
) -> Result<DatabaseRelease, reqwest::Error> {
    let url = format!(
        "{}/releases/{}?curr_abbr={}",
        &client.api_endpoint,
        &release,
        &currency.unwrap_or(CurrAbbr::USD).to_string(),
    );
    client
        .http_client
        .get(&url)
        .header(reqwest::header::USER_AGENT, &client.user_agent)
        .send()
        .await?
        .json::<DatabaseRelease>()
        .await
}
