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
