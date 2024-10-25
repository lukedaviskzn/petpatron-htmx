use std::{iter, net::SocketAddr, sync::Arc, time::Duration};

use axum::{extract::{Query, State}, http::{StatusCode, Uri}, response::{IntoResponse, Response}, routing::*, Form, Router};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod templates;
mod db;

#[derive(Clone)]
struct AppState {
    conn: Arc<libsql::Connection>,
}

#[derive(Parser)]
struct Args {
    #[clap(long, env)]
    host: SocketAddr,
    #[clap(long, env)]
    turso_database_url: String,
    #[clap(long, env)]
    turso_auth_token: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().expect("failed to read dotenv");

    let Args {
        host,
        turso_database_url,
        turso_auth_token,
    } = Args::parse();

    let conn = Arc::new(libsql::Builder::new_remote(turso_database_url, turso_auth_token).build().await
        .expect("failed to build turso database").connect().expect("failed to connect to turso database"));

    let listener = TcpListener::bind(&host).await.expect(&format!("failed to listen on {host}"));
    let app = Router::new()
        .route("/", get(index))
        .route("/dogs", get(dogs))
        .route("/dogviewer", get(dog_viewer))
        .route("/donate", post(donate))
        .nest_service("/static", ServeDir::new("./static"))
        .with_state(AppState { conn });
    axum::serve(listener, app).await.expect(&format!("failed to serve on {host}"));
}

async fn index() -> templates::Index {
    templates::Index {}
}


#[derive(Debug, serde::Deserialize)]
struct DogsQuery {
    shelter: usize,
    thanks: Option<i64>,
}

async fn dogs(State(AppState { conn }): State<AppState>, Query(DogsQuery { shelter, thanks }): Query<DogsQuery>) -> templates::Dogs {
    templates::Dogs {
        shelter,
        // thanks: thanks.and_then(|id| SHELTERS[shelter].iter().enumerate().find_map(|(i, d)| (d.id == id).then_some(templates::Thanks { dog_index: i, dog_id: id }))),
        thanks: None,
        dogs: db::Dog::fetch_all(&*conn).await.unwrap(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct DogViewerQuery {
    shelter: usize,
    dog: i64,
    appreciative: Option<String>,
}

async fn dog_viewer(State(AppState { conn }): State<AppState>, Query(DogViewerQuery { shelter, dog, appreciative }): Query<DogViewerQuery>) -> templates::DogViewer {
    let dog = db::Dog::fetch(&*conn, dog).await.unwrap().unwrap();
    
    templates::DogViewer {
        shelter,
        dog,
        appreciative: appreciative.is_some(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct DonateForm {
    shelter: i64,
    dog: i64,
    wallet: String,
    amount: i64,
    recurring: Option<String>,
}

async fn donate(Form(DonateForm { shelter, dog, wallet, amount, recurring }): Form<DonateForm>) -> impl IntoResponse {
    let recurring = recurring.is_some();

    Response::builder()
    .status(StatusCode::SEE_OTHER)
    .header("Location", format!("/dogs?shelter={shelter}&thanks={dog}#dogviewer"))
    .body(String::new()).unwrap()
}
