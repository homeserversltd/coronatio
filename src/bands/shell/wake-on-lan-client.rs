fn shell_wake_on_lan_client() -> &'static str {
    r####"    const wakeOnLanState = { devices: [], binding: null, currentData: false, stale: false, hasSnapshot: false };
    function wakeOnLanRoot(binding = wakeOnLanState.binding) { return binding?.root instanceof Element ? binding.root : null; }
    function wakeOnLanCurrent(binding = wakeOnLanState.binding) { return Boolean(binding && wakeOnLanState.binding === binding && wakeOnLanState.currentData && !wakeOnLanState.stale && viewportFamilyAdmitted('wake-on-lan') && scaffoldRegistry.current(binding)); }
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
    async function wakeOnLanJson(route, options = {}) { const response = await fetch(route, { cache: 'no-store', headers: options.body ? { 'content-type': 'application/json' } : {}, ...options }); const body = await response.json().catch(() => ({})); if (!response.ok || body?.ok === false) throw body; return body; }
    function wakeOnLanSetError(binding, message, retry = false, hidden = false) {
      if (!binding || !scaffoldRegistry.current(binding)) return;
      const error = wakeOnLanRoot(binding)?.querySelector('[data-wake-on-lan-error]'); if (!error) return;
      error.hidden = hidden; error.textContent = message;
      if (retry) { const button = document.createElement('button'); button.type = 'button'; button.dataset.wakeOnLanRetry = ''; button.textContent = 'Retry'; error.append(document.createTextNode(' '), button); }
    }
    function renderWakeOnLan(binding = wakeOnLanState.binding) {
      if (!binding || !scaffoldRegistry.current(binding)) return;
      const rows = wakeOnLanRoot(binding)?.querySelector('[data-wake-on-lan-rows]'); if (!rows) return;
      const html = wakeOnLanState.devices.map(device => `<li id="wol-device-${encodeURIComponent(device.mac).replace(/%/g, '_')}" class="wake-on-lan-device" data-wake-on-lan-mac="${escapeHtml(device.mac)}"><div class="wake-on-lan-device__identity"><strong>${escapeHtml(device.name)}</strong><code>${escapeHtml(device.mac)}</code><span>${escapeHtml(device.ip)}</span></div><div class="wake-on-lan-device__action"><button type="button" class="ui-button ui-button--primary ui-button--small" data-wake-on-lan-wake="${escapeHtml(device.mac)}" ${wakeOnLanCurrent(binding) ? '' : 'disabled'}>Wake</button></div></li>`).join('') || `<li class="wake-on-lan-empty" data-wake-on-lan-empty>${wakeOnLanState.hasSnapshot ? 'No DHCP devices are available.' : 'Waiting for a current device list.'}</li>`;
      morphLivePane(rows, html);
    }
    function retireWakeOnLan(binding) {
      if (wakeOnLanState.binding !== binding) return;
      wakeOnLanState.binding = null; wakeOnLanState.currentData = false; wakeOnLanState.stale = true;
      const rows = wakeOnLanRoot(binding)?.querySelector('[data-wake-on-lan-rows]');
      rows?.querySelectorAll('[data-wake-on-lan-wake]').forEach(button => { button.disabled = true; });
    }
    function retireWakeOnLanPrivilegedState() {
      const binding = wakeOnLanState.binding;
      if (binding) scaffoldRegistry.retire(binding, 'Wake-on-LAN privileged state retired');
      document.querySelectorAll('[data-wake-on-lan-wake]').forEach(button => { button.disabled = true; });
      document.querySelectorAll('[data-wake-on-lan-rows]').forEach(rows => { rows.replaceChildren(); });
      document.querySelectorAll('[data-wake-on-lan-error]').forEach(status => {
        status.hidden = false;
        status.textContent = 'Wake-on-LAN unavailable while Admin Mode is off.';
      });
      wakeOnLanState.devices = []; wakeOnLanState.hasSnapshot = false; wakeOnLanState.currentData = false; wakeOnLanState.stale = true; wakeOnLanState.binding = null;
    }
    window.addEventListener('coronatio:privileged-state-retired', retireWakeOnLanPrivilegedState);
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane, binding = wakeOnLanState.binding;
      if (detail.id !== 'wake-on-lan' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="wake-on-lan"]') !== pane) return;
      if (binding && (binding.guest !== detail.id || scaffoldRegistry.current(binding))) return;
      wakeOnLanState.devices = []; wakeOnLanState.hasSnapshot = false; wakeOnLanState.currentData = false; wakeOnLanState.stale = false; wakeOnLanState.binding = null;
    });
    async function hydrateWakeOnLan() {
      const binding = viewportFamilyAdmitted('wake-on-lan') ? scaffoldRegistry.capture('wake-on-lan') : null;
      if (!binding) return false;
      if (wakeOnLanState.binding !== binding) {
        const previous = wakeOnLanState.binding;
        if (previous) scaffoldRegistry.retire(previous, 'Wake-on-LAN binding replaced');
        wakeOnLanState.binding = binding; wakeOnLanState.currentData = false; wakeOnLanState.stale = true;
        scaffoldRegistry.onRetire(binding, () => retireWakeOnLan(binding));
      }
      const rowsSlot = binding.root.querySelector('[data-scaffold-slot="wakeonlan-devices"]');
      if (!rowsSlot) return Promise.resolve(false);
      return scaffoldRegistry.run(binding, 'wakeonlan-devices', async context => {
        if (!context.current() || !viewportFamilyAdmitted('wake-on-lan') || wakeOnLanState.binding !== binding) return false;
        wakeOnLanState.currentData = false; wakeOnLanState.stale = true;
        renderWakeOnLan(binding);
        wakeOnLanSetError(binding, 'Refreshing device list; wake is disabled until current data arrives.', false, false);
        const payload = await context.fetchJson('/api/wakeonlan/devices');
        if (!context.current() || !viewportFamilyAdmitted('wake-on-lan') || wakeOnLanState.binding !== binding) return false;
        if (!payload || typeof payload !== 'object' || Array.isArray(payload) || payload.ok === false) throw new Error('Wake-on-LAN device response was refused');
        const collections = wakeOnLanRows(payload);
        const devices = new Map();
        [...collections.leases, ...collections.reservations].forEach(row => { const device = wakeOnLanDevice(row); if (device) devices.set(device.mac, device); });
        if (!context.current() || !viewportFamilyAdmitted('wake-on-lan') || wakeOnLanState.binding !== binding) return false;
        wakeOnLanState.devices = [...devices.values()]; wakeOnLanState.hasSnapshot = true; wakeOnLanState.currentData = true; wakeOnLanState.stale = false;
        renderWakeOnLan(binding); wakeOnLanSetError(binding, '', false, true);
        context.ready(rowsSlot);
        return true;
      }, {
        slots: [rowsSlot], timeoutMs: 8000, slowMs: 750,
        onFailure: ({ current, reason }) => {
          if (!current() || !scaffoldRegistry.current(binding)) return;
          wakeOnLanState.currentData = false; wakeOnLanState.stale = true;
          renderWakeOnLan(binding);
          wakeOnLanSetError(binding, `Device list unavailable: ${reason || 'Read failed'}. ${wakeOnLanState.hasSnapshot ? 'Showing last known devices' : 'No current device list is available'}; wake is disabled.`, true, false);
          wakeOnLanRoot(binding)?.querySelectorAll('[data-wake-on-lan-wake]').forEach(button => { button.disabled = true; });
        }
      });
    }
    document.addEventListener('click', async event => {
      const target = event.target instanceof Element ? event.target : null; if (!target) return;
      const retry = target.closest('[data-wake-on-lan-retry]');
      if (retry) { const binding = scaffoldRegistry.capture('wake-on-lan'); if (!binding) return; event.preventDefault(); retry.disabled = true; scaffoldRegistry.retry(binding, 'wakeonlan-devices').finally(() => { if (retry.isConnected) retry.disabled = false; }); return; }
      const button = target.closest('[data-wake-on-lan-wake]'); if (!button) return;
      const binding = scaffoldRegistry.capture('wake-on-lan');
      if (!wakeOnLanCurrent(binding) || !wakeOnLanRoot(binding)?.contains(button) || button.disabled) return;
      const device = wakeOnLanState.devices.find(candidate => candidate.mac === button.dataset.wakeOnLanWake); if (!device) return;
      button.disabled = true;
      try { await wakeOnLanJson('/api/wakeonlan/wake', { method: 'POST', body: JSON.stringify({ mac: device.mac, ip: device.ip }) }); if (wakeOnLanCurrent(binding)) showCoronatioToast('Packet sent — waiting…', 'success'); }
      catch (failure) { if (wakeOnLanCurrent(binding)) showCoronatioToast(wakeOnLanMessage(failure), 'error'); }
      finally { if (wakeOnLanCurrent(binding) && button.isConnected) button.disabled = false; }
    });
"####
}