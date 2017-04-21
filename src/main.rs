extern crate hyper;
extern crate hyper_native_tls;
extern crate url;
extern crate xml;

use std::env;
// use std::io;
use hyper::Client;
use hyper::client::{Response};
use hyper::header::{Headers, Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use url::{ParseError, Url};
use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;

#[derive(Debug)]
enum UriError {
    ArgumentError(String),
    FormatError(ParseError),
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn collection_uri(hatena_id: &str, blog_id: &str) -> Result<Url, UriError> {
    if hatena_id.is_empty() {
        return Err(UriError::ArgumentError("hatena_id".to_owned()));
    }
    if blog_id.is_empty() {
        return Err(UriError::ArgumentError("blog_id".to_owned()));
    }
    let base_url = "https://blog.hatena.ne.jp";
    let url_string = format!("{base_url}/{hatena_id}/{blog_id}/atom/entry",
                             base_url = base_url,
                             hatena_id = hatena_id,
                             blog_id = blog_id);
    Url::parse(&url_string).map_err(|e| UriError::FormatError(e))
}

fn get() -> Response {
    let hatena_id = env::var("HATENA_USERNAME").unwrap();
    let blog_id = env::var("HATENA_BLOG_ID").unwrap();
    let api_key = env::var("HATENA_API_KEY").unwrap();
    let url = collection_uri(&hatena_id, &blog_id).unwrap();

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut headers = Headers::new();
    let basic = Basic {
        username: hatena_id.to_owned(),
        password: Some(api_key.to_owned()),
    };
    headers.set(Authorization(basic));
    let response = client.get(url).headers(headers).send().unwrap();
    // println!("Response: {}", response.status);
    // println!("Headers:\n{}", response.headers);
    // io::copy(&mut response, &mut io::stdout()).unwrap();
    response
}

fn parse_xml(response: Response) -> () {
    let parser = EventReader::new(response);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let OwnedName { local_name, .. } = name;
                println!("{}<{}", indent(depth), local_name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn main() {
    let response = get();
    parse_xml(response);
}
