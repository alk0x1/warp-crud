use warp::{Filter, Rejection, Reply};
use super::controllers;  
// Import other controllers as needed

// Define your routes using the imported controllers

macro_rules! resource_routes {
  ($resource_name:ident, $controller:expr) => {
    fn resource_routes(resource_name: &str) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
      let create_route = warp::path(resource_name)
        .and(warp::post())
        .and_then($controller.create);
    
      let get_all_route = warp::path(resource_name)
        .and(warp::get())
        .and_then(move || $controller.getAll);
    
      let get_by_id_route = warp::path(resource_name)
        .and(warp::get())
        .and(warp::path::param::<u32>())
        .and_then($controller.getOne);
    
      let update_route = warp::path(resource_name)
        .and(warp::put())
        .and(warp::path::param::<u32>())
        .and_then($controller.update);
    
      let remove_route = warp::path(resource_name)
        .and(warp::delete())
        .and(warp::path::param::<u32>())
        .and_then($controller.delete);
    
      create_route
        .or(get_all_route)
        .or(get_by_id_route)
        .or(update_route)
        .or(remove_route)
    }
  }
}

mod routes {
  fn users_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    resource_routes!(users, controllers::user);
  }
  
  fn place_route() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    resource_routes!(places, controllers::place);
  }

  pub fn start_routes() {
    users_route();
    place_route();
  }
}