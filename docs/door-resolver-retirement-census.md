# Door resolver retirement census

This is the exact per-call-site return for the completed resolver migration: [door-resolver-retirement-census.csv](door-resolver-retirement-census.csv). Every row is preserved as R001 through R137 with method, public path, resolver path, source, disposition, historical target, direct target, and a cautious candidate note.

## Census inputs

Three source/readback lanes underpin this census:
1. Coronatio base revision `db3511d8cf26e69917bcfc74a4d4a511bea81bcc`.
2. Caduceus current `origin/main` revision `f527658d54f3e251e8ae9d7223ea905cdbdaf2f1`.
3. Live `GET /api/v1/doors` readback with schema `caduceus.doors.readback.v1` and 109 advertised routes, cross-checked against historical typed-seat witness `ba8f440a893a407724f9bfbcee02e3f8873b60ae`.

## Exact totals

- 137 endpoint/method dispositions: R001–R137.
- 123 `translation-debt` rows.
- 2 `migrate-identity` rows.
- 9 `migrate-explicit-historical-target` rows.
- 11 direct routes total across the two direct dispositions.
- 3 `preserve-cartridge-existence-check` rows.
- Total: 123 + 2 + 9 + 3 = 137.

## Proven direct routes

| ID | Method | Public path | Direct target | Disposition |
|---|---|---|---|---|
| R006 | POST | `/admit/admin/action/view-logs-clear` | `/api/v1/log/clear` | `migrate-identity` |
| R105 | POST | `/api/network/device/claim` | `/api/v1/network/device/claim` | `migrate-explicit-historical-target` |
| R106 | GET | `/api/network/dhcp/boundary` | `/api/v1/network/dhcp/boundary` | `migrate-explicit-historical-target` |
| R107 | GET | `/api/network/dhcp/leases` | `/api/v1/network/dhcp/leases` | `migrate-explicit-historical-target` |
| R109 | GET | `/api/network/dns/read` | `/api/v1/network/dns/read` | `migrate-explicit-historical-target` |
| R111 | GET | `/api/dhcp/health` | `/api/v1/network/dhcp/health` | `migrate-explicit-historical-target` |
| R112 | GET | `/api/dhcp/leases` | `/api/v1/network/dhcp/leases` | `migrate-explicit-historical-target` |
| R113 | GET | `/api/dhcp/pool-boundary` | `/api/v1/network/dhcp/boundary` | `migrate-explicit-historical-target` |
| R119 | GET | `/api/dhcp/statistics` | `/api/v1/network/dhcp/statistics` | `migrate-explicit-historical-target` |
| R120 | GET | `/api/dhcp/status` | `/api/v1/network/dhcp/status` | `migrate-explicit-historical-target` |
| R131 | GET | `/api/v1/network/dns/read` | `/api/v1/network/dns/read` | `migrate-identity` |

## Preserved cartridge checks

These are not counted as direct migration routes. They preserve the existing cartridge existence check: the narrow resolver accepts the three flat public routes only after the live `caduceus.doors.readback.v1` advertisement is present.

| ID | Method | Route | Check |
|---|---|---|---|
| R135 | GET | `/api/v1/cartridges` | `resolve_cartridge_door` flat advertisement/existence check |
| R136 | POST | `/api/v1/cartridges/admit` | `resolve_cartridge_door` flat advertisement/existence check |
| R137 | POST | `/api/v1/cartridges/remove` | `resolve_cartridge_door` flat advertisement/existence check |

## Translation-debt families

These are debts, not selected routes. Candidate text is family-level only and is never a direct target. `none` means the current advertised list offers no plausible family for this call site.

### storage/disk — 9 rows

Candidate: `candidate-only: current storage/disk family (/api/v1/storage/disk/...) is plausible; no target selected`

| IDs and call sites |
|---|
| `R001` `POST` `/admit/admin/action/hard-drive-test` → `/api/admin/hard-drive-test/start`; historical `/api/admin/hard-drive-test/start` |
| `R011` `POST` `/api/admin/diskman/apply-permissions` → `/api/admin/diskman/apply-permissions` |
| `R012` `POST` `/api/admin/diskman/assign-nas` → `/api/admin/diskman/assign-nas` |
| `R013` `POST` `/api/admin/diskman/create-key` → `/api/admin/diskman/create-key` |
| `R014` `POST` `/api/admin/diskman/key-status` → `/api/admin/diskman/key-status` |
| `R015` `POST` `/api/admin/diskman/manage-services` → `/api/admin/diskman/manage-services` |
| `R016` `POST` `/api/admin/diskman/unlock-with-password` → `/api/admin/diskman/unlock-with-password` |
| `R017` `POST` `/api/admin/diskman/update-key` → `/api/admin/diskman/update-key` |
| `R018` `POST` `/api/admin/hard-drive-test/start` → `/api/admin/hard-drive-test/start`; historical `/api/admin/hard-drive-test/start` |

### storage/vault — 2 rows

Candidate: `candidate-only: current storage/vault family (/api/v1/storage/vault/...) is plausible; no target selected`

| IDs and call sites |
|---|
| `R079` `POST` `/api/pre-unlock` → `/api/pre-unlock` |
| `R094` `POST` `/api/vault/unlock` → `/api/vault/unlock` |

### update/interactables — 9 rows

Candidate: `candidate-only: current update/interactables families (/api/v1/update/... and /api/v1/interactables/...) are plausible; no target selected`

| IDs and call sites |
|---|
| `R025` `POST` `/api/admin/system/update-password` → `/api/admin/system/update-password` |
| `R026` `POST` `/api/admin/updates/apply` → `/api/admin/updates/apply` |
| `R027` `POST` `/api/admin/updates/force` → `/api/admin/updates/force` |
| `R028` `POST` `/api/admin/updates/interactives/:interactive_id/run` → `/api/admin/updates/interactives/:interactive_id/run` |
| `R029` `POST` `/api/admin/updates/modules/:module_name/branch` → `/api/admin/updates/modules/:module_name/branch` |
| `R030` `POST` `/api/admin/updates/modules/:module_name/components/:component_name/toggle` → `/api/admin/updates/modules/:module_name/components/:component_name/toggle` |
| `R031` `POST` `/api/admin/updates/modules/:module_name/toggle` → `/api/admin/updates/modules/:module_name/toggle` |
| `R032` `POST` `/api/admin/updates/schedule` → `/api/admin/updates/schedule` |
| `R092` `POST` `/api/system/update` → `/api/system/update` |

### settings/ssh — 4 rows

Candidate: `candidate-only: current settings/ssh family (/api/v1/settings/ssh) is plausible; no target selected`

| IDs and call sites |
|---|
| `R008` `POST` `/admit/admin/toggle/ssh-password-authentication` → `/api/admin/ssh/toggle`; historical `/api/admin/ssh/toggle` |
| `R009` `POST` `/admit/admin/toggle/ssh-service` → `/api/admin/ssh/service`; historical `/api/admin/ssh/service` |
| `R021` `POST` `/api/admin/ssh/service` → `/api/admin/ssh/service`; historical `/api/admin/ssh/service` |
| `R022` `POST` `/api/admin/ssh/toggle` → `/api/admin/ssh/toggle`; historical `/api/admin/ssh/toggle` |

### network/tailnet — 7 rows

Candidate: `candidate-only: current network/tailnet family (/api/v1/network/tailnet) is plausible; no target selected`

| IDs and call sites |
|---|
| `R080` `POST` `/api/status/tailscale/authkey` → `/api/status/tailscale/authkey` |
| `R081` `POST` `/api/status/tailscale/config` → `/api/status/tailscale/config` |
| `R082` `POST` `/api/status/tailscale/connect` → `/api/status/tailscale/connect` |
| `R083` `POST` `/api/status/tailscale/disable` → `/api/status/tailscale/disable` |
| `R084` `POST` `/api/status/tailscale/disconnect` → `/api/status/tailscale/disconnect` |
| `R085` `POST` `/api/status/tailscale/enable` → `/api/status/tailscale/enable` |
| `R086` `POST` `/api/status/tailscale/update-tailnet` → `/api/status/tailscale/update-tailnet` |

### network/vpn — 4 rows

Candidate: `candidate-only: current network/vpn family (/api/v1/network/vpn) is plausible; no target selected`

| IDs and call sites |
|---|
| `R087` `POST` `/api/status/vpn/disable` → `/api/status/vpn/disable` |
| `R088` `POST` `/api/status/vpn/enable` → `/api/status/vpn/enable` |
| `R089` `POST` `/api/status/vpn/updatekey/pia` → `/api/status/vpn/updatekey/pia` |
| `R090` `POST` `/api/status/vpn/updatekey/transmission` → `/api/status/vpn/updatekey/transmission` |

### portals/linker/upload — 9 rows

Candidate: `candidate-only: current portals family (/api/v1/portals/...) is plausible; no target selected`

| IDs and call sites |
|---|
| `R057` `POST` `/api/files/upload` → `/api/files/upload`; historical `/api/v1/file/ingress` |
| `R072` `DELETE` `/api/nasLinker/delete` → `/api/nasLinker/delete` |
| `R073` `POST` `/api/nasLinker/deploy` → `/api/nasLinker/deploy` |
| `R074` `POST` `/api/nasLinker/newdir` → `/api/nasLinker/newdir` |
| `R075` `POST` `/api/nasLinker/rename` → `/api/nasLinker/rename` |
| `R076` `POST` `/api/portals` → `/api/portals` |
| `R077` `DELETE` `/api/portals/:portal_name` → `/api/portals/:portal_name` |
| `R078` `PUT` `/api/portals/:portal_name` → `/api/portals/:portal_name` |
| `R134` `POST` `/api/upload/force-permissions` → `/api/upload/force-permissions`; historical `/api/v1/upload/force-permissions` |

### firewall — 6 rows

Candidate: `candidate-only: current network/device/whitelist and settings/child-device families are plausible; no target selected`

| IDs and call sites |
|---|
| `R121` `POST` `/api/firewall/children` → `/api/admin/firewall/list`; historical `/api/admin/firewall/list` |
| `R122` `POST` `/api/firewall/children` → `/api/admin/firewall/register`; historical `/api/admin/firewall/register` |
| `R123` `POST` `/api/firewall/children/:mac` → `/api/admin/firewall/unregister`; historical `/api/admin/firewall/unregister` |
| `R124` `POST` `/api/firewall/children/:mac/whitelist` → `/api/admin/firewall/whitelist-get`; historical `/api/admin/firewall/whitelist-get` |
| `R125` `POST` `/api/firewall/children/:mac/whitelist` → `/api/admin/firewall/whitelist-set`; historical `/api/admin/firewall/whitelist-set` |
| `R126` `POST` `/api/firewall/observed` → `/api/admin/firewall/observed`; historical `/api/admin/firewall/observed` |

### network/dns/status — 6 rows

Candidate: `candidate-only: current network/dns/status family (/api/v1/network/dns/status) is plausible; no target selected`

| IDs and call sites |
|---|
| `R127` `POST` `/api/v1/network/dns/adblock` → `/api/v1/network/dns/adblock`; historical `/api/v1/network/dns/adblock` |
| `R128` `POST` `/api/v1/network/dns/blocklist/update` → `/api/v1/network/dns/blocklist/update`; historical `/api/v1/network/dns/blocklist/update` |
| `R129` `POST` `/api/v1/network/dns/device-name/create` → `/api/v1/network/dns/device-name/create`; historical `/api/v1/network/dns/device-name/create` |
| `R130` `POST` `/api/v1/network/dns/device-name/remove` → `/api/v1/network/dns/device-name/remove`; historical `/api/v1/network/dns/device-name/remove` |
| `R132` `GET` `/api/v1/network/dns/resolver/status` → `/api/v1/network/dns/resolver/status`; historical `/api/v1/network/dns/resolver/status` |
| `R133` `POST` `/api/v1/network/dns/upstream` → `/api/v1/network/dns/upstream`; historical `/api/v1/network/dns/upstream` |

### network/dhcp — 8 rows

Candidate: `candidate-only: current network/dhcp family (/api/v1/network/dhcp/...) is plausible; no target selected`

| IDs and call sites |
|---|
| `R056` `POST` `/api/dhcp/config` → `/api/dhcp/config`; historical `/api/v1/network/dhcp` |
| `R108` `GET` `/api/network/dhcp/reservations` → `/api/network/dhcp/reservations`; historical `/api/v1/network/dhcp/reservations` |
| `R110` `GET` `/api/dhcp/config` → `/api/dhcp/config`; historical `/api/v1/network/dhcp` |
| `R114` `POST` `/api/dhcp/pool-boundary` → `/api/dhcp/pool-boundary`; historical `/api/v1/network/dhcp/pool-boundary` |
| `R115` `GET` `/api/dhcp/reservations` → `/api/dhcp/reservations`; historical `/api/v1/network/dhcp/reservations` |
| `R116` `POST` `/api/dhcp/reservations` → `/api/dhcp/reservations`; historical `/api/v1/network/dhcp/reservations` |
| `R117` `DELETE` `/api/dhcp/reservations/:reservation_id` → `/api/dhcp/reservations/:reservation_id`; historical `/api/v1/network/dhcp/reservations/:reservation_id` |
| `R118` `PUT` `/api/dhcp/reservations/:reservation_id` → `/api/dhcp/reservations/:reservation_id`; historical `/api/v1/network/dhcp/reservations/:reservation_id` |

### network/device — 1 rows

Candidate: `candidate-only: current network/device family (/api/v1/network/device/...) is plausible; no target selected`

| IDs and call sites |
|---|
| `R104` `GET` `/api/network/device` → `/api/network/device`; historical `/api/v1/network/device` |

### network/log — 1 rows

Candidate: `candidate-only: current log family (/api/v1/log/...) is plausible; no target selected`

| IDs and call sites |
|---|
| `R091` `POST` `/api/system/log` → `/api/system/log` |

### backup — 23 rows

Candidate: `none`

| IDs and call sites |
|---|
| `R033` `POST` `/api/backup/auto-update/check` → `/api/backup/auto-update/check` |
| `R034` `POST` `/api/backup/auto-update/toggle` → `/api/backup/auto-update/toggle` |
| `R035` `POST` `/api/backup/backup/run` → `/api/backup/backup/run` |
| `R036` `POST` `/api/backup/cleanup` → `/api/backup/cleanup` |
| `R037` `POST` `/api/backup/cloud/test` → `/api/backup/cloud/test` |
| `R038` `POST` `/api/backup/config` → `/api/backup/config` |
| `R039` `POST` `/api/backup/debug/toggle` → `/api/backup/debug/toggle` |
| `R040` `POST` `/api/backup/install` → `/api/backup/install` |
| `R041` `POST` `/api/backup/key` → `/api/backup/key` |
| `R042` `DELETE` `/api/backup/keyman/credentials/:service_name` → `/api/backup/keyman/credentials/:service_name` |
| `R043` `POST` `/api/backup/keyman/credentials/:service_name` → `/api/backup/keyman/credentials/:service_name` |
| `R044` `PUT` `/api/backup/keyman/credentials/:service_name` → `/api/backup/keyman/credentials/:service_name` |
| `R045` `POST` `/api/backup/providers/:provider_name/config` → `/api/backup/providers/:provider_name/config` |
| `R046` `POST` `/api/backup/providers/:provider_name/disable` → `/api/backup/providers/:provider_name/disable` |
| `R047` `POST` `/api/backup/providers/:provider_name/enable` → `/api/backup/providers/:provider_name/enable` |
| `R048` `POST` `/api/backup/providers/:provider_name/test` → `/api/backup/providers/:provider_name/test` |
| `R049` `POST` `/api/backup/restore` → `/api/backup/restore` |
| `R050` `POST` `/api/backup/schedule` → `/api/backup/schedule` |
| `R051` `POST` `/api/backup/schedule/config` → `/api/backup/schedule/config` |
| `R052` `POST` `/api/backup/schedule/test` → `/api/backup/schedule/test` |
| `R053` `POST` `/api/backup/sync-now` → `/api/backup/sync-now` |
| `R054` `POST` `/api/backup/test/cycle` → `/api/backup/test/cycle` |
| `R055` `POST` `/api/backup/uninstall` → `/api/backup/uninstall` |

### miner — 14 rows

Candidate: `none`

| IDs and call sites |
|---|
| `R058` `POST` `/api/miner/config` → `/api/miner/config` |
| `R059` `POST` `/api/miner/config/coin/:coin_id` → `/api/miner/config/coin/:coin_id` |
| `R060` `POST` `/api/miner/config/ssh-password` → `/api/miner/config/ssh-password` |
| `R061` `POST` `/api/miner/fleet/restart` → `/api/miner/fleet/restart` |
| `R062` `POST` `/api/miner/fleet/sync` → `/api/miner/fleet/sync` |
| `R063` `POST` `/api/miner/fleet/update-all` → `/api/miner/fleet/update-all` |
| `R064` `POST` `/api/miner/fleet/update-coins` → `/api/miner/fleet/update-coins` |
| `R065` `POST` `/api/miner/fleet/update-system` → `/api/miner/fleet/update-system` |
| `R066` `POST` `/api/miner/fleet/update-wallets` → `/api/miner/fleet/update-wallets` |
| `R067` `POST` `/api/miner/miners/:miner_id/claim` → `/api/miner/miners/:miner_id/claim` |
| `R068` `POST` `/api/miner/miners/:miner_id/coins/:coin_id/disable` → `/api/miner/miners/:miner_id/coins/:coin_id/disable` |
| `R069` `POST` `/api/miner/miners/:miner_id/coins/:coin_id/enable` → `/api/miner/miners/:miner_id/coins/:coin_id/enable` |
| `R070` `POST` `/api/miner/miners/:miner_id/restart` → `/api/miner/miners/:miner_id/restart` |
| `R071` `POST` `/api/miner/miners/:miner_id/unclaim` → `/api/miner/miners/:miner_id/unclaim` |

### test — 1 rows

Candidate: `none`

| IDs and call sites |
|---|
| `R093` `POST` `/api/test/analytics/process` → `/api/test/analytics/process` |

### backblaze — 9 rows

Candidate: `none`

| IDs and call sites |
|---|
| `R095` `POST` `/api/backblaze/buckets` → `/api/backblaze/buckets` |
| `R096` `DELETE` `/api/backblaze/buckets/:bucket` → `/api/backblaze/buckets/:bucket` |
| `R097` `DELETE` `/api/backblaze/buckets/:bucket/items` → `/api/backblaze/buckets/:bucket/items` |
| `R098` `POST` `/api/backblaze/buckets/:bucket/items` → `/api/backblaze/buckets/:bucket/items` |
| `R099` `POST` `/api/backblaze/buckets/:bucket/run` → `/api/backblaze/buckets/:bucket/run` |
| `R100` `POST` `/api/backblaze/buckets/:bucket/toggle` → `/api/backblaze/buckets/:bucket/toggle` |
| `R101` `POST` `/api/backblaze/buckets/:bucket/verify` → `/api/backblaze/buckets/:bucket/verify` |
| `R102` `POST` `/api/backblaze/config` → `/api/backblaze/config`; historical `/api/v1/backblaze/config` |
| `R103` `POST` `/api/backblaze/run` → `/api/backblaze/buckets/:bucket/run` |

### none advertised — 10 rows

Candidate: `none`

| IDs and call sites |
|---|
| `R002` `POST` `/admit/admin/action/restart` → `/api/admin/system/restart`; historical `/api/admin/system/restart` |
| `R003` `POST` `/admit/admin/action/restart-website` → `/api/admin/services/hard-reset`; historical `/api/admin/services/hard-reset` |
| `R004` `POST` `/admit/admin/action/rotate-capability-key` → `/usr/local/sbin/caduceus-keyman-rotate-capability` |
| `R005` `POST` `/admit/admin/action/shutdown` → `/api/admin/system/shutdown`; historical `/api/admin/system/shutdown` |
| `R007` `POST` `/admit/admin/toggle/samba-file-sharing` → `/api/admin/samba/service`; historical `/api/admin/samba/service` |
| `R010` `POST` `/api/admin/crypto/test` → `/api/admin/crypto/test` |
| `R019` `POST` `/api/admin/samba/service` → `/api/admin/samba/service`; historical `/api/admin/samba/service` |
| `R020` `POST` `/api/admin/services/hard-reset` → `/api/admin/services/hard-reset`; historical `/api/admin/services/hard-reset` |
| `R023` `POST` `/api/admin/system/restart` → `/api/admin/system/restart`; historical `/api/admin/system/restart` |
| `R024` `POST` `/api/admin/system/shutdown` → `/api/admin/system/shutdown`; historical `/api/admin/system/shutdown` |


## Backblaze literal-pattern wall

The literal `:bucket` patterns are pre-existing defects, not intentional shape matching. The historical typed seat contained no matching bucket rows except the config alias; the stale resolver therefore already returned unmapped. Every former crossing remains explicit translation debt. The exact bucket templates are `/api/backblaze/buckets/:bucket`, `/api/backblaze/buckets/:bucket/items`, `/api/backblaze/buckets/:bucket/run`, `/api/backblaze/buckets/:bucket/toggle`, and `/api/backblaze/buckets/:bucket/verify`; they must not be interpreted as authorization to materialize a bucket identifier into an upstream path.

## Operational artifact boundary

`/var/lib/coronatio/doors-cache.json` is now an unreferenced live operational artifact. Remove it by hand only after an admitted new binary is running. This contract does not delete it live.
