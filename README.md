# ptera-api

## Development Environment

```shell
cargo install cargo-lambda
```

## Build

```shell
cargo lambda build
// Release build
cargo lambda build --release
// ARM64 build
cargo lambda build --arm64
```

## Deploy

```shell
cargo lambda deploy --iam-role arn:aws:iam::<***> --env-file .env
```