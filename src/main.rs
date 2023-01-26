use std::io::Write;

use age::armor::ArmoredWriter;
use age::armor::Format::AsciiArmor;
use age::secrecy::Secret;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Plaintext to encrypt
    #[arg(short = 't', long)]
    plaintext: String,

    /// Passphrase
    #[arg(short, long)]
    passphrase: String,
}

fn main() {
    let args = Args::parse();

    let plaintext = args.plaintext.as_bytes();
    let passphrase = args.passphrase.as_str();

    // Encrypt the plaintext to a ciphertext using the passphrase...
    let encrypted: String = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

        let mut encrypted = vec![];

        let armored_writer = match ArmoredWriter::wrap_output(&mut encrypted, AsciiArmor) {
            Ok(w) => w,
            Err(error) => panic!("Error: {:?}", error),
        };

        let mut writer = match encryptor.wrap_output(armored_writer) {
            Ok(w) => w,
            Err(error) => panic!("Error: {:?}", error),
        };

        match writer.write_all(plaintext) {
            Ok(()) => (),
            Err(error) => panic!("Error: {:?}", error),
        }

        let output = match writer.finish().and_then(|armor| armor.finish()) {
            Ok(e) => e.to_owned(),
            Err(error) => panic!("Error: {:?}", error),
        };

        match std::string::String::from_utf8(output) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    };

    println!("{}", encrypted);
}
