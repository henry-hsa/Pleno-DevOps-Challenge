use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};

use actix_cors::Cors;
use awc::Client;
use std::time::Duration;

const MAX_RETRIES: usize = 5;
const RETRY_DELAY: u64 = 2; // seconds

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world in service 1!")
}

#[get("/ping")]
async fn ping_service2() -> Result<HttpResponse, Error> {
    let client = Client::new();
    let ip_ranges = vec!["172.18", "34.101"];
    let mut responses = Vec::new();

    for ip_range in ip_ranges {
        for i in 0..256 {
            for j in 0..256 {
                let ip_address = format!("{}.{}.{}", ip_range, i, j);
                let url = format!("http://{}:8081/pong", ip_address);
                for _ in 0..MAX_RETRIES {
                    match client.post(&url).send().await {
                        Ok(mut response) => {
                            let body = response.body().await?;
                            responses.push(body);
                        }
                        Err(_) => {
                            tokio::time::sleep(Duration::from_secs(RETRY_DELAY)).await;
                        }
                    }
                }
            }
        }
    }

     if responses.is_empty() { 
        Err(actix_web::error::ErrorInternalServerError(
        "Service2 is not responding"
    ))
    }  else {
        Ok(HttpResponse::Ok().body(format!("{:?}", responses)))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allowed_origin_fn(|_, _| true);
        App::new().wrap(cors).service(hello).service(ping_service2)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{self, TestRequest};

    #[actix_rt::test]
    async fn test_hello() {
        let mut app = test::init_service(App::new().service(hello)).await;

        let req = TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world in service 1!");
    }
}