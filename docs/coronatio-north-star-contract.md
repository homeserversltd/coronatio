# Coronatio North Star Contract

## Crown intent

Coronatio is the HOMESERVER crown: a Rust-owned appliance host for infinite services and infinite tabs.

It replaces the old Flask/React HOMESERVER control surface by extracting the premium-tab state-machine law and rebuilding it as typed contracts, explicit lanes, receipts, and appliance readbacks.

## Preserved goods from HOMESERVER quarry

The old premium-tab system proved these goods are required:

- a folder can become a visible tab;
- tab installation can merge config;
- tab installation can add backend capability;
- tab installation can add frontend surface;
- tab installation can add dependencies and permissions;
- a whole-host rebuild/restart is acceptable when the lane requires it;
- installed tab state must survive future rebuilds;
- admin visibility, order, health, and enabled state belong to the tab contract.

## Replacement law

Coronatio SHALL expose three extension lanes rather than pretending one lane solves every tab.

### Dynamic cartridge lane

Default lane for user-installed services.

- Load manifest at runtime.
- Serve installed static assets by tab id.
- Proxy or call a local service boundary.
- Enforce admin/capability policy at Coronatio.
- No Coronatio recompile required.

### Source injection/recompile lane

Trusted lane for tabs that must enter the native source tree.

- Copy or generate source into declared modules.
- Rebuild the whole Coronatio binary.
- Run full tests/build.
- Publish/admit through Cibation when repo-backed.
- Keep installed source and registry state reproducible.

### First-party native lane

Core product lane.

- Compile directly into Coronatio.
- Use Rust routes, typed state, Tokio tasks, and appliance panes.
- Carry the highest UX and proof burden.

## Open decisions

The scaffolding SHALL make these decisions visible before they harden:

1. Manifest schema: JSON now, possible TOML later if human editing becomes more important.
2. UI payload: prebuilt static JS/WASM island versus server-rendered generic pane.
3. Backend boundary: localhost HTTP first, Unix socket later if service hardening demands it.
4. Install authority: direct installer command versus Fulcrum/Harmonia mediated installation.
5. Rebuild lane: when a premium tab is allowed to inject source and force a full Coronatio recompile.
6. Receipt ledger: per-tab receipts under the tab root versus central Coronatio ledger.
7. Sandboxing: systemd service first; WASI only when pure plugin code wants stronger sandboxing.
8. Promotion path: dynamic cartridge to first-party native when a tab becomes crown law.

## Current scaffold proof

The first scaffold proves:

- the repo is Coronatio, not Arcadia;
- `/api` names the Coronatio root;
- `/api/tabs` loads installed tab manifests dynamically;
- unsafe tab ids are rejected;
- the architecture keeps both options alive: runtime cartridges and full-host recompilation.
