# lecture
A lightweight bot for searching and downloading lectures.

## Environments

Create a .env file with this default envs

| Key               | Value                                |
|-------------------|--------------------------------------|
| RUST_LOG          | debug                                |
| RUST_BACKTRACE    | 1                                    |
| DATABASE_USER     | postgres                             |
| DATABASE_PASSWORD | postgres                             |
| DATABASE_NAME     | postgres                             |
| DATABASE_HOST     | localhost                            |
| REDIS_URL         | redis://localhost/0                  |
| TELOXIDE_TOKEN    | 123123123:blablabla (telegram token) |

## How to execute
```shell
docker-compose up -d
cargo run --release
```