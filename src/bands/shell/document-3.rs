fn shell_document_3() -> &'static str {
    r####"        const documentIncarnation = (globalThis.crypto?.randomUUID?.() || `${Date.now()}-${Math.random()}`).replace(/[^a-zA-Z0-9._-]/g, '');
    let inactivityHeadless = false;
    const coronatioFetch = window.fetch.bind(window);
    window.fetch = (input, init = {}) => {
      const headers = new Headers(init.headers || {});
      headers.set('X-Caduceus-Document', documentIncarnation);
      return coronatioFetch(input, { ...init, headers, credentials: 'same-origin' });
    };
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
      if (infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="power-meter"]') && powerChartState.chart) renderPowerModal();
    }
    function applyAdminDomState() {
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
      if (changePinButton) {
        changePinButton.hidden = true;
        changePinButton.title = 'PIN changes are unavailable until a successor route is declared.';
      }
    }
    function setAdminMode(value) {
      const previousActive = currentActiveTabId();
      headerState.isAdmin = Boolean(value);
      saveHeaderState();
      applyAdminDomState();
      refreshTabBar(previousActive).then(selectedTab => {
        applyTabBarVisibility();
        refreshElementFragment('stats');
        if (selectedTab) showPane(selectedTab, { refresh: true });
      });
    }
    async function clearAdminMode() {
      try {
        await fetch('/api/session/clear', { method: 'POST', cache: 'no-store' });
      } catch (_) {
        // Browser projection still becomes guest when the clear route is unavailable.
      } finally {
        setAdminMode(false);
      }
    }
    async function bootstrapAdminMode() {
      // A document is always born GUEST. No cookie, refresh, or prior document may restore attendance.
      setAdminMode(false);
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
    __INDICATOR_MODAL_REGISTRY__
    function indicatorModalTemplate(kind) {
      const render = indicatorModalTemplates[kind];
      return render ? render() : '';
    }
    function modalTemplate(kind) {
      const indicator = indicatorModalTemplate(kind);
      if (indicator) return indicator;
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
    const powerChartState = { labels: [], watts: [], chart: null };
    function formatPowerWatts(watts) {
      const value = Number(watts);
      return Number.isFinite(value) ? (value * POWER_DISPLAY_FACTOR).toFixed(1) : 'unavailable';
    }
    function powerStatusColor(watts) {
      if (watts < 1) return 'var(--statusUp)';
      if (watts < 5) return 'var(--statusPartial)';
      return 'var(--statusDown)';
    }
    function pushPowerChartPoint(label, watts) {
      powerChartState.labels.push(label);
      powerChartState.watts.push(watts);
      if (powerChartState.labels.length > 60) {
        powerChartState.labels.shift();
        powerChartState.watts.shift();
      }
    }
    function averagePowerSamples(seconds) {
      const values = powerChartState.watts.slice(-seconds);
      return values.length ? values.reduce((sum, value) => sum + value, 0) / values.length : null;
    }
    function hydratePowerHistoryUI() {
      const modal = infoBody.querySelector('[data-modal-kind-body="power-meter"]');
      if (!modal) return;
      const latest = powerChartState.watts.at(-1);
      const latestNode = modal.querySelector('.power-value-number');
      const color = latest === undefined ? 'var(--statusUnknown)' : powerStatusColor(latest / POWER_DISPLAY_FACTOR);
      if (latestNode) latestNode.textContent = latest === undefined ? '—' : latest.toFixed(1);
      const powerValue = modal.querySelector('.power-value');
      if (powerValue) powerValue.style.color = color;
      [5, 30, 60].forEach(seconds => {
        const node = modal.querySelector(`[data-power-average="${seconds}"]`);
        const average = averagePowerSamples(seconds);
        if (node) { node.textContent = average === null ? '—W' : average.toFixed(1) + 'W'; node.style.color = color; }
      });
    }
    function renderPowerModal() {
      hydratePowerHistoryUI();
      const canvas = infoBody.querySelector('[data-power-chart]');
      if (!canvas || !window.Chart) return;
      if (powerChartState.chart) powerChartState.chart.destroy();
      powerChartState.chart = new Chart(canvas, {
        type: 'line',
        data: { labels: powerChartState.labels, datasets: [lineDataset('Power', powerChartState.watts, themeCssColor('--accent', '#90cff3'), 'y')] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { legend: { display: false }, tooltip: chartTooltip(context => Number(context.parsed.y || 0).toFixed(1) + 'W') },
          scales: {
            x: { ticks: chartTicks('--hiddenTabText', value => powerChartState.labels[value] || value), grid: { display: false } },
            y: { beginAtZero: true, ticks: chartTicks('--hiddenTabText', value => Number(value).toFixed(0) + 'W'), grid: chartGrid() }
          }
        })
      });
    }
    function hydratePowerIndicator(data) {
      const button = document.querySelector('[data-indicator="power-meter"]');
      const number = button?.querySelector('.power-value-small-number');
      if (!button || !number) return;
      if (data?.ok && typeof data.current === 'number') {
        const display = formatPowerWatts(data.current);
        const color = powerStatusColor(data.current);
        number.textContent = display;
        button.classList.remove('ok', 'warn', 'error');
        button.style.color = color;
        button.querySelector('.indicator-icon')?.style.setProperty('color', color);
        button.querySelector('.power-value-small')?.style.setProperty('color', color);
        button.title = 'Power: ' + display + 'W';
        button.setAttribute('aria-label', 'Power Usage ' + display + ' Watts');
      } else {
        number.textContent = '—';
        button.classList.remove('ok', 'warn', 'error');
        button.style.color = 'var(--statusUnknown)';
        button.title = 'Power readback unavailable';
        button.setAttribute('aria-label', 'Power readback unavailable');
      }
    }
    async function refreshPowerIndicator() {
      try {
        const response = await fetch('/api/status/power/usage', { cache: 'no-store' });
        const data = await response.json();
        hydratePowerIndicator(data);
        if (response.ok && data?.ok && typeof data.current === 'number') {
          pushPowerChartPoint(formatChartTime(), Number(formatPowerWatts(data.current)));
          if (infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="power-meter"]')) renderPowerModal();
        }
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
      if (kind === 'power-meter') renderPowerModal();
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
    adminButton?.addEventListener('click', async () => {
      if (headerState.isAdmin) await clearAdminMode();
      else openPinModal('enter');
    });
    document.querySelector('[data-pin-cancel]')?.addEventListener('click', closePinModal);
    document.querySelector('[data-pin-confirm-button]')?.addEventListener('click', async () => {
      if (modalMode === 'change') {
        modalMessage.textContent = 'PIN changes are unavailable until a successor route is declared.';
        return;
      }
      if (modalMode === 'enter' && !currentPinInput.value) { modalMessage.textContent = 'Enter PIN'; return; }
      if (modalMode === 'enter') {
        try {
          const response = await fetch('/api/session/mint', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ pin: currentPinInput.value }) });
          const result = await response.json().catch(() => ({}));
          const explicitPinRefusal = response.status === 401 && result?.firstMissingSignal === 'caduceus-access-refused';
          if (explicitPinRefusal) { modalMessage.textContent = 'Invalid PIN'; return; }
          if (!response.ok || result.admin !== true) { modalMessage.textContent = 'PIN check unavailable'; return; }
        } catch (_) { modalMessage.textContent = 'PIN check unavailable'; return; }
      }
      setAdminMode(true);
      modalMessage.textContent = '';
      closePinModal();
    });
    function enterInactivityHeadless() {
      if (inactivityHeadless) return;
      inactivityHeadless = true;
      if (headerState.isAdmin) { headerState.isAdmin = false; saveHeaderState(); applyAdminDomState(); }
      [pulseStream, coreStream].forEach(stream => { try { stream?.close(); } catch (_) {} });
      pulseStream = null; coreStream = null;
      document.documentElement.dataset.connectionState = 'headless';
      const notice = document.querySelector('[data-crown-headless-notice]') || document.createElement('p');
      notice.dataset.crownHeadlessNotice = 'true';
      notice.textContent = 'You have been disconnected due to inactivity.';
      notice.setAttribute('role', 'status');
      if (!notice.parentNode) document.querySelector('[data-product="Coronatio"]')?.prepend(notice);
    }
    let lastEligibleActivity = Date.now();
    const activityEvents = ['scroll', 'touchstart', 'pointerdown'];
    activityEvents.forEach(type => document.addEventListener(type, () => { lastEligibleActivity = Date.now(); }, { passive: true }));
    document.addEventListener('click', event => {
      if (!['INPUT', 'SELECT', 'TEXTAREA', 'BUTTON', 'LABEL'].includes(event.target?.tagName)) lastEligibleActivity = Date.now();
    }, { passive: true });
    window.setInterval(() => { if (Date.now() - lastEligibleActivity >= 15 * 60 * 1000) enterInactivityHeadless(); }, 60 * 1000);
    let pulseRenewTimer = null;
    let pulseStreamId = null;
    let coreStream = null;
    let coreRenewTimer = null;
    const coreTopicIds = ['internet.status', 'tailscale.status', 'vpn.status', 'services.status', 'power.status'];
    function applyCoreTopic(topicId, envelope) {
      const data = envelope?.snapshot || {};
      if (topicId === 'internet.status') setInternetIndicatorState(data);
      if (topicId === 'power.status') {
        hydratePowerIndicator(data);
        if (data?.ok && typeof data.current === 'number') pushPowerChartPoint(formatChartTime(), Number(formatPowerWatts(data.current)));
      }
      const indicatorId = ({ 'tailscale.status': 'tailscale', 'vpn.status': 'openvpn', 'services.status': 'services' })[topicId];
      const button = indicatorId ? document.querySelector(`[data-indicator="${indicatorId}"]`) : null;
      if (button) {
        button.classList.remove('loading', 'ok', 'warn', 'error');
        button.classList.add(envelope?.status === 'snapshot' && data?.ok !== false ? 'ok' : 'warn');
      }
    }
    function scheduleCoreRenewal(route) {
      if (coreRenewTimer) window.clearTimeout(coreRenewTimer);
      if (!route) return;
      coreRenewTimer = window.setTimeout(async () => {
        try { await fetch(route, { method: 'POST', cache: 'no-store' }); } catch (_) {}
        if (coreStream && coreStream.readyState !== EventSource.CLOSED) scheduleCoreRenewal(route);
      }, 15000);
    }
    function connectCoreStream() {
      if (!window.EventSource || coreStream) return;
      coreStream = new EventSource('/api/core/pulse');
      coreStream.addEventListener('core.open', event => {
        let data = {};
        try { data = JSON.parse(event.data || '{}'); } catch (_) {}
        scheduleCoreRenewal(data.renewRoute);
      });
      coreTopicIds.forEach(topicId => coreStream.addEventListener(topicId, event => {
        try { applyCoreTopic(topicId, JSON.parse(event.data || '{}')); } catch (_) {}
      }));
      coreStream.addEventListener('core.expired', () => {
        coreStream.close(); coreStream = null; window.setTimeout(connectCoreStream, 0);
      });
    }
    loadThemeCatalog();
    bootstrapAdminMode();
    connectCoreStream();
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
      const activeParam = activeTabId ? '?active=' + encodeURIComponent(activeTabId) : '';
      const response = await fetch('/api/tab-bar' + activeParam);
      if (!response.ok) return null;
      replaceTabBar(await response.text());
      return currentActiveTabId();
    }
    function replaceTabBar(html) { if (!tabBar) return;
      tabBar.innerHTML = html;
      if (window.htmx) window.htmx.process(tabBar); tabs = [...document.querySelectorAll('[data-pane]')];
      bindTabControls();
      applyTabBarVisibility();
    }
    function clearPulseRenewal() {
      if (pulseRenewTimer) window.clearTimeout(pulseRenewTimer);
      pulseRenewTimer = null;
    }
    const viewportStreamFamilies = Object.freeze({
      stats: Object.freeze({ topics: ['stats.system'], snapshotRoutes: ['/api/stats', '/api/network/notes'], eventRoute: '/api/stats/pulse', renewRoute: '/api/stats/pulse/renew', authClass: 'public-enhanced' }),
      dhcp: Object.freeze({ topics: ['admin.dhcp'], snapshotRoutes: ['/api/dhcp/leases', '/api/dhcp/reservations', '/api/dhcp/statistics', '/api/dhcp/pool-boundary'], eventRoute: null, renewRoute: null, authClass: 'admin' }),
      portals: Object.freeze({ topics: ['core.services'], snapshotRoutes: ['/api/portals/elements'], eventRoute: null, renewRoute: null, authClass: 'public' })
    });
    function viewportFamilyAdmitted(id) {
      const family = viewportStreamFamilies[id];
      if (!family || window.getImmortalFloorState?.() !== 'Seated' || document.visibilityState !== 'visible' || currentActiveTabId() !== id) return false;
      return family.authClass !== 'admin' || headerState.isAdmin;
    }
    function closeViewportStreamFamily() {
      clearPulseRenewal();
      if (pulseStream) pulseStream.close();
      pulseStream = null;
      pulseStreamId = null;
    }
    let portalCurrentnessTimer = null;
    function stopPortalCurrentnessCadence() { if (portalCurrentnessTimer) window.clearInterval(portalCurrentnessTimer); portalCurrentnessTimer = null; }
    function startPortalCurrentnessCadence() {
      stopPortalCurrentnessCadence();
      if (!viewportFamilyAdmitted('portals')) return;
      refreshPortalCurrentness();
      portalCurrentnessTimer = window.setInterval(() => { if (viewportFamilyAdmitted('portals')) refreshPortalCurrentness(); else stopPortalCurrentnessCadence(); }, 5000);
    }
    function reconcileViewportStreamFamily() {
      closeViewportStreamFamily();
      stopPortalCurrentnessCadence();
      if (window.getImmortalFloorState?.() !== 'Seated') return;
      const active = currentActiveTabId();
      if (!viewportFamilyAdmitted(active)) return;
      if (active === 'stats') { hydrateStats(); connectPulseStream(); }
      if (active === 'dhcp') hydrateDhcp();
      if (active === 'portals') startPortalCurrentnessCadence();
    }
    function schedulePulseRenewal(renewRoute) {
      clearPulseRenewal();
      if (!renewRoute) return;
      pulseRenewTimer = window.setTimeout(async () => {
        try { await fetch(renewRoute, { method: 'POST', cache: 'no-store' }); }
        catch (_) {}
        if (pulseStream && pulseStream.readyState !== EventSource.CLOSED) schedulePulseRenewal(renewRoute);
      }, 15000);
    }
    function reconnectPulseStream() {
      closeViewportStreamFamily();
      if (viewportFamilyAdmitted('stats')) window.setTimeout(() => {
        if (viewportFamilyAdmitted('stats')) connectPulseStream();
      }, 1000);
    }
    function connectPulseStream() {
      if (!window.EventSource || !viewportFamilyAdmitted('stats')) return;
      clearPulseRenewal();
      if (pulseStream) pulseStream.close();
      pulseStream = new EventSource('/api/stats/pulse');
      pulseStream.addEventListener('pulse.open', event => {
        let data = {};
        try { data = JSON.parse(event.data || '{}'); } catch (_) {}
        pulseStreamId = data.streamId || event.lastEventId || null;
        schedulePulseRenewal(data.renewRoute || (pulseStreamId ? '/api/stats/pulse/renew?streamId=' + encodeURIComponent(pulseStreamId) : null));
      });
      pulseStream.addEventListener('tabs.changed', () => {
        const active = currentActiveTabId();
        refreshTabBar(active).then(selected => { if (selected) showPane(selected); }).catch(() => {});
      });
      pulseStream.addEventListener('stats.tick', () => {
        refreshElementFragment('stats').catch(() => {});
      });
      pulseStream.addEventListener('pulse.expired', reconnectPulseStream);
      pulseStream.addEventListener('error', () => {
        if (pulseStream && pulseStream.readyState === EventSource.CLOSED) reconnectPulseStream();
      });
    }
    const crownDebug = installCrownDebugEmitter(); window.crownDebug = crownDebug; installCrownRequestDiagnostics(crownDebug); installCrownLayoutDiagnostics(crownDebug); const immortalFloorStates = Object.freeze(['BootFloor', 'Seated', 'GuestRevolution', 'BareFloor']);
    const immortalFloor = (() => {
      let state = 'BootFloor'; let generation = 0; let activeGuest = null;
      let crossingGuest = null;
      let floorDebugHandle = crownDebug.begin('immortal-floor-boot', { phase: 'boot', event: 'begin' });
      const admissionTimeoutMs = 1500;
      const hydrationTimeoutMs = 750;
      const ready = new Promise(resolve => requestAnimationFrame(() => {
        if (immortalFloorShell && tabBar && panes.length) {
          immortalFloorShell.dataset.startupPhase = 'Ready';
          resolve(true);
        } else resolve(false);
      }));
      function expose(next, detail = '') {
        crownDebug.mark(floorDebugHandle, 'expose', { state: next, phase: 'state', detail });
        if (!immortalFloorStates.includes(next)) throw new Error('Invalid Immortal Floor state: ' + next);
        state = next;
        document.documentElement.dataset.immortalFloorState = next;
        if (immortalFloorShell) immortalFloorShell.dataset.immortalFloorState = next;
        if (immortalFloorGuestSlot) immortalFloorGuestSlot.dataset.slotEmpty = String(!activeGuest && next !== 'Seated');
        const message = immortalFloorShell?.querySelector('[data-immortal-floor-message]');
        if (message) message.textContent = detail || ({ BootFloor: 'Preparing your controls…', GuestRevolution: 'Changing view…', BareFloor: 'Controls remain available. Choose a tab to try again.', Seated: '' }[next]);
      }
      function emptySlot() {
        closeViewportStreamFamily();
        panes.forEach(pane => {
          pane.classList.remove('active', 'immortal-floor-enter');
          pane.setAttribute('aria-hidden', 'true');
        });
        activeGuest = null;
        if (immortalFloorGuestSlot) immortalFloorGuestSlot.dataset.slotEmpty = 'true';
      }
      function panelIdFromHtmxEvent(event) {
        const panel = panelFromHtmxEvent(event);
        return panel instanceof HTMLElement ? (panel.dataset.viewPanel || panel.dataset.panePanel || '') : '';
      }
      function bounded(task, timeoutMs, kind) {
        let timer = null;
        return Promise.race([
          Promise.resolve().then(task),
          new Promise((_, reject) => { timer = window.setTimeout(() => reject(new Error(kind)), timeoutMs); })
        ]).finally(() => { if (timer) window.clearTimeout(timer); });
      }
      async function admitFreshGuest(id) {
        const pane = panes.find(candidate => candidate.dataset.panePanel === id);
        const tab = tabs.find(candidate => candidate.dataset.pane === id);
        if (!pane) throw new Error('guest-missing');
        if (tab?.getAttribute('hx-get') && window.htmx) {
          await new Promise((resolve, reject) => {
            let timer = null;
            const cleanup = () => {
              document.body.removeEventListener('htmx:afterSwap', afterSwap);
              document.body.removeEventListener('htmx:timeout', failed);
              document.body.removeEventListener('htmx:responseError', failed);
              document.body.removeEventListener('htmx:sendError', failed);
              if (timer) window.clearTimeout(timer);
            };
            const afterSwap = event => { if (panelIdFromHtmxEvent(event) !== id) return; crownDebug.mark(floorDebugHandle, 'after-swap', { guest: id, phase: 'admission' }); cleanup(); resolve(); };
            const failed = event => { if (panelIdFromHtmxEvent(event) !== id) return; crownDebug.mark(floorDebugHandle, 'admission-fault', { guest: id, phase: 'admission' }); cleanup(); reject(new Error('admission-fault')); };
            document.body.addEventListener('htmx:afterSwap', afterSwap);
            document.body.addEventListener('htmx:timeout', failed);
            document.body.addEventListener('htmx:responseError', failed);
            document.body.addEventListener('htmx:sendError', failed);
            timer = window.setTimeout(() => { crownDebug.mark(floorDebugHandle, 'admission-timeout', { guest: id, phase: 'admission' }); cleanup(); reject(new Error('admission-timeout')); }, admissionTimeoutMs);
            crownDebug.mark(floorDebugHandle, 'admission-trigger', { guest: id, phase: 'admission' });
            window.htmx.trigger(tab, 'immortal-floor-admit');
          });
        }
        try { if (id === 'stats') await bounded(() => hydrateStats(), hydrationTimeoutMs, 'hydration-timeout');
          else if (id === 'dhcp') await bounded(() => hydrateDhcp(), hydrationTimeoutMs, 'hydration-timeout'); }
        catch (error) { if ((error?.message || '') === 'hydration-timeout') crownDebug.mark(floorDebugHandle, 'hydration-timeout', { guest: id, phase: 'hydration' }); throw error; }
        if (!pane || pane.dataset.viewportFaulted === 'true') throw new Error('guest-unhealthy');
        return pane;
      }
      async function seatGuest(id) {
        const pane = panes.find(candidate => candidate.dataset.panePanel === id); if (!pane) return false;
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        crownDebug.mark(floorDebugHandle, 'paint-reveal-boundary', { phase: 'reveal', guest: id });
        tabs.forEach(tab => { const active = tab.dataset.pane === id; tab.setAttribute('aria-selected', String(active)); tab.classList.toggle('active', active); });
        panes.forEach(candidate => { const active = candidate === pane; candidate.classList.toggle('active', active); candidate.classList.toggle('immortal-floor-enter', active); candidate.setAttribute('aria-hidden', String(!active)); });
        activeGuest = id; crossingGuest = null; expose('Seated'); crownDebug.settle(floorDebugHandle, true, { event: 'settle', phase: 'seated', guest: id }); floorDebugHandle = null; applyAdminDomState(); applyTabBarVisibility();
        if (location.hash !== '#' + id) history.replaceState(null, '', '#' + id);
        reconcileViewportStreamFamily();
        return true;
      }
      async function activate(requested, options = {}) {
        const selected = lawfulPaneCandidate(requested); crossingGuest = selected;
        if (!floorDebugHandle) floorDebugHandle = crownDebug.begin('immortal-floor-boot', { phase: 'activation', event: 'begin', guest: selected });
        if (state === 'Seated' && activeGuest === selected && !options.refresh) {
          applyAdminDomState(); reconcileViewportStreamFamily(); return true;
        }
        const crossing = ++generation; const readyNow = await ready;
        if (!readyNow) {
          if (crossing === generation) { emptySlot(); expose('BareFloor'); }
          crownDebug.settle(floorDebugHandle, false, { event: 'settle', phase: 'bare-floor', reason: 'not-ready', guest: selected }); floorDebugHandle = null;
          return false;
        }
        if (crossing !== generation) return false; // A newer crossing owns the terminal state.
        expose('GuestRevolution');
        // Keep the healthy outgoing floor-2 guest visible under the held frame until reveal.
        try {
          await admitFreshGuest(selected);
          if (crossing !== generation) return false;
          if (!await seatGuest(selected)) { fault('guest-missing'); return false; }
          return true;
        } catch (error) {
          if (crossing === generation) fault(error?.message || 'admission-fault');
          return false;
        }
      }
      function fault(kind = 'guest-fault') {
        generation += 1;
        emptySlot();
        expose('BareFloor', 'This view could not open. Choose a tab to try again.');
        crownDebug.settle(floorDebugHandle, false, { event: 'settle', phase: 'bare-floor', reason: kind, guest: crossingGuest });
        floorDebugHandle = null;
        crossingGuest = null;
        document.documentElement.dataset.immortalFloorFault = kind;
      }
      function faultForPanel(panelId, kind = 'guest-fault') {
        if (state !== 'GuestRevolution' || !panelId || lawfulPaneCandidate(panelId) !== crossingGuest) return false;
        fault(kind);
        return true;
      }
      expose('BootFloor');
      return Object.freeze({ activate, fault, faultForPanel, get state() { return state; }, get activeGuest() { return activeGuest; } });
    })();
    window.immortalFloor = immortalFloor;
    window.getImmortalFloorState = () => immortalFloor.state;
    function showPane(id, options) { return immortalFloor.activate(id, options); }
    function bindTabControls() {
      tabs.forEach(tab => {
        if (tab.dataset.immortalFloorBound === 'true') return;
        tab.dataset.immortalFloorBound = 'true';
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
          if (response.ok && tabBar) replaceTabBar(await response.text());
          else { const failure = await response.json().catch(() => ({})); showCoronatioToast('Could not set favorite tab: ' + (failure.firstMissingSignal || failure.error || 'request failed') + '.', 'error'); }
        } catch (_) { showCoronatioToast('Could not set favorite tab: request failed.', 'error'); }
      }));
      document.querySelectorAll('[data-tab-visibility-toggle]').forEach(button => button.addEventListener('click', async event => {
        event.stopPropagation();
        const id = button.dataset.tabVisibilityToggle;
        const visible = button.dataset.visible !== 'true';
        try {
          const response = await fetch('/api/tabs/visibility', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ tab: id, visible }) });
          if (response.ok && tabBar) replaceTabBar(await response.text());
        } catch (_) {}
        const active = tabs.find(tab => tab.getAttribute('aria-selected') === 'true');
        if (!active || active.dataset.visibility === 'hidden') showPane(firstVisibleTab());
      }));
    }
    bindTabControls();
    setStarredTab(tabState.starredTab);
    document.addEventListener('visibilitychange', reconcileViewportStreamFamily);
    async function fetchInto(route, target, method = 'GET') {
      const el = document.getElementById(target);"####
}