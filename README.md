# Coronatio

Coronatio is the Rust appliance host for the next HOMESERVER control surface.

Arcadia remains the HomeConsole/game appliance. Coronatio is the crown for the broader HOMESERVER frontend/backend replacement: the place where the old Flask/React premium-tab law becomes a typed Rust appliance substrate.

## North Star

Coronatio SHALL preserve the HOMESERVER promise of infinite services and infinite tabs while replacing the hostile Flask/React/Socket.IO substrate with explicit Rust state machines.

The old premium-tab system is quarry, not trash. It contains hard-won behavior law:

- folders can introduce new tabs;
- a tab carries UI, backend routes, config, dependencies, permissions, and install/remove lifecycle;
- install mutates a registry and produces a visible tab;
- rebuild/restart is allowed when the chosen lane requires it;
- installed tabs survive future host builds;
- admin visibility and service health are part of the tab contract.

Coronatio re-inscribes that law as typed Rust contracts. The crown primary tabs are `admin`, `stats`, `portals`, and `upload`; third-party platform-owned brands stay out of primary navigation.

## Initial substrate

This scaffold provides the first host surface:

- `GET /` — static Coronatio crown shell with lawful primary panes.
- `GET /health` — host health readback.
- `GET /api` — root Coronatio object with native-pane readback.
- `GET /api/panes` — first-party native pane registry for `admin`, `stats`, `portals`, and `upload`.
- `GET /api/panes/:pane_id` — one native pane readback.
- `GET /api/tabs` — dynamic tab registry read from installed cartridges plus native-pane readback.
- `GET /api/tabs/:tab_id/manifest` — one cartridge manifest readback.
- `/tabs/<tab-id>/...` — static installed tab assets under the configured tab root.

Default tab root:

```text
/var/lib/coronatio/tabs
```

Override for development:

```bash
CORONATIO_TAB_ROOT=/path/to/tabs CORONATIO_PORT=8090 cargo run
```

## Cartridge shape

A dynamic tab is a directory with a `tab.json` manifest:

```text
/var/lib/coronatio/tabs/<tab-id>/tab.json
/var/lib/coronatio/tabs/<tab-id>/static/...
/var/lib/coronatio/tabs/<tab-id>/receipts/...
```

Minimal manifest:

```json
{
  "id": "portals",
  "title": "Portals",
  "description": "Service portal launcher and local ingress appliance",
  "order": 30,
  "adminOnly": true,
  "routePrefix": "/api/tabs/portals",
  "staticDir": "static",
  "serviceUrl": "http://127.0.0.1:9910",
  "healthRoute": "/health",
  "installMode": "dynamic-cartridge"
}
```

## The key decision

Coronatio does not need one installation strategy. It needs a lawful membrane for each class of extension.

1. `dynamic-cartridge`
   - no Coronatio recompile;
   - manifest + static assets + local service;
   - best default for user-installed services and infinite tabs.

2. `source-injection-recompile`
   - folder is injected into the source tree and Coronatio is rebuilt;
   - preserves the old premium-tab ability to recompile the whole host when that is the right answer;
   - best for trusted first-party/native tabs that need compile-time integration.

3. `first-party-native`
   - compiled into Coronatio as core appliance firmware;
   - best for crown-level panes that define the product itself.

The acceptance law is not “never recompile.” The acceptance law is: choose the lane that preserves infinite installation while making the membrane explicit, typed, reversible, and provable.

## Proof commands

```bash
cargo fmt --check
cargo test
cargo build --release
```

## Done gate for this scaffold

This scaffold is accepted when:

- Fulcrum attaches `HOMESERVERSLTD/coronatio` as `attachments/coronatio`.
- Coronatio builds as a Rust binary.
- Tests prove the host names Coronatio, not Arcadia.
- Tests prove the native crown panes are exactly `admin`, `stats`, `portals`, and `upload`.
- Tests prove the shell renders those primary panes without platform-owned brand navigation.
- Tests prove dynamic cartridge manifests load without host recompile.
- Paligenesis carries the North Star contract and open decisions.
