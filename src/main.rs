#![deny(warnings)]

use std::convert::Infallible;

use hyper::{Body, header, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde::Serialize;

#[derive(Serialize)]
pub struct Image {
    pub image: String
}

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>
            Ok(
                Response::builder()
                    .header( header::CONTENT_TYPE, "application/octet-stream" )
                    .body(Body::from(include_bytes!("example.jpeg").to_vec()))
                    .unwrap()
            ),
        (&Method::GET, "/json") => {
            let data = Image {
                image: base64::encode(include_bytes!("example.jpeg"))
            };
            Ok(
                Response::builder()
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&data).unwrap()))
                    .unwrap()
            )
        }
        _ => {
            Ok(
                Response::default()
            )
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}