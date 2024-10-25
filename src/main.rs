use std::{iter, net::SocketAddr, time::Duration};

use axum::{extract::Query, http::{StatusCode, Uri}, response::{IntoResponse, Response}, routing::*, Form, Router};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod templates;

#[derive(Parser)]
struct Args {
    #[clap(long, env)]
    host: SocketAddr,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().expect("failed to read dotenv");

    let Args {
        host,
    } = Args::parse();

    let listener = TcpListener::bind(&host).await.expect(&format!("failed to listen on {host}"));
    let app = Router::new()
        .route("/", get(index))
        .route("/dogs", get(dogs))
        .route("/dogviewer", get(dog_viewer))
        .route("/donate", post(donate))
        .nest_service("/static", ServeDir::new("./static"));
    axum::serve(listener, app).await.expect(&format!("failed to serve on {host}"));
}

const SHELTERS: &[&[templates::Dog<'static>]] = &[
    &[
        templates::Dog {
            id: 1,
            name: "Yoshi 1",
            bio: "Yoshi 1 is a dog.",
            breed: "Shiba Inu",
            image: "Yoshi.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 2,
            name: "Yoshi 2",
            bio: "Yoshi 2 is a dog.",
            breed: "Shiba Inu",
            image: "Yoshi.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 3,
            name: "Yoshi 3",
            bio: "Yoshi 3 is a dog.",
            breed: "Shiba Inu",
            image: "Yoshi.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 4,
            name: "Yoshi 4",
            bio: "Yoshi 4 is a dog.",
            breed: "Shiba Inu",
            image: "Yoshi.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 5,
            name: "Yoshi 5",
            bio: "Yoshi 5 is a dog.",
            breed: "Shiba Inu",
            image: "Yoshi.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 6,
            name: "Yoshi 6",
            bio: "Yoshi 6 is a dog.",
            breed: "Shiba Inu",
            image: "Yoshi.jpg",
            // birth_date: todo!(),
            account: 30,
        },
    ],
    &[
        templates::Dog {
            id: 1,
            name: "Ruby 1",
            bio: "Ruby 1 is a cat.",
            breed: "Rat Creature",
            image: "Ruby.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 2,
            name: "Ruby 2",
            bio: "Ruby 2 is a cat.",
            breed: "Rat Creature",
            image: "Ruby.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 3,
            name: "Ruby 3",
            bio: "Ruby 3 is a cat.",
            breed: "Rat Creature",
            image: "Ruby.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 4,
            name: "Ruby 4",
            bio: "Ruby 4 is a cat.",
            breed: "Rat Creature",
            image: "Ruby.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 5,
            name: "Ruby 5",
            bio: "Ruby 5 is a cat.",
            breed: "Rat Creature",
            image: "Ruby.jpg",
            // birth_date: todo!(),
            account: 30,
        },
        templates::Dog {
            id: 6,
            name: "Ruby 6",
            bio: "Ruby 6 is a cat.",
            breed: "Rat Creature",
            image: "Ruby.jpg",
            // birth_date: todo!(),
            account: 30,
        },
    ],
];

async fn index() -> templates::Index {
    templates::Index {}
}


#[derive(Debug, serde::Deserialize)]
struct DogsQuery {
    shelter: usize,
    thanks: Option<i64>,
}

async fn dogs(Query(DogsQuery { shelter, thanks }): Query<DogsQuery>) -> templates::Dogs<'static> {
    templates::Dogs {
        shelter,
        thanks: thanks.and_then(|id| SHELTERS[shelter].iter().enumerate().find_map(|(i, d)| (d.id == id).then_some(templates::Thanks { dog_index: i, dog_id: id }))),
        dogs: SHELTERS[shelter],
    }
}

#[derive(Debug, serde::Deserialize)]
struct DogViewerQuery {
    shelter: usize,
    dog: i64,
    appreciative: Option<String>,
}

async fn dog_viewer(Query(DogViewerQuery { shelter, dog, appreciative }): Query<DogViewerQuery>) -> templates::DogViewer<'static> {
    std::thread::sleep(Duration::from_secs(2));
    
    let dog = SHELTERS[shelter].iter().find(|d| d.id == dog).unwrap();
    
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
    std::thread::sleep(Duration::from_secs(2));
    
    let recurring = recurring.is_some();

    Response::builder()
    .status(StatusCode::SEE_OTHER)
    .header("Location", format!("/dogs?shelter={shelter}&thanks={dog}#dogviewer"))
    .body(String::new()).unwrap()
}
