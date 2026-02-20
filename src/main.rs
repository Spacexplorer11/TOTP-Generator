use colored::Colorize;
use dotenvy::dotenv;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{io, thread};
use totp_rs::{Algorithm, Secret, TOTP, TotpUrlError};

fn main() {
    let title_screen_art = r"___  __  ___  __      __   ___       ___  __       ___  __   __
 |  /  \  |  |__)    / _` |__  |\ | |__  |__)  /\   |  /  \ |__)
 |  \__/  |  |       \__> |___ | \| |___ |  \ /~~\  |  \__/ |  \ ";
    println!("{}", title_screen_art);
    if !dotenv().is_ok() {
        println!(
            "{} .env file could not be loaded, saved secrets will not be used.",
            "Warning:".yellow()
        );
    } else {
        dotenv().ok();
    }
    println!(
        "If you wish to use multiple tokens, please read the README here: https://github.com/Spacexplorer11/TOTP-Generator/blob/main/README.md"
    );
    println!(
        "{} This program {} after you press Enter. Please ensure you have no information you may need later in this terminal session.",
        "Warning:".yellow(),
        "clears your terminal".red()
    );
    println!("To abort/quit use CTRL+C");
    println!("If you wish to only use stored / loaded secrets, please enter 'N'");
    println!("Enter your secret (Base32 - Only include A-Z & 2-7):");
    let mut secret_str = String::new();

    io::stdin()
        .read_line(&mut secret_str)
        .expect("An error occurred while taking input");

    let mut errors: Vec<String> = Vec::new();

    let secret_str = secret_str.trim().replace(" ", "").to_uppercase();
    let mut secrets: Vec<(String, Vec<u8>)> = Vec::new();
    if secret_str != "N" {
        // secret_str is made uppercase 2 lines above so this works
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
            Err(_) => {
                errors.push(format!(
                    "{} environment variable '{}' was ignored because it does not contain a valid TOTP secret.", "Error:".red(),
                    var.0
                ));
            }
        }
    }

    let mut totps: Vec<(String, TOTP)> = Vec::new();

    for secret in secrets {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.1);
        match totp {
            Ok(totp) => totps.push((secret.0, totp)),
            Err(TotpUrlError::SecretSize(size)) => errors.push(format!("{} secret '{}' has an invalid size of {} bits. (It must be at least 26 Base32 characters)", "Error:".red(), secret.0, size)),
            Err(_) => errors.push(format!("{} secret '{}' could not be used.","Error:".red(), secret.0))
        };
    }

    if totps.is_empty() {
        println!("{}",
            "No secrets were found/could be used, exiting. Please make sure the env variables begin with 'TOTP_'".red()
        );
        if !errors.is_empty() {
            println!();
            println!("{}", "------------------------".red());
            println!("{}", "Errors:".red());
            for error in &errors {
                println!("{}", error);
            }
        }
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
        if !errors.is_empty() {
            println!();
            println!("{}", "------------------------".red());
            println!("{}", "Errors:".red());
            for error in &errors {
                println!("{}", error);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
