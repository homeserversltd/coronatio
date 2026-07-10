fn shell_document_2() -> &'static str {
    r####"  </style>
  <script src="/static/vendor/chart.umd.min.js" data-chart-dependency="chartjs-4.4.0"></script>
  <script src="/static/vendor/chartjs-plugin-datalabels.min.js" data-chart-dependency="chartjs-plugin-datalabels-2.2.0"></script>
</head>
<body>
  <main class="app" data-product="Coronatio" data-source-material="homeserver-main-site" data-admin-mode="false">
    <header class="top-bar header" data-flask-react-quarry="Header">
      <div class="header-top-row">
        <div class="header-left"><span class="uptime" data-uptime-indicator title="Server uptime">connecting...</span></div>
        <div class="header-center">
          <div class="status-indicators" aria-label="Status indicators">
            <button type="button" class="indicator ok tailscale-indicator" data-indicator="tailscale" data-modal-kind="tailscale" data-modal-title="Tailscale Status" aria-label="Tailscale Status" title="Tailscale Status"><svg class="indicator-icon" data-packed-icon="network-wired" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 5h16v8H4V5zm2 2v4h12V7H6zm5 8h2v2h7v2h-7v2h-2v-2H4v-2h7v-2z"/></svg></button>
            <button type="button" class="indicator loading internet-indicator" data-indicator="internet" data-modal-kind="internet" data-modal-title="Internet Status" data-internet-status-indicator aria-label="Checking Internet Status" title="Checking internet connection..."><svg class="indicator-icon indicator-icon-plug" data-packed-icon="plug" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 2h2v5h2V2h2v5h2V2h2v7a5 5 0 0 1-4 4.9V17h3v2h-3v3h-2v-3H8v-2h3v-3.1A5 5 0 0 1 7 9V2z"/></svg><svg class="indicator-icon indicator-icon-spinner" data-packed-icon="spinner" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2a10 10 0 0 1 10 10h-3a7 7 0 0 0-7-7V2z"/></svg></button>
            <button type="button" class="indicator warn openvpn-indicator" data-indicator="openvpn" data-modal-kind="openvpn" data-modal-title="VPN & Transmission Configuration" aria-label="VPN & Transmission Configuration" title="VPN & Transmission Configuration"><svg class="indicator-icon" data-packed-icon="lock" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 10V8a5 5 0 0 1 10 0v2h2v11H5V10h2zm2 0h6V8a3 3 0 0 0-6 0v2zm2 4v3h2v-3h-2z"/></svg></button>
            <button type="button" class="indicator warn services-indicator" data-indicator="services" data-modal-kind="services" data-modal-title="Services Status" aria-label="Services Status" title="Services Status"><svg class="indicator-icon" data-packed-icon="server" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 3h16v8H4V3zm2 2v4h12V5H6zm-2 8h16v8H4v-8zm2 2v4h12v-4H6zm9-8h2v2h-2V7zm0 10h2v2h-2v-2z"/></svg></button>
            <button type="button" class="indicator power-indicator" data-indicator="power-meter" data-modal-kind="power-meter" data-modal-title="Power Consumption" aria-label="Measuring Power Usage" title="Measuring power usage"><svg class="indicator-icon" data-packed-icon="bolt" viewBox="0 0 24 24" aria-hidden="true"><path d="M13 2 4 14h7l-1 8 10-13h-7l1-7z"/></svg><span class="power-value-small" data-power-indicator-value><span class="power-value-small-number">—</span><span class="power-value-small-unit">W</span></span></button>
          </div>
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
    <section class="content">
      <section class="pane" id="pane-admin" data-pane-panel="admin" data-view-panel="admin" role="tabpanel" aria-label="Admin">

        <div class="admin-tablet" data-admin-quarry="flask-react-admin" data-admin-quarry-button-total="74" data-admin-only="true" data-admin-viewport="admin">
          <section class="mb-6" style="margin-bottom: 0.5rem">
          <div class="system-controls-container" data-admin-quarry-group="system-controls" aria-label="System controls">
            <span class="admin-quarry-note" data-admin-quarry-count-readback hidden>74 buttons</span>
            <div class="system-controls" data-admin-action-strip="single-row" data-admin-action-strip-count="7">
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="1" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="1" data-admin-action-id="hard-drive-test" hx-post="/admit/admin/action/hard-drive-test" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">▣</span><span>Hard Drive Test</span></button>
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="2" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="2" data-admin-action-id="update" hx-post="/admit/admin/action/update" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">⬇</span><span>Update</span></button>
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="3" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="3" data-admin-action-id="restart" hx-post="/admit/admin/action/restart" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Double Click to Restart: confirm system restart."><span class="admin-action-icon">⟳</span><span>Restart</span></button>
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="4" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="4" data-admin-action-id="shutdown" hx-post="/admit/admin/action/shutdown" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Double Click to Shut Down: confirm system shutdown."><span class="admin-action-icon">⏻</span><span>Shutdown</span></button>
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="5" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="5" data-admin-action-id="restart-website" hx-post="/admit/admin/action/restart-website" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">↻</span><span>Restart Website</span></button>
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="6" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="6" data-admin-action-id="view-logs" hx-get="/admit/admin/action/view-logs" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">▤</span><span>View Logs</span></button>
              <button type="button" class="system-controls-btn" data-admin-quarry-button data-admin-quarry-index="7" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="7" data-admin-action-id="install-certificate" hx-post="/admit/admin/action/install-certificate" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">◆</span><span>Install Certificate</span></button>
            </div>
            <div class="update-status-container" data-admin-action-result data-og-affordance="toast-mapped-to-result-strip" aria-live="polite">Ready</div>
            <span class="admin-quarry-note" hidden>7 buttons · system-controls</span>
            <div class="system-service-controls" data-admin-service-controls data-state-source="/api/services/data">
              <div class="ssh-controls">
                <div class="ssh-control" data-service-card="ssh-password-authentication" data-state-field="ssh.password_auth_enabled" data-state-source="/api/services/data">
                  __ADMIN_SSH_PASSWORD_CARD__
                </div>
                <div class="ssh-control" data-service-card="ssh-service" data-state-field="sshd.running" data-state-source="/api/services/data">
                  __ADMIN_SSH_SERVICE_CARD__
                </div>
              </div>
              <div class="samba-control" data-service-card="samba-file-sharing" data-state-field="samba.running" data-state-source="/api/services/data">
                __ADMIN_SAMBA_SERVICE_CARD__
              </div>
            </div>
          </div>
          </section>

          <section class="mb-6" style="margin-bottom: 0.5rem">
          <div class="key-manager" data-admin-quarry-group="key-manager">
            <h3><span class="admin-action-icon">⚿</span> Key Management</h3>
            <div class="key-manager-content">
              <div class="key-manager-left">
                <div class="security-status">
                  <div class="status-item"><span class="status-icon secure">🛡</span><div class="status-details"><p>This is the key to your vault. When you boot your HOMESERVER and visit home.arpa, this is what unlocks your encrypted storage system - just like unlocking your smartphone. Your /vault partition contains the sensitive keys stored on the device. Unlock the vault and everything HOMESERVER specifically stores is accessible. This is the device's master key.</p></div></div>
                </div>
              </div>
              <div class="key-manager-right"><div class="key-actions">
                <button type="button" class="action-button create-button" data-admin-quarry-button data-admin-quarry-index="21" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>+ Create New Key</span></button>
                <button type="button" class="action-button update-button" data-admin-quarry-button data-admin-quarry-index="22" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>⟳ Update Key on Drive</span></button>
                <button type="button" class="action-button admin-password-button" data-admin-quarry-button data-admin-quarry-index="23" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>🔒 Admin Password</span></button>
              </div></div>
            </div>
            <span class="admin-quarry-note" hidden>3 buttons · key-manager</span>
          </div>
          </section>

          <section class="mb-6" style="margin-bottom: 0.5rem">
          <div class="disk-manager" data-admin-quarry-group="disk-manager">
            <div class="disk-manager-container">
              <div class="disk-column"><h4>Available Devices</h4><div class="disk-list" data-admin-devices-readback="/api/services/data">__ADMIN_AVAILABLE_DEVICES__</div></div>
              <div class="disk-column"><h4>Mount Destinations</h4><div class="disk-list" data-admin-mounts-readback="/api/services/data">__ADMIN_MOUNT_DESTINATIONS__</div></div>
            </div>
          </div>
          </section>

          <section class="admin-modal-shelf" aria-label="Admin modal controls">
            <article class="modal-window" data-admin-quarry-group="admin-password-modal"><div class="modal-titlebar">Admin Password <span class="admin-quarry-note">2 buttons · admin-password-modal</span></div><div class="modal-body-port"><label class="form-group">New password<input class="field" value="••••••" readonly></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="27" data-admin-quarry-source="components/modals/AdminPasswordModal.tsx" data-admin-quarry-placement="admin password modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="28" data-admin-quarry-source="components/modals/AdminPasswordModal.tsx" data-admin-quarry-placement="admin password modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Update Password</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="create-key-modal"><div class="modal-titlebar">Create New Key <span class="admin-quarry-note">2 buttons · create-key-modal</span></div><div class="modal-body-port"><label class="radio-option selected"><input type="radio" checked readonly><span class="radio-label"><span class="radio-label-title">NAS drive key</span></span></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="29" data-admin-quarry-source="components/modals/CreateKeyModal.tsx" data-admin-quarry-placement="create key modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="30" data-admin-quarry-source="components/modals/CreateKeyModal.tsx" data-admin-quarry-placement="create key modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Create Key</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="hard-drive-test-modal"><div class="modal-titlebar">Hard Drive Test <span class="admin-quarry-note">6 buttons · hard-drive-test-modal</span></div><div class="modal-body-port"><div class="modal-toolbar"><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="31" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Refresh Devices</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="32" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Run New Test</span></button></div><div class="update-status-container">Device test ready</div><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="33" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Run New Test</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="34" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>View Results</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="35" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Back to Results</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="36" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Start Test</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="log-viewer-modal"><div class="modal-titlebar">Log Viewer <span class="admin-quarry-note">6 buttons · log-viewer-modal</span></div><div class="modal-body-port"><div class="modal-toolbar"><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="37" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Refresh logs</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="38" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Copy logs to clipboard</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="39" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Download logs</span></button> <button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="40" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Clear logs</span></button></div><pre class="log-frame">system log readback preview...</pre><div class="modal-pager"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="41" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Previous</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="42" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Next</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="password-input-modal"><div class="modal-titlebar">Password Required <span class="admin-quarry-note">3 buttons · password-input-modal</span></div><div class="modal-body-port"><label class="form-group">Password<input class="field" type="password" value="••••" readonly></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="43" data-admin-quarry-source="components/modals/PasswordInputModal.tsx" data-admin-quarry-placement="password prompt controls" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Show/Hide password</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="44" data-admin-quarry-source="components/modals/PasswordInputModal.tsx" data-admin-quarry-placement="password prompt controls" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="45" data-admin-quarry-source="components/modals/PasswordInputModal.tsx" data-admin-quarry-placement="password prompt controls" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Submit</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="premium-tab-modal"><div class="modal-titlebar">Premium Tab Manager <span class="admin-quarry-note">16 buttons · premium-tab-modal</span></div><div class="modal-body-port"><div class="modal-toolbar"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="46" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Back</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="47" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="48" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Validate &amp; Clone</span></button></div><pre class="log-frame">repository clone/install log preview...</pre><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="49" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Copy logs to clipboard</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="50" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Refresh logs</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="51" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Back</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="52" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="53" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="8" data-stub-action="true" aria-disabled="true" ><span>Confirm</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="54" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="9" data-stub-action="true" aria-disabled="true" ><span>Add Repository</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="55" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="10" data-stub-action="true" aria-disabled="true" ><span>Install All</span></button><button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="56" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="11" data-stub-action="true" aria-disabled="true" ><span>Uninstall All</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="57" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="12" data-stub-action="true" aria-disabled="true" ><span>View Logs</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="58" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="13" data-stub-action="true" aria-disabled="true" ><span>Refresh</span></button><button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="59" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="14" data-stub-action="true" aria-disabled="true" ><span>Uninstall</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="60" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="15" data-stub-action="true" aria-disabled="true" ><span>Install</span></button><button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="61" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="16" data-stub-action="true" aria-disabled="true" ><span>Delete</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="root-ca-modal"><div class="modal-titlebar">Root CA Certificate <span class="admin-quarry-note">5 buttons · root-ca-modal</span></div><div class="modal-body-port"><div class="update-status-container">Certificate installed</div><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="62" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="63" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Confirm Refresh</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="64" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Download Certificate</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="65" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Refresh Certificate</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="66" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Close</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="sync-schedule-modal"><div class="modal-titlebar">Auto Sync Schedule <span class="admin-quarry-note">2 buttons · sync-schedule-modal</span></div><div class="modal-body-port"><label class="form-group">Schedule<select class="field"><option>Daily</option></select></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="67" data-admin-quarry-source="components/modals/SyncScheduleModal.tsx" data-admin-quarry-placement="sync schedule modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="68" data-admin-quarry-source="components/modals/SyncScheduleModal.tsx" data-admin-quarry-placement="sync schedule modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Save Schedule</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="system-action-modal"><div class="modal-titlebar">System Action <span class="admin-quarry-note">1 buttons · system-action-modal</span></div><div class="modal-body-port"><p class="system-action-text">Action output</p><pre class="log-frame">waiting...</pre><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="69" data-admin-quarry-source="components/modals/SystemModals.tsx" data-admin-quarry-placement="system progress modal" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Copy Logs</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="update-key-modal"><div class="modal-titlebar">Update Key on Drive <span class="admin-quarry-note">2 buttons · update-key-modal</span></div><div class="modal-body-port"><label class="form-group">Drive<select class="field"><option>/dev/sdb</option></select></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="70" data-admin-quarry-source="components/modals/UpdateKeyModal.tsx" data-admin-quarry-placement="update key modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="71" data-admin-quarry-source="components/modals/UpdateKeyModal.tsx" data-admin-quarry-placement="update key modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Update Key</span></button></div></div></article>
            <article class="modal-window update-manager-modal" data-admin-quarry-group="update-manager-modal"><div class="modal-titlebar">Update Manager <span class="admin-quarry-note">19 buttons · update-manager-modal</span></div><div class="update-manager-header"><div class="view-tabs"><button type="button" class="tab-button active admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="82" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="11" data-stub-action="true" aria-disabled="true" ><span>Overview</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="83" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="12" data-stub-action="true" aria-disabled="true" ><span>Modules</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="84" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="13" data-stub-action="true" aria-disabled="true" ><span>Schedule</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="85" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="14" data-stub-action="true" aria-disabled="true" ><span>Interactives</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="86" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="15" data-stub-action="true" aria-disabled="true" ><span>Logs</span></button></div></div><div class="modal-body-port update-manager-content"><div class="update-status-container">Updates available</div><table class="modules-table"><tr><th>Module</th><th>Branch</th><th>Actions</th></tr><tr><td>coronatio</td><td>main</td><td><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="72" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Refresh Modules</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="73" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Toggle Module</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="74" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Apply Branch</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="75" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Reset to default</span></button></td></tr></table><div class="modal-toolbar"><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="76" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Refresh Interactives</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="77" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Run Interactive</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="78" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span>Save Schedule</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="79" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="8" data-stub-action="true" aria-disabled="true" ><span>Back to Overview</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="80" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="9" data-stub-action="true" aria-disabled="true" ><span>Copy Contents</span></button> <button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="81" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="10" data-stub-action="true" aria-disabled="true" ><span>Refresh Logfile</span></button></div><div class="update-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="87" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="16" data-stub-action="true" aria-disabled="true" ><span>Close</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="88" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="17" data-stub-action="true" aria-disabled="true" ><span>Check Updates</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="89" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="18" data-stub-action="true" aria-disabled="true" ><span>Update</span></button> <button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="90" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="19" data-stub-action="true" aria-disabled="true" ><span>Force Update</span></button></div></div></article>
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
          <div class="portals-grid" data-portals-grid data-portals-source="/api/portals/elements" data-portals-fragment="/api/portals/elements" hx-get="/api/portals/elements" hx-trigger="load, portals-refresh" hx-swap="innerHTML">
            <article class="portal-card portal-loading" data-portals-loading><div><h2>Admitted services</h2><p>Reading homeserver.json portal entries.</p></div></article>
          </div>
          <div class="portal-modal-overlay" data-add-portal-modal hidden>
            <div class="portal-modal-content">
              <div class="add-portal-modal" data-portal-modal-not-wired="true">
                <div class="modal-header"><h2>Add New Portal</h2><button type="button" class="close-button" data-portal-modal-close aria-label="Close modal"><i class="fas fa-times"></i></button></div>
                <form class="portal-form" data-portal-add-form aria-disabled="true" data-portal-create-not-wired="true">
                  <div class="form-group"><label for="portal-name">Portal Name *</label><input id="portal-name" type="text" placeholder="e.g., MyApp" aria-disabled="true" data-portal-create-not-wired="true"><small class="help-text">Create portal endpoint is not wired in this tranche.</small></div>
                  <div class="form-group"><label for="portal-description">Description *</label><input id="portal-description" type="text" placeholder="e.g., My custom application" aria-disabled="true" data-portal-create-not-wired="true"></div>
                  <div class="form-group"><label for="portal-type">Service Type</label><select id="portal-type" aria-disabled="true" data-portal-create-not-wired="true"><option value="systemd">Systemd Service</option><option value="script">Script-managed Service</option><option value="link">Link Only</option></select><small class="help-text">Systemd services can be controlled directly. Script-managed services require system restart. Link-only portals are simple links without service management.</small></div>
                  <div class="form-group"><label for="portal-services">Services *</label><input id="portal-services" type="text" placeholder="e.g., myapp, myapp-worker (comma-separated)" aria-disabled="true" data-portal-create-not-wired="true"><small class="help-text">Enter service names separated by commas</small></div>
                  <div class="form-group"><label for="portal-port">Port *</label><input id="portal-port" type="number" min="1" max="65535" placeholder="e.g., 8080" aria-disabled="true" data-portal-create-not-wired="true"></div>
                  <div class="form-group"><label for="portal-local-url">Local URL *</label><input id="portal-local-url" type="url" placeholder="e.g., https://myapp.home.arpa" aria-disabled="true" data-portal-create-not-wired="true"></div>
                  <div class="form-actions"><button type="button" class="cancel-button" data-portal-modal-close>Cancel</button><button type="submit" class="submit-button" aria-disabled="true" data-portal-create-not-wired="true"><i class="fas fa-plus"></i> Create Portal</button></div>
                </form>
              </div>
            </div>
          </div>
          <div class="portal-modal-overlay" data-service-status-modal hidden>
            <div class="portal-modal-content">
              <div class="service-status-modal"><pre class="service-status-content" data-service-status-content></pre><button type="button" class="copy-button" data-service-status-copy>Copy to Clipboard</button></div>
            </div>
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
                <button type="button" class="toggle-pin-button" data-admin-only data-admin-viewport="upload" data-upload-pin-toggle title="Enable PIN requirement for uploads (Currently Off)" aria-label="Toggle PIN requirement (currently disabled)"></button>
              </div>
              <div class="directory-breadcrumb-container">
                <div class="breadcrumb-navigation" data-upload-breadcrumbs><span class="breadcrumb-item current" data-path="/mnt/nas">nas</span></div>
              </div>
              <div class="directory-loading-initial" data-upload-directory-loading hidden>Loading directory tree…</div>
              <div class="directory-tree-container" data-upload-tree role="tree">__UPLOAD_TREE_FRAGMENT__</div>
            </div>
            <div class="file-upload-section" data-upload-regular="file-ingress" data-upload-file-section>
              <input type="file" multiple data-upload-file aria-label="Upload files">
              <button type="button" data-upload-submit disabled>Upload Selected Files</button>
            </div>
          </div>
          <div class="modal-window" data-upload-history-modal hidden>
            <div class="modal-titlebar">Upload History</div>
            <div class="upload-history-modal-content"><div class="uploadHistoryModal empty"><div class="upload-history-empty-message">No upload history available</div></div><div class="upload-history-list" hidden></div><button type="button" class="clear-history-button" data-upload-clear-history disabled>Clear History</button></div>
          </div>
          <div class="modal-window" data-upload-blacklist-modal hidden>
            <div class="modal-titlebar">Manage Blacklist</div>
            <div class="blacklist-manager"><div class="blacklist-entries" data-upload-blacklist-entries></div><div class="blacklist-controls"><form class="add-entry" data-upload-blacklist-form><input type="text" class="entry-input" placeholder="Enter path to blacklist" data-upload-blacklist-input><button type="button" class="add-button" data-upload-blacklist-add>New</button></form><button type="button" class="submit-button" data-upload-blacklist-submit>Submit</button></div></div>
          </div>
          <div class="modal-window" data-upload-pin-modal hidden>
            <div class="modal-titlebar">Admin PIN Required</div>
            <form class="pin-modal-form" data-upload-pin-form><p>Please enter the admin PIN to proceed with the upload.</p><input type="text" name="username" autocomplete="username" style="position:absolute;left:-9999px;opacity:0" tabindex="-1" aria-hidden="true"><input type="password" placeholder="Admin PIN" class="pin-input" autocomplete="new-password" data-upload-pin-input></form>
            <div class="modal-actions"><button type="button" class="secondary" data-upload-pin-cancel>Cancel</button><button type="button" data-upload-pin-confirm>Confirm</button></div>
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
      <section class="pane" id="pane-chia-mining" data-pane-panel="chia-mining" data-view-panel="chia-mining" role="tabpanel" aria-label="Chia Mining">
        <article class="card og-stub-pane" data-og-stub-pane="chia-mining"><h2>Chia Mining</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-dhcp" data-pane-panel="dhcp" data-view-panel="dhcp" role="tabpanel" aria-label="DHCP">
        <article class="card og-stub-pane" data-og-stub-pane="dhcp"><h2>DHCP</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-youtube" data-pane-panel="youtube" data-view-panel="youtube" role="tabpanel" aria-label="YouTube">
        <article class="card og-stub-pane" data-og-stub-pane="youtube"><h2>YouTube</h2><p>not yet ported</p></article>
      </section>
    </section>
  </main>
  <script>
    const appRoot = document.querySelector('[data-product="Coronatio"]');
    const tabBar = document.querySelector('[role="tablist"]');
    let tabs = [...document.querySelectorAll('[data-pane]')];
    const panes = [...document.querySelectorAll('[data-pane-panel]')];
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
      if (panel instanceof HTMLElement) {
        panel.innerHTML = `<section class="card error-message" data-cartridge-fault-presentation="og-pane"><h2>Cartridge fault</h2><p>${kind}</p></section>`;
        panel.dataset.viewportFaulted = 'true';
      }
      document.documentElement.dataset.cartridgeFaultReceipt = 'typed';
      document.documentElement.dataset.cartridgeFaultLast = kind;
    }
    document.body.addEventListener('htmx:timeout', event => presentCartridgeFault('timeout', event));
    document.body.addEventListener('htmx:responseError', event => presentCartridgeFault(faultKindFromResponse(event, 'upstream-error'), event));
    document.body.addEventListener('htmx:sendError', event => presentCartridgeFault('proxy-unreachable', event));
    document.body.addEventListener('htmx:afterSwap', event => {
      const panel = panelFromHtmxEvent(event);
      if (!(panel instanceof HTMLElement)) return;
      panel.dataset.viewportFaulted = 'false';
      const id = panel.dataset.viewPanel || panel.dataset.panePanel || '';
      if (id === 'stats') hydrateStats();
      if (id === 'portals') hydratePortals();
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
      const aliasMap = { background: '--background', text: '--text', primary: '--primary', primaryHover: '--primaryHover', secondary: '--secondary', accent: '--accent', error: '--error', success: '--success', warning: '--warning', border: '--border', statusUp: '--status-up', statusDown: '--status-down', statusPartial: '--status-partial', statusUnknown: '--status-unknown', hiddenTabBackground: '--hiddenTabBackground', hiddenTabText: '--hiddenTabText' };
      const lines = [];
      Object.entries(theme).forEach(([key, value]) => {
        lines.push('  --theme-' + key + ': ' + value + ';');
        if (aliasMap[key]) lines.push('  ' + aliasMap[key] + ': ' + value + ';');
      });
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
        button.className = 'theme-choice';"####
}
