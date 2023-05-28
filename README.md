# zero2template
[![CI](https://github.com/1024bees/zero2template/actions/workflows/ci.yml/badge.svg)](https://github.com/1024bees/zero2template/actions/workflows/ci.yml)

A template, to be used with [cargo-generate], to create a small web service. Much of the code is based off the approaches suggested in the [zero2production](https://www.zero2prod.com/index.html) book.


To generate a project using this template:

```bash
cargo generate -a 1024bees/zero2template
```

After running the command, there will be a few prompts:
- `Project Name`: Name of the crate.
- `Add sqlx, with sqlite as the database driver?`: Will add middleware and configuration scaffolding to interact with an sqlite database


## Relevant reading, libraries

[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[`axum`]: https://github.com/tokio-rs/axum
[`sqlx`]: https://github.com/launchbadge/sqlx
[zero2production]: https://www.zero2prod.com/index.html

