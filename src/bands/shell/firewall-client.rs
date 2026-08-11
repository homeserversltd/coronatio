fn shell_firewall_client() -> &'static str {
    r####"    const firewallState = { observed: [], children: [], selectedMac: '', hosts: [] };
    function firewallRows(payload, key) { if (Array.isArray(payload)) return payload; for (const source of [payload?.[key], payload?.devices, payload?.device?.whitelist, payload?.receipt?.[key], payload?.receipt?.devices, payload?.receipt?.device?.whitelist, payload?.data?.[key], payload?.result?.[key], payload?.items]) if (Array.isArray(source)) return source; return []; }
    function firewallMac(row) { return String(row?.mac || row?.macAddress || row?.['hw-address'] || row?.hwAddress || row || '').toUpperCase(); }
    function firewallMessage(payload) { return payload?.message || payload?.firstMissingSignal || payload?.error || 'Child-device controls are unavailable'; }
    async function firewallJson(route, options = {}) { const response = await fetch(route, { cache: 'no-store', headers: options.body ? { 'content-type': 'application/json' } : {}, ...options }); const body = await response.json().catch(() => ({})); if (!response.ok || body?.ok === false) throw body; return body; }
    function renderFirewall() {
      const observed = document.querySelector('[data-firewall-observed]'); const children = document.querySelector('[data-firewall-children]');
      if (observed) observed.innerHTML = firewallState.observed.map(row => { const mac = firewallMac(row); const registered = firewallState.children.some(child => firewallMac(child) === mac); return `<li><code>${escapeHtml(mac)}</code><button type="button" class="ui-button ui-button--primary ui-button--small" data-firewall-register="${escapeHtml(mac)}" ${registered ? 'disabled' : ''}>${registered ? 'Registered' : 'Register child device'}</button></li>`; }).join('') || '<li>No observed MAC addresses are available.</li>';
      if (children) children.innerHTML = firewallState.children.map(row => { const mac = firewallMac(row); return `<li><code>${escapeHtml(mac)}</code><div><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-whitelist="${escapeHtml(mac)}">Website whitelist</button><button type="button" class="ui-button ui-button--danger ui-button--small" data-firewall-unregister="${escapeHtml(mac)}">Unregister</button></div></li>`; }).join('') || '<li>No child devices are registered.</li>';
    }
    async function hydrateFirewall() {
      const tablet = document.querySelector('[data-firewall-tablet]'); if (!tablet || !viewportFamilyAdmitted('firewall')) return;
      const error = document.querySelector('[data-firewall-error]'); if (error) error.hidden = true;
      try { const [observed, children] = await Promise.all([firewallJson('/api/firewall/observed'), firewallJson('/api/firewall/children')]); firewallState.observed = firewallRows(observed, 'observed'); firewallState.children = firewallRows(children, 'children'); renderFirewall(); }
      catch (failure) { if (error) { error.hidden = false; error.textContent = firewallMessage(failure); } }
    }
    const firewallModal = document.querySelector('[data-firewall-modal-backdrop]');
    function closeFirewallModal() { if (firewallModal) { firewallModal.classList.remove('open'); firewallModal.setAttribute('aria-hidden', 'true'); } }
    async function openFirewallModal(mac) {
      try { const payload = await firewallJson(`/api/firewall/children/${encodeURIComponent(mac)}/whitelist`); firewallState.selectedMac = mac; firewallState.hosts = firewallRows(payload, 'hostnames'); document.querySelector('[data-firewall-modal-title]').textContent = `Website whitelist — ${mac}`; renderFirewallHosts(); firewallModal.classList.add('open'); firewallModal.setAttribute('aria-hidden', 'false'); document.querySelector('[data-firewall-host-input]')?.focus(); }
      catch (failure) { showCoronatioToast(firewallMessage(failure), 'error'); }
    }
    function renderFirewallHosts() { const target = document.querySelector('[data-firewall-hosts]'); if (target) target.innerHTML = firewallState.hosts.map(host => `<li><code>${escapeHtml(host)}</code><button type="button" class="ui-button ui-button--secondary ui-button--small" data-firewall-remove-host="${escapeHtml(host)}">Remove</button></li>`).join('') || '<li>No websites allowed yet.</li>'; }
    document.body.addEventListener('submit', async event => { const form = event.target.closest?.('[data-firewall-host-form]'); if (!form) return; event.preventDefault(); const input = form.querySelector('[data-firewall-host-input]'); const host = String(input?.value || '').trim().toLowerCase().replace(/\.$/, ''); if (!host || firewallState.hosts.includes(host)) return; firewallState.hosts.push(host); input.value = ''; renderFirewallHosts(); });
    document.addEventListener('click', async event => {
      if (event.target === firewallModal || event.target.closest?.('[data-firewall-modal-cancel]')) return closeFirewallModal();
      if (event.target.closest?.('[data-firewall-refresh]')) return hydrateFirewall();
      const register = event.target.closest?.('[data-firewall-register]'); if (register) { try { const receipt = await firewallJson('/api/firewall/children', { method: 'POST', body: JSON.stringify({ mac: register.dataset.firewallRegister }) }); showCoronatioToast(receipt.message || 'Child device registered', 'success'); await hydrateFirewall(); } catch (failure) { showCoronatioToast(firewallMessage(failure), 'error'); } return; }
      const unregister = event.target.closest?.('[data-firewall-unregister]'); if (unregister) { try { const receipt = await firewallJson(`/api/firewall/children/${encodeURIComponent(unregister.dataset.firewallUnregister)}`, { method: 'DELETE' }); showCoronatioToast(receipt.message || 'Child device unregistered', 'success'); await hydrateFirewall(); } catch (failure) { showCoronatioToast(firewallMessage(failure), 'error'); } return; }
      const whitelist = event.target.closest?.('[data-firewall-whitelist]'); if (whitelist) return openFirewallModal(whitelist.dataset.firewallWhitelist);
      const remove = event.target.closest?.('[data-firewall-remove-host]'); if (remove) { firewallState.hosts = firewallState.hosts.filter(host => host !== remove.dataset.firewallRemoveHost); return renderFirewallHosts(); }
      if (event.target.closest?.('[data-firewall-modal-save]')) { try { const receipt = await firewallJson(`/api/firewall/children/${encodeURIComponent(firewallState.selectedMac)}/whitelist`, { method: 'PUT', body: JSON.stringify({ hostnames: firewallState.hosts }) }); document.querySelector('[data-firewall-receipt]').textContent = JSON.stringify(receipt, null, 2); showCoronatioToast(receipt.message || 'Website whitelist saved', 'success'); closeFirewallModal(); } catch (failure) { showCoronatioToast(firewallMessage(failure), 'error'); } }
    }, true);
"####
}
