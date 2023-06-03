//use std::iter::repeat_with;

use actix_web::{get, web, Responder, Result};
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

#[get("/bet")]
async fn new_bet() -> Result<impl Responder> {
    let numbers = MsNumbers::new();

    Ok(web::Json(numbers))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(new_bet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
