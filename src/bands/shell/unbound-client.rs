fn shell_unbound_client() -> &'static str {
    r####"    const dnsState = { records: [] };
    function dnsSet(selector, value) { const node = document.querySelector(selector); if (node) node.textContent = value; }
    async function dnsJson(route, options = {}) {
      const response = await fetch(route, { cache: 'no-store', headers: options.body ? { 'content-type': 'application/json' } : {}, ...options });
      const body = await response.json().catch(() => ({}));
      if (!response.ok || body.ok === false) throw new Error(body.firstMissingSignal || body.error || `DNS request failed (${response.status})`);
      return body;
    }
    function dnsRecords(payload) { return payload.records || payload.data?.records || payload.result?.records || []; }
    function renderDns() {
      const target = document.querySelector('[data-dns-records]'); if (!target) return;
      target.replaceChildren();
      for (const record of dnsState.records) {
        const row = document.createElement('div'); row.className = 'dns-record';
        const name = document.createElement('strong'); name.textContent = String(record.name || '');
        const address = document.createElement('span'); address.textContent = String(record.address || '');
        const remove = document.createElement('button'); remove.type = 'button'; remove.className = 'ui-button ui-button--danger ui-button--small'; remove.dataset.dnsRemove = String(record.name || ''); remove.textContent = 'Remove';
        row.append(name, address, remove); target.appendChild(row);
      }
      const empty = document.querySelector('[data-dns-empty]'); if (empty) empty.hidden = dnsState.records.length !== 0;
    }
    async function hydrateDns() {
      const pane = document.querySelector('[data-dns-tablet]'); if (!pane || !viewportFamilyAdmitted('unbound') || document.visibilityState !== 'visible') return;
      dnsSet('[data-dns-state]', 'Loading local records…');
      try { dnsState.records = dnsRecords(await dnsJson('/api/dns/records/status', { method: 'POST', body: JSON.stringify({}) })); renderDns(); dnsSet('[data-dns-state]', `${dnsState.records.length} local record(s)`); }
      catch (failure) { dnsSet('[data-dns-state]', `DNS unavailable: ${failure.message}`); showCoronatioToast(failure.message, 'error'); }
    }
    document.body.addEventListener('submit', async event => {
      const form = event.target.closest?.('[data-dns-form]'); if (!form) return;
      event.preventDefault(); const data = new FormData(form); const name = String(data.get('name') || '').trim(); const address = String(data.get('address') || '').trim();
      if (!/^[a-z0-9-]+(\.[a-z0-9-]+)*\.home\.arpa$/i.test(name) || !/^\d{1,3}(\.\d{1,3}){3}$/.test(address)) { dnsSet('[data-dns-state]', 'Enter a home.arpa hostname and private IPv4 address.'); return; }
      try { const receipt = await dnsJson('/api/dns/records', { method: 'POST', body: JSON.stringify({ name, address }) }); dnsSet('[data-dns-state]', receipt.changed === false ? 'Record already current.' : 'Record saved.'); form.reset(); await hydrateDns(); showCoronatioToast('Local DNS record saved.', 'success'); }
      catch (failure) { dnsSet('[data-dns-state]', `DNS refused: ${failure.message}`); showCoronatioToast(failure.message, 'error'); }
    });
    document.body.addEventListener('click', async event => {
      if (event.target.closest?.('[data-dns-refresh]')) return hydrateDns();
      const remove = event.target.closest?.('[data-dns-remove]'); if (!remove) return;
      const name = remove.dataset.dnsRemove; if (!name || !window.confirm(`Remove ${name}?`)) return;
      try { await dnsJson('/api/dns/records/' + encodeURIComponent(name), { method: 'DELETE' }); await hydrateDns(); showCoronatioToast('Local DNS record removed.', 'success'); }
      catch (failure) { dnsSet('[data-dns-state]', `DNS refused: ${failure.message}`); showCoronatioToast(failure.message, 'error'); }
    });
"####
}
