use dotenv::dotenv;
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
        "Warning: This program clears your terminal after click enter, please ensure you have no information you may need later in this terminal session"
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
    if secret_str.to_lowercase() != "n" {
        secrets.push((
            String::from("Untitled"),
            Secret::Encoded(secret_str)
                .to_bytes()
                .expect("Your secret was invalid."),
        ));
    }

    for var in env::vars() {
        let secret = Secret::Encoded(var.1).to_bytes();
        match secret {
            Ok(secret) => secrets.push((var.0, secret)),
            _ => (),
        }
    }

    let mut totps: Vec<(String, TOTP)> = Vec::new();

    for secret in secrets {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.1);
        match totp {
            Ok(totp) => totps.push((secret.0, totp)),
            _ => (),
        };
    }

    let mut i: u8 = 30;

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
        if i > 0 {
            i -= 1;
        } else {
            i = 30
        }
    }
}
