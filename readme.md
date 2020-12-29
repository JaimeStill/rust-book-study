# Rust Book Projects

Projects built working through the official [Rust Book](https://doc.rust-lang.org/book/). 

All directories, except for [hello_world](./hello_world), are created with cargo and can be executed by running `cargo run`. 

To run [hello_world](./hello_world), you must first run `rustc main.rs --out-dir ./target` to generate the **main** executable, then you can run `./target/main` directly from the command line.

> See [Rust Docs](https://www.rust-lang.org/learn).

## Cross-Compiling from Linux to Windows

My Rust environment is setup in WSL2 on Ubuntu. In order to target builds for Windows from Ubuntu, the following needs to be executed once:

```bash
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
sudo apt install mingw-w64
```

Once this is complete, whenever you want to target a build for windows, simply build using:

```bash
cargo build --target x86_64-pc-windows-gnu
```

It will generate output to `./target/x86_64-pc-windows-gnu/debug/`. If you copy this directory into your Windows filesystem, you can run the generated executable natively without the need for any Rust infrastructure installed.

> Credit to this [StackOverflow answer](https://stackoverflow.com/a/62853319/3971984) as well as this random internet friend aliased as Screwtape: [Building Rust programs for Windows on Debian Linux](https://zork.net/~st/jottings/rust-windows-and-debian.html).