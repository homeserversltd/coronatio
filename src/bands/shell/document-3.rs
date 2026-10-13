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
      if (infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="power-meter"]') && powerChartState.chart) renderPowerModal();
    }
    const adminDiskSnapshotFamily = Object.freeze({ paneId: 'admin', topics: [], snapshotRoutes: ['/api/v1/storage/disk/census'], authClass: 'admin', timeoutMs: 15000 });
    let adminDiskSnapshotOwner = null;
    let adminDiskSnapshotLastGood = null;
    let adminDiskPageHidden = false;
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
    }
    function retirePrivilegedClientState() {
      window.scaffoldRegistry?.retire('admin', 'Admin role retired');
      window.immortalFloor?.adminLost?.();
      if (window.__coronatioPrivilegedStateRetired !== true) {
        window.__coronatioPrivilegedStateRetired = true;
        window.dispatchEvent(new Event('coronatio:privileged-state-retired'));
      }
    }
    function setAdminMode(value, options = {}) {
      const previousActive = currentActiveTabId();
      const previousAdmin = headerState.isAdmin;
      headerState.isAdmin = Boolean(value);
      if (headerState.isAdmin) window.__coronatioPrivilegedStateRetired = false;
      if (!headerState.isAdmin) {
        retireAdminDiskSnapshot();
        if (previousAdmin) {
          retirePrivilegedClientState();
          invalidateAdminIndicatorModal();
          internetState.publicIp = undefined;
        }
      }
      saveHeaderState();
      applyAdminDomState();
      if (previousAdmin && !headerState.isAdmin) setInternetIndicatorState({ status: internetState.status });
      const currentIndicatorKind = infoBody.querySelector('[data-modal-kind-body]')?.dataset.modalKindBody;
      if (previousAdmin !== headerState.isAdmin && infoBackdrop.classList.contains('open') && ['power-meter', 'tailscale', 'internet', 'services'].includes(currentIndicatorKind)) {
        if (!headerState.isAdmin && currentIndicatorKind === 'power-meter') cancelPowerCalibrationEditing();
        openInfoModal(infoTitle.textContent, currentIndicatorKind);
      }
      if (previousAdmin !== headerState.isAdmin && infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="source-currency"]')) {
        openInfoModal(infoTitle.textContent, 'source-currency');
      }
      if (headerState.isAdmin) void upgradeOpenStreams();
      else void downgradeOpenStreams();
      if (options.boot) return Promise.resolve(true);
      if (headerState.isAdmin && options.admission) return AdminChromeProjection(previousActive);
      return refreshTabBar(previousActive).then(selectedTab => {
        refreshElementFragment('stats');
        if (!sessionLawfulTab(selectedTab)) void runFavoriteLadder({ startAt: 1 });
        if (previousAdmin !== headerState.isAdmin && infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="openvpn"]')) {
          openInfoModal(infoTitle.textContent, 'openvpn');
        }
        return true;
      });
    }
    let adminDocumentPatchPendingHydration = false;
    function installAdminDocumentPatch(patch, tabsHtml) {
      if (typeof patch !== 'string' || typeof tabsHtml !== 'string' || !immortalFloorGuestSlot) return false;
      const template = document.createElement('template');
      template.innerHTML = patch;
      const admitted = template.content.querySelector('[data-admin-document-patch="true"]');
      if (!admitted) return false;
      document.querySelector('[data-admin-document-patch="true"]')?.remove();
      immortalFloorGuestSlot.appendChild(admitted);
      panes = [...document.querySelectorAll('[data-pane-panel]')];
      adminDocumentPatchPendingHydration = true;
      replaceTabBar(tabsHtml);
      return true;
    }
    function removeAdminDocumentPatch() {
      retireAdminDiskSnapshot();
      if (document.querySelector('[data-admin-document-patch="true"]') || window.immortalFloor?.activeGuest === 'admin' || headerState.isAdmin) retirePrivilegedClientState();
      document.querySelector('[data-admin-document-patch="true"]')?.remove();
      panes = [...document.querySelectorAll('[data-pane-panel]')];
      adminDocumentPatchPendingHydration = false;
    }
    async function clearAdminMode() {
      try {
        await downgradeOpenStreams();
        if (coronatioAttendanceRuntime.currentAttendance) await fetch('/api/v1/attendance/invalidate', { method: 'POST', cache: 'no-store' });
      } catch (_) {
        // Browser projection still becomes guest when the clear route is unavailable.
      } finally {
        coronatioAttendanceRuntime.currentAttendance = null;
        removeAdminDocumentPatch();
        setAdminMode(false);
      }
    }
    async function bootstrapAdminMode() {
      // A document is always born GUEST. No cookie, refresh, or prior document may restore attendance.
      setAdminMode(false, { boot: true });
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
    function closePinModal() { [currentPinInput, changeCurrentPinInput, newPinInput, confirmPinInput].forEach(input => { input.value = ''; });
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
      const status = headerState.isAdmin && internetState.publicIp ? `${base} (${internetState.publicIp})` : base;
      return internetState.firstMissingSignal ? `${status} · ${internetState.firstMissingSignal}` : status;
    }
    function setInternetIndicatorState(data) {
      if (!data) return;
      internetState.status = data.status || 'unavailable';
      internetState.publicIp = headerState.isAdmin && typeof data.publicIp === 'string' ? data.publicIp : undefined;
      internetState.firstMissingSignal = readSignal(data, 'firstMissingSignal', 'first_missing_signal');
      if (internetIndicator) {
        internetIndicator.classList.remove('ok', 'warn', 'error', 'loading');
        if (internetState.status === 'connected') internetIndicator.classList.add('ok');
        else if (internetState.status === 'disconnected' || internetState.status === 'error' || internetState.status === 'unavailable') internetIndicator.classList.add('error');
        else internetIndicator.classList.add('loading');
        const title = internetState.status === 'loading' ? 'Checking internet connection...' : (headerState.isAdmin && internetState.publicIp ? `Internet: ${internetState.status} (${internetState.publicIp})` : `Internet: ${internetState.status}`);
        internetIndicator.title = internetState.firstMissingSignal ? `${title} · ${internetState.firstMissingSignal}` : title;
        internetIndicator.setAttribute('aria-label', internetState.status === 'loading' ? 'Checking Internet Status' : 'Internet Status ' + internetState.status);
      }
      const modalStatus = infoBody.querySelector('[data-modal-kind-body="internet"] [data-modal-status][data-route-read="/api/status"]');
      if (modalStatus) {
        modalStatus.textContent = internetStatusModalText();
        modalStatus.classList.remove('loading', 'connected', 'disconnected', 'error', 'unavailable');
        modalStatus.classList.add(internetState.status);
      }
    }
    function setServicesIndicatorState(data) {
      const button = document.querySelector('[data-indicator="services"]');
      if (!button) return;
      const status = data?.status || 'unavailable';
      button.classList.remove('loading', 'ok', 'warn', 'error');
      const indicatorClass = status === 'up' ? 'ok' : (status === 'partial' || status === 'unknown' ? 'warn' : (status === 'unavailable' || status === 'down' ? 'error' : 'loading'));
      button.classList.add(indicatorClass);
      const signal = readSignal(data, 'firstMissingSignal');
      button.title = status === 'unavailable' ? `Services status unavailable${signal ? ': ' + signal : ''}` : `Services: ${status}${signal ? ' · ' + signal : ''}`;
      button.setAttribute('aria-label', 'Services Status ' + status);
    }
    const sourceCurrencyUpdateState = { inFlight: false, latestEnvelope: null };
    let sourceCurrencyModalGeneration = 0;
    let infoModalGeneration = 0;
    function clearTransmissionSecrets(root = infoBody) {
      root.querySelectorAll('[data-transmission-password], [data-transmission-username], [data-authkey-input]').forEach(input => { input.value = ''; });
    }
    function clearTransmissionAdminDetails(root = infoBody) {
      root.querySelectorAll('[data-transmission-detail], [data-transmission-condition], [data-transmission-key-presence], [data-transmission-service-active], [data-transmission-service-enabled], [data-vpn-projection-enabled], [data-vpn-projection-unit]').forEach(node => { node.textContent = 'Unavailable'; });
    }
    function invalidateAdminIndicatorModal() {
      infoModalGeneration += 1;
      cancelPowerCalibrationEditing();
      clearTransmissionSecrets();
      clearTransmissionAdminDetails();
    }
    function modalResponseIsCurrent(generation, expectedModal, expectedAdmin) {
      return generation === infoModalGeneration &&
        headerState.isAdmin === expectedAdmin &&
        infoBackdrop.classList.contains('open') &&
        expectedModal instanceof Element &&
        infoBody.querySelector('[data-modal-kind-body]') === expectedModal;
    }
    function setSourceCurrencyIndicatorState(envelope) {
      sourceCurrencyUpdateState.latestEnvelope = envelope;
      const button = document.querySelector('[data-indicator="source-currency"]');
      if (!button) return;
      const data = envelope?.snapshot || {};
      const relation = data?.relation || (envelope?.status === 'unavailable' ? 'unavailable' : 'unknown');
      const state = relation === 'current' ? { status: 'up', className: 'up ok', label: 'Current' } :
        relation === 'behind' ? { status: 'partial', className: 'partial warn', label: 'Update available' } :
        relation === 'diverged' ? { status: 'down', className: 'down error', label: 'Diverged' } :
        { status: 'unknown', className: 'unknown error', label: 'Unknown' };
      button.classList.remove('loading', 'ok', 'warn', 'error', 'up', 'partial', 'down', 'unknown');
      button.classList.add(...state.className.split(' '));
      button.dataset.sourceCurrencyStatus = state.status;
      button.title = 'Source currency: ' + state.label;
      button.setAttribute('aria-label', 'Source Currency ' + state.label);
      const modal = infoBody.querySelector('[data-modal-kind-body="source-currency"]');
      if (!modal) return;
      modal.querySelector('[data-source-currency-label]')?.replaceChildren(document.createTextNode(state.label));
      modal.querySelector('[data-source-currency-relation]')?.replaceChildren(document.createTextNode(relation === 'current' ? 'The running build matches origin/main.' : relation === 'behind' ? 'An update is available from origin/main.' : relation === 'diverged' ? 'The running build differs from origin/main.' : 'The source-currency relation is unavailable.'));
      modal.querySelector('[data-source-currency-build-sha]')?.replaceChildren(document.createTextNode(data?.buildSha || 'Unavailable'));
      modal.querySelector('[data-source-currency-origin-main-sha]')?.replaceChildren(document.createTextNode(data?.originMainSha || 'Unavailable'));
      const update = modal.querySelector('[data-source-currency-update]');
      if (update) {
        const available = headerState.isAdmin && relation === 'behind';
        update.hidden = !available;
        update.disabled = !available || sourceCurrencyUpdateState.inFlight;
        update.setAttribute('aria-busy', String(sourceCurrencyUpdateState.inFlight));
      }
    }
    function sourceCurrencyUpdateServiceModal() {
      return infoBody.querySelector('[data-modal-kind-body="source-currency"]');
    }
    function renderSourceCurrencyUpdateServiceStatus(modal, status) {
      const control = modal?.querySelector('[data-source-currency-update-service-toggle]');
      const label = modal?.querySelector('[data-source-currency-update-service-label]');
      if (!control) return;
      const enabled = typeof status?.enabled === 'boolean' ? status.enabled : null;
      if (enabled === null) {
        control.checked = false;
        control.disabled = true;
        delete control.dataset.currentEnabled;
        control.setAttribute('aria-label', 'Automatic updates status unknown');
        control.setAttribute('aria-busy', 'false');
        if (label) label.textContent = 'Automatic updates status unknown';
        return;
      }
      const activeLabel = typeof status.active === 'boolean'
        ? (status.active ? 'timer active' : 'timer inactive')
        : 'timer state unknown';
      const enabledLabel = enabled ? 'On' : 'Off';
      control.checked = enabled;
      control.disabled = false;
      control.dataset.currentEnabled = String(enabled);
      control.setAttribute('aria-label', `Automatic updates ${enabledLabel}; ${activeLabel}`);
      control.setAttribute('aria-busy', 'false');
      if (label) label.textContent = `Automatic updates ${enabledLabel} · ${activeLabel}`;
    }
    async function refreshSourceCurrencyUpdateServiceStatus(generation = sourceCurrencyModalGeneration, expectedModal = sourceCurrencyUpdateServiceModal()) {
      const modal = expectedModal;
      const control = modal?.querySelector('[data-source-currency-update-service-toggle]');
      if (!headerState.isAdmin || !modal || !control) return;
      const isCurrent = () => generation === sourceCurrencyModalGeneration &&
        headerState.isAdmin && infoBackdrop.classList.contains('open') &&
        sourceCurrencyUpdateServiceModal() === modal;
      control.disabled = true;
      control.setAttribute('aria-busy', 'true');
      try {
        const response = await fetch('/api/caduceus/update/service/status', { cache: 'no-store' });
        const result = await response.json().catch(() => ({}));
        if (!isCurrent()) return;
        const readback = result?.readback;
        const status = response.ok && result?.ok === true && readback?.ok === true
          ? readback.body
          : null;
        renderSourceCurrencyUpdateServiceStatus(modal, status);
      } catch (_) {
        if (isCurrent()) renderSourceCurrencyUpdateServiceStatus(modal, null);
      }
    }
    async function startSourceCurrencyServiceToggle(control) {
      const modal = sourceCurrencyUpdateServiceModal();
      if (!headerState.isAdmin || !modal || modal.querySelector('[data-source-currency-update-service-toggle]') !== control || control.disabled) return;
      const currentEnabled = control.dataset.currentEnabled;
      if (currentEnabled !== 'true' && currentEnabled !== 'false') return;
      const state = currentEnabled === 'true' ? 'off' : 'on';
      const generation = sourceCurrencyModalGeneration;
      control.disabled = true;
      control.setAttribute('aria-busy', 'true');
      try {
        const response = await fetch('/api/caduceus/update/service/toggle', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          cache: 'no-store',
          body: JSON.stringify({ state })
        });
        const receipt = await response.json().catch(() => ({}));
        const readback = receipt?.readback || {};
        if (readback.status === 404) {
          showCoronatioToast('Automatic updates unavailable — Caduceus is not deployed yet.', 'warning');
        } else if (receipt?.accepted === true) {
          showCoronatioToast(`Automatic updates turned ${state === 'on' ? 'on' : 'off'}.`, 'success');
        } else if (readback.status === 409) {
          showCoronatioToast('Automatic updates could not be changed yet.', 'warning');
        } else {
          showCoronatioToast(readback.firstMissingSignal || receipt.firstMissingSignal || 'Automatic updates could not be changed.', 'warning');
        }
      } catch (_) {
        showCoronatioToast('Automatic updates could not be changed.', 'warning');
      } finally {
        await refreshSourceCurrencyUpdateServiceStatus(generation, modal);
      }
    }
    async function startSourceCurrencyUpdate(update) {
      if (!headerState.isAdmin || sourceCurrencyUpdateState.inFlight || update.disabled) return;
      sourceCurrencyUpdateState.inFlight = true;
      update.disabled = true;
      update.setAttribute('aria-busy', 'true');
      try {
        const response = await fetch('/api/caduceus/update/now', { method: 'POST', cache: 'no-store' });
        const receipt = await response.json().catch(() => ({}));
        const readback = receipt?.readback || {};
        if (readback.status === 404) {
          showCoronatioToast('Update pending — Caduceus is not deployed yet.', 'warning');
        } else if (receipt?.accepted) {
          showCoronatioToast('Update started — the crown will restart when it completes', 'success');
        } else if (readback.status === 409) {
          showCoronatioToast('Update is already running.', 'warning');
        } else {
          showCoronatioToast(readback.firstMissingSignal || receipt.firstMissingSignal || 'Update could not be started.', 'warning');
        }
      } catch (_) {
        showCoronatioToast('Update could not be started.', 'warning');
      } finally {
        sourceCurrencyUpdateState.inFlight = false;
        setSourceCurrencyIndicatorState(sourceCurrencyUpdateState.latestEnvelope);
      }
    }
    function transmissionStatusText(value) {
      return value === 'running' ? 'Running' : (value === 'stopped' ? 'Stopped' : 'Unavailable');
    }
    function setVpnIndicatorState(data, stateOverride = null) {
      const button = document.querySelector('[data-indicator="openvpn"]');
      if (!button) return;
      const valid = ['running', 'stopped'].includes(data?.vpnStatus) && ['running', 'stopped'].includes(data?.transmissionStatus);
      const state = ['ok', 'warn', 'error'].includes(stateOverride) ? stateOverride : (data?.status === 'available' && valid
        ? (data.vpnStatus === 'running' && data.transmissionStatus === 'running' ? 'ok' : (data.vpnStatus === 'running' || data.transmissionStatus === 'running' ? 'warn' : 'error'))
        : 'unavailable');
      button.classList.remove('loading', 'ok', 'warn', 'error');
      button.classList.add(state === 'unavailable' ? 'error' : state);
      button.dataset.indicatorState = state;
      const signal = readSignal(data, 'firstMissingSignal');
      button.title = state === 'unavailable' ? `VPN & Transmission status unavailable${signal ? ': ' + signal : ''}` : `VPN & Transmission ${state}`;
      button.setAttribute('aria-label', state === 'unavailable' ? 'VPN & Transmission status unavailable' : `VPN & Transmission ${state}`);
    }
    function hydrateTransmissionModal(data, modal = infoBody.querySelector('[data-modal-kind-body="openvpn"]')) {
      if (!modal) return;
      setVpnIndicatorState(data);
      const vpnNode = modal.querySelector('[data-modal-status]');
      const transmissionNode = modal.querySelector('[data-modal-secondary-status]');
      if (vpnNode) vpnNode.textContent = transmissionStatusText(data?.vpnStatus);
      if (transmissionNode) transmissionNode.textContent = transmissionStatusText(data?.transmissionStatus);
      const signalNode = modal.querySelector('[data-vpn-first-missing-signal]');
      if (signalNode) signalNode.textContent = readSignal(data, 'firstMissingSignal') || (data?.status === 'available' ? 'none' : 'vpn-status-unavailable');
      if (!headerState.isAdmin) return;
      const enabledNode = modal.querySelector('[data-vpn-projection-enabled]');
      if (enabledNode) enabledNode.textContent = data?.enabled === true ? 'Enabled' : (data?.enabled === false ? 'Disabled' : 'Unavailable');
      const unitNode = modal.querySelector('[data-vpn-projection-unit]');
      if (unitNode) unitNode.textContent = typeof data?.transmissionUnit === 'string' && data.transmissionUnit ? data.transmissionUnit : 'Unavailable';
    }
    function renderTransmissionKeyPresence(modal, data) {
      const presence = data?.presence || {};
      for (const [service, label] of [['pia', 'PIA'], ['transmission', 'Transmission']]) {
        const node = modal.querySelector(`[data-transmission-key-presence="${service}"]`);
        const value = presence[service];
        const state = typeof value === 'boolean' ? (value ? 'present' : 'absent') : 'unavailable';
        if (node) node.textContent = `${label} key: ${state}`;
      }
    }
    function renderTransmissionServiceStatus(modal, data) {
      const successful = (data?.ok === true || data?.success === true) && data?.success !== false && data?.ok !== false;
      const active = successful && typeof data?.active === 'boolean' ? (data.active ? 'active' : 'inactive') : 'unavailable';
      const enabled = successful && typeof data?.enabled === 'boolean' ? (data.enabled ? 'enabled' : 'disabled') : 'unavailable';
      const activeNode = modal.querySelector('[data-transmission-service-active]');
      const enabledNode = modal.querySelector('[data-transmission-service-enabled]');
      if (activeNode) activeNode.textContent = active;
      if (enabledNode) enabledNode.textContent = enabled;
    }
    async function hydrateTransmissionAdminReads(generation, modal) {
      if (!headerState.isAdmin || !modal) return;
      const isCurrent = () => modalResponseIsCurrent(generation, modal, true);
      try {
        const [keys, service] = await Promise.all([
          fetch('/api/transmission/keys', { cache: 'no-store' }).then(async response => ({ httpOk: response.ok, data: await response.json().catch(() => ({})) })),
          fetch('/api/service/control', { method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store', body: JSON.stringify({ service: 'transmissionVPN', action: 'status' }) }).then(async response => ({ httpOk: response.ok, data: await response.json().catch(() => ({})) }))
        ]);
        if (!isCurrent()) return;
        renderTransmissionKeyPresence(modal, keys.httpOk ? keys.data : null);
        renderTransmissionServiceStatus(modal, service.httpOk ? service.data : null);
      } catch (_) {
        if (!isCurrent()) return;
        renderTransmissionKeyPresence(modal, null);
        renderTransmissionServiceStatus(modal, null);
      }
    }
    function readSignal(data, ...keys) {
      for (const key of keys) {
        const value = data?.[key];
        if (typeof value === 'string' && value.trim() && value !== 'none') return value;
      }
      return '';
    }
    function unavailableReadData(data, fallback) {
      return {
        ok: false,
        success: false,
        status: 'unavailable',
        firstMissingSignal: readSignal(data, 'firstMissingSignal', 'first_missing_signal') || fallback
      };
    }
    function unavailablePowerUsage(data, fallback, firstMissingSignal = '') {
      const unavailable = unavailableReadData(data, fallback);
      if (firstMissingSignal) unavailable.firstMissingSignal = firstMissingSignal;
      const factor = data?.calibration;
      unavailable.calibration = typeof factor === 'number' && Number.isFinite(factor) && factor >= POWER_CALIBRATION_MIN && factor <= POWER_CALIBRATION_MAX
        ? factor
        : powerState.calibrationFactor;
      unavailable.historyFirstMissingSignal = readSignal(data, 'historyFirstMissingSignal', 'history_first_missing_signal') || unavailable.firstMissingSignal;
      return unavailable;
    }
    function routeUnavailableSignal(route) {
      return ({
        '/api/status': 'internet-status-unavailable',
        '/api/status/tailscale': 'tailscale-status-unavailable',
        '/api/status/tailscale/config': 'tailscale-config-unavailable',
        '/api/status/vpn/pia': 'vpn-status-unavailable',
        '/api/status/vpn/transmission': 'transmission-status-unavailable',
        '/api/status/services': 'services-status-unavailable',
        '/api/status/power/usage': 'power-usage-unavailable'
      })[route] || 'status-read-unavailable';
    }
    function routeReadWasRefused(response, data) {
      return !response.ok || !data || typeof data !== 'object' || Array.isArray(data) || data?.ok === false || data?.success === false || data?.refused === true;
    }
    function routeReadLabel(route, data) {
      const ok = data && (data.ok === true || data.success === true);
      const signal = readSignal(data, 'firstMissingSignal', 'first_missing_signal');
      const missing = signal ? ' · ' + signal : '';
      if (route === '/api/status/tailscale/config') return ok && typeof data.tailnet === 'string' ? data.tailnet : (signal || 'Tailnet unavailable');
      if (route === '/api/status') {
        setInternetIndicatorState(data);
        return internetStatusModalText();
      }
      if (route === '/api/status/tailscale') return ok ? 'Tailscale status: ' + String(data.status || 'unavailable') + missing : (signal || 'Tailscale status unavailable');
      if (route === '/api/status/vpn/pia') return data?.vpnStatus ? String(data.vpnStatus).toUpperCase() : (signal || 'VPN status unavailable');
      if (route === '/api/status/vpn/transmission') return data?.transmissionStatus ? String(data.transmissionStatus).toUpperCase() : (signal || 'Transmission status unavailable');
      if (route === '/api/status/services') return ok ? 'Services status: ' + String(data.status || 'unavailable') + missing : (signal || 'Services status unavailable');
      if (route.includes('power')) return typeof data?.current === 'number' && Number.isFinite(data.current) ? formatPowerWatts(data.current) + ' W' : (signal || 'Power readback unavailable');
      return ok ? 'Internet status: ' + String(data.status || 'unavailable') + missing : (signal || 'Internet status unavailable');
    }
    const POWER_CALIBRATION_MIN = 0.1;
    const POWER_CALIBRATION_MAX = 10;
    const powerState = { usage: null, calibrationFactor: 1, calibrationReadbackConfirmed: false, previewFactor: null, previewInvalid: false, usageSignal: '', historySignal: '', calibrationSignal: '' };
    const powerChartState = { chart: null };
    let powerPreviewFrame = null;
    let powerCalibrationRevision = 0;
    let powerCalibrationSaveTimer = null;
    const POWER_CALIBRATION_SAVE_DEBOUNCE_MS = 400;
    let powerSaveQueuedFactor = null;
    let powerSaveRunner = null;
    let powerSaveController = null;
    let powerSaveEpoch = 0;
    function currentPowerFactor() {
      const factor = powerState.previewFactor ?? powerState.calibrationFactor;
      return Number.isFinite(factor) && factor >= POWER_CALIBRATION_MIN && factor <= POWER_CALIBRATION_MAX ? factor : 1;
    }
    function powerFactorSuffix(factor = currentPowerFactor()) { return Math.abs(factor - 1) < 1e-9 ? '' : ` ×${factor.toFixed(2)}`; }
    function formatPowerWatts(rawWatts, factor = currentPowerFactor()) {
      return typeof rawWatts === 'number' && Number.isFinite(rawWatts) ? (rawWatts * factor).toFixed(1) : 'unavailable';
    }
    function calibratedPowerValue(rawWatts) {
      return typeof rawWatts === 'number' && Number.isFinite(rawWatts) ? rawWatts * currentPowerFactor() : null;
    }
    function powerStatusColor(displayWatts) {
      if (displayWatts < 1) return 'var(--statusUp)';
      if (displayWatts < 5) return 'var(--statusPartial)';
      return 'var(--statusDown)';
    }
    function powerIndicatorState(displayWatts) {
      if (displayWatts === null) return powerState.usage === null ? 'loading' : 'error';
      return displayWatts < 1 ? 'ok' : (displayWatts < 5 ? 'warn' : 'error');
    }
    function setPowerUsage(data) {
      if (!data || typeof data !== 'object') return;
      powerState.usage = data;
      powerState.usageSignal = readSignal(data, 'firstMissingSignal');
      powerState.historySignal = readSignal(data, 'historyFirstMissingSignal');
      const factor = data.calibration;
      if (!powerState.calibrationReadbackConfirmed && !powerState.previewInvalid && powerState.previewFactor === null && typeof factor === 'number' && Number.isFinite(factor) && factor >= POWER_CALIBRATION_MIN && factor <= POWER_CALIBRATION_MAX) {
        powerState.calibrationFactor = factor;
      }
      hydratePowerIndicator();
      if (infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="power-meter"]')) renderPowerModal();
    }
    function hydratePowerIndicator() {
      const button = document.querySelector('[data-indicator="power-meter"]');
      const number = button?.querySelector('.power-value-small-number');
      if (!button || !number) return;
      const current = powerState.usage?.current;
      const display = calibratedPowerValue(current);
      const label = button.querySelector('[data-power-factor-label]');
      const suffix = powerFactorSuffix();
      number.textContent = display === null ? '—' : display.toFixed(1);
      if (label) { label.textContent = suffix; label.hidden = !suffix; }
      button.classList.remove('ok', 'warn', 'error', 'loading');
      const state = powerIndicatorState(display);
      button.classList.add(state);
      const color = display === null ? 'var(--statusUnknown)' : powerStatusColor(display);
      button.style.color = color;
      button.querySelector('.indicator-icon')?.style.setProperty('color', color);
      button.querySelector('.power-value-small')?.style.setProperty('color', color);
      const signal = readSignal(powerState.usage, 'firstMissingSignal');
      button.title = display === null ? `Power readback unavailable${signal ? ': ' + signal : ''}` : `Power: ${display.toFixed(1)}W${suffix}`;
      button.setAttribute('aria-label', display === null ? 'Power readback unavailable' : `Power Usage ${display.toFixed(1)} Watts${suffix}`);
    }
    function powerTimestamp(data = powerState.usage) {
      const value = data?.timestamp ?? data?.ts;
      return typeof value === 'string' || typeof value === 'number' ? String(value) : '';
    }
    function renderPowerModal() {
      const modal = infoBody.querySelector('[data-modal-kind-body="power-meter"]');
      if (!modal) return;
      const factor = currentPowerFactor();
      const suffix = powerFactorSuffix(factor);
      const current = calibratedPowerValue(powerState.usage?.current);
      const latestNode = modal.querySelector('.power-value-number');
      if (latestNode) latestNode.textContent = current === null ? '—' : current.toFixed(1);
      const factorNode = modal.querySelector('[data-power-modal-factor]');
      if (factorNode) { factorNode.textContent = suffix; factorNode.hidden = !suffix; }
      const powerValue = modal.querySelector('.power-value');
      if (powerValue) powerValue.style.color = current === null ? 'var(--statusUnknown)' : powerStatusColor(current);
      [5, 30, 60].forEach(seconds => {
        const node = modal.querySelector(`[data-power-average="${seconds}"]`);
        const rawAverage = powerState.usage?.averages?.[`${seconds}s`];
        const average = typeof rawAverage === 'number' && Number.isFinite(rawAverage) ? rawAverage * factor : null;
        if (node) {
          node.textContent = average === null ? '—W' : `${average.toFixed(1)}W${suffix}`;
          node.style.color = average === null ? 'var(--statusUnknown)' : powerStatusColor(average);
        }
      });
      const signalNode = modal.querySelector('[data-power-reading-signal]');
      const signals = [];
      const currentValid = typeof powerState.usage?.current === 'number' && Number.isFinite(powerState.usage.current);
      if (powerState.usageSignal) {
        const label = !currentValid ? 'Current' : (powerState.usageSignal.toLowerCase().includes('calibration') ? 'Calibration' : 'Read');
        signals.push(label + ': ' + powerState.usageSignal);
      }
      if (powerState.historySignal && powerState.historySignal !== powerState.usageSignal) signals.push('History: ' + powerState.historySignal);
      if (!currentValid && !powerState.usageSignal) signals.push('Current: unavailable');
      if (powerState.calibrationSignal && powerState.calibrationSignal !== powerState.usageSignal) signals.push('Calibration: ' + powerState.calibrationSignal);
      if (signalNode) { signalNode.textContent = signals.join(' · '); signalNode.hidden = signals.length === 0; }
      const timestampNode = modal.querySelector('[data-power-timestamp]');
      if (timestampNode) timestampNode.textContent = powerTimestamp() ? 'Latest sample: ' + powerTimestamp() : 'Latest sample time unavailable';
      const canvas = modal.querySelector('[data-power-chart]');
      if (!canvas || !window.Chart) return;
      if (powerChartState.chart && powerChartState.chart.canvas !== canvas) { powerChartState.chart.destroy(); powerChartState.chart = null; }
      const chart = powerChartState.chart || createPowerChart(canvas);
      const rawHistory = Array.isArray(powerState.usage?.historical) ? powerState.usage.historical.filter(value => typeof value === 'number' && Number.isFinite(value)) : [];
      const labels = rawHistory.map((_, index) => index === rawHistory.length - 1 ? powerTimestamp() : '');
      chart.data.labels = labels;
      chart.data.datasets[0].data = rawHistory.map(value => value * factor);
      chart.data.datasets[0].label = 'Power' + suffix;
      chart.update();
    }
    function createPowerChart(canvas) {
      powerChartState.chart = new Chart(canvas, {
        type: 'line',
        data: { labels: [], datasets: [lineDataset('Power', [], themeCssColor('--accent', '#90cff3'), 'y')] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { legend: { display: false }, tooltip: chartTooltip(context => {
            const value = Number(context.parsed.y);
            return Number.isFinite(value) ? `${value.toFixed(1)}W${powerFactorSuffix()}` : 'unavailable';
          }) },
          scales: {
            x: { ticks: chartTicks('--hiddenTabText', value => powerChartState.chart?.data.labels[value] || ''), grid: { display: false } },
            y: { beginAtZero: true, ticks: chartTicks('--hiddenTabText', value => Number(value).toFixed(0) + 'W' + powerFactorSuffix()), grid: chartGrid() }
          }
        })
      });
      return powerChartState.chart;
    }
    function powerModalIsCurrent(generation, modal, expectedAdmin = headerState.isAdmin) {
      return headerState.isAdmin === expectedAdmin && modalResponseIsCurrent(generation, modal, expectedAdmin) && modal.dataset.modalKindBody === 'power-meter';
    }
    function setPowerCalibrationStatus(modal, message, kind = '') {
      if (!(modal instanceof Element) || !modal.isConnected) return;
      const node = modal.querySelector('[data-power-calibration-status]');
      if (node) { node.textContent = message; node.dataset.state = kind; }
    }
    function syncPowerCalibrationControls(modal, factor = currentPowerFactor()) {
      const slider = modal?.querySelector('[data-power-factor-slider]');
      const number = modal?.querySelector('[data-power-factor-number]');
      if (slider) slider.value = String(factor);
      if (number) number.value = String(factor);
      const save = modal?.querySelector('[data-power-factor-save]');
      if (save) save.disabled = !headerState.isAdmin || powerState.previewInvalid;
    }
    function updatePowerPreview(modal, input, generation, isRange) {
      if (!headerState.isAdmin || !powerModalIsCurrent(generation, modal)) return;
      if (powerCalibrationSaveTimer !== null) window.clearTimeout(powerCalibrationSaveTimer);
      powerCalibrationSaveTimer = null;
      powerCalibrationRevision += 1;
      powerSaveQueuedFactor = null;
      const revision = powerCalibrationRevision;
      const factor = input.valueAsNumber;
      if (!Number.isFinite(factor) || factor < POWER_CALIBRATION_MIN || factor > POWER_CALIBRATION_MAX) {
        powerState.previewInvalid = true;
        const save = modal.querySelector('[data-power-factor-save]');
        if (save) save.disabled = true;
        setPowerCalibrationStatus(modal, 'Enter a finite factor from 0.1 to 10. Automatic save cancelled.', 'invalid');
        return;
      }
      powerState.previewInvalid = false;
      powerState.previewFactor = factor;
      const slider = modal.querySelector('[data-power-factor-slider]');
      const number = modal.querySelector('[data-power-factor-number]');
      if (isRange && number) number.value = input.value;
      if (!isRange && slider) slider.value = input.value;
      const save = modal.querySelector('[data-power-factor-save]');
      if (save) save.disabled = false;
      setPowerCalibrationStatus(modal, `Preview factor ×${factor.toFixed(2)}; saving automatically…`, 'preview');
      if (powerPreviewFrame !== null) cancelAnimationFrame(powerPreviewFrame);
      powerPreviewFrame = requestAnimationFrame(() => {
        powerPreviewFrame = null;
        if (!powerModalIsCurrent(generation, modal)) return;
        hydratePowerIndicator();
        renderPowerModal();
      });
      powerCalibrationSaveTimer = window.setTimeout(() => {
        powerCalibrationSaveTimer = null;
        if (revision !== powerCalibrationRevision || powerState.previewInvalid || powerState.previewFactor !== factor || !powerModalIsCurrent(generation, modal)) return;
        enqueuePowerCalibrationSave(factor, generation, modal);
      }, POWER_CALIBRATION_SAVE_DEBOUNCE_MS);
    }
    function wirePowerCalibrationControls(generation, modal) {
      if (!headerState.isAdmin || !modal) return;
      const slider = modal.querySelector('[data-power-factor-slider]');
      const number = modal.querySelector('[data-power-factor-number]');
      const save = modal.querySelector('[data-power-factor-save]');
      if (!slider || !number || !save) return;
      const factor = powerState.previewFactor ?? powerState.calibrationFactor;
      syncPowerCalibrationControls(modal, factor);
      slider.addEventListener('input', () => updatePowerPreview(modal, slider, generation, true));
      number.addEventListener('input', () => updatePowerPreview(modal, number, generation, false));
      save.addEventListener('click', () => {
        if (!powerModalIsCurrent(generation, modal) || powerState.previewInvalid) return;
        const requested = powerState.previewFactor ?? powerState.calibrationFactor;
        if (!Number.isFinite(requested) || requested < POWER_CALIBRATION_MIN || requested > POWER_CALIBRATION_MAX) {
          setPowerCalibrationStatus(modal, 'Enter a finite factor from 0.1 to 10.', 'invalid');
          return;
        }
        enqueuePowerCalibrationSave(requested, generation, modal);
      });
    }
    function calibrationReadSucceeded(response, data) {
      const factor = data?.factor;
      return response.ok && data?.ok === true && !readSignal(data, 'firstMissingSignal') && typeof factor === 'number' && Number.isFinite(factor) && factor >= POWER_CALIBRATION_MIN && factor <= POWER_CALIBRATION_MAX;
    }
    async function loadPowerCalibration(generation, modal) {
      if (!headerState.isAdmin || !powerModalIsCurrent(generation, modal, true)) return;
      const revision = powerCalibrationRevision;
      const hasPreview = () => powerState.previewFactor !== null || powerState.previewInvalid;
      const preservePreviewStatus = message => {
        if (powerState.previewInvalid) setPowerCalibrationStatus(modal, 'Enter a finite factor from 0.1 to 10. Automatic save cancelled. ' + message, 'invalid');
        else if (powerState.previewFactor !== null) setPowerCalibrationStatus(modal, `Preview factor ×${powerState.previewFactor.toFixed(2)}; saving automatically… ${message}`, 'preview');
      };
      try {
        const response = await fetch('/api/status/power/calibration', { cache: 'no-store' });
        const data = await response.json().catch(() => ({}));
        if (!powerModalIsCurrent(generation, modal, true) || revision !== powerCalibrationRevision) return;
        if (!calibrationReadSucceeded(response, data)) {
          powerState.calibrationSignal = readSignal(data, 'firstMissingSignal') || 'power-calibration-read-unavailable';
          const message = 'Saved calibration read unavailable: ' + powerState.calibrationSignal;
          if (hasPreview()) preservePreviewStatus(message);
          else setPowerCalibrationStatus(modal, message, 'error');
          return;
        }
        powerState.calibrationFactor = data.factor;
        powerState.calibrationReadbackConfirmed = true;
        powerState.calibrationSignal = '';
        if (hasPreview()) {
          preservePreviewStatus(`Current saved factor ×${data.factor.toFixed(2)}.`);
        } else {
          syncPowerCalibrationControls(modal, data.factor);
          setPowerCalibrationStatus(modal, `Current saved factor ×${data.factor.toFixed(2)}.`, 'current');
        }
        hydratePowerIndicator();
        renderPowerModal();
      } catch (_) {
        if (!powerModalIsCurrent(generation, modal, true) || revision !== powerCalibrationRevision) return;
        powerState.calibrationSignal = 'power-calibration-read-failed';
        const message = 'Saved calibration read unavailable: ' + powerState.calibrationSignal;
        if (hasPreview()) preservePreviewStatus(message);
        else setPowerCalibrationStatus(modal, message, 'error');
      }
    }
    async function loadPowerUsage(generation, modal, expectedAdmin = headerState.isAdmin) {
      try {
        const response = await fetch('/api/status/power/usage', { cache: 'no-store' });
        const data = await response.json().catch(() => ({}));
        if (!powerModalIsCurrent(generation, modal, expectedAdmin)) return;
        if (routeReadWasRefused(response, data)) setPowerUsage(unavailablePowerUsage(data, 'power-usage-read-unavailable'));
        else if (data && typeof data === 'object') setPowerUsage(data);
        else setPowerUsage(unavailablePowerUsage(null, 'power-usage-read-invalid'));
      } catch (_) {
        if (!powerModalIsCurrent(generation, modal, expectedAdmin)) return;
        setPowerUsage(unavailablePowerUsage(null, 'power-usage-read-failed'));
      }
    }
    async function savePowerCalibration(requested, epoch, revision, generation, modal) {
      if (epoch !== powerSaveEpoch || !powerModalIsCurrent(generation, modal)) return;
      const controller = new AbortController();
      powerSaveController = controller;
      const signal = receipt => readSignal(receipt, 'firstMissingSignal', 'first_missing_signal');
      let writeAccepted = false;
      try {
        if (revision === powerCalibrationRevision) setPowerCalibrationStatus(modal, 'Saving calibration…', 'pending');
        const response = await fetch('/api/status/power/calibration', {
          method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store', signal: controller.signal,
          body: JSON.stringify({ factor: requested })
        });
        const receipt = await response.json().catch(() => ({}));
        const accepted = response.ok && (receipt?.ok === true || receipt?.accepted === true || receipt?.success === true) && receipt?.ok !== false && receipt?.accepted !== false && receipt?.success !== false && receipt?.refused !== true && !signal(receipt);
        if (epoch !== powerSaveEpoch || !powerModalIsCurrent(generation, modal)) return;
        if (!accepted) {
          if (revision === powerCalibrationRevision) {
            const refusal = signal(receipt) || receipt?.error || receipt?.refusal || 'power-calibration-write-refused';
            setPowerCalibrationStatus(modal, 'Calibration not confirmed: ' + String(refusal), 'error');
          }
          return;
        }
        writeAccepted = true;
        const readResponse = await fetch('/api/status/power/calibration', { cache: 'no-store', signal: controller.signal });
        const readback = await readResponse.json().catch(() => ({}));
        if (epoch !== powerSaveEpoch || !powerModalIsCurrent(generation, modal) || revision !== powerCalibrationRevision) return;
        if (!calibrationReadSucceeded(readResponse, readback)) {
          const refusal = signal(readback) || 'power-calibration-readback-unavailable';
          powerState.calibrationSignal = refusal;
          setPowerCalibrationStatus(modal, 'Write accepted; saved factor not confirmed: ' + refusal, 'error');
          return;
        }
        const actual = readback.factor;
        powerState.calibrationFactor = actual;
        powerState.calibrationReadbackConfirmed = true;
        powerState.calibrationSignal = '';
        const matches = Math.abs(actual - requested) <= 1e-9 * Math.max(1, Math.abs(requested));
        if (matches && powerState.previewFactor === requested) powerState.previewFactor = null;
        if (powerState.previewFactor === null) syncPowerCalibrationControls(modal, actual);
        setPowerCalibrationStatus(modal, matches ? `Saved factor ×${actual.toFixed(2)} (read back).` : `Saved factor read back as ×${actual.toFixed(2)}; requested ×${requested.toFixed(2)}.`, matches ? 'saved' : 'mismatch');
        hydratePowerIndicator();
        renderPowerModal();
      } catch (error) {
        if (epoch !== powerSaveEpoch || !powerModalIsCurrent(generation, modal) || revision !== powerCalibrationRevision) return;
        const refusal = error?.name === 'AbortError'
          ? 'power-calibration-request-cancelled'
          : (writeAccepted ? 'power-calibration-readback-failed' : 'power-calibration-write-failed');
        setPowerCalibrationStatus(modal, writeAccepted ? 'Write accepted; saved factor not confirmed: ' + refusal : 'Calibration not confirmed: ' + refusal, 'error');
      } finally {
        if (powerSaveController === controller) powerSaveController = null;
      }
    }
    function enqueuePowerCalibrationSave(factor, generation, modal) {
      if (!headerState.isAdmin || !powerModalIsCurrent(generation, modal) || powerState.previewInvalid) return;
      if (!Number.isFinite(factor) || factor < POWER_CALIBRATION_MIN || factor > POWER_CALIBRATION_MAX) return;
      if (powerCalibrationSaveTimer !== null) window.clearTimeout(powerCalibrationSaveTimer);
      powerCalibrationSaveTimer = null;
      const revision = ++powerCalibrationRevision;
      powerSaveQueuedFactor = { factor, revision };
      if (powerSaveRunner) return;
      const epoch = powerSaveEpoch;
      const runner = (async () => {
        while (epoch === powerSaveEpoch && powerSaveQueuedFactor !== null && powerModalIsCurrent(generation, modal)) {
          const next = powerSaveQueuedFactor;
          powerSaveQueuedFactor = null;
          await savePowerCalibration(next.factor, epoch, next.revision, generation, modal);
        }
      })();
      powerSaveRunner = runner;
      void runner.finally(() => { if (powerSaveEpoch === epoch && powerSaveRunner === runner) powerSaveRunner = null; });
    }
    function cancelPowerCalibrationEditing() {
      powerSaveEpoch += 1;
      powerCalibrationRevision += 1;
      if (powerCalibrationSaveTimer !== null) window.clearTimeout(powerCalibrationSaveTimer);
      powerCalibrationSaveTimer = null;
      powerSaveQueuedFactor = null;
      powerSaveRunner = null;
      powerSaveController?.abort();
      powerSaveController = null;
      if (powerPreviewFrame !== null) cancelAnimationFrame(powerPreviewFrame);
      powerPreviewFrame = null;
      powerState.previewFactor = null;
      powerState.previewInvalid = false;
      hydratePowerIndicator();
      renderPowerModal();
    }
    function internetSpeedTestHtml() {
      if (internetState.speedTestError) return `<div class="error-message">${internetState.speedTestError}</div>`;
      const result = internetState.speedTestResults;
      if (!result) return '';
      return `<div class="speed-results"><p>Download: ${result.download} Mbps</p><p>Upload: ${result.upload} Mbps</p><p>Latency: ${result.latency} ms</p></div>`;
    }
    function safeHttpUrl(value) {
      if (typeof value !== 'string' || !value.trim()) return null;
      try {
        const url = new URL(value.trim());
        return url.protocol === 'http:' || url.protocol === 'https:' ? url.href : null;
      } catch (_) { return null; }
    }
    function projectionText(value) {
      if (value === null || value === undefined || value === '') return 'Unavailable';
      if (typeof value === 'number' && !Number.isFinite(value)) return 'Unavailable';
      return String(value);
    }
    function tailscaleStatusClass(data) {
      const status = data?.status;
      if (status === 'loading') return 'loading';
      return ['connected', 'disconnected', 'needs-login', 'absent'].includes(status) ? status : 'unavailable';
    }
    function setTailscaleIndicatorState(data) {
      const button = document.querySelector('[data-indicator="tailscale"]');
      if (!button) return;
      const status = tailscaleStatusClass(data);
      button.classList.remove('loading', 'ok', 'warn', 'error');
      const indicatorClass = status === 'connected' ? 'ok' : (status === 'needs-login' ? 'warn' : (status === 'loading' ? 'loading' : 'error'));
      button.classList.add(indicatorClass);
      button.dataset.tailscaleStatus = status;
      const signal = readSignal(data, 'firstMissingSignal');
      button.title = status === 'loading' ? 'Checking Tailscale status…' : `Tailscale: ${status}${signal ? ' · ' + signal : ''}`;
      button.setAttribute('aria-label', 'Tailscale Status ' + status);
    }
    function hydrateTailscaleModal(data) {
      const modal = infoBody.querySelector('[data-modal-kind-body="tailscale"]');
      const statusNode = modal?.querySelector('[data-modal-status]');
      if (!modal || !statusNode) return;
      const status = tailscaleStatusClass(data);
      setTailscaleIndicatorState(data);
      statusNode.className = 'status-text ' + status;
      const ip = headerState.isAdmin && typeof data?.ip === 'string' && data.ip ? ` (${data.ip})` : '';
      statusNode.textContent = status === 'loading' ? 'LOADING...' : status.toUpperCase() + ip;
      const interfaceNode = modal.querySelector('[data-tailscale-interface]');
      if (interfaceNode) interfaceNode.textContent = projectionText(data?.interface);
      const timestampNode = modal.querySelector('[data-tailscale-timestamp]');
      if (timestampNode) timestampNode.textContent = projectionText(data?.timestamp ?? data?.ts);
      const firstMissing = modal.querySelector('[data-tailscale-first-missing-signal]');
      if (firstMissing) firstMissing.textContent = readSignal(data, 'firstMissingSignal') || (data?.ok === true ? 'none' : 'tailscale-status-unavailable');
      const ipNode = modal.querySelector('[data-tailscale-ip]');
      if (ipNode) ipNode.textContent = headerState.isAdmin ? projectionText(data?.ip) : 'Unavailable';
      const enabledNode = modal.querySelector('[data-tailscale-enabled]');
      if (enabledNode) enabledNode.textContent = data?.isEnabled === true ? 'Enabled' : (data?.isEnabled === false ? 'Disabled' : 'Unavailable');
      const enabledSignalNode = modal.querySelector('[data-tailscale-enabled-signal]');
      if (enabledSignalNode) enabledSignalNode.textContent = readSignal(data, 'enabledFirstMissingSignal') || (typeof data?.isEnabled === 'boolean' ? 'none' : 'Unavailable');
      const loginSection = modal.querySelector('[data-tailscale-login-section]');
      const loginLink = modal.querySelector('[data-tailscale-login-url]');
      const copyButton = modal.querySelector('[data-copy-login-url]');
      const loginUnavailable = modal.querySelector('[data-tailscale-login-unavailable]');
      const login = headerState.isAdmin && status === 'needs-login' ? safeHttpUrl(data?.loginUrl) : null;
      if (loginSection) loginSection.hidden = !(headerState.isAdmin && status === 'needs-login');
      if (loginLink) { loginLink.hidden = !login; if (login) { loginLink.href = login; loginLink.textContent = login; } else loginLink.removeAttribute('href'); }
      if (copyButton) copyButton.hidden = !login;
      if (loginUnavailable) loginUnavailable.hidden = Boolean(login) || !(headerState.isAdmin && status === 'needs-login');
      if (copyButton && copyButton.dataset.copyBound !== 'true') {
        copyButton.dataset.copyBound = 'true';
        copyButton.addEventListener('click', () => { const href = modal.querySelector('[data-tailscale-login-url]')?.href; if (href) void navigator.clipboard?.writeText(href); });
      }
    }
    function serviceBoolean(value) {
      return typeof value === 'boolean' ? (value ? 'Yes' : 'No') : 'Unavailable';
    }
    function appendServiceFact(parent, label, value) {
      const fact = document.createElement('span');
      fact.className = 'service-status-fact';
      fact.textContent = `${label}: ${projectionText(value)}`;
      parent.appendChild(fact);
    }
    function hydrateServicesModal(data) {
      const status = typeof data?.status === 'string' ? data.status : 'unavailable';
      const statusNode = infoBody.querySelector('[data-modal-kind-body="services"] [data-modal-status]');
      const signal = readSignal(data, 'firstMissingSignal');
      const successful = Boolean(data && (data.ok === true || data.success === true));
      if (statusNode) {
        const statusClass = status === 'up' ? 'ok' : (status === 'partial' ? 'warn' : (status === 'down' || status === 'unavailable' ? 'error' : 'loading'));
        statusNode.className = 'loading-section ' + statusClass;
        statusNode.textContent = String(status).toUpperCase() + (signal ? ' · ' + signal : '');
      }
      const list = infoBody.querySelector('[data-modal-kind-body="services"] .service-status-list');
      const services = Array.isArray(data?.services) ? data.services : [];
      if (list) {
        list.replaceChildren();
        if (!successful || !services.length) {
          const item = document.createElement('li');
          item.textContent = !successful ? 'Services status unavailable: ' + (signal || 'services-status-unavailable') : 'No service rows returned.';
          list.appendChild(item);
        }
        services.forEach(service => {
          const item = document.createElement('li');
          const serviceStatus = ['up', 'down', 'partial', 'unknown'].includes(service?.status) ? service.status : 'unknown';
          item.className = 'service-status-item ' + serviceStatus;
          const content = document.createElement('div');
          content.className = 'service-item-content';
          const identity = document.createElement('div');
          identity.className = 'service-identity';
          const name = document.createElement('strong');
          name.className = 'service-name';
          name.textContent = projectionText(service?.name ?? service?.systemdName);
          const description = document.createElement('span');
          description.className = 'service-description';
          description.textContent = projectionText(service?.description);
          identity.append(name, description);
          const state = document.createElement('span');
          state.className = 'service-state';
          state.textContent = projectionText(service?.status);
          content.append(identity, state);
          item.appendChild(content);
          const facts = document.createElement('div');
          facts.className = 'service-status-facts';
          appendServiceFact(facts, 'Unit', service?.systemdName);
          appendServiceFact(facts, 'Details', service?.statusDetails);
          appendServiceFact(facts, 'Active', typeof service?.isActive === 'boolean' ? service.isActive : null);
          appendServiceFact(facts, 'Managed by script', typeof service?.isScriptManaged === 'boolean' ? service.isScriptManaged : null);
          appendServiceFact(facts, 'Port', service?.port);
          appendServiceFact(facts, 'Needs reboot', typeof service?.needsReboot === 'boolean' ? service.needsReboot : null);
          const rowSignal = readSignal(service, 'firstMissingSignal');
          if (rowSignal) appendServiceFact(facts, 'First missing signal', rowSignal);
          if (headerState.isAdmin) {
            const enabled = typeof service?.enabled === 'boolean' ? service.enabled : service?.isEnabled;
            appendServiceFact(facts, 'Enabled', typeof enabled === 'boolean' ? enabled : null);
            const enabledSignal = readSignal(service, 'enabledFirstMissingSignal');
            if (enabledSignal) appendServiceFact(facts, 'Enablement signal', enabledSignal);
          }
          item.appendChild(facts);
          list.appendChild(item);
        });
      }
      const adminSummary = infoBody.querySelector('[data-services-enable-summary]');
      if (adminSummary) {
        const rows = services.map(service => typeof service?.enabled === 'boolean' ? service.enabled : service?.isEnabled).filter(value => typeof value === 'boolean');
        const missing = services.length - rows.length;
        adminSummary.textContent = successful ? `Enabled: ${rows.filter(Boolean).length}; disabled: ${rows.filter(value => !value).length}; unavailable: ${missing}.` : 'Enablement unavailable: ' + (signal || 'services-status-unavailable');
      }
      const unitsNode = infoBody.querySelector('[data-service-units]');
      if (unitsNode) unitsNode.textContent = headerState.isAdmin && Array.isArray(data?.units) ? JSON.stringify(data.units, null, 2) : 'Unavailable';
      setServicesIndicatorState(data);
    }
    function modalRequestBody(button) {
      const route = button.dataset.modalFetch || '';
      if (route.endsWith('/update-tailnet')) return JSON.stringify({ name: infoBody.querySelector('[data-tailnet-input]')?.value || '' });
      if (route.endsWith('/authkey')) {
        const input = infoBody.querySelector('[data-authkey-input]');
        const key = input?.value || '';
        if (input) input.value = '';
        return JSON.stringify({ key });
      }
      if (route === '/api/service/control' && button.dataset.serviceAction) return JSON.stringify({ service: 'transmissionVPN', action: button.dataset.serviceAction });
      if (route === '/api/transmission/keys/replace') {
        const username = infoBody.querySelector('[data-transmission-username]')?.value || '';
        const passwordInput = infoBody.querySelector('[data-transmission-password]');
        const password = passwordInput?.value || '';
        if (passwordInput) passwordInput.value = '';
        return JSON.stringify({ service: 'pia', username, password });
      }
      if (route === '/api/transmission/keys/rotate') return JSON.stringify({ service: 'transmission' });
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
      const generation = infoModalGeneration;
      const expectedModal = infoBody.querySelector('[data-modal-kind-body]');
      const expectedAdmin = headerState.isAdmin;
      const nodes = [...infoBody.querySelectorAll('[data-route-read]')];
      await Promise.all(nodes.map(async node => {
        const route = node.dataset.routeRead;
        try {
          const response = await fetch(route, { cache: 'no-store' });
          const data = await response.json();
          if (!modalResponseIsCurrent(generation, expectedModal, expectedAdmin) || !infoBody.contains(node)) return;
          const servicesRoute = kind === 'services' && route === '/api/status/services';
          const tailscaleStatusRoute = kind === 'tailscale' && route === '/api/status/tailscale';
          const tailscaleConfigRoute = kind === 'tailscale' && route === '/api/status/tailscale/config';
          const transmissionRoute = kind === 'openvpn' && (route === '/api/status/vpn/pia' || route === '/api/status/vpn/transmission');
          const powerRoute = kind === 'power-meter' && route === '/api/status/power/usage';
          const failed = routeReadWasRefused(response, data);
          const readData = failed ? unavailableReadData(data, routeUnavailableSignal(route)) : data;
          const powerData = failed ? unavailablePowerUsage(data, routeUnavailableSignal(route)) : data;
          if (tailscaleStatusRoute) hydrateTailscaleModal(readData);
          else if (servicesRoute) hydrateServicesModal(readData);
          else if (transmissionRoute) hydrateTransmissionModal(readData, expectedModal);
          if (powerRoute) {
            setPowerUsage(powerData);
            renderPowerModal();
          }
          if (tailscaleConfigRoute) {
            const input = expectedModal?.querySelector('[data-tailnet-input]');
            if (!failed && data?.ok === true && typeof data.tailnet === 'string' && input && !input.dataset.userEdited) input.value = data.tailnet;
          }
          const labelData = powerRoute ? powerData : readData;
          const label = routeReadLabel(route, labelData);
          if (servicesRoute) {}
          else if (node.matches('ul')) node.innerHTML = `<li>${label}</li>`;
          else if (node.classList.contains('power-value')) {
            const statusNode = node.querySelector('[data-modal-status]');
            if (statusNode) statusNode.textContent = failed ? powerData.firstMissingSignal : label.replace('Power readback: ', '').replace('Power readback unavailable', 'unavailable');
          } else if (!transmissionRoute && !tailscaleStatusRoute) node.textContent = label;
          if (route === '/api/status') {
            node.classList.remove('loading', 'connected', 'disconnected', 'error', 'unavailable');
            node.classList.add(internetState.status || 'loading');
          }
          node.classList.remove('loading');
          node.dataset.hydrated = 'true';
        } catch (_) {
          if (!modalResponseIsCurrent(generation, expectedModal, expectedAdmin) || !infoBody.contains(node)) return;
          const signal = routeUnavailableSignal(route);
          const unavailable = unavailableReadData(null, signal);
          const fallback = 'Status unavailable: ' + route;
          const servicesRoute = kind === 'services' && route === '/api/status/services';
          const tailscaleStatusRoute = kind === 'tailscale' && route === '/api/status/tailscale';
          const transmissionRoute = kind === 'openvpn' && (route === '/api/status/vpn/pia' || route === '/api/status/vpn/transmission');
          const powerRoute = kind === 'power-meter' && route === '/api/status/power/usage';
          if (tailscaleStatusRoute) hydrateTailscaleModal(unavailable);
          else if (servicesRoute) hydrateServicesModal(unavailable);
          else if (transmissionRoute) hydrateTransmissionModal(unavailable, expectedModal);
          else if (node.matches('ul')) node.innerHTML = `<li>${fallback}</li>`;
          else if (powerRoute || node.classList.contains('power-value')) {
            const powerUnavailable = unavailablePowerUsage(null, signal);
            setPowerUsage(powerUnavailable);
            renderPowerModal();
            const statusNode = node.querySelector('[data-modal-status]');
            if (statusNode) statusNode.textContent = powerUnavailable.firstMissingSignal;
          } else if (route === '/api/status') {
            setInternetIndicatorState(unavailable);
            node.textContent = internetStatusModalText();
            node.classList.remove('loading', 'connected', 'disconnected', 'error');
            node.classList.add('unavailable');
          } else node.textContent = fallback;
          node.classList.remove('loading');
          node.dataset.hydrated = 'false';
        }
      }));
      if (kind === 'internet' && modalResponseIsCurrent(generation, expectedModal, expectedAdmin)) {
        const statusNode = infoBody.querySelector('[data-modal-status][data-route-read="/api/status"]');
        if (statusNode) statusNode.textContent = internetStatusModalText();
      }
    }
    function redactTailscaleAuthkeyReceipt(value, secret) {
      if (typeof value === 'string') return secret ? value.split(secret).join('[redacted]') : value;
      if (Array.isArray(value)) return value.map(item => redactTailscaleAuthkeyReceipt(item, secret));
      if (value && typeof value === 'object') {
        return Object.fromEntries(Object.entries(value).map(([name, item]) => [name, /^(auth.?key|key)$/i.test(name) ? '[redacted]' : redactTailscaleAuthkeyReceipt(item, secret)]));
      }
      return value;
    }
    function setTailscaleActionLogin(modal, value, secret = '') {
      const section = modal?.querySelector('[data-tailscale-action-login-section]');
      const link = modal?.querySelector('[data-tailscale-action-login-url]');
      const raw = typeof value === 'string' ? value : '';
      const safe = secret && raw.includes(secret) ? null : safeHttpUrl(raw);
      if (section) section.hidden = !safe;
      if (link) { if (safe) { link.href = safe; link.textContent = safe; } else { link.removeAttribute('href'); link.textContent = ''; } }
    }
    function wireModalFetches() {
      infoBody.querySelectorAll('[data-modal-fetch]').forEach(button => button.addEventListener('click', async () => {
        const output = infoBody.querySelector('[data-modal-output]');
        if (!headerState.isAdmin && button.closest('[data-admin-only]')) { if (output) output.textContent = 'Enter Admin Mode'; return; }
        const route = button.dataset.modalFetch || '';
        const requiresAdmin = Boolean(button.closest('[data-admin-only]')) || route.startsWith('/api/transmission/keys') || route === '/api/service/control';
        if (!headerState.isAdmin && requiresAdmin) { if (output) output.textContent = 'Enter Admin Mode'; return; }
        const originalLabel = button.textContent;
        const isSpeedTest = button.hasAttribute('data-speed-test-button');
        const isTransmissionKeysMutation = route === '/api/transmission/keys/replace' || route === '/api/transmission/keys/rotate';
        const isTailscaleAction = route.startsWith('/api/status/tailscale/') && route !== '/api/status/tailscale/config';
        let requestSecret = '';
        const requestGeneration = infoModalGeneration;
        const requestModal = infoBody.querySelector('[data-modal-kind-body]');
        const expectedAdmin = headerState.isAdmin;
        const isCurrent = () => modalResponseIsCurrent(requestGeneration, requestModal, expectedAdmin);
        if (isSpeedTest) {
          internetState.isSpeedTesting = true;
          internetState.speedTestResults = null;
          internetState.speedTestError = null;
          button.textContent = 'Running Speed Test...';
        } else if (button.dataset.operationLabel) button.textContent = button.dataset.operationLabel;
        button.classList.add('pending-operation');
        button.disabled = true;
        if (output) output.textContent = isTransmissionKeysMutation ? 'Submitting key operation…' : ('Loading ' + route + '…');
        let requestBody;
        try {
          if (route.endsWith('/authkey')) requestSecret = infoBody.querySelector('[data-authkey-input]')?.value || '';
          requestBody = modalRequestBody(button);
          const response = await fetch(route, { method: button.dataset.method || 'GET', headers: requestBody ? { 'Content-Type': 'application/json' } : undefined, body: requestBody });
          const text = await response.text();
          requestBody = undefined;
          let parsed = null;
          try { parsed = JSON.parse(text); } catch (_) {}
          if (!isCurrent()) return;
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
          if (isTransmissionKeysMutation) {
            const signal = typeof parsed?.firstMissingSignal === 'string' && /^[A-Za-z0-9._:-]{1,160}$/.test(parsed.firstMissingSignal) ? parsed.firstMissingSignal : 'transmission-keys-refused';
            const success = response.ok && (parsed?.ok === true || parsed?.accepted === true);
            if (output) output.textContent = success ? 'Key operation accepted.' : signal;
            if (success && expectedAdmin) void hydrateTransmissionAdminReads(requestGeneration, requestModal);
          } else if (route === '/api/service/control' && parsed && isCurrent()) {
            if (requestModal?.dataset.modalKindBody === 'openvpn') renderTransmissionServiceStatus(requestModal, parsed);
            if (output) output.textContent = parsed?.firstMissingSignal && parsed.firstMissingSignal !== 'none' ? String(parsed.firstMissingSignal) : (response.ok ? 'Service action completed.' : 'Service action refused.');
            if (expectedAdmin && button.dataset.serviceAction !== 'status') void hydrateTransmissionAdminReads(requestGeneration, requestModal);
          } else if (isTailscaleAction) {
            const refusalSignal = readSignal(parsed, 'firstMissingSignal', 'first_missing_signal');
            const explicitSuccess = parsed?.ok === true || parsed?.accepted === true || parsed?.success === true;
            const success = response.ok && explicitSuccess && parsed?.ok !== false && parsed?.accepted !== false && parsed?.success !== false && parsed?.refused !== true && !refusalSignal;
            const action = route.split('/').pop() || 'action';
            const safeReceipt = redactTailscaleAuthkeyReceipt(parsed, requestSecret);
            const safeSignal = requestSecret ? String(refusalSignal || parsed?.error || parsed?.refusal || '').split(requestSecret).join('[redacted]') : (refusalSignal || parsed?.error || parsed?.refusal || 'tailscale-action-refused');
            if (output) output.textContent = `${success ? 'Accepted' : 'Refused'} Tailscale ${action}${success ? '' : ': ' + safeSignal}\n${safeReceipt ? JSON.stringify(safeReceipt, null, 2) : (requestSecret ? text.split(requestSecret).join('[redacted]') : text)}`;
            if (action === 'connect') setTailscaleActionLogin(requestModal, success ? parsed?.loginUrl : null, requestSecret);
            requestSecret = '';
          } else if (output) {
            output.textContent = parsed ? JSON.stringify(parsed, null, 2) : text;
          }
        } catch (error) {
          if (!isCurrent()) return;
          if (isSpeedTest) {
            internetState.speedTestError = error?.message || 'Speed test failed unexpectedly.';
            internetState.isSpeedTesting = false;
            openInfoModal('Internet Status', 'internet');
            return;
          }
          if (output) output.textContent = isTransmissionKeysMutation ? 'transmission-keys-request-failed' : ('fetch failed: ' + (error?.message || 'request-failed'));
        } finally {
          requestBody = undefined;
          requestSecret = '';
          if (isSpeedTest) internetState.isSpeedTesting = false;
          if (isCurrent()) {
            button.textContent = originalLabel;
            button.classList.remove('pending-operation');
            button.disabled = false;
          }
        }
      }));
    }
    function openInfoModal(title, kind = 'status') {
      if (infoBody.querySelector('[data-modal-kind-body="power-meter"]')) cancelPowerCalibrationEditing();
      sourceCurrencyModalGeneration += 1;
      infoModalGeneration += 1;
      clearTransmissionSecrets();
      const modalGeneration = sourceCurrencyModalGeneration;
      infoTitle.textContent = title;
      infoBody.innerHTML = modalTemplate(kind);
      themeChoiceRow.hidden = kind !== 'theme';
      infoBackdrop.classList.add('open');
      infoBackdrop.setAttribute('aria-hidden', 'false');
      wireModalFetches();
      hydrateModalRouteReads(kind);
      if (kind === 'tailscale') {
        const tailnetInput = infoBody.querySelector('[data-tailnet-input]');
        tailnetInput?.addEventListener('input', () => { tailnetInput.dataset.userEdited = 'true'; });
      }
      if (kind === 'openvpn' && headerState.isAdmin) {
        void hydrateTransmissionAdminReads(infoModalGeneration, infoBody.querySelector('[data-modal-kind-body="openvpn"]'));
      }
      if (kind === 'source-currency') {
        setSourceCurrencyIndicatorState(sourceCurrencyUpdateState.latestEnvelope);
        void refreshSourceCurrencyUpdateServiceStatus(modalGeneration);
      }
      if (kind === 'power-meter') {
        const generation = infoModalGeneration;
        const modal = infoBody.querySelector('[data-modal-kind-body="power-meter"]');
        wirePowerCalibrationControls(generation, modal);
        void loadPowerUsage(generation, modal, headerState.isAdmin);
        if (headerState.isAdmin) void loadPowerCalibration(generation, modal);
        renderPowerModal();
      }
    }
    function closeInfoModal() {
      if (infoBody.querySelector('[data-modal-kind-body="power-meter"]')) cancelPowerCalibrationEditing();
      sourceCurrencyModalGeneration += 1;
      infoModalGeneration += 1;
      clearTransmissionSecrets();
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
    changePinButton?.addEventListener('click', () => openPinModal('change'));
    document.querySelector('[data-pin-cancel]')?.addEventListener('click', closePinModal);
    document.querySelector('[data-pin-confirm-button]')?.addEventListener('click', async () => {
      if (modalMode === 'change') {
        if (!changeCurrentPinInput.value || !newPinInput.value || !confirmPinInput.value) { modalMessage.textContent = 'Please fill in all fields'; return; }
        if (newPinInput.value !== confirmPinInput.value) { modalMessage.textContent = 'New PINs do not match'; return; }
        try { const response = await fetch('/api/v1/attendance/change-pin', { method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store', body: JSON.stringify({ currentPin: changeCurrentPinInput.value, newPin: newPinInput.value }) }); const result = await response.json().catch(() => ({}));
          if (!response.ok || result.ok !== true) { const returnedError = result?.error || result?.message || (result?.firstMissingSignal !== 'none' ? result?.firstMissingSignal : ''); modalMessage.textContent = returnedError || 'Failed to change PIN'; return; }
        } catch (_) { modalMessage.textContent = 'Failed to change PIN'; return; }
        [changeCurrentPinInput, newPinInput, confirmPinInput].forEach(input => { input.value = ''; }); modalMessage.textContent = 'PIN changed successfully'; window.setTimeout(closePinModal, 1000); return;
      }
      if (modalMode === 'enter' && !currentPinInput.value) { modalMessage.textContent = 'Enter PIN'; return; }
      if (modalMode === 'enter') {
        try {
          const response = await fetch('/api/v1/attendance/open', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ pin: currentPinInput.value }) });
          const result = await response.json().catch(() => ({}));
          const explicitPinRefusal = response.status === 401 && result?.firstMissingSignal === 'caduceus-attendance-refused';
          if (explicitPinRefusal) { modalMessage.textContent = 'Invalid PIN'; return; }
          if (!response.ok || result.admin !== true) { modalMessage.textContent = 'PIN check unavailable'; return; }
          coronatioAttendanceRuntime.currentAttendance = typeof result.attendance === 'string' ? result.attendance : null;
          if (!coronatioAttendanceRuntime.currentAttendance || !installAdminDocumentPatch(result.adminPatch, result.adminTabs)) { coronatioAttendanceRuntime.currentAttendance = null; modalMessage.textContent = 'PIN check unavailable'; return; }
        } catch (_) { modalMessage.textContent = 'PIN check unavailable'; return; }
      }
      modalMessage.textContent = '✓ Admin access confirmed. Loading…';
      await new Promise(resolve => requestAnimationFrame(resolve));
      const admitted = await setAdminMode(true, { admission: true });
      if (admitted) closePinModal();
    });
    async function enterInactivityHeadless() {
      if (coronatioAttendanceRuntime.inactivityHeadless) return;
      coronatioAttendanceRuntime.inactivityHeadless = true;
      try {
        if (coronatioAttendanceRuntime.currentAttendance) await fetch('/api/v1/attendance/invalidate', { method: 'POST', cache: 'no-store' });
      } catch (_) { /* Browser projection still becomes guest when invalidation is unavailable. */ }
      coronatioAttendanceRuntime.currentAttendance = null;
      removeAdminDocumentPatch();
      if (headerState.isAdmin) { headerState.isAdmin = false; saveHeaderState(); applyAdminDomState(); }
      [pulseStream, coreStream].forEach(stream => { try { stream?.close(); } catch (_) {} });
      pulseStream = null; pulseStreamId = null; coreStream = null; coreStreamId = null;
      document.documentElement.dataset.connectionState = 'headless';
      showPane('headless');
    }
    if (!coronatioAttendanceRuntime.recordEligibleActivity) {
      const ATTENDANCE_TOUCH_THROTTLE_MS = 60 * 1000;
      coronatioAttendanceRuntime.recordEligibleActivity = () => { const now = Date.now(); coronatioAttendanceRuntime.lastEligibleActivity = now; if (!coronatioAttendanceRuntime.currentAttendance || coronatioAttendanceRuntime.inactivityHeadless || now - coronatioAttendanceRuntime.lastAttendanceTouch < ATTENDANCE_TOUCH_THROTTLE_MS) return; coronatioAttendanceRuntime.lastAttendanceTouch = now; void fetch('/api/v1/attendance/touch', { method: 'POST', cache: 'no-store' }).catch(() => {}); };
      const activityCensusListener = event => { if (event.isTrusted === false) return; coronatioAttendanceRuntime.recordEligibleActivity(); };
      ['click', 'pointerdown', 'touchstart', 'keydown', 'input', 'scroll'].forEach(type => document.addEventListener(type, activityCensusListener, { capture: true, passive: true }));
      coronatioAttendanceRuntime.inactivityInterval = window.setInterval(() => { if (Date.now() - coronatioAttendanceRuntime.lastEligibleActivity >= 15 * 60 * 1000) enterInactivityHeadless(); }, 60 * 1000);
      coronatioAttendanceRuntime.activityCensusInstallCount++;
    }
    let pulseStream = null;
    let pulseRenewTimer = null;
    let pulseStreamId = null;
    let pulseStaleTimer = null;
    let pulseLastActivityAt = 0;
    let coreStream = null;
    let coreRenewTimer = null;
    let coreStreamId = null;
    const sseReconnectBaseDelayMs = 1000;
    const sseReconnectMaxDelayMs = 30000;
    let coreReconnectDelayMs = sseReconnectBaseDelayMs;
    let pulseReconnectDelayMs = sseReconnectBaseDelayMs;
    const pulseStaleTimeoutMs = 20000;
    async function setStreamMembership(family, streamId, action) {
      if (!streamId) return null;
      return fetch(`/api/${family}/pulse/${action}?streamId=${encodeURIComponent(streamId)}`, { method: 'POST', cache: 'no-store' });
    }
    async function upgradeOpenStreams() {
      const requests = [];
      if (pulseStreamId) requests.push(setStreamMembership('stats', pulseStreamId, 'upgrade'));
      if (coreStreamId) requests.push(setStreamMembership('core', coreStreamId, 'upgrade'));
      return Promise.all(requests);
    }
    async function downgradeOpenStreams() {
      const requests = [];
      if (pulseStreamId) requests.push(setStreamMembership('stats', pulseStreamId, 'downgrade'));
      if (coreStreamId) requests.push(setStreamMembership('core', coreStreamId, 'downgrade'));
      return Promise.all(requests);
    }
    const coreTopicIds = ['internet.status', 'tailscale.status', 'vpn.status', 'services.status', 'power.status', 'source.currency'];
    function applyCoreTopic(topicId, envelope) {
      if (topicId === 'source.currency') {
        setSourceCurrencyIndicatorState(envelope);
        return;
      }
      const snapshotData = envelope?.snapshot;
      const snapshot = envelope?.status === 'snapshot' && snapshotData && typeof snapshotData === 'object' && !Array.isArray(snapshotData);
      const fallbackSignal = ({
        'internet.status': 'internet-status-unavailable',
        'tailscale.status': 'tailscale-status-unavailable',
        'vpn.status': 'vpn-status-unavailable',
        'services.status': 'services-status-unavailable',
        'power.status': 'power-usage-unavailable'
      })[topicId] || 'core-status-unavailable';
      const firstMissingSignal = readSignal(envelope, 'firstMissingSignal', 'first_missing_signal') || readSignal(snapshotData, 'firstMissingSignal', 'first_missing_signal') || fallbackSignal;
      const data = snapshot
        ? snapshotData
        : { ok: false, success: false, status: 'unavailable', firstMissingSignal };
      if (topicId === 'vpn.status') {
        if (infoBackdrop.classList.contains('open')) hydrateTransmissionModal(data);
        setVpnIndicatorState(data, snapshot ? envelope?.indicatorState : null);
      }
      if (topicId === 'internet.status') setInternetIndicatorState(data);
      if (topicId === 'tailscale.status') {
        setTailscaleIndicatorState(data);
        if (infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="tailscale"]')) hydrateTailscaleModal(data);
      }
      if (topicId === 'services.status') {
        setServicesIndicatorState(data);
        if (infoBackdrop.classList.contains('open') && infoBody.querySelector('[data-modal-kind-body="services"]')) hydrateServicesModal(data);
      }
      if (topicId === 'power.status') {
        const powerData = snapshot ? data : unavailablePowerUsage(snapshotData, fallbackSignal, firstMissingSignal);
        setPowerUsage(powerData);
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
        coreStreamId = data.streamId || event.lastEventId || null;
        coreReconnectDelayMs = sseReconnectBaseDelayMs;
        scheduleCoreRenewal(data.renewRoute);
        if (headerState.isAdmin) void setStreamMembership('core', coreStreamId, 'upgrade');
      });
      coreTopicIds.forEach(topicId => coreStream.addEventListener(topicId, event => {
        try { applyCoreTopic(topicId, JSON.parse(event.data || '{}')); } catch (_) {}
      }));
      coreStream.addEventListener('core.expired', () => {
        coreStream.close(); coreStream = null; coreStreamId = null;
        const delayMs = coreReconnectDelayMs;
        coreReconnectDelayMs = Math.min(coreReconnectDelayMs * 2, sseReconnectMaxDelayMs);
        window.setTimeout(connectCoreStream, delayMs);
      });
    }
    loadThemeCatalog();
    bootstrapAdminMode();
    connectCoreStream();
    function eligibleRegularTabs() { return tabs.filter(tab => tab.dataset.visibility !== 'hidden' && tab.dataset.adminOnly !== 'true'); }
    function visibleTabs() { return headerState.isAdmin ? tabs.filter(tab => tab.dataset.pane !== fallbackTab) : eligibleRegularTabs(); }
    function firstVisibleTab() { return eligibleRegularTabs()[0]?.dataset.pane || fallbackTab; }
    function currentActiveTabId() { if (coronatioAttendanceRuntime.inactivityHeadless) return 'headless'; return tabs.find(tab => tab.getAttribute('aria-selected') === 'true')?.dataset.pane || fallbackTab; }
    function canStarTab(id) { return eligibleRegularTabs().some(tab => tab.dataset.pane === id); }
    function sessionLawfulTab(id) {
      const tab = tabs.find(candidate => candidate.dataset.pane === id);
      return Boolean(tab && tab.dataset.visibility !== 'hidden' && (tab.dataset.adminOnly !== 'true' || headerState.isAdmin));
    }
    function lawfulPaneCandidate(id) {
      if (id === 'headless' && coronatioAttendanceRuntime.inactivityHeadless) return id;
      return sessionLawfulTab(id) ? id : firstVisibleTab();
    }
    function applyTabBarVisibility() {
      if (!tabBar) return;
      const selected = currentActiveTabId();
      const hidden = coronatioAttendanceRuntime.inactivityHeadless || (!headerState.isAdmin && (selected === fallbackTab || eligibleRegularTabs().length <= 2));
      tabBar.classList.toggle('hidden', hidden);
      tabBar.dataset.hidden = String(hidden);
    }
    function paintStarredTab(id) {
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
    function setStarredTab(id) {
      if (!canStarTab(id)) return;
      tabState.starredTab = id;
      saveTabState(tabState);
      paintStarredTab(id);
    }
    function browserFavoriteTab() {
      try { const saved = JSON.parse(localStorage.getItem(storageKey) || '{}'); return typeof saved.starredTab === 'string' ? saved.starredTab : null; }
      catch (_) { return null; }
    }
    function renderedApplianceFavoriteTab() { return tabs.find(tab => tab.querySelector('[data-tab-star][aria-pressed="true"]'))?.dataset.pane || null; }
    function paintEffectiveFavoriteTab(applianceFavorite = renderedApplianceFavoriteTab()) {
      const browserFavorite = browserFavoriteTab();
      const effective = canStarTab(browserFavorite) ? browserFavorite : (canStarTab(applianceFavorite) ? applianceFavorite : null);
      if (effective) paintStarredTab(effective);
      return effective;
    }
    async function refreshTabBar(activeTabId = currentActiveTabId()) {
      if (!tabBar) return;
      const activeParam = activeTabId ? '?active=' + encodeURIComponent(activeTabId) : '';
      const response = await fetch('/api/tab-bar' + activeParam);
      if (!response.ok) return null;
      replaceTabBar(await response.text());
      return currentActiveTabId();
    }
    function reconcileAdmittedPaneHosts() { if (!immortalFloorGuestSlot) return; const declaredPaneIds = new Set(tabs.map(tab => tab.dataset.pane).filter(Boolean));
      panes.filter(pane => pane.dataset.admittedPaneHost === 'true' && !declaredPaneIds.has(pane.dataset.panePanel)).forEach(pane => pane.remove()); for (const tab of tabs) { const id = tab.dataset.pane; if (!id || panes.some(pane => pane.dataset.panePanel === id)) continue;
        const pane = document.createElement('section'); pane.className = 'pane'; pane.id = 'pane-' + id;
        pane.dataset.panePanel = id; pane.dataset.viewPanel = id; pane.dataset.admittedPaneHost = 'true'; pane.dataset.immortalFloorLayer = '2';
        pane.setAttribute('role', 'tabpanel'); pane.setAttribute('aria-label', tab.querySelector('.tab-name')?.textContent?.trim() || id); pane.setAttribute('aria-hidden', 'true'); immortalFloorGuestSlot.appendChild(pane); }
      panes = [...document.querySelectorAll('[data-pane-panel]')]; }
    function replaceTabBar(html) { if (!tabBar) return;
      tabBar.innerHTML = html;
      tabs = [...document.querySelectorAll('[data-pane]')];
      reconcileAdmittedPaneHosts();
      if (window.htmx) window.htmx.process(tabBar);
      bindTabControls();
      paintEffectiveFavoriteTab();
      applyTabBarVisibility();
    }
    function clearPulseRenewal() {
      if (pulseRenewTimer) window.clearTimeout(pulseRenewTimer);
      pulseRenewTimer = null;
    }
    function viewportFamilyAdmitted(id) {
      return Boolean(
        document.querySelector(`[data-pane-panel="${id}"]`) &&
        window.getImmortalFloorState?.() === 'Seated' &&
        document.visibilityState === 'visible' &&
        currentActiveTabId() === id
      );
    }
    const scaffoldSlotStructures = new WeakMap();
    const scaffoldDynamicAttributes = new Set(['data-scaffold-state', 'data-scaffold-pending', 'data-scaffold-error', 'data-scaffold-stale', 'data-scaffold-updated-at', 'aria-busy', 'aria-invalid', 'aria-selected', 'aria-checked', 'aria-pressed', 'checked', 'selected', 'value', 'open']);
    function scaffoldStructureSignature(node) {
      if (node.nodeType === Node.TEXT_NODE) return '#text';
      if (node.nodeType === Node.COMMENT_NODE) return '#comment';
      if (node.nodeType !== Node.ELEMENT_NODE) return `#${node.nodeType}`;
      const attrs = Array.from(node.attributes)
        .filter(attribute => !scaffoldDynamicAttributes.has(attribute.name) && attribute.name !== 'data-scaffold-status')
        .map(attribute => [attribute.name, attribute.value])
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([name, value]) => `${name}=${JSON.stringify(value)}`).join(';');
      const children = Array.from(node.childNodes)
        .filter(child => !(child.nodeType === Node.ELEMENT_NODE && child.hasAttribute('data-scaffold-status')))
        .map(scaffoldStructureSignature).join('|');
      return `<${node.tagName}[${attrs}]>${children}</${node.tagName}>`;
    }
    function rememberScaffoldSlotStructures(root) {
      root?.querySelectorAll('[data-scaffold-slot]').forEach(slot => {
        if (!scaffoldSlotStructures.has(slot)) scaffoldSlotStructures.set(slot, scaffoldStructureSignature(slot));
      });
    }
    function morphLivePane(target, html) {
      const template = document.createElement('template');
      template.innerHTML = html;
      let mutated = false;
      const sameKind = (left, right) => left.nodeType === right.nodeType && (left.nodeType !== Node.ELEMENT_NODE || left.tagName === right.tagName);
      const nodeKey = node => {
        if (node.nodeType !== Node.ELEMENT_NODE) return null;
        if (node.id) return `id:${node.id}`;
        if (node.hasAttribute('data-scaffold-slot')) return `slot:${node.dataset.scaffoldSlot}`;
        if (node.hasAttribute('data-scaffold-key')) return `scaffold:${node.dataset.scaffoldKey}`;
        if (node.hasAttribute('data-stat-element-id')) return `stat:${node.dataset.statElementId}`;
        return null;
      };
      const morphAttributes = (current, next) => {
        Array.from(current.attributes).forEach(attribute => {
          if (!next.hasAttribute(attribute.name)) { current.removeAttribute(attribute.name); mutated = true; }
        });
        Array.from(next.attributes).forEach(attribute => {
          if (current.getAttribute(attribute.name) !== attribute.value) { current.setAttribute(attribute.name, attribute.value); mutated = true; }
        });
      };
      const morphChildren = (current, next) => {
        const previous = Array.from(current.childNodes);
        const keyed = new Map(previous.map(child => [nodeKey(child), child]).filter(([key]) => key));
        const used = new Set();
        Array.from(next.childNodes).forEach((nextChild, index) => {
          const key = nodeKey(nextChild);
          let currentChild = key ? keyed.get(key) : previous[index];
          if (currentChild && used.has(currentChild)) currentChild = null;
          if (!currentChild) {
            currentChild = nextChild.cloneNode(true);
            current.insertBefore(currentChild, current.childNodes[index] || null);
            mutated = true;
            if (currentChild.nodeType === Node.ELEMENT_NODE && currentChild.hasAttribute('data-scaffold-slot')) scaffoldSlotStructures.set(currentChild, scaffoldStructureSignature(nextChild));
            used.add(currentChild);
            return;
          }
          if (currentChild !== current.childNodes[index]) { current.insertBefore(currentChild, current.childNodes[index] || null); mutated = true; }
          const result = morphNode(currentChild, nextChild);
          used.add(result);
        });
        Array.from(current.childNodes).forEach(child => {
          if (!used.has(child)) { child.remove(); mutated = true; }
        });
      };
      const morphNode = (current, next) => {
        if (!sameKind(current, next)) {
          const replacement = next.cloneNode(true);
          current.replaceWith(replacement); mutated = true;
          if (replacement.nodeType === Node.ELEMENT_NODE && replacement.hasAttribute('data-scaffold-slot')) scaffoldSlotStructures.set(replacement, scaffoldStructureSignature(next));
          return replacement;
        }
        if (current.nodeType === Node.TEXT_NODE || current.nodeType === Node.COMMENT_NODE) {
          if (current.nodeValue !== next.nodeValue) { current.nodeValue = next.nodeValue; mutated = true; }
          return current;
        }
        if (current.hasAttribute('data-scaffold-slot') && next.hasAttribute('data-scaffold-slot') && current.dataset.scaffoldSlot === next.dataset.scaffoldSlot) {
          const incomingShape = scaffoldStructureSignature(next);
          const liveShape = scaffoldSlotStructures.get(current) || scaffoldStructureSignature(current);
          if (liveShape === incomingShape) {
            scaffoldSlotStructures.set(current, incomingShape);
            return current;
          }
        }
        morphAttributes(current, next);
        // Stats owns chart canvases and live readouts between element-fact pulls. Its
        // element wrapper carries the fact attributes; preserve its live descendants.
        if (current.hasAttribute('data-stat-element-id')) {
          const currentHeader = Array.from(current.children).find(child => child.classList.contains('stat-header'));
          const nextHeader = Array.from(next.children).find(child => child.classList.contains('stat-header'));
          if (currentHeader && nextHeader) morphNode(currentHeader, nextHeader);
          return current;
        }
        morphChildren(current, next);
        if (current.hasAttribute('data-scaffold-slot')) scaffoldSlotStructures.set(current, scaffoldStructureSignature(next));
        return current;
      };
      morphChildren(target, template.content);
      return mutated;
    }
    const scaffoldRegistry = (() => {
      const guests = new Map();
      const bindings = new Map();
      const validStates = new Set(['pending', 'ready', 'fault']);
      const DEFAULT_READ_TIMEOUT_MS = 8000;
      const DEFAULT_SLOW_READ_MS = 750;
      const DEFAULT_BODY_MAX_BYTES = 4 * 1024 * 1024;
      function slotsFor(binding) { return binding.root ? [...binding.root.querySelectorAll('[data-scaffold-slot]')] : []; }
      function rootFor(registration) {
        try { return typeof registration.root === 'function' ? registration.root() : document.querySelector(registration.root); }
        catch (_) { return null; }
      }
      function stateFor(slot) { return validStates.has(slot.dataset.scaffoldState) ? slot.dataset.scaffoldState : 'pending'; }
      function removeStatus(slot) {
        slot.querySelector('[data-scaffold-status]')?.remove();
        slot.removeAttribute('aria-invalid');
        slot.removeAttribute('data-scaffold-error');
        delete slot.dataset.scaffoldStale;
      }
      function statusNode(slot, state, guest, readKey, message, retry = false) {
        let status = slot.querySelector('[data-scaffold-status]');
        if (!status) { status = document.createElement('div'); status.dataset.scaffoldStatus = state; status.className = `scaffold-slot-${state}`; status.setAttribute('role', state === 'fault' ? 'alert' : 'status'); slot.appendChild(status); }
        status.dataset.scaffoldStatus = state;
        status.className = `scaffold-slot-${state}`;
        status.textContent = '';
        const text = document.createElement('span'); text.textContent = message; status.appendChild(text);
        if (retry && readKey) {
          const button = document.createElement('button');
          button.type = 'button'; button.className = 'scaffold-slot-retry';
          button.dataset.scaffoldRetry = guest; button.dataset.scaffoldReadKey = readKey;
          button.textContent = 'Retry'; status.appendChild(button);
        }
      }
      function setPending(slot, key, guest, readKey = null) {
        slot.setAttribute('aria-busy', 'true');
        if (stateFor(slot) === 'ready') {
          slot.dataset.scaffoldStale = 'true';
          statusNode(slot, 'stale', guest, readKey, `Refreshing ${slot.dataset.scaffoldPlaceholder || key || 'content'}; showing the last successful read${slot.dataset.scaffoldUpdatedAt ? ` from ${new Date(Number(slot.dataset.scaffoldUpdatedAt)).toLocaleTimeString()}` : ''}.`, Boolean(readKey));
          return;
        }
        removeStatus(slot);
        slot.dataset.scaffoldState = 'pending';
        slot.dataset.scaffoldPending = 'true';
        if (!slot.hasAttribute('aria-label')) slot.setAttribute('aria-label', 'Loading ' + (slot.dataset.scaffoldPlaceholder || key || 'content'));
      }
      function setReady(slot) {
        removeStatus(slot);
        slot.dataset.scaffoldState = 'ready';
        slot.dataset.scaffoldUpdatedAt = String(Date.now());
        delete slot.dataset.scaffoldPending;
        slot.setAttribute('aria-busy', 'false');
      }
      function setFault(slot, guest, detail, readKey = null) {
        const message = detail || 'Content unavailable';
        if (stateFor(slot) === 'ready') { setStale(slot, guest, message, readKey); return; }
        slot.dataset.scaffoldState = 'fault';
        delete slot.dataset.scaffoldPending;
        slot.setAttribute('aria-busy', 'false');
        slot.setAttribute('aria-invalid', 'true');
        slot.dataset.scaffoldError = message;
        statusNode(slot, 'fault', guest, readKey, `${message}. `, Boolean(readKey));
      }
      function setStale(slot, guest, detail, readKey = null) {
        const message = detail || 'Showing last known data';
        slot.dataset.scaffoldState = 'ready';
        slot.dataset.scaffoldStale = 'true';
        slot.setAttribute('aria-busy', 'false');
        slot.removeAttribute('aria-invalid');
        slot.dataset.scaffoldError = message;
        const age = slot.dataset.scaffoldUpdatedAt ? ` Last successful read: ${new Date(Number(slot.dataset.scaffoldUpdatedAt)).toLocaleTimeString()}.` : '';
        statusNode(slot, 'stale', guest, readKey, `${message}.${age} `, Boolean(readKey));
      }
      function current(binding, run = null, allowTimedOut = false) {
        if (!binding || binding.retired || bindings.get(binding.guest) !== binding || guests.get(binding.guest) !== binding.registration || !(binding.root instanceof Element) || !binding.root.isConnected || document.visibilityState !== 'visible') return false;
        if (typeof viewportFamilyAdmitted === 'function' && !viewportFamilyAdmitted(binding.guest)) return false;
        if (window.immortalFloor?.activeGuest !== binding.guest) return false;
        if (typeof sessionLawfulTab === 'function' && !sessionLawfulTab(binding.guest)) return false;
        try { if (typeof binding.registration.authorize === 'function' && !binding.registration.authorize(binding.guest, binding)) return false; }
        catch (_) { return false; }
        return !run || (!run.retired && (!run.controller.signal.aborted || (allowTimedOut && (run.timedOut || run.externalAbort))) && binding.runs.get(run.key) === run);
      }
      function resolveSlots(binding, selected) {
        binding.slots = slotsFor(binding);
        const values = Array.isArray(selected) ? selected : binding.slots;
        const resolved = values.map(value => value instanceof Element ? value : slot(binding, value)).filter(node => node instanceof Element && binding.root.contains(node));
        return [...new Set(resolved)];
      }
      function stopRun(binding, run, reason = 'Read retired') {
        if (!run || run.retired) return false;
        run.retired = true;
        window.clearTimeout(run.timer); window.clearTimeout(run.slowTimer);
        run.timer = null; run.slowTimer = null;
        if (run.upstream) run.upstream.removeEventListener('abort', run.relay);
        if (binding.runs.get(run.key) === run) binding.runs.delete(run.key);
        try { run.controller.abort(new DOMException(reason, 'AbortError')); } catch (_) { run.controller.abort(); }
        try { run.cancel?.(new DOMException(reason, 'AbortError')); } catch (_) {}
        return true;
      }
      function retireBinding(binding, reason = 'Guest retired') {
        if (!binding || binding.retired) return false;
        binding.retired = true;
        [...binding.runs.values()].forEach(run => stopRun(binding, run, reason));
        [...binding.readOwners.values()].forEach(owner => { try { owner.retire?.(reason); } catch (_) {} });
        [...binding.retireCallbacks].forEach(callback => { try { callback({ guest: binding.guest, binding, reason }); } catch (_) {} });
        binding.retireCallbacks.clear();
        binding.retryTasks.clear(); binding.readOwners.clear(); binding.slots = [];
        if (bindings.get(binding.guest) === binding) bindings.delete(binding.guest);
        binding.root = null; binding.registration = null;
        return true;
      }
      function registerGuest(registration) {
        if (!registration || typeof registration.guest !== 'string' || !registration.root || !(typeof registration.root === 'function' || typeof registration.root === 'string')) return false;
        const active = bindings.get(registration.guest);
        if (active) retireBinding(active, 'Guest registration replaced');
        guests.set(registration.guest, registration);
        return true;
      }
      function hasGuest(guest) { return guests.has(String(guest)); }
      function bind(guest) {
        guest = String(guest);
        let registration = guests.get(guest);
        if (!registration) {
          registration = { guest, root: () => document.querySelector(`[data-pane-panel="${CSS.escape(guest)}"]`), authorize: () => typeof sessionLawfulTab !== 'function' || sessionLawfulTab(guest) };
          guests.set(guest, registration);
        }
        retireBinding(bindings.get(guest), 'Guest binding replaced');
        const root = rootFor(registration);
        if (!(root instanceof Element)) return null;
        const binding = { guest, registration, root, slots: [], runs: new Map(), retireCallbacks: new Set(), retryTasks: new Map(), readOwners: new Map(), retired: false };
        binding.slots = slotsFor(binding);
        rememberScaffoldSlotStructures(binding.root);
        bindings.set(guest, binding);
        if (!current(binding)) { retireBinding(binding, 'Guest is not currently admitted'); return null; }
        binding.slots.forEach(slot => {
          if (!validStates.has(slot.dataset.scaffoldState)) setPending(slot, slot.dataset.scaffoldKey || slot.dataset.scaffoldSlot, guest);
          else if (stateFor(slot) === 'fault') setFault(slot, guest, slot.dataset.scaffoldError || 'Content unavailable');
          else if (stateFor(slot) === 'pending') setPending(slot, slot.dataset.scaffoldKey || slot.dataset.scaffoldSlot, guest);
          else setReady(slot);
        });
        return binding;
      }
      function capture(guest) {
        guest = String(guest);
        let binding = bindings.get(guest);
        if (!binding && window.immortalFloor?.activeGuest === guest) binding = bind(guest);
        return current(binding) ? binding : null;
      }
      function signal(binding, readKey) { return binding?.runs.get(String(readKey || 'default'))?.controller.signal || null; }
      function onRetire(binding, callback) {
        if (typeof callback !== 'function') return () => {};
        if (!current(binding)) { try { callback({ guest: binding?.guest || null, binding, reason: 'Already retired' }); } catch (_) {} return () => {}; }
        binding.retireCallbacks.add(callback);
        return () => binding.retireCallbacks.delete(callback);
      }
      function slot(binding, key, run = null) {
        if (!current(binding, run) || !(binding.root instanceof Element)) return null;
        const value = key instanceof Element ? null : String(key);
        let target = key instanceof Element ? key : binding.root.querySelector(`[data-scaffold-slot="${CSS.escape(value)}"], [data-scaffold-key="${CSS.escape(value)}"]`);
        if (target && run && !run.slots.includes(target)) return null;
        return target;
      }
      function markRunFailure(run, target) { if (run && target) run.failedSlots.add(target); }
      function setSlot(binding, key, state, detail = '', run = null, allowTimedOut = false) {
        if (!current(binding, run, allowTimedOut)) return false;
        const target = slot(binding, key, run);
        if (!target || !binding.root.contains(target)) return false;
        if (state === 'ready') { setReady(target); run?.failedSlots.delete(target); }
        else if (state === 'fault') { setFault(target, binding.guest, safeFailureText(detail, 'Content unavailable'), run?.key); markRunFailure(run, target); }
        else if (state === 'stale') { setStale(target, binding.guest, safeFailureText(detail, 'Showing last known data'), run?.key); markRunFailure(run, target); }
        else setPending(target, key, binding.guest, run?.key || null);
        return true;
      }
      function publish(binding, payload, options = {}) {
        const run = options.run || null;
        if (!current(binding, run)) return false;
        const selected = run ? run.slots : resolveSlots(binding, options.slots);
        selected.forEach(target => {
          if (!current(binding, run) || !binding.root.contains(target)) return;
          const key = target.dataset.scaffoldKey || target.dataset.scaffoldSlot;
          const fillerName = target.dataset.scaffoldFill || key;
          const filler = binding.registration.fillers?.[fillerName];
          if (!filler || typeof filler.present !== 'function' || typeof filler.fill !== 'function') { setFault(target, binding.guest, 'This slot is not configured', run?.key); markRunFailure(run, target); return; }
          let present = false;
          try { present = Boolean(filler.present(payload)); }
          catch (_) { setFault(target, binding.guest, 'Slot data is invalid', run?.key); markRunFailure(run, target); return; }
          if (!present) { setFault(target, binding.guest, 'Required data is unavailable in this response', run?.key); markRunFailure(run, target); return; }
          try {
            if (!current(binding, run)) return;
            filler.fill(payload, { guest: binding.guest, key, slot: target, binding, run });
            setReady(target); run?.failedSlots.delete(target);
          } catch (_) { setFault(target, binding.guest, 'Slot render failed', run?.key); markRunFailure(run, target); }
        });
        return true;
      }
      function failureReason(error) {
        if (error?.name === 'AbortError') return 'Read cancelled';
        const message = String(error?.message || '');
        if (message === 'Read timed out') return message;
        if (message === 'response-body-too-large') return 'Response exceeded the 4 MiB read limit';
        if (message === 'Read returned invalid JSON') return message;
        if (message === 'Read helper accepts bodyless GET only') return message;
        const status = message.match(/^Read failed \(HTTP (\d{3})\)$/);
        if (status) return `Read failed (HTTP ${status[1]})`;
        return 'Read failed';
      }
      function safeFailureText(detail, fallback = 'Read failed') {
        const raw = typeof detail === 'string' ? detail : String(detail?.message || '');
        if (['Content unavailable', 'Showing last known data', 'This slot is not configured', 'Slot data is invalid', 'Required data is unavailable in this response', 'Slot render failed'].includes(raw)) return raw;
        return failureReason(typeof detail === 'string' ? new Error(raw) : detail) || fallback;
      }
      function fail(binding, detail = 'Content unavailable', options = {}) {
        const run = options.run || null;
        const allowTimedOut = options.allowTimedOut === true;
        if (!current(binding, run, allowTimedOut)) return false;
        const selected = run ? run.slots : resolveSlots(binding, options.slots);
        const message = safeFailureText(detail, 'Content unavailable');
        selected.forEach(target => {
          if (!current(binding, run, allowTimedOut) || !binding.root.contains(target)) return;
          if (stateFor(target) === 'ready') setStale(target, binding.guest, message, run?.key);
          else setFault(target, binding.guest, message, run?.key);
          markRunFailure(run, target);
        });
        return true;
      }
      async function responseText(response, signal, maxBytes = DEFAULT_BODY_MAX_BYTES) {
        const declared = Number(response.headers?.get('content-length'));
        if (Number.isFinite(declared) && declared > maxBytes) {
          try { await response.body?.cancel(); } catch (_) {}
          throw new Error('response-body-too-large');
        }
        if (!response.body) return '';
        const reader = response.body.getReader();
        const chunks = [];
        let total = 0;
        const cancelReader = () => { try { void reader.cancel().catch(() => {}); } catch (_) {} };
        if (signal?.aborted) { cancelReader(); throw new DOMException('Read aborted', 'AbortError'); }
        signal?.addEventListener('abort', cancelReader, { once: true });
        try {
          while (true) {
            if (signal?.aborted) throw new DOMException('Read aborted', 'AbortError');
            const part = await reader.read();
            if (part.done) break;
            total += part.value.byteLength;
            if (total > maxBytes) { cancelReader(); throw new Error('response-body-too-large'); }
            chunks.push(part.value);
          }
        } finally {
          signal?.removeEventListener('abort', cancelReader);
          if (signal?.aborted) { try { await reader.cancel(); } catch (_) {} }
        }
        const joined = new Uint8Array(total);
        let offset = 0;
        chunks.forEach(chunk => { joined.set(chunk, offset); offset += chunk.byteLength; });
        return new TextDecoder().decode(joined);
      }
      async function fetchText(binding, url, options = {}) {
        const { run = null, signal: suppliedSignal = null, maxBytes: requestedMax = DEFAULT_BODY_MAX_BYTES, method, body, ...init } = options;
        if (method && String(method).toUpperCase() !== 'GET' || body != null) throw new Error('Read helper accepts bodyless GET only');
        const signal = suppliedSignal || run?.controller.signal;
        if (!current(binding, run)) throw new DOMException('Read retired', 'AbortError');
        const response = await fetch(url, { ...init, method: 'GET', cache: 'no-store', signal });
        if (!current(binding, run)) { try { await response.body?.cancel(); } catch (_) {} throw new DOMException('Read retired', 'AbortError'); }
        if (!response.ok) { try { await response.body?.cancel(); } catch (_) {} throw new Error(`Read failed (HTTP ${response.status})`); }
        const candidate = Number(requestedMax);
        const maxBytes = Number.isSafeInteger(candidate) && candidate > 0 ? Math.min(candidate, DEFAULT_BODY_MAX_BYTES) : DEFAULT_BODY_MAX_BYTES;
        const text = await responseText(response, signal, maxBytes);
        if (!current(binding, run)) throw new DOMException('Read retired', 'AbortError');
        return text;
      }
      async function fetchJson(binding, url, options = {}) {
        const text = await fetchText(binding, url, options);
        try { return JSON.parse(text); }
        catch (_) { throw new Error('Read returned invalid JSON'); }
      }
      function registerRetry(binding, key, task, options = {}) {
        if (!current(binding) || typeof task !== 'function') return false;
        key = String(key);
        if (!binding.retryTasks.has(key) && binding.retryTasks.size >= 128) binding.retryTasks.delete(binding.retryTasks.keys().next().value);
        binding.retryTasks.set(key, { task, options });
        return true;
      }
      function clearRetry(binding, key) {
        if (!binding || binding.retired) return false;
        return binding.retryTasks.delete(String(key));
      }
      function retry(binding, key) {
        if (!current(binding)) return Promise.resolve(false);
        const retryTask = binding.retryTasks.get(String(key));
        if (!retryTask) return Promise.resolve(false);
        return run(binding, String(key), retryTask.task, { ...retryTask.options, supersede: false });
      }
      function run(guestOrBinding, readKey, task, options = {}) {
        const binding = typeof guestOrBinding === 'object' ? guestOrBinding : capture(guestOrBinding);
        if (!current(binding) || typeof task !== 'function') return Promise.resolve(false);
        const key = String(readKey || 'default');
        const existing = binding.runs.get(key);
        if (existing && !options.supersede) return existing.promise;
        if (existing) stopRun(binding, existing, 'Read superseded');
        const controller = new AbortController();
        const timeoutCandidate = Number(options.timeoutMs);
        const timeoutMs = Number.isSafeInteger(timeoutCandidate) && timeoutCandidate > 0 ? Math.min(timeoutCandidate, 30000) : DEFAULT_READ_TIMEOUT_MS;
        const slowCandidate = Number(options.slowMs);
        const slowMs = Number.isSafeInteger(slowCandidate) && slowCandidate > 0 ? Math.min(slowCandidate, Math.max(1, timeoutMs - 1)) : Math.min(DEFAULT_SLOW_READ_MS, Math.max(1, timeoutMs - 1));
        const slots = resolveSlots(binding, options.slots);
        const upstream = options.signal || null;
        if (upstream?.aborted) return Promise.resolve(false);
        const record = { key, controller, timer: null, slowTimer: null, retired: false, timedOut: false, promise: null, slots, failedSlots: new Set(), upstream, relay: null, cancel: null, failureNotified: false };
        const retryOptions = { slots: Array.isArray(options.slots) ? options.slots.slice() : undefined, timeoutMs, slowMs, onFailure: options.onFailure, onRetire: options.onRetire };
        let retryTask = null;
        if (options.retry !== false) {
          if (!registerRetry(binding, key, task, retryOptions)) return Promise.resolve(false);
          retryTask = binding.retryTasks.get(key);
        }
        binding.runs.set(key, record);
        slots.forEach(target => setPending(target, target.dataset.scaffoldKey || target.dataset.scaffoldSlot, binding.guest, key));
        record.relay = () => { record.externalAbort = true; controller.abort(); record.cancel?.(new DOMException('Read cancelled', 'AbortError')); };
        if (upstream) { if (upstream.aborted) record.relay(); else upstream.addEventListener('abort', record.relay, { once: true }); }
        let rejectCancel;
        const cancelled = new Promise((_, reject) => { rejectCancel = reject; });
        record.cancel = rejectCancel;
        const reportFailure = (error, allowTimedOut = false) => {
          if (record.failureNotified || !current(binding, record, allowTimedOut)) return;
          record.failureNotified = true;
          const reason = failureReason(error);
          fail(binding, reason, { run: record, allowTimedOut });
          try {
            options.onFailure?.({ guest: binding.guest, readKey: key, reason, errorName: String(error?.name || 'Error'), slots: record.slots.slice(), setSlot: (target, state = 'fault', detail = reason) => setSlot(binding, target, state, detail, record, allowTimedOut), current: () => current(binding, record, allowTimedOut) });
          } catch (_) {}
        };
        const context = {
          binding, run: record, key, guest: binding.guest, slots: slots.slice(), signal: controller.signal,
          current: () => current(binding, record),
          slot: keyName => slot(binding, keyName, record),
          setSlot: (target, state, detail = '') => setSlot(binding, target, state, detail, record),
          ready: target => setSlot(binding, target, 'ready', '', record),
          fail: detail => { if (!current(binding, record)) return false; reportFailure(detail instanceof Error ? detail : new Error(typeof detail === 'string' ? detail : 'Read failed')); return true; },
          publish: payload => publish(binding, payload, { run: record }),
          fetchJson: (url, fetchOptions = {}) => fetchJson(binding, url, { ...fetchOptions, run: record }),
          fetchText: (url, fetchOptions = {}) => fetchText(binding, url, { ...fetchOptions, run: record })
        };
        const operation = Promise.resolve().then(() => current(binding, record) ? task(context) : false);
        const deadline = new Promise((_, reject) => {
          record.timer = window.setTimeout(() => { record.timedOut = true; controller.abort(); reject(new Error('Read timed out')); }, timeoutMs);
          record.slowTimer = window.setTimeout(() => {
            if (!current(binding, record)) return;
            slots.forEach(target => {
              if (!binding.root.contains(target)) return;
              if (stateFor(target) === 'ready') setPending(target, target.dataset.scaffoldKey || target.dataset.scaffoldSlot, binding.guest, key);
              else statusNode(target, 'pending', binding.guest, key, `Still waiting for ${target.dataset.scaffoldPlaceholder || target.dataset.scaffoldSlot || 'this region'}.`, true);
            });
          }, slowMs);
        });
        record.promise = Promise.race([operation, deadline, cancelled]).then(result => {
          if (!current(binding, record)) return false;
          if (result === false) { reportFailure(new Error('Read failed')); return false; }
          if (!record.failedSlots.size && binding.retryTasks.get(key) === retryTask) binding.retryTasks.delete(key);
          return result === undefined ? true : result;
        }).catch(error => {
          reportFailure(error, record.timedOut || record.externalAbort);
          return false;
        }).finally(() => {
          window.clearTimeout(record.timer); window.clearTimeout(record.slowTimer);
          record.timer = null; record.slowTimer = null;
          if (upstream) upstream.removeEventListener('abort', record.relay);
          if (binding.runs.get(key) === record) binding.runs.delete(key);
          record.slots = []; record.cancel = null; record.relay = null;
        });
        return record.promise;
      }
      function read(guest, options = {}) {
        const binding = capture(guest);
        if (!binding || typeof binding.registration.read !== 'function') return Promise.resolve(false);
        return run(binding, options.readKey || 'default', async context => {
          const payload = await binding.registration.read({ ...context, guest: binding.guest, root: binding.root });
          if (!context.current()) return false;
          return publish(binding, payload, { run: context.run });
        }, { ...options, timeoutMs: options.timeoutMs || binding.registration.timeoutMs });
      }
      function rebind(guest) { return bind(guest); }
      function retire(guestOrBinding, reason = 'Guest retired') {
        const binding = typeof guestOrBinding === 'object' ? guestOrBinding : bindings.get(guestOrBinding);
        return retireBinding(binding, reason);
      }
      document.addEventListener('click', event => {
        const target = event.target instanceof Element ? event.target : null;
        const retryButton = target?.closest('[data-scaffold-retry]');
        const htmxRetryButton = target?.closest('[data-native-read-retry]');
        if (retryButton) {
          const binding = capture(retryButton.dataset.scaffoldRetry);
          if (!binding || !retryButton.dataset.scaffoldReadKey) return;
          event.preventDefault();
          retry(binding, retryButton.dataset.scaffoldReadKey).catch(() => {});
          return;
        }
        if (htmxRetryButton) {
          const binding = capture(htmxRetryButton.dataset.nativeReadGuest);
          if (!binding || !htmxRetryButton.dataset.nativeReadRetry) return;
          event.preventDefault();
          retry(binding, htmxRetryButton.dataset.nativeReadRetry).catch(() => {});
        }
      });
      return Object.freeze({ registerGuest, hasGuest, bind, capture, current, signal, onRetire, slot, setSlot, publish, fail, run, read, retry, registerRetry, clearRetry, rebind, retire, fetchJson, fetchText });
    })();
    window.scaffoldRegistry = scaffoldRegistry;
    const elementsChangedConsumers = new Map();
    function consumeElementsChanged({ paneId, route, target, afterReplace = () => {} }) {
      const pull = () => {
        const binding = scaffoldRegistry.capture(paneId);
        if (!binding) return Promise.resolve(false);
        return scaffoldRegistry.run(binding, `structure-${paneId}`, async context => {
          const fragment = target();
          if (!fragment || !context.current() || !binding.root.contains(fragment)) return false;
          const html = await context.fetchText(route);
          if (!context.current() || !binding.root.contains(fragment)) return false;
          const changed = morphLivePane(fragment, html);
          if (!context.current()) return false;
          if (changed) {
            afterReplace(fragment, binding, context);
            applyAdminDomState();
          }
          fragment.removeAttribute('data-structure-read-error');
          fragment.removeAttribute('data-structure-read-stale');
          return { changed };
        }, {
          slots: [], supersede: false,
          onFailure: ({ current, reason }) => {
            const fragment = target();
            if (!current() || !fragment || !binding.root?.contains(fragment)) return;
            fragment.dataset.structureReadStale = 'true';
            fragment.dataset.structureReadError = reason || 'Structure refresh failed';
          }
        }).then(result => result && typeof result === 'object' && Object.prototype.hasOwnProperty.call(result, 'changed') ? result.changed : result);
      };
      elementsChangedConsumers.set(paneId, pull);
      return pull;
    }
    window.consumeElementsChanged = consumeElementsChanged;
    const nativeReadGuests = ['admin', 'portals', 'upload', 'stats', 'backblaze', 'wake-on-lan', 'test', 'linker', 'dhcp', 'firewall', 'unbound'];
    nativeReadGuests.forEach(guest => scaffoldRegistry.registerGuest({
      guest,
      root: () => document.querySelector(`[data-pane-panel="${CSS.escape(guest)}"]`),
      authorize: () => sessionLawfulTab(guest)
    }));
    const nativeHtmxOwners = new WeakMap();
    const nativeHtmxTargetIds = new WeakMap();
    let nextNativeHtmxTargetId = 0;
    function nativeHtmxOwner(event) { const xhr = event.detail?.xhr; return xhr ? nativeHtmxOwners.get(xhr) : null; }
    function nativeReadTargetKey(target, guest) {
      if (target.id) return `id:${target.id}`;
      for (const [attribute, name] of [['data-service-card', 'service'], ['data-linker-readback', 'linker'], ['data-upload-tree', 'upload-tree'], ['data-portals-grid', 'portals-grid'], ['data-scaffold-slot', 'slot'], ['data-scaffold-key', 'key']]) {
        if (target.hasAttribute(attribute)) return `${name}:${target.getAttribute(attribute) || guest}`;
      }
      let identity = nativeHtmxTargetIds.get(target);
      if (identity) return `node:${identity}`;
      const nextIdentity = ++nextNativeHtmxTargetId;
      nativeHtmxTargetIds.set(target, nextIdentity);
      return `node:${nextIdentity}`;
    }
    function nativeReadStatusHost(owner) {
      if (owner.guest === 'admin') {
        const card = owner.target.closest('[data-service-card]');
        const label = card?.querySelector('.toggle-label');
        if (label) return { container: label.parentElement || card, label };
      }
      if (owner.guest === 'linker') {
        const status = owner.target.querySelector('[data-linker-read-status]') || owner.pane.querySelector('[data-linker-read-status]');
        if (status) return { container: status.parentElement || owner.target, label: status };
      }
      if (owner.guest === 'upload') {
        const status = owner.pane.querySelector('[data-upload-tree-read-status]');
        if (status) return { container: status, label: status };
      }
      if (owner.guest === 'portals') {
        const status = owner.pane.querySelector('[data-portals-loading]');
        if (status) return { container: status, label: status.querySelector('p') || status };
      }
      let status = owner.target.querySelector('[data-native-read-status]');
      if (!status) {
        status = document.createElement('div'); status.dataset.nativeReadStatus = owner.readKey;
        status.setAttribute('role', 'status'); owner.target.appendChild(status);
      }
      return { container: status, label: status };
    }
    function displayNativeReadState(owner, message, withRetry = false) {
      if (!owner || owner.retired || !scaffoldRegistry.current(owner.binding) || !owner.target.isConnected || !owner.binding.root.contains(owner.target)) return;
      const host = nativeReadStatusHost(owner);
      if (!host?.container || !host.label) return;
      if (owner.originalStatusText === undefined) owner.originalStatusText = host.label.textContent || '';
      host.label.textContent = message;
      host.label.setAttribute('aria-live', 'polite');
      let retry = host.container.querySelector('[data-native-read-retry]');
      if (withRetry) {
        if (!retry) { retry = document.createElement('button'); retry.type = 'button'; retry.className = 'scaffold-slot-retry'; retry.textContent = 'Retry'; host.container.appendChild(retry); }
        retry.dataset.nativeReadRetry = owner.readKey;
        retry.dataset.nativeReadGuest = owner.guest;
      } else retry?.remove();
    }
    function clearNativeReadState(owner) {
      if (!owner?.target?.isConnected) return;
      const host = nativeReadStatusHost(owner);
      if (!host?.container || !host.label) return;
      host.container.querySelector('[data-native-read-retry]')?.remove();
      const generated = host.container.matches('[data-native-read-status]');
      if (generated) host.container.remove();
      else if (owner.originalStatusText !== undefined) host.label.textContent = owner.originalStatusText;
    }
    function retireNativeHtmxOwner(owner, reason = 'Read completed', { abort = false, keepRetry = false } = {}) {
      if (!owner || owner.retired) return false;
      owner.retired = true;
      window.clearTimeout(owner.timer); window.clearTimeout(owner.slowTimer);
      owner.timer = null; owner.slowTimer = null;
      if (owner.release) { const release = owner.release; owner.release = null; release(); }
      if (owner.binding.readOwners.get(owner.readKey) === owner) owner.binding.readOwners.delete(owner.readKey);
      if (!keepRetry) scaffoldRegistry.clearRetry(owner.binding, owner.readKey);
      if (abort) { try { if (owner.xhr.readyState !== XMLHttpRequest.DONE) owner.xhr.abort(); } catch (_) {} }
      return true;
    }
    function failNativeHtmxOwner(owner, reason = 'Read unavailable') {
      if (!owner || owner.retired || !scaffoldRegistry.current(owner.binding)) return;
      owner.responseHandled = true;
      displayNativeReadState(owner, reason, false);
      const path = owner.path;
      const target = owner.target;
      const swap = owner.swap;
      scaffoldRegistry.registerRetry(owner.binding, owner.readKey, () => {
        if (!scaffoldRegistry.current(owner.binding) || !target.isConnected || !owner.binding.root.contains(target)) return false;
        return window.htmx?.ajax('GET', path, { target, swap });
      }, { slots: [], timeoutMs: owner.timeoutMs, slowMs: DEFAULT_NATIVE_SLOW_READ_MS });
      displayNativeReadState(owner, reason, true);
      retireNativeHtmxOwner(owner, reason, { abort: reason === 'Read timed out', keepRetry: true });
    }
    function nativeHtmxResponseFailure(owner, html) {
      const source = String(html || '');
      if (/data-(?:cartridge-fault|cartridge-fault-kind|service-card-fault-kind|admin-membrane-refusal)=/.test(source)) return 'Read returned an upstream refusal';
      const template = document.createElement('template');
      template.innerHTML = source;
      if (template.content.querySelector('script')) return 'Read returned forbidden script markup';
      if (template.content.querySelector('[data-cartridge-fault], [data-cartridge-fault-kind], [data-service-card-fault-kind], [data-admin-membrane-refusal]')) return 'Read returned an upstream refusal';
      return '';
    }
    function completeNativeHtmxOwner(owner, html) {
      if (!owner || owner.retired || owner.responseHandled || !scaffoldRegistry.current(owner.binding)
          || !owner.target.isConnected || !owner.binding.root.contains(owner.target)) return false;
      owner.responseHandled = true;
      const status = Number(owner.xhr?.status || 0);
      const failure = status < 200 || status >= 300 ? `Read failed (HTTP ${status || 'unavailable'})` : nativeHtmxResponseFailure(owner, html);
      if (failure) { failNativeHtmxOwner(owner, failure); return false; }
      const changed = morphLivePane(owner.target, String(html || ''));
      if (!scaffoldRegistry.current(owner.binding)) { retireNativeHtmxOwner(owner, 'Read owner retired', { abort: true }); return false; }
      if (changed) {
        window.htmx?.process(owner.target);
        if (owner.guest === 'portals') bindPortalFragmentControls(owner.target);
        if (owner.guest === 'upload') syncUploadTreeSelection();
        applyAdminDomState();
      }
      clearNativeReadState(owner);
      retireNativeHtmxOwner(owner, 'Read completed');
      return true;
    }
    const DEFAULT_NATIVE_READ_TIMEOUT_MS = 8000;
    const DEFAULT_NATIVE_SLOW_READ_MS = 750;
    document.body.addEventListener('htmx:beforeRequest', event => {
      const detail = event.detail || {};
      const source = detail.elt instanceof Element ? detail.elt : null;
      const target = detail.target instanceof Element ? detail.target : null;
      if (source?.closest('[data-pane]')) return;
      const request = source?.closest('[hx-get], [hx-post], [hx-put], [hx-patch], [hx-delete]');
      const configuredVerb = String(detail.requestConfig?.verb || detail.requestConfig?.method || '').toLowerCase();
      const verb = configuredVerb || (request?.hasAttribute('hx-get') ? 'get' : request?.hasAttribute('hx-post') ? 'post' : request?.hasAttribute('hx-put') ? 'put' : request?.hasAttribute('hx-patch') ? 'patch' : request?.hasAttribute('hx-delete') ? 'delete' : '');
      if (verb !== 'get' || !detail.xhr) return;
      const pane = source?.closest('[data-pane-panel]') || target?.closest('[data-pane-panel]');
      const guest = pane?.dataset.panePanel;
      if (!pane || !guest) return;
      const binding = scaffoldRegistry.capture(guest) || (window.immortalFloor?.activeGuest === guest ? scaffoldRegistry.bind(guest) : null);
      if (!binding || !scaffoldRegistry.current(binding) || !target || !binding.root.contains(target)) {
        event.preventDefault();
        return;
      }
      const path = String(detail.requestConfig?.path || request?.getAttribute('hx-get') || '');
      if (!path) { event.preventDefault(); return; }
      const readKey = `htmx:${nativeReadTargetKey(target, guest)}`;
      const prior = binding.readOwners.get(readKey);
      if (prior && !prior.retired) {
        if (prior.path === path) { event.preventDefault(); return; }
        retireNativeHtmxOwner(prior, 'Read superseded', { abort: true });
      }
      const requestedTimeout = Number(detail.requestConfig?.timeout || detail.timeout);
      const timeoutMs = Number.isSafeInteger(requestedTimeout) && requestedTimeout > 0 ? Math.min(requestedTimeout, 30000) : DEFAULT_NATIVE_READ_TIMEOUT_MS;
      const swap = String(request?.getAttribute('hx-swap') || detail.requestConfig?.swap || 'innerHTML');
      const owner = { guest, pane, source, target, binding, xhr: detail.xhr, path, swap, readKey, timeoutMs, retired: false, release: null, timer: null, originalStatusText: undefined };
      binding.readOwners.set(readKey, owner);
      scaffoldRegistry.clearRetry(binding, readKey);
      clearNativeReadState(owner);
      try { owner.xhr.timeout = timeoutMs; } catch (_) {}
      owner.release = scaffoldRegistry.onRetire(binding, retirement => retireNativeHtmxOwner(owner, retirement?.reason || 'Guest retired', { abort: true }));
      nativeHtmxOwners.set(owner.xhr, owner);
      owner.timer = window.setTimeout(() => failNativeHtmxOwner(owner, 'Read timed out'), timeoutMs + 25);
      owner.slowTimer = window.setTimeout(() => displayNativeReadState(owner, `${guest} is still loading.`), Math.min(DEFAULT_NATIVE_SLOW_READ_MS, timeoutMs - 1));
    });
    document.body.addEventListener('htmx:beforeSwap', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner) return;
      if (owner.retired || event.detail?.target !== owner.target || !scaffoldRegistry.current(owner.binding) || !owner.target.isConnected || !owner.binding.root.contains(owner.target)) {
        event.detail.shouldSwap = false;
        retireNativeHtmxOwner(owner, 'Read owner retired', { abort: true });
        return;
      }
      event.detail.shouldSwap = false;
      completeNativeHtmxOwner(owner, event.detail?.serverResponse || owner.xhr?.responseText || '');
    });
    document.body.addEventListener('htmx:afterSwap', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || event.detail?.target !== owner.target || !scaffoldRegistry.current(owner.binding)) return;
      clearNativeReadState(owner);
      retireNativeHtmxOwner(owner);
    });
    document.body.addEventListener('htmx:afterRequest', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || (event.detail?.target && event.detail.target !== owner.target)) return;
      if (event.detail?.successful === true) {
        completeNativeHtmxOwner(owner, owner.xhr?.responseText || '');
        return;
      }
      failNativeHtmxOwner(owner, 'Read unavailable');
    });
    function reportNativeHtmxFailure(event) {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || !scaffoldRegistry.current(owner.binding)) return;
      const reason = event.type === 'htmx:timeout' ? 'Read timed out' : event.type === 'htmx:sendError' ? 'Connection unavailable' : 'Read unavailable';
      failNativeHtmxOwner(owner, reason);
    }
    ['htmx:timeout', 'htmx:responseError', 'htmx:sendError'].forEach(name => document.body.addEventListener(name, reportNativeHtmxFailure));
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState !== 'visible') {
        const guest = window.immortalFloor?.activeGuest;
        if (guest) scaffoldRegistry.retire(guest, 'Document hidden');
      }
    });
    function pullHeldElementFragments() {
      const activePane = lawfulPaneCandidate(currentActiveTabId());
      if ((activePane === 'portals' || activePane === 'stats') && viewportFamilyAdmitted(activePane)) {
        return (elementsChangedConsumers.get(activePane)?.() || Promise.resolve(false)).then(() => activePane);
      }
      return Promise.resolve(activePane);
    }
    function closeViewportStreamFamily(preserveDiskSnapshot = false) {
      if (!preserveDiskSnapshot) retireAdminDiskSnapshot();
      clearPulseRenewal();
      if (pulseStaleTimer) window.clearTimeout(pulseStaleTimer);
      pulseStaleTimer = null;
      if (pulseStream) pulseStream.close();
      pulseStream = null;
      pulseStreamId = null;
    }
    let portalCurrentnessTimer = null;
    let portalCurrentnessBinding = null;
    function stopPortalCurrentnessCadence() { if (portalCurrentnessTimer) window.clearInterval(portalCurrentnessTimer); portalCurrentnessTimer = null; portalCurrentnessBinding = null; }
    function startPortalCurrentnessCadence() {
      const binding = viewportFamilyAdmitted('portals') ? scaffoldRegistry.capture('portals') : null;
      if (!binding) { stopPortalCurrentnessCadence(); return; }
      if (portalCurrentnessBinding === binding && portalCurrentnessTimer) return;
      stopPortalCurrentnessCadence(); portalCurrentnessBinding = binding;
      refreshPortalCurrentness(binding);
      portalCurrentnessTimer = window.setInterval(() => { if (portalCurrentnessBinding === binding && viewportFamilyAdmitted('portals') && scaffoldRegistry.current(binding)) refreshPortalCurrentness(binding); else stopPortalCurrentnessCadence(); }, 5000);
    }
    function resumeIncompleteNativeReads(active) {
      if (!window.htmx) return;
      const pane = document.querySelector(`[data-pane-panel="${CSS.escape(active)}"]`);
      if (!pane) return;
      if (active === 'admin') pane.querySelectorAll('[data-service-card]').forEach(card => {
        const label = card.querySelector('.toggle-label');
        if (label && (/^Loading/i.test(label.textContent.trim()) || /^admin is still loading/i.test(label.textContent.trim()))) {
          window.htmx.process(card); window.htmx.trigger(card, 'admin-ready');
        }
      });
      if (active === 'portals') {
        const grid = pane.querySelector('[data-portals-grid]');
        const loading = grid?.querySelector('[data-portals-loading] p');
        if (grid && loading && (/^Reading/i.test(loading.textContent.trim()) || /^portals is still loading/i.test(loading.textContent.trim()))) {
          window.htmx.process(grid); window.htmx.trigger(grid, 'portals-admitted');
        }
      }
      if (active === 'upload') {
        const treeRead = pane.querySelector('[data-upload-tree-read-status]');
        if (treeRead && (/^Directory contents load after/i.test(treeRead.textContent.trim()) || /^upload is still loading/i.test(treeRead.textContent.trim()))) {
          window.htmx.process(treeRead); window.htmx.trigger(treeRead, 'upload-tree-admitted');
        }
      }
      if (active === 'linker') {
        const readback = pane.querySelector('[data-linker-readback]');
        const status = readback?.querySelector('[data-linker-read-status]');
        if (readback && status && (/loads after this view is admitted/i.test(status.textContent) || /^linker is still loading/i.test(status.textContent.trim()))) {
          window.htmx.process(readback); window.htmx.trigger(readback, 'linker-readback');
        }
      }
    }
    function reconcileViewportStreamFamily() {
      closeViewportStreamFamily(true);
      stopPortalCurrentnessCadence();
      if (document.visibilityState !== 'visible') {
        const hiddenGuest = window.immortalFloor?.activeGuest;
        if (hiddenGuest) scaffoldRegistry.retire(hiddenGuest, 'Document hidden');
        return;
      }
      if (window.getImmortalFloorState?.() !== 'Seated') return;
      const active = currentActiveTabId();
      if (!viewportFamilyAdmitted(active)) return;
      if (!scaffoldRegistry.capture(active)) scaffoldRegistry.bind(active);
      resumeIncompleteNativeReads(active);
      if (active === 'stats') { hydrateStats(); connectPulseStream(); }
      if (active === 'upload') hydrateUploadPolicy();
      if (active === 'dhcp') hydrateDhcp();
      if (active === 'test') hydrateTestNetworkCatalog();
      if (active === 'unbound') hydrateDns();
      if (active === 'firewall') hydrateFirewall();
      if (active === 'backblaze') hydrateBackblaze();
      if (active === 'portals') { startPortalCurrentnessCadence(); connectPulseStream(); }
      if (active === 'admin') {
        if (adminDocumentPatchPendingHydration && window.htmx) adminDocumentPatchPendingHydration = false;
        reconcileAdminDiskSnapshot();
      }
      if (active === 'wake-on-lan') hydrateWakeOnLan();
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
    function setStatsPulseStaleSignal(stale) {
      const root = document.querySelector('[data-scaffold-root][data-guest-id="stats"]');
      if (!root) return;
      root.dataset.pulseState = stale ? 'stale' : 'live';
      let status = root.querySelector('[data-stats-pulse-status]');
      if (!stale) { status?.remove(); return; }
      if (!status) {
        status = document.createElement('div');
        status.dataset.statsPulseStatus = 'stale';
        status.className = 'stats-pulse-status';
        status.setAttribute('role', 'status');
        root.prepend(status);
      }
      status.textContent = 'Live stats connection is stale. Reconnecting…';
    }
    function armPulseStaleWatchdog() {
      if (pulseStaleTimer) window.clearTimeout(pulseStaleTimer);
      pulseLastActivityAt = Date.now();
      pulseStaleTimer = window.setTimeout(() => {
        if (!pulseStream) return;
        const quietFor = Date.now() - pulseLastActivityAt;
        if (quietFor >= pulseStaleTimeoutMs) {
          setStatsPulseStaleSignal(true);
          reconnectPulseStream();
        } else armPulseStaleWatchdog();
      }, pulseStaleTimeoutMs);
    }
    function markPulseLiveness() {
      pulseLastActivityAt = Date.now();
      setStatsPulseStaleSignal(false);
      armPulseStaleWatchdog();
    }
    function reconnectPulseStream() {
      closeViewportStreamFamily();
      const delayMs = pulseReconnectDelayMs;
      pulseReconnectDelayMs = Math.min(pulseReconnectDelayMs * 2, sseReconnectMaxDelayMs);
      if (viewportFamilyAdmitted('stats') || viewportFamilyAdmitted('portals')) window.setTimeout(() => {
        if (viewportFamilyAdmitted('stats') || viewportFamilyAdmitted('portals')) connectPulseStream();
      }, delayMs);
    }
    function connectPulseStream() {
      if (!window.EventSource || !(viewportFamilyAdmitted('stats') || viewportFamilyAdmitted('portals'))) return;
      clearPulseRenewal();
      if (pulseStream) pulseStream.close();
      pulseStream = new EventSource('/api/stats/pulse');
      armPulseStaleWatchdog();
      pulseStream.addEventListener('pulse.open', event => {
        markPulseLiveness();
        let data = {};
        try { data = JSON.parse(event.data || '{}'); } catch (_) {}
        pulseStreamId = data.streamId || event.lastEventId || null;
        pulseReconnectDelayMs = sseReconnectBaseDelayMs;
        schedulePulseRenewal(data.renewRoute || (pulseStreamId ? '/api/stats/pulse/renew?streamId=' + encodeURIComponent(pulseStreamId) : null));
        if (headerState.isAdmin) void setStreamMembership('stats', pulseStreamId, 'upgrade');
      });
      pulseStream.addEventListener('tabs.changed', () => {
        const active = currentActiveTabId();
        refreshTabBar(active).then(selected => { if (selected) showPane(selected); }).catch(() => {});
      });
      pulseStream.addEventListener('elements.changed', () => {
        pullHeldElementFragments().catch(() => {});
      });
      pulseStream.addEventListener('stats.tick', () => {
        markPulseLiveness();
        hydrateStats().catch(() => {});
      });
      pulseStream.addEventListener('pulse.expired', reconnectPulseStream);
      pulseStream.addEventListener('error', () => {
        if (pulseStream && pulseStream.readyState === EventSource.CLOSED) {
          setStatsPulseStaleSignal(true);
          reconnectPulseStream();
        }
      });
    }
    const immortalFloorStates = Object.freeze(['BootFloor', 'Seated', 'GuestRevolution', 'BareFloor']);
    const immortalFloor = (() => {
      let state = 'BootFloor'; let generation = 0; let activeGuest = null;
      let crossingGuest = null;
      const hydrationTimeoutMs = 750;
      const statsHydrationTimeoutMs = 4000;
      const ready = new Promise(resolve => requestAnimationFrame(() => {
        if (immortalFloorShell && tabBar && panes.length) {
          immortalFloorShell.dataset.startupPhase = 'Ready';
          resolve(true);
        } else resolve(false);
      }));
      function expose(next, detail = '') {
        if (!immortalFloorStates.includes(next)) throw new Error('Invalid Immortal Floor state: ' + next);
        state = next;
        if (next !== 'Seated') retireAdminDiskSnapshot();
        document.documentElement.dataset.immortalFloorState = next;
        if (immortalFloorShell) immortalFloorShell.dataset.immortalFloorState = next;
        if (immortalFloorGuestSlot) immortalFloorGuestSlot.dataset.slotEmpty = String(!activeGuest && next !== 'Seated');
        const message = immortalFloorShell?.querySelector('[data-immortal-floor-message]');
        if (message) message.textContent = detail || ({ BootFloor: 'Preparing your controls…', GuestRevolution: 'Changing view…', BareFloor: 'Controls remain available. Choose a tab to try again.', Seated: '' }[next]);
      }
      function emptySlot() {
        closeViewportStreamFamily();
        if (activeGuest) scaffoldRegistry.retire(activeGuest);
        panes.forEach(pane => {
          pane.classList.remove('active', 'immortal-floor-enter');
          pane.setAttribute('aria-hidden', 'true');
        });
        activeGuest = null;
        if (immortalFloorGuestSlot) immortalFloorGuestSlot.dataset.slotEmpty = 'true';
      }
      let admissionRequest = null;
      const admissionOwners = new WeakMap();
      function retireAdmissionRequest(record = admissionRequest, reason = 'Admission retired') {
        if (!record || record.retired) return false;
        record.retired = true;
        if (admissionRequest === record) admissionRequest = null;
        record.cleanup?.();
        record.pendingHtml = null;
        try { record.cancel?.(new DOMException(reason, 'AbortError')); } catch (_) {}
        try { if (record.xhr && record.xhr.readyState !== XMLHttpRequest.DONE) record.xhr.abort(); } catch (_) {}
        return true;
      }
      document.body.addEventListener('htmx:beforeRequest', event => {
        const record = admissionRequest;
        const detail = event.detail || {};
        if (!record || record.tab !== detail.elt || detail.target !== record.pane || detail.xhr == null || record.crossing !== generation) return;
        record.xhr = detail.xhr;
        record.target = detail.target;
        try { record.xhr.timeout = record.timeoutMs; } catch (_) {}
        admissionOwners.set(record.xhr, record);
      });
      document.body.addEventListener('htmx:beforeSwap', event => {
        const xhr = event.detail?.xhr;
        const owner = xhr && admissionOwners.get(xhr);
        if (!owner) return;
        if (owner !== admissionRequest || owner.retired || owner.crossing !== generation || crossingGuest !== owner.id) {
          event.detail.shouldSwap = false;
          return;
        }
        const html = String(event.detail.serverResponse || '');
        if (!healthyResponse(owner.pane, html)) { owner.failed = true; event.detail.shouldSwap = false; return; }
        if (owner.sameGuestRefresh) {
          owner.pendingHtml = html;
          owner.accepted = true;
          event.detail.shouldSwap = false;
        }
      });
      function panelIdFromHtmxEvent(event) {
        const panel = panelFromHtmxEvent(event);
        return panel instanceof HTMLElement ? (panel.dataset.viewPanel || panel.dataset.panePanel || '') : '';
      }
      function healthyResponse(pane, html) {
        if (!(pane?.isConnected && String(html || '').trim() && !String(html || '').includes('data-cartridge-fault="true"'))) return false;
        const template = document.createElement('template');
        template.innerHTML = html;
        return template.content.childElementCount > 0;
      }
      function healthyStructure(pane, html) {
        return Boolean(pane?.isConnected && String(html || '').trim() && !String(html || '').includes('data-cartridge-fault="true"') && pane.childElementCount > 0);
      }
      async function admitFreshGuest(id, crossing, sameGuestRefresh = false) {
        const pane = panes.find(candidate => candidate.dataset.panePanel === id);
        const tab = tabs.find(candidate => candidate.dataset.pane === id);
        if (!pane) throw new Error('guest-missing');
        if (!tab?.getAttribute('hx-get') || !window.htmx) return pane;
        const timeoutCandidate = Number(tab.dataset.admissionTimeoutMs);
        const admissionTimeoutMs = Number.isSafeInteger(timeoutCandidate) && timeoutCandidate > 0 ? Math.min(timeoutCandidate, 30000) : 1500;
        if (admissionRequest) retireAdmissionRequest(admissionRequest, 'Admission superseded');
        const record = { id, tab, pane, crossing, sameGuestRefresh, xhr: null, target: null, timer: null, timeoutMs: admissionTimeoutMs, accepted: false, failed: false, retired: false, settled: false, pendingHtml: null, cleanup: null, cancel: null, resolve: null, reject: null };
        admissionRequest = record;
        await new Promise((resolve, reject) => {
          record.resolve = resolve; record.reject = reject;
          const cleanup = () => {
            if (record.cleaned) return;
            record.cleaned = true;
            document.body.removeEventListener('htmx:afterSwap', afterSwap);
            document.body.removeEventListener('htmx:afterRequest', afterRequest);
            document.body.removeEventListener('htmx:timeout', failed);
            document.body.removeEventListener('htmx:responseError', failed);
            document.body.removeEventListener('htmx:sendError', failed);
            if (record.timer !== null) window.clearTimeout(record.timer);
            record.timer = null;
            if (admissionRequest === record) admissionRequest = null;
            record.pendingHtml = null;
            record.cleanup = null;
          };
          const settle = (error = null, value = undefined) => {
            if (record.settled) return false;
            record.settled = true;
            cleanup();
            const resolveRecord = record.resolve; const rejectRecord = record.reject;
            record.resolve = null; record.reject = null; record.cancel = null;
            if (error) rejectRecord?.(error); else resolveRecord?.(value);
            return true;
          };
          record.cleanup = cleanup;
          record.cancel = error => settle(error || new DOMException('Admission retired', 'AbortError'));
          const isOwnRequest = event => event.detail?.xhr && event.detail.xhr === record.xhr && event.detail.target === pane && admissionRequest === record && !record.retired && record.crossing === generation && crossingGuest === id;
          const afterSwap = event => {
            if (!isOwnRequest(event)) return;
            if (!healthyStructure(pane, pane.innerHTML)) { settle(new Error('admission-structure-invalid')); return; }
            record.accepted = true;
            pane.dataset.nativeStructureReady = 'true';
            if (!sameGuestRefresh) settle(null, pane);
          };
          const afterRequest = async event => {
            if (!isOwnRequest(event)) return;
            if (record.failed) { settle(new Error('admission-structure-invalid')); return; }
            if (!sameGuestRefresh) return;
            if (!record.accepted || !record.pendingHtml) { settle(new Error('admission-fault')); return; }
            await new Promise(resolveFrame => requestAnimationFrame(() => requestAnimationFrame(resolveFrame)));
            if (record.retired || admissionRequest !== record || record.crossing !== generation || crossingGuest !== id) { settle(new Error('admission-superseded')); return; }
            const pendingHtml = record.pendingHtml;
            record.pendingHtml = null;
            morphLivePane(pane, pendingHtml);
            pane.dataset.nativeStructureReady = String(pane.childElementCount > 0);
            settle(null, pane);
          };
          const failed = event => {
            if (!record.xhr || event.detail?.xhr !== record.xhr || (event.detail?.target && event.detail.target !== pane)) return;
            settle(new Error(event.type === 'htmx:timeout' ? 'admission-timeout' : 'admission-fault'));
          };
          document.body.addEventListener('htmx:afterSwap', afterSwap);
          document.body.addEventListener('htmx:afterRequest', afterRequest);
          document.body.addEventListener('htmx:timeout', failed);
          document.body.addEventListener('htmx:responseError', failed);
          document.body.addEventListener('htmx:sendError', failed);
          record.timer = window.setTimeout(() => retireAdmissionRequest(record, 'admission-timeout'), admissionTimeoutMs);
          window.htmx.trigger(tab, 'immortal-floor-admit');
        });
        return pane;
      }
      async function seatGuest(id, crossing, detail = '') {
        const pane = panes.find(candidate => candidate.dataset.panePanel === id);
        if (!pane) return false;
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        if (crossing !== generation || crossingGuest !== id || !healthyStructure(pane, pane.innerHTML)) return false;
        tabs.forEach(tab => { const active = tab.dataset.pane === id; tab.setAttribute('aria-selected', String(active)); tab.classList.toggle('active', active); });
        panes.forEach(candidate => { const active = candidate === pane; candidate.classList.toggle('active', active); candidate.classList.toggle('immortal-floor-enter', active); candidate.setAttribute('aria-hidden', String(!active)); });
        if (activeGuest && activeGuest !== id) scaffoldRegistry.retire(activeGuest);
        activeGuest = id; crossingGuest = null; expose('Seated', detail); applyAdminDomState(); applyTabBarVisibility();
        scaffoldRegistry.bind(id);
        if (id === 'admin' && window.htmx) {
          if (adminDocumentPatchPendingHydration) {
            adminDocumentPatchPendingHydration = false;
            window.htmx.process(pane);
          }
          pane.querySelectorAll('[data-service-card]').forEach(card => window.htmx.trigger(card, 'admin-ready'));
        }
        if (id === 'linker' && window.htmx) {
          const readback = pane.querySelector('[data-linker-readback]');
          if (readback) { window.htmx.process(readback); window.htmx.trigger(readback, 'linker-readback'); }
        }
        if (id === 'portals' && window.htmx) {
          const grid = pane.querySelector('[data-portals-grid]');
          if (grid) { window.htmx.process(grid); window.htmx.trigger(grid, 'portals-admitted'); }
        }
        if (id === 'upload' && window.htmx) {
          const treeRead = pane.querySelector('[data-upload-tree-read-status]');
          if (treeRead) { window.htmx.process(treeRead); window.htmx.trigger(treeRead, 'upload-tree-admitted'); }
        }
        reconcileViewportStreamFamily();
        return true;
      }
      async function activate(requested, options = {}) {
        const selected = lawfulPaneCandidate(requested);
        if (state === 'Seated' && activeGuest === selected && !options.refresh) {
          crossingGuest = null;
          applyAdminDomState();
          return true;
        }
        const outgoing = activeGuest;
        const hadHealthyFloor = state === 'Seated' && Boolean(outgoing);
        const crossing = ++generation;
        if (admissionRequest) retireAdmissionRequest(admissionRequest);
        if (outgoing) scaffoldRegistry.retire(outgoing);
        closeViewportStreamFamily();
        stopPortalCurrentnessCadence();
        crossingGuest = selected;
        const readyNow = await ready;
        if (!readyNow) {
          if (crossing === generation) { emptySlot(); expose('BareFloor'); }
          return false;
        }
        if (crossing !== generation) return false;
        expose('GuestRevolution');
        try {
          const sameGuestRefresh = hadHealthyFloor && outgoing === selected;
          await admitFreshGuest(selected, crossing, sameGuestRefresh);
          if (crossing !== generation || crossingGuest !== selected) return false;
          if (!await seatGuest(selected, crossing, options.detail)) throw new Error('guest-structure-unhealthy');
          return true;
        } catch (error) {
          if (crossing !== generation) return false;
          retireAdmissionRequest();
          if (hadHealthyFloor && activeGuest === outgoing) {
            crossingGuest = null;
            const retainedPane = panes.find(p=>p.dataset.panePanel===outgoing);
            if(retainedPane)retainedPane.dataset.viewportFaulted='false';
            expose('Seated', 'The requested view did not refresh. The last healthy view remains open.');
            applyAdminDomState();
            applyTabBarVisibility();
            reconcileViewportStreamFamily();
          } else {
            fault(error?.message || 'admission-fault');
          }
          return false;
        }
      }
      function fault(kind = 'guest-fault') {
        if (admissionRequest) retireAdmissionRequest(admissionRequest, 'Admission fault');
        generation += 1;
        emptySlot();
        expose('BareFloor', 'This view could not open. Choose a tab to try again.');
        console.warn('[coronatio] view failed to open', { pane: crossingGuest, guest: crossingGuest, kind });
        crossingGuest = null;
        document.documentElement.dataset.immortalFloorFault = kind;
      }
      function admissionOwner(xhr) {
        const record = xhr && admissionOwners.get(xhr);
        if (!record || record !== admissionRequest || record.xhr !== xhr || record.retired || record.settled || record.crossing !== generation || crossingGuest !== record.id || record.target !== record.pane || !record.pane?.isConnected) return null;
        return { guest: record.id, pane: record.pane };
      }
      function isAdmissionRequest(xhr) { return admissionOwner(xhr) !== null; }
      function failAdmission(xhr, kind = 'admission-fault') {
        const record = xhr && admissionOwners.get(xhr);
        if (!record || !admissionOwner(xhr)) return false;
        const coldFault = !activeGuest;
        retireAdmissionRequest(record, kind);
        // activate() owns the cold-versus-refresh decision.  Do not empty the
        // floor here: it may still be showing a healthy outgoing guest.
        return coldFault;
      }
      function adminLost() {
        scaffoldRegistry.retire('admin', 'Admin role retired');
        if (crossingGuest === 'admin') {
          generation += 1;
          retireAdmissionRequest(admissionRequest, 'Admin role retired');
          crossingGuest = null;
          if (activeGuest === 'admin') {
            emptySlot();
            expose('BareFloor', 'Admin access ended. Choose a permitted tab.');
          } else if (activeGuest) {
            expose('Seated', 'Admin access ended.');
            applyAdminDomState(); applyTabBarVisibility();
          } else {
            emptySlot();
            expose('BareFloor', 'Admin access ended. Choose a permitted tab.');
          }
          return true;
        }
        if (activeGuest === 'admin') {
          emptySlot();
          if (crossingGuest && admissionRequest) expose('GuestRevolution', 'Admin access ended. Opening the selected view…');
          else {
            generation += 1;
            retireAdmissionRequest(admissionRequest, 'Admin role retired');
            crossingGuest = null;
            expose('BareFloor', 'Admin access ended. Choose a permitted tab.');
          }
          return true;
        }
        return false;
      }
      expose('BootFloor');
      return Object.freeze({ activate, fault, admissionOwner, isAdmissionRequest, failAdmission, adminLost, get state() { return state; }, get activeGuest() { return activeGuest; } });
    })();
    window.immortalFloor = immortalFloor;
    window.getImmortalFloorState = () => immortalFloor.state;
    function showPane(id, options) { return immortalFloor.activate(id, options); }
    async function AdminChromeProjection(previousActive) {
      try {
        const selectedGuest = lawfulPaneCandidate(previousActive);
        const crossed = await showPane(selectedGuest, { refresh: true });
        return crossed === true && window.getImmortalFloorState?.() === 'Seated' && currentActiveTabId() === selectedGuest;
      } catch (error) {
        console.warn('[coronatio] admin admission projection fault', error?.message || error);
        return false;
      }
    }
    async function runFavoriteLadder({ startAt = 1, useHash = false } = {}) {
      const regular = eligibleRegularTabs().map(tab => tab.dataset.pane).filter(Boolean);
      if (regular.length === 0) return showPane(fallbackTab, { detail: 'This device is locked down.' });
      let selected = null;
      if (useHash && startAt <= 0) { const requested = location.hash.slice(1); if (requested && sessionLawfulTab(requested)) { selected = requested; history.replaceState(history.state, document.title, location.pathname + location.search); } }
      const browserFavorite = browserFavoriteTab();
      if (!selected && startAt <= 1 && canStarTab(browserFavorite)) selected = browserFavorite;
      let applianceFavorite = renderedApplianceFavoriteTab();
      if (!selected && startAt <= 2) { try { const favorite = await fetch('/api/favorites', { cache: 'no-store' }).then(response => response.ok ? response.json() : null); if (favorite?.starredTab) { applianceFavorite = favorite.starredTab; if (canStarTab(applianceFavorite)) selected = applianceFavorite; } } catch (_) {} }
      selected = selected || (canStarTab(applianceFavorite) ? applianceFavorite : regular[0]);
      paintEffectiveFavoriteTab(applianceFavorite);
      for (const candidate of [selected, ...regular.filter(id => id !== selected)]) if (await showPane(candidate)) return true;
      return showPane(fallbackTab);
    }
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
      document.querySelectorAll('[data-tab-star]').forEach(button => button.addEventListener('click', event => {
        event.stopPropagation();
        if (canStarTab(button.dataset.tabStar)) setStarredTab(button.dataset.tabStar);
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
    reconcileAdmittedPaneHosts();
    bindTabControls();
    document.querySelector('[data-crown-headless-reload]')?.addEventListener('click', () => window.location.reload());
    document.addEventListener('visibilitychange', reconcileViewportStreamFamily); async function fetchInto(route, target, method = 'GET') {
      const el = document.getElementById(target);"####
}