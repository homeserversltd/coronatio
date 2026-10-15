fn shell_wake_on_lan_client() -> &'static str {
    r####"    const WAKE_ON_LAN_READ_KEY = 'wakeonlan-modal';
    const wakeOnLanState = { devices: [], binding: null, currentData: false, stale: false, hasSnapshot: false, generation: 0, readController: null, waking: new Set() };
    const wakeOnLanRetirementBindings = new WeakSet();
    function wakeOnLanRoot(binding = wakeOnLanState.binding) { return binding?.root instanceof Element ? binding.root : null; }
    function wakeOnLanModal() { return infoBody?.querySelector('[data-modal-kind-body="wake-on-lan"]') || null; }
    function wakeOnLanStatsIsActive() { return currentActiveTabId() === 'stats' && panes.find(candidate => candidate.dataset.panePanel === 'stats')?.classList.contains('active') === true; }
    function wakeOnLanOwnerIsCurrent(generation = wakeOnLanState.generation, binding = wakeOnLanState.binding) {
      const root = wakeOnLanRoot(binding), modal = wakeOnLanModal();
      return Boolean(binding && wakeOnLanState.binding === binding && generation === wakeOnLanState.generation
        && root === document.querySelector('[data-pane-panel="stats"][data-scaffold-root][data-guest-id="stats"]') && headerState.isAdmin && appRoot?.dataset.adminMode === 'true'
        && viewportFamilyAdmitted('stats') && scaffoldRegistry.current(binding) && wakeOnLanStatsIsActive()
        && infoBackdrop.classList.contains('open') && modal instanceof Element && modal.isConnected && infoBody.contains(modal));
    }
    function wakeOnLanCurrent(binding = wakeOnLanState.binding, generation = wakeOnLanState.generation) {
      return Boolean(binding && wakeOnLanState.binding === binding && wakeOnLanState.currentData && !wakeOnLanState.stale
        && wakeOnLanOwnerIsCurrent(generation, binding));
    }
    function wakeOnLanRows(payload) {
      let source = payload;
      for (let depth = 0; depth < 5; depth += 1) {
        if (source && typeof source === 'object' && Array.isArray(source.leases) && Array.isArray(source.reservations)) return source;
        const next = source?.payload?.result || source?.result || source?.data?.result || source?.data || source?.payload || source?.receipt;
        if (!next || next === source) break;
        source = next;
      }
      throw new Error('Wake-on-LAN device response is incomplete');
    }
    function wakeOnLanDevice(row) {
      if (!row || typeof row !== 'object' || Array.isArray(row)) throw new Error('Wake-on-LAN device row is malformed');
      const mac = String(row.mac || row.macAddress || row['hw-address'] || row.hwAddress || '').toUpperCase();
      const ip = String(row.ip || row.ipAddress || row['ip-address'] || row.address || '').trim();
      if (!mac || !ip) return null;
      const name = String(row.hostname || row.hostName || row.name || ip).trim();
      return { mac, ip, name: name || ip };
    }
    function wakeOnLanMessage(payload) { return payload?.error || payload?.message || payload?.firstMissingSignal || 'Wake-on-LAN is unavailable'; }
    async function wakeOnLanJson(route, options = {}) {
      const response = await fetch(route, { cache: 'no-store', headers: options.body ? { 'content-type': 'application/json' } : {}, ...options });
      const body = await response.json().catch(() => ({}));
      if (!response.ok || body?.ok === false) throw body;
      return body;
    }
    function wakeOnLanModalTemplate() {
      return `<div data-modal-kind-body="wake-on-lan"><div class="wake-on-lan-tablet" data-wake-on-lan-tablet data-wake-on-lan-read-state="pending" aria-busy="true"><header class="wake-on-lan-header"><div><h2>Wake on LAN</h2><p>Devices known to DHCP.</p></div></header><div class="wake-on-lan-error" data-wake-on-lan-error role="alert" hidden></div><ul class="wake-on-lan-rows" data-wake-on-lan-rows aria-live="polite"><li class="wake-on-lan-empty">Loading devices…</li></ul></div></div>`;
    }
    function wakeOnLanSetReadState(generation, state) {
      if (!wakeOnLanOwnerIsCurrent(generation)) return;
      const tablet = wakeOnLanModal()?.querySelector('[data-wake-on-lan-tablet]');
      if (!tablet) return;
      tablet.dataset.wakeOnLanReadState = state;
      tablet.setAttribute('aria-busy', String(state === 'pending'));
    }
    function wakeOnLanSetError(generation, message, retry = false, hidden = false) {
      if (!wakeOnLanOwnerIsCurrent(generation)) return;
      const error = wakeOnLanModal()?.querySelector('[data-wake-on-lan-error]');
      if (!error) return;
      error.hidden = hidden;
      error.textContent = message;
      if (retry) {
        const button = document.createElement('button');
        button.type = 'button';
        button.dataset.wakeOnLanRetry = '';
        button.textContent = 'Retry';
        error.append(document.createTextNode(' '), button);
      }
    }
    function wakeOnLanTargetKey(device) { return JSON.stringify([device.mac, device.ip]); }
    function renderWakeOnLan(generation = wakeOnLanState.generation) {
      if (!wakeOnLanOwnerIsCurrent(generation)) return;
      const rows = wakeOnLanModal()?.querySelector('[data-wake-on-lan-rows]');
      if (!rows) return;
      const html = wakeOnLanState.devices.map(device => {
        const pending = wakeOnLanState.waking.has(wakeOnLanTargetKey(device));
        return `<li id="wol-device-${encodeURIComponent(device.mac).replace(/%/g, '_')}" class="wake-on-lan-device" data-wake-on-lan-mac="${escapeHtml(device.mac)}"><div class="wake-on-lan-device__identity"><strong>${escapeHtml(device.name)}</strong><code>${escapeHtml(device.mac)}</code><span>${escapeHtml(device.ip)}</span></div><div class="wake-on-lan-device__action"><button type="button" class="ui-button ui-button--primary ui-button--small" data-wake-on-lan-wake="${escapeHtml(device.mac)}" data-wake-on-lan-ip="${escapeHtml(device.ip)}" ${pending || !wakeOnLanCurrent(wakeOnLanState.binding, generation) ? 'disabled' : ''}>Wake</button></div></li>`;
      }).join('') || `<li class="wake-on-lan-empty">${wakeOnLanState.hasSnapshot ? 'No DHCP devices are available.' : 'Waiting for a current device list.'}</li>`;
      morphLivePane(rows, html);
    }
    function invalidateWakeOnLanModal() {
      const modal = wakeOnLanModal();
      const binding = wakeOnLanState.binding;
      const controller = wakeOnLanState.readController;
      const changed = Boolean(modal || controller || wakeOnLanState.hasSnapshot || wakeOnLanState.devices.length);
      wakeOnLanState.generation += 1;
      wakeOnLanState.readController = null;
      if (controller) controller.abort();
      if (binding) scaffoldRegistry.clearRetry(binding, WAKE_ON_LAN_READ_KEY);
      modal?.remove();
      wakeOnLanState.devices = [];
      wakeOnLanState.currentData = false;
      wakeOnLanState.stale = true;
      wakeOnLanState.hasSnapshot = false;
      return changed;
    }
    function closeWakeOnLanModal() {
      if (!wakeOnLanModal()) return false;
      closeInfoModal();
      return true;
    }
    function scrubWakeOnLan() {
      if (!closeWakeOnLanModal()) invalidateWakeOnLanModal();
      wakeOnLanState.binding = null;
      wakeOnLanState.devices = [];
      wakeOnLanState.currentData = false;
      wakeOnLanState.stale = true;
      wakeOnLanState.hasSnapshot = false;
    }
    function retireWakeOnLanPrivilegedState() { scrubWakeOnLan(); }
    window.addEventListener('coronatio:privileged-state-retired', retireWakeOnLanPrivilegedState);
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {};
      const pane = detail.pane;
      if (detail.id !== 'stats' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="stats"]') !== pane) return;
      scrubWakeOnLan();
    });
    function bindWakeOnLanRetirement(binding) {
      if (wakeOnLanRetirementBindings.has(binding)) return;
      wakeOnLanRetirementBindings.add(binding);
      scaffoldRegistry.onRetire(binding, () => {
        if (wakeOnLanState.binding !== binding) return;
        scrubWakeOnLan();
      });
    }
    async function hydrateWakeOnLan() {
      if (!headerState.isAdmin || appRoot?.dataset.adminMode !== 'true' || !wakeOnLanStatsIsActive()
          || !infoBackdrop.classList.contains('open') || !wakeOnLanModal()) return false;
      const binding = scaffoldRegistry.capture('stats');
      if (!binding || !wakeOnLanRoot(binding)) return false;
      const generation = ++wakeOnLanState.generation;
      const previousController = wakeOnLanState.readController;
      wakeOnLanState.readController = null;
      if (previousController) previousController.abort();
      const previousBinding = wakeOnLanState.binding;
      if (previousBinding && previousBinding !== binding) scaffoldRegistry.clearRetry(previousBinding, WAKE_ON_LAN_READ_KEY);
      wakeOnLanState.binding = binding;
      bindWakeOnLanRetirement(binding);
      const controller = new AbortController();
      wakeOnLanState.readController = controller;
      wakeOnLanState.currentData = false;
      wakeOnLanState.stale = true;
      wakeOnLanSetReadState(generation, 'pending');
      const error = wakeOnLanModal()?.querySelector('[data-wake-on-lan-error]');
      if (error) error.hidden = true;
      renderWakeOnLan(generation);
      return scaffoldRegistry.run(binding, WAKE_ON_LAN_READ_KEY, async context => {
        if (!context.current() || !wakeOnLanOwnerIsCurrent(generation, binding)) return false;
        const payload = await context.fetchJson('/api/wakeonlan/devices');
        if (!context.current() || !wakeOnLanOwnerIsCurrent(generation, binding)) return false;
        if (!payload || typeof payload !== 'object' || Array.isArray(payload) || payload.ok === false) throw new Error('Wake-on-LAN device response was refused');
        const collections = wakeOnLanRows(payload);
        const devices = new Map();
        [...collections.leases, ...collections.reservations].forEach(row => {
          const device = wakeOnLanDevice(row);
          if (device) devices.set(device.mac, device);
        });
        if (!context.current() || !wakeOnLanOwnerIsCurrent(generation, binding)) return false;
        wakeOnLanState.devices = [...devices.values()];
        wakeOnLanState.hasSnapshot = true;
        wakeOnLanState.currentData = true;
        wakeOnLanState.stale = false;
        renderWakeOnLan(generation);
        wakeOnLanSetReadState(generation, 'ready');
        wakeOnLanSetError(generation, '', false, true);
        return true;
      }, {
        slots: [], timeoutMs: 8000, slowMs: 750, signal: controller.signal, supersede: true,
        onFailure: ({ current, reason }) => {
          if (!current() || !wakeOnLanOwnerIsCurrent(generation, binding)) return;
          wakeOnLanState.currentData = false;
          wakeOnLanState.stale = true;
          renderWakeOnLan(generation);
          wakeOnLanSetReadState(generation, 'fault');
          wakeOnLanSetError(generation, `Device list unavailable: ${reason || 'Read failed'}. ${wakeOnLanState.hasSnapshot ? 'Showing last known devices' : 'No current device list is available'}; wake is disabled.`, true, false);
        }
      }).finally(() => {
        if (wakeOnLanState.readController === controller) wakeOnLanState.readController = null;
      });
    }
    document.addEventListener('click', async event => {
      const target = event.target instanceof Element ? event.target : null;
      if (!target) return;
      const opener = target.closest('[data-wake-on-lan-open]');
      if (opener) {
        if (!headerState.isAdmin || appRoot?.dataset.adminMode !== 'true' || !wakeOnLanStatsIsActive() || opener.hidden || !opener.isConnected) return;
        event.preventDefault();
        openInfoModal('Wake on LAN', 'wake-on-lan');
        void hydrateWakeOnLan();
        return;
      }
      const retry = target.closest('[data-wake-on-lan-retry]');
      if (retry) {
        if (!wakeOnLanOwnerIsCurrent() || retry.disabled) return;
        event.preventDefault();
        retry.disabled = true;
        void hydrateWakeOnLan();
        return;
      }
      const button = target.closest('[data-wake-on-lan-wake]');
      const binding = wakeOnLanState.binding;
      const generation = wakeOnLanState.generation;
      if (!button || !wakeOnLanCurrent(binding, generation) || !wakeOnLanModal()?.contains(button) || button.disabled) return;
      const device = wakeOnLanState.devices.find(candidate => candidate.mac === button.dataset.wakeOnLanWake && candidate.ip === button.dataset.wakeOnLanIp);
      if (!device) return;
      const key = wakeOnLanTargetKey(device);
      if (wakeOnLanState.waking.has(key)) return;
      wakeOnLanState.waking.add(key);
      button.disabled = true;
      try {
        await wakeOnLanJson('/api/wakeonlan/wake', { method: 'POST', body: JSON.stringify({ mac: device.mac, ip: device.ip }) });
        if (wakeOnLanCurrent(binding, generation)) showCoronatioToast('Packet sent — waiting…', 'success');
      } catch (failure) {
        if (wakeOnLanCurrent(binding, generation)) showCoronatioToast(wakeOnLanMessage(failure), 'error');
      } finally {
        wakeOnLanState.waking.delete(key);
        const currentBinding = wakeOnLanState.binding;
        const currentGeneration = wakeOnLanState.generation;
        if (!wakeOnLanCurrent(currentBinding, currentGeneration)) return;
        const matchingButton = [...(wakeOnLanModal()?.querySelectorAll('[data-wake-on-lan-wake]') || [])]
          .find(candidate => candidate.dataset.wakeOnLanWake === device.mac && candidate.dataset.wakeOnLanIp === device.ip);
        if (matchingButton) matchingButton.disabled = false;
      }
    });
    window.addEventListener('pagehide', () => scrubWakeOnLan());
    document.addEventListener('visibilitychange', () => { if (document.visibilityState !== 'visible') scrubWakeOnLan(); });
"####
}
