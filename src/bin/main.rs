use std::str::FromStr;

use clap::{Args, Parser};
use url::Url;

use libcerts::error::{Error, Result};
use libcerts::fetch::Fetch;

const ABOUT: &str = r#"Extract server certificates"#;

#[derive(Args, Clone, Debug)]
pub struct ShowOpts {
    /// URL to fetch certificate from
    pub(crate) url: String,
}

#[derive(Parser)]
#[command(bin_name = "certs")]
#[command(next_line_help = true)]
#[command(name = "certs", author, version, about, long_about = Some(ABOUT))]
pub enum Cli {
    /// Shows the certificate for the provided server domain
    Show(ShowOpts),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self {
            Self::Show(opts) => {
                let url = Url::from_str(&opts.url)
                    .map_err(|_e| Error::InvalidUrl(opts.url.to_string()))?;
                let fetch = Fetch::new(&url)?;

                println!("Fetching certificate from: {}", fetch.addr());

                let cert = fetch.certificate_pem()?;

                println!("\n{}", cert);
            }
        }

        Ok(())
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    cli.exec()?;
    Ok(())
}
