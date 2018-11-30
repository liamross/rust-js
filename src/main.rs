extern crate actix_web;
extern crate listenfd;

use actix_web::{server, App, HttpRequest, Responder};
use listenfd::ListenFd;

fn hello(_req: &HttpRequest) -> impl Responder {
    "Hello World!"
}

fn test(_req: &HttpRequest) -> impl Responder {
    "Test World!"
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    // let mut server = server::new(|| App::new().resource("/", |r| r.f(index)));

    let mut server = server::new(|| {
        vec![
            App::new()
                .resource("/", |r| r.f(hello))
                .resource("/test", |r| r.f(test)),
        ]
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}
