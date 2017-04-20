extern crate hyper;
extern crate hyper_native_tls;

use std::env;
use std::io;
use hyper::Client;
use hyper::header::{Headers, Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn collection_uri(hatena_id: &str, blog_id: &str) -> String {
    format!("https://blog.hatena.ne.jp/{hatena_id}/{blog_id}/atom/entry",
            hatena_id = hatena_id,
            blog_id = blog_id)
}

fn get_env(key: &str) -> Option<String> {
    env::vars().find(|&(ref k, _)| k == key).map(|(_, v)| v)
}

fn get() {
    let hatena_id = get_env("HATENA_USERNAME").unwrap();
    let blog_id = get_env("HATENA_BLOG_ID").unwrap();
    let api_key = get_env("HATENA_API_KEY").unwrap();
    let url = collection_uri(&hatena_id, &blog_id);
    let username = hatena_id;
    let password = api_key;

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut headers = Headers::new();
    let basic = Basic {
        username: username.to_owned(),
        password: Some(password.to_owned()),
    };
    headers.set(Authorization(basic));
    let mut res = client.get(&url).headers(headers).send().unwrap();
    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    io::copy(&mut res, &mut io::stdout()).unwrap();
}

fn main() {
    get();
}
