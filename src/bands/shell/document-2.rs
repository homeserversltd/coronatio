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
        <article class="backblaze-tablet" data-backblaze-tablet><section class="ui-card backblaze-config-card"><div class="ui-card__header"><h2>Configuration</h2><span class="ui-badge ui-badge--secondary" data-backblaze-config-lock hidden>Locked</span></div><div class="ui-card__body"><form data-backblaze-config-form><div class="backblaze-form-grid"><label class="ui-input">Key ID<input type="text" name="keyId" autocomplete="off" required></label><label class="ui-input">Application Key<input type="password" name="applicationKey" autocomplete="new-password" required></label><label class="ui-input">Bucket<input type="text" name="bucket" autocomplete="off" required></label><label class="ui-input">Prefix <span>(optional)</span><input type="text" name="prefix" autocomplete="off"></label></div><fieldset class="backblaze-path-editor"><legend>Paths</legend><div data-backblaze-path-editor></div><button type="button" class="ui-button ui-button--secondary" data-backblaze-path-add>Add path</button></fieldset><div class="backblaze-actions"><button type="submit" class="ui-button ui-button--primary" data-backblaze-config-save>Save configuration</button><span data-backblaze-config-caption aria-live="polite"></span></div><p class="backblaze-config-result" data-backblaze-config-result role="status" aria-live="polite"></p></form></div></section><section class="ui-card"><div class="ui-card__header"><h2>Readiness</h2></div><div class="ui-card__body backblaze-checklist"><div class="backblaze-check-row"><span class="ui-badge ui-badge--secondary" data-backblaze-check="restic">Checking</span><span>restic installed</span></div><div class="backblaze-check-row"><span class="ui-badge ui-badge--secondary" data-backblaze-check="config">Checking</span><span data-backblaze-config-label>tabs.backblaze.config present and valid</span></div><div class="backblaze-check-row"><span class="ui-badge ui-badge--secondary" data-backblaze-check="credentials">Checking</span><span>Keyman Backblaze credential available</span></div></div></section><section class="ui-card"><div class="ui-card__header"><h2>Last run <span class="ui-badge ui-badge--secondary" data-backblaze-badge>Checking</span></h2></div><div class="ui-card__body"><dl class="backblaze-detail-list"><div><dt>State</dt><dd data-backblaze-state>Checking</dd></div><div><dt>Last run time</dt><dd data-backblaze-time>Never</dd></div><div><dt>Size</dt><dd data-backblaze-size>Unknown</dd></div><div><dt>Result</dt><dd data-backblaze-status aria-live="polite">Checking backup status…</dd></div></dl></div><div class="ui-card__footer backblaze-actions"><button type="button" class="ui-button ui-button--primary" data-backblaze-run>Run Backup Now</button><span data-backblaze-result aria-live="polite"></span></div></section><section class="ui-card"><div class="ui-card__header"><h2>Snapshots</h2></div><div class="ui-card__body"><div class="ui-table-container"><table class="ui-table ui-table--responsive"><thead><tr><th>Time</th><th>ID</th><th>Paths</th></tr></thead><tbody data-backblaze-snapshots></tbody></table></div><p class="backblaze-empty" data-backblaze-snapshots-empty aria-live="polite">Loading snapshots…</p></div></section><section class="ui-card"><div class="ui-card__header"><h2>Protected paths</h2></div><div class="ui-card__body"><div data-backblaze-paths class="backblaze-paths" aria-live="polite">Loading configured paths…</div></div></section><p class="backblaze-note">Backup schedule: manual only.</p></article>
      </section>
      <section class="pane" id="pane-wake-on-lan" data-pane-panel="wake-on-lan" data-view-panel="wake-on-lan" role="tabpanel" aria-label="Wake on LAN">
        <div class="wake-on-lan-tablet" data-wake-on-lan-tablet><header class="wake-on-lan-header"><div><h2>Wake on LAN</h2><p>Devices known to DHCP.</p></div></header><div class="wake-on-lan-error" data-wake-on-lan-error role="alert" hidden></div><ul class="wake-on-lan-rows" data-wake-on-lan-rows aria-live="polite"></ul></div>
      </section>
      <section class="pane" id="pane-linker" data-pane-panel="linker" data-view-panel="linker" role="tabpanel" aria-label="Linker" data-admin-only="true" data-admin-viewport="linker">
        <article class="linker-tablet" data-linker-tablet><p>Linker admission is loaded when this pane is admitted.</p></article>
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
            <div class="dhcp-banner dhcp-banner--loading" data-dhcp-loading aria-live="polite"><span class="loading-spinner medium" role="progressbar" aria-label="Loading devices"></span><span>Loading device identities…</span></div>
            <section class="identity-roster-host" data-identity-roster-host="dhcp" data-identity-perspective="address"><h3>Devices</h3><div data-identity-roster></div></section>
          </div>
        </div>
        <div class="ui-modal-backdrop" data-identity-claim-modal aria-hidden="true">
          <section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="identity-claim-title">
            <h2 id="identity-claim-title">Claim device</h2><div data-identity-claim-body></div>
            <div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-identity-claim-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-identity-claim-save>Save</button></div>
          </section>
        </div>
        <div class="ui-modal-backdrop" data-identity-reservation-modal aria-hidden="true">
          <section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="identity-reservation-title">
            <h2 id="identity-reservation-title" data-identity-reservation-title>Reservation</h2><div data-identity-reservation-body></div>
            <div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-identity-reservation-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-identity-reservation-save>Save</button></div>
          </section>
        </div>
      </section>
      <section class="pane" id="pane-firewall" data-pane-panel="firewall" data-view-panel="firewall" role="tabpanel" aria-label="Firewall">
        <div class="firewall-tablet" data-firewall-tablet data-admin-only="true" data-admin-viewport="firewall">
          <header class="firewall-header"><div><h2>Firewall</h2><p>Child-device website access.</p></div><button class="ui-button ui-button--secondary ui-button--small" type="button" data-firewall-refresh>Refresh</button></header>
          <div class="firewall-banner firewall-banner--error" data-firewall-error role="alert" hidden></div>
          <section class="firewall-section"><h3>Observed devices</h3><p>MAC addresses seen on the network.</p><ul class="firewall-list" data-firewall-observed></ul></section>
          <section class="firewall-section"><h3>Child devices</h3><p>Registered child devices can have a website whitelist.</p><ul class="firewall-list" data-firewall-children></ul></section>
          <pre class="firewall-receipt" data-firewall-receipt aria-live="polite"></pre>
        </div>
        <div class="ui-modal-backdrop" data-firewall-modal-backdrop aria-hidden="true"><section class="ui-modal" role="dialog" aria-modal="true" aria-labelledby="firewall-modal-title"><h2 id="firewall-modal-title" data-firewall-modal-title>Website whitelist</h2><form data-firewall-host-form><label class="firewall-modal-field">Website hostname<input class="ui-input ui-input--medium" data-firewall-host-input maxlength="253" placeholder="example.com"></label><button class="ui-button ui-button--secondary ui-button--small" type="submit">Add website</button></form><ul class="firewall-list" data-firewall-hosts></ul><div class="ui-modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-modal-cancel>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-firewall-modal-save>Save whitelist</button></div></section></div>
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
      <div class="modal-content"><p>A loadable cartridge is admitted at runtime through the tab registry with process-level fault isolation. Cartridge ingress will arrive through the Caduceus twin-snake method. Today, tabs are first-party native crown panes compiled into the crown.</p></div>
      <div class="modal-actions"><button type="button" class="secondary" data-add-tab-modal-close>Close</button></div>
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
        panel.dataset.viewportFaulted = 'true';
      }
      document.documentElement.dataset.cartridgeFaultReceipt = 'typed';
      document.documentElement.dataset.cartridgeFaultLast = kind;
      showCoronatioToast(`Pane could not be loaded (${kind}).`, 'error');
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
    function renderThemeChoices() {
      if (!themeChoiceRow) return;
      themeChoiceRow.innerHTML = '';
      themes.forEach(name => {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'theme-choice';
"####
}
