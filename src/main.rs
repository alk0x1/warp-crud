use warp::{Filter, Rejection, Reply};
// mod routes;
mod db;
mod routes;
mod controllers;
mod services;
mod entities;

async fn hello_world() -> Result<impl Reply, Rejection> {
    Ok("Hello, world!")
}

#[tokio::main]
async fn main() {
  println!("Starting Server in 8080...");
  
  warp::serve(/*controllers */).run(([127, 0, 0, 1], 8080)).await;

}
 