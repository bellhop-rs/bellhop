Bellhop
=======

Asset reservation system.

## First Time Setup

This project requires the lastest Rust nightly.

You'll also need to have diesel's cli tool installed to be able to work with the
database:

```
cargo install diesel_cli --no-default-features --features postgres
```

#### Installing Postgres

```
sudo apt install postgresql-9.5 postgresql-client libpq-dev
sudo -u postgres createuser -P bellhop
sudo -u postgres createdb -O bellhop bellhop
```

## Running

After performing the setup, running the server is pretty simple:

```
cargo run
```


## Unsorted Notes

### Generating API Documentation

```
npm install redoc-cli
export REDOC_CLI="$(pwd)/node_modules/redoc-cli/index.js"
```
