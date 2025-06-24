# Rust REST API

This project was created to learn more about Rust programming language, and the target here is to have a working simple raw REST API written in Rust.

## Dependencies

```
docker compose up -d
```

## How to run

Use cargo to build and run at the terminal:

```
cargo build
```
And then
```
cargo run
```

It will use the `.env` file to set the PORT, or if it's not defined, the port `8080`  will be used.
