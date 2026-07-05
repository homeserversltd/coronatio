# UXPORT-000 Great Porting map

Governing law: THE OLD LOOK, REPRESENTED IN THE NEW UX LIBRARY. This slice mutates no pane markup. It records the quarry citations and the recomposition map so later tranches restore old React-visible fidelity without taste calls.

Doctrine readback:
- `workflow-coronatio-flask-react-visual-ux-identity-contract`: Coronatio is the Rust implementation, while visible graphics, controls, tab behavior, and UX remain identical to Flask/React until an approved divergence.
- `workflow-coronatio-flask-react-one-to-one-ux-ledger`: pane work must inspect Flask/React source and produce source-level/rendered inventories; button/control mismatch is incomplete port work.

Quarry roots inspected:
- Shared UI primitives: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/components/ui/`
- Shared UI declarations: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/styles/common/ui/`
- Admin: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/admin/`
- Portals: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/portals/`
- Upload: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/upload/`
- Stats: `/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/stats/`
- Premium placeholders: `/fulcrum/attachments/homeserver/initialization/flask/inject/premium/{backblazeTab,wakeonlan,miner,dhcp,youtube}/frontend/`

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
| Pane root | `admin-tablet` from `admin/index.tsx` | `admin-tablet admin-visual-port` | ABSORB | Promote `admin-tablet` pane pack from admin tab CSS; remove `admin-visual-port` after matching evidence is in the library. |
| System controls container | `system-controls-container`, `system-controls` from `SystemControls.tsx` and `SystemControls.css` | `system-controls-container`, `system-controls` | ABSORB | Absorb the system-controls domain pack byte-for-byte; keep names as library vocabulary. |
| System action buttons | `system-controls-btn` from `SystemControls.tsx` / tab-local CSS | `system-controls-btn admin-quarry-button` | ABSORB | Absorb `system-controls-btn`; delete the Rust-only helper once library domain pack owns the visual declaration. |
| SSH password toggle row | `ssh-controls`, `ssh-control`, `ssh-status`, `ssh-toggle`, `toggle-switch`, `toggle-slider`, `toggle-label`, `ssh-icon enabled/disabled` from `SystemControls.tsx` | `ssh-controls`, `ssh-control` plus approximated static state | ABSORB | Absorb SSH control/toggle pack; later tranche must restore the full row and enabled/disabled icon state vocabulary. |
| SSH/Samba service toggles | `samba-control`, `samba-status`, `samba-toggle`, shared `toggle-*` classes from `SystemControls.tsx` | `ssh-control`, `samba-control` partial | ABSORB | Absorb service-toggle domain pack and recompose all service rows onto it. |
| Key manager root/layout | `key-manager`, `key-manager-content`, `key-manager-left`, `key-manager-right` from `KeyManager.tsx` / `KeyManager.css` | same root/layout classes | ABSORB | Promote key-manager domain pack; current emitted names become faithful only after library owns declarations. |
| Security status | `security-status`, `status-item`, `status-icon secure`, `status-details` | same classes | ABSORB | Absorb key-manager security-status declarations; no bespoke Rust style. |
| Key manager action buttons | `action-button info-button/create-button/update-button/admin-password-button` | `action-button create-button/update-button/admin-password-button admin-quarry-button` | ABSORB | Absorb `action-button` variants; remove `admin-quarry-button` and use absorbed variants. |
| Disk manager root/layout | `disk-manager`, `disk-manager-container`, `disk-column`, `disk-list` from `DiskManager.tsx` / `DiskManager.css` | same root/layout classes | ABSORB | Promote disk-manager domain pack; byte-identical declarations preserve old selectable two-column disk appliance. |
| Disk item cards | `disk-item selected/mounted/non-standard-mount/unavailable/available/locked-pair/nas-compatible/unlocked-ready` | `disk-item` family partial | ABSORB | Absorb full disk-item state pack and restore every state selector. |
| Disk metadata and badges | `disk-icon`, `disk-info`, `disk-name`, `nas-badge`, `nas-role-badge`, `disk-details`, `disk-space-usage`, `disk-serial`, `partition-info`, `mapper-info`, `filesystem-label` | same/partial names | ABSORB | Absorb disk metadata/badge pack as named library domain primitives. |
| Disk action buttons | `action-button format/encrypt/assign-primary/assign-backup/unassign-nas/import-nas/permissions/unlock/mount/unmount/sync/auto-sync` | current Rust emits fewer `action-button` variants | ABSORB | Absorb the action-button variants; later markup tranche restores missing actions onto library names. |
| Admin modals | `modal-*`, `premium-tab-*`, `update-manager-*`, `rootca-*`, `hard-drive-test-*`, `sync-*`, `password-input-*`, `key-management-info-*`, `service-results-*`, `log-viewer-*` from `components/modals/*.css` | generic `modal`, `modal-body`, `modal-actions`, service status modals in `document-3.rs` | ABSORB | Promote modal domain packs in tranche-specific order; generic modal may stay only where og used the shared modal substrate. |
| Basic buttons inside modals | Shared `Button` component ultimately emits `ui-button*` from `styles/common/ui/_button.css` where used by modal components | `primary-button`, `secondary`, `modal-actions` | RESTORE | Recompose shared-button cases onto `ui-button ui-button--{variant}`; do not absorb duplicate button declarations unless tab-local CSS actually owned the old look. |

## Portals pane

OG citations: `src/tablets/portals/index.tsx`, `components/PortalCard.tsx`, `components/AddPortalCard.tsx`, `components/AddPortalModal.tsx`, `components/ServiceStatusModal.tsx`, `PortalCard.css`. Current citations: `src/bands/shell/document-4.rs`, `src/bands/full-rust-routes/portals.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `portals-tablet` from `portals/index.tsx` and `PortalCard.css` | generated portal fragment inside shell; no stable pane root in inspected route | ABSORB | Promote `portals-tablet` domain root and require later route/pane root to speak it. |
| Grid | `portals-grid` from `portals/index.tsx` | no full grid class found in current emitted fragment | ABSORB | Absorb grid declarations from tab-local CSS; restore grid root in markup tranche. |
| Portal card | `portal-card ${service.status}` from `PortalCard.tsx`; declarations in `PortalCard.css` | `card portal-card ${status}` | ABSORB | Promote `portal-card` as a portals domain card. Current `card` helper must give way to the absorbed class unless exact og class stack includes a library card variant. |
| Portal card status | `portal-card.up/down/partial/unknown` | `portal-card ${status}` | ABSORB | Absorb byte-identical status border selectors. |
| Portal card header/icon/name/description/meta | `portal-card-header`, `portal-icon`, `portal-name`, `portal-description`, `portal-meta` | same classes | ABSORB | Promote card interior pack; current names become faithful after library owns declarations. |
| Admin controls shell | `admin-controls`, `admin-controls-row`, `script-management-notice`, `script-notice-text` | `portal-admin-controls`, `admin-controls-row` | ABSORB | Absorb original `admin-controls` pack; replace Rust-only `portal-admin-controls` if it is not declaration-identical. |
| Admin service buttons | buttons inside `.admin-controls button` from `PortalCard.css` | unclassified buttons in generated row | ABSORB | Promote portal admin-button domain selectors or recompose to `ui-button` only if browser proof shows the shared button declaration is identical. |
| Visibility toggle | `visibility-toggle`, `fas fa-eye/-slash` in `portals/index.tsx` | `visibility-toggle` | RESTORE | Old code uses a shared visibility affordance class family already present in `styles/common/ui/_visibility-toggle.css`; recompose to `ui-visibility-toggle*` unless the pane-local selector proves unique. |
| Add portal card | `portal-card add-portal-card`, `add-portal-content`, `add-portal-icon`, `add-portal-title`, `add-portal-description` | no complete add-card found in current fragment | ABSORB | Absorb add-portal card pack and restore caboose card on that primitive. |
| Add portal modal | `portal-modal-overlay`, `portal-modal-content`, `add-portal-modal`, `modal-header`, `close-button`, `portal-form`, `form-group`, `form-actions`, `cancel-button`, `submit-button` | generic modal classes elsewhere; not full portal modal | ABSORB | Promote portal modal/form pack; form controls may internally RESTORE to `ui-input`, `ui-select`, and `ui-button` where declarations match shared UI primitives. |
| Service status modal | `service-status-modal`, `service-status-content`, `copy-button` | no full service modal found in portals route; generic status modal substrate in shell | ABSORB | Absorb service-status modal pack and restore copy control; browser drill must prove modal size/scroll/pre text. |

## Upload pane

OG citations: `src/tablets/upload/index.tsx`, `components/DirectoryBrowser.tsx`, `components/DirectoryBrowser.css`, `components/BlacklistManager.tsx`, `components/UploadProgress.tsx`, `upload.css`. Current citations: `src/bands/shell/document-2.rs`, `src/bands/shell/document-4.rs`, `src/bands/full-rust-routes/upload.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `upload-tablet` from `upload/index.tsx` / `upload.css` | `upload-tablet` | ABSORB | Promote upload pane pack; current name becomes faithful only after library owns declarations. |
| Upload controls row | `upload-controls`, `file-upload-section`, `.file-upload-section button` from `upload/index.tsx` / `upload.css` | `upload-controls`, `file-upload-section` | ABSORB | Absorb upload controls layout; RESTORE individual file input/buttons to shared primitives where the old component used them. |
| File picker | og tablet owns native input/button in `upload/index.tsx`; shared library has `FileInput.tsx` and `_file-input.css` | no `ui-file-input*`; current route has file section classes | RESTORE | Recompose file chooser onto `ui-file-input`, `ui-file-input__controls`, `ui-file-input__button`, and `ui-file-input__display`. |
| Upload progress item | `upload-progress ${status}`, `upload-header`, `status-icon`, `filename`, `remove-button`, `progress-section`, `progress-bar-container`, `progress-bar`, `progress-text`, `upload-stats`, `speed`, `error-message` from `UploadProgress.tsx` / `upload.css` | same classes in `document-4.rs` | RESTORE + ABSORB | `progress-bar*` has shared `ui-progress-bar*` bedrock in `_progress-bar.css` (RESTORE); upload-specific outer/status/error declarations are ABSORB into an upload-progress pack. |
| Directory browser root/header | `directory-browser`, `directory-browser-header`, `directory-tree-container`, `directory-empty`, `directory-loading-initial`, `directory-error nas-unavailable` from `DirectoryBrowser.tsx` / CSS | route emits `directory-entry`, `directory-error`, `directory-error nas-unavailable`; root/header partial | ABSORB | Absorb directory-browser domain pack; restore missing root/header wrappers in markup tranche. |
| Breadcrumbs | `breadcrumb-navigation`, `breadcrumb-item current`, `breadcrumb-separator` from `DirectoryBrowser.tsx`; shared library has `Breadcrumbs.tsx` / `_breadcrumbs.css` | `breadcrumb-item`, `breadcrumb-separator` | RESTORE | Recompose to `ui-breadcrumbs`, `ui-breadcrumbs__item`, `ui-breadcrumbs__item--current`, `ui-breadcrumbs__separator`. |
| Directory entries | `directory-entry selected/loading`, `tree-line vertical/horizontal`, `expand-control`, `entry-icon`, `entry-name`, `entry-selected` | same/partial classes | ABSORB | Promote directory tree/entry pack; current preserved classes become library vocabulary after absorption. |
| Directory admin buttons | `refresh-button`, `admin-button force-allow-button`, `admin-button set-default-button`, `admin-button blacklist-button`, `admin-button upload-history-button`, `toggle-pin-button active` | not all present; generic current buttons absent or approximated | RESTORE + ABSORB | Buttons should RESTORE onto `ui-button` variants if shared; special `toggle-pin-button` and admin path affordances ABSORB as upload domain selectors. |
| Blacklist manager | `blacklist-manager`, `blacklist-entries`, `blacklist-entry`, `entry-path`, `remove-entry`, `blacklist-controls`, `add-entry`, `entry-input`, `add-button`, `submit-button` from `BlacklistManager.tsx` / `upload.css` | `blacklist-entry`, `entry-path`, `remove-entry` partial | ABSORB | Absorb blacklist-manager pack; `entry-input` may later RESTORE to `ui-input` only if old declaration is intentionally replaced by shared input with identical visible result. |
| Upload history modal | `upload-history-modal-content`, `uploadHistoryModal empty`, `upload-history-empty-message`, `upload-history-list`, `history-item success/error`, `clear-history-button` | `history-item success/error` partial | ABSORB | Promote upload-history pack and restore clear/history modal around it. |
| PIN modal | `pin-modal-form`, `pin-input` | generic PIN modal in shell | ABSORB | Absorb upload PIN form class declarations or map to existing shared modal/input only with exact old-look proof. |

## Stats pane, including power fragment

OG citations: `src/tablets/stats/index.tsx`, `components/KeaLeasesTable.tsx`, `components/NetworkSpeedChart.tsx`, `components/MemoryRadialBar.tsx`, `components/ProcessUsageList.tsx`, `components/StatChart.tsx`, `components/CpuStatChart.tsx`, `components/DiskIoChart.tsx`, `components/DiskUsageChart.tsx`, `stats.css`, plus `styles/common/ui/_table.css`, `_progress-bar.css`, `_checkbox.css`, `_editable-field.css`, `_visibility-toggle.css`. Current citations: `src/bands/shell/document-2.rs`, `src/bands/shell/document-3.rs`, `src/bands/shell/document-4.rs`, `src/bands/full-rust-routes/power.rs`, `src/bands/crown-law/stats-tabbar.rs`.

| element | og class / source | current class | classification | action |
| --- | --- | --- | --- | --- |
| Pane root | `stats-tablet` from `stats/index.tsx` / `stats.css` | `stats-tablet` | ABSORB | Promote stats pane pack; preserve flex/gap/padding declarations. |
| Stat card/root | `stat-element`, `stat-header`, `stat-title`, `stat-content` | `stat-element`, `stat-header`, `stat-title`, `stat-content` | ABSORB | Absorb stats card pack; current emitted classes become faithful when library owns declarations. |
| Visibility toggle | `visibility-toggle`, eye icon in `stats/index.tsx` | `visibility-toggle`, `eye-icon` in `stats-tabbar.rs` and stats fragment | RESTORE | Recompose onto `ui-visibility-toggle*` from `_visibility-toggle.css`; preserve old eye/eye-slash placement. |
| Chart host | `stat-chart`, `cpu-stats-container`, `cpu-chart`, `network-stats-container`, `network-speed-chart`, `disk-io-chart` | `stat-chart` and fragments partial | ABSORB | Promote chart host pack; browser drill must prove chart canvas/SVG sizing and tooltip treatment. |
| Chart tooltip/grid | `recharts-default-tooltip`, `recharts-tooltip-label`, `recharts-tooltip-item`, `recharts-cartesian-grid line` | not emitted in Rust static proof | ABSORB | Absorb chart vendor override pack; later route/chart implementation must carry selectors only when actual rendered chart uses them. |
| Memory bars | `memory-stats`, `memory-current`, `memory-label`, `memory-bar`, `memory-bar-fill`, `memory-bar-fill-swap`, `memory-text`, `memory-details` | same classes in `document-4.rs` | RESTORE | `_progress-bar.css` already carries legacy `memory-bar*` declarations; recompose to `ui-progress-bar*` or mark legacy aliases faithful only if the library keeps them as canonical compatibility names. |
| Process bars | `process-usage-list`, `process-bar`, `process-bar-fill`, `process-text-container`, `process-name`, `process-usage`, loading/empty states | same classes in `document-4.rs` | RESTORE | `_progress-bar.css` carries legacy `process-bar*`; migrate to progress primitive or declare aliases as library-owned. |
| Disk usage bars | `disk-usage-stats`, `disk-usage-item`, `disk-usage-header`, `disk-device`, `disk-mountpoint`, `disk-usage-bar`, `disk-usage-fill`, `disk-usage-details` | same classes in `document-4.rs` | RESTORE | `_progress-bar.css` carries legacy `disk-usage-bar/fill`; recompose onto progress primitive while absorbing surrounding disk-usage pack. |
| Network interface table | `network-interfaces`, `network-interfaces-table`, `interface-name`, `interface-label`, `data-cell` | `interface-name`, `interface-label`, `data-cell` partial | RESTORE | `_table.css` includes `network-interfaces-table` compatibility declarations; restore table wrapper and use `ui-table`/owned legacy alias. |
| Kea leases table | `kea-leases-table`, `device-note-cell`, `note-text`, `edit-note-button`, `edit-note-modal`, `note-textarea` | `device-note-cell`, `note-text`, `edit-note-button` | RESTORE + ABSORB | `_table.css` owns `kea-leases-table`; `note-textarea` should RESTORE to `ui-text-box`/`ui-editable-field` if exact; note cell/action pack ABSORB where tab-local. |
| Disk I/O checkboxes | `device-control`, `device-name`, `device-checkboxes`; native checkboxes in `DiskIoChart.tsx` | `device-control`, `device-name`, `device-checkboxes`, `drive-checkbox` | RESTORE | Shared `_checkbox.css` names the stats tab style. Recompose native checkboxes to `ui-checkbox*`; absorb only device-control layout. |
| CPU load averages | `load-averages`, `load-average-values`, `load-average-item`, `load-label`, `load-value` | current fragment not complete | ABSORB | Promote CPU load pack from `stats.css`. |
| Power meter modal/fragment | no dedicated React power tab found in inspected stats quarry; power appears as current Coronatio header/modal classes: `power-indicator`, `power-meter-modal`, `power-usage-display`, `power-value*`, `power-history-section`, `power-average-*` | `document-2.rs`, `document-3.rs`, `full-rust-routes/power.rs` classes | ABSORB | Quarry-gap for React origin. Searched stats/admin/upload/portals and premium roots; map as Coronatio-local fragment needing browser drill and a proposed library `power-meter` domain pack only after old runtime/source citation is found. |
| Stats tabbar checkboxes/star | shared `Tab`, `TabGroup`, `VisibilityToggle`, checkbox primitives in `components/ui` and `_tabs.css` / `_visibility-toggle.css` | `tab-visibility-column`, `visibility-toggle`, `star-button {star_class} fa-star`, `tab {active_class}` | RESTORE | Recompose crown-law tabbar onto `ui-tab*`, `ui-tab__visibility-toggle`, `ui-tab__star-button`, `ui-checkbox*` where used. |

## Placeholder premium panes

These are not current port targets. They are included so eventual ports land directly on bedrock through the Adding a crown pane ladder.

| placeholder pane | og tab source | og library primitives / local packs | port map |
| --- | --- | --- | --- |
| backblaze | `/fulcrum/attachments/homeserver/initialization/flask/inject/premium/backblazeTab/frontend/index.tsx`, `backblazeTab.css`, and child panels under `frontend/` | imports `Button`, `Card`, `Collapsible`, `Input`, `Select`, `Calendar`, `TimePicker`, `Tab`, `TabGroup`, `RowInfoTile`, `Breadcrumbs`; local backblaze bucket/file/ledger/forgejo packs | Start from shared UI primitives for controls; ABSORB tab-local backblaze domain declarations into a library `backblaze` pack before Rust pane markup. |
| wake-on-lan | `/fulcrum/attachments/homeserver/initialization/flask/inject/premium/wakeonlan/frontend/index.tsx`, `PortalCard.css` | imports `Button`, `Checkbox`; local classes `wakeonlan-tablet`, `wakeonlan-section`, `wakeonlan-target-*`, `wakeonlan-lease-*`, `wakeonlan-message`, `wakeonlan-actions` | RESTORE `Button` and `Checkbox`; ABSORB wakeonlan list/message/action domain pack. |
| chia-mining | `/fulcrum/attachments/homeserver/initialization/flask/inject/premium/miner/frontend/index.tsx`, `styles/dashboard.css`, `styles/miners.css`, plus miner modal/card components | local classes `miner-tablet`, `fleet-header`, `fleet-stats`, `stat-box`, `fleet-actions`, `action-btn`, `error-banner`, `empty-state`, `miners-grid`; components `MinerCard`, `ClaimMinerModal`, `PoolConfigModal`, `FleetControls` | ABSORB miner dashboard/card/modal packs; where buttons/inputs are not shared, either migrate to `ui-button`/`ui-input` with exact look proof or absorb as miner pack. |
| dhcp | `/fulcrum/attachments/homeserver/initialization/flask/inject/premium/dhcp/frontend/index.tsx`, `PortalCard.css`, `components/DhcpCard.tsx`, `components/ReservationSlider.tsx` | local `dhcp-tablet`, `dhcp-info-banner`, `dhcp-button-row`, `anonymize-toggle-*`, `dhcp-list-*`, `mac-input`, `ip-input`, `add-reservation-button`; slider/card components | RESTORE shared toggle/slider/input/checkbox primitives where exact; ABSORB DHCP list/banner/card pack. |
| youtube | `/fulcrum/attachments/homeserver/initialization/flask/inject/premium/youtube/frontend/index.tsx`, `PortalCard.css`, `components/{DownloadForm,SubscriptionList,ScheduleConfig,DownloadSettings,LogsView}.tsx` | local `youtube-tablet`, `youtube-tabs`, `tab-button active`, `youtube-content`, `youtube-section(s)`, plus form/list/log component classes | RESTORE tab group/buttons/inputs/text boxes to shared UI; ABSORB YouTube section/log/subscription domain packs. |

## Proposed tranche order

1. Upload breadcrumbs + progress proof slice. Risk: low-to-medium. It has clear existing shared primitives (`Breadcrumbs`, `FileInput`, `ProgressBar`) and current Rust already emits many og upload classes. This establishes RESTORE and ABSORB method on a contained pane without broad admin modal overlap.
2. Stats bars/tables/checks slice. Risk: medium. `styles/common/ui/_progress-bar.css`, `_table.css`, and `_checkbox.css` already carry legacy aliases and explicit stats-tab style comments. Browser drill is required for chart geometry and responsive table behavior.
3. Portals card/add/modal slice. Risk: medium. Card and modal declarations are tab-local and visually distinctive; the card grid has limited overlap with admin/upload.
4. Admin controls/key/disk slice. Risk: high. It has the largest modal/action surface and service/disk state matrix. Split into SystemControls, KeyManager, DiskManager, then admin modals if needed.
5. Power fragment. Risk: unknown. Current Rust has power classes, but this audit did not find a React power pane source; treat as quarry-gap until a source/runtime citation is recovered.
6. Placeholder panes by Adding-a-crown-pane ladder: backblaze first only if it is the active business target; otherwise wake-on-lan is the smallest premium proof, DHCP has medium form/list complexity, miner has broader card/modal complexity, and youtube has multi-tab/config/log complexity.

Overlap risks:
- `action-button`, `submit-button`, `cancel-button`, and `modal-*` names recur across panes but are not automatically shared primitives. Each tranche must cite whether the declaration came from `styles/common/ui`, `styles/common`, or tab-local CSS before RESTORE/ABSORB.
- Some current Rust classes preserve og names, but this is not FAITHFUL until the declarations are library-owned. Preserved names with tab-local CSS are ABSORB.
- `card` is not a valid substitute for og `portal-card`, `disk-item`, or `stat-element` unless the absorbed/library class stack proves identical rendered geometry.

## Required wall shape for later tranches

Each implementation tranche must carry:

1. Source wall: exact og paths read, with line/selector citations for every changed element.
2. Library wall: each adopted class is asserted as one of:
   - existing shared `ui-*` primitive from `styles/common/ui`, or
   - absorbed domain primitive with byte-identical CSS declarations copied from the og tab-local source into the library band.
3. Markup wall: per-pane assertions that the Rust pane emits only library vocabulary for the changed surface, including forbidden old Rust helper classes where applicable (`admin-quarry-button`, `portal-admin-controls`, generic `card` in place of domain card, or missing `ui-*` roots).
4. Non-drift wall: exact markup/class assertions are necessary but not sufficient. The tranche names the old and new class stacks and proves no unclassified class remains.
5. Browser drill wall: only a browser can prove final visual identity for geometry, responsive layout, hover/focus, modal stacking, scroll behavior, chart/canvas/SVG sizing, file input rendering, checkbox/toggle feel, and history/log overflow. The browser drill must compare old quarry or captured old runtime against Coronatio at the same viewport/theme/session/data state.
6. Cargo wall: full Rust suite remains green; the expected baseline for this slice is `cargo test` green (98 tests on main today).

## Quarry gaps found

- No React `power` pane source was found in the inspected og roots. Searched the core tablets and premium roots named above. Current power fragment is Coronatio-local in `document-2.rs`, `document-3.rs`, and `full-rust-routes/power.rs`; it needs a recovered old source or explicit divergence before fidelity claims.
- Placeholder premium panes often import shared UI components and also carry tab-local `PortalCard.css` or domain CSS. Their eventual Rust ports should begin by classifying those imported primitives before writing pane markup.
