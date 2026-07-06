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
      if (!themes.includes(headerState.theme)) headerState.theme = themeCatalog.default || themes[0] || 'light';
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
      const wasAdmin = headerState.isAdmin;
      const previousActive = currentActiveTabId();
      headerState.isAdmin = Boolean(value);
      saveHeaderState();
      if (adminButton) {
        adminButton.dataset.adminState = headerState.isAdmin ? 'logged-in' : 'logged-out';
        adminButton.textContent = headerState.isAdmin ? 'Exit Admin Mode' : 'Enter Admin Mode';
      }
      if (appRoot) appRoot.dataset.adminMode = headerState.isAdmin ? 'true' : 'false';
      if (!headerState.isAdmin) {
        const token = localStorage.getItem('coronatioAdminToken');
        if (token) fetch('/api/logout', { method: 'POST', headers: { 'X-Admin-Token': token } }).catch(() => {});
        localStorage.removeItem('coronatioAdminToken');
      }
      if (tabBar) tabBar.dataset.adminMode = headerState.isAdmin ? 'true' : 'false';
      document.querySelectorAll('[data-admin-only]:not([data-admin-only="false"])').forEach(el => {
        el.hidden = !headerState.isAdmin;
        el.setAttribute('aria-hidden', String(!headerState.isAdmin));
      });
      if (changePinButton) changePinButton.hidden = !headerState.isAdmin;
      refreshTabBar(previousActive).then(selectedTab => {
        applyTabBarVisibility();
        if (selectedTab) showPane(selectedTab);
      });
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
      if (kind === 'tailscale') return `<div class="tailscale-status-modal" data-modal-kind-body="tailscale" data-flask-react-quarry="TailscaleIndicator">
        <div class="status-section"><p class="status-text loading" data-modal-status data-route-read="/api/status/tailscale"><span data-spinner>⟳</span> LOADING...</p>
          <div class="login-required-section" data-tailscale-login-section hidden>
            <div class="login-message"><strong>Authentication Required</strong><p>Tailscale service is running but needs authentication. Click the link below to complete login:</p></div>
            <div class="login-url-container"><a href="#" target="_blank" rel="noopener noreferrer" class="login-url-link" data-tailscale-login-url></a><button class="copy-url-button" data-copy-login-url title="Copy URL to clipboard">Copy URL</button></div>
            <div class="login-instructions"><p><strong>Instructions:</strong></p><ol><li>Click the authentication link above (opens in new tab)</li><li>Sign in to your Tailscale account</li><li>Authorize this device</li><li>Return here - the status should update automatically</li></ol></div>
          </div>
        </div>
        ${indicatorAdminSection(`<div class="controls-section"><div class="connection-buttons"><button class="primary-button" data-modal-fetch="/api/status/tailscale/connect" data-method="POST" data-operation-label="Connecting...">Connect</button><button class="primary-button" data-modal-fetch="/api/status/tailscale/disconnect" data-method="POST" data-operation-label="Disconnecting...">Disconnect</button></div><div class="service-controls"><button class="primary-button" data-modal-fetch="/api/status/tailscale/enable" data-method="POST" data-operation-label="Enabling...">Enable Service</button><button class="primary-button" data-modal-fetch="/api/status/tailscale/disable" data-method="POST" data-operation-label="Disabling...">Disable Service</button></div></div>
        <div class="config-section"><div class="current-tailnet"><span class="label">Current Tailnet:</span><span class="value" data-route-read="/api/status/tailscale/config">Loading...</span></div><div class="config-form"><input data-tailnet-input placeholder="Enter Tailnet name"><button class="primary-button" data-modal-fetch="/api/status/tailscale/update-tailnet" data-method="POST" data-operation-label="Updating...">Update Tailnet</button><div class="tooltip-text">Unique name used for DNS entries and TLS certificates.
          You can find this name on the DNS page of your tailscale dashboard.
          This change will reboot the website and tailscale service. 
          Please wait and refresh the page after submitting changes.

          Note: HOMESERVER will automatically regenerate the HTTPS self-signed
          certificate to reference your new tailnet. If you previously
          installed the certificate on any device, open the site in a
          private/incognito window and re-download the certificate before
          returning to normal browsing. Until the new certificate is
          installed, browsers may report a certificate name mismatch for both
          local and remote access.</div></div></div>
        <div class="authkey-section"><div class="authkey-alternative"><p class="alternative-text"><strong>Alternative:</strong> If the login link isn't working, you can use an auth key instead.</p></div><div class="authkey-form"><input class="authkey-input" data-authkey-input placeholder="Enter your tskey-auth-... or tskey-client-... key"><button class="primary-button" data-modal-fetch="/api/status/tailscale/authkey" data-method="POST" data-operation-label="Authenticating...">Authenticate</button></div><div class="authkey-help"><p>Get your auth key from the Tailscale admin console under Settings → Keys.</p></div></div>`)}<pre class="readout action-output" data-modal-output></pre>
      </div>`;
      if (kind === 'internet') return `<div class="internet-status-modal" data-modal-kind-body="internet"><div class="status-section"><p class="status-text ${internetState.status}" data-modal-status data-route-read="/api/status">${internetStatusModalText()}</p></div>${indicatorAdminSection(`${internetAdminDetailsHtml()}<div class="speed-test-section"><div class="button-row"><button class="primary-button" data-speed-test-button data-modal-fetch="/api/status/internet/speedtest" data-method="POST" ${internetState.isSpeedTesting || internetState.status === 'loading' ? 'disabled' : ''}>${internetState.isSpeedTesting ? 'Running Speed Test...' : 'Run Speed Test'}</button></div>${internetSpeedTestHtml()}</div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'services') return `<div class="services-status-modal" data-modal-kind-body="services"><div class="loading-section" data-modal-status data-route-read="/api/status/services">Loading service status…</div><ul class="service-status-list" data-route-read="/api/status/services"><li>No status data available</li></ul>${indicatorAdminSection(`<div class="admin-service-grid"><div class="admin-service-description">Description</div><div class="admin-service-name">Service</div><div class="admin-service-right"><span class="admin-service-status">enabled</span></div></div><div class="button-row"><button data-modal-fetch="/api/status/services">Refresh</button><button data-modal-fetch="/api/services/data">Service Data</button></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'openvpn') return `<div class="vpn-status-modal" data-modal-kind-body="openvpn"><div class="status-section"><div class="service-statuses"><div class="status-item loading"><span>VPN Status:</span><span class="status-value" data-modal-status data-route-read="/api/status/vpn/pia">Loading VPN…</span></div><div class="status-item loading"><span>Transmission Status:</span><span class="status-value" data-modal-secondary-status data-route-read="/api/status/vpn/transmission">Loading Transmission…</span></div>${headerState.isAdmin ? `<div class="status-item" data-admin-only data-admin-surface="indicator-modal"><span>Systemd Service:</span><span class="status-value">LOADING</span></div>` : ''}</div></div>${indicatorAdminSection(`<div class="credentials-section"><div class="modal-grid"><div class="credential-group"><input placeholder="PIA Username"><input type="password" placeholder="PIA Password"><button data-modal-fetch="/api/status/vpn/updatekey/pia" data-method="POST">Create PIA Key</button></div><div class="credential-group"><input placeholder="Transmission Username"><input type="password" placeholder="Transmission Password"><button data-modal-fetch="/api/status/vpn/updatekey/transmission" data-method="POST">Create Transmission</button></div></div></div><div class="service-controls"><div class="button-row"><button data-modal-fetch="/api/status/vpn/enable" data-method="POST">Enable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/disable" data-method="POST">Disable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/pia/exists">PIA Key Exists</button><button data-modal-fetch="/api/status/vpn/transmission/exists">Transmission Key Exists</button></div></div><div class="restart-notice"><p>Note: Service changes require a restart to take effect.</p></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'power-meter') return `<div class="power-meter-modal" data-modal-kind-body="power-meter"><div class="power-usage-display"><div class="power-value" data-route-read="/api/status/power/usage"><span class="power-value-number" data-modal-status>Loading power…</span><span class="power-value-unit">Watts</span></div></div><div class="power-history-section"><div class="power-averages"><div class="power-average-row"><div class="power-average-label">5s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">30s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">60s average:</div><div class="power-average-value">—W</div></div></div></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'theme') return `<div class="theme-modal"><p>Current theme: ${headerState.theme}.</p><p>Theme selection comes from homeserver.json global.theme.name through /api/themes.</p></div>`;
      return '';
    }
    function internetStatusModalText() {
      if (internetState.status === 'loading') return 'CHECKING...';
      const base = String(internetState.status || 'loading').toUpperCase();
      return headerState.isAdmin && internetState.publicIp ? `${base} (${internetState.publicIp})` : base;
    }
    function internetAdminDetailsHtml() {
      if (!headerState.isAdmin || !internetState.ipDetails) return '';
      const details = internetState.ipDetails;
      const rows = [];
      if (details.city && details.region) rows.push(`<p><strong>Location:</strong> ${details.city}, ${details.region}, ${details.country || ''}</p>`);
      if (details.org) rows.push(`<p><strong>ISP:</strong> ${details.org}</p>`);
      if (details.timezone) rows.push(`<p><strong>Timezone:</strong> ${details.timezone}</p>`);
      return rows.length ? `<div class="admin-details-section" data-admin-details-section><div class="ip-details">${rows.join('')}</div></div>` : '';
    }
    function internetSpeedTestHtml() {
      if (internetState.speedTestError) return `<div class="error-message">${internetState.speedTestError}</div>`;
      const result = internetState.speedTestResults;
      if (!result) return '';
      return `<div class="speed-results"><p>Download: ${result.download} Mbps</p><p>Upload: ${result.upload} Mbps</p><p>Latency: ${result.latency} ms</p></div>`;
    }
    function setInternetIndicatorState(data) {
      if (!data) return;
      internetState.status = data.status || 'loading';
      internetState.publicIp = data.publicIp;
      internetState.ipDetails = data.ipDetails;
      if (!internetIndicator) return;
      internetIndicator.classList.remove('ok', 'warn', 'error', 'loading');
      if (internetState.status === 'connected') internetIndicator.classList.add('ok');
      else if (internetState.status === 'disconnected' || internetState.status === 'error') internetIndicator.classList.add('error');
      else internetIndicator.classList.add('loading');
      const title = internetState.status === 'loading' ? 'Checking internet connection...' : (headerState.isAdmin && internetState.publicIp ? `Internet: ${internetState.status} (${internetState.publicIp})` : `Internet: ${internetState.status}`);
      internetIndicator.title = title;
      internetIndicator.setAttribute('aria-label', internetState.status === 'loading' ? 'Checking Internet Status' : 'Internet Status');
    }
    function routeReadLabel(route, data) {
      const ok = data && (data.ok === true || data.success === true);
      const status = data?.status || (ok ? 'ok' : 'unavailable');
      const missing = data?.firstMissingSignal && data.firstMissingSignal !== 'none' ? ' · ' + data.firstMissingSignal : '';
      if (route.includes('/api/status/tailscale/config')) return data?.tailnet || data?.tailnetName || data?.readback?.tailnet || (ok ? 'Loading...' + missing : 'Loading...');
      if (route === '/api/status') {
        setInternetIndicatorState(data);
        return internetStatusModalText();
      }
      if (route.includes('tailscale')) return ok ? 'Tailscale status: ' + status + missing : 'Tailscale status unavailable';
      if (route.includes('/api/status/vpn/pia')) return ok ? 'VPN status: ' + status + missing : 'VPN status unavailable';
      if (route.includes('/api/status/vpn/transmission')) return ok ? 'Transmission status: ' + status + missing : 'Transmission status unavailable';
      if (route.includes('services')) return ok ? 'Services status: ' + status + missing : 'Services status unavailable';
      if (route.includes('power')) return ok && typeof data?.current === 'number' ? formatPowerWatts(data.current) : (ok ? 'Power readback: ' + status + missing : 'Power readback unavailable');
      return ok ? 'Internet status: ' + status + missing : 'Internet status unavailable';
    }
    const POWER_DISPLAY_FACTOR = 1.6;
    function formatPowerWatts(watts) {
      const value = Number(watts);
      return Number.isFinite(value) ? (value * POWER_DISPLAY_FACTOR).toFixed(1) : 'unavailable';
    }
    function hydratePowerIndicator(data) {
      const button = document.querySelector('[data-indicator="power-meter"]');
      const number = button?.querySelector('.power-value-small-number');
      if (!button || !number) return;
      if (data?.ok && typeof data.current === 'number') {
        const display = formatPowerWatts(data.current);
        number.textContent = display;
        button.classList.remove('ok', 'warn', 'error');
        button.title = 'Power: ' + display + 'W';
        button.setAttribute('aria-label', 'Power Usage ' + display + ' Watts');
      } else {
        number.textContent = '—';
        button.classList.remove('ok', 'warn', 'error');
        button.title = 'Power readback unavailable';
        button.setAttribute('aria-label', 'Power readback unavailable');
      }
    }
    async function refreshPowerIndicator() {
      try {
        const response = await fetch('/api/status/power/usage', { cache: 'no-store' });
        hydratePowerIndicator(await response.json());
      } catch (error) {
        hydratePowerIndicator(null);
      }
    }
    function tailscaleStatusClass(data) {
      if (!data || data.status === 'loading') return 'loading';
      if (data.status === 'connected') return 'connected';
      if (data.status === 'disconnected' && data.loginUrl) return 'disconnected login-required';
      if (data.status === 'disconnected') return 'disconnected';
      if (data.status === 'error') return 'error';
      return data.status || 'unknown';
    }
    function hydrateTailscaleModal(data) {
      const statusNode = infoBody.querySelector('[data-modal-kind-body="tailscale"] [data-modal-status]');
      if (!statusNode) return;
      const state = data?.status || (data?.ok ? 'rust-route' : 'loading');
      statusNode.className = 'status-text ' + tailscaleStatusClass(data);
      statusNode.textContent = state === 'loading' ? 'LOADING...' : String(state).toUpperCase() + (headerState.isAdmin && data?.ip ? ' (' + data.ip + ')' : '');
      const login = data?.loginUrl || data?.authUrl || data?.url;
      const loginSection = infoBody.querySelector('[data-tailscale-login-section]');
      const loginLink = infoBody.querySelector('[data-tailscale-login-url]');
      const showLogin = Boolean(headerState.isAdmin && login && state === 'disconnected');
      if (loginSection) loginSection.hidden = !showLogin;
      if (loginLink && login) { loginLink.href = login; loginLink.textContent = login; }
      infoBody.querySelector('[data-copy-login-url]')?.addEventListener('click', () => navigator.clipboard?.writeText(loginLink?.href || ''));
      const input = infoBody.querySelector('[data-tailnet-input]');
      if (input && data?.tailnet && !input.value) input.value = data.tailnet;
    }
    function modalRequestBody(button) {
      const route = button.dataset.modalFetch || '';
      if (route.endsWith('/update-tailnet')) return JSON.stringify({ tailnetName: infoBody.querySelector('[data-tailnet-input]')?.value || '' });
      if (route.endsWith('/authkey')) return JSON.stringify({ authKey: infoBody.querySelector('[data-authkey-input]')?.value || '' });
      return undefined;
    }
    async function hydrateInternetIndicator() {
      try {
        const response = await fetch('/api/status', { cache: 'no-store' });
        const data = await response.json();
        setInternetIndicatorState(data);
      } catch (_) {
        setInternetIndicatorState({ status: 'disconnected' });
      }
    }
    async function hydrateModalRouteReads(kind) {
      const nodes = [...infoBody.querySelectorAll('[data-route-read]')];
      await Promise.all(nodes.map(async node => {
        const route = node.dataset.routeRead;
        try {
          const response = await fetch(route, { cache: 'no-store' });
          const data = await response.json();
          if (kind === 'tailscale' && route === '/api/status/tailscale') hydrateTailscaleModal(data);
          const label = routeReadLabel(route, data);
          if (node.matches('ul')) node.innerHTML = `<li>${label}</li>`;
          else if (node.classList.contains('power-value')) node.querySelector('[data-modal-status]').textContent = label.replace('Power readback: ', '').replace('Power readback unavailable', 'unavailable');
          else if (!(kind === 'tailscale' && route === '/api/status/tailscale')) node.textContent = label;
          if (route === '/api/status') {
            node.classList.remove('loading', 'connected', 'disconnected', 'error');
            node.classList.add(internetState.status || 'loading');
          }
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
      if (kind === 'internet') {
        const statusNode = infoBody.querySelector('[data-modal-status][data-route-read="/api/status"]');
        if (statusNode) statusNode.textContent = internetStatusModalText();
      }
    }
    function wireModalFetches() {
      infoBody.querySelectorAll('[data-modal-fetch]').forEach(button => button.addEventListener('click', async () => {
        const output = infoBody.querySelector('[data-modal-output]');
        if (!headerState.isAdmin && button.closest('[data-admin-only]')) { if (output) output.textContent = 'Enter Admin Mode'; return; }
        const originalLabel = button.textContent;
        const isSpeedTest = button.hasAttribute('data-speed-test-button');
        if (isSpeedTest) {
          internetState.isSpeedTesting = true;
          internetState.speedTestResults = null;
          internetState.speedTestError = null;
          button.textContent = 'Running Speed Test...';
        } else if (button.dataset.operationLabel) button.textContent = button.dataset.operationLabel;
        button.classList.add('pending-operation');
        button.disabled = true;
        if (output) output.textContent = 'Loading ' + button.dataset.modalFetch + '…';
        try {
          const body = modalRequestBody(button);
          const response = await fetch(button.dataset.modalFetch, { method: button.dataset.method || 'GET', headers: body ? { 'Content-Type': 'application/json' } : undefined, body });
          const text = await response.text();
          let parsed = null;
          try { parsed = JSON.parse(text); } catch (_) {}
          if (isSpeedTest) {
            if (parsed?.error) throw new Error(parsed.error);
            if (parsed && (parsed.download !== undefined || parsed.upload !== undefined || parsed.latency !== undefined)) {
              internetState.speedTestResults = { download: parsed.download, upload: parsed.upload, latency: parsed.latency };
            } else if (!response.ok) {
              throw new Error(text || 'Speed test failed unexpectedly.');
            } else {
              internetState.speedTestError = parsed?.firstMissingSignal && parsed.firstMissingSignal !== 'none' ? parsed.firstMissingSignal : 'Speed test result unavailable.';
            }
            openInfoModal('Internet Status', 'internet');
            return;
          }
          if (output) { output.textContent = parsed ? JSON.stringify(parsed, null, 2) : text; }
        } catch (error) {
          if (isSpeedTest) {
            internetState.speedTestError = error?.message || 'Speed test failed unexpectedly.';
            openInfoModal('Internet Status', 'internet');
            return;
          }
          if (output) output.textContent = 'fetch failed: ' + error;
        }
        finally { if (isSpeedTest) internetState.isSpeedTesting = false; button.textContent = originalLabel; button.classList.remove('pending-operation'); button.disabled = false; }
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
    document.body.addEventListener('htmx:configRequest', event => {
      const token = localStorage.getItem('coronatioAdminToken');
      if (token) event.detail.headers['X-Admin-Token'] = token;
    });
    document.querySelector('[data-pin-cancel]')?.addEventListener('click', closePinModal);
    document.querySelector('[data-pin-confirm-button]')?.addEventListener('click', async () => {
      if (modalMode === 'change' && (!changeCurrentPinInput.value || !newPinInput.value || !confirmPinInput.value)) { modalMessage.textContent = 'Please fill in all fields'; return; }
      if (modalMode === 'change' && newPinInput.value !== confirmPinInput.value) { modalMessage.textContent = 'New PINs do not match'; return; }
      if (modalMode === 'enter' && !currentPinInput.value) { modalMessage.textContent = 'Enter PIN'; return; }
      if (modalMode === 'enter') {
        try {
          const response = await fetch('/api/validatePin', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ pin: currentPinInput.value }) });
          const result = await response.json().catch(() => ({}));
          if (!response.ok || !result.valid) { modalMessage.textContent = 'Invalid PIN'; return; }
          if (!result.token) { modalMessage.textContent = 'PIN check unavailable'; return; }
          localStorage.setItem('coronatioAdminToken', result.token);
        } catch (_) { modalMessage.textContent = 'PIN check unavailable'; return; }
      }
      setAdminMode(true);
      modalMessage.textContent = modalMode === 'change' ? 'PIN changed successfully' : '';
      if (modalMode === 'enter') closePinModal();
    });
    loadThemeCatalog();
    hydrateInternetIndicator();
    setInterval(hydrateInternetIndicator, 1000);
    refreshPowerIndicator();
    setInterval(refreshPowerIndicator, 5000);
    setAdminMode(headerState.isAdmin);
    function eligibleRegularTabs() { return tabs.filter(tab => tab.dataset.visibility !== 'hidden' && tab.dataset.adminOnly !== 'true'); }
    function visibleTabs() { return headerState.isAdmin ? tabs.filter(tab => tab.dataset.pane !== fallbackTab) : eligibleRegularTabs(); }
    function firstVisibleTab() { return eligibleRegularTabs()[0]?.dataset.pane || fallbackTab; }
    function currentActiveTabId() { return tabs.find(tab => tab.getAttribute('aria-selected') === 'true')?.dataset.pane || fallbackTab; }
    function canStarTab(id) { return eligibleRegularTabs().some(tab => tab.dataset.pane === id); }
    function lawfulPaneCandidate(id) {
      const tab = tabs.find(candidate => candidate.dataset.pane === id);
      if (!tab) return firstVisibleTab();
      if (tab.dataset.adminOnly === 'true') return headerState.isAdmin ? id : firstVisibleTab();
      if (tab.dataset.visibility === 'hidden') return firstVisibleTab();
      return id;
    }
    function applyTabBarVisibility() {
      if (!tabBar) return;
      const selected = currentActiveTabId();
      const hidden = !headerState.isAdmin && (selected === fallbackTab || eligibleRegularTabs().length <= 2);
      tabBar.classList.toggle('hidden', hidden);
      tabBar.dataset.hidden = String(hidden);
    }
    function reconcileActiveTabAfterAdminExit(previousActive) {
      if (eligibleRegularTabs().length === 0) { showPane(fallbackTab); return; }
      if (canStarTab(previousActive)) { showPane(previousActive); return; }
      if (canStarTab(tabState.starredTab)) { showPane(tabState.starredTab); return; }
      showPane(firstVisibleTab());
    }
    function setStarredTab(id) {
      const selected = canStarTab(id) ? id : firstVisibleTab();
      tabState.starredTab = selected;
      saveTabState(tabState);
      tabs.forEach(tab => {
        const starred = tab.dataset.pane === selected;
        const button = tab.querySelector('[data-tab-star]');
        if (button) {
          button.classList.toggle('fas', starred);
          button.classList.toggle('far', !starred);
          button.setAttribute('aria-pressed', String(starred));
          button.title = starred ? tab.querySelector('.tab-name').textContent + ' tab is starred' : 'Star ' + tab.querySelector('.tab-name').textContent + ' tab';
        }
      });
    }
    async function refreshTabBar(activeTabId = currentActiveTabId()) {
      if (!tabBar) return;
      const token = localStorage.getItem('coronatioAdminToken');
      const headers = token ? { 'X-Admin-Token': token } : {};
      const activeParam = activeTabId ? '?active=' + encodeURIComponent(activeTabId) : '';
      const response = await fetch('/api/tab-bar' + activeParam, { headers });
      if (!response.ok) return null;
      tabBar.innerHTML = await response.text();
      tabs = [...document.querySelectorAll('[data-pane]')];
      bindTabControls();
      applyTabBarVisibility();
      return currentActiveTabId();
    }
    function showPane(id) {
      const selected = lawfulPaneCandidate(id);
      tabs.forEach(tab => {
        const active = tab.dataset.pane === selected;
        tab.setAttribute('aria-selected', String(active));
        tab.classList.toggle('active', active);
      });
      panes.forEach(pane => pane.classList.toggle('active', pane.dataset.panePanel === selected));
      applyTabBarVisibility();
      if (location.hash !== '#' + selected) history.replaceState(null, '', '#' + selected);
    }
    function bindTabControls() {
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
        if (!canStarTab(button.dataset.tabStar)) return;
        try {
          const response = await fetch('/api/set_starred_tab', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ tabName: button.dataset.tabStar }) });
          if (response.ok && tabBar) { tabBar.innerHTML = await response.text(); tabs = [...document.querySelectorAll('[data-pane]')]; bindTabControls(); }
        } catch (_) {}
      }));
      document.querySelectorAll('[data-tab-visibility-toggle]').forEach(button => button.addEventListener('click', async event => {
        event.stopPropagation();
        const id = button.dataset.tabVisibilityToggle;
        const visible = button.dataset.visible !== 'true';
        const token = localStorage.getItem('coronatioAdminToken');
        try {
          const response = await fetch('/api/tabs/visibility', { method: 'POST', headers: { 'Content-Type': 'application/json', ...(token ? { 'X-Admin-Token': token } : {}) }, body: JSON.stringify({ tab: id, visible }) });
          if (response.ok && tabBar) { tabBar.innerHTML = await response.text(); tabs = [...document.querySelectorAll('[data-pane]')]; bindTabControls(); }
        } catch (_) {}
        const active = tabs.find(tab => tab.getAttribute('aria-selected') === 'true');
        if (!active || active.dataset.visibility === 'hidden') showPane(firstVisibleTab());
      }));
    }
    bindTabControls();
    setStarredTab(tabState.starredTab);
    async function fetchInto(route, target, method = 'GET') {
      const el = document.getElementById(target);"####
}