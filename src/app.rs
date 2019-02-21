extern crate warp;
use warp::Filter;

pub fn app(port: u16) {
    let hello = path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("Running app on port: {}", port);
    warp::serve(hello)
        .run(([127, 0, 0, 1], port));
}
