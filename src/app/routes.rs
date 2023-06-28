use warp::{Filter, Rejection, Reply};

// users controllers goes here
// place controllers goes here

pub fn resource_routes(resource_name: &str) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let create_route = warp::path(resource_name)
        .and(warp::post())
        .and_then(move || common_routes::create(resource_name));

    let get_all_route = warp::path(resource_name)
        .and(warp::get())
        .and_then(move || common_routes::get_all(resource_name));

    let get_by_id_route = warp::path(resource_name)
        .and(warp::get())
        .and(warp::path::param::<u32>())
        .and_then(move |id| common_routes::get_by_id(resource_name, id));

    let update_route = warp::path(resource_name)
        .and(warp::put())
        .and(warp::path::param::<u32>())
        .and_then(move |id| common_routes::update(resource_name, id));

    let remove_route = warp::path(resource_name)
        .and(warp::delete())
        .and(warp::path::param::<u32>())
        .and_then(move |id| common_routes::remove(resource_name, id));

    create_route
        .or(get_all_route)
        .or(get_by_id_route)
        .or(update_route)
        .or(remove_route)
}
