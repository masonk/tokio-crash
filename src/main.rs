extern crate actix_web;
extern crate ldap3;

use actix_web::{server, App, HttpRequest, Responder, http};

fn main() {
    server::new(|| {
        let ldap = ldap3::LdapConn::new("ldap://localhost:389").unwrap();
        App::new().resource("/", |r|  r.f(index))
    })
    .bind("127.0.0.1:8080").unwrap()
    .run();
}

fn index(_req: &HttpRequest) -> String {
    format!("Hello, world")
}