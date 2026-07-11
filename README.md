# Coronatio

Coronatio is the living crown of HomeServer: the Rust interface that turns a household machine into one coherent appliance face.

The crown stays still while its panes can grow without end. Native panes belong to the firmware. Cartridges let installed services join at runtime. Beneath both, typed routes keep reads, sessions, and privileged actions from becoming an unmarked tangle. The household becomes visible without exposing the machinery that keeps it safe.

We are rebuilding the proven HomeServer control surface from Flask and React as Rust firmware. The implementation changes; the familiar face does not. Admin, Stats, Portals, and Upload retain their recognizable controls and behavior while Rust, Caduceus, and Harmonia replace the old web stack underneath.

## How the crown works

- **Crown shell** — the stable header, tab bar, session controls, and pane frame served by Coronatio.
- **Native panes** — first-party appliance surfaces compiled into the binary.
- **Cartridges** — manifest-backed tabs loaded from the installed cartridge root without recompiling the host.
- **Caduceus** — the narrow privileged hand. Coronatio asks it to perform admitted system changes rather than acquiring broad host power itself.
- **Harmonia** — the keeper of installed service and configuration convergence. It carries the appliance toward its declared profile outside the browser request path.
- **`homeserver.json`** — the household memory: theme, favorites, visible tabs, portals, upload defaults, and other shared configuration.
- **Pulse** — a small Server-Sent Events stream that tells the browser when relevant state changed; the browser then reads the affected state through ordinary typed routes.

The metaphors are names for real boundaries, not decoration. See [the architecture guide](docs/architecture.md) for the complete map.

## Run it locally

You need a current Rust toolchain.

```bash
cargo run
```

Coronatio listens on port `8090` by default. For an isolated development run:

```bash
CORONATIO_TAB_ROOT=/tmp/coronatio-tabs cargo run
```

Open `http://127.0.0.1:8090`. Set `CORONATIO_PORT` to choose another port. Development and tests may point `CORONATIO_HOMESERVER_JSON` at a fixture; an installed appliance reads `/etc/homeserver.json` first.

## Extend it

Choose the smallest lane that fits:

1. **Cartridge** — use a directory containing `tab.json`, static assets, and a local service boundary when a tab should load at runtime.
2. **Native pane** — add Rust routes, state, rendering, and tests when the feature is part of the crown itself.
3. **Source-injection recompile** — reserve whole-host rebuilds for trusted additions that truly need compile-time integration.

Cartridge identifiers and manifests are validated before admission. A faulty cartridge should lose its own pane, not the crown.

## Prove a change

```bash
cargo fmt --check
cargo test
cargo build --release
```

Documentation starts at [docs/README.md](docs/README.md). Contributor theming guidance lives at [docs/development/theme-tokens.md](docs/development/theme-tokens.md).

> Governing design: `pali:coronatio-north-star-contract` and `pali:workflow-coronatio-flask-react-visual-ux-identity-contract`. These are design authority; this README is the human map.
