fn shell_firewall_client() -> &'static str {
    r####"    const firewallState = { observed: [], children: [], observedReady: false, childrenReady: false, binding: null, selectedMac: '', hosts: [], revision: undefined, whitelistReady: false, busy: new Map(), confirm: null };
    function firewallRows(payload, key) {
      for (const item of [payload, payload?.payload?.result, payload?.payload, payload?.result, payload?.data, payload?.receipt]) if (Array.isArray(item?.[key])) return item[key];
      if (key === 'hostnames') return payload?.device?.whitelist || payload?.payload?.device?.whitelist || payload?.receipt?.device?.whitelist || null;
      return null;
    }
    function firewallMac(row) { return String(typeof row === 'string' ? row : row?.mac || row?.macAddress || row?.['hw-address'] || row?.hwAddress || '').trim().toUpperCase(); }
    function firewallMessage(payload) {
      const p = payload?.payload?.firstMissingSignal ? payload.payload : payload;
      const words = { 'firewall-allowlist-empty': 'Add at least one website.', 'firewall-hostname-global-zone-conflict': 'Home names are always allowed; remove it from the list.', 'firewall-revision-conflict': 'This list changed elsewhere; it was reloaded.', 'firewall-mac-not-observed': 'This device is not on the network right now.' };
      return words[p?.firstMissingSignal] || p?.message || p?.error || 'Firewall controls are unavailable.';
    }
    function firewallCurrent(binding = firewallState.binding) { return Boolean(binding && firewallState.binding === binding && headerState.isAdmin && window.scaffoldRegistry?.current(binding) && binding.guest === 'firewall'); }
    function firewallRefusal(body) { return !body || body.ok === false || body.success === false || body.payload?.ok === false || body.payload?.success === false; }
    function firewallAcceptArray(body, key) {
      if (firewallRefusal(body)) throw new Error(firewallMessage(body));
      const rows = firewallRows(body, key); if (!rows || rows.some(row => !firewallMac(row))) throw new Error(`Invalid ${key} data`);
      return rows.map(row => typeof row === 'string' ? { mac: firewallMac(row) } : { mac: firewallMac(row), hostname: typeof row.hostname === 'string' ? row.hostname : '', ip: typeof row.ip === 'string' ? row.ip : '', registered: row.registered === true, hostnames: Array.isArray(row.hostnames) ? row.hostnames : null, enforcement: row.enforcement, way: row.way, revision: row.revision });
    }
    function firewallAdoptBinding(binding) {
      if (!binding || firewallState.binding === binding) return firewallState.binding === binding;
      if (firewallState.binding && window.scaffoldRegistry?.current(firewallState.binding)) return false;
      Object.assign(firewallState, { binding, observed: [], children: [], observedReady: false, childrenReady: false, selectedMac: '', hosts: [], revision: undefined, whitelistReady: false, confirm: null });
      firewallState.busy.clear();
      window.scaffoldRegistry.onRetire(binding, () => { if (firewallState.binding === binding) { firewallState.observedReady = false; firewallState.childrenReady = false; firewallState.whitelistReady = false; } });
      return true;
    }
    function firewallLanIp(row) { const ip = String(row?.ip || '').trim(); const match = /^192\.168\.123\.(\d{1,3})$/.exec(ip); return match && Number(match[1]) <= 255 ? ip : ''; }
    function firewallObservedName(row) {
      const name = String(row?.hostname || '').trim(); if (name) return name;
      const octet = firewallMac(row).split(':')[0]; return /^[0-9A-F]{2}$/.test(octet) && (parseInt(octet, 16) & 2) ? 'Unknown · private address' : 'Unknown';
    }
    function firewallWay(row) { return row?.way === 'blocked' ? 'blocked' : row?.way === 'allowlist' || row?.way === 'only' ? 'only' : 'open'; }
    function firewallRow(mac) {
      const observed = firewallState.observed.find(row => firewallMac(row) === mac) || {};
      const child = firewallState.children.find(row => firewallMac(row) === mac);
      return child ? { ...observed, ...child, hostname: observed.hostname || child.hostname, ip: observed.ip || child.ip, registered: true } : { ...observed, mac, registered: false };
    }
    function firewallSegmented(row) {
      const mac = firewallMac(row), busy = firewallState.busy.has(mac), way = busy ? firewallState.busy.get(mac) : firewallWay(row);
      const name = `firewall-way-${mac}`;
      return `<fieldset class="ui-segmented"${busy ? ' aria-busy="true" data-busy' : ''}><legend class="ui-segmented__legend">${escapeHtml(firewallObservedName(row))}</legend>${[['open','Open'],['only','Only these sites'],['blocked','Blocked']].map(([value, word]) => `<label class="ui-segmented__option"><input class="ui-segmented__input" type="radio" name="${escapeHtml(name)}" value="${value}"${way === value ? ' checked' : ''}><span class="ui-segmented__word">${word}</span></label>`).join('')}</fieldset>`;
    }
    function renderFirewall(binding = firewallState.binding, preserveReadySlots = false) {
      if (!firewallCurrent(binding)) return;
      const observed = binding.root.querySelector('[data-firewall-observed]'), children = binding.root.querySelector('[data-firewall-children]');
      const sort = (a,b) => firewallObservedName(a).localeCompare(firewallObservedName(b)) || firewallMac(a).localeCompare(firewallMac(b));
      const rowHtml = (row, other = false) => {
        const mac = firewallMac(row), way = firewallWay(row);
        const status = way === 'blocked' ? 'Blocked · no internet' : way === 'only' && Array.isArray(row.hostnames) ? `${row.hostnames.length} ${row.hostnames.length === 1 ? 'site' : 'sites'}` : '';
        const detail = [status || (other ? `${String(row.ip || '').trim() || '—'} · ${mac}` : firewallLanIp(row) || '—'), row.enforcement === 'pending-renewal' ? 'takes effect when it reconnects' : ''].filter(Boolean).join(' · ');
        return `<li class="ui-setting-row ui-setting-row--ruled ui-setting-row--inline firewall-device" data-scaffold-key="firewall:${escapeHtml(mac)}" data-firewall-device="${escapeHtml(mac)}"><span class="firewall-row-text"><strong>${escapeHtml(firewallObservedName(row))}</strong><span>${escapeHtml(detail)}</span></span>${firewallSegmented(row)}</li>`;
      };
      const policies = firewallState.children.map(row => firewallRow(firewallMac(row))).sort(sort);
      const open = firewallState.observed.filter(row => !firewallState.children.some(child => firewallMac(child) === firewallMac(row))).sort(sort);
      const main = open.filter(row => String(row.hostname || '').trim() || firewallLanIp(row));
      const other = open.filter(row => !String(row.hostname || '').trim() && !firewallLanIp(row));
      if (children && scaffoldReadProjectionAllowed(children, preserveReadySlots)) morphLivePane(children, policies.map(row => rowHtml(row)).join(''));
      if (observed && scaffoldReadProjectionAllowed(observed, preserveReadySlots)) { const expanded = Boolean(observed.querySelector('details')?.open); morphLivePane(observed, `<ul class="firewall-list">${main.map(row => rowHtml(row)).join('')}</ul><details class="firewall-other"${expanded ? ' open' : ''}><summary>Other addresses (${other.length})</summary><ul class="firewall-list">${other.map(row => rowHtml(row, true)).join('')}</ul></details>`); }
      binding.root.querySelectorAll('[data-firewall-device] input').forEach(input => { const mac = input.closest('[data-firewall-device]').dataset.firewallDevice; input.disabled = !firewallState.observedReady || !firewallState.childrenReady || firewallState.busy.has(mac); });
    }
    function firewallRead(binding, key, route, field, slotName, options = {}) {
      const target = binding.root.querySelector(`[data-scaffold-slot="${slotName}"]`); if (!target) return Promise.resolve(false);
      return scaffoldRegistry.run(binding, key, async ctx => {
        if (!ctx.current() || !firewallCurrent(binding)) return false;
        if (field === 'observed') firewallState.observedReady = false; else firewallState.childrenReady = false;
        renderFirewall(binding, true); const body = await ctx.fetchJson(route);
        if (!ctx.current() || !firewallCurrent(binding)) return false;
        const rows = firewallAcceptArray(body, field);
        if (field === 'observed') { firewallState.observed = rows; firewallState.observedReady = true; } else { firewallState.children = rows; firewallState.childrenReady = true; }
        ctx.ready(target); renderFirewall(binding); return true;
      }, { slots: [target], supersede: options.supersede === true, onFailure: ({ current, reason }) => { if (current() && firewallCurrent(binding)) { if (field === 'observed') firewallState.observedReady = false; else firewallState.childrenReady = false; firewallShowError(binding, reason || 'Firewall controls are unavailable.'); renderFirewall(binding, true); } } });
    }
    function hydrateFirewall(options = {}) {
      const binding = scaffoldRegistry.capture('firewall');
      if (!binding || !headerState.isAdmin || !viewportFamilyAdmitted('firewall') || !firewallAdoptBinding(binding)) return Promise.resolve(false);
      return Promise.all([firewallRead(binding, 'firewall-observed', '/api/firewall/observed', 'observed', 'firewall-observed', options), firewallRead(binding, 'firewall-children', '/api/firewall/children', 'children', 'firewall-children', options)]);
    }
    function firewallShowError(binding, text) { const error = binding.root.querySelector('[data-firewall-error]'); if (error) { error.textContent = text || ''; error.hidden = !text; } }
    function firewallModalElement() { return firewallState.binding?.root?.querySelector('[data-firewall-modal-backdrop]') || null; }
    function closeFirewallModal() {
      const modal = firewallModalElement(); if (modal) closeCrownDialog(modal);
      Object.assign(firewallState, { selectedMac: '', hosts: [], revision: undefined, whitelistReady: false });
    }
    function renderFirewallHosts(binding = firewallState.binding) {
      if (!firewallCurrent(binding)) return; const target = binding.root.querySelector('[data-firewall-hosts]'); if (!target) return;
      morphLivePane(target, firewallState.hosts.map(host => `<li class="ui-setting-row ui-setting-row--ruled ui-setting-row--inline" data-scaffold-key="firewall-host:${escapeHtml(host)}"><code>${escapeHtml(host)}</code><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-remove-host="${escapeHtml(host)}">Remove</button></li>`).join(''));
    }
    async function openFirewallModal(mac, opener) {
      const binding = firewallState.binding, row = firewallRow(mac); if (!firewallCurrent(binding) || !firewallState.childrenReady || !firewallState.observedReady) return;
      firewallState.selectedMac = mac; firewallState.hosts = Array.isArray(row.hostnames) ? row.hostnames.slice() : []; firewallState.revision = row.revision; firewallState.whitelistReady = !row.registered || (Array.isArray(row.hostnames) && row.revision != null);
      const modal = firewallModalElement(); modal.querySelector('[data-firewall-modal-title]').textContent = `${firewallObservedName(row)} can reach only these websites.`;
      openCrownDialog(modal, opener); renderFirewallHosts(binding);
      if (row.registered && !firewallState.whitelistReady) {
        try {
          const response = await fetch(`/api/firewall/children/${encodeURIComponent(mac)}/whitelist`, { cache: 'no-store', credentials: 'same-origin' }); const body = await response.json();
          if (!response.ok || firewallRefusal(body)) throw body;
          const hosts = firewallRows(body, 'hostnames'), revision = body.revision ?? body.payload?.revision ?? body.payload?.result?.revision;
          if (!hosts || revision == null) throw new Error('Website list is unavailable.');
          if (!firewallCurrent(binding) || firewallState.selectedMac !== mac) return;
          firewallState.hosts = hosts.slice(); firewallState.revision = revision; firewallState.whitelistReady = true; renderFirewallHosts(binding);
        } catch (failure) { if (firewallCurrent(binding)) firewallShowError(binding, firewallMessage(failure)); }
      }
    }
    async function firewallWrite(binding, mac, way, hosts, openedRevision) {
      const row = firewallRow(mac); if (!firewallCurrent(binding) || firewallState.busy.has(mac) || !firewallState.childrenReady || !firewallState.observedReady) return false;
      const route = way === 'open' ? `/api/firewall/children/${encodeURIComponent(mac)}` : row.registered ? '/api/firewall/way' : '/api/firewall/children';
      const data = { mac, way: way === 'only' ? 'allowlist' : way };
      if (way === 'only') data.hostnames = hosts;
      if (row.registered && way !== 'open') data.revision = way === 'only' ? openedRevision : row.revision;
      if (row.registered && way !== 'open' && data.revision == null) { firewallShowError(binding, 'This device’s setting could not be loaded. Refresh and try again.'); return false; }
      firewallShowError(binding, ''); firewallState.busy.set(mac, way); renderFirewall(binding);
      const controller = new AbortController(); let timedOut = false;
      const timer = setTimeout(() => { timedOut = true; controller.abort(); }, 30000);
      try {
        const response = await fetch(route, { method: way === 'open' ? 'DELETE' : 'POST', cache: 'no-store', credentials: 'same-origin', signal: controller.signal, headers: way === 'open' ? {} : { 'content-type': 'application/json' }, ...(way === 'open' ? {} : { body: JSON.stringify(data) }) });
        const body = await response.json().catch(() => null); if (!response.ok || firewallRefusal(body)) throw body || new Error(`Request failed (${response.status})`);
        if (!firewallCurrent(binding)) return false;
        await hydrateFirewall({ supersede: true }); return true;
      } catch (failure) {
        if (!firewallCurrent(binding)) return false;
        const result = await hydrateFirewall({ supersede: true });
        if (!firewallCurrent(binding)) return false;
        if (!result || result.some(ok => !ok)) { firewallShowError(binding, 'The setting may have changed. Refresh to see its current state.'); return false; }
        const held = firewallRow(mac), heldWay = firewallWay(held);
        const matched = heldWay === way && (way !== 'only' || (Array.isArray(held.hostnames) && held.hostnames.length === hosts.length && hosts.every(host => held.hostnames.includes(host))));
        if (matched) return true;
        if (timedOut || failure?.message === 'Read timed out') return false;
        if (firewallCurrent(binding)) {
          const signal = failure?.firstMissingSignal || failure?.payload?.firstMissingSignal;
          if (signal === 'firewall-revision-conflict' && firewallState.selectedMac === mac) await openFirewallModal(mac);
          firewallShowError(binding, firewallMessage(failure));
          const sheetError = firewallModalElement()?.querySelector('[data-firewall-whitelist-error]');
          if (sheetError && firewallModalElement().open) { sheetError.hidden = false; sheetError.textContent = firewallMessage(failure); }
        }
        return false;
      } finally { clearTimeout(timer); firewallState.busy.delete(mac); if (firewallCurrent(binding)) renderFirewall(binding); }
    }
    document.body.addEventListener('submit', event => {
      const form = event.target.closest?.('[data-firewall-host-form]'), binding = firewallState.binding; if (!form || !firewallCurrent(binding) || !binding.root.contains(form)) return;
      event.preventDefault(); if (!firewallState.whitelistReady) return; const input = form.querySelector('[data-firewall-host-input]'), host = String(input.value || '').trim().toLowerCase().replace(/\.$/, '');
      if (host && !firewallState.hosts.includes(host)) { firewallState.hosts.push(host); input.value = ''; renderFirewallHosts(binding); }
    });
    document.addEventListener('click', async event => {
      const binding = firewallState.binding, pane = binding?.root; if (!pane || !pane.contains(event.target)) return;
      const control = selector => { const item = event.target.closest?.(selector); return item && pane.contains(item) ? item : null; };
      if (control('[data-firewall-modal-cancel]')) { closeFirewallModal(); renderFirewall(binding); return; }
      if (control('[data-firewall-confirm-cancel]')) { closeCrownDialog(pane.querySelector('[data-firewall-confirm]')); firewallState.confirm = null; renderFirewall(binding); return; }
      if (control('[data-firewall-refresh]')) { if (firewallCurrent(binding)) await hydrateFirewall({ supersede: true }); return; }
      const radio = control('[data-firewall-device] input[value="only"]');
      if (radio && !radio.disabled) { event.preventDefault(); const mac = radio.closest('[data-firewall-device]').dataset.firewallDevice; renderFirewall(binding); await openFirewallModal(mac, radio); return; }
      const remove = control('[data-firewall-remove-host]'); if (remove && firewallState.whitelistReady) { firewallState.hosts = firewallState.hosts.filter(host => host !== remove.dataset.firewallRemoveHost); renderFirewallHosts(binding); return; }
      if (control('[data-firewall-confirm-save]')) { const pending = firewallState.confirm; firewallState.confirm = null; closeCrownDialog(pane.querySelector('[data-firewall-confirm]')); if (pending) await firewallWrite(binding, pending.mac, pending.way); return; }
      if (control('[data-firewall-modal-save]')) {
        const mac = firewallState.selectedMac; if (!firewallCurrent(binding) || !mac || !firewallState.whitelistReady) return;
        if (!firewallState.hosts.length) { firewallShowError(binding, 'Add at least one website.'); const error = pane.querySelector('[data-firewall-whitelist-error]'); if (error) { error.hidden = false; error.textContent = 'Add at least one website.'; } return; }
        const hosts = firewallState.hosts.slice(); const ok = await firewallWrite(binding, mac, 'only', hosts, firewallState.revision); if (ok) closeFirewallModal();
      }
    }, true);
    document.addEventListener('change', async event => {
      const input = event.target.closest?.('[data-firewall-device] input.ui-segmented__input'), binding = firewallState.binding;
      if (!input || !firewallCurrent(binding) || !binding.root.contains(input) || input.disabled || input.value === 'only') return;
      const mac = input.closest('[data-firewall-device]').dataset.firewallDevice, row = firewallRow(mac), way = input.value;
      if (way === firewallWay(row)) return;
      if (firewallWay(row) === 'only' && !Array.isArray(row.hostnames)) { renderFirewall(binding); firewallShowError(binding, 'The website list could not be loaded. Refresh and try again.'); return; }
      if (firewallWay(row) === 'only' && row.hostnames.length) {
        firewallState.confirm = { mac, way }; const dialog = binding.root.querySelector('[data-firewall-confirm]'); dialog.querySelector('[data-firewall-confirm-title]').textContent = `Forget ${row.hostnames.length} sites?`; renderFirewall(binding); openCrownDialog(dialog, input); return;
      }
      await firewallWrite(binding, mac, way);
    });
    window.addEventListener('coronatio:privileged-state-retired', () => {
      const binding = firewallState.binding; if (binding) { const sheet = firewallModalElement(), confirm = binding.root.querySelector('[data-firewall-confirm]'); if (sheet) closeCrownDialog(sheet); if (confirm) closeCrownDialog(confirm); }
      window.scaffoldRegistry?.retire('firewall', 'Admin role retired');
      Object.assign(firewallState, { observed: [], children: [], hosts: [], revision: undefined, selectedMac: '', observedReady: false, childrenReady: false, whitelistReady: false, binding: null, confirm: null }); firewallState.busy.clear();
      document.querySelectorAll('[data-pane-panel="firewall"] [data-firewall-observed], [data-pane-panel="firewall"] [data-firewall-children], [data-pane-panel="firewall"] [data-firewall-hosts]').forEach(node => node.replaceChildren());
    });
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane, binding = firewallState.binding;
      if (detail.id !== 'firewall' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="firewall"]') !== pane) return;
      if (binding && (binding.guest !== detail.id || window.scaffoldRegistry?.current(binding))) return;
      Object.assign(firewallState, { observed: [], children: [], hosts: [], revision: undefined, selectedMac: '', observedReady: false, childrenReady: false, whitelistReady: false, binding: null, confirm: null }); firewallState.busy.clear();
    });
"####
}
