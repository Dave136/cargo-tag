<div>
  <h1 align="center">cargo tag</h1>
  <h4 align="center">
    Cargo plugin to bump crate's versions and Git tag them for release
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/cargo-tag.svg)](https://crates.io/crates/cargo-tag)
  [![Documentation](https://docs.rs/cargo-tag/badge.svg)](https://docs.rs/cargo-tag)
  ![Build](https://github.com/EstebanBorai/cargo-tag/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/cargo-tag/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/cargo-tag/workflows/fmt/badge.svg)

</div>

Cargo plugin to bump crate's versions and Git tag them
for release.

"cargo tag" helps to automate the process of bumping versions
similar to how "npm version" does.

When bumping versions with "cargo tag", the
Cargo.toml and Cargo.lock files are updated with the new version, then a Git
commit and a Git tag are both created.

```
Usage: cargo-tag [COMMAND]

Commands:
  current
          Print current package version
  minor
          Bumps crate's minor version and create a git tag
  major
          Bumps crate's major version and create a git tag
  patch
          Bumps crate's patch version and create a git tag
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

## Installation

```bash
cargo install cargo-tag
```

> Requires Git to be installed in your system.

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Distributed under the terms of both the MIT license
