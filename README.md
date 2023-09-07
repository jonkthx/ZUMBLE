# Zumble

A mumble server for FiveM.

Goal is to have an external server handling voice chat for FiveM servers instead of using the built-in voice chat.

**This is a work in progress. Use it at your own risk.**

## Features

 * 100% compatible with FiveM -> drop in replacement for client natives
 * Http api to mimic server side native calls (MumbleIsPlayerMuted / MumbleSetPlayerMuted / MumbleCreateChannel)
 * Performance (multithreaded server, separated from game network)
 * Can be installed on a separate machine
 * prometheus metrics

## Installation

 1. Clone this repository
 2. Build the server using cargo: `cargo build --release`
 3. Locate your FiveM FXserver and open both certificate files server-tls.crt (private key) and server-tls.key (certificate) in notepad or any other text editor
 4. Create certificates directly in the ZUMBLE directory. To create both certificate I use `nano key.pem` and copy-paste the content of server-tls.crt then do the same `nano cert.pem` with server-tls.key

## How to run it

 1. Do this command `tmux`
 2. Locate Executable and go in the directory with `cd` In my case it's zumble\ZUMBLE
 3. Use this command `./target/release/zumble --https --http-password <HTTP_PASSWORD>`
 4. Detach from the session by pressing `Ctrl-b` followed by `d`.
 5. If you go back to ssh again and want to reattach to the tmux session `tmux attach`


Future versions will include pre-built binaries in release section of GitHub.

## Usage

```
USAGE:
    zumble [OPTIONS] --http-password <HTTP_PASSWORD>

OPTIONS:
        --cert <CERT>
            Path to the certificate file for the TLS certificate [default: cert.pem]

    -h, --http-listen <HTTP_LISTEN>
            Listen address for HTTP connections for the admin api [default: 0.0.0.0:8080]

        --help
            Print help information

        --http-password <HTTP_PASSWORD>
            Password for the http server api basic authentification

        --http-user <HTTP_USER>
            User for the http server api basic authentification [default: admin]

        --https
            Use TLS for the http server (https), will use the same certificate as the mumble server

        --key <KEY>
            Path to the key file for the TLS certificate [default: key.pem]

    -l, --listen <LISTEN>
            Listen address for TCP and UDP connections for mumble voip clients (or other clients
            that support the mumble protocol) [default: 0.0.0.0:64738]

    -V, --version
            Print version information
```

## Credits

  * [mumble-protocol](https://github.com/Johni0702/rust-mumble-protocol) for the crypt / decrypt algorithm of the mumble protocol, it was rewritten here to work on pure rust library (no openssl)
