use axum::{extract::Extension, response::IntoResponse, routing, Router};

use crate::controllers::tweets;
use crate::database::{self, RepositoryProvider};
use crate::response;
use crate::services;

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/tweets", tweets::tweets())
        .layer(database_layer)
}

async fn get(Extension(repository_provider): Extension<RepositoryProvider>) -> impl IntoResponse {
    let tweet_repo = repository_provider.tweets();
    let home = services::list_tweets(&tweet_repo).await;
    response::from_template(home);
}
