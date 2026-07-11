# Page-load fetch flood

## Symptom

A single Coronatio page load could produce more than a thousand API requests in a few seconds. The tab bar, Stats data, and Stats and Portals element fragments repeated without user input. Each open browser consumed CPU, network, and server work while useful readbacks disappeared into noise.

## Cause

Two independent refresh cycles met in the browser.

First, an element-fragment refresh called the full admin-session refresh function after every swap. That function fetched the tab bar and both element fragments, whose completion entered the same function again.

Second, the tab-bar fragment marked the active tab with HTMX's `load` trigger. Replacing the tab bar inserted a fresh active-tab element, so HTMX treated it as a new page-load instruction and admitted the active pane again. That pane swap hydrated Stats and Portals, while session and pulse updates could replace the tab bar once more.

Both mistakes confused two kinds of work: projecting already-known state into the DOM and performing a network refresh.

## Fix

The repair broke both cycles at their owning edges:

- `applyAdminDomState()` now changes browser-visible admin state without fetching anything.
- `setAdminMode()` remains the intentional session transition and performs its bounded refresh once.
- Full-page tab markup may use `hx-trigger="load, click"` to admit the initial pane.
- Replacement tab-bar fragments use click-only triggers, so inserting a refreshed bar cannot masquerade as another page load.

The tab order, controls, active selection, and click behavior did not change. Only the accidental re-entry paths were removed.

## Prevention

Two regression tests guard the graph rather than a guessed request-rate threshold:

- element-fragment refreshes must not re-enter the admin network refresh;
- `/api/tab-bar` fragments must contain no load trigger, while the first full-page shell must retain the one intentional initial load.

This incident also established a review rule: any function that both swaps DOM and starts network work must name those responsibilities separately, and any HTMX `load` trigger must be checked in the context of repeated fragment insertion.

## Verification

Run the focused walls or the full suite:

```bash
cargo test flood_001_wall -- --nocapture
cargo test
```

The focused test currently covers both cycle breaks. A real-browser request census remains the strongest end-to-end confirmation because a minimal JavaScript harness does not execute HTMX's inserted-markup behavior.
