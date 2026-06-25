fn render_crown_shell() -> String {
    let nav = render_flask_react_tabbar_quarry();

    let shell = r###"<!doctype html>
<html lang="en" class="theme-loaded">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Coronatio</title>
  <style>
    :root {
      color-scheme: dark;
      --theme-color-primary: #00f2fe;
      --theme-color-secondary: #4CAF50;
      --theme-bg-primary: #2a2a2a;
      --theme-bg-secondary: #1a1a1a;
      --theme-bg-tertiary: #222222;
      --theme-bg-hover: #333333;
      --theme-bg-active: #3a3a3a;
      --theme-text-primary: #ffffff;
      --theme-text-secondary: #dddddd;
      --theme-text-tertiary: #a7a7a7;
      --theme-text-disabled: #777777;
      --theme-text-accent: #00f2fe;
      --theme-status-success: #4CAF50;
      --theme-status-error: #f44336;
      --theme-status-warning: #ff9800;
      --theme-status-info: #2196f3;
      --theme-spacing-xs: 0.25rem;
      --theme-spacing-sm: 0.5rem;
      --theme-spacing-md: 1rem;
      --theme-spacing-lg: 1.5rem;
      --theme-spacing-xl: 2rem;
      --theme-font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --theme-font-mono: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
      --theme-font-size-xs: 12px;
      --theme-font-size-sm: 0.875rem;
      --theme-font-size-base: 16px;
      --theme-font-size-md: 16px;
      --theme-font-size-lg: 1.125rem;
      --theme-font-size-xl: 24px;
      --theme-font-weight-normal: 400;
      --theme-font-weight-medium: 500;
      --theme-font-weight-bold: 600;
      --theme-line-height-tight: 1.2;
      --theme-line-height-normal: 1.5;
      --theme-line-height-loose: 1.8;
      --theme-transition-fast: 150ms ease;
      --theme-transition-normal: 250ms ease;
      --theme-transition-slow: 350ms ease;
      --theme-shadow-sm: 0 1px 2px rgba(0,0,0,0.1);
      --theme-shadow-md: 0 2px 4px rgba(0,0,0,0.1);
      --theme-shadow-lg: 0 4px 8px rgba(0,0,0,0.1);
      --theme-radius: 4px;
      --background: var(--theme-bg-secondary);
      --surface: var(--theme-bg-primary);
      --surface-soft: var(--theme-bg-tertiary);
      --text: var(--theme-text-primary);
      --text-secondary: var(--theme-text-secondary);
      --accent: var(--theme-color-primary);
      --accent-soft: color-mix(in srgb, var(--theme-color-primary) 16%, transparent);
      --primary: var(--theme-color-secondary);
      --border: color-mix(in srgb, var(--theme-text-primary) 14%, transparent);
      --error: var(--theme-status-error);
      --warning: var(--theme-status-warning);
      --success: var(--theme-status-success);
      --shadow: var(--theme-shadow-md);
      font-family: var(--theme-font-family);
      background: var(--background);
      color: var(--text);
    }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: var(--background); color: var(--text); }
    .app { min-height: 100vh; display: flex; flex-direction: column; }
    .top-bar {
      min-height: 48px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      padding: 0 20px;
      background: var(--surface);
      border-bottom: 1px solid var(--border);
      box-shadow: var(--shadow);
    }
    .header-top-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; min-width: 0; flex: 1 1 auto; }
    .header-left, .header-center, .header-right { display: flex; align-items: center; gap: .5rem; }
    .header-center { justify-content: center; flex: 2 1 auto; min-width: 0; }
    .header-right { justify-content: flex-end; flex-wrap: wrap; }
    .uptime { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: .82rem; padding: .24rem .5rem; border-radius: 6px; background: rgba(255,255,255,.06); color: var(--text); }
    .status-indicators { display: flex; flex-direction: row; align-items: center; justify-content: center; gap: 8px; }
    .indicator { cursor: pointer; padding: .25rem; display: flex; align-items: center; justify-content: center; border: 0; border-radius: 6px; background: transparent; color: var(--text); transition: transform .2s ease, background-color .2s ease, box-shadow .2s ease; }
    .indicator:hover { background-color: var(--primary-hover); transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,0,0,.15); }
    .indicator:active { transform: translateY(1px); box-shadow: 0 1px 2px rgba(0,0,0,.1); }
    .indicator-icon { font-family: "Font Awesome 6 Free", "Font Awesome 5 Free", sans-serif; font-weight: 900; font-size: 1.15rem; line-height: 1; }
    .indicator.ok .indicator-icon { color: var(--success); }
    .indicator.warn .indicator-icon { color: var(--warning); }
    .modal-body { display: grid; gap: .75rem; color: var(--text-secondary); }
    .modal-body ul { margin: .25rem 0 0; padding-left: 1.15rem; }
    .modal-section, .status-section, .config-section, .credentials-section, .service-controls, .power-history-section, .speed-test-section { display: grid; gap: .55rem; }
    .modal-grid { display: grid; gap: .55rem; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); }
    .modal-body input { width: 100%; padding: .55rem .65rem; border: 1px solid var(--border); border-radius: 6px; background: var(--background); color: var(--text); }
    .status-item, .power-average-row, .current-tailnet { display: flex; justify-content: space-between; gap: .75rem; border-bottom: 1px solid rgba(255,255,255,.08); padding-bottom: .35rem; }
    .status-value, .power-value, .power-average-value, .value { color: var(--text); font-weight: 700; }
    .action-output:empty { display: none; }
    .theme-choice-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-top: .5rem; }
    .theme-choice-row[hidden] { display: none !important; }
    .theme-choice { min-width: 90px; }
    .header-control, .admin-button, .theme-button, .change-admin-pin-button { min-height: 34px; min-width: 120px; padding: 0 .9rem; border: none; border-radius: 6px; background: var(--primary); color: #061006; font-size: .86rem; font-weight: 700; box-shadow: inset 0 2px 4px rgba(0,0,0,.2); }
    .header-control:hover, .admin-button:hover, .theme-button:hover, .change-admin-pin-button:hover { filter: brightness(1.08); transform: translateY(-1px); }
    .modal-backdrop { position: fixed; inset: 0; display: none; place-items: center; background: rgba(0,0,0,.55); z-index: 2000; padding: 1rem; }
    .modal-backdrop.open { display: grid; }
    .modal { width: min(420px, 100%); background: var(--surface); border: 1px solid var(--border); border-radius: 10px; box-shadow: 0 16px 40px rgba(0,0,0,.45); padding: 1rem; }
    .modal h2 { margin: 0 0 .75rem; font-size: 1.05rem; }
    .pin-modal { display: flex; flex-direction: column; gap: 16px; padding: 16px 0 0; }
    .pin-modal input { padding: 8px 12px; border: 1px solid var(--border); border-radius: 4px; background: var(--background); color: var(--text); font-size: 14px; outline: none; }
    .pin-modal input:focus { border-color: var(--primary); }
    .modal-actions { display: flex; justify-content: flex-end; gap: .5rem; margin-top: 1rem; }
    .toast-slot { min-height: 1.2rem; color: var(--warning); font-size: .84rem; margin-top: .5rem; }
    .status-strip { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: .45rem; color: var(--text-secondary); font-size: .82rem; }
    .status-pill { border: 1px solid var(--border); border-radius: 999px; padding: .18rem .55rem; background: rgba(255,255,255,.045); }
    .status-pill.ok { color: var(--success); border-color: color-mix(in srgb, var(--success) 45%, transparent); }
    .tab-bar {
      min-height: 48px;
      display: flex;
      align-items: center;
      gap: .5rem;
      padding: 0 20px;
      background: var(--surface-soft);
      border-bottom: 1px solid var(--border);
      overflow-x: auto;
    }
    .tab-bar.hidden { display: none; }
    .tab {
      display: grid;
      grid-template-columns: 28px minmax(max-content, 1fr) 28px;
      align-items: center;
      min-height: 38px;
      gap: .35rem;
      padding: .35rem .45rem;
      border-radius: 8px;
      border: 1px solid transparent;
      color: var(--text-secondary);
      font-weight: 600;
      cursor: pointer;
      transition: background .18s ease, color .18s ease, border-color .18s ease, opacity .18s ease;
      user-select: none;
    }
    .tab:hover, .tab.active, .tab[aria-selected="true"] {
      background: var(--accent-soft);
      color: var(--text);
      border-color: color-mix(in srgb, var(--accent) 42%, transparent);
    }
    .tab[data-visibility="hidden"] { opacity: .48; border-style: dashed; }
    .tab[data-visibility="hidden"] .tab-name { text-decoration: line-through; }
    .tab-visibility-column, .tab-star-column { display: grid; place-items: center; min-width: 24px; }
    .visibility-toggle, .star-button {
      display: grid;
      place-items: center;
      width: 24px;
      height: 24px;
      padding: 0;
      border: 1px solid transparent;
      border-radius: 999px;
      background: transparent;
      color: var(--text-secondary);
      line-height: 1;
    }
    .visibility-toggle:hover, .star-button:hover { background: rgba(255,255,255,.1); color: var(--text); }
    .visibility-toggle[data-visible="false"] { color: var(--warning); }
    .star-button.fas { color: var(--warning); }
    .star-button.far { color: rgba(255,255,255,.42); }
    .tab-name { white-space: nowrap; }
    .content { flex: 1; padding: 20px; }
    .pane { display: none; max-width: 1180px; margin: 0 auto; }
    .pane.active { display: block; }
    .pane-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(270px, 1fr)); gap: 16px; align-items: start; }
    .portal-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px; align-items: stretch; }
    .card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 1rem;
      box-shadow: var(--shadow);
      min-height: 112px;
    }
    .portal-card { min-height: 180px; display: flex; flex-direction: column; justify-content: space-between; gap: .9rem; }
    .card h2, .card h3 { margin: 0 0 .65rem; font-size: 1rem; color: var(--text); }
    .card p { margin: .25rem 0; color: var(--text-secondary); font-size: .92rem; }
    .metric { font-size: 1.65rem; font-weight: 750; color: var(--accent); line-height: 1.1; }
    .muted { color: var(--text-secondary); }
    .readout { margin-top: .75rem; padding: .75rem; border-radius: 6px; background: rgba(0,0,0,.22); border: 1px solid rgba(255,255,255,.08); font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: .78rem; color: #cdefff; overflow: auto; max-height: 220px; white-space: pre-wrap; }
    .button-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-top: .85rem; }
    button, .action-link { border: 1px solid var(--border); border-radius: 6px; padding: .55rem .7rem; background: var(--primary); color: #061006; font-weight: 700; cursor: pointer; text-decoration: none; }
    button.secondary, .action-link.secondary { background: transparent; color: var(--text); }
    .warning { color: var(--warning); }
    .error { color: var(--error); }
    .success { color: var(--success); }
    .drop-zone { border: 1px dashed color-mix(in srgb, var(--accent) 55%, transparent); border-radius: 8px; padding: 1.2rem; background: rgba(0,242,254,.07); }
    [data-admin-mode="false"] [data-admin-only] { display: none !important; }
    [data-admin-mode="true"] [data-admin-only] { display: revert; }
    [data-admin-only] { border-style: dashed; border-color: color-mix(in srgb, var(--warning) 42%, var(--border)); }
    @media (max-width: 760px) {
      .top-bar { align-items: stretch; flex-direction: column; padding: .75rem 1rem; }
      .header-top-row { flex-direction: column; align-items: stretch; }
      .header-left, .header-center, .header-right { justify-content: center; }
      .theme-button, .change-admin-pin-button, .admin-button { flex: 1 1 auto; min-width: 0; }
      .tab-bar, .content { padding-left: 12px; padding-right: 12px; }
      .portal-grid, .pane-grid { grid-template-columns: 1fr; }
    }
  </style>
</head>
<body>
  <main class="app" data-product="Coronatio" data-source-material="homeserver-main-site" data-admin-mode="false">
    <header class="top-bar header" data-flask-react-quarry="Header">
      <div class="header-top-row">
        <div class="header-left"><span class="uptime" data-uptime-indicator title="Server uptime">connecting...</span></div>
        <div class="header-center">
          <div class="status-indicators" aria-label="Status indicators">
            <button type="button" class="indicator ok tailscale-indicator" data-indicator="tailscale" data-modal-kind="tailscale" data-modal-title="Tailscale Status" aria-label="Tailscale Status" title="Tailscale Status"><span class="indicator-icon" data-fa-icon="network-wired" aria-hidden="true">&#xf6ff;</span></button>
            <button type="button" class="indicator ok internet-indicator" data-indicator="internet" data-modal-kind="internet" data-modal-title="Internet Status" aria-label="Internet Status" title="Internet Status"><span class="indicator-icon" data-fa-icon="plug" aria-hidden="true">&#xf1e6;</span></button>
            <button type="button" class="indicator warn openvpn-indicator" data-indicator="openvpn" data-modal-kind="openvpn" data-modal-title="VPN & Transmission Configuration" aria-label="VPN & Transmission Configuration" title="VPN & Transmission Configuration"><span class="indicator-icon" data-fa-icon="lock" aria-hidden="true">&#xf023;</span></button>
            <button type="button" class="indicator ok services-indicator" data-indicator="services" data-modal-kind="services" data-modal-title="Services Status" aria-label="Services Status" title="Services Status"><span class="indicator-icon" data-fa-icon="server" aria-hidden="true">&#xf233;</span></button>
            <button type="button" class="indicator warn power-indicator" data-indicator="power-meter" data-modal-kind="power-meter" data-modal-title="Power Consumption" aria-label="Power Consumption" title="Power Consumption"><span class="indicator-icon" data-fa-icon="bolt" aria-hidden="true">&#xf0e7;</span></button>
          </div>
        </div>
        <div class="header-right">
          <button type="button" class="theme-button" data-theme-button title="Current theme: dark. Open theme selector."><span>dark</span></button>
          <button type="button" class="change-admin-pin-button" data-change-pin-button hidden>Change PIN</button>
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
      <section class="pane active" id="pane-admin" data-pane-panel="admin" role="tabpanel" aria-label="Admin">
        <div class="pane-grid">
          <article class="card"><h2>Admin authority</h2><p>Caduceus is the live privileged membrane; Coronatio reads and triggers it through same-origin receipts.</p><div class="button-row"><button data-fetch="/api/caduceus/status" data-target="admin-session">Read live status</button><button class="secondary" data-admin-only data-admin-viewport="admin" data-fetch="/api/caduceus/update/check" data-target="admin-installer" data-method="POST">Check machine</button></div><pre class="readout" id="admin-session">Waiting for Caduceus status.</pre></article>
          <article class="card"><h2>Caduceus membrane</h2><p>Host mutation now enters Caduceus and Harmonia; policy still gates each command before root work runs.</p><div class="button-row"><button data-admin-only data-admin-viewport="admin" data-fetch="/api/caduceus/update/now" data-target="admin-installer" data-method="POST">Make harmonious</button><button class="secondary" data-fetch="/api/caduceus/receipts/latest" data-target="admin-installer">Latest receipt</button></div><pre class="readout" id="admin-installer">Harmonia receipt readback will appear here.</pre></article>
          <article class="card"><h2>Visible contract</h2><p><span class="success">On:</span> Caduceus/Harmonia routes are live; any blocked action names the first missing signal instead of pretending completion.</p></article>
        </div>
      </section>
      <section class="pane" id="pane-stats" data-pane-panel="stats" role="tabpanel" aria-label="Stats">
        <div class="pane-grid">
          <article class="card"><h2>System telemetry</h2><div class="metric" id="stats-load">—</div><p>Load average</p><pre class="readout" id="stats-readout">Fetching /api/stats…</pre></article>
          <article class="card"><h2>Stream lane</h2><p id="stats-stream">Stats stream state pending.</p><div class="button-row"><button data-fetch="/api/stats/events" data-target="stats-event">Read event frame</button><button class="secondary" data-admin-only data-admin-viewport="stats" data-fetch="/api/stats/events/renew" data-target="stats-event" data-method="POST">Renew lease</button></div><pre class="readout" id="stats-event">No event readback yet.</pre></article>
          <article class="card"><h2>Missing signal</h2><p id="stats-missing" class="warning">Checking collector status…</p></article>
        </div>
      </section>
      <section class="pane" id="pane-portals" data-pane-panel="portals" role="tabpanel" aria-label="Portals">
        <div class="portal-grid">
          <article class="card portal-card"><div><h2>Admitted services</h2><p>Portal cards follow the main HomeServer service-grid pattern and expose the live config contract.</p></div><div class="button-row"><button data-fetch="/api/services/data" data-target="portals-readout">Read service contract</button><a class="action-link secondary" href="https://home.arpa/">Open main HomeServer</a></div></article>
          <article class="card portal-card"><div><h2>Coronatio</h2><p>Rust crown preview, port 3013.</p></div><div class="button-row" data-admin-only data-admin-viewport="portals"><button data-fetch="/api/portals" data-method="POST" data-target="portals-readout">Add portal</button><button class="secondary" data-fetch="/api/portals/coronatio" data-method="PUT" data-target="portals-readout">Edit portal</button></div><span class="status-pill ok">online</span></article>
          <article class="card portal-card"><div><h2>Caduceus</h2><p>Privileged actuator membrane, port 3014.</p></div><div class="button-row"><button data-fetch="/api/caduceus/status" data-target="portals-readout">Status</button><a class="action-link secondary" href="http://home.arpa:3014/health">Health</a></div></article>
        </div>
        <pre class="readout" id="portals-readout">Service contract readback will appear here.</pre>
      </section>
      <section class="pane" id="pane-upload" data-pane-panel="upload" role="tabpanel" aria-label="Upload">
        <div class="pane-grid upload-viewport" data-upload-viewport>
          <article class="card upload-card"><h2>Safe file ingress</h2><form class="upload-form" data-upload-form><label>Destination <input class="field" name="path" data-upload-path value="/mnt/nas" autocomplete="off"></label><label>File <input class="field" name="file" data-upload-file type="file"></label><div class="button-row"><button type="submit">Upload through Caduceus</button><button type="button" class="secondary" data-admin-only data-admin-viewport="upload" data-fetch="/api/upload/default-directory" data-target="upload-readout">Default directory</button></div></form><div class="drop-zone" data-upload-drop><strong>Choose a file</strong><p>Coronatio reads the browser file and sends upload metadata through the Caduceus staff membrane.</p></div></article>
          <article class="card"><h2>Upload controls</h2><div class="button-row"><button class="secondary" data-fetch="/api/upload/history" data-target="upload-readout">History</button><button data-admin-only data-admin-viewport="upload" data-fetch="/api/upload/pin-required-status" data-target="upload-readout">PIN requirement</button><button class="secondary" data-admin-only data-admin-viewport="upload" data-fetch="/api/upload/blacklist/list" data-target="upload-readout">Blacklist</button></div><pre class="readout" id="upload-readout">Select a file to send the upload intent to Caduceus.</pre></article>
        </div>
      </section>
    </section>
  </main>
  <script>
    const appRoot = document.querySelector('[data-product="Coronatio"]');
    const tabBar = document.querySelector('[role="tablist"]');
    const tabs = [...document.querySelectorAll('[data-pane]')];
    const panes = [...document.querySelectorAll('[data-pane-panel]')];
    const fallbackTab = 'admin';
    const storageKey = 'coronatio.flask-react-tabbar.v1';
    const loadTabState = () => {
      try { return JSON.parse(localStorage.getItem(storageKey) || '{}'); }
      catch (_) { return {}; }
    };
    const saveTabState = state => localStorage.setItem(storageKey, JSON.stringify(state));
    const tabState = Object.assign({ starredTab: 'portals', hiddenTabs: [] }, loadTabState());
    const headerStateKey = 'coronatio.flask-react-header.v1';
    const preferredThemeKey = 'preferred-theme';
    const themeDataKey = 'themeData';
    let themeCatalog = { default: 'dark', themes: {} };
    let themes = [];
    const savedHeaderState = (() => { try { return JSON.parse(localStorage.getItem(headerStateKey) || '{}'); } catch (_) { return {}; } })();
    const savedPreferredTheme = localStorage.getItem(preferredThemeKey);
    const headerState = Object.assign({ theme: savedPreferredTheme || savedHeaderState.theme || 'dark', isAdmin: false }, savedHeaderState, { theme: savedPreferredTheme || savedHeaderState.theme || 'dark' });
    const saveHeaderState = () => {
      localStorage.setItem(headerStateKey, JSON.stringify(headerState));
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
    let modalMode = 'enter';
    function themeToCss(theme) {
      if (!theme) return '';
      return ':root {\n' + Object.entries(theme).map(([key, value]) => '  --theme-' + key + ': ' + value + ';').join('\n') + '\n}';
    }
    function themeLabel(name) {
      return name.split('-').map(part => part.charAt(0).toUpperCase() + part.slice(1)).join(' ');
    }
    function renderThemeChoices() {
      if (!themeChoiceRow) return;
      themeChoiceRow.innerHTML = '';
      themes.forEach(name => {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'theme-choice';
        button.dataset.themeChoice = name;
        button.textContent = themeLabel(name);
        button.addEventListener('click', () => {
          headerState.theme = name;
          saveHeaderState();
          applyTheme();
          closeInfoModal();
        });
        themeChoiceRow.appendChild(button);
      });
    }
    function ensureThemeStyleElement() {
      let style = document.querySelector('style[data-theme-styles]');
      if (!style) {
        style = document.createElement('style');
        style.setAttribute('data-theme-styles', '');
        document.head.appendChild(style);
      }
      return style;
    }
    function applyTheme() {
      if (!themes.includes(headerState.theme)) headerState.theme = themeCatalog.default || themes[0] || 'dark';
      const theme = themeCatalog.themes[headerState.theme];
      document.documentElement.dataset.theme = headerState.theme;
      ensureThemeStyleElement().textContent = themeToCss(theme);
      saveHeaderState();
      if (themeButton) {
        const label = themeLabel(headerState.theme);
        themeButton.querySelector('span').textContent = label;
        themeButton.title = 'Current theme: ' + label + '. Open theme selector.';
      }
      document.querySelectorAll('[data-theme-choice]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.themeChoice === headerState.theme)));
    }
    function setAdminMode(value) {
      headerState.isAdmin = Boolean(value);
      saveHeaderState();
      if (adminButton) {
        adminButton.dataset.adminState = headerState.isAdmin ? 'logged-in' : 'logged-out';
        adminButton.textContent = headerState.isAdmin ? 'Exit Admin Mode' : 'Enter Admin Mode';
      }
      if (appRoot) appRoot.dataset.adminMode = headerState.isAdmin ? 'true' : 'false';
      if (tabBar) tabBar.dataset.adminMode = headerState.isAdmin ? 'true' : 'false';
      document.querySelectorAll('[data-admin-only]').forEach(el => {
        el.hidden = !headerState.isAdmin;
        el.setAttribute('aria-hidden', String(!headerState.isAdmin));
      });
      if (changePinButton) changePinButton.hidden = !headerState.isAdmin;
    }
    function openPinModal(mode) {
      modalMode = mode;
      modalTitle.textContent = mode === 'change' ? 'Change Admin PIN' : 'Enter Admin Mode';
      currentPinInput.hidden = mode === 'change';
      changeCurrentPinInput.hidden = mode !== 'change';
      newPinInput.hidden = mode !== 'change';
      confirmPinInput.hidden = mode !== 'change';
      modalMessage.textContent = '';
      [currentPinInput, changeCurrentPinInput, newPinInput, confirmPinInput].forEach(input => { input.value = ''; });
      modalBackdrop.classList.add('open');
      modalBackdrop.setAttribute('aria-hidden', 'false');
      (mode === 'change' ? changeCurrentPinInput : currentPinInput).focus();
    }
    function closePinModal() {
      modalBackdrop.classList.remove('open');
      modalBackdrop.setAttribute('aria-hidden', 'true');
    }
    function modalTemplate(kind) {
      if (kind === 'tailscale') return `<div class="tailscale-status-modal">
        <div class="status-section"><p class="status-text loading">LOADING...</p></div>
        <div class="config-section"><div class="current-tailnet"><span class="label">Current Tailnet:</span><span class="value" data-route-read="/api/status/tailscale/config">Loading...</span></div><input data-tailnet-input placeholder="Enter Tailnet name"><div class="button-row"><button data-modal-fetch="/api/status/tailscale/update-tailnet" data-method="POST">Update Tailnet</button><button data-modal-fetch="/api/status/tailscale/connect" data-method="POST">Connect</button><button data-modal-fetch="/api/status/tailscale/disconnect" data-method="POST">Disconnect</button><button data-modal-fetch="/api/status/tailscale/enable" data-method="POST">Enable Service</button><button data-modal-fetch="/api/status/tailscale/disable" data-method="POST">Disable Service</button></div></div>
        <div class="authkey-section"><input class="authkey-input" placeholder="Enter your tskey-auth-... or tskey-client-... key"><div class="button-row"><button data-modal-fetch="/api/status/tailscale/authkey" data-method="POST">Authenticate</button></div></div><pre class="readout action-output" data-modal-output></pre>
      </div>`;
      if (kind === 'internet') return `<div class="internet-status-modal"><div class="status-section"><p class="status-text loading">CHECKING...</p></div><div class="speed-test-section"><div class="button-row"><button data-modal-fetch="/api/status/internet/speedtest" data-method="POST">Run Speed Test</button></div><div class="speed-results"><p>Download: — Mbps</p><p>Upload: — Mbps</p><p>Latency: — ms</p></div></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'services') return `<div class="services-status-modal"><div class="loading-section">Loading service status...</div><ul class="service-status-list" data-route-read="/api/status/services"><li>No status data available</li></ul><div class="button-row"><button data-modal-fetch="/api/status/services">Refresh</button><button data-modal-fetch="/api/services/data">Service Data</button></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'openvpn') return `<div class="vpn-status-modal"><div class="status-section"><div class="service-statuses"><div class="status-item loading"><span>VPN Status:</span><span class="status-value">LOADING</span></div><div class="status-item loading"><span>Transmission Status:</span><span class="status-value">LOADING</span></div><div class="status-item"><span>Systemd Service:</span><span class="status-value">LOADING</span></div></div></div><div class="credentials-section"><div class="modal-grid"><div class="credential-group"><input placeholder="PIA Username"><input type="password" placeholder="PIA Password"><button data-modal-fetch="/api/status/vpn/updatekey/pia" data-method="POST">Create PIA Key</button></div><div class="credential-group"><input placeholder="Transmission Username"><input type="password" placeholder="Transmission Password"><button data-modal-fetch="/api/status/vpn/updatekey/transmission" data-method="POST">Create Transmission</button></div></div></div><div class="service-controls"><div class="button-row"><button data-modal-fetch="/api/status/vpn/enable" data-method="POST">Enable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/disable" data-method="POST">Disable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/pia/exists">PIA Key Exists</button><button data-modal-fetch="/api/status/vpn/transmission/exists">Transmission Key Exists</button></div></div><div class="restart-notice"><p>Note: Service changes require a restart to take effect.</p></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'power-meter') return `<div class="power-meter-modal"><div class="power-usage-display"><div class="power-value"><span class="power-value-number">—</span><span class="power-value-unit">Watts</span></div></div><div class="power-history-section"><div class="power-averages"><div class="power-average-row"><div class="power-average-label">5s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">30s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">60s average:</div><div class="power-average-value">—W</div></div></div></div><div class="button-row"><button data-modal-fetch="/api/status/power/usage">Refresh</button></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'theme') return `<div class="theme-modal"><p>Current theme: ${headerState.theme}.</p><p>Themes are loaded from /api/themes backed by static/themes/theme.json.</p></div>`;
      return '';
    }
    function wireModalFetches() {
      infoBody.querySelectorAll('[data-modal-fetch]').forEach(button => button.addEventListener('click', async () => {
        const output = infoBody.querySelector('[data-modal-output]');
        if (output) output.textContent = 'Loading ' + button.dataset.modalFetch + '…';
        try {
          const response = await fetch(button.dataset.modalFetch, { method: button.dataset.method || 'GET' });
          const text = await response.text();
          if (output) { try { output.textContent = JSON.stringify(JSON.parse(text), null, 2); } catch (_) { output.textContent = text; } }
        } catch (error) { if (output) output.textContent = 'fetch failed: ' + error; }
      }));
    }
    function openInfoModal(title, kind = 'status') {
      infoTitle.textContent = title;
      infoBody.innerHTML = modalTemplate(kind);
      themeChoiceRow.hidden = kind !== 'theme';
      infoBackdrop.classList.add('open');
      infoBackdrop.setAttribute('aria-hidden', 'false');
      wireModalFetches();
    }
    function closeInfoModal() {
      infoBackdrop.classList.remove('open');
      infoBackdrop.setAttribute('aria-hidden', 'true');
    }
    document.querySelector('[data-info-modal-close]')?.addEventListener('click', closeInfoModal);
    document.querySelectorAll('[data-indicator]').forEach(button => button.addEventListener('click', () => openInfoModal(button.dataset.modalTitle, button.dataset.modalKind)));
    themeButton?.addEventListener('click', () => {
      openInfoModal('Theme', 'theme');
    });
    async function loadThemeCatalog() {
      try {
        const catalog = await fetch('/api/themes').then(response => response.json());
        if (!catalog.themes || Object.keys(catalog.themes).length === 0) throw new Error('empty theme catalog');
        themeCatalog = catalog;
        themes = Object.keys(catalog.themes);
        renderThemeChoices();
        applyTheme();
      } catch (error) {
        console.error('theme catalog load failed', error);
        themes = [];
        ensureThemeStyleElement().textContent = '';
        if (themeButton) themeButton.title = 'Theme catalog unavailable: ' + error;
      }
    }
    adminButton?.addEventListener('click', () => {
      if (headerState.isAdmin) setAdminMode(false);
      else openPinModal('enter');
    });
    changePinButton?.addEventListener('click', () => {
      if (!headerState.isAdmin) { modalMessage.textContent = 'Must be in admin mode to change PIN'; return; }
      openPinModal('change');
    });
    document.querySelector('[data-pin-cancel]')?.addEventListener('click', closePinModal);
    document.querySelector('[data-pin-confirm-button]')?.addEventListener('click', async () => {
      if (modalMode === 'change' && (!changeCurrentPinInput.value || !newPinInput.value || !confirmPinInput.value)) { modalMessage.textContent = 'Please fill in all fields'; return; }
      if (modalMode === 'change' && newPinInput.value !== confirmPinInput.value) { modalMessage.textContent = 'New PINs do not match'; return; }
      if (modalMode === 'enter' && !currentPinInput.value) { modalMessage.textContent = 'Enter PIN'; return; }
      setAdminMode(true);
      modalMessage.textContent = modalMode === 'change' ? 'PIN changed successfully' : '';
      if (modalMode === 'enter') closePinModal();
    });
    loadThemeCatalog();
    setAdminMode(headerState.isAdmin);
    function visibleTabs() { return tabs.filter(tab => tab.dataset.visibility !== 'hidden' && tab.dataset.adminOnly !== 'true'); }
    function firstVisibleTab() { return visibleTabs()[0]?.dataset.pane || fallbackTab; }
    function setStarredTab(id) {
      tabState.starredTab = id;
      saveTabState(tabState);
      tabs.forEach(tab => {
        const starred = tab.dataset.pane === id;
        const button = tab.querySelector('[data-tab-star]');
        if (button) {
          button.classList.toggle('fas', starred);
          button.classList.toggle('far', !starred);
          button.setAttribute('aria-pressed', String(starred));
          button.title = starred ? tab.querySelector('.tab-name').textContent + ' tab is starred' : 'Star ' + tab.querySelector('.tab-name').textContent + ' tab';
        }
      });
    }
    function applyVisibilityState() {
      tabs.forEach(tab => {
        const hidden = tabState.hiddenTabs.includes(tab.dataset.pane);
        tab.dataset.visibility = hidden ? 'hidden' : 'visible';
        const button = tab.querySelector('[data-tab-visibility-toggle]');
        if (button) {
          button.dataset.visible = String(!hidden);
          button.title = (hidden ? 'Show ' : 'Hide ') + tab.querySelector('.tab-name').textContent + ' tab';
          button.setAttribute('aria-label', button.title);
          button.querySelector('.eye-icon').textContent = hidden ? '🙈' : '👁';
        }
      });
    }
    function showPane(id) {
      const selected = panes.some(pane => pane.dataset.panePanel === id) ? id : firstVisibleTab();
      tabs.forEach(tab => {
        const active = tab.dataset.pane === selected;
        tab.setAttribute('aria-selected', String(active));
        tab.classList.toggle('active', active);
      });
      panes.forEach(pane => pane.classList.toggle('active', pane.dataset.panePanel === selected));
      if (location.hash !== '#' + selected) history.replaceState(null, '', '#' + selected);
    }
    tabs.forEach(tab => {
      tab.addEventListener('click', event => {
        if (event.target.closest('button')) return;
        event.preventDefault();
        if (tab.dataset.visibility !== 'hidden') showPane(tab.dataset.pane);
      });
      tab.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); showPane(tab.dataset.pane); } });
    });
    document.querySelectorAll('[data-tab-star]').forEach(button => button.addEventListener('click', event => { event.stopPropagation(); setStarredTab(button.dataset.tabStar); }));
    document.querySelectorAll('[data-tab-visibility-toggle]').forEach(button => button.addEventListener('click', event => {
      event.stopPropagation();
      const id = button.dataset.tabVisibilityToggle;
      const hidden = tabState.hiddenTabs.includes(id);
      tabState.hiddenTabs = hidden ? tabState.hiddenTabs.filter(tab => tab !== id) : [...new Set([...tabState.hiddenTabs, id])];
      saveTabState(tabState);
      applyVisibilityState();
      if (!hidden && tabState.starredTab === id) setStarredTab(firstVisibleTab());
      const active = tabs.find(tab => tab.getAttribute('aria-selected') === 'true');
      if (!active || active.dataset.visibility === 'hidden') showPane(firstVisibleTab());
    }));
    applyVisibilityState();
    setStarredTab(tabState.starredTab);
    async function fetchInto(route, target, method = 'GET') {
      const el = document.getElementById(target);
      if (!el) return;
      el.textContent = 'Loading ' + route + '…';
      try {
        const response = await fetch(route, { method });
        const text = await response.text();
        try { el.textContent = JSON.stringify(JSON.parse(text), null, 2); }
        catch (_) { el.textContent = text; }
      } catch (error) { el.textContent = 'fetch failed: ' + error; }
    }
    document.querySelectorAll('[data-fetch]').forEach(button => button.addEventListener('click', () => fetchInto(button.dataset.fetch, button.dataset.target, button.dataset.method || 'GET')));

    const uploadForm = document.querySelector('[data-upload-form]');
    uploadForm?.addEventListener('submit', async event => {
      event.preventDefault();
      const file = document.querySelector('[data-upload-file]')?.files?.[0];
      const destination = document.querySelector('[data-upload-path]')?.value || '/mnt/nas';
      const out = document.getElementById('upload-readout');
      if (!file) { out.textContent = 'Choose a file first.'; return; }
      const form = new FormData();
      form.append('file', file);
      form.append('path', destination);
      out.textContent = 'Uploading ' + file.name + ' through Caduceus…';
      try {
        const response = await fetch('/api/files/upload', { method: 'POST', body: form });
        const text = await response.text();
        try { out.textContent = JSON.stringify(JSON.parse(text), null, 2); }
        catch (_) { out.textContent = text; }
      } catch (error) { out.textContent = 'upload failed: ' + error; }
    });

    async function hydrateUptime() {
      const uptime = document.querySelector('[data-uptime-indicator]');
      if (!uptime) return;
      try {
        const data = await fetch('/uptime').then(r => r.json()).catch(() => null);
        uptime.textContent = data?.uptime ? data.uptime + 's' : 'connecting...';
      } catch (_) { uptime.textContent = navigator.onLine ? 'connecting...' : 'disconnected'; }
    }
    async function hydrateStats() {
      try {
        const data = await fetch('/api/stats').then(r => r.json());
        const caduceus = await fetch('/api/caduceus/status').then(r => r.json()).catch(() => null);
        document.getElementById('stats-load').textContent = caduceus?.ok ? 'connected' : (data.telemetry?.load1 ?? 'unwired');
        document.getElementById('stats-stream').textContent = (data.transport?.streamStatus || 'unknown') + ' — ' + (data.transport?.streamReason || '');
        document.getElementById('stats-missing').textContent = caduceus?.firstMissingSignal || data.telemetry?.firstMissingSignal || 'none';
        document.getElementById('stats-readout').textContent = JSON.stringify({ stats: data, caduceus }, null, 2);
      } catch (error) { document.getElementById('stats-readout').textContent = String(error); }
    }
    showPane((location.hash || '#' + (tabState.starredTab || firstVisibleTab())).slice(1));
    hydrateUptime();
    hydrateStats();
  </script>
</body>
</html>"###;
    shell.replace("__NAV__", &nav)
}

fn is_safe_tab_id(tab_id: &str) -> bool {
    !tab_id.is_empty()
        && tab_id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

