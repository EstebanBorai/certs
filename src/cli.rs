use clap::{Args, Parser};

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
    pub fn exec(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Show(opts) => {
                println!("{opts:?}");
            }
        }

        Ok(())
    }
}
