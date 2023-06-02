use std::iter::repeat_with;

use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MsNumbers {
    n1: i8,
    n2: i8,
    n3: i8,
    n4: i8,
    n5: i8,
    n6: i8,
}

impl MsNumbers {
    fn new() -> MsNumbers {
        let rng = fastrand::Rng::new();
        let v: Vec<i8> = repeat_with(|| rng.i8(1..61)).take(6).collect();

        MsNumbers {
            n1: v[0],
            n2: v[1],
            n3: v[2],
            n4: v[3],
            n5: v[4],
            n6: v[5],
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
