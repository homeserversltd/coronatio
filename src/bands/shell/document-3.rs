fn shell_document_3() -> &'static str {
    r####"        button.dataset.themeChoice = name;
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
        themeButton.title = 'Current theme: ' + label + '. Click to switch theme.';
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
      document.querySelectorAll('[data-admin-only]:not([data-admin-only="false"])').forEach(el => {
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
    function indicatorAdminSection(inner) {
      return headerState.isAdmin ? `<div data-admin-only data-admin-surface="indicator-modal" data-admin-enhanced="true">${inner}</div>` : '';
    }
    function modalTemplate(kind) {
      if (kind === 'tailscale') return `<div class="tailscale-status-modal" data-modal-kind-body="tailscale">
        <div class="status-section"><p class="status-text loading" data-modal-status data-route-read="/api/status/tailscale">Loading Tailscale status…</p></div>
        ${indicatorAdminSection(`<div class="config-section"><div class="current-tailnet"><span class="label">Current Tailnet:</span><span class="value" data-route-read="/api/status/tailscale/config">Loading...</span></div><input data-tailnet-input placeholder="Enter Tailnet name"><div class="button-row"><button data-modal-fetch="/api/status/tailscale/update-tailnet" data-method="POST">Update Tailnet</button><button data-modal-fetch="/api/status/tailscale/connect" data-method="POST">Connect</button><button data-modal-fetch="/api/status/tailscale/disconnect" data-method="POST">Disconnect</button><button data-modal-fetch="/api/status/tailscale/enable" data-method="POST">Enable Service</button><button data-modal-fetch="/api/status/tailscale/disable" data-method="POST">Disable Service</button></div></div>
        <div class="authkey-section"><input class="authkey-input" placeholder="Enter your tskey-auth-... or tskey-client-... key"><div class="button-row"><button data-modal-fetch="/api/status/tailscale/authkey" data-method="POST">Authenticate</button></div></div>`)}<pre class="readout action-output" data-modal-output></pre>
      </div>`;
      if (kind === 'internet') return `<div class="internet-status-modal" data-modal-kind-body="internet"><div class="status-section"><p class="status-text loading" data-modal-status data-route-read="/api/status">Checking internet status…</p></div>${indicatorAdminSection(`<div class="admin-details-section" data-admin-details-section><div class="ip-details"><p><strong>Location:</strong> —</p><p><strong>ISP:</strong> —</p><p><strong>Timezone:</strong> —</p></div></div><div class="speed-test-section"><div class="button-row"><button data-modal-fetch="/api/status/internet/speedtest" data-method="POST">Run Speed Test</button></div><div class="speed-results"><p>Download: — Mbps</p><p>Upload: — Mbps</p><p>Latency: — ms</p></div></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'services') return `<div class="services-status-modal" data-modal-kind-body="services"><div class="loading-section" data-modal-status data-route-read="/api/status/services">Loading service status…</div><ul class="service-status-list" data-route-read="/api/status/services"><li>No status data available</li></ul>${indicatorAdminSection(`<div class="admin-service-grid"><div class="admin-service-description">Description</div><div class="admin-service-name">Service</div><div class="admin-service-right"><span class="admin-service-status">enabled</span></div></div><div class="button-row"><button data-modal-fetch="/api/status/services">Refresh</button><button data-modal-fetch="/api/services/data">Service Data</button></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'openvpn') return `<div class="vpn-status-modal" data-modal-kind-body="openvpn"><div class="status-section"><div class="service-statuses"><div class="status-item loading"><span>VPN Status:</span><span class="status-value" data-modal-status data-route-read="/api/status/vpn/pia">Loading VPN…</span></div><div class="status-item loading"><span>Transmission Status:</span><span class="status-value" data-modal-secondary-status data-route-read="/api/status/vpn/transmission">Loading Transmission…</span></div>${headerState.isAdmin ? `<div class="status-item" data-admin-only data-admin-surface="indicator-modal"><span>Systemd Service:</span><span class="status-value">LOADING</span></div>` : ''}</div></div>${indicatorAdminSection(`<div class="credentials-section"><div class="modal-grid"><div class="credential-group"><input placeholder="PIA Username"><input type="password" placeholder="PIA Password"><button data-modal-fetch="/api/status/vpn/updatekey/pia" data-method="POST">Create PIA Key</button></div><div class="credential-group"><input placeholder="Transmission Username"><input type="password" placeholder="Transmission Password"><button data-modal-fetch="/api/status/vpn/updatekey/transmission" data-method="POST">Create Transmission</button></div></div></div><div class="service-controls"><div class="button-row"><button data-modal-fetch="/api/status/vpn/enable" data-method="POST">Enable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/disable" data-method="POST">Disable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/pia/exists">PIA Key Exists</button><button data-modal-fetch="/api/status/vpn/transmission/exists">Transmission Key Exists</button></div></div><div class="restart-notice"><p>Note: Service changes require a restart to take effect.</p></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'power-meter') return `<div class="power-meter-modal" data-modal-kind-body="power-meter"><div class="power-usage-display"><div class="power-value" data-route-read="/api/status/power/usage"><span class="power-value-number" data-modal-status>Loading power…</span><span class="power-value-unit">Watts</span></div></div><div class="power-history-section"><div class="power-averages"><div class="power-average-row"><div class="power-average-label">5s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">30s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">60s average:</div><div class="power-average-value">—W</div></div></div></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'theme') return `<div class="theme-modal"><p>Current theme: ${headerState.theme}.</p><p>Themes are loaded from /api/themes backed by static/themes/theme.json.</p></div>`;
      return '';
    }
    function routeReadLabel(route, data) {
      const ok = data && (data.ok === true || data.success === true);
      const status = data?.status || (ok ? 'ok' : 'unavailable');
      const missing = data?.firstMissingSignal && data.firstMissingSignal !== 'none' ? ' · ' + data.firstMissingSignal : '';
      if (route.includes('tailscale')) return ok ? 'Tailscale status: ' + status + missing : 'Tailscale status unavailable';
      if (route.includes('/api/status/vpn/pia')) return ok ? 'VPN status: ' + status + missing : 'VPN status unavailable';
      if (route.includes('/api/status/vpn/transmission')) return ok ? 'Transmission status: ' + status + missing : 'Transmission status unavailable';
      if (route.includes('services')) return ok ? 'Services status: ' + status + missing : 'Services status unavailable';
      if (route.includes('power')) return ok ? 'Power readback: ' + status + missing : 'Power readback unavailable';
      return ok ? 'Internet status: ' + status + missing : 'Internet status unavailable';
    }
    async function hydrateModalRouteReads(kind) {
      const nodes = [...infoBody.querySelectorAll('[data-route-read]')];
      await Promise.all(nodes.map(async node => {
        const route = node.dataset.routeRead;
        try {
          const response = await fetch(route, { cache: 'no-store' });
          const data = await response.json();
          const label = routeReadLabel(route, data);
          if (node.matches('ul')) node.innerHTML = `<li>${label}</li>`;
          else if (node.classList.contains('power-value')) node.querySelector('[data-modal-status]').textContent = label.replace('Power readback: ', '').replace('Power readback unavailable', 'unavailable');
          else node.textContent = label;
          node.classList.remove('loading');
          node.dataset.hydrated = 'true';
        } catch (error) {
          const fallback = 'Status unavailable: ' + route;
          if (node.matches('ul')) node.innerHTML = `<li>${fallback}</li>`;
          else if (node.classList.contains('power-value')) node.querySelector('[data-modal-status]').textContent = 'unavailable';
          else node.textContent = fallback;
          node.classList.remove('loading');
          node.dataset.hydrated = 'false';
        }
      }));
    }
    function wireModalFetches() {
      infoBody.querySelectorAll('[data-modal-fetch]').forEach(button => button.addEventListener('click', async () => {
        const output = infoBody.querySelector('[data-modal-output]');
        if (!headerState.isAdmin && button.closest('[data-admin-only]')) { if (output) output.textContent = 'Enter Admin Mode'; return; }
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
      hydrateModalRouteReads(kind);
    }
    function closeInfoModal() {
      infoBackdrop.classList.remove('open');
      infoBackdrop.setAttribute('aria-hidden', 'true');
    }
    function closeModalOnOutsideClick(backdrop, closeModal) {
      backdrop?.addEventListener('click', event => {
        if (event.target === event.currentTarget) closeModal();
      });
    }
    closeModalOnOutsideClick(modalBackdrop, closePinModal);
    closeModalOnOutsideClick(infoBackdrop, closeInfoModal);
    document.querySelector('[data-info-modal-close]')?.addEventListener('click', closeInfoModal);
    document.querySelectorAll('[data-indicator]').forEach(button => button.addEventListener('click', () => openInfoModal(button.dataset.modalTitle, button.dataset.modalKind)));
    function cycleTheme() {
      if (!themes.length) return;
      const currentIndex = Math.max(0, themes.indexOf(headerState.theme));
      headerState.theme = themes[(currentIndex + 1) % themes.length];
      saveHeaderState();
      applyTheme();
    }
    themeButton?.addEventListener('click', cycleTheme);
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
    document.querySelectorAll('[data-tab-star]').forEach(button => button.addEventListener('click', async event => {
      event.stopPropagation();
      setStarredTab(button.dataset.tabStar);
      try {
        await fetch('/api/set_starred_tab', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ tabName: button.dataset.tabStar }) });
      } catch (_) {}
    }));
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
      const el = document.getElementById(target);"####
}
