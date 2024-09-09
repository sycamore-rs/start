# Sycamore Starter Template

This is a bare-bones quickstart template for [Sycamore](https://github.com/sycamore-rs/sycamore).

## Requirements

You should have an updated installation of the following tools.

- [Rust](https://rust-lang.org)
- [Trunk](https://trunkrs.dev)

You also need the `wasm32-unknown-unknown` target installed. If you used `rustup` to install Rust, you can simply run the following command to install the right toolchain.

```sh
rustup target add wasm32-unknown-unknown
```

## Usage

Run the following commands to clone the template and start the server.

```sh
# Replace 'hello-world' with whatever you want to name your project
git clone https://github.com/sycamore-rs/start hello-world
cd hello-world/
trunk serve
```

Now open your browser to <localhost:8080> to see your brand new Sycamore app!
