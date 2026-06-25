# Coronatio source bands

`src/main.rs` is the thin Rust process face. These bands hold the Coronatio crown contracts, runtime router, Caduceus membrane, legacy-compatible route readbacks, crown law, shell, and tests in infinite-infinite order.

## One-to-one port doctrine

Coronatio's migration intent is a one-to-one port of the old Flask/React HOMESERVER user experience into Rust-owned firmware.

A port is not a reinterpretation, redesign, summary, scaffold, or inspired-by rebuild. A port directly queries the original Flask/React source and live quarry for intended behavior, then reproduces the same visible controls, control placement, labels, click outcomes, state transitions, persistence keys, layout/feel, data boundaries, and result class in Coronatio unless an explicit approved divergence is recorded.

Identical means indistinguishable to the user under the same viewport, theme, session/admin state, configuration, and data state. Implementation substrate may change from Flask/React/Python to Rust/Caduceus/Harmonia; the experienced surface may not drift. Tests that only prove word presence, contract existence, or approximate semantics are not acceptance for port work. Every port tranche must cite or inspect the original source/living quarry, name the old behavior, reproduce it in Rust, and prove the same user-visible behavior once.

## Favorite and first-load manifest law

Original first load is governed by the Flask tab registry and favorite system: `flask-0-7-tabdrawing.md` loads `get_starred_tab() or get_first_visible_tab()` into the root template; `flask-0-8-favoritecomponent.md` loads `/api/get_starred_tab`, updates exactly one visible non-admin favorite, and updates the star UI. Coronatio records that law in the single HomeServer config, `homeserver.json`: `tabs.starred` selects the default route and `tabs.<tab>.config` plus `tabs.<tab>.visibility` define visible/admin tab state. `/api/favorites` and `/api/get_starred_tab` project that one config truth; they do not read a Coronatio sidecar favorites JSON. The star is the favorite/default-route control. The eye is the visibility control: in admin mode it hides or shows a tab without deleting, disabling, uninstalling, or marking it failed.
