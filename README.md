# Coronatio

Coronatio is a Rust web interface for a self-hosted appliance. It puts system status, service links, file upload, network controls, and other local tools behind one consistent tabbed interface.

The binary ships with native tabs for Admin, Portals, Upload, Stats, backBlaze, Wake on LAN, Test, Linker, DHCP, Firewall, and DNS. Some tabs are available only in admin mode, and DHCP is hidden by default.

## Loadable cartridges

Any web interface on your network can become a Coronatio tab. In admin mode, select the `+` button in the tab bar, enter a title and URL, and choose whether the tab is admin-only. Coronatio stores the cartridge through its privileged actuator and loads the target in a sandboxed iframe; no rebuild is required.

The target service must allow embedding. A service that sends `X-Frame-Options: SAMEORIGIN` or `DENY`, or a restrictive Content-Security-Policy `frame-ancestors` rule, will show an empty pane. See [Loadable cartridges](docs/cartridges.md) for setup, validation rules, and troubleshooting.

A cartridge displays the target service as it is. It does not restyle that service to match Coronatio. Its value is giving you one place to view and control the web services on your network.

## Run it locally

You need a current Rust toolchain.

```bash
cargo run
```

Open `http://127.0.0.1:8090`. Coronatio listens on port `8090` by default and on all interfaces. Set `CORONATIO_PORT` to choose another port.

A local run may not have the appliance configuration and privileged actuator used by installed systems. You can point read-only configuration and cartridge discovery at development files:

```bash
CORONATIO_HOMESERVER_JSON=/path/to/config.json \
CORONATIO_CARTRIDGE_REGISTRY=/path/to/cartridges.json \
cargo run
```

`CORONATIO_STATIC_ROOT` can select a static asset directory.

## Documentation

- [Documentation index](docs/README.md)
- [Loadable cartridges](docs/cartridges.md)
- [Architecture](docs/architecture.md)
- [Theme tokens](docs/development/theme-tokens.md)

## Check a change

```bash
cargo fmt --check
cargo test
cargo build --release
```
