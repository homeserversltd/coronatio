# UXPORT-000 Great Porting map

Governing law: THE OLD LOOK, REPRESENTED IN THE NEW UX LIBRARY. This slice mutates no pane markup. It records the quarry citations and the recomposition map so later tranches restore old React-visible fidelity without taste calls.

Doctrine readback:
- `workflow-coronatio-flask-react-visual-ux-identity-contract`: Coronatio is the Rust implementation, while visible graphics, controls, tab behavior, and UX remain identical to Flask/React until an approved divergence.
- `workflow-coronatio-flask-react-one-to-one-ux-ledger`: pane work must inspect Flask/React source and produce source-level/rendered inventories; button/control mismatch is incomplete port work.

Quarry roots inspected:
- Shared UI primitives: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/components/ui/`; UXPORT-001 uses `src/components/ui/{Breadcrumbs,FileInput,ProgressBar}.tsx`.
- Shared UI declarations: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/styles/common/ui/`; UXPORT-001 uses `src/styles/common/ui/{_breadcrumbs,_file-input,_progress-bar}.css`.
- Admin: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/admin/`
- Portals: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/portals/`
- Upload: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/upload/`
- Stats: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/stats/`

Current Coronatio surfaces inspected:
- `src/bands/shell/document-2.rs`
- `src/bands/shell/document-3.rs`
- `src/bands/shell/document-4.rs`
- `src/bands/full-rust-routes/upload.rs`
- `src/bands/full-rust-routes/portals.rs`
- `src/bands/full-rust-routes/power.rs`
- `src/bands/crown-law/stats-tabbar.rs`
- `src/bands/shell/test.rs` as the library primitive showcase and existing catalog proof

Classification key:
- RESTORE: og used a shared UI primitive from `styles/common/ui` or `components/ui`; current Rust approximated it or emitted legacy/local class vocabulary. Recompose onto the existing `ui-*` library primitive.
- ABSORB: og used tab-local CSS. Promote the exact CSS declarations into the library band as a named primitive/domain pack, then recompose onto that absorbed library name.
- FAITHFUL: current Rust already emits the library class or preserved og class that is already registered as library vocabulary.

No fourth bucket. A quarry gap is recorded as an element action finding, not a stop condition.

## Admin pane

OG citations: `src/tablets/admin/index.tsx`, `components/SystemControls.tsx`, `components/SystemControls.css`, `components/KeyManager.tsx`, `components/KeyManager.css`, `components/DiskManager.tsx`, `components/DiskManager.css`, `components/modals/*.tsx`, `components/modals/*.css`. Current citations: `src/bands/shell/document-2.rs` and modal substrate in `document-3.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `admin-tablet` from `admin/index.tsx` plus `section.mb-6` inline `marginBottom: 0.5rem` wrappers | `admin-tablet`; `admin-visual-port` retired | ABSORB | UXPORT-005: `packs/admin.css` born; pane root and inline section fold absorbed; markup emits og root and section wrappers without the Rust-only helper. |
| System controls container | `system-controls-container`, `system-controls` from `SystemControls.tsx` and `SystemControls.css` | `system-controls-container`, `system-controls` | ABSORB | UXPORT-005: system-controls declarations copied into `packs/admin.css`; names now library vocabulary. |
| System action buttons | `system-controls-btn` from `SystemControls.tsx` / tab-local CSS | `system-controls-btn`; `admin-quarry-button` retired for this row | ABSORB | UXPORT-005: `system-controls-btn` declarations absorbed; system buttons no longer carry the Rust-only helper. |
| SSH password toggle row | `ssh-controls`, `ssh-control`, `ssh-status`, `ssh-toggle`, `toggle-switch`, `toggle-slider`, `toggle-label`, `ssh-icon enabled/disabled` from `SystemControls.tsx` | same og class family hydrated by existing state placeholders | ABSORB | UXPORT-005: service-toggle declarations and enabled/disabled icon vocabulary absorbed; state machine behavior untouched. |
| SSH/Samba service toggles | `samba-control`, `samba-status`, `samba-toggle`, shared `toggle-*` classes from `SystemControls.tsx` | same og class family hydrated by existing service placeholders | ABSORB | UXPORT-005: Samba/SSH toggle declarations absorbed; service state/card behavior untouched. |
| Key manager root/layout | `key-manager`, `key-manager-content`, `key-manager-left`, `key-manager-right` from `KeyManager.tsx` / `KeyManager.css` | same root/layout classes | ABSORB | UXPORT-005: key-manager root/layout declarations copied into `packs/admin.css`; emitted names are library-owned. |
| Security status | `security-status`, `status-item`, `status-icon secure`, `status-details` | same classes | ABSORB | UXPORT-005: security-status declarations absorbed; no bespoke Rust style. |
| Key manager action buttons | `action-button info-button/create-button/update-button/admin-password-button` | `action-button create-button/update-button/admin-password-button`; helper retired | ABSORB | UXPORT-005: action-button variants absorbed; key buttons no longer carry `admin-quarry-button`. `info-button` remains an absorbed declaration for og/modal continuation even where no current key card emits it. |
| Disk manager root/layout | `disk-manager`, `disk-manager-container`, `disk-column`, `disk-list` from `DiskManager.tsx` / `DiskManager.css` | same root/layout classes | ABSORB | UXPORT-005: disk-manager declarations copied into `packs/admin.css`; two-column class stack stays og. |
| Disk item cards | `disk-item selected/mounted/non-standard-mount/unavailable/available/locked-pair/nas-compatible/unlocked-ready` | `disk-item` state family emitted by current runtime helpers where state exists | ABSORB | UXPORT-005: full disk-item state selector family absorbed; runtime data decides active state classes without behavior edits. |
| Disk metadata and badges | `disk-icon`, `disk-info`, `disk-name`, `nas-badge`, `nas-role-badge`, `disk-details`, `disk-space-usage`, `disk-serial`, `partition-info`, `mapper-info`, `filesystem-label` | same/partial names | ABSORB | UXPORT-005: disk metadata/badge declarations absorbed as admin domain primitives. |
| Disk action buttons | `action-button format/encrypt/assign-primary/assign-backup/unassign-nas/import-nas/permissions/unlock/mount/unmount/sync/auto-sync` | current Rust emits fewer `action-button` variants | ABSORB | UXPORT-005: disk action-button variant declarations absorbed; action restoration remains backend/behavior scoped and untouched. |
| Admin modals | `modal-*`, `legacy-installer-*`, `update-manager-*`, `rootca-*`, `hard-drive-test-*`, `sync-*`, `password-input-*`, `key-management-info-*`, `service-results-*`, `log-viewer-*` from `components/modals/*.css` | generic/modal-shelf preview classes | CONTINUATION | UXPORT-005 split honestly before modal fleet: first three admin pane bodies landed; modal rows remain continuation because og-true modal class-stack/size/stacking needs runtime DOM drill. |
| Basic buttons inside modals | Shared `Button` component ultimately emits `ui-button*` from `styles/common/ui/_button.css` where used by modal components | `primary-button`, `secondary`, `modal-actions` | CONTINUATION | Deferred with modal fleet; the button-family RESTORE/ABSORB split will cite each modal import/render tree before mutation. |

## Portals pane

OG citations: `src/tablets/portals/index.tsx`, `components/PortalCard.tsx`, `components/AddPortalCard.tsx`, `components/AddPortalModal.tsx`, `components/ServiceStatusModal.tsx`, `PortalCard.css`. Current citations: `src/bands/shell/document-4.rs`, `src/bands/full-rust-routes/portals.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `portals-tablet` from `portals/index.tsx` and `PortalCard.css` | `portals-tablet` restored in `document-2.rs` | ABSORB | UXPORT-004: `packs/portals.css` owns the domain root; markup now speaks the og pane root. |
| Grid | `portals-grid` from `portals/index.tsx` | `portals-grid` restored; old `portal-grid` helper drained | ABSORB | UXPORT-004: tab-local grid declaration copied into `packs/portals.css`; helper class retired from the portals pane. |
| Portal card | `portal-card ${service.status}` from `PortalCard.tsx`; declarations in `PortalCard.css` | `portal-card ${status}`; generic `card` removed from portal card stack | ABSORB | UXPORT-004: `portal-card=ABSORB`; status card uses og class stack and domain pack. |
| Portal card status | `portal-card.up/down/partial/unknown` | `portal-card ${status}` | ABSORB | UXPORT-004: status border selectors copied from `PortalCard.css` into `packs/portals.css`. |
| Portal card header/icon/name/description/meta | `portal-card-header`, `portal-icon`, `portal-name`, `portal-description`, `portal-meta` | same classes, with `portal-meta` wrapper restored around admin controls | ABSORB | UXPORT-004: card interior declarations absorbed; emitted classes match og. |
| Admin controls shell | `admin-controls`, `admin-controls-row`, `script-management-notice`, `script-notice-text` | `admin-controls`, `admin-controls-row`; `portal-admin-controls` retired | ABSORB | UXPORT-004: og admin-controls pack absorbed; script notice CSS present for later data cases; no VIS behavior changes. |
| Admin service buttons | buttons inside `.admin-controls button` from `PortalCard.css` | buttons inside `.admin-controls button` | ABSORB | UXPORT-004: button declarations remain the og domain selectors; no `ui-button` rename because og did not render shared Button here. |
| Visibility toggle | `visibility-toggle`, `fas fa-eye/-slash` in `portals/index.tsx` | `visibility-toggle` current state-machine markup retained | DEFERRED-to-VIS | Explicitly deferred: eye/star/admin-mode visibility recomposition stays for the VIS campaign; UXPORT-004 does not mutate toggle behavior or class stack. |
| Add portal card | `portal-card add-portal-card`, `add-portal-content`, `add-portal-icon`, `add-portal-title`, `add-portal-description` | restored caboose card, admin-gated by existing chrome | ABSORB | UXPORT-004: add-card pack absorbed and markup restored. |
| Add portal modal | `portal-modal-overlay`, `portal-modal-content`, `add-portal-modal`, `modal-header`, `close-button`, `portal-form`, `form-group`, `form-actions`, `cancel-button`, `submit-button` | restored hidden shell with delegated open/close; create endpoint labeled not-wired | ABSORB | UXPORT-004: og source rendered native inputs/select/buttons, not shared UI components, so modal/form declarations were absorbed and controls carry `aria-disabled` + `data-portal-create-not-wired`. |
| Service status modal | `service-status-modal`, `service-status-content`, `copy-button` | restored hidden shell; status action writes JSON into the og pre and copy button uses delegated chrome | ABSORB | UXPORT-004: service-status modal declarations absorbed; browser drill still proves size/scroll/pre text. |

## Upload pane

OG citations: `src/tablets/upload/index.tsx`, `components/DirectoryBrowser.tsx`, `components/DirectoryBrowser.css`, `components/BlacklistManager.tsx`, `components/UploadProgress.tsx`, `upload.css`. Current citations: `src/bands/shell/document-2.rs`, `src/bands/shell/document-4.rs`, `src/bands/full-rust-routes/upload.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `upload-tablet` from `upload/index.tsx` / `upload.css` | `upload-tablet` | ABSORB | Promote upload pane pack; current name becomes faithful only after library owns declarations. |
| Upload controls row | `upload-controls`, `file-upload-section`, `.file-upload-section button` from `upload/index.tsx` / `upload.css` | `upload-controls`, `file-upload-section` | ABSORB | Absorb upload controls layout; RESTORE individual file input/buttons to shared primitives where the old component used them. |
| File picker | og tablet owns native input/button in `upload/index.tsx`; shared library has `FileInput.tsx` and `_file-input.css` | `file-upload-section` native input/button | ABSORB | Declaration diff: differing. `_file-input.css` hides the native input, adds label/display controls, uses `border-radius: var(--border-radius, 8px)`, disabled display state, and 480px column layout; og `upload.css` styles the actual `input[type="file"]` and adjacent button. Keep og markup classes and absorb `file-upload-section` input/button declarations into the upload library pack. |
| Upload progress item | `upload-progress ${status}`, `upload-header`, `status-icon`, `filename`, `remove-button`, `progress-section`, `progress-bar-container`, `progress-bar`, `progress-text`, `upload-stats`, `speed`, `error-message` from `UploadProgress.tsx` / `upload.css` | same classes in `document-4.rs` | ABSORB | Declaration diff: differing. `_progress-bar.css` uses 24px/12px/40px geometry, `color: var(--background)`, `.85rem` text, and lighter text shadow; og `upload.css` progress uses 20px/10px/24px geometry, `color: var(--text)`, `.8em` text, and stronger shadow. Keep og progress markup and absorb the exact upload progress declarations. |
| Directory browser root/header | `directory-browser`, `directory-browser-header`, `directory-tree-container`, `directory-empty`, `directory-loading-initial`, `directory-error nas-unavailable` from `DirectoryBrowser.tsx` / CSS | route emits `directory-entry`, `directory-error`, `directory-error nas-unavailable`; root/header partial | ABSORB | Absorb directory-browser domain pack; restore missing root/header wrappers in markup tranche. |
| Breadcrumbs | `directory-breadcrumb-container`, `breadcrumb-navigation`, `breadcrumb-item current`, `breadcrumb-separator` from `DirectoryBrowser.tsx` / `DirectoryBrowser.css`; shared library has `Breadcrumbs.tsx` / `_breadcrumbs.css` | same og classes | ABSORB | Declaration diff: differing. `_breadcrumbs.css` adds `font-weight: bold` to current crumb, adds item `user-select` and focus outline, and moves the 480px font-size shrink to `.ui-breadcrumbs`; og tab-local current crumb is not bold and shrinks the container. Keep og markup classes and absorb breadcrumb declarations into the upload library pack. |
| Directory entries | `directory-entry selected/loading`, `tree-line vertical/horizontal`, `expand-control`, `entry-icon`, `entry-name`, `entry-selected` | same/partial classes | ABSORB | Promote directory tree/entry pack; current preserved classes become library vocabulary after absorption. |
| Directory admin buttons | `refresh-button`, `admin-button force-allow-button`, `admin-button set-default-button`, `admin-button blacklist-button`, `admin-button upload-history-button`, `toggle-pin-button active` | not all present; generic current buttons absent or approximated | RESTORE + ABSORB | Buttons should RESTORE onto `ui-button` variants if shared; special `toggle-pin-button` and admin path affordances ABSORB as upload domain selectors. |
| Blacklist manager | `blacklist-manager`, `blacklist-entries`, `blacklist-entry`, `entry-path`, `remove-entry`, `blacklist-controls`, `add-entry`, `entry-input`, `add-button`, `submit-button` from `BlacklistManager.tsx` / `upload.css` | `blacklist-entry`, `entry-path`, `remove-entry` partial | ABSORB | Absorb blacklist-manager pack; `entry-input` may later RESTORE to `ui-input` only if old declaration is intentionally replaced by shared input with identical visible result. |
| Upload history modal | `upload-history-modal-content`, `uploadHistoryModal empty`, `upload-history-empty-message`, `upload-history-list`, `history-item success/error`, `clear-history-button` | `history-item success/error` partial | ABSORB | Promote upload-history pack and restore clear/history modal around it. |
| PIN modal | `pin-modal-form`, `pin-input` | generic PIN modal in shell | ABSORB | Absorb upload PIN form class declarations or map to existing shared modal/input only with exact old-look proof. |

## Stats pane, including power fragment

OG citations: `src/tablets/stats/index.tsx`, `components/KeaLeasesTable.tsx`, `components/NetworkSpeedChart.tsx`, `components/MemoryRadialBar.tsx`, `components/ProcessUsageList.tsx`, `components/StatChart.tsx`, `components/CpuStatChart.tsx`, `components/DiskIoChart.tsx`, `components/DiskUsageChart.tsx`, `stats.css`, plus `styles/common/ui/_table.css`, `_progress-bar.css`, `_checkbox.css`, `_editable-field.css`, `_visibility-toggle.css`. Current citations: `src/bands/shell/document-2.rs`, `src/bands/shell/document-3.rs`, `src/bands/shell/document-4.rs`, `src/bands/full-rust-routes/power.rs`, `src/bands/crown-law/stats-tabbar.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `stats-tablet` from `stats/index.tsx` / `stats.css` | `stats-tablet` | ABSORB | UXPORT-003: stats pack born; og `stats.css` section copied and current holding-pen selectors drained into `ux/packs/stats.css`. |
| Stat card/root | `stat-element`, `stat-header`, `stat-title`, `stat-content` | same | ABSORB | UXPORT-003 receipt `stat-card=ABSORB`; tab-local stats.css declarations own the old look. |
| Visibility toggle | `visibility-toggle`, eye icon in `stats/index.tsx` | `visibility-toggle`, `eye-icon` in `stats-tabbar.rs` and stats fragment | RESTORE | Recompose onto `ui-visibility-toggle*` from `_visibility-toggle.css`; preserve old eye/eye-slash placement. |
| Chart host | `stat-chart`, `cpu-stats-container`, `cpu-chart`, `network-stats-container`, `network-speed-chart`, `disk-io-chart` plus current Chart.js `chart-container`/`coronatio-chart-canvas` inline fold | current Chart.js canvas hosts | ABSORB | UXPORT-003 receipt `chart-hosts=ABSORB`; old Recharts host geometry plus current canvas sizing folded into stats pack; browser drill still checks canvas/tooltips. |
| Chart tooltip/grid | `recharts-default-tooltip`, `recharts-tooltip-label`, `recharts-tooltip-item`, `recharts-cartesian-grid line` | current implementation renders Chart.js canvas, not Recharts DOM | ABSORB noted, deferred selectors unused | UXPORT-003: not copied as dead CSS; row retained as vendor quarry note until a rendered Recharts implementation exists. |
| Memory bars | `memory-stats`, `memory-current`, `memory-label`, `memory-bar`, `memory-bar-fill`, `memory-bar-fill-swap`, `memory-text`, `memory-details`; TSX inline `width` | same classes in `document-4.rs`; Rust sets `style.width` | FAITHFUL-after-diff + ABSORB surround | UXPORT-003: shared `_progress-bar.css` legacy aliases match bar/fill/text declarations; surrounding memory layout remains absorbed in `packs/stats.css`; markup keeps og classes. |
| Process bars | `process-usage-list`, `process-bar`, `process-bar-fill`, `process-text-container`, `process-name`, `process-usage`, loading/empty states; TSX inline `width` | same classes in `document-4.rs`; Rust sets `style.width` | FAITHFUL-after-diff + ABSORB surround | UXPORT-003: shared `_progress-bar.css` legacy aliases match process bar/fill; text/list selectors absorbed in stats pack. |
| Disk usage bars | `disk-usage-stats`, `disk-usage-item`, `disk-usage-header`, `disk-device`, `disk-mountpoint`, `disk-usage-bar`, `disk-usage-fill`, `disk-usage-details`; TSX inline `width` | same classes in `document-4.rs`; Rust sets `style.width` | FAITHFUL-after-diff + ABSORB surround | UXPORT-003: shared `_progress-bar.css` legacy aliases match bar/fill; disk surround absorbed in stats pack. |
| Network interface table | `network-interfaces`, `network-interfaces-table`, `interface-name`, `interface-label`, `data-cell` | same class stack in stats pane/renderNetwork | FAITHFUL-after-diff + ABSORB surround | UXPORT-003: `_table.css` owns `network-interfaces-table`; wrapper/interior labels absorbed in stats pack; markup keeps og alias. |
| Kea leases table | `kea-leases-table`, `device-note-cell`, `note-text`, `edit-note-button`, `edit-note-modal`, `note-textarea` | `kea-leases-table`, `device-note-cell`, `note-text`, `edit-note-button` | FAITHFUL-after-diff + ABSORB note pack | UXPORT-003: `_table.css` owns base table alias; note cell/button/modal/textarea declarations absorbed in stats pack; `ui-text-box` differed, so no rename. |
| Disk I/O checkboxes | `device-control`, `device-name`, `device-checkboxes`; native checkbox inputs in `DiskIoChart.tsx` | `device-control`, `device-name`, `device-checkboxes`, `drive-checkbox` labels with native inputs | ABSORB | UXPORT-003 diff: shared `_checkbox.css` requires `ui-checkbox__wrapper/input/checkmark` markup og never rendered; native checkbox feel/layout absorbed, no ui-* rename. |
| CPU load averages | `load-averages`, `load-average-values`, `load-average-item`, `load-label`, `load-value` | same class stack in CPU stats fragment | ABSORB | UXPORT-003 receipt `load-averages=ABSORB`; tab-local stats.css declarations own the old look. |
| Power meter modal/fragment | Runtime DOM truth supplied by operator: `<div class="modal" role="dialog" ...><div class="modal-title">Power Consumption</div><div class="modal-content"><div class="power-meter-modal"><div class="power-usage-display"><div class="power-value" style="color: var(--statusDown);"><span class="power-value-number">30.04</span><span class="power-value-unit">Watts</span></div></div><div class="power-history-section"><div class="power-averages"><div class="power-average-row"><div class="power-average-label">5s average:</div><div class="power-average-value">29.54W</div></div>... (30s, 60s rows)</div></div></div></div><button class="modal-close">×</button></div>`. Source widened outside tablets and found `src/components/StatusIndicators/PowerMeterIndicator.tsx`, `src/components/StatusIndicators/indicators.css`, `src/components/Modal/index.tsx`, and `src/components/Popup/PopupManager.tsx`; shared substrate signals are `modal`, `modal-title`, `modal-content`, `modal-close`, `data-popup-id`, and `data-stay-open`. | `document-2.rs`, `document-3.rs`, `full-rust-routes/power.rs` classes | ABSORB | Reclassify as a power-meter domain pack inside the shared modal substrate. Do not implement in UXPORT-001; later tranche must absorb the power-meter declarations from `components/StatusIndicators/indicators.css` while preserving shared modal substrate. |
| Stats tabbar checkboxes/star | shared `Tab`, `TabGroup`, `VisibilityToggle`, checkbox primitives in `components/ui` and `_tabs.css` / `_visibility-toggle.css` | `tab-visibility-column`, `visibility-toggle`, `star-button {star_class} fa-star`, `tab {active_class}` | RESTORE | Recompose crown-law tabbar onto `ui-tab*`, `ui-tab__visibility-toggle`, `ui-tab__star-button`, `ui-checkbox*` where used. |

## Placeholder legacy installer panes

These are not current port targets. They are included so eventual ports land directly on bedrock through the Adding a crown pane ladder.

| placeholder pane | og tab source | og library primitives / local packs | port map |
| --- | --- | --- | --- |
| backblaze | `/fulcrum/attachments/homeserver/initialization/flask/inject/legacy-tabs/backblazeTab/frontend/index.tsx`, `backblazeTab.css`, and child panels under `frontend/` | imports `Button`, `Card`, `Collapsible`, `Input`, `Select`, `Calendar`, `TimePicker`, `Tab`, `TabGroup`, `RowInfoTile`, `Breadcrumbs`; local backblaze bucket/file/ledger/forgejo packs | Start from shared UI primitives for controls; ABSORB tab-local backblaze domain declarations into a library `backblaze` pack before Rust pane markup. |
| wake-on-lan | `/fulcrum/attachments/homeserver/initialization/flask/inject/legacy-tabs/wakeonlan/frontend/index.tsx`, `PortalCard.css` | imports `Button`, `Checkbox`; local classes `wakeonlan-tablet`, `wakeonlan-section`, `wakeonlan-target-*`, `wakeonlan-lease-*`, `wakeonlan-message`, `wakeonlan-actions` | RESTORE `Button` and `Checkbox`; ABSORB wakeonlan list/message/action domain pack. |
| chia-mining | `/fulcrum/attachments/homeserver/initialization/flask/inject/legacy-tabs/miner/frontend/index.tsx`, `styles/dashboard.css`, `styles/miners.css`, plus miner modal/card components | local classes `miner-tablet`, `fleet-header`, `fleet-stats`, `stat-box`, `fleet-actions`, `action-btn`, `error-banner`, `empty-state`, `miners-grid`; components `MinerCard`, `ClaimMinerModal`, `PoolConfigModal`, `FleetControls` | ABSORB miner dashboard/card/modal packs; where buttons/inputs are not shared, either migrate to `ui-button`/`ui-input` with exact look proof or absorb as miner pack. |
| dhcp | `/fulcrum/attachments/homeserver/initialization/flask/inject/legacy-tabs/dhcp/frontend/index.tsx`, `PortalCard.css`, `components/DhcpCard.tsx`, `components/ReservationSlider.tsx` | local `dhcp-tablet`, `dhcp-info-banner`, `dhcp-button-row`, `anonymize-toggle-*`, `dhcp-list-*`, `mac-input`, `ip-input`, `add-reservation-button`; slider/card components | RESTORE shared toggle/slider/input/checkbox primitives where exact; ABSORB DHCP list/banner/card pack. |

## VIS deferred audit pointer

- VIS-000 opens IRIS in `docs/iris-transition-table.md`: the citation-true og eye transition table and architecture preamble for the deferred visibility-toggle/eye/star/admin-mode behavior rows.

## Proposed tranche order

1. Upload breadcrumbs + progress proof slice. Risk: low-to-medium. It has clear existing shared primitives (`Breadcrumbs`, `FileInput`, `ProgressBar`) and current Rust already emits many og upload classes. This establishes RESTORE and ABSORB method on a contained pane without broad admin modal overlap.
2. Stats bars/tables/checks slice. Risk: medium. `styles/common/ui/_progress-bar.css`, `_table.css`, and `_checkbox.css` already carry legacy aliases and explicit stats-tab style comments. Browser drill is required for chart geometry and responsive table behavior.
3. Portals card/add/modal slice. UXPORT-004 published: `packs/portals.css` born, portal pane/card/grid/add/modal/service-status look-alikes absorbed, VIS state-machine recomposition deferred.
4. Admin controls/key/disk slice. Risk: high. It has the largest modal/action surface and service/disk state matrix. Split into SystemControls, KeyManager, DiskManager, then admin modals if needed.
5. Power fragment. Risk: unknown. Current Rust has power classes, but this audit did not find a React power pane source; treat as quarry-gap until a source/runtime citation is recovered.

Overlap risks:
- `action-button`, `submit-button`, `cancel-button`, and `modal-*` names recur across panes but are not automatically shared primitives. Each tranche must cite whether the declaration came from `styles/common/ui`, `styles/common`, or tab-local CSS before RESTORE/ABSORB.
- Some current Rust classes preserve og names, but this is not FAITHFUL until the declarations are library-owned. Preserved names with tab-local CSS are ABSORB.
- `card` is not a valid substitute for og `portal-card`, `disk-item`, or `stat-element` unless the absorbed/library class stack proves identical rendered geometry.

## Required wall shape for later tranches

Each implementation tranche must carry:

1. Source wall: exact og paths read, with line/selector citations for every changed element.
2. Library wall: each adopted class is asserted as one of:
   - existing shared `ui-*` primitive from `styles/common/ui` with a per-element declaration-diff receipt proving byte-equivalence modulo whitespace, or
   - absorbed domain primitive with byte-identical CSS declarations copied from the og tab-local source into the library band.
   RESTORE requires that declaration-diff receipt; a shared-component sighting alone is not enough.
3. Markup wall: per-pane assertions that the Rust pane emits only library vocabulary for the changed surface, including forbidden old Rust helper classes where applicable (`admin-quarry-button`, `portal-admin-controls`, generic `card` in place of domain card, or missing `ui-*` roots).
4. Non-drift wall: exact markup/class assertions are necessary but not sufficient. The tranche names the old and new class stacks and proves no unclassified class remains.
5. Browser drill wall: only a browser can prove final visual identity for geometry, responsive layout, hover/focus, modal stacking, scroll behavior, chart/canvas/SVG sizing, file input rendering, checkbox/toggle feel, and history/log overflow. The browser drill must compare old quarry or captured old runtime against Coronatio at the same viewport/theme/session/data state.
6. Cargo wall: full Rust suite remains green; the expected baseline for this slice is `cargo test` green (98 tests on main today).

## UXPORT-001 operator browser-drill checklist

- Breadcrumbs: ABSORB; inspect `directory-breadcrumb-container > breadcrumb-navigation > breadcrumb-item.current` and verify the current crumb is not bold while hover/spacing matches og.
- File picker: ABSORB; inspect native `input[type="file"]` plus adjacent upload button inside `file-upload-section`, not the shared FileInput wrapper/display control.
- Upload progress: ABSORB; inspect `progress-bar-container > progress-bar > progress-text` for 20px/10px/24px og geometry and text color/shadow.


## UXPORT-003 operator browser-drill checklist

- Bars geometry: memory/process/disk bars keep 24px height, 12px radius, 40px min fill, and inline width behavior.
- Table layouts: network and Kea tables keep alias class layouts, Kea note cell sizing, mobile card behavior, and MAC wrapping/truncation.
- Checkbox feel: disk I/O controls keep native og checkbox inputs and label spacing; no `ui-checkbox*` wrapper appeared.
- Chart canvas/tooltips: current Chart.js hosts keep 180px canvas sizing; compare tooltip/grid behavior against old source/runtime at the same viewport/theme/data.
- Holding pen: `ux/shell/document-2-css.css` has zero stats selectors after drain; `ux/packs/stats.css` carries `holding-pen drain` and declaration-diff receipt comments.

## UXPORT-004 operator browser-drill checklist

- Card grid geometry: `portals-tablet > portals-grid` uses auto-fill 300px columns, 20px gap, full width, and the add-card remains the caboose.
- Status border colors: `portal-card.up/down/partial/unknown` resolve to `--statusUp`, `--statusDown`, `--statusPartial`, and `--statusUnknown` at same theme/session state.
- Card face: `portal-card-header`, 120px `portal-icon`, centered `portal-name`, clipped `portal-description`, and `portal-meta` match og; no generic `card` helper governs portal geometry.
- Add-card affordance: `portal-card add-portal-card` hover/focus, plus-circle, title, and description match og.
- Modal stacking/size: `portal-modal-overlay > portal-modal-content > add-portal-modal` and `service-status-modal` occupy the og overlay/max-width/max-height/scroll behavior.
- Admin-controls row: `admin-controls > admin-controls-row` buttons keep og grid and hover; `portal-admin-controls` is absent.
- Deferred VIS: visibility toggle/eye/admin-mode behavior remains current and is not acceptance for this tranche.

## Quarry gaps found

- Power quarry gap closed by operator-supplied runtime DOM and widened source search. The React source is outside tablets: `src/components/StatusIndicators/PowerMeterIndicator.tsx`, `src/components/StatusIndicators/indicators.css`, `src/components/Modal/index.tsx`, and `src/components/Popup/PopupManager.tsx`. Power remains a later ABSORB tranche.
- Placeholder legacy installer panes often import shared UI components and also carry tab-local `PortalCard.css` or domain CSS. Their eventual Rust ports should begin by classifying those imported primitives before writing pane markup.

## UXPORT-005 operator browser-drill checklist

- System controls: `admin-tablet > section.mb-6[style="margin-bottom: 0.5rem"] .system-controls-container > .system-controls > .system-controls-btn` keeps og flex wrap, 180px button geometry, hover/active depth, and no `admin-quarry-button` helper on the first action row.
- Service toggles: SSH password, SSH service, and Samba rows render `ssh-status`/`samba-status`, `toggle-switch`, `toggle-slider`, `toggle-label`, and `ssh-icon`/`samba-icon enabled|disabled`; PIN/token/service behavior is unchanged.
- Key manager: `key-manager-content` remains two-column desktop with sticky `key-actions`, `security-status > status-item > status-icon secure`, and absorbed `action-button` variants without the helper class.
- Disk manager: `disk-manager-container` remains two-column; `disk-item` state colors/glows/labels (`selected`, `mounted`, `available`, `locked-pair`, `nas-compatible`, `unlocked-ready`) resolve from `packs/admin.css`; runtime helper state is unchanged.
- Holding pen: `packs/admin.css` is loaded before `shell/document-2-css.css`; system/key/disk declarations live in the pack. Remaining holding-pen selectors are modal continuation or non-admin/session substrate.
- Modal fleet: continuation row; compare modal stacking/sizes/buttons in the next slice before removing modal helper classes.
