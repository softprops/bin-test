extern crate hyper;

extern crate hyper_native_tls;
//extern crate hyper_rustls;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
//use hyper_rustls::TlsClient;

fn main() {
    println!("sup google");
    let _ = std::io::copy(
        &mut hyper::Client::with_connector(
            HttpsConnector::new(
                NativeTlsClient::new().unwrap()
            )
        ).get("https://www.google.com/").send().unwrap(), &mut std::io::stdout()
    );
}
