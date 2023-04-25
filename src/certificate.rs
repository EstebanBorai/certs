use std::fmt::Display;

use x509_parser::x509::X509Version;

#[derive(Clone, Debug)]
pub struct Certificate {
    pub issuer: String,
    pub subject: String,
    pub val_not_before: String,
    pub val_not_after: String,
    pub version: X509Version,
}

impl Certificate {
    pub fn from_der(bytes: &[u8]) -> Self {
        use x509_parser::prelude::*;

        if let Ok((_, cert)) = X509Certificate::from_der(bytes) {
            let validity = cert.validity.clone();
            let val_not_before = validity.not_before.to_datetime().to_string();
            let val_not_after = validity.not_after.to_datetime().to_string();
            let certificate = Certificate {
                issuer: cert.issuer().to_string(),
                subject: cert.subject().to_string(),
                val_not_after,
                val_not_before,
                version: cert.version(),
            };

            return certificate;
        }

        panic!("Failed to parse certificate");
    }
}

impl Display for Certificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issuer: {}\nSubject: {}\nNot Before: {}\nNot After: {}\nVersion: {}",
            self.issuer, self.subject, self.val_not_before, self.val_not_after, self.version
        )
    }
}
