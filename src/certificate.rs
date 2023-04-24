pub struct Certificate(String);

impl Certificate {
    pub fn from_der<'a>(bytes: &'a [u8]) {
        use x509_parser::prelude::*;

        let res = X509Certificate::from_der(bytes);
        match res {
            Ok((rem, cert)) => {
                println!("REM: {:?}", rem);
                println!("CERT: {:?}", cert);
            }
            _ => panic!("x509 parsing failed: {:?}", res),
        }
    }
}
