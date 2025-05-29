# dlna-dmr

[![GitHub License](https://img.shields.io/github/license/PRO-2684/dlna-dmr?logo=opensourceinitiative)](https://github.com/PRO-2684/dlna-dmr/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/dlna-dmr/release.yml?logo=githubactions)](https://github.com/PRO-2684/dlna-dmr/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/dlna-dmr?logo=githubactions)](https://github.com/PRO-2684/dlna-dmr/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/dlna-dmr/total?logo=github)](https://github.com/PRO-2684/dlna-dmr/releases)
[![Crates.io Version](https://img.shields.io/crates/v/dlna-dmr?logo=rust)](https://crates.io/crates/dlna-dmr)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/dlna-dmr?logo=rust)](https://crates.io/crates/dlna-dmr)
[![docs.rs](https://img.shields.io/docsrs/dlna-dmr?logo=rust)](https://docs.rs/dlna-dmr)

A dummy DLNA Digital Media Renderer.

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

## 📖 Usage

```shell
$ dlna-dmr
[2025-05-29T10:05:46Z INFO  dlna_dmr] DMR started
[2025-05-29T10:05:46Z INFO  dlna_dmr::ssdp] SSDP server running on 172.31.117.144:1900
[2025-05-29T10:05:46Z INFO  dlna_dmr::http] HTTP server listening on 172.31.117.144:8080
[2025-05-29T10:06:19Z INFO  dlna_dmr::http] Current URI: https://example.com/media.mp4
^C[2025-05-29T10:06:29Z INFO  dlna_dmr::http] HTTP server stopped
[2025-05-29T10:06:29Z INFO  dlna_dmr::ssdp] SSDP server stopped
[2025-05-29T10:06:29Z INFO  dlna_dmr] DMR stopped
```

## ✅ TODO

- [ ] "Heartbeat" - send periodic alive messages to the network
- [ ] Command line arguments parsing
- [ ] Config file
- [ ] Testing HTTP server via [`TestRequest`](https://docs.rs/tiny_http/0.12.0/tiny_http/struct.TestRequest.html)

## 🎉 Credits

TODO
