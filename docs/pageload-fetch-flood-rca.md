# FLOOD-001 pageload API fetch flood RCA

## Executive finding

The runaway page-load flood was a client-side re-entrancy loop in the Coronatio crown chrome. VIS-003 made `refreshElementFragment()` call `setAdminMode(headerState.isAdmin)` after every stats/portals element fragment swap. `setAdminMode()` is not a DOM-only projector: it re-fetches the tab bar and both element fragments. Therefore every element refresh scheduled a new session refresh, which scheduled two more element refreshes, indefinitely.

This is not explained by the lawful interval cadences. The declared periodic loops are `hydrateInternetIndicator()` every 1000ms and `refreshPowerIndicator()` every 5000ms in `src/bands/shell/document-3.rs:388-392`. The observed local pre-fix harness produced 215 `/api/*` requests in 1s, including 51 `/api/stats/elements`, 51 `/api/portals/elements`, 73 `/api/tab-bar`, and 36 `/api/stats`, which is orders of magnitude beyond those two intervals.

## Exact cycle

Pre-fix source cycle, using the offending committed lines:

1. The shell bootstrap calls `setAdminMode(headerState.isAdmin)` during page load (`src/bands/shell/document-3.rs:393` at current head; the function body is `src/bands/shell/document-3.rs:48-65`).
2. `setAdminMode()` calls `refreshTabBar(previousActive)`, then `refreshElementFragment('stats')` and `refreshElementFragment('portals')` (`src/bands/shell/document-3.rs:59-63`).
3. `refreshTabBar()` fetches `/api/tab-bar?active=<tab>` and swaps the returned HTML into the tab bar (`src/bands/shell/document-3.rs:432-443`).
4. The tab-bar fragment includes the active tab as an HTMX load trigger: `hx-get="/admit/{id}"`, `hx-target="[data-view-panel='{id}']"`, `hx-swap="innerHTML"`, and `hx-trigger="load, click"` for the active tab (`src/bands/crown-law/stats-tabbar.rs:717-719`). Replacing the tab bar therefore re-seats a load-triggered active tab.
5. `refreshElementFragment(tabId)` fetches `/api/stats/elements` or `/api/portals/elements`, swaps the response into `[data-stats-viewport]` or `[data-portals-grid]`, and before this fix called `setAdminMode(headerState.isAdmin)` at `src/bands/shell/document-4.rs@9052ddaa:529-540`.
6. That line re-entered step 2. Each pass generated at least one `/api/tab-bar` fetch, one stats-element fetch, one portals-element fetch, and a stats hydration fetch. Because stats and portals both re-entered, the graph fanned out instead of waiting for a timer.
7. HTMX amplified the visible symptom: the active tab's `hx-trigger="load, click"` was lawful for one initial pane admission, but re-swapping the tab bar repeatedly reprocessed that active element and caused repeated `/admit/stats`/after-swap work. The after-swap handler then calls `hydrateStats()` for stats panels and `hydratePortals()` for portals panels (`src/bands/shell/document-2.rs:254-260`), adding `/api/stats` and `/api/portals/elements` traffic to the same loop.

In short:

```text
setAdminMode
  -> refreshTabBar -> /api/tab-bar -> active hx-trigger="load" re-seated
  -> refreshElementFragment(stats) -> /api/stats/elements -> setAdminMode  # pre-fix
  -> refreshElementFragment(portals) -> /api/portals/elements -> setAdminMode # pre-fix
  -> repeats
```

## Introduction history

`git log -S 'setAdminMode(headerState.isAdmin)' -- src/bands/shell/document-4.rs` identifies commit `9052dda` (`VIS-003 portals and stats elements ride iris plan`) as the introduction of the recursive element-refresh call. `git blame -L 529,541 -- src/bands/shell/document-4.rs` attributed the whole pre-fix `refreshElementFragment()` body, including the re-entry call, to `9052ddaa`.

The older HTMX active load trigger was introduced earlier by the tab plan. `git blame -L 717,720 -- src/bands/crown-law/stats-tabbar.rs` attributes the current active-tab `hx-trigger` rendering to `a8ab3eec` (`CONTRACT VIS-002 session membrane tab plan`). That trigger is not by itself the root defect; it becomes a flood amplifier only when the tab bar is repeatedly swapped by the VIS-003 re-entry loop.

The after-swap hydration handler predates VIS-003 as HTMX substrate. `git blame -L 254,260 -- src/bands/shell/document-2.rs` attributes that handler to `7e06eb62` (`HX-001 resurrect HTMX engine spine beneath og shell`). It is also not independently defective; it becomes part of the flood when the loop repeatedly swaps HTMX panes/tab chrome.

## Blast radius

- Server load: every open browser can drive hundreds of `/api/*` route executions per second. The loop hits `/api/tab-bar`, `/api/stats/elements`, `/api/portals/elements`, `/api/stats`, `/api/status`, `/api/status/power/usage`, `/api/themes`, `/api/favorites`, `/api/uptime`, and `/api/upload/pin-required-status` in the operator's drill evidence.
- Receipt/log pollution: the loop repeatedly traverses projection and state-read routes that are intended to be meaningful readbacks, burying legitimate events in noise.
- Client cost: laptops/tablets/phones burn CPU, battery, and network on page load without user action.
- Live estate exposure: the defect is in the crown shell served at `home.arpa`, so any live client that loads Coronatio can become a flood source against the home server.

## Fix included

The fix is the smallest cycle break: split DOM-only admin visibility projection from the network/session refresh function.

- New `applyAdminDomState()` only updates `data-admin-mode`, admin button text/state, `data-admin-only` visibility, and the change-PIN button (`src/bands/shell/document-3.rs:35-47`).
- `setAdminMode()` still owns session mode, logout invalidation, tab-bar refresh, and the one intentional stats/portals element refresh after a session change (`src/bands/shell/document-3.rs:48-65`).
- `refreshElementFragment()` and `toggleElementVisibility()` now call `applyAdminDomState()` after swaps instead of re-entering `setAdminMode()` (`src/bands/shell/document-4.rs:529-556`).

This preserves rendering and lawful cadences. No CSS, markup shape, tab order, text, HTMX active-tab trigger, or interval cadence was changed.

## Wall

`tests::flood_001_wall_fragment_refresh_does_not_reenter_admin_network_refresh` asserts the trigger graph stays acyclic at the known seam:

- the DOM-only `applyAdminDomState()` helper exists;
- session changes still refresh stats and portals once;
- `refreshElementFragment()` and `toggleElementVisibility()` use `applyAdminDomState()`;
- neither function contains `setAdminMode(headerState.isAdmin)`.

## Reproduction and proof receipts

Local reproduction used a drill homeserver JSON fixture at `/var/opt/hermes/workspace/flood-001-homeserver.json`, the Cibation worktree binary on `CORONATIO_PORT=18090`, and a local Node harness that loads the real served `/static/crown/chrome.js`, uses real `fetch()` against the running server, and counts `/api/*` routes while providing a minimal DOM. Chromium could not launch on this body (`DevToolsActivePort`/trace-breakpoint), so this is an ad-hoc browser-logic harness rather than Playwright.

Before fix, the harness counted 215 `/api/*` requests in 1000ms:

```json
{
  "/api/portals/elements": 51,
  "/api/stats": 36,
  "/api/stats/elements": 51,
  "/api/status": 2,
  "/api/status/power/usage": 1,
  "/api/tab-bar": 73,
  "/api/themes": 1
}
```

After fix, the same harness counted 10 `/api/*` requests in 4000ms:

```json
{
  "/api/portals/elements": 1,
  "/api/stats": 1,
  "/api/stats/elements": 1,
  "/api/status": 4,
  "/api/status/power/usage": 1,
  "/api/tab-bar": 1,
  "/api/themes": 1
}
```

The remaining `/api/status` cadence is the declared 1s internet indicator loop; `/api/status/power/usage` is the declared 5s power indicator loop.

Test receipts:

```text
/fulcrum/cli.py lib test-env run --worktree /fulcrum/attachments/coronatio/.worktrees/flood_001 -- cargo test flood_001_wall_fragment_refresh_does_not_reenter_admin_network_refresh -- --nocapture
-> blocked by test-env missing-pyproject-toml (Rust repo fallback used)

cargo test flood_001_wall_fragment_refresh_does_not_reenter_admin_network_refresh -- --nocapture
-> 1 passed; 180 filtered out

cargo test
-> 181 passed; 0 failed
```

## Out-of-scope observation

The local harness also reports a current separate `pulseRenewTimer` temporal-dead-zone throw after `setAdminMode()` starts its async work. That is not the fetch-flood root cause: the pre-fix flood continues after the throw, and the cycle above is sufficient to reproduce the request storm. This RCA leaves that pulse-ordering defect untouched because FLOOD-001 scope was docs plus the minimal flood cycle break.
