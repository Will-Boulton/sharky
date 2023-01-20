use warp::Filter;


#[tokio::main]
async fn main() {
    let shark = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./www/sharks/happyShark.html"));

    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::fs::file("./www/favicon.ico"));

    let routes = shark.or(favicon);

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
