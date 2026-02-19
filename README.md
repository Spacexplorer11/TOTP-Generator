# TOTP Generator
![Hackatime](https://hackatime-badge.hackclub.com/U08D22QNUVD/TOTP-Generator)![GitHub commit activity](https://img.shields.io/github/commit-activity/m/spacexplorer11/TOTP-Generator)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/spacexplorer11/TOTP-Generator/total)

This is a CLI tool which allows you to securely generate Time-Based One-time Passcodes for any purpose you may need!

## How to use?
1. Download the binary for your OS from the [latest release](https://github.com/Spacexplorer11/TOTP-Generator/releases/latest).
2. Run the binary and input your secret!

### Want to use multiple secrets?
If you want to use multiple secrets, you must set them as environment variables.  
You can do this by running the following command for your OS with your data for however many secrets you want. 

>[!Note]
> You must include the `TOTP_` at the start of your env variable as otherwise it will be ignored

#### macOS / Linux:
`export TOTP_MY_SECRET=MY_SPECIAL_SECRET`
#### Windows (Command Prompt):
`set TOTP_MY_SECRET=MY_SPECIAL_SECRET`
#### Windows (PowerShell):
`$env:TOTP_MY_SECRET="MY_SPECIAL_SECRET"`
After you close the terminal session, your secrets will disappear and you will need to re-set them when you want to use them again

>[!Tip]
> Want your secrets to persist across terminal sessions?
> Make a .env file in the same directory you downloaded TOTP-Generator and put all your secrets in there, like this!
> `TOTP_MY_SECRET=MY_SPECIAL_SECRET`
>
> **Security Warning:** The .env file stores TOTP secrets in **plaintext** (unencrypted). To protect your secrets:
> - Restrict file permissions: `chmod 600 .env` (macOS/Linux) or right-click → Properties → Security tab (Windows)
> - Never commit .env to version control (already in .gitignore)


## Local development
1. Clone the repo
2. Run `cargo run`

## Contributions
Contributions are always welcome!