#![deny(warnings)]

use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, header, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(
        Response::builder()
        .header( header::CONTENT_TYPE, "application/octet-stream" )
            .body(Body::from(include_bytes!("example.jpeg").to_vec()))
            .unwrap()
    )
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