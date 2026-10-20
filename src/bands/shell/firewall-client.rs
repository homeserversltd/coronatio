fn shell_firewall_client() -> &'static str {
    r####"    const firewallState = { observed: [], children: [], observedReady: false, childrenReady: false, binding: null, selectedMac: '', hosts: [], revision: undefined, whitelistReady: false };
    function firewallRows(payload, key) {
      if (Array.isArray(payload?.[key])) return payload[key];
      if (Array.isArray(payload?.payload?.result?.[key])) return payload.payload.result[key];
      if (Array.isArray(payload?.payload?.[key])) return payload.payload[key];
      if (Array.isArray(payload?.result?.[key])) return payload.result[key];
      if (Array.isArray(payload?.data?.[key])) return payload.data[key];
      if (Array.isArray(payload?.receipt?.[key])) return payload.receipt[key];
      if (key === 'hostnames' && (Array.isArray(payload?.device?.whitelist) || Array.isArray(payload?.payload?.device?.whitelist) || Array.isArray(payload?.receipt?.device?.whitelist))) return payload.device?.whitelist || payload.payload?.device?.whitelist || payload.receipt?.device?.whitelist;
      return null;
    }
    function firewallMac(row) { return String(typeof row === 'string' ? row : row?.mac || row?.macAddress || row?.['hw-address'] || row?.hwAddress || '').trim().toUpperCase(); }
    function firewallMessage(payload) { return payload?.message || payload?.firstMissingSignal || payload?.error || 'Child-device controls are unavailable'; }
    function firewallCurrent(binding = firewallState.binding) { return Boolean(binding && firewallState.binding === binding && headerState.isAdmin && window.scaffoldRegistry?.current(binding) && binding.guest === 'firewall'); }
    function firewallRefusal(body) {
      if (!body || typeof body !== 'object') return 'Read returned invalid data';
      if (body.ok === false || body.success === false) return firewallMessage(body);
      if (body.payload && typeof body.payload === 'object' && (body.payload.ok === false || body.payload.success === false)) return firewallMessage(body.payload);
      return null;
    }
    function firewallAcceptArray(body, key) {
      const refusal = firewallRefusal(body); if (refusal) throw new Error(refusal);
      const rows = firewallRows(body, key); if (!rows) throw new Error(`Missing ${key} data`);
      if (rows.some(row => !firewallMac(row))) throw new Error(`Invalid ${key} row`);
      return rows.map(row => typeof row === 'string' ? { mac: firewallMac(row) } : { mac: firewallMac(row), hostname: typeof row?.hostname === 'string' ? row.hostname : '', ip: typeof row?.ip === 'string' ? row.ip : '' });
    }
    async function firewallMutation(route, options, binding) {
      if (!firewallCurrent(binding)) throw new Error('Firewall access ended');
      const response = await fetch(route, { cache: 'no-store', credentials: 'same-origin', signal: options.signal, headers: options.body ? { 'content-type': 'application/json' } : {}, ...options });
      const body = await response.json().catch(() => null);
      if (!response.ok || !body || firewallRefusal(body)) throw body || new Error(`Request failed (${response.status})`);
      if (!firewallCurrent(binding)) throw new DOMException('Read retired', 'AbortError');
      return body;
    }
    function firewallMarkRetired(binding) {
      if (firewallState.binding !== binding) return;
      firewallState.observedReady = false; firewallState.childrenReady = false; firewallState.whitelistReady = false;
      const root = binding.root; if (!(root instanceof Element) || !root.isConnected) return;
      firewallSetControls(binding);
    }
    function firewallAdoptBinding(binding) {
      if (!binding || firewallState.binding === binding) return firewallState.binding === binding;
      const previous = firewallState.binding;
      if (previous && window.scaffoldRegistry?.current(previous)) return false;
      firewallState.binding = binding; firewallState.observedReady = false; firewallState.childrenReady = false; firewallState.whitelistReady = false;
      firewallState.selectedMac = ''; firewallState.hosts = []; firewallState.revision = undefined;
      window.scaffoldRegistry.onRetire(binding, () => firewallMarkRetired(binding));
      return true;
    }
    function firewallSetControls(binding = firewallState.binding, preserveReadySlots = false) {
      if (!binding?.root) return;
      const current = firewallCurrent(binding);
      binding.root.querySelectorAll('[data-firewall-register]').forEach(button => {
        if (!scaffoldReadProjectionAllowed(button, preserveReadySlots)) return;
        const mac = firewallMac(button.dataset.firewallRegister);
        const registered = firewallState.children.some(child => firewallMac(child) === mac);
        button.disabled = !current || !firewallState.observedReady || !firewallState.childrenReady || registered;
      });
      binding.root.querySelectorAll('[data-firewall-unregister], [data-firewall-whitelist]').forEach(button => {
        if (!scaffoldReadProjectionAllowed(button, preserveReadySlots)) return;
        button.disabled = !current || !firewallState.childrenReady;
      });
      const editing = current && firewallState.childrenReady && firewallState.whitelistReady && Boolean(firewallState.selectedMac);
      binding.root.querySelectorAll('[data-firewall-host-input], [data-firewall-host-form] button[type="submit"], [data-firewall-remove-host], [data-firewall-modal-save]').forEach(button => { if (scaffoldReadProjectionAllowed(button, preserveReadySlots)) button.disabled = !editing; });
    }
    function renderFirewall(binding = firewallState.binding, preserveReadySlots = false) {
      if (!firewallCurrent(binding)) return;
      const observed = binding.root.querySelector('[data-firewall-observed]'); const children = binding.root.querySelector('[data-firewall-children]');
      if (observed && scaffoldReadProjectionAllowed(observed, preserveReadySlots)) morphLivePane(observed, firewallState.observed.map(row => {
        const mac = firewallMac(row); const hostname = String(row?.hostname ?? '').trim() || 'Unknown'; const ip = String(row?.ip ?? '').trim() || 'Unknown';
        const registered = firewallState.childrenReady && firewallState.children.some(child => firewallMac(child) === mac);
        const disabled = !firewallState.observedReady || !firewallState.childrenReady || registered;
        const label = registered ? 'Registered' : (firewallState.childrenReady ? 'Register child device' : 'Checking registration');
        return `<li data-scaffold-key="firewall-observed:${escapeHtml(mac)}"><code>${escapeHtml(hostname)} — ${escapeHtml(ip)} — ${escapeHtml(mac)}</code><button type="button" class="ui-button ui-button--primary ui-button--small" data-firewall-register="${escapeHtml(mac)}"${disabled ? ' disabled' : ''}>${label}</button></li>`;
      }).join('') || (firewallState.observedReady ? '<li data-scaffold-key="firewall-observed-empty">No observed MAC addresses are available.</li>' : ''));
      if (children && scaffoldReadProjectionAllowed(children, preserveReadySlots)) morphLivePane(children, firewallState.children.map(row => {
        const mac = firewallMac(row);
        return `<li data-scaffold-key="firewall-child:${escapeHtml(mac)}"><code>${escapeHtml(mac)}</code><div><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-whitelist="${escapeHtml(mac)}"${firewallState.childrenReady ? '' : ' disabled'}>Website whitelist</button><button type="button" class="ui-button ui-button--danger ui-button--small" data-firewall-unregister="${escapeHtml(mac)}"${firewallState.childrenReady ? '' : ' disabled'}>Unregister</button></div></li>`;
      }).join('') || (firewallState.childrenReady ? '<li data-scaffold-key="firewall-children-empty">No child devices are registered.</li>' : ''));
      firewallSetControls(binding, preserveReadySlots);
    }
    function firewallRead(binding, key, route, field, slotName, options = {}) {
      const target = binding.root.querySelector(`[data-scaffold-slot="${slotName}"]`);
      if (!target) return Promise.resolve(false);
      return scaffoldRegistry.run(binding, key, async ctx => {
        if (!ctx.current() || !firewallCurrent(binding)) return false;
        if (field === 'observed') firewallState.observedReady = false; else firewallState.childrenReady = false;
        renderFirewall(binding, true);
        const body = await ctx.fetchJson(route);
        if (!ctx.current() || !firewallCurrent(binding)) return false;
        const rows = firewallAcceptArray(body, field);
        if (!ctx.current() || !firewallCurrent(binding)) return false;
        if (field === 'observed') { firewallState.observed = rows; firewallState.observedReady = true; }
        else { firewallState.children = rows; firewallState.childrenReady = true; }
        renderFirewall(binding);
        ctx.ready(target);
        const error = binding.root.querySelector('[data-firewall-error]'); if (error) error.hidden = true;
        return true;
      }, { slots: [target], supersede: options.supersede === true, onFailure: ({ current, reason }) => { if (!current() || !firewallCurrent(binding)) return; if (field === 'observed') firewallState.observedReady = false; else firewallState.childrenReady = false; renderFirewall(binding, true); const error = binding.root.querySelector('[data-firewall-error]'); if (error) { error.hidden = false; error.textContent = reason || 'Child-device controls are unavailable'; } } });
    }
    function hydrateFirewall(options = {}) {
      const binding = scaffoldRegistry.capture('firewall');
      if (!binding || !headerState.isAdmin || !viewportFamilyAdmitted('firewall') || !firewallAdoptBinding(binding)) return Promise.resolve(false);
      return Promise.all([firewallRead(binding, 'firewall-observed', '/api/firewall/observed', 'observed', 'firewall-observed', options), firewallRead(binding, 'firewall-children', '/api/firewall/children', 'children', 'firewall-children', options)]);
    }
    function firewallPaneElement() { return firewallState.binding?.root?.isConnected ? firewallState.binding.root : null; }
    function firewallModalElement() { return firewallPaneElement()?.querySelector('[data-firewall-modal-backdrop]') || null; }
    function closeFirewallModal() {
      const modal = firewallModalElement(); if (modal) { modal.classList.remove('open'); modal.setAttribute('aria-hidden', 'true'); }
      firewallState.selectedMac = ''; firewallState.hosts = []; firewallState.revision = undefined; firewallState.whitelistReady = false;
      if (firewallState.binding) firewallSetControls(firewallState.binding);
    }
    function firewallWhitelistRead(binding, mac, options = {}) {
      const slot = binding.root.querySelector('[data-scaffold-slot="firewall-whitelist"]'); if (!slot) return Promise.resolve(false);
      const readKey = 'firewall-whitelist';
      return scaffoldRegistry.run(binding, readKey, async ctx => {
        if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac || !firewallState.childrenReady || !firewallState.children.some(row => firewallMac(row) === mac)) return false;
        firewallState.whitelistReady = false; firewallSetControls(binding, true);
        const body = await ctx.fetchJson(`/api/firewall/children/${encodeURIComponent(mac)}/whitelist`);
        if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return false;
        const refusal = firewallRefusal(body); if (refusal) throw new Error(refusal);
        const hosts = firewallRows(body, 'hostnames');
        const revision = body.revision ?? body.payload?.revision ?? body.payload?.result?.revision;
        if (!hosts || hosts.some(host => typeof host !== 'string') || (typeof revision !== 'string' && typeof revision !== 'number')) throw new Error('Whitelist data is incomplete');
        if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return false;
        firewallState.hosts = hosts.map(host => String(host)); firewallState.revision = revision; firewallState.whitelistReady = true;
        renderFirewallHosts(binding); firewallSetControls(binding); ctx.ready(slot);
        const error = slot.querySelector('[data-firewall-whitelist-error]'); if (error) { error.hidden = true; error.textContent = ''; }
        const modal = firewallModalElement(); if (modal) { modal.classList.add('open'); modal.setAttribute('aria-hidden', 'false'); binding.root.querySelector('[data-firewall-host-input]')?.focus(); }
        return true;
      }, { slots: [slot], supersede: options.supersede === true, onFailure: ({ current, reason }) => { if (!current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return; firewallState.whitelistReady = false; firewallSetControls(binding, true); const error = slot.querySelector('[data-firewall-whitelist-error]'); if (error && scaffoldReadProjectionAllowed(error, true)) { error.hidden = false; error.textContent = reason || 'Whitelist is unavailable'; } } });
    }
    async function openFirewallModal(mac) {
      const binding = scaffoldRegistry.capture('firewall'); const value = String(mac || '').trim().toUpperCase();
      if (!binding || !firewallCurrent(binding) || !firewallState.childrenReady || !firewallState.children.some(row => firewallMac(row) === value)) return;
      if (firewallState.selectedMac !== value) { firewallState.hosts = []; firewallState.revision = undefined; }
      firewallState.selectedMac = value; firewallState.whitelistReady = false;
      const modal = firewallModalElement(); if (modal) { modal.classList.add('open'); modal.setAttribute('aria-hidden', 'false'); }
      firewallSetControls(binding); await firewallWhitelistRead(binding, value, { supersede: true });
    }
    function renderFirewallHosts(binding = firewallState.binding) {
      if (!firewallCurrent(binding)) return;
      const target = binding.root.querySelector('[data-firewall-hosts]'); if (!target) return;
      morphLivePane(target, firewallState.hosts.map(host => `<li data-scaffold-key="firewall-host:${escapeHtml(host)}"><code>${escapeHtml(host)}</code><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-remove-host="${escapeHtml(host)}">Remove</button></li>`).join('') || (firewallState.whitelistReady ? '<li data-scaffold-key="firewall-hosts-empty">No websites allowed yet.</li>' : ''));
      firewallSetControls(binding);
    }
    function firewallWrite(binding, key, route, options) {
      const slot=binding.root.querySelector('[data-scaffold-slot="firewall-children"]');if(!slot)return Promise.resolve(false);
      return scaffoldRegistry.run(binding,`firewall-write-${key}`,async ctx=>{if(!ctx.current()||!firewallCurrent(binding)||!firewallState.childrenReady||(key==='register'&&!firewallState.observedReady))return false;const receipt=await firewallMutation(route,{...options,signal:ctx.signal},binding);if(!ctx.current()||!firewallCurrent(binding))return false;ctx.ready(slot);return receipt;},{slots:[slot],retry:false,timeoutMs:15000,onFailure:({current})=>{if(current()&&firewallCurrent(binding)){firewallState.childrenReady=false;renderFirewall(binding);}}});
    }
    document.body.addEventListener('submit', event => {
      const binding = scaffoldRegistry.capture('firewall'); const form = event.target.closest?.('[data-firewall-host-form]');
      if (!binding || !form || !binding.root.contains(form)) return;
      event.preventDefault(); if (!firewallCurrent(binding) || !form.isConnected || form.querySelector('button[type="submit"]')?.disabled || !firewallState.whitelistReady) return;
      const input = form.querySelector('[data-firewall-host-input]'); const host = String(input?.value || '').trim().toLowerCase().replace(/\.$/, '');
      if (!host || firewallState.hosts.includes(host)) return;
      firewallState.hosts.push(host); if (input) input.value = ''; renderFirewallHosts(binding);
    });
    document.addEventListener('click', async event => {
      const binding = scaffoldRegistry.capture('firewall'); const pane = binding?.root;
      if (!pane || !pane.contains(event.target)) return;
      const control = selector => { const match = event.target.closest?.(selector); return match && pane.contains(match) ? match : null; };
      const modal = firewallModalElement();
      if (event.target === modal || control('[data-firewall-modal-cancel]')) return closeFirewallModal();
      if (control('[data-firewall-refresh]')) { const button = control('[data-firewall-refresh]'); if (!firewallCurrent(binding) || button.disabled) return; return hydrateFirewall({ supersede: true }); }
      const register = control('[data-firewall-register]');
      if (register) {
        const mac = String(register.dataset.firewallRegister || '').toUpperCase();
        if (!firewallCurrent(binding) || register.disabled || !firewallState.observedReady || !firewallState.childrenReady || firewallState.children.some(row => firewallMac(row) === mac)) return;
        try { const receipt = await firewallWrite(binding, 'register', '/api/firewall/children', { method: 'POST', body: JSON.stringify({ mac }) }); if (!receipt || !firewallCurrent(binding)) return; const message = receipt.message || 'Child device registered'; showCoronatioToast(receipt.pinned?.ip ? `${message} (${receipt.pinned.ip})` : message, 'success'); await hydrateFirewall(); }
        catch (failure) { if (firewallCurrent(binding)) showCoronatioToast(firewallMessage(failure), 'error'); } return;
      }
      const unregister = control('[data-firewall-unregister]');
      if (unregister) {
        const mac = String(unregister.dataset.firewallUnregister || '').toUpperCase();
        if (!firewallCurrent(binding) || unregister.disabled || !firewallState.childrenReady || !firewallState.children.some(row => firewallMac(row) === mac)) return;
        try { const receipt = await firewallWrite(binding, 'unregister', `/api/firewall/children/${encodeURIComponent(mac)}`, { method: 'DELETE' }); if (!receipt || !firewallCurrent(binding)) return; showCoronatioToast(receipt.message || 'Child device unregistered', 'success'); await hydrateFirewall(); }
        catch (failure) { if (firewallCurrent(binding)) showCoronatioToast(firewallMessage(failure), 'error'); } return;
      }
      const whitelist = control('[data-firewall-whitelist]'); if (whitelist) { if (!firewallCurrent(binding) || whitelist.disabled) return; return openFirewallModal(whitelist.dataset.firewallWhitelist); }
      const remove = control('[data-firewall-remove-host]');
      if (remove) { if (!firewallCurrent(binding) || remove.disabled || !firewallState.whitelistReady) return; firewallState.hosts = firewallState.hosts.filter(host => host !== remove.dataset.firewallRemoveHost); return renderFirewallHosts(binding); }
      const save = control('[data-firewall-modal-save]');
      if (save) {
        const mac = firewallState.selectedMac; const slot = pane.querySelector('[data-scaffold-slot="firewall-whitelist"]');
        if (!firewallCurrent(binding) || save.disabled || !firewallState.whitelistReady || !mac || !slot) return;
        const proposedHosts = firewallState.hosts.slice(); const openedRevision = firewallState.revision;
        const readResult = await scaffoldRegistry.run(binding, 'firewall-whitelist', async ctx => {
          if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac || !firewallState.childrenReady || !firewallState.children.some(row => firewallMac(row) === mac)) return false;
          const fresh = await ctx.fetchJson(`/api/firewall/children/${encodeURIComponent(mac)}/whitelist`);
          if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return false;
          const refusal = firewallRefusal(fresh); if (refusal) throw new Error(refusal);
          const freshHosts = firewallRows(fresh, 'hostnames'); const freshRevision = fresh.revision ?? fresh.payload?.revision ?? fresh.payload?.result?.revision;
          if (!freshHosts || freshHosts.some(host => typeof host !== 'string') || (typeof freshRevision !== 'string' && typeof freshRevision !== 'number')) throw new Error('Whitelist data is incomplete');
          if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return false;
          if (freshRevision !== openedRevision) {
            firewallState.hosts = freshHosts.map(host => String(host)); firewallState.revision = freshRevision; firewallState.whitelistReady = true;
            renderFirewallHosts(binding); ctx.ready(slot); const error = slot.querySelector('[data-firewall-whitelist-error]'); if (error) { error.hidden = false; error.textContent = 'Whitelist changed; current entries loaded. Review them before saving again.'; }
            return true;
          }
          if (!ctx.current() || !firewallCurrent(binding) || !firewallState.whitelistReady) return false;
          const receipt = await firewallMutation(`/api/firewall/children/${encodeURIComponent(mac)}/whitelist`, { method: 'PUT', body: JSON.stringify({ hostnames: proposedHosts, expectedRevision: freshRevision }), signal: ctx.signal }, binding);
          if (!ctx.current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return false;
          firewallState.hosts = proposedHosts; firewallState.revision = receipt.revision ?? freshRevision;
          const receiptTarget = pane.querySelector('[data-firewall-receipt]'); if (receiptTarget) receiptTarget.textContent = receipt.message || 'Website whitelist saved.';
          showCoronatioToast(receipt.message || 'Website whitelist saved', 'success'); ctx.ready(slot); closeFirewallModal(); return true;
        }, { slots: [slot], supersede: true, onFailure: ({ current, reason }) => { if (!current() || !firewallCurrent(binding) || firewallState.selectedMac !== mac) return; const error = slot.querySelector('[data-firewall-whitelist-error]'); if (error) { error.hidden = false; error.textContent = reason || 'Whitelist could not be saved'; } } });
        if (readResult === false && firewallCurrent(binding)) { const error = slot.querySelector('[data-firewall-whitelist-error]'); if (error && error.hidden) { error.hidden = false; error.textContent = 'Whitelist could not be saved'; } }
      }
    }, true);
    window.addEventListener('coronatio:privileged-state-retired', () => {
      window.scaffoldRegistry?.retire('firewall','Admin role retired');
      firewallState.observed = []; firewallState.children = []; firewallState.hosts = []; firewallState.revision = undefined; firewallState.selectedMac = ''; firewallState.observedReady = false; firewallState.childrenReady = false; firewallState.whitelistReady = false; firewallState.binding = null;
      document.querySelectorAll('[data-pane-panel="firewall"] [data-firewall-observed], [data-pane-panel="firewall"] [data-firewall-children], [data-pane-panel="firewall"] [data-firewall-hosts]').forEach(node => node.replaceChildren());
      document.querySelectorAll('[data-pane-panel="firewall"] [data-firewall-receipt]').forEach(node => { node.textContent = ''; });
      const modal = document.querySelector('[data-pane-panel="firewall"] [data-firewall-modal-backdrop]'); if (modal) { modal.classList.remove('open'); modal.setAttribute('aria-hidden', 'true'); }
      document.querySelectorAll('[data-pane-panel="firewall"] [data-firewall-register], [data-pane-panel="firewall"] [data-firewall-unregister], [data-pane-panel="firewall"] [data-firewall-whitelist], [data-pane-panel="firewall"] [data-firewall-modal-save]').forEach(node => { node.disabled = true; });
    });
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane, binding = firewallState.binding;
      if (detail.id !== 'firewall' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="firewall"]') !== pane) return;
      if (binding && (binding.guest !== detail.id || window.scaffoldRegistry?.current(binding))) return;
      firewallState.observed = []; firewallState.children = []; firewallState.hosts = []; firewallState.revision = undefined;
      firewallState.selectedMac = ''; firewallState.observedReady = false; firewallState.childrenReady = false; firewallState.whitelistReady = false; firewallState.binding = null;
    });
"####
}