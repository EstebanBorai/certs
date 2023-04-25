#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to fetch {0}")]
    FetchFailed(String),
    #[error("Failed to write to peer on {0}")]
    StreamWrite(String),
    #[error("Failed to retrieve peer certificate")]
    RetrievePeerCertificate,
    #[error("No certificate available on {0}")]
    CertificateNotFound(String),
    #[error("Connector Configuration Error. {0:?}")]
    ConnectorConfiguration(native_tls::Error),
    #[error("Failed to retrieve DER Encoded Value. {0:?}")]
    DerEncodedRetrieval(native_tls::Error),
    #[error("Only HTTPS URLs are supported. Provided {0}")]
    HttpsOnly(String),
    #[error("Invalid URL Provided. {0}")]
    InvalidUrl(String),
}

pub type Result<T> = std::result::Result<T, Error>;
