use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Certificate {
    pub issuer: String,
    pub subject: String,
    pub val_not_before: String,
    pub val_not_after: String,
}

impl Certificate {
    pub fn from_der<'a>(bytes: &'a [u8]) -> Self {
        use x509_parser::prelude::*;

        if let Ok((_, cert)) = X509Certificate::from_der(bytes) {
            let issuer = cert.issuer.to_string();
            let subject = cert.subject.to_string();
            let validity = cert.validity.clone();
            let val_not_before = validity.not_before.to_datetime().to_string();
            let val_not_after = validity.not_after.to_datetime().to_string();

            return Self {
                issuer,
                subject,
                val_not_after,
                val_not_before,
            };
        }

        panic!("Failed to parse certificate");
    }
}

impl Display for Certificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Issuer: {}\nSubject: {}\nNot Before: {}\nNot After: {}",
            self.issuer, self.subject, self.val_not_before, self.val_not_after
        )
    }
}
