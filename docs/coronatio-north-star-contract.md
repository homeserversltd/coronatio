# Coronatio North Star Contract

## Crown intent

Coronatio is the HOMESERVER crown: a Rust-owned appliance host for infinite services and infinite tabs.

It replaces the old Flask/React HOMESERVER control surface by extracting the premium-tab state-machine law and rebuilding it as typed contracts, explicit lanes, receipts, and appliance readbacks. The lawful primary tabs are `admin`, `stats`, `portals`, and `upload`; platform-owned third-party brands are cartridge content only, never crown primary navigation.

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

## Premium installer tranche law

The old Flask premium installer is behavior quarry, not runtime authority. Coronatio currently exposes it as `/api/installer`, a typed readback/state-machine contract. The route records the package schema (`name`, `version`, `config.repository`, `git_managed`, `files`), component operation schema (`source`, `target`, `type`, `identifier`, `marker`, `description`), validation order, install order, rollback order, uninstall/reinstall/batch law, and lane mapping. It does not copy files, append blueprints, install dependencies, patch config, rebuild frontend assets, or restart services. Those live mutations require a later Caduceus actuator with non-secret receipts.

## Current scaffold proof

The first scaffold proves:

- the repo is Coronatio, not Arcadia;
- `/` renders the initial Rust crown shell;
- `/api` names the Coronatio root;
- `/api/panes` exposes the first-party native crown pane registry;
- native crown panes are exactly `admin`, `stats`, `portals`, and `upload`;
- `/api/registry` exposes extracted registry/tab law from `homeserver.json`, including config, visibility, data, starred/default route, admin gating, enabled state, order, and validation rules;
- `/api/startup` exposes startup and initial-tab law: forced tab, failed connection fallback, valid visible starred tab, first visible tab, and tab-bar visibility;
- `/api/lanes` exposes dynamic cartridge, source-injection/recompile, and first-party native failure/recovery policy;
- `/api/fallback` exposes the safe fallback pane, activation reasons, recovery sequence, and receipt fields;
- `/api/session` and `/api/admin/session` expose admin session law: PIN validation, 30-minute token lease, keepalive/renewal, admin-enhanced field filtering, and Caduceus privileged mutation membrane;
- `/api/topics` replaces Socket.IO subscribe/unsubscribe law with SSE EventSource plus POST renew topic admission;
- `/api/monitor/pulse`, `/api/stats/events`, and `/api/stats/events/renew` prove the first `stats.system` monitor pulse contract end-to-end;
- `/api/boundary` and the fallback router expose Flask route and SPA boundary law: `/api/*` JSON 404, non-API shell fallback, and cartridge static serving;
- `/api/installer` exposes the old premium installer as contract-only law: manifest fields, copy/append operation schema, validation phases, install phases, rollback/uninstall/reinstall/batch policy, lane mapping, and the missing Caduceus actuator signal;
- `/api/stats` exposes the first honest Stats snapshot readback and names the SSE lease routes;
- `/api/tabs` loads installed tab manifests dynamically while preserving native pane readback;
- unsafe tab ids are rejected;
- the architecture keeps both options alive: runtime cartridges and full-host recompilation.
