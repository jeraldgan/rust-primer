## Production

- Containerise build
- CI: tests, fmets, building in alpine
- Bonus: cargo t, clippy, audit
- Doc generation, testing your docs
- Release: targbench, fuzzing, quickcheck
- Bonus: Error handling ecosystem, error reporting and tracking

---

## Objectives

- Look at objectives again

## Questions

- Was it easy to get started with/install?
- Was the compiler helpful?
- Was tooling (cargo, rustfmt, clippy) good?
- Was testing code easy?
- Were the docs good?
- Did you have confidence in your program when it compiled?
- What did you like or dislike about this taste of Rust?

## Good stuff

- Developer productivity
- Compiler writes code for you
- If it compiles, it works
- Modern tooling
- Performant: all the things you have seen today have zero runtime cost
- Hard to shoot yourself in the foot = even better for newer programmers
- Able to sleep well at night

## Bad stuff

- Learning curve
- Relativly young ecosystem
- Compile times (vs Go)

## Good? stuff

- No runtime
- Bright future

## What do people use Rust for

- CLI tools
- WASM
- Web (no Django/Rails/Spring equivalent yet)
- APIs
- Games
- Performance-sensitive code
- Libraries (to C, JS)
- Embedded

## Who is using Rust?

- Mozilla

  - Firefox (https://bugzilla.mozilla.org/show_bug.cgi?id=1135640)

- Discord

  - Read states service (https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f)

- Dropbox

  - File sync engine (https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine)

- AWS

  - Virtual machine monitor (https://firecracker-microvm.github.io/)
  - Container OS (https://aws.amazon.com/bottlerocket/)

- Microsoft

  - Started an internal Rust team for Windows (https://cloudblogs.microsoft.com/opensource/2021/02/08/microsoft-joins-rust-foundation/)
  - https://github.com/microsoft/windows-rs

- Google

  - Android Bluetooth modules (https://opensource.googleblog.com/2021/02/google-joins-rust-foundation.html)

- curl

  - Adding a HTTP backend https://daniel.haxx.se/blog/2020/10/09/rust-in-curl-with-hyper/

- Figma

  - Multiplayer server (https://www.figma.com/blog/rust-in-production-at-figma/)

- Cloudflare

  - Cloudflare workers, WASM (https://blog.cloudflare.com/introducing-wrangler-cli/)
  - Cron parser (https://blog.cloudflare.com/using-one-cron-parser-everywhere-with-rust-and-saffron/)

- Deliveroo

  - Rider dispatch (https://deliveroo.engineering/2019/02/14/moving-from-ruby-to-rust.html)

- 1Password

  - All backend services (https://blog.1password.com/1passwordx-december-2019-release/)

- Linux
  - Drivers (https://lwn.net/Articles/829858/)

## Next steps

- Learn Rust
- See what's going on in Rustland
- Rust at workplace?
