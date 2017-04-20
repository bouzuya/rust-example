extern crate hyper;
extern crate hyper_native_tls;

use std::io;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn get() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut res = client
        .get("http://blog.bouzuya.net/posts.json")
        .send()
        .unwrap();
    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    io::copy(&mut res, &mut io::stdout()).unwrap();
}

fn main() {
    get();
}
