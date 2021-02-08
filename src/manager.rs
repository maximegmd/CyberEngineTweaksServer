use hyper::{Body, Request, Response, StatusCode, Method};
use std::sync::Arc;
use async_std::sync::Mutex;
use std::convert::Infallible;
use url::form_urlencoded;
use std::collections::HashMap;
use std::str::FromStr;

use crate::context::Context;

fn bad_request() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::empty()).unwrap())
}

async fn handle_announce(req: Request<Body>, ctx: Arc::<Mutex::<Context>>) -> Result<Response<Body>, Infallible> {

    let forwarded = req.headers().get("X-Forwarded-For")
        .and_then(|n| match n.to_str() {
            Ok(n) => Some(n),
            Err(_) => None
        })
        .map_or(String::from("127.0.0.1"), |n| String::from(n) );

    let host = forwarded.split(",").next().map_or(String::from("127.0.0.1"), |n| String::from(n));

    let b = hyper::body::to_bytes(req).await;
    let b = match b {
        Ok(bytes) => bytes,
        Err(_) => return bad_request()
    };

    let params = form_urlencoded::parse(b.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let mut name = match params.get("name") {
        Some(n) => String::from_str(n).unwrap(),
        _ => return bad_request()
    };

    name.truncate(128);

    ctx.lock().await.update(host, name);

    Ok(Response::builder()
    .status(StatusCode::OK)
    .body(Body::empty())
    .unwrap())
}

async fn handle_stat(_req: Request<Body>, ctx: Arc::<Mutex::<Context>>) -> Result<Response<Body>, Infallible> {

    Ok(Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .body(Body::from(ctx.lock().await.generate_stats()))
    .unwrap())
}

pub async fn handle(req: Request<Body>, ctx: Arc::<Mutex::<Context>>) -> Result<Response<Body>, Infallible> {

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/announce") => handle_announce(req, ctx).await,
        (&Method::GET, "/stat") => handle_stat(req, ctx).await,
        _ => Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap())
    }
}