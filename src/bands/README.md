# Coronatio source bands

`src/main.rs` is the thin Rust process face. These bands hold the Coronatio crown contracts, runtime router, Caduceus membrane, legacy-compatible route readbacks, crown law, shell, and tests in infinite-infinite order.

## One-to-one port doctrine

Coronatio's migration intent is a one-to-one port of the old Flask/React HOMESERVER user experience into Rust-owned firmware.

A port is not a reinterpretation, redesign, summary, scaffold, or inspired-by rebuild. A port directly queries the original Flask/React source and live quarry for intended behavior, then reproduces the same visible controls, control placement, labels, click outcomes, state transitions, persistence keys, layout/feel, data boundaries, and result class in Coronatio unless an explicit approved divergence is recorded.

Identical means indistinguishable to the user under the same viewport, theme, session/admin state, configuration, and data state. Implementation substrate may change from Flask/React/Python to Rust/Caduceus/Harmonia; the experienced surface may not drift. Tests that only prove word presence, contract existence, or approximate semantics are not acceptance for port work. Every port tranche must cite or inspect the original source/living quarry, name the old behavior, reproduce it in Rust, and prove the same user-visible behavior once.


## Central HomeServer state authority

Coronatio is a one-to-one port before it is a new product surface. The central HomeServer config file, `homeserver.json`, is the first runtime state authority for config, theme, favorites, tabs, portals, upload defaults, and starred/default route behavior. Coronatio SHALL read `/etc/homeserver.json` first on the installed homeserver, then the legacy live HomeServer config as migration fallback, before any Coronatio-local fallback or firmware default. Sidecar JSON may exist only as quarry, fixture, or packed firmware default; it is not the living user-state authority.

## Favorite and first-load manifest law

Original first load is governed by the Flask tab registry and favorite system: `flask-0-7-tabdrawing.md` loads `get_starred_tab() or get_first_visible_tab()` into the root template; `flask-0-8-favoritecomponent.md` loads `/api/get_starred_tab`, updates exactly one visible non-admin favorite, and updates the star UI. Coronatio records that law in the single HomeServer config, `homeserver.json`: `tabs.starred` selects the default route and `tabs.<tab>.config` plus `tabs.<tab>.visibility` define visible/admin tab state. `/api/favorites` and `/api/get_starred_tab` project that one config truth; they do not read a Coronatio sidecar favorites JSON. The star is the favorite/default-route control. The eye is the visibility control: in admin mode it hides or shows a tab without deleting, disabling, uninstalling, or marking it failed.

## Adding a crown pane

Coronatio SHALL add a crown pane through this five-step ladder:

1. Add one `CrownPane` literal to `native_crown_panes()` with a lowercase-hyphen `id`, an `order`, `route: /#<id>`, and `state_route: /api/panes/<id>`.
2. Write `render_<pane>()` returning `<section class="pane" data-pane-panel="<id>" data-view-panel="<id>">` and compose it only from `ui-*` primitives proven in the Test tab showcase. The Test tab is the executable spec; copy its markup, do not invent classes.
3. Splice the pane through a `__PLACEHOLDER__` in `render_crown_shell()` so the shell owns placement while the pane owns its body.
4. Put visible CSS in the right UX hierarchy home: `src/bands/shell/ux/library/` for og `styles/common/ui` primitives, `src/bands/shell/ux/packs/` for pane domain packs, and `src/bands/shell/ux/shell/` for crown chrome/base substrate; then add the file to `src/bands/shell/ux/index.json` in serve order.
4. Deliver server state as HTMX `/admit/<pane>` fragments with `Cache-Control: no-store`.
5. Bind interactivity through delegated body listeners keyed by `data-*` attributes. A pane fragment SHALL NOT bind listeners at init time, because HTMX swaps orphan them; break-glass commit `17bf406` proves delegated chrome survives while init-time bindings die.

Pane HTML lives in `r#""#` `format!` bodies; literal braces are escaped as `{{ }}`.
