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
    const adminMountDestinationBindings = new WeakSet();
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
      if (!headerState.isAdmin) closeWakeOnLanModal();
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
    function captureAdminDocumentSkeletons(root) {
      ['admin', 'backblaze', 'firewall', 'unbound'].forEach(id => {
        const pane = [...root.children].find(node => node instanceof HTMLElement
          && node.dataset.panePanel === id
          && node.hasAttribute('data-admin-document-patch-owned'));
        if (pane) window.nativePaneSkeletonFloor?.register(id, 'admin', pane);
      });
      root.querySelectorAll('template[data-admin-document-patch-owned][data-native-role-skeleton][data-role="admin"]').forEach(template => {
        window.nativePaneSkeletonFloor?.registerRole(template.dataset.nativeRoleSkeleton, 'admin', template.innerHTML);
        template.remove();
      });
    }
    function retireAdminDocumentSkeletons() {
      ['admin', 'backblaze', 'firewall', 'unbound'].forEach(id => window.nativePaneSkeletonFloor?.retire(id));
      ['upload', 'stats', 'dhcp'].forEach(id => window.nativePaneSkeletonFloor?.retire(id, 'admin'));
    }
    function installAdminDocumentPatch(patch, tabsHtml) {
      if (typeof patch !== 'string' || typeof tabsHtml !== 'string' || !immortalFloorGuestSlot) return false;
      const template = document.createElement('template');
      template.innerHTML = patch;
      const children = [...template.content.children];
      const marker = children.find(node => node.matches('[data-admin-document-patch="true"]'));
      const paneIds = ['admin', 'backblaze', 'linker', 'firewall', 'unbound'];
      const panesToInstall = children.filter(node => node.matches('[data-admin-document-patch-owned][data-pane-panel]'));
      const roleTemplates = children.filter(node => node.matches('template[data-admin-document-patch-owned][data-native-role-skeleton][data-role="admin"]'));
      const installedIds = new Set(panesToInstall.map(node => node.dataset.panePanel));
      const roleIds = new Set(roleTemplates.map(node => node.dataset.nativeRoleSkeleton));
      if (!marker || panesToInstall.length !== paneIds.length || paneIds.some(id => !installedIds.has(id))
          || roleTemplates.length !== 3 || ['upload', 'stats', 'dhcp'].some(id => !roleIds.has(id))) return false;
      immortalFloorGuestSlot.querySelectorAll('[data-admin-document-patch-owned], [data-admin-document-patch="true"]').forEach(node => node.remove());
      marker.setAttribute('data-admin-document-patch-owned', 'true');
      immortalFloorGuestSlot.appendChild(marker);
      [...panesToInstall, ...roleTemplates].forEach(node => {
        node.setAttribute('data-admin-document-patch-owned', 'true');
        immortalFloorGuestSlot.appendChild(node);
      });
      captureAdminDocumentSkeletons(immortalFloorGuestSlot);
      panes = [...document.querySelectorAll('[data-pane-panel]')];
      adminDocumentPatchPendingHydration = true;
      replaceTabBar(tabsHtml);
      return true;
    }
    function removeAdminDocumentPatch() {
      retireAdminDiskSnapshot();
      if (document.querySelector('[data-admin-document-patch-owned], [data-admin-document-patch="true"]')
          || window.immortalFloor?.activeGuest === 'admin' || headerState.isAdmin) retirePrivilegedClientState();
      document.querySelectorAll('[data-admin-document-patch-owned], [data-admin-document-patch="true"]').forEach(node => node.remove());
      retireAdminDocumentSkeletons();
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
      if (kind === 'wake-on-lan') return wakeOnLanModalTemplate();
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
      root.querySelectorAll('[data-transmission-detail], [data-transmission-condition], [data-transmission-key-presence], [data-transmission-service-active], [data-transmission-service-enabled], [data-vpn-projection-enabled], [data-vpn-projection-unit], [data-transmission-vpn-settings-status]').forEach(node => { node.textContent = 'Unavailable'; });
      root.querySelectorAll('[data-transmission-vpn-control]').forEach(control => { control.disabled = true; if ('value' in control) control.value = ''; });
      root.querySelectorAll('[data-transmission-vpn-settings-save]').forEach(control => { control.disabled = true; });
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
    function transmissionVpnSettingsBlock(data) {
      if (!data || typeof data !== 'object' || Array.isArray(data)) return null;
      const transmission = data.transmission;
      if (!transmission || typeof transmission !== 'object' || Array.isArray(transmission)) return null;
      const vpn = transmission.vpn;
      if (vpn != null && (typeof vpn !== 'object' || Array.isArray(vpn))) return null;
      return transmission;
    }
    function transmissionVpnSettingsSafeRefusalSignal(data) {
      const candidate = readSignal(data, 'firstMissingSignal', 'first_missing_signal');
      return candidate.length <= 160 && candidate.trim() === candidate &&
        /^(?:caduceus-|transmission-|agathodaimon-)[A-Za-z0-9_.:-]+$/.test(candidate)
        ? candidate
        : '';
    }
    function transmissionVpnSettingsFormValues(transmission) {
      const vpn = transmission?.vpn;
      if (vpn != null && (typeof vpn !== 'object' || Array.isArray(vpn))) return null;
      const protocol = vpn?.protocol == null || vpn.protocol === 'openvpn' ? 'openvpn' : (vpn.protocol === 'wireguard' ? 'wireguard' : null);
      const region = vpn?.region == null ? '' : (typeof vpn.region === 'string' ? vpn.region : null);
      const dip = vpn?.dip == null || vpn.dip === false ? 'off' : (vpn.dip === true ? 'on' : null);
      const dns = vpn?.dns == null || vpn.dns === '1.1.1.1' ? '1.1.1.1' : (vpn.dns === 'provider' ? 'provider' : null);
      const rescan = transmission?.rescan == null || transmission.rescan === false ? 'off' : (transmission.rescan === true ? 'on' : null);
      if (!protocol || region === null || !dip || !dns || !rescan) return null;
      return { protocol, region, dip, dns, rescan };
    }
    function cloneTransmissionVpnBlock(transmission) {
      return JSON.parse(JSON.stringify(transmission));
    }
    function hydrateTransmissionVpnSettings(generation, modal) {
      if (!headerState.isAdmin || !modal) return;
      const statusNode = modal.querySelector('[data-transmission-vpn-settings-status]');
      const save = modal.querySelector('[data-transmission-vpn-settings-save]');
      const controls = [...modal.querySelectorAll('[data-transmission-vpn-control]')];
      const protocol = modal.querySelector('[data-transmission-vpn-protocol]');
      const region = modal.querySelector('[data-transmission-vpn-region]');
      const dip = modal.querySelector('[data-transmission-vpn-dip]');
      const dns = modal.querySelector('[data-transmission-vpn-dns]');
      const rescan = modal.querySelector('[data-transmission-vpn-rescan]');
      if (!statusNode || !save || controls.length !== 5 || !protocol || !region || !dip || !dns || !rescan) return;
      const isCurrent = () => modalResponseIsCurrent(generation, modal, true);
      let transmission = null;
      let authoritative = false;
      let saving = false;
      const setLocked = locked => {
        controls.forEach(control => { control.disabled = locked; });
        save.disabled = locked || !authoritative || saving;
      };
      const setValues = values => {
        protocol.value = values.protocol;
        region.value = values.region;
        region.placeholder = 'Automatic (leave blank)';
        dip.value = values.dip;
        dns.value = values.dns;
        rescan.value = values.rescan;
      };
      setLocked(true);
      statusNode.textContent = 'Loading VPN settings from Caduceus…';
      save.addEventListener('click', async () => {
        if (!isCurrent() || !authoritative || saving || !transmission) return;
        const nextTransmission = cloneTransmissionVpnBlock(transmission);
        if (nextTransmission.vpn == null) nextTransmission.vpn = {};
        nextTransmission.vpn.protocol = protocol.value === 'wireguard' ? 'wireguard' : null;
        nextTransmission.vpn.region = region.value.trim() ? region.value : null;
        nextTransmission.vpn.dip = dip.value === 'on' ? true : null;
        nextTransmission.vpn.dns = dns.value === 'provider' ? 'provider' : null;
        nextTransmission.rescan = rescan.value === 'on' ? true : null;
        saving = true;
        setLocked(true);
        statusNode.textContent = 'Saving VPN settings…';
        try {
          const response = await fetch('/api/v1/transmission/vpn/settings', {
            method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store',
            body: JSON.stringify({ transmission: nextTransmission })
          });
          const receipt = await response.json().catch(() => null);
          if (!isCurrent()) return;
          const written = transmissionVpnSettingsBlock(receipt);
          const writtenValues = written && transmissionVpnSettingsFormValues(written);
          const reportedSignal = readSignal(receipt, 'firstMissingSignal', 'first_missing_signal');
          const refusalSignal = transmissionVpnSettingsSafeRefusalSignal(receipt);
          const refused = receipt?.ok === false || receipt?.converged === false || receipt?.refused === true ||
            receipt?.accepted === false || receipt?.success === false || receipt?.failed === true ||
            Boolean(reportedSignal) || Boolean(receipt?.failure) || Boolean(receipt?.refusal) || Boolean(receipt?.error);
          const confirmed = response.ok && !refused && receipt?.appliesOnNextStart === true && Boolean(writtenValues);
          if (!confirmed) {
            statusNode.textContent = refusalSignal === 'caduceus-attendance-required'
              ? 'Caduceus attendance is required. Enter Admin Mode and provide your PIN, then try again.'
              : refusalSignal
                ? `VPN settings save not confirmed: ${refusalSignal}.`
                : 'VPN settings save not confirmed.';
            return;
          }
          transmission = cloneTransmissionVpnBlock(written);
          setValues(writtenValues);
          statusNode.textContent = 'applies on next start';
        } catch (_) {
          if (isCurrent()) statusNode.textContent = 'VPN settings save not confirmed.';
        } finally {
          if (isCurrent()) {
            saving = false;
            setLocked(!authoritative);
          }
        }
      });
      void (async () => {
        try {
          const response = await fetch('/api/v1/transmission/vpn/settings', { cache: 'no-store' });
          const data = await response.json().catch(() => null);
          if (!isCurrent()) return;
          const signal = readSignal(data, 'firstMissingSignal', 'first_missing_signal');
          const currentTransmission = transmissionVpnSettingsBlock(data);
          const values = currentTransmission && transmissionVpnSettingsFormValues(currentTransmission);
          const readRefused = data?.ok === false || data?.success === false || data?.accepted === false || data?.refused === true || Boolean(data?.refusal) || Boolean(data?.error);
          if (!response.ok || readRefused || signal || !currentTransmission || !values) {
            statusNode.textContent = 'VPN settings unavailable.';
            return;
          }
          transmission = cloneTransmissionVpnBlock(currentTransmission);
          setValues(values);
          authoritative = true;
          setLocked(false);
          statusNode.textContent = 'VPN settings loaded from Caduceus.';
        } catch (_) {
          if (isCurrent()) statusNode.textContent = 'VPN settings unavailable.';
        }
      })();
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
      invalidateWakeOnLanModal();
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
        const transmissionModal = infoBody.querySelector('[data-modal-kind-body="openvpn"]');
        void hydrateTransmissionAdminReads(infoModalGeneration, transmissionModal);
        void hydrateTransmissionVpnSettings(infoModalGeneration, transmissionModal);
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
      invalidateWakeOnLanModal();
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
__PANE_STRUCTURE_OWNER__
__NATIVE_HTMX_OWNER__
    document.addEventListener('visibilitychange', () => {
      const changed = window.immortalFloor?.visibilityChanged?.();
      if (changed && document.visibilityState !== 'visible') cancelXeniaColdRetries(null);
      if (changed) reconcileViewportStreamFamily();
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
    function adminMountDestinationRows(payload) {
      const rows = payload?.adminRuntime?.mountDestinations;
      if (!Array.isArray(rows)) throw new Error('Read failed');
      const expectedPaths = ['/mnt/nas', '/mnt/nas_backup'];
      const byPath = new Map();
      rows.forEach(row => {
        if (!row || typeof row !== 'object' || Array.isArray(row) || !expectedPaths.includes(row.path)) return;
        if (byPath.has(row.path) || typeof row.role !== 'string' || !row.role.trim() || typeof row.inUse !== 'boolean') throw new Error('Read failed');
        byPath.set(row.path, row);
      });
      if (expectedPaths.some(path => !byPath.has(path))) throw new Error('Read failed');
      return expectedPaths.map(path => byPath.get(path));
    }
    function renderAdminMountDestinations(rows) {
      return rows.map(destination => {
        const mounted = destination.inUse;
        const device = typeof destination.device === 'string' && destination.device.trim() ? destination.device : 'unknown';
        const freeBytes = destination.freeBytes, totalBytes = destination.totalBytes;
        const space = typeof freeBytes === 'number' && Number.isFinite(freeBytes) && freeBytes >= 0
          && typeof totalBytes === 'number' && Number.isFinite(totalBytes) && totalBytes >= 0
          ? diskSpaceDisplayValue({ availableBytes: freeBytes, sizeBytes: totalBytes })
          : 'Space unavailable';
        const mount = mounted
          ? `<div class="disk-mount-info">Device: <span class="device-label">${escapeHtml(device)}</span><div class="disk-space-usage"><strong>Space:</strong> ${escapeHtml(space)}</div></div>`
          : '';
        return `<div class="disk-item${mounted ? ' mounted locked' : ''}"><span class="disk-icon">▦</span><div class="disk-info"><div class="disk-name">${escapeHtml(destination.role)}</div><div class="disk-details">${escapeHtml(destination.path)}</div>${mount}</div></div>`;
      }).join('');
    }
    function adminMountDestinationCurrent(binding, context, host) {
      return Boolean(headerState.isAdmin && coronatioAttendanceRuntime.currentAttendance && context.current()
        && viewportFamilyAdmitted('admin') && host?.isConnected && binding.root?.contains(host)
        && host.closest('[data-pane-panel="admin"]') === binding.root);
    }
    function readAdminMountDestinations(binding) {
      const host = binding?.root?.querySelector('[data-admin-mounts-readback]');
      const readKey = 'admin-mount-destinations';
      if (!binding || binding.guest !== 'admin' || !host || host.getAttribute('data-admin-mounts-readback') !== '/api/services/data'
          || !headerState.isAdmin || !coronatioAttendanceRuntime.currentAttendance || !viewportFamilyAdmitted('admin')
          || !scaffoldRegistry.current(binding)) return Promise.resolve(false);
      if (adminMountDestinationBindings.has(binding)) return binding.runs.get(readKey)?.promise || Promise.resolve(true);
      if (host.dataset.scaffoldState === 'ready' && host.dataset.scaffoldStale !== 'true') {
        adminMountDestinationBindings.add(binding);
        return Promise.resolve(true);
      }
      adminMountDestinationBindings.add(binding);
      return scaffoldRegistry.run(binding, readKey, async context => {
        if (!adminMountDestinationCurrent(binding, context, host)) return false;
        const payload = await context.fetchJson('/api/services/data');
        if (!adminMountDestinationCurrent(binding, context, host)) return false;
        const rows = adminMountDestinationRows(payload);
        const markup = renderAdminMountDestinations(rows);
        if (!adminMountDestinationCurrent(binding, context, host)) return false;
        host.innerHTML = markup;
        context.setSlot(host, 'ready');
        return true;
      }, { slots: [host], timeoutMs: 8000, slowMs: 750 });
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
      if (active !== 'stats') closeWakeOnLanModal();
      if (!viewportFamilyAdmitted(active)) return;
      let binding = scaffoldRegistry.capture(active);
      if (!binding) binding = scaffoldRegistry.bind(active);
      const activePane = panes.find(pane => pane.dataset.panePanel === active);
      if (binding && activePane) window.startGuestScaffoldSlots?.(active, activePane, binding);
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
        if (binding) void readAdminMountDestinations(binding);
        reconcileAdminDiskSnapshot();
      }
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
__IMMORTAL_FLOOR_OWNER__
    __LINKER_CLIENT__
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
      for (const candidate of [selected, ...regular.filter(id => id !== selected)]) {
        const visibilityEpoch = window.immortalFloor?.visibilityEpoch;
        if (await showPane(candidate)) return true;
        if (document.visibilityState !== 'visible' || (visibilityEpoch !== undefined && window.immortalFloor?.visibilityEpoch !== visibilityEpoch)) return false;
        if (window.getImmortalFloorState?.() === 'BareFloor') return false;
      }
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
    async function fetchInto(route, target, method = 'GET') {
      const el = document.getElementById(target);"####
}