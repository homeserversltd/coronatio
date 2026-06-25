# Tests band

Recursive child band for crate-private Coronatio tests. `src/bands/tests.rs` is the wrapper that imports test support and includes these child bands.

- `core-routes.rs` — Core route, shell, pane, and manifest tests.
- `registry-stats.rs` — Registry, startup, lane, fallback, session, and stats tests.
- `topics-storage.rs` — Topic, SSE, API boundary, installer, and frontend storage tests.
- `service-transactions.rs` — Service data, registry transaction, and API contract tests.
- `helpers.rs` — Shared test helpers.
