# Browser proof harness

`node tests/browser/buttery-convergence.mjs` runs the dependency-free Chromium CDP wall for the buttery convergence slice. It builds no production fixture behavior: the harness starts the existing Coronatio binary with temporary `homeserver.json` and `systemctl` fixture paths, then removes the server, browser profile, and fixtures on every exit.

It requires Node 22+, `/usr/bin/chromium`, and an already-built `target/debug/coronatio`. `BUTTERY_TIMEOUT_MS` bounds the entire run; `CORONATIO_BROWSER_BIN` and `CHROMIUM` may override executable paths for a deliberate local runner.
