# Coronatio source bands

This directory is the infinite-infinite preservation spine for Coronatio's Rust source.

`src/main.rs` is now the thin process/crate face. It imports shared crates and includes these ordered bands at crate root so existing private item visibility and runtime behavior remain unchanged during the first split.

The ordered spine lives in `index.json`. Later tranches may promote individual bands to ordinary Rust modules after their typed interfaces are explicit and proven.

## Band order

- `contracts.rs` — Constants and contract/readback structs shared by Coronatio bands.
- `server.rs` — Process entry and Axum router spine.
- `routes.rs` — HTTP route handlers and native API response faces.
- `legacy-proxy.rs` — Legacy HomeServer asset, Unix-socket proxy, chunk decoding, and route-boundary behavior.
- `tab-manifests.rs` — Dynamic tab manifest loading and validation routes.
- `crown-readbacks.rs` — Crown pane, registry, startup, lane, fallback, and admin-session readbacks.
- `topic-readbacks.rs` — Socket/SSE topic and monitor-pulse readback contracts.
- `frontend-storage.rs` — Frontend persistence and service-data migration readbacks.
- `installer-stats.rs` — Boundary, installer, registry, and stats snapshot readbacks.
- `legacy-shell.rs` — Exact legacy HomeServer shell rendering plus safety/shutdown helpers.
- `tests.rs` — Unit tests preserving Coronatio behavior through the split.
