# FLOOD-001 pageload API fetch flood RCA

## Executive finding

FLOOD-001 had two independent client flood lanes.

Lane 1 was the VIS-003 JavaScript re-entry loop: `refreshElementFragment()` called `setAdminMode(headerState.isAdmin)` after every stats/portals element fragment swap. `setAdminMode()` is a network/session refresh function, so every element refresh re-fetched the tab bar and both element fragments, indefinitely.

Lane 2 was the surviving HTMX graph loop in the real browser: the `/api/tab-bar` fragment re-rendered the active tab with `hx-trigger="load, click"`. Every JavaScript or pulse-driven tab-bar re-swap therefore inserted a fresh load-triggered element. HTMX executed the load trigger on the new active tab, fetched `/admit/stats`, swapped the stats pane, and the `htmx:afterSwap` handler hydrated stats. Because the tab-bar fragment itself could be re-swapped repeatedly, this became an unconditional fragment-level re-fire lane even after the JS re-entry call was split out.

The translator's Chromium receipt against head `f861843a` proves lane 2 survived the first fix: 1404 `/api/*` requests in six seconds from one page load, dominated by `/api/tab-bar` 557, `/api/stats/elements` 279, `/api/portals/elements` 279, and `/api/stats` 279. That shape is not a lawful startup burst or interval cadence.

## Exact cycles

### Lane 1: VIS-003 JavaScript re-entry, fixed at `f861843a`

1. The shell bootstrap calls `setAdminMode(headerState.isAdmin)` during page load (`src/bands/shell/document-3.rs:393` at the first FLOOD-001 head; the function body is `src/bands/shell/document-3.rs:48-65`).
2. `setAdminMode()` calls `refreshTabBar(previousActive)`, then `refreshElementFragment('stats')` and `refreshElementFragment('portals')` (`src/bands/shell/document-3.rs:59-63`).
3. `refreshTabBar()` fetches `/api/tab-bar?active=<tab>` and swaps the returned HTML into the tab bar (`src/bands/shell/document-3.rs:435-443`).
4. Pre-fix `refreshElementFragment(tabId)` fetched `/api/stats/elements` or `/api/portals/elements`, swapped the response, then called `setAdminMode(headerState.isAdmin)` again at `src/bands/shell/document-4.rs@9052ddaa:529-540`.
5. That line re-entered step 2. Each pass generated at least one `/api/tab-bar` fetch, one stats-element fetch, one portals-element fetch, and a stats hydration fetch.

```text
setAdminMode
  -> refreshTabBar -> /api/tab-bar
  -> refreshElementFragment(stats) -> /api/stats/elements -> setAdminMode  # pre-fix
  -> refreshElementFragment(portals) -> /api/portals/elements -> setAdminMode # pre-fix
  -> repeats
```

### Lane 2: HTMX active-tab fragment load cycle, fixed in this bounce

1. The tab-bar route serves `/api/tab-bar` through `tab_bar_fragment_route()` and `tab_bar_html_response_with_active()` (`src/bands/routes.rs:240-256`).
2. At head `f861843a`, that response called `render_plan_tabbar_with_active(session, active)` (`src/bands/routes.rs@f861843a:251-253`). That renderer used the same active-tab load-triggered markup as the full page.
3. The active tab renderer emitted `hx-get="/admit/{id}"`, `hx-target="[data-view-panel='{id}']"`, `hx-swap="innerHTML"`, and `hx-trigger="load, click"` when `grant.tab_id == active_tab` (`src/bands/crown-law/stats-tabbar.rs@f861843a:714-719`).
4. Therefore any `/api/tab-bar` swap inserted a new active tab carrying an HTMX load trigger.
5. HTMX processes load triggers when fresh markup enters the DOM. The fresh active tab fetched `/admit/stats`, targeting `[data-view-panel='stats']` and swapping the stats pane.
6. The after-swap handler then runs `hydrateStats()` for the stats panel and `hydratePortals()` for the portals panel (`src/bands/shell/document-2.rs:254-260`). `hydrateStats()` calls `/api/stats`; `hydratePortals()` reaches `refreshElementFragment('portals')`, which calls `/api/portals/elements` (`src/bands/shell/document-4.rs:558-562`).
7. Separately, the session/pulse/tab refresh lane keeps re-swapping `/api/tab-bar` (`src/bands/shell/document-3.rs:482-485`), so the fragment load trigger is re-seated and re-fired instead of remaining a one-time initial pane admission.

That graph explains the translator's ratio: repeated tab-bar swaps are the high-count driver; every other active-pane load produces stats pane hydration plus element fragment fetches, yielding the observed `/api/tab-bar` about 2x the stats/element families.

```text
/api/tab-bar fragment
  -> active tab hx-trigger="load, click" inserted
  -> HTMX GET /admit/stats into [data-view-panel='stats']
  -> htmx:afterSwap(stats) -> hydrateStats() -> /api/stats
  -> htmx/session/pulse refreshes reseat /api/tab-bar fragment
  -> repeats because the fragment still carried a fresh load trigger
```

## Introduction history

`git log -S 'setAdminMode(headerState.isAdmin)' -- src/bands/shell/document-4.rs` identifies commit `9052dda` (`VIS-003 portals and stats elements ride iris plan`) as the introduction of the lane-1 recursive element-refresh call. `git blame -L 529,541 -- src/bands/shell/document-4.rs` attributed the whole pre-fix `refreshElementFragment()` body, including the re-entry call, to `9052ddaa`.

The HTMX active load trigger was introduced earlier by the tab plan. `git blame -L 717,720 -- src/bands/crown-law/stats-tabbar.rs` at `f861843a` attributes the active-tab `hx-trigger` rendering to `a8ab3eec` (`CONTRACT VIS-002 session membrane tab plan`). It is lawful only for the first full-page render, where it performs the initial active pane admission once. It is unlawful in the repeatedly served `/api/tab-bar` fragment because every fragment swap creates a fresh HTMX load trigger.

The after-swap hydration handler predates VIS-003 as HTMX substrate. `git blame -L 254,260 -- src/bands/shell/document-2.rs` attributes that handler to `7e06eb62` (`HX-001 resurrect HTMX engine spine beneath og shell`). It is lawful when panes swap intentionally; it becomes part of the flood when the tab-bar fragment keeps re-seating active load triggers.

## Blast radius

- Server load: every open browser can drive hundreds of `/api/*` route executions per second. The loop hits `/api/tab-bar`, `/api/stats/elements`, `/api/portals/elements`, `/api/stats`, `/api/status`, `/api/status/power/usage`, `/api/themes`, `/api/favorites`, `/api/uptime`, and `/api/upload/pin-required-status` in operator drill evidence.
- Receipt/log pollution: the loop repeatedly traverses projection and state-read routes that are intended to be meaningful readbacks, burying legitimate events in noise.
- Client cost: laptops/tablets/phones burn CPU, battery, and network on page load without user action.
- Live estate exposure: the defect is in the crown shell served at `home.arpa`, so any live client that loads Coronatio can become a flood source against the home server.

## Fix included

The fix is a two-lane graph break, not a debounce.

Lane 1 remains fixed by splitting DOM-only admin visibility projection from the network/session refresh function:

- `applyAdminDomState()` only updates `data-admin-mode`, admin button text/state, `data-admin-only` visibility, and the change-PIN button (`src/bands/shell/document-3.rs:35-47`).
- `setAdminMode()` still owns session mode, logout invalidation, tab-bar refresh, and the one intentional stats/portals element refresh after a session change (`src/bands/shell/document-3.rs:48-65`).
- `refreshElementFragment()` and `toggleElementVisibility()` call `applyAdminDomState()` after swaps instead of re-entering `setAdminMode()` (`src/bands/shell/document-4.rs:529-556`).

Lane 2 is fixed by splitting full-page tab-bar markup from served fragment markup:

- `render_plan_tabbar_with_active()` remains the full-page renderer and passes `active_load_trigger=true`, preserving the lawful first-page active pane load (`src/bands/crown-law/stats-tabbar.rs:647-655`).
- New `render_plan_tabbar_fragment_with_active()` passes `active_load_trigger=false` for fragment responses (`src/bands/crown-law/stats-tabbar.rs:651-655`).
- `/api/tab-bar` now calls `render_plan_tabbar_fragment_with_active()`, so the served fragment contains only click triggers and never reseats an HTMX load trigger (`src/bands/routes.rs:251-256`).
- `render_plan_tab_grant()` now emits `hx-trigger="load, click"` only when the tab is active and the caller explicitly allows active load triggers; otherwise it emits `hx-trigger="click"` (`src/bands/crown-law/stats-tabbar.rs:703-728`).

This preserves og look/feel: tab order, tab chrome, eyes/stars, active selection, targets, and click admission remain unchanged. Only the unsafe fragment-time `load` trigger is removed; first full-page render still carries the initial active-pane `load, click` trigger.

## Walls

- `tests::flood_001_wall_fragment_refresh_does_not_reenter_admin_network_refresh` asserts lane 1 remains acyclic: the DOM-only projector exists, session changes still refresh stats/portals once, and element refresh/toggle bodies do not call `setAdminMode(headerState.isAdmin)`.
- `tests::flood_001_wall_tab_bar_fragment_is_acyclic_no_load_trigger` asserts the actual served `/api/tab-bar?active=stats` response preserves `hx-get="/admit/stats"`, contains no `hx-trigger="load`, and the full first-page shell still contains `hx-trigger="load, click"` for initial pane admission.

## Reproduction and proof receipts

The first FLOOD-001 local harness used a drill homeserver JSON fixture at `/var/opt/hermes/workspace/flood-001-homeserver.json`, the Cibation worktree binary on `CORONATIO_PORT=18090`, and a local Node harness that loaded the real served `/static/crown/chrome.js`, used real `fetch()` against the running server, and counted `/api/*` routes while providing a minimal DOM.

That harness did not execute the HTMX attribute graph or real browser swapped-markup processing. Its previous 10-request post-fix number was valid only for the JavaScript re-entry lane, not for the browser HTMX fragment lane. This RCA therefore does not claim a new request-rate proof from that harness. Rate proof for the HTMX graph remains the translator's Playwright/Chromium rig unless/until a local browser-capable proof lane is available.

Current source/test receipts:

```text
/fulcrum/cli.py lib test-env run --worktree /fulcrum/attachments/coronatio/.worktrees/flood_001 -- cargo test flood_001_wall_fragment_refresh_does_not_reenter_admin_network_refresh -- --nocapture
-> blocked by test-env missing-pyproject-toml (Rust repo fallback used)

cargo test flood_001_wall -- --nocapture
-> 2 passed; 180 filtered out

cargo test
-> 182 passed; 0 failed
```

## Out-of-scope observation

The local harness also reported a separate `pulseRenewTimer` temporal-dead-zone throw after `setAdminMode()` starts its async work. That is not the fetch-flood root cause: the pre-fix flood continued after the throw, and the two cycles above are sufficient to reproduce the request storm. This RCA leaves that pulse-ordering defect untouched because FLOOD-001 scope is docs plus the minimal flood cycle breaks.
