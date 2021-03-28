# External libraries

- crates.io > npm equivalent
- Can add dependencies install from git, local, workspace (monorepo)
- Private registries: https://doc.rust-lang.org/nightly/cargo/reference/registries.html
- `Cargo.lock` > `package-lock.json`, `Gemfile.lock`

## Common libraries

| What               | Which                    |
| ------------------ | ------------------------ |
| Format parsing     | **serde**                |
| CLI parsing        | **clap**                 |
| Error handling     | anyhow, thiserror        |
| Network request    | reqwest, hyper           |
| Web framework      | tower, rocket, actix-web |
| ORM, query builder | diesel                   |
| Async runtime      | tokio                    |

## Objectives

- Import external libraries
- Add external libraries

## Exercise

Guided example on adding external libraries.

We will be using two libraries to create our profile printer.

- `serde` and `serde_json` to deserialise JSON into structs
- `clap` to parse CLI arguments.

To begin,

- Add `serde`, `clap` as libraries to `Cargo.toml` (you just have to uncomment)
- import the traits provided by the libraries (`clap::Clap`, `serde::{Deserialize, Serialize}`)
- Annotate structs with `Serialize`, `Deserialize`
- Fill out `main`
