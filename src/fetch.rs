use std::io::{Read, Write};
use std::net::TcpStream;

use native_tls::TlsConnector;
use url::Url;

use crate::certificate::Certificate;

const SSL_PORT: u16 = 443;
const HTTP_GET_REQUEST: &[u8] = b"GET / HTTP/1.0\r\n\r\n";

pub struct Fetch {
    domain: String,
}

impl Fetch {
    pub fn new(url: &Url) -> Self {
        if url.scheme() != "https" {
            panic!("Only HTTPS is Supported");
        }

        let domain = url.domain().unwrap();

        Self {
            domain: domain.to_string(),
        }
    }

    pub fn get_certificate_pem(&self) -> Result<Certificate, Box<dyn std::error::Error>> {
        let connector = TlsConnector::builder().build()?;
        let stream = TcpStream::connect(self.addr()).unwrap();
        let mut stream = connector.connect(&self.domain, stream).unwrap();

        stream.write_all(HTTP_GET_REQUEST).unwrap();
        let bytes = stream
            .peer_certificate()
            .unwrap()
            .unwrap()
            .to_der()
            .unwrap();
        let certificate = Certificate::from_der(bytes.as_slice());

        Ok(certificate)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.domain, SSL_PORT)
    }
}
