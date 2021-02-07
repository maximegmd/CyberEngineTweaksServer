use hyper::{Body, Request, Response};
use std::sync::Arc;
use async_std::sync::Mutex;
use std::convert::Infallible;

use crate::context::{Context};

pub async fn handle(_req: Request<Body>, ctx: Arc::<Mutex::<Context>>) -> Result<Response<Body>, Infallible> {

    let stats = ctx.lock().await.stats;

    let result = format!("# HELP cet_player_count Online player count
# TYPE cet_player_count gauge
cet_player_count {}", stats.player_count);

    Ok(Response::new(Body::from(result)))
}