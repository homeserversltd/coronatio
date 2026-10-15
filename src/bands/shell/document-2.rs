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
      <section class="pane" id="pane-headless" data-pane-panel="headless" data-view-panel="headless" role="tabpanel" aria-label="Disconnected">
        <article class="headless-tablet" data-crown-headless-viewport>
          <img class="headless-tablet-logo" src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNjAiIGhlaWdodD0iMTYwIiB2aWV3Qm94PSIwIDAgMTYwIDE2MCI+PHJlY3Qgd2lkdGg9IjE2MCIgaGVpZ2h0PSIxNjAiIHJ4PSIzMiIgZmlsbD0iIzFmMjkzNyIvPjxwYXRoIGQ9Ik0yOCA4MiA4MCAzNGw1MiA0OHY0NEgyOHoiIGZpbGw9IiM2MGE1ZmEiLz48cGF0aCBkPSJNNTkgMTI2Vjg2aDQydjQwIiBmaWxsPSIjMWYyOTM3Ii8+PHRleHQgeD0iODAiIHk9IjE1MSIgZmlsbD0id2hpdGUiIGZvbnQtZmFtaWx5PSJzYW5zLXNlcmlmIiBmb250LXNpemU9IjEzIiBmb250LXdlaWdodD0iNzAwIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIj5IT01FU0VSVkVSPC90ZXh0Pjwvc3ZnPg==" alt="HomeServer logo">
          <h1>You have been disconnected due to inactivity.</h1>
          <button type="button" class="headless-tablet-reload" data-crown-headless-reload>Reload Page</button>
          <small>Product of HOMESERVER LLC</small>
        </article>
      </section>
      <section class="pane active" id="pane-stats" data-pane-panel="stats" data-view-panel="stats" data-scaffold-root data-guest-id="stats" data-scaffold-version="1" role="tabpanel" aria-label="Stats">
        <div class="stats-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-admin-only="true" data-wake-on-lan-open hidden aria-hidden="true">Wake on LAN</button></div>
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
      <section class="pane" id="pane-upload" data-pane-panel="upload" data-view-panel="upload" data-scaffold-root data-guest-id="upload" data-scaffold-version="1" role="tabpanel" aria-label="Upload">
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
              <div class="directory-tree-container" data-upload-tree data-scaffold-slot="upload-tree" data-scaffold-placeholder="directory entries" role="tree">__UPLOAD_TREE_FRAGMENT__</div>
            </div>
            <div class="file-upload-section" data-upload-regular="file-ingress" data-upload-file-section>
              <input type="file" multiple data-upload-file aria-label="Upload files">
              <button type="button" data-upload-submit disabled>Upload Selected Files</button>
            </div>
          </div>
          <div class="modal-overlay" data-admin-only="true" data-upload-history-backdrop data-upload-history-modal aria-hidden="true" hidden>
            <section class="modal" role="dialog" aria-modal="true" aria-labelledby="upload-history-title">
              <button type="button" class="modal-close" data-upload-modal-close aria-label="Close modal">×</button><h2 class="modal-title" id="upload-history-title">Upload History</h2>
              <div class="modal-content upload-history-modal-content"><div class="uploadHistoryModal empty"><div class="upload-history-empty-message">No upload history available</div></div><div class="upload-history-list" hidden></div><button type="button" class="clear-history-button" data-upload-clear-history disabled>Clear History</button></div>
            </section>
          </div>
          <div class="modal-overlay" data-admin-only="true" data-upload-blacklist-backdrop data-upload-blacklist-modal aria-hidden="true" hidden>
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
      <section class="pane" id="pane-backblaze" data-pane-panel="backblaze" data-view-panel="backblaze" data-admin-only-pane="true" data-scaffold-root data-guest-id="backblaze" data-scaffold-version="1" role="tabpanel" aria-label="Backblaze">
        <article class="backblaze-tablet" data-backblaze-tablet>
          <div class="backblaze-ribbon" data-backblaze-ribbon><span class="backblaze-state-pill" data-backblaze-state>Loading</span><span>Last: <strong data-backblaze-last>—</strong> <strong data-backblaze-duration>duration unavailable</strong></span><span>Next: <strong data-backblaze-next>Not scheduled</strong></span></div>
          <nav class="backblaze-subtabs" aria-label="Backblaze sections" role="tablist"><button type="button" class="backblaze-subtab is-active" data-backblaze-view="buckets" role="tab" aria-selected="true">Buckets</button><button type="button" class="backblaze-subtab" data-backblaze-view="ledger" role="tab" aria-selected="false">Ledger</button></nav>
          <section class="backblaze-view" data-backblaze-panel="buckets" role="tabpanel">
            <div class="backblaze-two-pane"><aside class="backblaze-bucket-pane"><header><h2>Buckets</h2><button type="button" class="ui-button ui-button--primary" data-backblaze-add aria-label="Add bucket">+</button></header><div data-backblaze-buckets data-scaffold-slot="backblaze-buckets" data-scaffold-placeholder="backup buckets" aria-live="polite"><div class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line"></span><span class="ui-skeleton__line ui-skeleton__line--medium"></span><span class="ui-skeleton__line ui-skeleton__line--short"></span></div></div></aside>
              <section class="backblaze-items-pane"><header><div><h2 data-backblaze-selected>Select a bucket</h2><p data-backblaze-bucket-status>Select a bucket.</p></div><button type="button" class="ui-button ui-button--primary" data-backblaze-run disabled>Run backup</button></header>
                <div class="backblaze-config"><section class="backblaze-section"><h3>Bucket</h3><label class="backblaze-bucket-value">Name <input class="ui-input" data-backblaze-bucket-name readonly></label><label class="ui-checkbox"><input type="checkbox" data-backblaze-encrypted> Encrypted</label></section><section class="backblaze-section"><h3>Account</h3><div class="backblaze-account-row"><span class="backblaze-account-status" data-backblaze-connected>—</span><button type="button" class="ui-button ui-button--secondary" data-backblaze-verify disabled>Verify</button><button type="button" class="ui-button ui-button--secondary" data-backblaze-update disabled>Update credentials</button></div></section><section class="backblaze-section"><h3>Files</h3><div class="backblaze-path-input"><input class="ui-input" data-backblaze-path placeholder="/absolute/path" disabled><button type="button" class="ui-button ui-button--secondary" data-backblaze-item-add disabled>Add</button></div><div data-backblaze-items data-scaffold-slot="backblaze-items" data-scaffold-placeholder="backup paths"><div class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line ui-skeleton__line--medium"></span><span class="ui-skeleton__line ui-skeleton__line--short"></span></div></div></section><section class="backblaze-section"><h3>Schedule</h3><div class="backblaze-form-grid"><label>Frequency <select class="ui-input" disabled><option>Manual</option></select></label><label>Time <input class="ui-input" type="time" disabled></label></div><p class="backblaze-note">Schedule persistence is unavailable; backups run manually.</p></section><section class="backblaze-section"><h3>Managers</h3><div class="backblaze-manager-row" data-backblaze-manager="forgejo"><span class="backblaze-manager-indicator" data-backblaze-manager-indicator aria-hidden="true"></span><strong>Forgejo</strong><button type="button" class="ui-button ui-button--secondary" data-backblaze-manager-open disabled>Open</button></div></section><button type="button" class="ui-button ui-button--primary" data-backblaze-save disabled>Save Config</button></div><p data-backblaze-result role="status"></p></section>
              <div class="backblaze-manager-modal" data-backblaze-manager-modal hidden><div class="backblaze-manager-card" role="dialog" aria-modal="true" aria-label="Forgejo managers"><h3>Forgejo</h3><label class="ui-checkbox"><input type="checkbox" data-backblaze-local-nas> Local NAS backup</label><label class="ui-checkbox"><input type="checkbox" data-backblaze-forgejo-backup> Backblaze backup to this bucket.</label><div><button type="button" class="ui-button ui-button--secondary" data-backblaze-manager-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary" data-backblaze-manager-close>Done</button></div></div></div>
            </div>
          </section><section class="backblaze-view" data-backblaze-panel="ledger" role="tabpanel" hidden><div class="backblaze-two-pane"><aside class="backblaze-bucket-pane"><header><h2>Buckets</h2></header><div data-backblaze-ledger-buckets data-scaffold-slot="backblaze-ledger-buckets" data-scaffold-placeholder="backup buckets" aria-live="polite"><div class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line"></span><span class="ui-skeleton__line ui-skeleton__line--short"></span></div></div></aside><section class="backblaze-items-pane" data-backblaze-ledger data-scaffold-slot="backblaze-ledger" data-scaffold-placeholder="backup ledger" aria-live="polite"><div class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line"></span><span class="ui-skeleton__line ui-skeleton__line--medium"></span><span class="ui-skeleton__block"></span></div></section></div></section>
        </article>
      </section>
      <section class="pane" id="pane-linker" data-pane-panel="linker" data-view-panel="linker" data-admin-only-pane="true" role="tabpanel" aria-label="Linker" data-admin-only="true" data-admin-viewport="linker">
        <article class="linker-tablet" data-linker-tablet><p>Linker admission is loaded when this pane is admitted.</p></article>
      </section>
      __TEST__
      __CARTRIDGE_PANES__
      <section class="pane" id="pane-dhcp" data-pane-panel="dhcp" data-view-panel="dhcp" data-scaffold-root data-guest-id="dhcp" data-scaffold-version="1" role="tabpanel" aria-label="DHCP">
        <div class="dhcp-tablet" data-dhcp-tablet data-admin-viewport="dhcp">
          <div class="dhcp-info-banner" data-dhcp-info-banner data-scaffold-slot="dhcp-statistics" data-scaffold-placeholder="DHCP statistics" aria-live="polite">
            <span class="dhcp-info-item">Homeserver: <span class="dhcp-info-value" data-dhcp-homeserver>—</span></span><span class="dhcp-info-separator" aria-hidden="true">|</span>
            <span class="dhcp-info-item">Reservations: <span class="dhcp-info-value" data-dhcp-reservations-count>—</span></span><span class="dhcp-info-separator" aria-hidden="true">|</span>
            <span class="dhcp-info-item">Hosts: <span class="dhcp-info-value" data-dhcp-hosts-count>—</span></span><span class="dhcp-info-separator" aria-hidden="true">|</span>
            <span class="dhcp-info-item">Leases: <span class="dhcp-info-value" data-dhcp-leases-count>—</span></span>
          </div>
          <div class="dhcp-button-row" data-admin-only="true">
            <button type="button" class="ui-button ui-button--secondary ui-button--small" data-dhcp-refresh>Refresh</button>
            <label class="ui-toggle ui-toggle--medium"><span class="ui-toggle__switch"><input type="checkbox" class="ui-toggle__input" data-dhcp-anonymize><span class="ui-toggle__slider"></span></span><span class="ui-toggle__label">Anonymize</span></label>
            <div class="reservation-slider" data-dhcp-boundary-control data-scaffold-slot="dhcp-pool-boundary" data-scaffold-placeholder="DHCP pool boundary">
              <label for="dhcp-reservation-boundary">Reserved addresses: <strong class="dhcp-boundary-value" data-dhcp-boundary-value>—</strong> <span class="dhcp-boundary-value" data-dhcp-boundary-min>—</span>–<span class="dhcp-boundary-value" data-dhcp-boundary-max>—</span></label>
              <input id="dhcp-reservation-boundary" class="ui-slider" type="range" min="0" max="249" value="0" data-dhcp-boundary>
              <button type="button" class="ui-button ui-button--primary ui-button--small" data-dhcp-boundary-save>Apply</button>
            </div>
          </div>
          <div class="dhcp-tablet-content">
            <div class="dhcp-banner dhcp-banner--error" data-dhcp-error role="alert" hidden></div>
            <div class="dhcp-banner dhcp-banner--loading" data-dhcp-loading aria-live="polite"><span class="loading-spinner medium" role="progressbar" aria-label="Loading devices"></span><span>Loading device identities…</span></div><span hidden data-scaffold-slot="identity-boundary" data-scaffold-placeholder="network DHCP boundary"></span><span hidden data-scaffold-slot="identity-leases" data-scaffold-placeholder="DHCP leases"></span><span hidden data-scaffold-slot="identity-reservations" data-scaffold-placeholder="DHCP reservations"></span><span hidden data-scaffold-slot="identity-dns" data-scaffold-placeholder="DNS records"></span><span hidden data-scaffold-slot="identity-mutation" data-scaffold-placeholder="network change"></span>
            <section class="identity-roster-host dhcp-device-roster" data-identity-roster-host="dhcp" data-identity-perspective="address"><h3>Device addresses</h3><p class="dhcp-device-roster-hint">Pinned addresses are reserved. Lease-only devices can be pinned in one click.</p><div data-scaffold-slot="identity-roster" data-scaffold-placeholder="network device roster"><div data-identity-roster><div class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line"></span><span class="ui-skeleton__line ui-skeleton__line--medium"></span><span class="ui-skeleton__block"></span></div></div></div></section>
          </div>
        </div>
        <div class="ui-modal-backdrop" data-admin-only="true" data-identity-claim-modal aria-hidden="true">
          <section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="identity-claim-title">
            <h2 id="identity-claim-title">Claim device</h2><div data-identity-claim-body></div>
            <div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-identity-claim-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-identity-claim-save>Save</button></div>
          </section>
        </div>
        <div class="ui-modal-backdrop" data-admin-only="true" data-identity-reservation-modal aria-hidden="true">
          <section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="identity-reservation-title">
            <h2 id="identity-reservation-title" data-identity-reservation-title>Reservation</h2><div data-identity-reservation-body></div>
            <div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-identity-reservation-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-identity-reservation-save>Save</button></div>
          </section>
        </div>
      </section>
      <section class="pane" id="pane-firewall" data-pane-panel="firewall" data-view-panel="firewall" data-admin-only-pane="true" data-scaffold-root data-guest-id="firewall" data-scaffold-version="1" role="tabpanel" aria-label="Firewall">
        <div class="firewall-tablet" data-firewall-tablet data-admin-only="true" data-admin-viewport="firewall">
          <header class="firewall-header"><div><h2>Firewall</h2><p>Child-device website access.</p></div><button class="ui-button ui-button--secondary ui-button--small" type="button" data-firewall-refresh>Refresh</button></header>
          <div class="firewall-banner firewall-banner--error" data-firewall-error role="alert" hidden></div>
          <section class="firewall-section"><h3>Observed devices</h3><p>Hostname, IP address and MAC address seen on the network.</p><div data-scaffold-slot="firewall-observed" data-scaffold-placeholder="observed devices"><ul class="firewall-list" data-firewall-observed><li class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line"></span><span class="ui-skeleton__line ui-skeleton__line--short"></span></li></ul></div></section>
          <section class="firewall-section"><h3>Child devices</h3><p>Registered child devices can have a website whitelist.</p><div data-scaffold-slot="firewall-children" data-scaffold-placeholder="registered child devices"><ul class="firewall-list" data-firewall-children><li class="ui-skeleton" aria-hidden="true"><span class="ui-skeleton__line ui-skeleton__line--medium"></span><span class="ui-skeleton__line ui-skeleton__line--short"></span></li></ul></div></section>
          <pre class="firewall-receipt" data-firewall-receipt aria-live="polite"></pre>
        </div>
        <div class="ui-modal-backdrop" data-firewall-modal-backdrop aria-hidden="true"><section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="firewall-modal-title"><h2 id="firewall-modal-title" data-firewall-modal-title>Website whitelist</h2><form data-firewall-host-form><label class="firewall-modal-field">Website hostname<input class="ui-input ui-input--medium" data-firewall-host-input maxlength="253" placeholder="example.com"></label><button class="ui-button ui-button--secondary ui-button--small" type="submit">Add website</button></form><div data-scaffold-slot="firewall-whitelist" data-scaffold-placeholder="website whitelist"><p data-firewall-whitelist-error role="alert" hidden></p><ul class="firewall-list" data-firewall-hosts></ul></div><div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-modal-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-firewall-modal-save>Save whitelist</button></div></section></div>
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
  <div class="modal-overlay" data-add-tab-modal aria-hidden="true" hidden>
    <section class="modal" role="dialog" aria-modal="true" aria-labelledby="add-tab-modal-title">
      <button type="button" class="modal-close" data-add-tab-modal-close aria-label="Close Add tab modal">×</button>
      <h2 class="modal-title" id="add-tab-modal-title">Add tab</h2>
      <div class="modal-content">
        <p>Loadable cartridges are the extension lane.</p>
        <form class="portal-form" data-cartridge-add-form>
          <div class="form-group"><label for="cartridge-choice">Cartridge</label><select id="cartridge-choice" name="cartridgeChoice"><option value="custom">Custom cartridge</option></select></div>
          <div class="form-group"><label for="cartridge-title">Title *</label><input id="cartridge-title" name="title" type="text" required maxlength="80" placeholder="e.g., Jellyfin"></div>
          <div class="form-group"><label for="cartridge-url">URL *</label><input id="cartridge-url" name="url" type="url" required placeholder="http://jellyfin.home.arpa:8096"></div>
          <div class="form-group"><label><input name="adminOnly" type="checkbox"> Admin only</label></div>
          <div class="modal-actions"><button type="button" class="secondary" data-add-tab-modal-close>Cancel</button><button type="submit" class="submit-button">Add tab</button></div>
        </form>
        <section data-cartridge-management><h3>Loadable cartridges</h3><ul data-cartridge-management-list></ul></section>
      </div>
    </section>
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
    function serviceCardFromHtmxEvent(event) {
      const detail = event.detail || {};
      for (const candidate of [detail.target, detail.elt, event.target]) {
        if (!(candidate instanceof Element)) continue;
        const card = candidate.closest('[data-service-card]');
        if (card instanceof HTMLElement) return card;
      }
      return null;
    }
    function panelFromHtmxEvent(event) {
      if (serviceCardFromHtmxEvent(event)) return null;
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
      const xhr = event.detail?.xhr || null;
      const floor = window.immortalFloor;
      const admission = floor?.admissionOwner(xhr);
      if (!admission) return;
      const panel = panelFromHtmxEvent(event);
      if (!(panel instanceof HTMLElement) || panel !== admission.pane) return;
      const panelId = admission.guest;
      // A structure failure belongs to this admission request; the floor's
      // admission owner decides whether an outgoing healthy view survives.
      document.documentElement.dataset.cartridgeFaultReceipt = 'typed';
      document.documentElement.dataset.cartridgeFaultLast = kind;
      showCoronatioToast(`Pane could not be loaded (${kind}).`, 'error');
      const coldFault = floor.failAdmission(xhr, kind) === true;
      if (coldFault && panelId) {
        xeniaRetryIntent = panelId;
        scheduleXeniaColdRetry(panelId);
      }
    }
    function presentServiceCardFault(kind, event) {
      const card = serviceCardFromHtmxEvent(event);
      if (!(card instanceof HTMLElement)) return false;
      const id = card.dataset.serviceCard || '';
      const samba = id === 'samba-file-sharing';
      const status = document.createElement('div');
      status.className = samba ? 'samba-status' : 'ssh-status';
      status.dataset.adminToggleCard = id || 'service';
      status.dataset.serviceCardFaultKind = String(kind || 'service-card-unavailable');
      const heading = document.createElement('h3');
      heading.textContent = id === 'ssh-password-authentication'
        ? 'SSH Password Authentication'
        : id === 'ssh-service'
          ? 'SSH Service'
          : samba
            ? 'Samba File Sharing'
            : 'Service';
      const controls = document.createElement('div');
      controls.className = samba ? 'samba-toggle' : 'ssh-toggle';
      const label = document.createElement('span');
      label.className = 'toggle-label';
      label.textContent = 'Unavailable';
      controls.append(label);
      status.append(heading, controls);
      card.replaceChildren(status);
      card.dataset.serviceCardFaultKind = String(kind || 'service-card-unavailable');
      return true;
    }
    document.body.addEventListener('htmx:timeout', event => presentCartridgeFault('timeout', event));
    document.body.addEventListener('htmx:responseError', event => presentCartridgeFault(faultKindFromResponse(event, 'upstream-error'), event));
    document.body.addEventListener('htmx:sendError', event => presentCartridgeFault('proxy-unreachable', event));
    const xeniaColdRetries = new Map();
    const xeniaRetryDelays = [5000, 15000, 30000];
    let xeniaRetryIntent = null;
    function cancelXeniaColdRetries(nextIntent = null) {
      for (const [id, retry] of xeniaColdRetries) {
        if (retry.timer !== null) window.clearTimeout(retry.timer);
        retry.timer = null;
        if (!retry.inFlight) xeniaColdRetries.delete(id);
      }
      xeniaRetryIntent = nextIntent;
    }
    function scheduleXeniaColdRetry(id) {
      if (!id || xeniaRetryIntent !== id) return;
      let retry = xeniaColdRetries.get(id);
      if (!retry) {
        retry = { attempt: 0, timer: null, inFlight: false };
        xeniaColdRetries.set(id, retry);
      }
      if (retry.timer !== null || retry.inFlight) return;
      const delay = xeniaRetryDelays[Math.min(retry.attempt, xeniaRetryDelays.length - 1)];
      retry.timer = window.setTimeout(async () => {
        retry.timer = null;
        if (xeniaRetryIntent !== id || xeniaColdRetries.get(id) !== retry || window.getImmortalFloorState?.() === 'Seated') {
          xeniaColdRetries.delete(id);
          return;
        }
        retry.inFlight = true;
        retry.attempt += 1;
        try {
          const seated = await showPane(id, { refresh: true });
          const panel = panes.find(candidate => candidate.dataset.panePanel === id);
          if (seated === true && window.getImmortalFloorState?.() === 'Seated' && panel?.classList.contains('active')) {
            retry.inFlight = false;
            cancelXeniaColdRetries(null);
            return;
          }
        } catch (_) {}
        retry.inFlight = false;
        if (xeniaRetryIntent === id && xeniaColdRetries.get(id) === retry) scheduleXeniaColdRetry(id);
        else xeniaColdRetries.delete(id);
      }, delay);
    }
    document.body.addEventListener('click', event => {
      const tab = event.target instanceof Element ? event.target.closest('[data-pane]') : null;
      if (tab?.dataset.pane && !event.target.closest('button')) cancelXeniaColdRetries(tab.dataset.pane);
    }, true);
    document.body.addEventListener('keydown', event => {
      const tab = event.target instanceof Element ? event.target.closest('[data-pane]') : null;
      if (tab?.dataset.pane && !event.target.closest('button') && (event.key === 'Enter' || event.key === ' ')) cancelXeniaColdRetries(tab.dataset.pane);
    }, true);
    let statsHydrationInFlight = false;
    document.body.addEventListener('htmx:afterSwap', event => {
      const card = serviceCardFromHtmxEvent(event);
      if (card instanceof HTMLElement) {
        const fault = card.querySelector('[data-service-card-fault-kind]');
        if (fault instanceof HTMLElement && fault.dataset.serviceCardFaultKind) {
          card.dataset.serviceCardFaultKind = fault.dataset.serviceCardFaultKind;
        } else {
          delete card.dataset.serviceCardFaultKind;
        }
        return;
      }
      const panel = panelFromHtmxEvent(event);
      if (!(panel instanceof HTMLElement)) return;
      const xhr = event.detail?.xhr || null;
      if (window.immortalFloor?.isAdmissionRequest(xhr) && window.immortalFloor?.admissionOwner(xhr)?.pane === panel) panel.dataset.viewportFaulted = 'false';
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
    let themeCatalog = { default: 'dark', themes: {} };
    let themes = [];
    const savedHeaderState = (() => { try { return JSON.parse(localStorage.getItem(headerStateKey) || '{}'); } catch (_) { return {}; } })();
    const savedPreferredTheme = localStorage.getItem(preferredThemeKey);
    const headerState = { theme: savedPreferredTheme || savedHeaderState.theme || themeCatalog.default, isAdmin: false };
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
    const internetState = { status: 'loading', publicIp: undefined, speedTestResults: null, speedTestError: null, isSpeedTesting: false };
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
    function renderThemeChoices() {
      if (!themeChoiceRow) return;
      themeChoiceRow.innerHTML = '';
      themes.forEach(name => {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'theme-choice';
"####
}
