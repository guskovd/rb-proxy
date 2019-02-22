extern crate warp;
extern crate futures;

use warp::{Filter,Rejection};

fn routes() -> impl Filter<Extract = (String,), Error = Rejection> {
    let server = path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    server
}

pub fn app(port: u16) {
    println!("runnint server on port {}", port);
    warp::serve(routes())
        .run(([127, 0, 0, 1], port));
}

#[test]
fn smoke_test() {
    use hyper::client::Client;
    use futures::sync::oneshot;
    use tokio::runtime::current_thread::Runtime;
    use std::thread;
    
    let port = 8080;
    let _thread = thread::Builder::new().name(String::from("thread")).spawn(move || {
        let (_tx, rx) = oneshot::channel();
        let (_addr, fut) = warp::serve(routes())
            .bind_with_graceful_shutdown(([127, 0, 0, 1], port), rx);
  
        let mut rt = Runtime::new().expect("rt new");

        rt.block_on(fut).unwrap();
    }).expect("thread spawn");

    let mut rt = Runtime::new().expect("rt new");
    let addr_str = format!("http://127.0.0.1:{}", port);
    rt.block_on(hyper::rt::lazy(move || {
        let client = Client::new();
        let uri = addr_str.parse().expect("server addr should parse");
        client.get(uri)
    })).unwrap();
}
