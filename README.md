# Micro-RDK Webhook Template

## Configure Device with Micro-RDK

- Rust Compiler with [`rustup`](https://www.rust-lang.org/tools/install)
- `cargo install cargo-espflash`
- `cargo install cargo-espup`

## Deploy Webhook with Fly.io

Follow steps 1, 2, and 3 to use fly.io's [`flyctl`](https://fly.io/docs/hands-on/install-flyctl/) tool.

1. `$ cd <my-webhook-project>`
2. `$ fly launch`; this is only done once, for updating apps use `fly deploy`

## Add Webhook Configs to Robot

Add the following attributes to your board's config

> Note: The template's SDK scripts, (i.e. `hook.py`, `hook.go`) assume your board is named `board`.

```json
{
    "webhook": "<my-webhook-endpoint>",
    "webhook-secret": "<my-robot-location-secret>",
    "pins": [
    ... etc
    ]
}
```
