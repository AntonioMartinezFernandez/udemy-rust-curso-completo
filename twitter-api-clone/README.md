# Twitter API clone

Twitter 'like' API.

## Getting Started

Install cargo watch

```
cargo install cargo-watch
```

Start database

```
docker compose up
```

Start server

```
cargo watch -x run
```

Endpoints (accessibles at http://localhost:8000)

```
GET /tweets
POST /tweet
GET /tweet/:id
DELETE /tweet/:id
GET /tweet/:id/likes
POST /tweet/:id/like
DELETE /tweet/:id/like
```

## Extra

### Diesel

IMPORTANT for Windows users:

- Download PosgreSQL v15.3 binaries from https://www.enterprisedb.com/download-postgresql-binaries
- Copy next files from **ZIP file**...

  - **libpq.lib** into _C:/Users/USER_NAME/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/x86_64-pc-windows-msvc/lib_ folder
  - **libcrypto-1_1.dll**, **libcrypto-3-x64.dll**, **libiconv-2.dll**, **libintl-9.dll**, **libpq.dll**, **libssl-1_1.dll**, **libssl-3-x64.dll** and **libwinpthread-1.dll** into _C:/Users/USER_NAME/.cargo/bin_

Install Diesel CLI:

```
cargo install diesel_cli --no-default-features --features postgres
```

Create Diesel setup -create and applicate initial migration- (from root folder of the project)

_IMPORTANT_: before execute this command, check the postgresql is running, and _.env_ file is created with the _DATABASE_URL_ variable

```
diesel setup
```

Create new migration

```
diesel migration generate database_name
```

Apply migrations

```
diesel migration run
```

### Containers

Build & Run docker image

```
docker build -t twitter-api-clone -f ./Dockerfile .
docker run -p 8000:8000 twitter-api-clone
```

## Resources

- [Actix Web - HTTP framework for Rust](https://crates.io/crates/actix-web)
- [Serde JSON - Serializing and deserializing Rust data structures](https://crates.io/crates/serde_json)
- [Diesel- ORM and Query Builder for Rust](https://diesel.rs/guides/getting-started)
