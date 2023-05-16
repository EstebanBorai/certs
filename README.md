<div>
  <h1 align="center">certs</h1>
  <h4 align="center">
    CLI tool to debug server's SSL/TLS Certificates
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/certs.svg)](https://crates.io/crates/certs)
  [![Documentation](https://docs.rs/certs/badge.svg)](https://docs.rs/certs)
  ![Build](https://github.com/EstebanBorai/certs/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/certs/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/certs/workflows/fmt/badge.svg)

</div>

## Installation

```bash
cargo install certs
```

## Usage

### Render a website certificate summary

```bash
$ certs show https://www.estebanborai.com

Fetching certificate from: www.estebanborai.com:443

Issuer: C=US, O=Cloudflare, Inc., CN=Cloudflare Inc ECC CA-3
Subject: C=US, ST=California, L=San Francisco, O=Cloudflare, Inc., CN=sni.cloudflaressl.com
Not Before: 2022-11-03 0:00:00.0 +00:00:00
Not After: 2023-11-03 23:59:59.0 +00:00:00
Version: V3
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

Distributed under the terms of both the MIT license
