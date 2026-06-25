# Coronatio North Star Contract

## Crown intent

Coronatio is the HOMESERVER crown: a Rust-owned appliance host for infinite services and infinite tabs.

It replaces the old Flask/React HOMESERVER control surface by extracting the premium-tab state-machine law and rebuilding it as typed contracts, explicit lanes, receipts, and appliance readbacks. The lawful primary tabs are `admin`, `stats`, `portals`, and `upload`; platform-owned third-party brands are cartridge content only, never crown primary navigation.

## One-to-one port doctrine

Coronatio's migration intent is a one-to-one port of the old Flask/React HOMESERVER user experience into Rust-owned firmware.

A port is not a reinterpretation, redesign, summary, scaffold, or inspired-by rebuild. A port directly queries the original Flask/React source and live quarry for intended behavior, then reproduces the same visible controls, control placement, labels, click outcomes, state transitions, persistence keys, layout/feel, data boundaries, and result class in Coronatio unless an explicit approved divergence is recorded.

Identical means indistinguishable to the user under the same viewport, theme, session/admin state, configuration, and data state. Implementation substrate may change from Flask/React/Python to Rust/Caduceus/Harmonia; the experienced surface may not drift. Tests that only prove word presence, contract existence, or approximate semantics are not acceptance for port work. Every port tranche must cite or inspect the original source/living quarry, name the old behavior, reproduce it in Rust, and prove the same user-visible behavior once.



## Favorite and first-load manifest law

Original first load is governed by the Flask tab registry and favorite system: `flask-0-6-tabstate.md` marks `upload` as the starred tab; `flask-0-7-tabdrawing.md` loads `get_starred_tab() or get_first_visible_tab()` into the root template; `flask-0-8-favoritecomponent.md` loads `/api/get_starred_tab`, updates exactly one visible non-admin favorite, and updates the star UI. Coronatio records that law in `static/favorites/favorites.json`, exposes it through `/api/favorites` and `/api/get_starred_tab`, and first-loads the manifest starred tab unless an explicit URL hash names a valid visible tab. The star is the favorite/default-route control. The eye is the visibility control: in admin mode it hides or shows a tab without deleting, disabling, uninstalling, or marking it failed.

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

## Frontend storage and persistence tranche law

The old browser persistence layer is behavior quarry, not runtime authority. Coronatio currently exposes it as `/api/frontend/storage`, a typed readback contract. The route records the old `homeserver-store`, `auth-storage`, and `themeData` keys; persisted fields `theme`, `visibility`, `starredTab`, `isInitialized`, `tabs`, `activeTab`, `isAdmin`, and `themeData`; debounce and timeout behavior; stale local-storage and stale async-load recovery; the split between registry/server-owned state and browser-only preferences; and a one-pass migration path. Credential/token/PIN/password/API-key persistence is explicitly forbidden. The route does not read browser storage or perform migration; that requires a later browser adapter and receipt.

## Service, portal, monitor, and data tranche law

The old portals, services monitor, and broadcast manager are behavior quarry, not runtime authority. Coronatio currently exposes them as `/api/services/data`, a typed readback contract. The route records portal fields (`name`, `description`, `services`, `type`, `port`, `localURL`, `remoteURL`), service-card health fields (`systemdName`, `isEnabled`, `isActive`, `status`, `statusDetails`, `isScriptManaged`, `port`, `needsReboot`), monitor topics for services, power, system, internet, tailscale, VPN, sync, hard-drive-test, admin disk, and admin system, plus admin field filters and topic-specific broadcast change predicates. It does not run host collectors or privileged actions; those require later Caduceus/Coronatio actuator receipts.

## Config patch and persistence tranche law

The old `ConfigManager` registry writer is behavior quarry, not runtime authority. Coronatio currently exposes it as `/api/registry/transaction`, a typed readback/state-machine contract. The route records object-recursive deep merge, scalar/array replacement, `tabs.starred` preservation, factory fallback validation, temp-candidate write/promote, `www-data:www-data`/`664` permission restoration, backup restore, patch-key revert, and mismatch preservation. It does not write live config, execute `factoryFallback.sh`, move temp files, or change ownership/mode. Those live mutations require a later Caduceus actuator with non-secret receipts.

## Premium installer tranche law

The old Flask premium installer is behavior quarry, not runtime authority. Coronatio currently exposes it as `/api/installer`, a typed readback/state-machine contract. The route records the package schema (`name`, `version`, `config.repository`, `git_managed`, `files`), component operation schema (`source`, `target`, `type`, `identifier`, `marker`, `description`), validation order, install order, rollback order, uninstall/reinstall/batch law, and lane mapping. It does not copy files, append blueprints, install dependencies, patch config, rebuild frontend assets, or restart services. Those live mutations require a later Caduceus actuator with non-secret receipts.


## Theme system tranche law

The old Flask/React theme system is behavior quarry and visual contract. Coronatio implements it as Rust-delivered firmware with the same browser-visible theme membrane: `preferred-theme`, `themeData`, `style[data-theme-styles]`, `data-theme`, and `--theme-*` CSS variables. Light, dark, and radioactive themes must apply through one token catalog and produce identical token-driven styling across Admin, Stats, Portals, Upload, header controls, modals, cards, and tab controls. A theme switch is real only when the DOM dataset, injected style membrane, local persistence, theme button label, and all pane card computed styles move together.

Runtime theme selection authority is the single HomeServer config, `homeserver.json`, specifically `global.theme.name`; the installed service prefers `/etc/homeserver.json` and may read the legacy live path only as fallback. Coronatio may carry firmware theme token defaults, but it SHALL NOT treat `static/themes/theme.json`, `CORONATIO_THEME_JSON`, or another sidecar JSON as runtime authority. `/api/themes` exposes the selected homeserver.json theme plus firmware token defaults so the browser derives theme choices, `<html data-theme>`, `preferred-theme`, `themeData`, and `style[data-theme-styles]` from the one config truth. The header theme button is a direct port of the old Header behavior: click cycles to the next JSON theme immediately; it does not open a theme modal.

## Current scaffold proof

The first scaffold proves:

- the repo is Coronatio, not Arcadia;
- `/` renders the initial Rust crown shell;
- `/api` names the Coronatio root;
- `/api/panes` exposes the first-party native crown pane registry;
- native crown panes are exactly `admin`, `stats`, `portals`, and `upload`;
- `/api/registry` exposes extracted registry/tab law from `homeserver.json`, including config, visibility, data, starred/default route, admin gating, enabled state, order, and validation rules;
- `/api/registry/transaction` exposes config patch and persistence law: deep merge, `tabs.starred` preservation, factory fallback validation, temp candidate promotion, owner/mode restoration, backup, and rollback;
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
