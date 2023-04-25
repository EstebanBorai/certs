use std::io::Write;
use std::net::TcpStream;

use native_tls::TlsConnector;
use url::Url;

use crate::certificate::Certificate;
use crate::error::{Error, Result};

const SSL_PORT: u16 = 443;
const HTTP_GET_REQUEST: &[u8] = b"GET / HTTP/1.0\r\n\r\n";

pub struct Fetch {
    domain: String,
}

impl Fetch {
    pub fn new(url: &Url) -> Result<Self> {
        if url.scheme() != "https" {
            return Err(Error::HttpsOnly(url.to_string()));
        }

        let Some(domain) = url.domain() else {
            return Err(Error::InvalidUrl(url.to_string()));
        };

        Ok(Self {
            domain: domain.to_string(),
        })
    }

    pub fn certificate_pem(&self) -> Result<Certificate> {
        let connector = TlsConnector::builder()
            .build()
            .map_err(Error::ConnectorConfiguration)?;
        let stream =
            TcpStream::connect(self.addr()).map_err(|_e| Error::FetchFailed(self.addr()))?;
        let mut stream = connector
            .connect(&self.domain, stream)
            .map_err(|_e| Error::FetchFailed(self.addr()))?;

        stream
            .write_all(HTTP_GET_REQUEST)
            .map_err(|_e| Error::StreamWrite(self.addr()))?;

        let Some(cert_bytes) = stream
            .peer_certificate()
            .map_err(|err| {
                eprint!("{err:?}");
                Error::RetrievePeerCertificate
            })? else {
                return Err(Error::CertificateNotFound(self.addr()));
            };
        let der_encoded = cert_bytes.to_der().map_err(Error::DerEncodedRetrieval)?;
        let certificate = Certificate::from_der(&der_encoded);

        Ok(certificate)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.domain, SSL_PORT)
    }
}
