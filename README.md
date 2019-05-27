# Securipi

### Install rust on your pi

Follow the instructions to install rustup at https://www.rust-lang.org/learn/get-started _on your raspberry pi_.

Currently:
```sh
curl https://sh.rustup.rs -sSf | sh
```

Then run `rustup` to install the latest version of the rust toolchain and start a new shell so that modifications to your environment have taken effect.

### Installing dependencies and running the application

Copy the example .env file and provide values for your twilio account:
```sh
cp .env.example .env
```

**Warning, the first compile takes hours on a pi**

```sh
cargo run
```
