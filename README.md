# dlna-dmr

[![GitHub License](https://img.shields.io/github/license/PRO-2684/dlna-dmr?logo=opensourceinitiative)](https://github.com/PRO-2684/dlna-dmr/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/dlna-dmr/release.yml?logo=githubactions)](https://github.com/PRO-2684/dlna-dmr/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/dlna-dmr?logo=githubactions)](https://github.com/PRO-2684/dlna-dmr/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/dlna-dmr/total?logo=github)](https://github.com/PRO-2684/dlna-dmr/releases)
[![Crates.io Version](https://img.shields.io/crates/v/dlna-dmr?logo=rust)](https://crates.io/crates/dlna-dmr)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/dlna-dmr?logo=rust)](https://crates.io/crates/dlna-dmr)
[![docs.rs](https://img.shields.io/docsrs/dlna-dmr?logo=rust)](https://docs.rs/dlna-dmr)

A dummy DLNA Digital Media Renderer.

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

To run the DMR, simply execute the following command in your terminal:

```shell
$ dlna-dmr
[2025-05-30T14:49:48Z INFO  dlna_dmr] DMR started
[2025-05-30T14:49:48Z INFO  dlna_dmr::ssdp] SSDP server running on 172.31.117.144:1900
[2025-05-30T14:49:48Z INFO  dlna_dmr::http] HTTP server listening on 172.31.117.144:8080
[2025-05-30T14:50:11Z INFO  dlna_dmr::http] RenderingControl::SetMute channel: Master, desired_mute: false
[2025-05-30T14:50:38Z INFO  dlna_dmr::http] AVTransport::SetAvTransportUri current_uri: http://example.com/sample.mp4?param1=a&param2=b
^C
[2025-05-30T14:50:46Z INFO  dlna_dmr::http] HTTP server stopped
[2025-05-30T14:50:46Z INFO  dlna_dmr::ssdp] SSDP server stopped
[2025-05-30T14:50:46Z INFO  dlna_dmr] DMR stopped
```

To configure, simply pass in a path to a configuration file:

```shell
dlna-dmr path/to/config.toml
```

## ✅ TODO

- [x] Actual XML parsing
- [ ] "Heartbeat" - send periodic alive messages to the network
- [ ] Command line arguments parsing
- [ ] Config file
- [ ] Testing HTTP server via [`TestRequest`](https://docs.rs/tiny_http/0.12.0/tiny_http/struct.TestRequest.html)

## 🎉 Credits

TODO
