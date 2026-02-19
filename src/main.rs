use dotenvy::dotenv;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{io, thread};
use totp_rs::{Algorithm, Secret, TOTP};

fn main() {
    let title_screen_art = r"___  __  ___  __      __   ___       ___  __       ___  __   __
 |  /  \  |  |__)    / _` |__  |\ | |__  |__)  /\   |  /  \ |__)
 |  \__/  |  |       \__> |___ | \| |___ |  \ /~~\  |  \__/ |  \ ";
    println!("{}", title_screen_art);
    if !dotenv().is_ok() {
        println!("Warning: .env file could not be loaded, saved secrets will not be used.");
    } else {
        dotenv().ok();
    }
    println!(
        "If you wish to use multiple tokens, please read the README here: https://github.com/Spacexplorer11/TOTP-Generator/blob/main/README.md"
    );
    println!(
        "Warning: This program clears your terminal after you press Enter. Please ensure you have no information you may need later in this terminal session."
    );
    println!("To abort/quit use CTRL+C");
    println!("If you wish to only use stored / loaded secrets, please enter 'N'");
    println!("Enter your secret (Base32 - Only include A-Z & 2-7):");
    let mut secret_str = String::new();

    io::stdin()
        .read_line(&mut secret_str)
        .expect("An error occurred while taking input");

    let secret_str = secret_str.trim().replace(" ", "").to_uppercase();
    let mut secrets: Vec<(String, Vec<u8>)> = Vec::new();
    if !secret_str.eq_ignore_ascii_case("n") {
        secrets.push((
            String::from("Untitled"),
            Secret::Encoded(secret_str)
                .to_bytes()
                .expect("Your secret was invalid."),
        ));
    }

    for var in env::vars() {
        if !var.0.starts_with("TOTP_") {
            continue;
        };
        let env_secret = var.1.trim().replace(" ", "").to_uppercase();
        let secret = Secret::Encoded(env_secret).to_bytes();
        match secret {
            Ok(secret) => {
                secrets.push((String::from(var.0.strip_prefix("TOTP_").unwrap()), secret))
            }
            Err(e) => {
                eprintln!(
                    "Warning: environment variable '{}' was ignored because it does not contain a valid TOTP secret: {:?}",
                    var.0, e
                );
            }
        }
    }

    let mut totps: Vec<(String, TOTP)> = Vec::new();

    for secret in secrets {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.1);
        match totp {
            Ok(totp) => totps.push((secret.0, totp)),
            Err(e) => eprintln!("Warning: secret '{}' could not be used: {:?}", secret.0, e),
        };
    }

    if totps.is_empty() {
        println!(
            "No secrets were found/could be used, exiting. Please make sure the env variables begin with 'TOTP_'"
        );
        return;
    }
    loop {
        clearscreen::clear().expect("failed to clear screen");
        println!("{}", title_screen_art);
        println!("Missing a token? Check your environment variables to see if it was set properly");
        for totp in &totps {
            let token = totp.1.generate_current().unwrap();
            println!(
                "{}: {} - {} seconds",
                totp.0,
                token,
                30 - (SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    % 30)
            );
        }
        thread::sleep(Duration::from_secs(1));
    }
}
