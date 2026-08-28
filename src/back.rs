use utoipa::{OpenApi, ToSchema};
use chrono::{DateTime, Utc};

#[derive(ToSchema)]
#[derive(Clone)]
struct Walk {
    id: u64,
    time: DateTime<Utc>,
    person: String,
}

#[utoipa::path(
    get,
    path = "/walks?from={from}&to={to}",
    responses(
        (status = 200, description = "Walks that Occurred", body = [Walk])
    ),
    params(
        ("from" = DateTime<Utc>, Path, description = "from time for walks list"),
        ("to" = DateTime<Utc>, Path, description = "to time for walks list"),
    )
)]
async fn get_walks_by_time_window<E>(from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<Walk>, E> {
    Ok([Walk {
        id: 5,
        time: Utc::now(),
        person: "alan".to_string(),
    }].to_vec())
}

#[derive(OpenApi)]
#[openapi(paths(get_walks_by_time_window))]
pub struct ApiDoc;

//println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
