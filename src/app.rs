extern crate warp;
extern crate futures;

use warp::{Filter,Rejection};

pub fn routes() -> impl Filter<Extract = (String,), Error = Rejection> {
    let server = path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    server
}

pub fn app(port: u16) {
    println!("runnint server on port {}", port);
    warp::serve(routes())
        .run(([127, 0, 0, 1], port));
}

mod tests {
    use hyper::client::Client;
    use futures::sync::oneshot;
    use tokio::runtime::current_thread::Runtime;
    use std::thread;

    const PORT: u16 = 8080;

    #[test]
    fn smoke_test() {
        serve();
        request("/");
    }

    #[test]
    fn fake_test() {
        serve();
        request("/hello");
    }

    fn serve() {
        let _thread = thread::Builder::new().name(String::from("thread")).spawn(move || {
            let (_tx, rx) = oneshot::channel();
            let (_addr, fut) = warp::serve(super::routes())
                .bind_with_graceful_shutdown(([127, 0, 0, 1], PORT), rx);
            
            let mut rt = Runtime::new().expect("rt new");

            rt.block_on(fut).unwrap();
        }).expect("thread spawn");

    }

    fn request(path: &str) {
        let mut rt = Runtime::new().expect("rt new");
        let addr_str = format!("http://127.0.0.1:{}/{}", PORT, path);
        rt.block_on(hyper::rt::lazy(move || {
            let client = Client::new();
            let uri = addr_str.parse().expect("server addr should parse");
            println!("{:?}", uri);
            client.get(uri)
        })).unwrap();
    }
}
