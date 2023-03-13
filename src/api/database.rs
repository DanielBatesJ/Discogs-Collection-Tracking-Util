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

/// GET /artists/{artist_id}
pub async fn get_artist(
    client: &DiscogsClient,
    artist_id: i64,
) -> Result<ArtistPage, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/artists/{}?token={}",
            &client.api_endpoint, artist_id, &client.user_token
        ))
        .await?
        .json::<ArtistPage>()
        .await
}

/// GET /artists/{artist_id}/releases{?sort,sort_order}
/// TODO: Better handle pagination?
pub async fn get_artist_releases(
    client: &DiscogsClient,
    artist_id: i64,
    sp: SortAndPaginationArtistReleases,
) -> Result<ArtistReleases, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/artists/{}/releases?page={}&per_page={}&sort={}&sort_order={}&token={}",
            &client.api_endpoint,
            artist_id,
            sp.page.unwrap_or(1),
            sp.per_page.unwrap_or(50),
            sp.sort.unwrap_or(SortArtist::Year).to_string(),
            sp.sort_order.unwrap_or(SortOrder::Asc).to_string(),
            client.user_token,
        ))
        .await?
        .json::<ArtistReleases>()
        .await
}

/// GET /labels/{label_id}
pub async fn get_label(client: &DiscogsClient, label_id: i64) -> Result<LabelPage, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/labels/{}?token={}",
            &client.api_endpoint, label_id, client.user_token,
        ))
        .await?
        .json::<LabelPage>()
        .await
}

/// GET /labels/{label_id}/releases{?page,per_page}
pub async fn get_label_releases(
    client: &DiscogsClient,
    label_id: i64,
    p: PaginationParams,
) -> Result<LabelReleases, reqwest::Error> {
    client
        .api_call(&format!(
            "{}/labels/{}/releases?page={}&per_page={}&token={}",
            &client.api_endpoint, label_id, p.page, p.per_page, client.user_token,
        ))
        .await?
        .json::<LabelReleases>()
        .await
}

/// GET /database/search?q={query}&{?type,title,release_title,credit,artist,anv,label,genre,style,country,year,format,catno,barcode,track,submitter,contributor}
///
pub async fn search(
    client: &DiscogsClient,
    params: SearchParams,
) -> Result<SearchResults, reqwest::Error> {
    client
        .api_call(&build_parameters_format(&client, params, "/database/search").await)
        .await?
        .json()
        .await
}

pub async fn build_parameters_format(
    client: &DiscogsClient,
    params: SearchParams,
    base_url: &str,
) -> String {
    let mut filters: Vec<String> = Vec::new();
    let mut url_string: String = String::new();
    if let Some(query) = params.query {
        filters.push(format!("q={}", query));
    }
    if let Some(search_type) = params.search_type {
        filters.push(format!("type={}", search_type));
    }
    if let Some(title) = params.title {
        filters.push(format!("title={}", title));
    }
    if let Some(release_title) = params.release_title {
        filters.push(format!("release_title={}", release_title));
    }
    if let Some(credit) = params.credit {
        filters.push(format!("credit={}", credit));
    }
    if let Some(artist) = params.artist {
        filters.push(format!("artist={}", artist));
    }
    if let Some(anv) = params.anv {
        filters.push(format!("anv={}", anv));
    }
    if let Some(label) = params.label {
        filters.push(format!("label={}", label));
    }
    if let Some(genre) = params.genre {
        filters.push(format!("genre={}", genre));
    }
    if let Some(style) = params.style {
        filters.push(format!("style={}", style));
    }
    if let Some(country) = params.country {
        filters.push(format!("country={}", country));
    }
    if let Some(year) = params.year {
        filters.push(format!("year={}", year));
    }
    if let Some(format) = params.format {
        filters.push(format!("format={}", format));
    }
    if let Some(catno) = params.catno {
        filters.push(format!("catno={}", catno));
    }
    if let Some(barcode) = params.barcode {
        filters.push(format!("barcode={}", barcode));
    }
    if let Some(track) = params.track {
        filters.push(format!("track={}", track));
    }
    if let Some(submitter) = params.submitter {
        filters.push(format!("submitter={}", submitter));
    }
    if let Some(contributor) = params.contributor {
        filters.push(format!("contributor={}", contributor));
    }
    if filters.is_empty() {
        return format!(
            "{}/{}?token={}",
            &client.api_endpoint, base_url, &client.user_token
        );
    }
    for (i, entry) in filters.iter().enumerate() {
        if i == 0 {
            url_string = format!("{}/{}?{}", &client.api_endpoint, base_url, entry);
        } else {
            url_string = format!("{}&{}", url_string, entry);
        }
    }
    format!("{}&token={}", url_string, &client.user_token)
    // Token
}
