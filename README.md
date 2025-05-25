# dlna-dmr

[![GitHub License](https://img.shields.io/github/license/PRO-2684/dlna-dmr?logo=opensourceinitiative)](https://github.com/PRO-2684/dlna-dmr/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/dlna-dmr/release.yml?logo=githubactions)](https://github.com/PRO-2684/dlna-dmr/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/dlna-dmr?logo=githubactions)](https://github.com/PRO-2684/dlna-dmr/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/dlna-dmr/total?logo=github)](https://github.com/PRO-2684/dlna-dmr/releases)
[![Crates.io Version](https://img.shields.io/crates/v/dlna-dmr?logo=rust)](https://crates.io/crates/dlna-dmr)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/dlna-dmr?logo=rust)](https://crates.io/crates/dlna-dmr)
[![docs.rs](https://img.shields.io/docsrs/dlna-dmr?logo=rust)](https://docs.rs/dlna-dmr)

DLNA Digital Media Renderer.

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `dlna-dmr` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/dlna-dmr/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `dlna-dmr`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `dlna-dmr`

4. [Add a repository secret](https://github.com/PRO-2684/dlna-dmr/settings/secrets/actions) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall dlna-dmr
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/dlna-dmr/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install dlna-dmr
```

## 💡 Examples

TODO

## 📖 Usage

TODO

## 🎉 Credits

TODO
