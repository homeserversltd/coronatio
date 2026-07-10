# VIS-000 IRIS transition table

This artifact opens the IRIS visibility campaign. It is an executable-spec seed for VIS-001: audit first, doctrine before implementation, no code wiring in this slice.

## A. Preamble: operator rulings and architecture

1. NAME. The visibility organ is IRIS: the aperture of the eye, deciding how much light each viewer is admitted. It is kin to Hermes and Caduceus in the estate pantheon. The campaign tranche namespace is VIS-000..; the organ name is `iris`.

2. ARCHITECTURE. IRIS is one pure total function:

```text
plan(config, session) -> RenderPlan
```

Stored facts are only `config.tabs.<tab>.config.adminOnly`, `config.tabs.<tab>.config.isEnabled`, `config.tabs.<tab>.visibility.tab`, `config.tabs.<tab>.visibility.elements`, `config.tabs.starred`, and the request/session mode. Everything else is derived. Render states are `Absent | Visible | DimmedHidden`. Renderers consume the plan; no renderer receives an `is_admin` boolean. Session is an enum constructed at the request membrane in exactly one place.

Scope-path keys:

```text
tab:<id>
<tab>/element:<eid>
```

RenderPlan grammar:

```text
RenderPlan = {
  session: Guest | Admin,
  tabs: [TabGrant],
  elements: [ElementGrant],
  starred: tab:<id> | fallback,
  fallback: { injected: bool, active: bool, reason: string | null }
}
TabGrant = { key: tab:<id>, state: Absent | Visible | DimmedHidden, eye: bool, star: bool, starEligible: bool }
ElementGrant = { key: <tab>/element:<eid>, state: Absent | Visible | DimmedHidden, eye: bool }
```

3. PARADIGM. IRIS is construction under policy, never filtration after construction. Config semantics remain og-faithful default-VISIBLE: absence means shown; the eye records exceptions. Pipeline mechanics are default-DENY: nothing renders except off an explicit grant in the plan. The great filter is dead; the great projector replaces it. Guests never receive markup or data they are not granted; hiding-with-CSS is heresy.

4. INVARIANTS BY TRANSACTION. The visibility write path applies the toggle, restores invariants, commits atomically, and renders the response fragment from the new plan. Starred must always be eligible and is re-derived inside the same write. No optimistic update, rollback, or debounce ports; those are og compensations that die with the paradigm.

5. PUSH-TO-POKE, PULL-TO-PROJECT. SSE events carry no data, only invalidation pokes. Every client re-fetches through its own session membrane and receives its own projection. Per-socket payload filtering ceases to exist as a category.

6. FIELD-FILTERING SCOPE. Per-topic admin field projection (`publicIp`, `credentials`, `processes`, and similar fields) is excluded from VIS. It is a later post-SSE campaign using the same iris organ as allowlist JSON projection. Keep `admin_field_filters` and `monitor_topic_laws` contracts as recorded law; do not build them in VIS.

7. BUG FIDELITY. Security-class og bugs are approved divergence and never port: (a) `system_stats` process leak from top-level key deletion missing nested `cpu.top_processes`; (b) PIN-substring/prefix token fallback bypass in `backend/auth/validation.py:197-205`; (c) unprotected `/api/crypto/getKey`. The port law demands identical look and feel, not identical vulnerabilities. UX-visible ambiguities are settled by runtime receipts, not judgment.

Doctrine readback used: `workflow-coronatio-flask-react-visual-ux-identity-contract` and `workflow-coronatio-flask-react-one-to-one-ux-ledger` were returned by the live Paligenesis router for coronatio/visibility work; they require Flask/React visual/behavioral identity until an approved divergence, and source-level inventories before porting.

## B. Stored-facts schema and absence semantics

Og config grammar is rooted at `tabs`. `backend/tabman/README.md:19-58` records tabs keyed by id, each with `config`, `visibility`, and `data`, plus special `tabs.starred`. `src/config/homeserver.json:2-185` provides live quarry examples.

| fact | type / values | og citations | actual og default / absence behavior |
| --- | --- | --- | --- |
| `tabs` | object keyed by tab id, plus special `starred` | `README.md:19-58`; `routes.py:17-27` | `/api/tabs` uses `config.get('tabs', {})`, forces non-dict to `{}`, excludes `starred`, excludes disabled tabs by `config.isEnabled` false, and only includes entries with a `config` object (`routes.py:17-27`). |
| `config.displayName` | string | `README.md:23-28`; `homeserver.json:4-9,21-26,139-144,159-164` | Display text is read with optional chaining in React (`TabBar/index.tsx:56-58`); absent text renders empty/undefined-looking React text, not a separate policy. |
| `config.adminOnly` | boolean | `README.md:23-28`; `homeserver.json:4-9` | Frontend treats only literal `true` as admin-only in access (`tabSlice.ts:225-228`) and star exclusion (`favoriteSlice.ts:50-53,178-182,217-225`). Backend star reject defaults missing to false (`routes.py:134-136`). Thus absence means non-admin tab. |
| `config.order` | number, lower first | `README.md:23-28`; `tabSlice.ts:135-151,157-162` | Missing order sorts as `999` (`tabSlice.ts:135-151,157-162`). Fallback is hardcoded `999` (`tabSlice.ts:43-52`; `routes.py:30-40`). |
| `config.isEnabled` | boolean | `README.md:23-28`; `homeserver.json:4-9` | Backend `/api/tabs` includes only entries whose config `.get('isEnabled', False)` is truthy (`routes.py:23-27`). Frontend access treats missing as enabled in `isTabVisible` (`visibilitySlice.ts:306-308`) but star eligibility often defaults missing to false (`favoriteSlice.ts:50-53,178-182,217-225`). IRIS must settle this at the plan layer. |
| `visibility.tab` | boolean | `README.md:29-34,69-72`; `homeserver.json:10-15,27-42,145-151,165-176` | Frontend initialization requires `tab.visibility.tab` to be boolean; missing or malformed visibility defaults hidden in local state (`visibilitySlice.ts:76-91`). Runtime access requires local `state.visibility[tabId]?.tab === true` (`tabSlice.ts:222-233`) and `isTabVisible` falls back false (`visibilitySlice.ts:306-308`). Backend tab write creates missing `visibility` then writes the provided value (`routes.py:199-204`). Backend element write creates missing `visibility` as `{tab: true, elements: {}}` (`routes.py:259-266`). |
| `visibility.elements` | object keyed by element id to boolean | `README.md:29-34,69-72`; `homeserver.json:27-42,165-176` | Frontend element visibility requires tab visible, then defaults missing element entry to visible (`visibilitySlice.ts:289-294`). Backend `@visibility_required` instead defaults missing visibility or missing element to false (`auth/utils.py:30-39`). Backend element write creates missing elements object and stores `bool(visibility)` (`routes.py:259-267`). |
| `tabs.starred` | string tab id or `fallback` | `README.md:56-58,77-80`; `homeserver.json:184` | `/api/tabs` reads `tabs.get('starred', 'fallback')`, then falls back if not present in valid tabs or disabled (`routes.py:42-48`). Frontend eligible star derives fallback when no visible tabs, keeps current visible/enabled non-fallback, else first visible (`favoriteSlice.ts:19-36`). |
| `global.admin.pin` | string/number compared as string | `homeserver.json:186-199`; `validation.py:97-125` | PIN validation reads `global.admin.pin`, rejects absent, and compares string forms exactly for `/api/validatePin` (`validation.py:107-125`). Token fallback later contains approved-divergence substring/prefix bugs (`validation.py:197-205`). |

Source-only edge cases: frontend config absence is not uniformly default-visible; tab visibility absence hides in initialization/access, while element entry absence shows in frontend but denies in backend decorators. These conflicts are marked in section E for runtime/operator settlement before VIS-001 wall generation.

## C. Exhaustive transition table

Legend: `A=adminOnly`, `E=isEnabled`, `V=visibility.tab`, `G=guest`, `M=admin session`. `eye` means an eye/slash toggle is rendered. `star` means a star button is rendered. `eligible` means the tab may be selected as starred.

Og proof basis:
- Admin mode tab list shows all tabs except fallback, sorted by order (`tabSlice.ts:143-152`). Guest mode uses `hasTabAccess` (`tabSlice.ts:153-163`).
- Guest access is `visibility.tab === true` plus non-admin-or-admin-session plus `isEnabled !== false` (`tabSlice.ts:214-233`).
- `TabBar` drops non-admin hidden tabs (`TabBar/index.tsx:356-359`) and renders tabs from `visibleTabs` (`TabBar/index.tsx:343-374`).
- Eye renders only when `isAdmin && !tab.config?.adminOnly` (`TabBar/index.tsx:41-53`).
- Star renders only when `isVisible && !tab.config?.adminOnly` (`TabBar/index.tsx:59-69`), where `isVisible` comes from `isTabVisible` (`TabBar/index.tsx:351-354`) and `isTabVisible` is `visibility && isEnabled` (`visibilitySlice.ts:296-315`).
- Star eligibility excludes fallback special-case excepted in the local `canStarTab`, and otherwise requires enabled and non-admin-only (`TabBar/index.tsx:295-298`); the backend also rejects disabled, hidden, and admin-only starred tabs (`routes.py:127-136`).

### Per-tab table

| A | E | V | session | render state | eye rendered? | star rendered? | star-eligible? | citations |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| false | true | true | guest | Visible | no | yes | yes | Guest list/access: `tabSlice.ts:153-163,214-233`; star: `TabBar/index.tsx:59-69`; eligibility: `routes.py:127-136`. |
| false | true | true | admin | Visible | yes | yes | yes | Admin all-tabs: `tabSlice.ts:143-152`; eye: `TabBar/index.tsx:41-53`; star: `TabBar/index.tsx:59-69`. |
| false | true | false | guest | Absent | no | no | no | Guest access false on visibility: `tabSlice.ts:222-233`; null gate: `TabBar/index.tsx:356-359`; backend star hidden reject: `routes.py:131-132`. |
| false | true | false | admin | DimmedHidden | yes | no | no | Admin list ignores visibility: `tabSlice.ts:143-152`; `data-visibility` records hidden: `TabBar/index.tsx:35-40`; eye slash source: `TabBar/index.tsx:41-53`; star suppressed by `isVisible`: `TabBar/index.tsx:59-69`. Visual dimming is ambiguous; see E1. |
| false | false | true | guest | Absent | no | no | no | `/api/tabs` excludes disabled tabs: `routes.py:23-27`; access also requires enabled: `tabSlice.ts:229-233`; backend star disabled reject: `routes.py:127-129`. |
| false | false | true | admin | DimmedHidden | yes | no | no | Admin list can include disabled only if already present client-side (`tabSlice.ts:143-152`), but `/api/tabs` excludes disabled before normal hydration (`routes.py:23-27`); `isTabVisible` false when disabled (`visibilitySlice.ts:306-315`); eye gate ignores enabled (`TabBar/index.tsx:41-53`). This is a source-level edge cell. |
| false | false | false | guest | Absent | no | no | no | Disabled filtered by `/api/tabs` (`routes.py:23-27`) and hidden fails access (`tabSlice.ts:222-233`). |
| false | false | false | admin | DimmedHidden | yes | no | no | Same admin/client-side caveat as disabled+visible; if present, admin list includes it (`tabSlice.ts:143-152`), eye renders because not admin-only (`TabBar/index.tsx:41-53`), star suppressed by `isTabVisible` (`visibilitySlice.ts:306-315`). |
| true | true | true | guest | Absent | no | no | no | Guest access denies admin-only: `tabSlice.ts:225-233`; null gate: `TabBar/index.tsx:356-359`; star admin-only reject: `routes.py:134-136`. |
| true | true | true | admin | Visible | no | no | no | Admin list includes admin-only (`tabSlice.ts:143-152`); eye suppressed for admin-only (`TabBar/index.tsx:41-53`); star suppressed for admin-only (`TabBar/index.tsx:59-69`). |
| true | true | false | guest | Absent | no | no | no | Hidden and admin-only both fail access: `tabSlice.ts:222-233`; null gate: `TabBar/index.tsx:356-359`. |
| true | true | false | admin | DimmedHidden | no | no | no | Admin list includes it (`tabSlice.ts:143-152`); `data-visibility=false` (`TabBar/index.tsx:35-40`); eye/star both suppressed by admin-only (`TabBar/index.tsx:41-53,59-69`). Visual dimming ambiguous; see E1. |
| true | false | true | guest | Absent | no | no | no | Disabled excluded by backend (`routes.py:23-27`) and guest access requires enabled (`tabSlice.ts:229-233`). |
| true | false | true | admin | Visible | no | no | no | Source caveat: backend normally excludes disabled (`routes.py:23-27`); if present client-side, admin list includes all non-fallback (`tabSlice.ts:143-152`), but eye/star are suppressed by admin-only (`TabBar/index.tsx:41-53,59-69`). |
| true | false | false | guest | Absent | no | no | no | Disabled/hidden/admin-only all deny guest (`routes.py:23-27`; `tabSlice.ts:222-233`). |
| true | false | false | admin | DimmedHidden | no | no | no | Same disabled source caveat; admin list includes client-side entry (`tabSlice.ts:143-152`), hidden recorded in `data-visibility` (`TabBar/index.tsx:35-40`), controls suppressed by admin-only (`TabBar/index.tsx:41-53,59-69`). |

### Fallback tab and injected fallback law

| condition | render / state | citations |
| --- | --- | --- |
| `/api/tabs` normal success | Backend injects `fallback` with display `produced by HOMESERVER LLC`, enabled true, adminOnly false, visible true, order 999 | `routes.py:29-40`. |
| config file missing | Backend returns only injected fallback and `starredTab=fallback` | `routes.py:54-71`. |
| frontend initial state | Store starts with only `fallback`, active `fallback`, visible true | `tabSlice.ts:43-53,75-78`. |
| frontend config initialization | Merges server tabs with fallback regardless of input | `tabSlice.ts:190-203`. |
| no visible tabs at initialization | Activates fallback mode | `tabSlice.ts:205-211`. |
| visibility initialization creates zero visible rows | Forces fallback visible | `visibilitySlice.ts:101-112`. |
| star fallback | Backend allows `fallback` without further validation | `routes.py:93-114`; frontend eligible-star returns `fallback` on zero visible tabs (`favoriteSlice.ts:19-25`). |

### Element-level rows: portals elements

Og proof basis:
- Portals elements are keyed by portal service name and read with `checkElementVisibility('portals', portal.name)` (`portals/index.tsx:45-56`).
- Remote access without a dynamic URL returns null before visibility policy (`portals/index.tsx:57-63`).
- Non-admin hidden elements return null (`portals/index.tsx:82-83`).
- Admin sees all portal elements and receives eye/slash button (`portals/index.tsx:84-104`).
- Toggling performs PUT `/api/tabs/elements`, then updates local state; there is no rollback on backend failure beyond a toast/log (`portals/index.tsx:65-80`).
- Frontend element visibility defaults missing element to visible when tab is visible (`visibilitySlice.ts:289-294`); backend decorator defaults missing to false (`auth/utils.py:30-39`).

| tab visible? | element `visibility.elements[eid]` | session | render state | eye rendered? | citations |
| --- | --- | --- | --- | --- | --- |
| true | true | guest | Visible | no | `visibilitySlice.ts:289-294`; `portals/index.tsx:82-104`. |
| true | true | admin | Visible | yes | `portals/index.tsx:84-104`. |
| true | false | guest | Absent | no | `portals/index.tsx:82-83`. |
| true | false | admin | DimmedHidden | yes | Admin eye/control and `data-visible=false`: `portals/index.tsx:84-104`; visual dimming depends CSS/runtime. |
| true | missing | guest | Visible in frontend; backend-decorated data endpoints deny by default | no | Frontend default true: `visibilitySlice.ts:289-294`; backend default false: `auth/utils.py:30-39`. |
| true | missing | admin | Visible | yes | Frontend default true and admin eye: `visibilitySlice.ts:289-294`; `portals/index.tsx:84-104`. |
| false | any | guest | Absent | no | Element visibility first rejects invisible tab: `visibilitySlice.ts:289-294`; non-admin null gate: `portals/index.tsx:82-83`. |
| false | any | admin | DimmedHidden | yes | Admin renders all local portal cards if remote URL passes, with `data-visible` and eye: `portals/index.tsx:84-104`; tab invisibility makes `checkElementVisibility` false through `visibilitySlice.ts:289-294`. |

### Element-level rows: stats elements

The translator's ambiguity note called per-stat eyes a Coronatio document-2 invention. Quarry source proves per-stat eyes did exist in og stats: `src/tablets/stats/index.tsx:50-77` toggles `/api/tabs/elements` and renders `button.visibility-toggle`; `src/tablets/stats/index.tsx:121-203` wraps `cpu-chart`, `io-section`, `disk-usage`, and `kea-leases`; `src/config/homeserver.json:165-176` lists `cpu-chart`, `network-chart`, `disk-usage`, `memory-usage`, `process-list`, `io-section`, and `kea-leases` in config. The specific source line for WAN network uses `elementId="network"` (`stats/index.tsx:129-140`), while config names `network-chart` (`homeserver.json:168-170`); this mismatch is an ambiguity to settle.

| tab visible? | element visible? | session | render state | eye rendered? | citations |
| --- | --- | --- | --- | --- | --- |
| true | true/missing | guest | Visible | no | `stats/index.tsx:61-83`; missing element frontend default: `visibilitySlice.ts:289-294`. |
| true | true/missing | admin | Visible | yes | `stats/index.tsx:61-83`. |
| true | false | guest | Absent | no | `stats/index.tsx:61-64`. |
| true | false | admin | DimmedHidden | yes | `stats/index.tsx:61-83`. |
| false | any | guest | Absent | no | Element visibility rejects invisible tab (`visibilitySlice.ts:289-294`) and StatElement nulls non-admin hidden (`stats/index.tsx:61-64`). |
| false | any | admin | DimmedHidden | yes | Admin eye/render path: `stats/index.tsx:61-83`. |

## D. Compound transitions

| transition | before | after / target order | og citations | IRIS replacement law |
| --- | --- | --- | --- | --- |
| Hide currently-starred regular tab as admin | `starredTab == tabId`, tab currently visible | Local visibility flips optimistically after 100ms debounce, then `handleVisibilityChange` derives visible regular tabs excluding fallback/admin-only, picks first visible non-hidden tab if still visible, else fallback; backend `setstarredtab` persists the new star | Debounce/update: `visibilitySlice.ts:118-227`; visible regular tab derivation: `favoriteSlice.ts:38-55`; hide-star branch: `favoriteSlice.ts:72-100`; backend star write: `routes.py:79-173`. | VIS write transaction applies tab visibility, derives eligible star in the same transaction, commits once, and renders from the new plan. |
| Exit admin mode while on admin-only tab | Current active tab is not accessible to guest | After 150ms, recompute visible non-admin tabs; if current inaccessible, call `getEligibleStarredTab(currentVisibleNonAdminTabs)` and `tabManager.setActiveTab(nextTabId, 'store')`; if none, activate fallback | `adminModeManager.ts:269-325`; access/visible tabs: `tabSlice.ts:153-163,214-233`; eligible star: `favoriteSlice.ts:19-36`. | Session changes re-run `plan(config, Guest)` and pick current-if-granted else plan-star/fallback. |
| Exit admin mode while on hidden regular tab | Active tab has `visibility.tab=false` | Same path as inaccessible: current not in guest visible tabs, so next is eligible starred/first visible/fallback | `adminModeManager.ts:281-325`; hidden access false: `tabSlice.ts:222-233`. | Same as above; no event cascade. |
| Star then hide | Starred tab is set, then its visibility becomes false | `setStarredTab` validates visible/enabled/non-admin before writing; later hide triggers visibility cascade and re-stars first visible/fallback | Set star validation: `favoriteSlice.ts:165-277`; backend rejects disabled/hidden/admin-only: `routes.py:127-136`; hide cascade: `visibilitySlice.ts:158-164`; `favoriteSlice.ts:72-100`. | One write may not leave ineligible star stored. |
| Hide all regular tabs | Last visible non-admin tab hidden | Favorite slice sets `starredTab='fallback'`; fallback mode can activate when no visible tabs | `favoriteSlice.ts:57-70,186-199`; fallback activation init/exit: `tabSlice.ts:205-211`; `adminModeManager.ts:286-289`. | Plan injects fallback when no guest-visible regular tabs. |
| Enter admin | Guest becomes admin after PIN/token success | `enterAdminMode` validates `/api/validatePin`, stores token, sets `isAdmin`, clears lockout, sets socket admin; manager enforces 1s transition cooldown and WS re-auth when connected with admin token | `adminSlice.ts:55-88`; cooldown: `adminModeManager.ts:37-39,179-201`; WS re-auth: `adminModeManager.ts:107-121`. | VIS-002 creates one request-membrane session enum; no renderer boolean threading. |
| PIN lockout ladder | Failed PIN or validation error | Failed attempts increment; lockout is `min(5min, 1000*2^(attempt-1 capped at exponent 8))`; lockout returns false until elapsed | `adminSlice.ts:34-35,55-121`. | Preserve UX-visible lockout ladder in VIS-002 session membrane. |
| Element toggle in portals/stats | Admin clicks an element eye | PUT `/api/tabs/elements`; local element state updates after success; no optimistic rollback lane, just error toast/log | `portals/index.tsx:65-80`; `stats/index.tsx:50-58`; backend write: `routes.py:233-296`. | Transaction writes element grant and returns new projection fragment. |
| SSE/WebSocket data filtering | Mixed/admin-only broadcast channels | Admin-only broadcasts skip non-admin sockets; mixed broadcasts delete registered admin fields per SID | Register/filter: `events.py:157-163,249-281`; admin-only emit checks: `events.py:471-492`; generic per-SID emit/filter: `events.py:355-391`. | Excluded from VIS; later post-SSE projection campaign. VIS-004 pokes only. |

## E. Ambiguous cells needing receipts

| id | ambiguity | source finding | status |
| --- | --- | --- | --- |
| E1 | Admin dimmed-hidden opacity: which CSS wins at each viewport? | CSS has `nav[data-admin-mode="true"] .tab.hidden { opacity: 1; ... }` (`TabBar.css:201-206`) and mobile override opacity `.7` (`TabBar.css:343-348`). But JSX emits `className="tab active?"` and `data-visibility`, not `hidden`/`admin-visible` (`TabBar/index.tsx:35-40,361-374`); `.tab-bar .tab.admin-visible` also appears in CSS (`TabBar.css:329-341`) with no matching source emission found. | HYPOTHESIS. Browser could not launch in this body; curl runtime cannot compute CSS. |
| E2 | Per-stat-card eyes: og or Coronatio invention? | Og source proves they existed: `StatElement` renders admin eye buttons and non-admin hidden null (`stats/index.tsx:50-83`). However config/source key mismatch exists: config has `network-chart` (`homeserver.json:168-170`) while React uses `elementId="network"` (`stats/index.tsx:129-140`). | SOURCE-RECEIPT for existence; key mismatch HYPOTHESIS. |
| E3 | Absence semantics edge cases | Frontend tab visibility absence hides (`visibilitySlice.ts:76-91,306-308`); frontend element entry absence shows (`visibilitySlice.ts:289-294`); backend element decorator absence denies (`auth/utils.py:30-39`); backend element write creates tab visible on missing visibility (`routes.py:259-266`). | HYPOTHESIS for UX-visible runtime if malformed config appears; no live mutation allowed. |
| E4 | Rapid-toggle behavior visible to user | Tab toggle is debounced 100ms and optimistic/rollback-capable (`visibilitySlice.ts:118-227`); portal/stat element toggles wait for PUT then update local state (`portals/index.tsx:65-80`; `stats/index.tsx:50-58`). | HYPOTHESIS; no safe live mutation. |
| E5 | Disabled tab admin rendering | Backend `/api/tabs` filters disabled out before hydration (`routes.py:23-27`), while admin frontend list would show disabled if present client-side (`tabSlice.ts:143-152`). | SOURCE-EDGE; runtime requires throwaway config, not live estate. |

## F. Og runtime receipts

Reachability finding: og runtime is reachable by curl from this body.

```text
curl -k -I https://home.arpa -> HTTP/2 200, server nginx, date Mon, 06 Jul 2026 04:47:01 GMT
curl -k -sS https://home.arpa/ -> Vite/React shell with /assets/index-BRoXzIjg.js and /assets/index-Co-PYpJ8.css
curl -k -sS https://home.arpa/api/tabs -> JSON with starredTab="portals", injected fallback, admin/portals/stats/upload and legacy installer tabs
```

Observed non-mutating runtime facts:
- `/api/tabs` returns injected `fallback` with `visibility.tab=true`, matching `routes.py:29-40`.
- `/api/tabs` returns `starredTab="portals"`, matching `routes.py:42-52`.
- Runtime confirms `global.admin.pin` is not needed for read-only `/api/tabs`; no admin token was requested and no state was mutated.

Receipt split for E-cells:
- Receipt-backed by source and read-only runtime/API: E2 existence of per-stat eyes in source; fallback injection and live `/api/tabs` shape.
- Hypothesis due no safe observational DOM/browser or no throwaway config: E1, E3, E4, E5, and E2 network key mismatch.

The browser tool failed before launch on this body (`Chrome exited early ... DevToolsActivePort`), so viewport CSS receipts were not captured. No live estate config mutation was attempted.

## G. Death row: mechanisms that do not port

| og mechanism | og citation | death-row reason | IRIS replacement |
| --- | --- | --- | --- |
| Optimistic local tab visibility update | `visibilitySlice.ts:127-156` | Compensates for post-construction cascade and async persistence. | Transactional write derives plan from committed state. |
| Rollback on failed tab visibility write | `visibilitySlice.ts:166-208` | Rollback is a symptom of optimistic mutation. | Write either commits atomically or returns error; old plan remains. |
| 100ms debounce and starring queue | `visibilitySlice.ts:13-19,158-164,211-227` | Timing artifact should not define policy. | One authoritative write path with invariant restoration. |
| Event-cascade star repair | `TabBar/index.tsx:224-246`; `favoriteSlice.ts:38-163` | Star invariant is repaired by multiple client listeners. | Re-derive star inside same server transaction. |
| Per-socket payload filtering | `events.py:249-281,355-391,471-492` | VIS-004 carries pokes only; clients pull their own projection. | SSE invalidation only; HTTP projection through session membrane. |
| Renderer `isAdmin` boolean threading | `TabBar/index.tsx:13,43,86-92,361-373`; `portals/index.tsx:50-56,82-104`; `stats/index.tsx:61-83` | Renderers are not policy engines. | Renderers consume `RenderPlan` grants; session enum exists only at request membrane. |
| Current Coronatio doc-3/localStorage lane | Campaign ruling | Browser-local privilege/session storage is not a policy authority. | VIS-002 server-side session from `global.admin.pin`, random token, hardcoded token string removed. |
| PIN substring/prefix token fallback | `validation.py:197-205` | Approved security divergence. | Exact token validation only; no PIN-containing token acceptance. |
| `system_stats` nested process leak | `events.py:487-492` records admin field families; operator ruling 7a supplies bug class | Approved security divergence. | Later projection allowlist must remove nested admin fields by schema. |
| Unprotected `/api/crypto/getKey` | Operator ruling 7c | Approved security divergence. | Do not port; protect in the proper Caduceus/security campaign. |

## H. VIS ladder readback

1. VIS-001: implement the pure `iris::plan(config, session) -> RenderPlan` organ, exhaustive-enumeration walls generated from sections C and D, and trace-replay walls. Wire it to nothing.
2. VIS-002: implement the real session membrane: `validatePin` against `global.admin.pin`, random token, server-side session set; remove the hardcoded `coronatio-session-token` string. Put the tab bar on the plan.
3. VIS-003: put portals and stats elements on the same visibility grammar.
4. VIS-004: convert SSE/WebSocket behavior to push-to-poke and pull-to-project.

## Mechanical row inventory for VIS-001

- Per-tab rows: 16 state combinations plus fallback law rows.
- Element rows: 8 portals rows and 6 stats rows.
- Compound transition rows: 8.
- Ambiguous cells: 5 total; 1 source-receipted existence finding plus 5 hypothesis/edge items (E2 has both a receipt-backed existence finding and a key-mismatch hypothesis).

