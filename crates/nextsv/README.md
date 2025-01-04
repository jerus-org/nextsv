# nextsv

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-badge]][circleci-url]
[![Rust 1.74+][version-badge]][version-url]
[![FOSSA Status][fossa-badge]][fossa-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]
[![codecov][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/crates/v/nextsv.svg
[crates-url]: https://crates.io/crates/nextsv
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/nextsv/blob/main/LICENSE
[circleci-badge]:https://circleci.com/gh/jerusdp/nextsv/tree/main.svg?style=svg
[circleci-url]: https://circleci.com/gh/jerusdp/nextsv/tree/?branch=main
[version-badge]: https://img.shields.io/badge/rust-1.74+-orange.svg
[version-url]: https://www.rust-lang.org
[fossa-badge]: https://app.fossa.com/api/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fnextsv.svg?type=shield
[fossa-url]: https://app.fossa.com/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fnextsv?ref=badge_shield
[docs-badge]:  https://docs.rs/nextsv/badge.svg
[docs-url]:  https://docs.rs/nextsv
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp
[codecov-badge]: https://codecov.io/gh/jerus-org/nextsv/graph/badge.svg?token=JL9HSHV4QR
[codecov-url]: https://codecov.io/gh/jerus-org/nextsv

A utility to calculate the level of change and the next semantic version number based on the conventional commits since the last version tag.

## Feature set

- [x] Calculate next semantic version number
- [x] Calculate the level to change for next semantic version number
- [x] Support basic semantic version components: Major, Minor, and Patch
- [x] Check for required files (e.g. CHANGELOG.md)
- [x] Set level of change (Breaking, Feature, Fix, Other) at which required files are required
- [x] Check that any changes made meet a specified level
- [x] Support pre-release versions (alpha, beta, rc)
- [x] Update to release version (removing pre-release identifiers)
- [ ] Handle case where no tag is found

## CLI Usage

Install the CLI using cargo install.

```sh

cargo install nextsv

```

Run in your project directory and check the version

```console
$ nextsv --version
nextsv 0.19.2

```

Running the application provides the level for the next semantic version change.

```sh

$ nextsv
[2022-08-03T06:33:54Z INFO  nextsv] Calculating the next version level
minor

```

Help provides all the options

```sh

$ nextsv -h
jerusdp <jrussell@jerus.ie>
Next semantic version calculator

USAGE:
    nextsv [OPTIONS]

OPTIONS:
    -f, --force <FORCE>      Force the calculation of the version number [possible values: major,
                             minor, patch, first]
    -h, --help               Print help information
        --level              Report the level of the version number change
        --number             Report the version number
    -p, --prefix <PREFIX>    Prefix string to identify version number tags [default: v]
    -q, --quiet              Pass many times for less log output
    -v, --verbose            Pass many times for more log output
    -V, --version            Print version information
$

```

A clean response of the level to update is reported using the -q flag.

```sh

$ nextsv -q
minor

```

This can be used with `cargo release` to update and publish a new release.

```sh

cargo release $(nextsv -q)

```

Further details on usage scenarios can be found [here](docs/sceanrios.md).

## Library Usage

To use the library add the crate to dependencies in the project's Cargo.toml.

```toml

[dependencies]
nextsv = "0.19.2"

```
