# Coronatio

Coronatio is the Rust appliance host for the next HOMESERVER control surface.

Arcadia remains the HomeConsole/game appliance. Coronatio is the crown for the broader HOMESERVER frontend/backend replacement: the place where the old Flask/React premium-tab law becomes a typed Rust appliance substrate.

## North Star

Coronatio SHALL preserve the HOMESERVER promise of infinite services and infinite tabs while replacing the hostile Flask/React/Socket.IO substrate with explicit Rust state machines.

## One-to-one port doctrine

Coronatio's migration intent is a one-to-one port of the old Flask/React HOMESERVER user experience into Rust-owned firmware.

A port is not a reinterpretation, redesign, summary, scaffold, or inspired-by rebuild. A port directly queries the original Flask/React source and live quarry for intended behavior, then reproduces the same visible controls, control placement, labels, click outcomes, state transitions, persistence keys, layout/feel, data boundaries, and result class in Coronatio unless an explicit approved divergence is recorded.

Identical means indistinguishable to the user under the same viewport, theme, session/admin state, configuration, and data state. Implementation substrate may change from Flask/React/Python to Rust/Caduceus/Harmonia; the experienced surface may not drift. Tests that only prove word presence, contract existence, or approximate semantics are not acceptance for port work. Every port tranche must cite or inspect the original source/living quarry, name the old behavior, reproduce it in Rust, and prove the same user-visible behavior once.


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
- `GET /api/registry` — extracted HOMESERVER tab registry law: visibility, admin gating, enabled state, order, starred/default route, and validation rules.
- `GET /api/registry/transaction` — config patch and persistence law: deep merge, `tabs.starred` preservation, factory fallback validation, temp candidate promotion, permission restoration, backup, and rollback contract.
- `GET /api/startup` — extracted startup and initial-tab law: phases, connection fallback, forced tab, starred tab, first visible tab, and tab-bar rule.
- `GET /api/lanes` — dynamic cartridge, source-injection/recompile, and first-party native failure/recovery policy.
- `GET /api/fallback` — safe fallback pane, activation reasons, recovery sequence, and receipt fields.
- `GET /api/session` / `GET|POST /api/admin/session` — admin session contract: PIN validation, 30-minute token lease, keepalive/renewal, admin field filtering, and Caduceus privileged mutation membrane.
- `GET /api/topics` — topic catalog replacing Socket.IO subscription law with SSE EventSource plus POST renew admission.
- `GET /api/monitor/pulse` — monitor payload/pulse contract for the first `stats.system` topic: cadence, changed predicate, admin fields, snapshot, SSE, renew, and first event proof.
- `GET /api/boundary` — Flask route and SPA boundary replacement: `/api/*` JSON 404, non-API shell fallback, cartridge static serving.
- `GET /api/installer` — premium installer law readback: root/component manifest schemas, copy/append operation fields, validation phases, install phases, rollback/uninstall/reinstall/batch law, lane mapping, and the first missing Caduceus live-mutation signal.
- `GET /api/stats/events` — first Stats SSE event stream contract.
- `POST /api/stats/events/renew` — first Stats SSE lease renewal contract.
- `GET /api/stats` — honest first-party Stats snapshot readback with SSE route posture.
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

## Frontend storage and persistence contract

`/api/frontend/storage` is a contract/readback route only. It extracts old browser persistence without reading or writing any browser storage. The readback names:

- old persisted stores: `homeserver-store`, `auth-storage`, and `themeData`;
- old persisted fields: `theme`, `visibility`, `starredTab`, `isInitialized`, `tabs`, `activeTab`, `isAdmin`, and `themeData`;
- debounce and timeout law: 500ms localStorage write debounce, 500ms duplicate tablet-load debounce, 15s tablet load fallback, and 7s startup config fetch timeout;
- stale-state recovery: malformed localStorage ignored, hidden/disabled/admin-forbidden tabs clipped, stale async loads discarded, failed visibility writes rolled back;
- Coronatio ownership: registry owns tabs/visibility/starred tab, browser owns active tab/theme preferences, startup/session receipts own initialization/admin state;
- migration path and forbidden credential persistence.

Live storage migration remains behind a later browser adapter and receipt.

## Service, portal, monitor, and data contract

`/api/services/data` is a contract/readback route only. It extracts old portal and monitor behavior without running systemctl, network checks, VPN/Tailscale commands, disk probes, sync jobs, or drive tests. The readback names:

- portal shape from `tabs.portals.data.portals[]`: `name`, `description`, `services`, `type`, `port`, `localURL`, `remoteURL`;
- service-card status fields: `systemdName`, `isEnabled`, `isActive`, `status`, `statusDetails`, `isScriptManaged`, `port`, `needsReboot`;
- monitor topics: services, power, system, internet, tailscale, VPN, sync, hard-drive-test, admin disk, and admin system;
- admin field filters and admin-only topic boundaries;
- old broadcast change predicates and their Coronatio SSE/readback replacement.

Live collectors and privileged actions remain behind later Caduceus/Coronatio actuator receipts.

## Registry transaction contract

`/api/registry/transaction` is a contract/readback route only. It extracts old `ConfigManager` behavior without writing `homeserver.json` or touching permissions. The readback names:

- deep-merge law: object recursion, scalar/array replacement, and tab-specific merge handling;
- `tabs.starred` law: preserve the current starred pointer while package patches add or update tab records;
- validation law: JSON syntax, temporary candidate validation, and factory fallback rejection;
- persistence law: backup current config, write temp candidate, promote only after validation, restore owner/mode;
- rollback law: backup restore, patch-key removal, whole-tab revert under `tabs`, and mismatch preservation.

Live config writes, `factoryFallback.sh`, `chown`, `chmod`, and temp-file promotion remain outside this tranche until Caduceus owns the actuator and receipt ledger.

## Premium installer contract

`/api/installer` is a contract/readback route only. It extracts the old premium installer’s manifest and lifecycle law without executing third-party premium code or mutating the host. The readback names:

- root package fields: `name`, `version`, `config.repository`, `git_managed`, and `files`;
- component operation fields: `source`, `target`, `type`, `identifier`, `marker`, and `description`;
- validation phases: current config, package manifest, name collision, version conflict, and dependency validation;
- install phases: backend/frontend/root/permissions file operations, Python requirements, npm patches, system deps, config patches, hooks, frontend rebuild, and service restart;
- rollback/lifecycle law for config restore, package rollback, file reversal, service state restore, uninstall, reinstall, batch mode, and restore-patches;
- lane mapping that replaces Flask blueprint injection with `dynamic-cartridge`, `source-injection-recompile`, or `first-party-native` contracts.

Live installation remains outside this tranche. Privileged file writes, dependency installs, config writes, rebuilds, and service restarts require a later Caduceus actuator/receipt membrane.


## Theme system tranche law

The old Flask/React theme system is behavior quarry and visual contract. Coronatio implements it as Rust-delivered firmware with the same browser-visible theme membrane: `preferred-theme`, `themeData`, `style[data-theme-styles]`, `data-theme`, and `--theme-*` CSS variables. Light, dark, and radioactive themes must apply through one token catalog and produce identical token-driven styling across Admin, Stats, Portals, Upload, header controls, modals, cards, and tab controls. A theme switch is real only when the DOM dataset, injected style membrane, local persistence, theme button label, and all pane card computed styles move together.

Runtime theme authority is `static/themes/theme.json`, not Rust source literals. The installed service reads `/opt/coronatio/source/static/themes/theme.json` unless `CORONATIO_THEME_JSON` points elsewhere. `/api/themes` validates and exposes that catalog at runtime so users can add a new theme object without editing Rust code; the browser then derives theme choices, `<html data-theme>`, `preferred-theme`, `themeData`, and `style[data-theme-styles]` from the JSON catalog. The header theme button is a direct port of the old Header behavior: click cycles to the next JSON theme immediately; it does not open a theme modal.

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
- Tests prove `/api/registry` captures old `homeserver.json` tab law: config, visibility, data, starred, admin gating, enabled state, order, and default route behavior.
- Tests prove `/api/registry/transaction` captures config patch persistence law: deep merge, starred preservation, factory fallback validation, temp promotion, permission restoration, backup, and rollback.
- Tests prove `/api/startup` captures forced tab, connection failure, starred/default route, first-visible fallback, and tab-bar behavior.
- Tests prove `/api/lanes` captures dynamic cartridge, source-injection/recompile, and first-party native failure policy.
- Tests prove `/api/fallback` captures safe fallback pane, activation reasons, recovery sequence, and receipt fields.
- Tests prove `/api/session` captures PIN validation, session timeout/keepalive, admin field filtering, and Caduceus mutation membrane.
- Tests prove `/api/topics` replaces Socket.IO subscription law with SSE + lease renew topic contracts.
- Tests prove `/api/monitor/pulse`, `/api/stats/events`, and `/api/stats/events/renew` expose the first `stats.system` monitor pulse end-to-end.
- Tests prove `/api/*` misses return JSON 404 while non-API unknown paths return the shell.
- Tests prove `/api/installer` exposes premium installer law as typed contract-only readback and does not claim live mutation.
- Tests prove `/api/stats` returns an honest snapshot with unavailable telemetry and SSE routes.
- Tests prove dynamic cartridge manifests load without host recompile and final manifest validation rejects unsafe/native cartridge shapes.
- Paligenesis carries the North Star contract and open decisions.
