//! Command line arguments
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::page::PageSize;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Page title (max. 64 characters)
    #[arg(short, long, default_value = "PaperAge")]
    pub title: String,

    /// Output file name. Use - for STDOUT.
    #[arg(short, long, default_value = "out.pdf")]
    pub output: PathBuf,

    /// Paper size
    #[arg(short = 's', long, default_value_t = PageSize::A4)]
    pub page_size: PageSize,

    /// Overwrite the output file if it already exists
    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    /// Draw a grid pattern for debugging layout issues
    #[arg(short, long, default_value_t = false)]
    pub grid: bool,

    /// Print out the license for the embedded fonts
    #[arg(long, default_value_t = false, exclusive = true)]
    pub fonts_license: bool,

    /// Verbose output for debugging
    #[clap(flatten)]
    pub verbose: Verbosity,

    /// Disable drawing of footer
    #[arg(short, long, default_value_t = true)]
    pub no_footer: bool,

    /// Input is raw age (i.e.: from encrypting input to Yubikey). Encode: cat in.txt | rage $(echo $identities) -e -a | paper-age --force --no-footer --title=YubikeyIds --age-input; Decode: cat out.age | rage -d -i ~/git/secrets/identities
    #[arg(short, long, default_value_t = false)]
    pub age_input: bool,

    /// Identities to encrypt to. Use when encrypting to Yubikey identities.
    #[arg(short, long, default_value = "")]
    pub identities: String,

    /// The path to the file to read. Defaults to standard input. Max. ~1.9KB. Usage: cat Justfile | rage -r age1yubikey1qf23uc3g4e9gv4fnn8zv80s3j6s0wghqgplhzz7lr9skhelfxxsy6lk3w9g -r age1yubikey1qgve538jrxtdv6al9aqr4f052f6mr9h24zu2l4440jk2e3kzrmywyj5cl39 -r age1yubikey1qf0ngpeua8h7pauzm68h5wa0x2q3vx59mgxl4cvlhkhj026fc8k358fjeuw -e -a | ./target/release/paper-age --force --title=YubikeyIds --age-input --no-footer --identities=' -r age1yubikey1qf23uc3g4e9gv4fnn8zv80s3j6s0wghqgplhzz7lr9skhelfxxsy6lk3w9g -r age1yubikey1qgve538jrxtdv6al9aqr4f052f6mr9h24zu2l4440jk2e3kzrmywyj5cl39 -r age1yubikey1qf0ngpeua8h7pauzm68h5wa0x2q3vx59mgxl4cvlhkhj026fc8k358fjeuw'
    pub input: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        super::Args::command().debug_assert()
    }

    #[test]
    fn test_args() {
        let args = Args::parse_from([
            "paper-age",
            "-f",
            "-g",
            "--title",
            "Hello",
            "--output",
            "test.pdf",
            "input.txt",
        ]);
        assert!(args.force);
        assert!(args.grid);
        assert_eq!(args.title, "Hello");
        assert_eq!(args.output.to_str().unwrap(), "test.pdf");
        assert_eq!(args.input.unwrap().to_str().unwrap(), "input.txt");
    }

    #[test]
    fn test_defaults() {
        let args = Args::parse_from(["paper-age"]);
        assert_eq!(args.title, "PaperAge");
        assert_eq!(args.output.to_str().unwrap(), "out.pdf");
        assert_eq!(args.input, None);
        assert!(!args.force);
    }

    #[test]
    fn test_fonts_license() {
        let args = Args::parse_from(["paper-age", "--fonts-license"]);
        assert!(args.fonts_license);
    }

    #[test]
    fn test_fonts_license_conflict() -> Result<(), Box<dyn std::error::Error>> {
        let result = Args::try_parse_from(["paper-age", "--fonts-license", "--grid"]);

        assert!(result.is_err());

        Ok(())
    }
}
