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
      --background: #1a1a1a;
      --surface: #2a2a2a;
      --surface-soft: #222;
      --text: #ffffff;
      --text-secondary: #dddddd;
      --accent: #00f2fe;
      --accent-soft: rgba(0, 242, 254, .16);
      --primary: #4CAF50;
      --border: rgba(255,255,255,.14);
      --error: #f44336;
      --warning: #ff9800;
      --success: #4CAF50;
      --shadow: 0 2px 4px rgba(0,0,0,.35);
      font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
    .brand { display: flex; align-items: center; gap: .65rem; font-weight: 700; letter-spacing: .02em; }
    .brand-mark { width: 26px; height: 26px; border-radius: 7px; border: 2px solid var(--accent); display: grid; place-items: center; color: var(--accent); font-size: .82rem; }
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
    @media (max-width: 760px) {
      .top-bar { align-items: flex-start; flex-direction: column; padding: .75rem 1rem; }
      .tab-bar, .content { padding-left: 12px; padding-right: 12px; }
      .portal-grid, .pane-grid { grid-template-columns: 1fr; }
    }
  </style>
</head>
<body>
  <main class="app" data-product="Coronatio" data-source-material="homeserver-main-site">
    <header class="top-bar">
      <div class="brand"><span class="brand-mark">⌂</span><span>HomeServer</span><span class="muted">/ Coronatio</span></div>
      <div class="status-strip" aria-label="Coronatio currentness">
        <span class="status-pill ok">Rust crown online</span>
        <span class="status-pill">Caduceus boundary protected</span>
        <span class="status-pill">Source quarry: main HomeServer</span>
      </div>
    </header>
    <nav class="tab-bar" aria-label="Coronatio primary tabs" role="tablist" data-admin-mode="true" data-hidden="false">__NAV__</nav>
    <section class="content">
      <section class="pane active" id="pane-admin" data-pane-panel="admin" role="tabpanel" aria-label="Admin">
        <div class="pane-grid">
          <article class="card"><h2>Admin authority</h2><p>Caduceus is the live privileged membrane; Coronatio reads and triggers it through same-origin receipts.</p><div class="button-row"><button data-fetch="/api/caduceus/status" data-target="admin-session">Read live status</button><button class="secondary" data-fetch="/api/caduceus/update/check" data-target="admin-installer" data-method="POST">Check machine</button></div><pre class="readout" id="admin-session">Waiting for Caduceus status.</pre></article>
          <article class="card"><h2>Caduceus membrane</h2><p>Host mutation now enters Caduceus and Harmonia; policy still gates each command before root work runs.</p><div class="button-row"><button data-fetch="/api/caduceus/update/now" data-target="admin-installer" data-method="POST">Make harmonious</button><button class="secondary" data-fetch="/api/caduceus/receipts/latest" data-target="admin-installer">Latest receipt</button></div><pre class="readout" id="admin-installer">Harmonia receipt readback will appear here.</pre></article>
          <article class="card"><h2>Visible contract</h2><p><span class="success">On:</span> Caduceus/Harmonia routes are live; any blocked action names the first missing signal instead of pretending completion.</p></article>
        </div>
      </section>
      <section class="pane" id="pane-stats" data-pane-panel="stats" role="tabpanel" aria-label="Stats">
        <div class="pane-grid">
          <article class="card"><h2>System telemetry</h2><div class="metric" id="stats-load">—</div><p>Load average</p><pre class="readout" id="stats-readout">Fetching /api/stats…</pre></article>
          <article class="card"><h2>Stream lane</h2><p id="stats-stream">Stats stream state pending.</p><div class="button-row"><button data-fetch="/api/stats/events" data-target="stats-event">Read event frame</button><button class="secondary" data-fetch="/api/stats/events/renew" data-target="stats-event" data-method="POST">Renew lease</button></div><pre class="readout" id="stats-event">No event readback yet.</pre></article>
          <article class="card"><h2>Missing signal</h2><p id="stats-missing" class="warning">Checking collector status…</p></article>
        </div>
      </section>
      <section class="pane" id="pane-portals" data-pane-panel="portals" role="tabpanel" aria-label="Portals">
        <div class="portal-grid">
          <article class="card portal-card"><div><h2>Admitted services</h2><p>Portal cards follow the main HomeServer service-grid pattern and expose the live config contract.</p></div><div class="button-row"><button data-fetch="/api/services/data" data-target="portals-readout">Read service contract</button><a class="action-link secondary" href="https://home.arpa/">Open main HomeServer</a></div></article>
          <article class="card portal-card"><div><h2>Coronatio</h2><p>Rust crown preview, port 3013.</p></div><span class="status-pill ok">online</span></article>
          <article class="card portal-card"><div><h2>Caduceus</h2><p>Privileged actuator membrane, port 3014.</p></div><div class="button-row"><button data-fetch="/api/caduceus/status" data-target="portals-readout">Status</button><a class="action-link secondary" href="http://home.arpa:3014/health">Health</a></div></article>
        </div>
        <pre class="readout" id="portals-readout">Service contract readback will appear here.</pre>
      </section>
      <section class="pane" id="pane-upload" data-pane-panel="upload" role="tabpanel" aria-label="Upload">
        <div class="pane-grid">
          <article class="card"><h2>Safe file ingress</h2><div class="drop-zone"><strong>Upload lane staged</strong><p>Files will enter through policy, receipt, and Caduceus-backed mutation. The native pane shows the product job instead of an empty tab.</p></div><div class="button-row"><button data-fetch="/api/panes/upload" data-target="upload-readout">Read upload pane</button></div></article>
          <article class="card"><h2>Boundary</h2><p class="warning">Live file mutation is not enabled until the Caduceus actuator and receipt ledger are wired.</p><pre class="readout" id="upload-readout">Waiting for upload pane readback.</pre></article>
        </div>
      </section>
    </section>
  </main>
  <script>
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
    async function hydrateStats() {
      try {
        const data = await fetch('/api/stats').then(r => r.json());
        const caduceus = await fetch('/api/caduceus/status').then(r => r.json()).catch(() => null);
        document.getElementById('stats-load').textContent = caduceus?.ok ? 'live' : (data.telemetry?.load1 ?? 'unwired');
        document.getElementById('stats-stream').textContent = (data.transport?.streamStatus || 'unknown') + ' — ' + (data.transport?.streamReason || '');
        document.getElementById('stats-missing').textContent = caduceus?.firstMissingSignal || data.telemetry?.firstMissingSignal || 'none';
        document.getElementById('stats-readout').textContent = JSON.stringify({ stats: data, caduceus }, null, 2);
      } catch (error) { document.getElementById('stats-readout').textContent = String(error); }
    }
    showPane((location.hash || '#' + (tabState.starredTab || firstVisibleTab())).slice(1));
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

