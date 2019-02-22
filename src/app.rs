extern crate warp;
extern crate futures;

use warp::{Filter,Rejection};

pub fn routes() -> impl Filter<Extract = (String,), Error = Rejection> {
    let server = warp::path::path("hello")
        .and(warp::path::param())
        .map(|name: String| {
            format!("Hello, {}!", name)
        });
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
    use std::sync::mpsc;
    use tokio::runtime::current_thread::Runtime;
    use std::thread;

    #[test]
    fn smoke_test() {
        let port = serve();
        request("/", port);
    }

    #[test]
    fn fake_test() {
        let port = serve();
        request("/", port);
    }

    fn serve() -> u16 {
        let (port_tx, port_rx) = mpsc::channel();
        let _thread = thread::Builder::new().name(String::from("thread")).spawn(move || {
            let (_tx, rx) = oneshot::channel();
            let (addr, fut) = warp::serve(super::routes())
                .bind_with_graceful_shutdown(([127, 0, 0, 1], 0), rx);

            port_tx.send(
                addr.port()
            ).expect("server addr tx");
            
            let mut rt = Runtime::new().expect("rt new");

            rt.block_on(fut).unwrap();
        }).expect("thread spawn");
        let port = port_rx.recv().expect("server addr rx");
        port
    }

    fn request(path: &str, port: u16) {
        let mut rt = Runtime::new().expect("rt new");
        let addr_str = format!("http://127.0.0.1:{}{}", port, path);
        rt.block_on(hyper::rt::lazy(move || {
            let client = Client::new();
            let uri = addr_str.parse().expect("server addr should parse");
            println!("{:?}", uri);
            client.get(uri)
        })).unwrap();
    }
}
