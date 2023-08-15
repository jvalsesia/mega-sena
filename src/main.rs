use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rand::{seq::SliceRandom, thread_rng};
use serde::Serialize;

#[derive(Serialize)]
struct MsNumbers {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    n5: i32,
    n6: i32,
}

#[derive(Serialize)]
struct Bets {
    bet: u32,
    numbers: MsNumbers,
}

impl MsNumbers {
    fn new() -> MsNumbers {
        let mut vec = Vec::new();
        for number in 1..=60 {
            vec.push(number);
        }
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);

        MsNumbers {
            n1: vec[9],
            n2: vec[19],
            n3: vec[29],
            n4: vec[39],
            n5: vec[49],
            n6: vec[59],
        }
    }
}

async fn new_bet(Path(num): Path<u32>) -> Response {
    let mut vec_bets: Vec<Bets> = Vec::new();

    for i in 1..=num {
        let numbers = MsNumbers::new();

        vec_bets.push(Bets { bet: i, numbers });
    }
    (StatusCode::OK, Json(vec_bets)).into_response()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/bet/:num", get(new_bet));

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
