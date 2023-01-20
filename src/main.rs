use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::Filter;
use warp::reply::{html, Html};


#[tokio::main]
async fn main() {
    let www = warp::get()
        .and(warp::fs::dir("./www"));



    let state = Arc::new(Mutex::new(0));

    let shark = warp::get()
        .and(warp::path("shark"))
        .map({
            let state = state.clone();
            move || {
                *state.lock().unwrap() += 1;
                let state = *state.lock().unwrap();
                return html(format!("{}", state));
            }
        }
        );

    let routes = shark.or(www);

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}