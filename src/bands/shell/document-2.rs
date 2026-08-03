fn shell_document_2() -> &'static str {
    r####"  </style>
  <link rel="stylesheet" href="/static/vendor/fontawesome/css/all.min.css" data-icon-substrate="fontawesome-free-5.15.4">
  <script src="/static/vendor/chart.umd.min.js" data-chart-dependency="chartjs-4.4.0"></script>
  <script src="/static/vendor/chartjs-plugin-datalabels.min.js" data-chart-dependency="chartjs-plugin-datalabels-2.2.0"></script>
</head>
<body>
  <main class="app" data-product="Coronatio" data-source-material="homeserver-main-site" data-admin-mode="false">
    <header class="top-bar header" data-flask-react-quarry="Header">
      <div class="header-top-row">
        <div class="header-left"><span class="uptime" data-uptime-indicator title="Server uptime">connecting...</span></div>
        <div class="header-center">
          <div class="status-indicators" aria-label="Status indicators" data-indicator-spine="coronatio.indicators.v1">__INDICATOR_SPINE__</div>
        </div>
        <div class="header-right">
          <button type="button" class="theme-button" data-theme-button data-admin-only="true" hidden title="Current theme: light. Click to switch theme."><span>light</span></button>
          <button type="button" class="change-admin-pin-button" data-change-pin-button data-admin-only="true" hidden>Change PIN</button>
          <button type="button" class="admin-button" data-admin-button data-admin-state="logged-out">Enter Admin Mode</button>
        </div>
      </div>
    </header>
    <div class="modal-backdrop" data-pin-modal-backdrop aria-hidden="true">
      <section class="modal" role="dialog" aria-modal="true" aria-labelledby="pin-modal-title">
        <h2 id="pin-modal-title">Enter Admin Mode</h2>
        <form class="pin-modal" data-pin-modal-form>
          <input type="text" autocomplete="username" value="admin" readonly style="display:none" aria-hidden="true">
          <input data-pin-current type="password" placeholder="Enter PIN" autocomplete="current-password">
          <input data-pin-change-current type="password" placeholder="Current PIN" autocomplete="current-password" hidden>
          <input data-pin-new type="password" placeholder="New PIN" autocomplete="new-password" hidden>
          <input data-pin-confirm type="password" placeholder="Confirm new PIN" autocomplete="new-password" hidden>
        </form>
        <div class="toast-slot" data-pin-modal-message></div>
        <div class="modal-actions"><button type="button" class="secondary" data-pin-cancel>Cancel</button><button type="button" data-pin-confirm-button>Confirm</button></div>
      </section>
    </div>
    <div class="modal-backdrop" data-info-modal-backdrop aria-hidden="true">
      <section class="modal" role="dialog" aria-modal="true" aria-labelledby="info-modal-title">
        <h2 id="info-modal-title">Status</h2>
        <div class="modal-body" data-info-modal-body></div>
        <div class="theme-choice-row" data-theme-choice-row hidden data-theme-json-source="/api/themes"></div>
        <div class="modal-actions"><button type="button" class="secondary" data-info-modal-close>Close</button></div>
      </section>
    </div>
    <nav class="tab-bar" aria-label="Coronatio primary tabs" role="tablist" data-admin-mode="false" data-hidden="false">__NAV__</nav>
    <section class="content" data-immortal-floor-shell data-startup-phase="Booting">
      <div class="immortal-floor-underlay" data-immortal-floor-layer="0" aria-hidden="true"></div>
      <aside class="immortal-floor-admission-frame" data-immortal-floor-layer="1" aria-live="polite">
        <div class="immortal-floor-underlay-card">
          <span class="immortal-floor-loader" aria-hidden="true"></span>
          <div><strong>HOMESERVER</strong><span data-immortal-floor-message>Preparing your controls…</span></div>
        </div>
      </aside>
      <div class="immortal-floor-guest-slot" data-immortal-floor-layer="2" data-slot-empty="true">
      <section class="pane" id="pane-admin" data-pane-panel="admin" data-view-panel="admin" role="tabpanel" aria-label="Admin">

        <div class="admin-tablet" data-admin-only="true" data-admin-viewport="admin">
          <section class="mb-6" style="margin-bottom: 0.5rem">
            <div class="system-controls-container" aria-label="System controls">
              <div class="system-controls" data-admin-action-strip="wrapped-row" data-admin-action-strip-count="7">
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="hard-drive-test" hx-post="/admit/admin/action/hard-drive-test" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">▣</span><span>Hard Drive Test</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="update" hx-post="/admit/admin/action/update" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">⬇</span><span>Update</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="restart" hx-post="/admit/admin/action/restart" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Restart HOMESERVER now? Active services will be interrupted."><span class="admin-action-icon">⟳</span><span>Restart</span></button>
                <button type="button" class="ui-button ui-button--danger ui-button--medium system-controls-btn" data-admin-action-id="shutdown" hx-post="/admit/admin/action/shutdown" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Shut down HOMESERVER now? Active services will stop."><span class="admin-action-icon">⏻</span><span>Shutdown</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="restart-website" hx-post="/admit/admin/action/restart-website" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Restart Website now? The web service will reset and this page will reconnect."><span class="admin-action-icon">↻</span><span>Restart Website</span></button>
                <button type="button" class="ui-button ui-button--secondary ui-button--medium system-controls-btn" data-admin-action-id="view-logs" hx-get="/admit/admin/action/view-logs" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">▤</span><span>View Logs</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-hestia-certificate-open><span class="admin-action-icon">◆</span><span>Install Certificate</span></button>
              </div>
              <section class="crown-diagnostics" data-crown-diagnostics aria-label="Crown Diagnostics">
                <div><strong>Crown Diagnostics</strong><span data-crown-diagnostics-state>Off</span></div>
                <p>Temporary, redacted troubleshooting signals. They turn off automatically.</p>
                <div data-crown-diagnostics-controls>
                  <button type="button" data-crown-diagnostic-toggle="immortal-floor">Floor &amp; crossings</button>
                  <button type="button" data-crown-diagnostic-toggle="requests">Requests &amp; resources</button>
                  <button type="button" data-crown-diagnostic-toggle="layout">Layout &amp; paint</button>
                  <button type="button" data-crown-diagnostic-toggle="sessions">Sessions &amp; access</button>
                  <button type="button" data-crown-diagnostic-toggle="streams">Streams</button>
                  <button type="button" data-crown-diagnostic-toggle="caduceus">Caduceus</button>
                </div>
                <small data-crown-diagnostics-unavailable>Sessions/access, streams, and Caduceus are control-ready; no additional safe browser source is claimed by this view.</small>
              </section>
              <div class="system-service-controls" data-admin-service-controls data-state-source="/api/services/data">
                <div class="ssh-controls">
                  <div class="ssh-control" data-service-card="ssh-password-authentication" data-state-field="ssh.password_auth_enabled" data-state-source="/api/services/data"><div class="ssh-status" data-admin-toggle-card="ssh-password-authentication"><h3>SSH Password Authentication</h3><div class="ssh-toggle"><label class="ui-toggle ui-toggle--medium toggle-switch"><span class="ui-toggle__switch"><input type="checkbox" class="ui-toggle__input" data-state-source="/api/services/data" hx-post="/admit/admin/toggle/ssh-password-authentication" hx-target="closest [data-service-card]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="ui-toggle__slider toggle-slider"></span></span></label><span class="toggle-label">Off</span><span class="ssh-icon disabled" data-packed-icon="lock" aria-hidden="true"><svg viewBox="0 0 24 24" focusable="false"><path d="M7 10V7a5 5 0 0 1 10 0v3h1a2 2 0 0 1 2 2v8H4v-8a2 2 0 0 1 2-2h1Zm2 0h6V7a3 3 0 0 0-6 0v3Z"/></svg></span></div></div></div>
                  <div class="ssh-control" data-service-card="ssh-service" data-state-field="sshd.running" data-state-source="/api/services/data"><div class="ssh-status" data-admin-toggle-card="ssh-service"><h3>SSH Service</h3><div class="ssh-toggle"><label class="ui-toggle ui-toggle--medium toggle-switch"><span class="ui-toggle__switch"><input type="checkbox" class="ui-toggle__input" data-state-source="/api/services/data" hx-post="/admit/admin/toggle/ssh-service" hx-target="closest [data-service-card]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="ui-toggle__slider toggle-slider"></span></span></label><span class="toggle-label">Stopped</span><span class="ssh-icon disabled" data-packed-icon="play" aria-hidden="true"><svg viewBox="0 0 24 24" focusable="false"><path d="M7 4v16l13-8L7 4Z"/></svg></span></div></div></div>
                </div>
                <div class="samba-control" data-service-card="samba-file-sharing" data-state-field="samba.running" data-state-source="/api/services/data"><div class="samba-status" data-admin-toggle-card="samba-file-sharing"><h3>Samba File Sharing</h3><div class="samba-toggle"><label class="ui-toggle ui-toggle--medium toggle-switch"><span class="ui-toggle__switch"><input type="checkbox" class="ui-toggle__input" data-state-source="/api/services/data" hx-post="/admit/admin/toggle/samba-file-sharing" hx-target="closest [data-service-card]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="ui-toggle__slider toggle-slider"></span></span></label><span class="toggle-label">Stopped</span><span class="samba-icon disabled" data-packed-icon="share-nodes" aria-hidden="true"><svg viewBox="0 0 24 24" focusable="false"><path d="M18 16a3 3 0 0 0-2.4 1.2l-6.7-3.4a3 3 0 0 0 0-3.6l6.7-3.4A3 3 0 1 0 15 5l-6.7 3.4a3 3 0 1 0 0 7.2L15 19a3 3 0 1 0 3-3Z"/></svg></span></div></div></div>
              </div>
              <div class="update-status-container" data-admin-action-result data-og-affordance="toast-mapped-to-result-strip" aria-live="polite"></div>
            </div>
          </section>

          <section class="mb-6" style="margin-bottom: 0.5rem">
            <div class="key-manager">
              <h3><span class="admin-action-icon">⚿</span> Key Management</h3>
              <div class="key-manager-content">
                <div class="key-manager-left">
                  <div class="security-status">
                    <div class="status-item"><span class="status-icon secure">🛡</span><div class="status-details"><p>This is the key to your vault. When you boot your HOMESERVER and visit home.arpa, this is what unlocks your encrypted storage system - just like unlocking your smartphone. Your /vault partition contains the sensitive keys stored on the device. Unlock the vault and everything HOMESERVER specifically stores is accessible. This is the device's master key.<button type="button" class="action-button info-button" data-manager-open="key-guide" aria-label="View Full Guide &amp; Critical Warnings"><span>ⓘ</span> View Full Guide &amp; Critical Warnings</button></p></div></div>
                  </div>
                </div>
                <div class="key-manager-right"><div class="key-actions">
                  <button type="button" class="action-button create-button" data-manager-open="create-key"><span>+ Create New Key</span></button>
                  <button type="button" class="action-button update-button" data-manager-open="update-key"><span>⟳ Update Key on Drive</span></button>
                  <button type="button" class="action-button admin-password-button" data-manager-open="admin-password"><span>🔒 Admin Password</span></button>
                </div></div>
              </div>
            </div>
          </section>

          <section class="mb-6" style="margin-bottom: 0.5rem">
            <div class="disk-manager">
              <div class="disk-manager-container">
                <div class="disk-column"><h4>Available Devices</h4><div class="disk-list" data-admin-devices-readback="/api/services/data">__ADMIN_AVAILABLE_DEVICES__</div></div>
                <div class="disk-column"><h4>Mount Destinations</h4><div class="disk-list" data-admin-mounts-readback="/api/services/data">__ADMIN_MOUNT_DESTINATIONS__</div></div>
              </div>
              <div class="disk-actions" data-disk-actions-state="no-selection" aria-live="polite">
                <p class="manager-action-reading" data-disk-action-reading>Select a device or mount destination to inspect its available actions.</p>
                <button type="button" class="action-button format" data-disk-action="format" disabled title="Select an eligible device first">Format</button>
                <button type="button" class="action-button encrypt" data-disk-action="encrypt" disabled title="Select an eligible device first">Encrypt</button>
                <button type="button" class="action-button assign-primary" data-disk-action="assign-primary" disabled title="Select an eligible device first">Assign as primary NAS</button>
                <button type="button" class="action-button assign-backup" data-disk-action="assign-backup" disabled title="Select an eligible device first">Assign as NAS Backup</button>
                <button type="button" class="action-button unassign-nas" data-disk-action="unassign" disabled title="Select an eligible device first">Unassign drive</button>
                <button type="button" class="action-button import-nas" data-disk-action="import" disabled title="Select an eligible device first">Import to NAS</button>
                <button type="button" class="action-button permissions" data-disk-action="setup-nas" disabled title="Select an eligible device first">Setup NAS</button>
                <button type="button" class="action-button unlock" data-disk-action="unlock" disabled title="Select an eligible device first">Unlock</button>
                <button type="button" class="action-button mount" data-disk-action="mount" disabled title="Select an eligible device first">Mount</button>
                <button type="button" class="action-button unmount" data-disk-action="unmount" disabled title="Select an eligible device first">Unmount</button>
                <button type="button" class="action-button sync" data-disk-action="sync" disabled title="Select an eligible device first">Sync Now</button>
                <button type="button" class="action-button auto-sync" data-disk-action="auto-sync" disabled title="Select an eligible device first">Auto Sync</button>
              </div>
            </div>
          </section>
        </div>
      </section>
      <section class="pane active" id="pane-stats" data-pane-panel="stats" data-view-panel="stats" role="tabpanel" aria-label="Stats">
        <div class="stats-tablet" data-stats-viewport data-react-quarry="StatsTablet" data-identity-standard="one-to-one">
          __STATS_ELEMENTS_FRAGMENT__
        </div>
      </section>
      <section class="pane" id="pane-portals" data-pane-panel="portals" data-view-panel="portals" role="tabpanel" aria-label="Portals">
        <div class="portals-tablet" data-portals-viewport data-react-quarry="PortalsTablet" data-identity-standard="one-to-one">
          <div class="portals-grid" data-portals-grid data-portals-source="/api/portals/elements" data-portals-fragment="/api/portals/elements">
            <article class="portal-card portal-loading" data-portals-loading><div><h2>Admitted services</h2><p>Reading homeserver.json portal entries.</p></div></article>
          </div>
        </div>
      </section>
      <section class="pane" id="pane-upload" data-pane-panel="upload" data-view-panel="upload" role="tabpanel" aria-label="Upload">
        <div class="upload-tablet" data-upload-viewport data-react-quarry="UploadTablet" data-identity-standard="one-to-one">
          <div class="upload-progress-list" data-upload-progress-list hidden></div>
          <div class="upload-controls">
            <div class="directory-browser" data-upload-regular="directory-browser" data-directory-browser>
              <div class="directory-browser-header">
                <button type="button" class="refresh-button" data-upload-refresh title="Refresh Directory Tree">🔄</button>
                <button type="button" class="admin-button force-allow-button" data-admin-only data-admin-viewport="upload" data-upload-force-allow title="Force Allow Upload (Admin)">🛡️ Allow</button>
                <button type="button" class="admin-button set-default-button" data-admin-only data-admin-viewport="upload" data-upload-set-default title="Set as Default Directory (Admin)">📌 Default</button>
                <button type="button" class="admin-button blacklist-button" data-admin-only data-admin-viewport="upload" data-upload-blacklist title="Manage Blacklist (Admin)">🚫 Blacklist</button>
                <button type="button" class="admin-button upload-history-button" data-admin-only data-admin-viewport="upload" data-upload-history title="View Upload History (Admin)">📜 History</button>
                <button type="button" class="toggle-pin-button" data-admin-only data-admin-viewport="upload" data-upload-pin-toggle title="Enable PIN requirement for uploads (Currently Off)" aria-label="Toggle PIN requirement (currently disabled)"><span class="loading-spinner" data-upload-pin-spinner hidden role="progressbar" aria-label="Saving PIN requirement"></span></button>
              </div>
              <div class="directory-breadcrumb-container">
                <div class="breadcrumb-navigation" data-upload-breadcrumbs><button type="button" class="breadcrumb-item current" data-upload-breadcrumb-path="/mnt/nas" aria-current="page">nas</button></div>
              </div>
              <div class="directory-loading-initial" data-upload-directory-loading hidden><span class="loading-spinner large" role="progressbar" aria-label="Loading directory tree"></span></div>
              <div class="directory-tree-container" data-upload-tree role="tree">__UPLOAD_TREE_FRAGMENT__</div>
            </div>
            <div class="file-upload-section" data-upload-regular="file-ingress" data-upload-file-section>
              <input type="file" multiple data-upload-file aria-label="Upload files">
              <button type="button" data-upload-submit disabled>Upload Selected Files</button>
            </div>
          </div>
          <div class="modal-overlay" data-upload-history-backdrop data-upload-history-modal aria-hidden="true" hidden>
            <section class="modal" role="dialog" aria-modal="true" aria-labelledby="upload-history-title">
              <button type="button" class="modal-close" data-upload-modal-close aria-label="Close modal">×</button><h2 class="modal-title" id="upload-history-title">Upload History</h2>
              <div class="modal-content upload-history-modal-content"><div class="uploadHistoryModal empty"><div class="upload-history-empty-message">No upload history available</div></div><div class="upload-history-list" hidden></div><button type="button" class="clear-history-button" data-upload-clear-history disabled>Clear History</button></div>
            </section>
          </div>
          <div class="modal-overlay" data-upload-blacklist-backdrop data-upload-blacklist-modal aria-hidden="true" hidden>
            <section class="modal" role="dialog" aria-modal="true" aria-labelledby="upload-blacklist-title">
              <button type="button" class="modal-close" data-upload-modal-close aria-label="Close modal">×</button><h2 class="modal-title" id="upload-blacklist-title">Manage Blacklist</h2>
              <div class="modal-content"><div class="blacklist-manager"><div class="blacklist-entries" data-upload-blacklist-entries></div><div class="blacklist-controls"><form class="add-entry" data-upload-blacklist-form><input type="text" class="entry-input" placeholder="Enter path to blacklist" data-upload-blacklist-input><button type="button" class="add-button" data-upload-blacklist-add>New</button></form><button type="button" class="submit-button" data-upload-blacklist-submit>Submit</button></div></div></div>
            </section>
          </div>
          <div class="modal-overlay" data-upload-pin-backdrop data-upload-pin-modal aria-hidden="true" hidden>
            <section class="modal" role="dialog" aria-modal="true" aria-labelledby="upload-pin-title">
              <button type="button" class="modal-close" data-upload-pin-cancel aria-label="Close modal">×</button><h2 class="modal-title" id="upload-pin-title">Admin PIN Required</h2>
              <div class="modal-content"><form class="pin-modal-form" data-upload-pin-form><p>Please enter the admin PIN to proceed with the upload.</p><input type="text" name="username" autocomplete="username" style="position:absolute;left:-9999px;opacity:0" tabindex="-1" aria-hidden="true"><input type="password" placeholder="Admin PIN" aria-label="Admin PIN" class="pin-input" autocomplete="new-password" data-upload-pin-input><p data-upload-pin-message role="alert" aria-live="polite"></p></form></div>
              <div class="modal-buttons"><button type="button" class="secondary" data-upload-pin-cancel>Cancel</button><button type="button" data-upload-pin-confirm>Confirm</button></div>
            </section>
          </div>
          <pre class="readout admin-quarry-note" id="upload-readout" data-upload-receipt-readout>Upload Caduceus receipts are diagnostic evidence, not the visible progress UI.</pre>
        </div>
      </section>
      <section class="pane" id="pane-backblaze" data-pane-panel="backblaze" data-view-panel="backblaze" role="tabpanel" aria-label="backBlaze">
        <article class="card og-stub-pane" data-og-stub-pane="backblaze"><h2>backBlaze</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-wake-on-lan" data-pane-panel="wake-on-lan" data-view-panel="wake-on-lan" role="tabpanel" aria-label="Wake on LAN">
        <article class="card og-stub-pane" data-og-stub-pane="wake-on-lan"><h2>Wake on LAN</h2><p>not yet ported</p></article>
      </section>
      __TEST__
      <section class="pane" id="pane-dhcp" data-pane-panel="dhcp" data-view-panel="dhcp" role="tabpanel" aria-label="DHCP">
        <div class="dhcp-tablet" data-dhcp-tablet data-admin-only="true" data-admin-viewport="dhcp">
          <div class="dhcp-info-banner" data-dhcp-info-banner aria-live="polite">
            <span class="dhcp-info-item">Homeserver: <span class="dhcp-info-value" data-dhcp-homeserver>192.168.123.1</span></span><span class="dhcp-info-separator" aria-hidden="true">|</span>
            <span class="dhcp-info-item">Reservations: <span class="dhcp-info-value" data-dhcp-reservations-count>—</span></span><span class="dhcp-info-separator" aria-hidden="true">|</span>
            <span class="dhcp-info-item">Hosts: <span class="dhcp-info-value" data-dhcp-hosts-count>—</span></span><span class="dhcp-info-separator" aria-hidden="true">|</span>
            <span class="dhcp-info-item">Leases: <span class="dhcp-info-value" data-dhcp-leases-count>—</span></span>
          </div>
          <div class="dhcp-button-row">
            <button type="button" class="ui-button ui-button--secondary ui-button--small" data-dhcp-refresh>Refresh</button>
            <label class="ui-toggle ui-toggle--medium"><span class="ui-toggle__switch"><input type="checkbox" class="ui-toggle__input" data-dhcp-anonymize><span class="ui-toggle__slider"></span></span><span class="ui-toggle__label">Anonymize</span></label>
            <div class="reservation-slider" data-dhcp-boundary-control>
              <label for="dhcp-reservation-boundary">Reserved addresses: <strong class="dhcp-boundary-value" data-dhcp-boundary-value>—</strong> <span class="dhcp-boundary-value" data-dhcp-boundary-min>—</span>–<span class="dhcp-boundary-value" data-dhcp-boundary-max>—</span></label>
              <input id="dhcp-reservation-boundary" class="ui-slider" type="range" min="0" max="249" value="0" data-dhcp-boundary>
              <button type="button" class="ui-button ui-button--primary ui-button--small" data-dhcp-boundary-save>Apply</button>
            </div>
          </div>
          <div class="dhcp-tablet-content">
            <div class="dhcp-banner dhcp-banner--error" data-dhcp-error role="alert" hidden></div>
            <div class="dhcp-banner dhcp-banner--loading" data-dhcp-loading aria-live="polite"><span class="loading-spinner medium" role="progressbar" aria-label="Loading DHCP devices"></span><span>Loading DHCP devices…</span></div>
            <div class="dhcp-list" data-dhcp-list>
              <form class="dhcp-list-item pinned add-reservation-row" data-dhcp-add-form>
                <div class="dhcp-list-item-content"><div class="dhcp-list-item-main"><div class="dhcp-list-item-info">
                  <label class="dhcp-list-item-mac"><span class="info-label">MAC Address:</span><input type="text" name="mac" class="ui-input ui-input--medium" placeholder="aa:bb:cc:dd:ee:ff" required></label>
                  <label class="dhcp-list-item-ip"><span class="info-label">IP Address:</span><input type="text" name="ip" class="ui-input ui-input--medium" placeholder="Automatic from reserved range"></label>
                  <label class="dhcp-list-item-hostname"><span class="info-label">Name:</span><input type="text" name="hostname" class="ui-input ui-input--medium" placeholder="Device name"></label>
                </div><div class="dhcp-list-item-badge"><span class="ui-badge ui-badge--success ui-badge--small">New</span></div></div><div class="dhcp-list-item-actions"><button type="submit" class="ui-button ui-button--success ui-button--small">Add Reservation</button></div></div>
              </form>
              <div class="dhcp-empty-state" data-dhcp-empty hidden><p>No DHCP leases or reservations found.</p></div>
              <div data-dhcp-items></div>
            </div>
          </div>
        </div>
        <div class="ui-modal-backdrop" data-dhcp-modal-backdrop aria-hidden="true">
          <section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="dhcp-modal-title">
            <h2 id="dhcp-modal-title" data-dhcp-modal-title>DHCP</h2>
            <div data-dhcp-modal-body></div>
            <div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-dhcp-modal-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-dhcp-modal-confirm>Confirm</button></div>
          </section>
        </div>
      </section>
      <section class="pane" id="pane-firewall" data-pane-panel="firewall" data-view-panel="firewall" role="tabpanel" aria-label="Firewall">
        <div class="firewall-tablet" data-firewall-tablet data-admin-only="true" data-admin-viewport="firewall">
          <header class="firewall-header"><div><h2>Firewall</h2><p>DNS website policy for one child device.</p></div><span class="ui-badge ui-badge--small ui-badge--info" data-firewall-state>Loading</span></header>
          <p class="firewall-disclosure">This is DNS website policy, not complete website enforcement: DNS cannot prove HTTPS hostnames, DoH can bypass it, and CDN or shared-IP behavior can affect results.</p>
          <div class="firewall-controls" data-firewall-editor><label>Child device <select class="ui-select" data-firewall-device aria-label="Child device by MAC"></select></label><label class="ui-toggle ui-toggle--medium"><span class="ui-toggle__switch"><input class="ui-toggle__input" type="checkbox" data-firewall-enabled><span class="ui-toggle__slider"></span></span><span class="ui-toggle__label">Enable policy</span></label><button class="ui-button ui-button--secondary ui-button--small" type="button" data-firewall-refresh>Refresh</button><button class="ui-button ui-button--primary ui-button--small" type="button" data-firewall-save>Save</button><button class="ui-button ui-button--danger ui-button--small" type="button" data-firewall-delete>Remove policy</button></div>
          <p class="firewall-binding" data-firewall-binding aria-live="polite">Checking stable binding…</p>
          <div class="firewall-banner firewall-banner--error" data-firewall-error role="alert" hidden></div>
          <form class="firewall-add-site" data-firewall-add-site-form><label for="firewall-site">Allowed website hostname</label><input id="firewall-site" class="ui-input ui-input--medium" data-firewall-site-input maxlength="253" placeholder="example.com" required><button class="ui-button ui-button--secondary ui-button--small" type="submit">Add website</button></form>
          <ul class="firewall-sites" data-firewall-sites></ul>
        </div>
      </section>
      __UNBOUND_PANE__
      </div>
    </section>
  </main>
  <!-- Body-level overlays preserve fixed viewport anchoring outside the transformed Immortal Floor. -->
  <div class="portal-modal-overlay" data-add-portal-modal hidden>
    <div class="portal-modal-content">
      <div class="add-portal-modal">
        <div class="modal-header"><h2>Add New Portal</h2><button type="button" class="close-button" data-portal-modal-close aria-label="Close modal"><i class="fas fa-times"></i></button></div>
        <form class="portal-form" data-portal-add-form>
          <div class="form-group"><label for="portal-name">Portal Name *</label><input id="portal-name" name="name" type="text" placeholder="e.g., MyApp" required><span class="error-text" data-portal-error-for="name" hidden></span></div>
          <div class="form-group"><label for="portal-description">Description *</label><input id="portal-description" name="description" type="text" placeholder="e.g., My custom application" required><span class="error-text" data-portal-error-for="description" hidden></span></div>
          <div class="form-group"><label for="portal-type">Service Type</label><select id="portal-type" name="type"><option value="systemd">Systemd Service</option><option value="script">Script-managed Service</option><option value="link">Link Only</option></select><small class="help-text">Systemd services can be controlled directly. Script-managed services require system restart. Link-only portals are simple links without service management.</small></div>
          <div class="form-group" data-portal-service-fields><label for="portal-services">Services *</label><input id="portal-services" name="services" type="text" placeholder="e.g., myapp, myapp-worker (comma-separated)"><span class="error-text" data-portal-error-for="services" hidden></span><small class="help-text">Enter service names separated by commas</small></div>
          <div class="form-group" data-portal-service-fields><label for="portal-port">Port *</label><input id="portal-port" name="port" type="number" min="1" max="65535" placeholder="e.g., 8080"><span class="error-text" data-portal-error-for="port" hidden></span></div>
          <div class="form-group"><label for="portal-local-url">Local URL *</label><input id="portal-local-url" name="localURL" type="url" placeholder="e.g., https://myapp.home.arpa" required><span class="error-text" data-portal-error-for="localURL" hidden></span></div>
          <div class="form-actions"><button type="button" class="cancel-button" data-portal-modal-close>Cancel</button><button type="submit" class="submit-button"><i class="fas fa-plus"></i> Create Portal</button></div>
        </form>
      </div>
    </div>
  </div>
  <div class="portal-modal-overlay" data-service-status-modal hidden>
    <div class="portal-modal-content">
      <div class="service-status-modal"><pre class="service-status-content" data-service-status-content></pre><button type="button" class="copy-button" data-service-status-copy>Copy to Clipboard</button></div>
    </div>
  </div>
  <div class="toast-container coronatio-toast-stack" data-coronatio-toast-stack aria-live="polite" aria-atomic="false"></div>
  <script>
    var coronatioAttendanceRuntimeKey = Symbol.for('coronatio.attendance.runtime.v1');
    var coronatioAttendanceRuntime = globalThis[coronatioAttendanceRuntimeKey];
    if (!coronatioAttendanceRuntime) {
      const nativeFetch = window.fetch.bind(window);
      coronatioAttendanceRuntime = {
        documentIncarnation: (globalThis.crypto?.randomUUID?.() || `${Date.now()}-${Math.random()}`).replace(/[^a-zA-Z0-9._-]/g, ''),
        inactivityHeadless: false,
        currentAttendance: null,
        lastEligibleActivity: Date.now(),
        lastAttendanceTouch: 0,
        fetchDecorationCount: 1,
        htmxHandlerInstallCount: 0,
        activityCensusInstallCount: 0,
      };
      const decoratedFetch = (input, init = {}) => {
        const headers = new Headers(init.headers || {});
        headers.set('X-Caduceus-Document', coronatioAttendanceRuntime.documentIncarnation);
        if (coronatioAttendanceRuntime.currentAttendance) headers.set('X-Caduceus-Attendance', coronatioAttendanceRuntime.currentAttendance);
        return nativeFetch(input, { ...init, headers, credentials: 'same-origin' });
      };
      Object.defineProperty(decoratedFetch, '__coronatioAttendanceDecoratorDepth', { value: 1 });
      coronatioAttendanceRuntime.decoratedFetch = decoratedFetch;
      Object.defineProperty(globalThis, coronatioAttendanceRuntimeKey, { value: coronatioAttendanceRuntime, configurable: false });
      window.fetch = decoratedFetch;
    }
    if (!coronatioAttendanceRuntime.htmxConfigRequestHandler) {
      coronatioAttendanceRuntime.htmxConfigRequestHandler = event => {
        event.detail.headers['X-Caduceus-Document'] = coronatioAttendanceRuntime.documentIncarnation;
        if (coronatioAttendanceRuntime.currentAttendance) event.detail.headers['X-Caduceus-Attendance'] = coronatioAttendanceRuntime.currentAttendance;
      };
      document.addEventListener('htmx:configRequest', coronatioAttendanceRuntime.htmxConfigRequestHandler);
      coronatioAttendanceRuntime.htmxHandlerInstallCount++;
    }
    const appRoot = document.querySelector('[data-product="Coronatio"]');
    const tabBar = document.querySelector('[role="tablist"]');
    let tabs = [...document.querySelectorAll('[data-pane]')];
    let panes = [...document.querySelectorAll('[data-pane-panel]')];
    const immortalFloorShell = document.querySelector('[data-immortal-floor-shell]');
    const immortalFloorGuestSlot = document.querySelector('[data-immortal-floor-layer="2"]');
    panes.forEach(pane => pane.dataset.immortalFloorLayer = '2');
    const htmxOrgan = window.htmx;
    if (htmxOrgan && htmxOrgan.config) {
      htmxOrgan.config.allowScriptTags = false;
      htmxOrgan.config.selfRequestsOnly = true;
      htmxOrgan.config.includeIndicatorStyles = false;
    }
    function panelFromHtmxEvent(event) {
      const detail = event.detail || {};
      const candidate = detail.target || detail.elt || event.target;
      return candidate instanceof Element ? candidate.closest('[data-view-panel]') : null;
    }
    function faultKindFromResponse(event, fallback) {
      const text = event.detail?.xhr?.responseText || '';
      if (!text.includes('data-cartridge-fault-kind')) return fallback;
      const template = document.createElement('template');
      template.innerHTML = text;
      const fault = template.content.querySelector('[data-cartridge-fault-kind]');
      return fault instanceof HTMLElement && fault.dataset.cartridgeFaultKind ? fault.dataset.cartridgeFaultKind : fallback;
    }
    function presentCartridgeFault(kind, event) {
      const panel = panelFromHtmxEvent(event);
      const panelId = panel instanceof HTMLElement ? (panel.dataset.viewPanel || panel.dataset.panePanel || '') : '';
      if (panel instanceof HTMLElement) {
        panel.innerHTML = `<section class="card error-message" data-cartridge-fault-presentation="og-pane"><h2>Cartridge fault</h2><p>${kind}</p></section>`;
        panel.dataset.viewportFaulted = 'true';
      }
      document.documentElement.dataset.cartridgeFaultReceipt = 'typed';
      document.documentElement.dataset.cartridgeFaultLast = kind;
      window.immortalFloor?.faultForPanel(panelId, kind);
    }
    document.body.addEventListener('htmx:timeout', event => presentCartridgeFault('timeout', event));
    document.body.addEventListener('htmx:responseError', event => presentCartridgeFault(faultKindFromResponse(event, 'upstream-error'), event));
    document.body.addEventListener('htmx:sendError', event => presentCartridgeFault('proxy-unreachable', event));
    let statsHydrationInFlight = false;
    document.body.addEventListener('htmx:afterSwap', event => {
      const panel = panelFromHtmxEvent(event);
      if (!(panel instanceof HTMLElement)) return;
      panel.dataset.viewportFaulted = 'false';
      const id = panel.dataset.viewPanel || panel.dataset.panePanel || '';
      if (id === currentActiveTabId() && window.getImmortalFloorState?.() === 'Seated' && id !== 'stats' && id !== 'portals') reconcileViewportStreamFamily();
    });
    const fallbackTab = 'admin';
    const storageKey = 'coronatio.flask-react-tabbar.v1';
    const loadTabState = () => {
      try { return JSON.parse(localStorage.getItem(storageKey) || '{}'); }
      catch (_) { return {}; }
    };
    const saveTabState = state => localStorage.setItem(storageKey, JSON.stringify(state));
    const tabState = Object.assign({ starredTab: 'stats' }, loadTabState());
    const headerStateKey = 'coronatio.flask-react-header.v1';
    const preferredThemeKey = 'preferred-theme';
    const themeDataKey = 'themeData';
    let themeCatalog = { default: 'light', themes: {} };
    let themes = [];
    const savedHeaderState = (() => { try { return JSON.parse(localStorage.getItem(headerStateKey) || '{}'); } catch (_) { return {}; } })();
    const savedPreferredTheme = localStorage.getItem(preferredThemeKey);
    const headerState = { theme: savedPreferredTheme || savedHeaderState.theme || 'light', isAdmin: false };
    const saveHeaderState = () => {
      localStorage.setItem(headerStateKey, JSON.stringify({ theme: headerState.theme }));
      localStorage.setItem(preferredThemeKey, headerState.theme);
      localStorage.setItem(themeDataKey, JSON.stringify({ name: headerState.theme, values: themeCatalog.themes[headerState.theme] || {} }));
    };
    const themeButton = document.querySelector('[data-theme-button]');
    const adminButton = document.querySelector('[data-admin-button]');
    const changePinButton = document.querySelector('[data-change-pin-button]');
    const modalBackdrop = document.querySelector('[data-pin-modal-backdrop]');
    const modalTitle = document.getElementById('pin-modal-title');
    const modalMessage = document.querySelector('[data-pin-modal-message]');
    const infoBackdrop = document.querySelector('[data-info-modal-backdrop]');
    const infoTitle = document.getElementById('info-modal-title');
    const infoBody = document.querySelector('[data-info-modal-body]');
    const themeChoiceRow = document.querySelector('[data-theme-choice-row]');
    const currentPinInput = document.querySelector('[data-pin-current]');
    const changeCurrentPinInput = document.querySelector('[data-pin-change-current]');
    const newPinInput = document.querySelector('[data-pin-new]');
    const confirmPinInput = document.querySelector('[data-pin-confirm]');
    const internetIndicator = document.querySelector('[data-internet-status-indicator]');
    const internetState = { status: 'loading', publicIp: undefined, ipDetails: undefined, speedTestResults: null, speedTestError: null, isSpeedTesting: false };
    let modalMode = 'enter';
    function themeToCss(theme) {
      if (!theme) return '';
      const lines = [];
      Object.entries(theme).forEach(([key, value]) => {
        lines.push('  --theme-' + key + ': ' + value + ';');
      });
      const hexToRgb = value => {
        const match = /^#([0-9a-f]{6})$/i.exec(String(value || '').trim());
        if (!match) return null;
        const number = Number.parseInt(match[1], 16);
        return [(number >> 16) & 255, (number >> 8) & 255, number & 255].join(', ');
      };
      const primaryRgb = hexToRgb(theme.primary);
      const backgroundRgb = hexToRgb(theme.background);
      lines.push('  --theme-accent-soft: color-mix(in srgb, var(--theme-accent) 16%, transparent);');
      if (primaryRgb) lines.push('  --theme-primary-rgb: ' + primaryRgb + ';');
      if (backgroundRgb) lines.push('  --theme-background-rgb: ' + backgroundRgb + ';');
      return ':root {\n' + lines.join('\n') + '\n}';
    }
    function themeLabel(name) {
      return name;
    }
    function installCrownDebugEmitter() {
      const endpoint = '/api/debug/emit', ttlMs = 5 * 60 * 1000;
      const families = Object.freeze({ 'immortal-floor': ['immortal-floor-boot'], requests: ['crown-requests'], layout: ['crown-layout'], sessions: ['crown-sessions'], streams: ['crown-streams'], caduceus: ['crown-caduceus'] });
      const safeKind = value => /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(String(value || ''));
      const familyFor = kind => Object.keys(families).find(family => families[family].includes(kind)) || kind;
      const read = () => { try { const value = JSON.parse(localStorage.getItem('coronatioDiagnostics') || '{}'); return value.expiresAt > Date.now() ? value : { enabled: [] }; } catch (_) { return { enabled: [] }; } };
      const write = value => { try { localStorage.setItem('coronatioDiagnostics', JSON.stringify(value)); } catch (_) {} };
      function debugParams() { try { return new URLSearchParams(window.location.search || ''); } catch (_) { return new URLSearchParams(''); } }
      function storageValue(name) { try { return window.localStorage?.getItem(name) || ''; } catch (_) { return ''; } }
      function enabled(kind) {
        if (!safeKind(kind)) return false;
        const params = debugParams();
        if (params.has('debug')) { const selected = params.get('debug'); return selected === null || selected === '' || selected === '1' || selected === 'true' || selected.split(',').map(value => value.trim()).includes(kind); }
        if (storageValue('coronatioDebug') === '1' || storageValue('coronatioDebug') === 'true') return true;
        if (storageValue('coronatioDebugKinds').split(',').map(value => value.trim()).includes(kind)) return true;
        return read().enabled.includes(familyFor(kind));
      }
      function refreshControl() {
        const state = read(), active = state.enabled || [], label = document.querySelector('[data-crown-diagnostics-state]');
        if (label) label.textContent = active.length ? `Active for ${Math.max(0, Math.ceil((state.expiresAt - Date.now()) / 1000))}s` : 'Off';
        document.querySelectorAll('[data-crown-diagnostic-toggle]').forEach(button => { const on = active.includes(button.dataset.crownDiagnosticToggle); button.setAttribute('aria-pressed', String(on)); button.classList.toggle('active', on); });
      }
      function toggle(family) { const state = read(), enabled = new Set(state.enabled || []); if (enabled.has(family)) enabled.delete(family); else enabled.add(family); write({ enabled: [...enabled], expiresAt: Date.now() + ttlMs }); refreshControl(); return enabled.has(family); }
      function clean(value, depth = 0) { if (value === null || value === undefined) return value; if (depth > 4) return '[trimmed]'; if (typeof value === 'string') return value.length > 512 ? value.slice(0, 512) + '…' : value; if (typeof value === 'number' || typeof value === 'boolean') return value; if (Array.isArray(value)) return value.slice(0, 32).map(item => clean(item, depth + 1)); if (typeof value === 'object') { const out = {}; Object.keys(value).slice(0, 48).forEach(key => { const lower = key.toLowerCase(); if (/(token|pin|password|secret|capability|connection)/.test(lower) || lower.includes('body') || lower.includes('payload') || lower === 'headers' || lower.includes('localstorage') || lower.includes('dom') || lower.includes('snapshot') || lower.includes('source')) return; out[key] = clean(value[key], depth + 1); }); return out; } return String(value); }
      function debugId() { return window.crypto?.randomUUID ? window.crypto.randomUUID() : 'dbg-' + Date.now().toString(36) + '-' + Math.random().toString(36).slice(2, 10); }
      function now() { return Math.round(performance.now()); }
      function emit(kind, event, attrs = {}) { if (!enabled(kind)) return false; try { window.fetch(endpoint, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ kind, event, correlationId: attrs?.correlationId || attrs?.bootId || attrs?.runId, attributes: clean(attrs) }), keepalive: true }).catch(() => {}); return true; } catch (_) { return false; } }
      function begin(kind, attrs = {}) { if (!enabled(kind)) return null; return { kind, correlationId: attrs.correlationId || attrs.bootId || attrs.runId || debugId(), startedAt: now(), attrs: clean(attrs), marks: [], settled: false }; }
      function mark(handleOrKind, markName, attrs = {}) { if (!handleOrKind) return false; if (typeof handleOrKind === 'string') return emit(handleOrKind, 'mark', Object.assign({ mark: markName, phase: markName }, attrs)); if (handleOrKind.settled || !enabled(handleOrKind.kind)) return false; handleOrKind.marks.push({ mark: markName, phase: attrs.phase || markName, t: now() - handleOrKind.startedAt, attributes: clean(attrs) }); return true; }
      function settle(handleOrKind, ok, attrs = {}) { if (!handleOrKind) return false; if (typeof handleOrKind === 'string') return emit(handleOrKind, ok ? 'settle-ok' : 'settle-fault', Object.assign({ ok }, attrs)); if (handleOrKind.settled || !enabled(handleOrKind.kind)) return false; handleOrKind.settled = true; return emit(handleOrKind.kind, attrs.event || 'settle', Object.assign({}, handleOrKind.attrs, attrs, { ok: Boolean(ok), correlationId: handleOrKind.correlationId, durationMs: now() - handleOrKind.startedAt, marks: handleOrKind.marks })); }
      window.setInterval(refreshControl, 1000); window.addEventListener('DOMContentLoaded', () => { document.querySelectorAll('[data-crown-diagnostic-toggle]').forEach(button => button.addEventListener('click', () => toggle(button.dataset.crownDiagnosticToggle))); refreshControl(); });
      return Object.freeze({ enabled, emit, begin, mark, settle, toggle, families });
    }
    function installCrownRequestDiagnostics(crownDebug) {
      const safePane = value => /^[a-z0-9-]+$/.test(String(value || '')) ? String(value) : undefined;
      const safePath = value => { try { const url = new URL(String(value || ''), window.location.origin); return url.origin === window.location.origin && url.pathname !== '/api/debug/emit' ? url.pathname : null; } catch (_) { return null; } };
      const started = new WeakMap();
      const metadata = (event, phase) => {
        const detail = event.detail || {}, xhr = detail.xhr, config = detail.requestConfig || {}, path = safePath(config.path || detail.pathInfo?.requestPath || xhr?.responseURL);
        if (!path) return null;
        const began = xhr && started.get(xhr), pane = safePane((detail.elt || detail.target || event.target)?.closest?.('[data-view-panel]')?.dataset?.viewPanel);
        const attrs = { phase, pathname: path };
        const method = String(config.verb || config.method || '').toUpperCase(); if (/^(GET|POST|PUT|PATCH|DELETE)$/.test(method)) attrs.method = method;
        if (Number.isInteger(xhr?.status) && xhr.status >= 0) attrs.status = xhr.status;
        if (began !== undefined) attrs.durationMs = Math.max(0, Math.round(performance.now() - began));
        if (pane) attrs.pane = pane;
        return attrs;
      };
      const observe = (name, phase) => document.body.addEventListener(name, event => {
        if (!crownDebug.enabled('crown-requests')) return;
        const xhr = event.detail?.xhr;
        if (name === 'htmx:beforeRequest' && xhr) started.set(xhr, performance.now());
        const attrs = metadata(event, phase); if (attrs) crownDebug.emit('crown-requests', 'htmx', attrs);
        if ((name === 'htmx:afterRequest' || name === 'htmx:responseError' || name === 'htmx:sendError') && xhr) started.delete(xhr);
      });
      observe('htmx:beforeRequest', 'before-request'); observe('htmx:afterRequest', 'after-request'); observe('htmx:responseError', 'response-error'); observe('htmx:sendError', 'send-error'); observe('htmx:beforeSwap', 'before-swap'); observe('htmx:afterSwap', 'after-swap');
    }
    function installCrownLayoutDiagnostics(crownDebug) {
      if (!window.PerformanceObserver) return;
      const observe = type => { try { new PerformanceObserver(list => {
        if (!crownDebug.enabled('crown-layout')) return;
        list.getEntries().forEach(entry => {
          const attrs = { entryType: type, name: String(entry.name || type).slice(0, 96), duration: Math.max(0, Number(entry.duration || 0)), phase: type };
          if (type === 'layout-shift') { attrs.value = Math.max(0, Number(entry.value || 0)); attrs.hadRecentInput = Boolean(entry.hadRecentInput); }
          crownDebug.emit('crown-layout', 'performance-entry', attrs);
        });
      }).observe({ type, buffered: true }); } catch (_) {} };
      ['paint', 'layout-shift', 'longtask'].forEach(observe);
    }
    function renderThemeChoices() {
      if (!themeChoiceRow) return;
      themeChoiceRow.innerHTML = '';
      themes.forEach(name => {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'theme-choice';
"####
}
