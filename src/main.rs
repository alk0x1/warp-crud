use warp::{Filter, Rejection, Reply};
mod app;

async fn hello_world() -> Result<impl Reply, Rejection> {
    Ok("Hello, world!")
}

#[tokio::main]
async fn main() {
  let hello_route = warp::path("hello").and(warp::get()).and_then(hello_world);

  warp::serve(hello_route).run(([127, 0, 0, 1], 8080)).await;
}
