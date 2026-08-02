fn shell_firewall_client() -> &'static str {
    r####"    const firewallState = { devices: [], policies: [], status: {}, selectedMac: '', lastReceipt: null };
    function canonicalFirewallMac(raw) {
      const value = String(raw || '').trim(); const match = value.match(/^([0-9a-f]{2})([:-])([0-9a-f]{2})\2([0-9a-f]{2})\2([0-9a-f]{2})\2([0-9a-f]{2})\2([0-9a-f]{2})$/i);
      return match ? [match[1], match[3], match[4], match[5], match[6], match[7]].join(':').toUpperCase() : '';
    }
    function firewallRows(payload, key) { if (Array.isArray(payload)) return payload; for (const source of [payload?.[key], payload?.data?.[key], payload?.result?.[key], payload?.items]) if (Array.isArray(source)) return source; return []; }
    function firewallField(row, ...names) { for (const name of names) if (row?.[name] !== undefined && row?.[name] !== null) return row[name]; return ''; }
    function firewallRevision(value) { const revision = typeof value === 'string' ? value : ''; return /^[0-9a-f]{64}$/i.test(revision) ? revision.toLowerCase() : ''; }
    function firewallExpectedRevision(policy, status) { return firewallRevision(firewallField(policy, 'expectedRevision', 'revision')) || firewallRevision(firewallField(status, 'expectedRevision', 'revision')) || firewallRevision(firewallField(status?.readback, 'expectedRevision', 'revision')); }
    function firewallEnforced(status, policy) {
      const mac = canonicalFirewallMac(firewallField(policy, 'mac', 'macAddress'));
      const receipt = policy?.receipt || status?.receipt || status?.readback || {};
      const receiptMac = canonicalFirewallMac(firewallField(receipt, 'mac', 'macAddress', 'policyMac', 'policy_mac'));
      const missing = receipt.firstMissingSignal ?? policy?.firstMissingSignal ?? status?.firstMissingSignal;
      return Boolean(policy && policy.enabled === true && mac && receiptMac === mac) && !missing && receipt.bindingVerified === true && receipt.nft?.applied === true && receipt.nft?.liveReadback === true && receipt.dns?.required === true && receipt.dns?.validated === true && receipt.dns?.applied === true;
    }
    function firewallPolicy(mac) { return firewallState.policies.find(policy => canonicalFirewallMac(firewallField(policy, 'mac', 'macAddress')) === mac); }
    function firewallSet(selector, text) { const node = document.querySelector(selector); if (node) node.textContent = text; }
    function firewallMessage(payload) { return payload?.firstMissingSignal || payload?.error || payload?.message || 'Caduceus did not confirm this request'; }
    async function firewallJson(route, options = {}) { const response = await fetch(route, { cache: 'no-store', headers: options.body ? { 'content-type': 'application/json' } : {}, ...options }); const body = await response.json().catch(() => ({})); if (!response.ok || body?.ok === false || body?.success === false || body?.accepted === false || body?.firstMissingSignal && body.firstMissingSignal !== 'none') throw body; return body; }
    function renderFirewall() {
      const select = document.querySelector('[data-firewall-device]'); if (!select) return;
      const selected = firewallState.selectedMac || canonicalFirewallMac(select.value) || canonicalFirewallMac(firewallField(firewallState.devices[0], 'mac', 'macAddress', 'hwAddress', 'hw-address'));
      firewallState.selectedMac = selected;
      select.innerHTML = firewallState.devices.map(device => { const mac = canonicalFirewallMac(firewallField(device, 'mac', 'macAddress', 'hwAddress', 'hw-address')); const name = firewallField(device, 'hostname', 'hostName', 'name') || 'Unnamed device'; return `<option value="${escapeHtml(mac)}" ${mac === selected ? 'selected' : ''}>${escapeHtml(name)} — ${escapeHtml(mac)}</option>`; }).join('');
      const policy = firewallPolicy(selected); const status = firewallState.status || {}; const enabled = Boolean(policy?.enabled); const enforced = firewallEnforced(status, policy);
      const availability = status?.stableBinding?.available ?? status?.stableBindingAvailable ?? false;
      firewallSet('[data-firewall-binding]', availability ? 'Stable binding available' : `Stable binding refused: ${firewallMessage(status)}`);
      firewallSet('[data-firewall-state]', enforced ? 'Enforced' : (policy ? 'Awaiting authoritative readback' : 'No policy'));
      const badge = document.querySelector('[data-firewall-state]'); if (badge) badge.className = `ui-badge ui-badge--small ${enforced ? 'ui-badge--success' : 'ui-badge--info'}`;
      const toggle = document.querySelector('[data-firewall-enabled]'); if (toggle) toggle.checked = enabled;
      const sites = firewallRows(policy || {}, 'sites'); const list = document.querySelector('[data-firewall-sites]'); if (list) list.innerHTML = sites.map(site => `<li><code>${escapeHtml(String(site))}</code><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-remove-site="${escapeHtml(String(site))}">Remove</button></li>`).join('') || '<li>No websites allowed yet.</li>';
      const revision = firewallExpectedRevision(policy, status); const editor = document.querySelector('[data-firewall-editor]'); if (editor) editor.dataset.firewallRevision = revision;
    }
    async function hydrateFirewall() {
      const tablet = document.querySelector('[data-firewall-tablet]'); if (!tablet || !viewportFamilyAdmitted('firewall')) return;
      const error = document.querySelector('[data-firewall-error]'); if (error) error.hidden = true;
      try {
        const [leases, policies, status] = await Promise.all([firewallJson('/api/dhcp/leases'), firewallJson('/api/firewall/policies'), firewallJson('/api/firewall/status')]);
        firewallState.devices = firewallRows(leases, 'leases'); firewallState.policies = firewallRows(policies, 'policies'); firewallState.status = status;
        renderFirewall();
      } catch (failure) { if (error) { error.hidden = false; error.textContent = `DNS website policy unavailable: ${firewallMessage(failure)}`; } }
    }
    async function saveFirewall(remove = false) {
      const mac = canonicalFirewallMac(firewallState.selectedMac); const policy = firewallPolicy(mac) || {}; const revision = firewallExpectedRevision(policy, firewallState.status || {}); const sites = [...document.querySelectorAll('[data-firewall-sites] code')].map(node => node.textContent);
      if (!mac || !revision) return showCoronatioToast(!mac ? 'Select a device first' : 'Policy revision unavailable; refresh first', 'error');
      const payload = { schema: 'caduceus.network.firewall.policy.v1', mac, mode: 'allow-only', sites, expectedRevision: revision, enabled: Boolean(document.querySelector('[data-firewall-enabled]')?.checked), enforcement: 'dns-policy' };
      const removePayload = { schema: 'caduceus.network.firewall.policy.delete.v1', mac, expectedRevision: revision };
      try { firewallState.lastReceipt = await firewallJson(`/api/firewall/policies/${encodeURIComponent(mac)}`, remove ? { method: 'DELETE', body: JSON.stringify(removePayload) } : { method: 'PUT', body: JSON.stringify(payload) }); await hydrateFirewall(); const changed = firewallState.lastReceipt?.changed === true; showCoronatioToast(changed ? (remove ? 'Policy removed' : 'Policy changed') : 'No policy change', 'success'); }
      catch (failure) { showCoronatioToast(firewallMessage(failure), 'error'); }
    }
    document.body.addEventListener('change', event => { const device = event.target.closest?.('[data-firewall-device]'); if (device) { firewallState.selectedMac = canonicalFirewallMac(device.value); renderFirewall(); } });
    document.body.addEventListener('submit', event => { const form = event.target.closest?.('[data-firewall-add-site-form]'); if (!form) return; event.preventDefault(); const input = form.querySelector('[data-firewall-site-input]'); const site = String(input?.value || '').trim().toLowerCase().replace(/\.$/, ''); if (!/^(?=.{1,253}$)(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,63}$/.test(site)) return showCoronatioToast('Enter a valid website hostname', 'error'); const policy = firewallPolicy(firewallState.selectedMac) || (firewallState.policies[firewallState.policies.push({ mac: firewallState.selectedMac, sites: [], enabled: false }) - 1]); if (!policy.sites.includes(site) && policy.sites.length < 64) policy.sites.push(site); input.value = ''; renderFirewall(); });
    document.addEventListener('click', event => { if (event.target.closest?.('[data-firewall-refresh]')) return hydrateFirewall(); if (event.target.closest?.('[data-firewall-save]')) return saveFirewall(); if (event.target.closest?.('[data-firewall-delete]')) return saveFirewall(true); const remove = event.target.closest?.('[data-firewall-remove-site]'); if (remove) { const policy = firewallPolicy(firewallState.selectedMac); if (policy) policy.sites = policy.sites.filter(site => site !== remove.dataset.firewallRemoveSite); renderFirewall(); } }, true);
"####
}
