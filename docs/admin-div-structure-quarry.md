# Admin DIV Structure Quarry

This document records the literal static DIV hierarchy of the original Flask/React Admin tablet and the Coronatio crown reconciliation made by `ADMIN-DIV-QUARRY-001`. Behavior and privileged mutation wiring remain outside this structural tranche.

## Quarry authority

- `attachments/homeserver/initialization/flask/inject/src/tablets/admin/index.tsx`
- `attachments/homeserver/initialization/flask/inject/src/tablets/admin/components/SystemControls.tsx`
- `attachments/homeserver/initialization/flask/inject/src/tablets/admin/components/KeyManager.tsx`
- `attachments/homeserver/initialization/flask/inject/src/tablets/admin/components/DiskManager.tsx`
- `attachments/homeserver/initialization/flask/inject/src/tablets/admin/utils/diskUtils.ts`
- `attachments/homeserver/initialization/flask/inject/backend/monitors/disk.py`

Crown surface:

- `src/bands/shell/document-2.rs`
- `src/bands/crown-law/stats-tabbar.rs`

## Governing law

`pali:workflow-coronatio-admin-tab-parity-emerald-tablet` is the constitutional
authority for the native Admin pane. This quarry remains the literal structural
evidence beneath that tablet and does not supersede its parity walls.

## Side-by-side tree

| Original Flask/React | Crown before | Crown after |
|---|---|---|
| `.admin-tablet` | `.admin-tablet[data-admin-quarry]` | `.admin-tablet` |
| `section.mb-6 > .system-controls-container` | same wrapper plus quarry attributes and hidden inventory note | same wrapper without quarry scaffolding |
| `.system-controls > .system-controls-btn ×7` | seven controls carrying quarry index/source attributes | seven controls; existing HTMX attributes and minimal `data-admin-action-id` retained |
| `.system-service-controls` immediately after `.system-controls` | `.update-status-container` inserted before service controls | service controls immediately follow controls; an empty action result target follows service controls |
| `.ssh-controls > .ssh-control > .ssh-status > h3 + .ssh-toggle` twice | service placeholders inside `.ssh-control` | rendered service card preserves `.ssh-status` and `.ssh-toggle` nesting |
| `.samba-control > .samba-status > h3 + .samba-toggle` | service placeholder inside `.samba-control` | rendered service card preserves `.samba-status` and `.samba-toggle` nesting |
| `section.mb-6 > .key-manager` | key manager plus quarry group/note attributes | literal key manager wrapper tree |
| `.status-details > p` contains `.action-button.info-button` | information button missing | `View Full Guide & Critical Warnings` button restored inside the paragraph |
| `.key-manager-right > .key-actions > three buttons` | three buttons with quarry and stub attributes | three original button classes and labels without quarry/stub attributes |
| `section.mb-6 > .disk-manager` | two columns, no action row | two columns followed by `.disk-actions` |
| mount destination `.disk-list` maps all `MOUNT_DESTINATIONS` | emitted only mounted destinations, otherwise one idle placeholder | always emits NAS `/mnt/nas` and NAS Backup `/mnt/nas_backup` rows; mounted state enriches either row |
| `.disk-actions` contains Format, Encrypt, Assign primary, Assign backup, Unassign, Import, Setup NAS, Unlock, Mount, Unmount, Sync Now, Auto Sync | absent | all twelve buttons present in original source order; behavior deferred |
| modals open through the application modal portal | twelve static `.admin-modal-shelf > article.modal-window` inventory cards | no static modal shelf |

Note: the contract prose says “×13,” but its enumerated action identifiers contain twelve entries, and the literal `DiskManager.tsx` quarry emits twelve buttons. The crown follows the literal source and the twelve named actions rather than inventing a thirteenth control.

## Debt classification

| Class | Crown before | Reconciliation / boundary |
|---|---|---|
| EXTRA | Static `section.admin-modal-shelf`; `data-admin-quarry*`; `admin-quarry-note`; `data-stub-action`; Ready strip between primary and service controls | Removed from emitted admin shell. Existing action-result target is empty and follows service controls. |
| MISSING | Key guide information button; disk action row; idle mount destination rows | Restored the information button, twelve literal disk actions, and both mount destination rows. |
| MISORDERED | `.update-status-container` interrupted `.system-controls + .system-service-controls` | Service controls now immediately follow the seven-button control row. |
| FILTER-DEBT | Original backend `DiskMonitor._filter_block_devices` recursively removes devices mounted at ignored paths, while frontend `SYSTEM_CRITICAL_PATHS` defines critical mount exclusions. Crown `admin_block_devices_from_mounts` is mounted-only and cannot represent unmounted block devices. | This slice fixes only the destination list shape. Full device discovery and parity with `_filter_block_devices` plus `SYSTEM_CRITICAL_PATHS` remains explicit debt for the next behavior/backend tranche. |

## Structural acceptance

The admin pane wall requires the original class stack, `.disk-actions`, and `.action-button.info-button`; requires both standard mount paths; and rejects `admin-modal-shelf`, `data-admin-quarry`, `admin-quarry-note`, and `data-stub-action` from the emitted admin shell.
