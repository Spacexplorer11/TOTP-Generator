use dotenv::dotenv;
use std::path::PathBuf;
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
        "Warning: This program clears your terminal after you enter the key, please ensure you have no information you may need later in this terminal session"
    );
    println!("To abort use CTRL+C");
    println!("Enter your secret (Base32 - Only include A-Z & 2-7):");
    let mut secret_str = String::new();

    io::stdin()
        .read_line(&mut secret_str)
        .expect("An error occurred while taking input");

    let secret_str = secret_str.trim().replace(" ", "").to_uppercase();
    let secret = Secret::Encoded(secret_str)
        .to_bytes()
        .expect("Invalid Base32 secret");

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret).unwrap();

    let mut i: u8 = 30;

    loop {
        let token = totp.generate_current().unwrap();
        clearscreen::clear().expect("failed to clear screen");
        println!("{}", title_screen_art);
        println!(
            "Current token: {} - {} seconds",
            token,
            30 - (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                % 30)
        );
        thread::sleep(Duration::from_secs(1));
        if i > 0 {
            i -= 1;
        } else {
            i = 30
        }
    }
}
