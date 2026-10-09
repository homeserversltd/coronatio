fn shell_unbound_client() -> &'static str {
    r####"    const dnsUiState = { resolver: null, resolverReady: false, identityReady: false, pickedMac: null, selectedOctet: null, calendarStart: null, calendarEnd: null, calendarPage: 0, nameDefault: '', mutationBusy: false };
    const dnsArray = value => Array.isArray(value) ? value : [];
    const dnsValue = (value, fallback = '—') => value === undefined || value === null || value === '' ? fallback : String(value);
    const dnsAddress = row => row?.address || row?.ip || row?.target || row?.value || row?.declared_reservation?.ip || row?.declared_reservation?.['ip-address'] || row?.observed_lease?.ip || '—';
    const dnsProviders = Object.freeze({ quad9: ['9.9.9.9', '149.112.112.112'], cloudflare: ['1.1.1.1', '1.0.0.1'], google: ['8.8.8.8', '8.8.4.4'] });
    function dnsEnvelopeNodes(value, depth = 0) { if (!value || typeof value !== 'object' || depth > 6) return []; if (Array.isArray(value)) return value.flatMap(item => dnsEnvelopeNodes(item, depth + 1)); const nodes = [value]; for (const key of ['payload', 'result', 'data', 'receipt', 'readback', 'caduceus']) nodes.push(...dnsEnvelopeNodes(value[key], depth + 1)); return nodes; }
    function dnsFindObject(value, predicate) { return dnsEnvelopeNodes(value).find(node => predicate(node)) || null; }
    function dnsRefusal(value) { for (const node of dnsEnvelopeNodes(value)) { if (node.ok === false || node.success === false) return node.firstMissingSignal || node.first_failing_boundary || node.first_missing_signal || node.error || 'dns-caduceus-refused'; } return null; }
    function dnsRequireAccepted(value) { const refusal = dnsRefusal(value); if (refusal) throw new Error(refusal); if (!dnsFindObject(value, node => node.ok === true || node.success === true || node.accepted === true)) throw new Error('dns-mutation-receipt-missing-success'); return value; }
    async function dnsJson(route, options = {}) { const response = await fetch(route, { cache: 'no-store', headers: options.body ? {'content-type':'application/json'} : {}, ...options }); const body = await response.json().catch(() => null); const refusal = dnsRefusal(body); if (!response.ok || refusal) throw new Error(refusal || body?.first_failing_boundary || body?.firstMissingSignal || body?.error || `Request failed (${response.status})`); return body; }
    function dnsSetState(message) { document.querySelectorAll('[data-dns-state]').forEach(node => { node.textContent = message; }); }
    function dnsSetResolverControls() { const status = dnsUiState.resolver; document.querySelectorAll('[data-dns-resolver-control], [data-dns-upstream-save]').forEach(node => { let available = dnsUiState.resolverReady; if (node.matches('[data-dns-adblock]')) available = available && typeof status?.adblockEnabled === 'boolean'; else if (node.matches('[data-dns-dot], [data-dns-upstream-provider], [data-dns-upstream-custom-toggle], [data-dns-upstream-custom-addresses], [data-dns-upstream-save]')) available = available && Array.isArray(status?.upstreams) && typeof status?.dnsOverTls === 'boolean'; node.disabled = !available || dnsUiState.mutationBusy; }); }
    function dnsSetIdentityControls() { document.querySelectorAll('[data-dns-add-name]').forEach(node => { node.disabled = !dnsUiState.identityReady || dnsUiState.mutationBusy; }); document.querySelectorAll('[data-dns-device-pick]').forEach(node => { node.disabled = !dnsUiState.identityReady || dnsUiState.mutationBusy; }); document.querySelectorAll('[data-dns-name-remove]').forEach(node => { node.disabled = !dnsUiState.identityReady || dnsUiState.mutationBusy; }); dnsUpdateAddSave(); }
    function dnsSetMutationBusy(busy) { dnsUiState.mutationBusy = Boolean(busy); dnsSetResolverControls(); dnsSetIdentityControls(); renderDnsNames(); renderDnsModalRoster(); renderDnsModalRecords(); if (dnsUiState.pickedMac) renderDnsIpCalendar(); }
    function dnsProvenanceHas(value, wanted) { const values = Array.isArray(value) ? value : [value]; return values.some(item => String(item || '').toLowerCase() === wanted); }
    function dnsOctet(value) { const match = String(value || '').match(/^192\.168\.123\.(\d{1,3})$/); if (!match) return null; const octet = Number(match[1]); return Number.isInteger(octet) && octet >= 1 && octet <= 254 ? octet : null; }
    function dnsReversePtr(address) { const parts = String(address || '').split('.'); return parts.length === 4 && parts.every(part => /^\d{1,3}$/.test(part)) ? `${parts.reverse().join('.')}.in-addr.arpa` : ''; }
    function dnsReadRoot(value) { return dnsFindObject(value, node => Array.isArray(node.devices)) || {}; }
    function dnsNameRows() {
      const rows = new Map();
      const add = (row, source) => {
        const name = String(row?.name || row?.hostname || row?.alias || '').trim();
        if (!name) return;
        const address = String(row?.address || row?.ip || row?.target || row?.cname || '—');
        const key = `${name.toLowerCase()}\u0000${address.toLowerCase()}`;
        const existing = rows.get(key);
        if (existing) { if (!existing.mac && row.mac) existing.mac = row.mac; if (!existing.ptr && row.ptr) existing.ptr = row.ptr; if (source === 'dns-owned-pair' && row.removable === true) { existing.provenance = row.provenance; existing.removable = true; if (row.ptr) existing.ptr = row.ptr; if (row.mac) existing.mac = row.mac; } return; }
        const provenance = row?.provenance ?? (source === 'alias' ? 'alias (read-only)' : source === 'roster' ? 'device projection' : 'observed');
        rows.set(key, { name, address, ptr: row?.ptr || '', mac: row?.mac || '', provenance, removable: source === 'dns-owned-pair' && row?.removable === true });
      };
      const root = dnsReadRoot(identityState?.dns || {});
      dnsArray(root.devices).forEach(device => {
        const name = device?.name || device?.hostname || device?.alias || dnsArray(device?.dns_names)[0];
        const rawRecords = dnsArray(device?.a_records);
        const records = rawRecords.length ? rawRecords : dnsArray(device?.a).map(address => ({ address }));
        records.forEach(record => {
          const address = record?.address || record?.ip || (typeof record === 'string' ? record : '');
          if (!address) return;
          const expectedPtr = dnsReversePtr(address).toLowerCase();
          const ptrRecord = dnsArray(device?.ptr_records).find(item => String(item?.name || '').toLowerCase().replace(/\.$/, '') === expectedPtr);
          const aOwned = dnsProvenanceHas(record?.provenance, 'owned');
          const ptrOwned = Boolean(ptrRecord) && dnsProvenanceHas(ptrRecord?.provenance, 'owned');
          const provenance = record?.provenance ?? 'observed';
          add({ name, address, ptr: ptrRecord?.name || '', provenance: aOwned && ptrOwned ? 'owned A/PTR pair' : provenance, removable: aOwned && ptrOwned && Boolean(dnsNormalizeBareLabel(name).label) }, aOwned && ptrOwned ? 'dns-owned-pair' : 'dns-device');
        });
      });
      dnsEnvelopeNodes(identityState?.dns || {}).forEach(node => {
        for (const key of ['records', 'names']) dnsArray(node?.[key]).forEach(row => add(row, 'dns-record'));
        for (const key of ['aliases', 'cnames']) dnsArray(node?.[key]).forEach(row => add({ ...row, address: row?.target || row?.cname || row?.address, provenance: row?.provenance || 'alias (read-only)' }, 'alias'));
      });
      const dnsNameKey = value => String(value ?? '').trim().replace(/\.$/, '').toLowerCase();
      const dnsNames = new Set([...rows.values()].map(row => dnsNameKey(row.name)).filter(Boolean));
      dnsArray(identityState?.roster).forEach(device => dnsArray(device?.dns_names).forEach(name => {
        const address = dnsAddress(device);
        if (address === '—' && dnsNames.has(dnsNameKey(name))) return;
        add({ name, address, mac: device?.mac || '' }, 'roster');
      }));
      return [...rows.values()].sort((a, b) => a.name.localeCompare(b.name) || a.address.localeCompare(b.address));
    }
    function renderDnsNames() { const target = document.querySelector('[data-dns-names]'); if (!target) return; const rows = dnsNameRows(); target.innerHTML = rows.length ? `<table class="ui-table dns-name-table"><thead><tr><th>Name</th><th>Address</th><th>PTR</th><th>Provenance</th><th>Action</th></tr></thead><tbody>${rows.map(row => `<tr><td><code>${escapeHtml(dnsValue(row.name))}</code></td><td><code>${escapeHtml(dnsValue(row.address))}</code></td><td><code>${escapeHtml(dnsValue(row.ptr))}</code></td><td>${escapeHtml(Array.isArray(row.provenance) ? row.provenance.join(', ') : dnsValue(row.provenance))}</td><td>${row.removable ? `<button type="button" class="ui-button ui-button--danger ui-button--small" data-dns-name-remove="${escapeHtml(row.name)}" data-dns-ip="${escapeHtml(row.address)}" aria-label="Remove owned A and PTR records for ${escapeHtml(row.name)}"${!dnsUiState.identityReady || dnsUiState.mutationBusy ? ' disabled' : ''}>Remove</button>` : '—'}</td></tr>`).join('')}</tbody></table>` : '<p>No LAN names are registered.</p>'; }
    function dnsDeviceLabel(device) { return String(device?.hostname || device?.name || device?.declared_reservation?.hostname || device?.observed_lease?.hostname || dnsArray(device?.dns_names)[0] || device?.mac || 'Unknown device'); }
    function dnsRosterDevice(mac = dnsUiState.pickedMac) { return dnsArray(identityState?.roster).find(device => String(device?.mac || '') === String(mac || '')) || null; }
    function renderDnsModalRoster() { const target = document.querySelector('[data-dns-device-roster]'); if (!target) return; const devices = dnsArray(identityState?.roster); target.innerHTML = `<table class="identity-roster"><thead><tr><th>Device</th><th>Address</th><th>Names</th><th>Pick</th></tr></thead><tbody>${devices.map(device => { const mac = String(device?.mac || ''); const selected = mac === dnsUiState.pickedMac; return `<tr data-device-mac="${escapeHtml(mac)}"><td><code>${escapeHtml(dnsValue(dnsDeviceLabel(device)))}</code><br><small>${escapeHtml(dnsValue(mac))}</small></td><td><code>${escapeHtml(dnsValue(dnsAddress(device)))}</code></td><td>${escapeHtml(dnsArray(device?.dns_names).join(', ') || '—')}</td><td><button type="button" class="ui-button ui-button--secondary ui-button--small" data-dns-device-pick="${escapeHtml(mac)}" aria-pressed="${selected}"${!dnsUiState.identityReady || dnsUiState.mutationBusy ? ' disabled' : ''}>${selected ? 'Selected' : 'Pick'}</button></td></tr>`; }).join('') || '<tr><td colspan="4">No devices reported.</td></tr>'}</tbody></table>`; }
    function renderDnsModalRecords() { const target = document.querySelector('[data-dns-modal-records]'); if (!target) return; const rows = dnsNameRows(); target.innerHTML = `<h3>Records</h3><table class="ui-table"><thead><tr><th>Name</th><th>Address</th><th>PTR</th><th>Provenance</th></tr></thead><tbody>${rows.map(row => `<tr><td><code>${escapeHtml(dnsValue(row.name))}</code></td><td><code>${escapeHtml(dnsValue(row.address))}</code></td><td><code>${escapeHtml(dnsValue(row.ptr))}</code></td><td>${escapeHtml(Array.isArray(row.provenance) ? row.provenance.join(', ') : dnsValue(row.provenance))}</td></tr>`).join('') || '<tr><td colspan="4">No records reported.</td></tr>'}</tbody></table>`; }
    function resolverReceipt(envelope) { return dnsFindObject(envelope, node => ['adblockEnabled', 'upstreams', 'dnsOverTls', 'blocklistDomainCount', 'blocklistLastUpdate', 'unboundActive'].some(key => Object.prototype.hasOwnProperty.call(node, key))) || {}; }
    function dnsUpstreamBase(address) { return String(address ?? '').trim().split(/[@#]/, 1)[0].trim(); }
    function dnsNormalizeUpstreams(addresses) { return [...new Set(dnsArray(addresses).map(dnsUpstreamBase).filter(Boolean))]; }
    function dnsSelectedUpstreamAddresses() { const selected = [...document.querySelectorAll('[data-dns-upstream-provider]:checked')].flatMap(input => dnsProviders[input.value] || []); const customToggle = document.querySelector('[data-dns-upstream-custom-toggle]'); const custom = customToggle?.checked ? (document.querySelector('[data-dns-upstream-custom-addresses]')?.value || '').split(',').map(dnsUpstreamBase).filter(Boolean) : []; return [...new Set([...selected, ...custom])]; }
    function renderDnsUpstreamReadout() { const target = document.querySelector('[data-dns-upstream-readout]'); if (!target) return; const dot = Boolean(document.querySelector('[data-dns-dot]')?.checked); const addresses = dnsSelectedUpstreamAddresses().map(address => dot ? `${address}@853` : address); target.textContent = `Upstreams: [${addresses.join(', ')}] · DoT: ${dot ? 'on' : 'off'}`; }
    function renderResolverStatus(status) {
      const target = document.querySelector('[data-dns-resolver-status]');
      const adblock = document.querySelector('[data-dns-adblock]');
      const dotInput = document.querySelector('[data-dns-dot]');
      const customToggle = document.querySelector('[data-dns-upstream-custom-toggle]');
      const customLabel = document.querySelector('[data-dns-upstream-custom]');
      const customInput = document.querySelector('[data-dns-upstream-custom-addresses]');
      const readout = document.querySelector('[data-dns-upstream-readout]');
      const providerInputs = [...document.querySelectorAll('[data-dns-upstream-provider]')];
      if (!status) {
        if (adblock) adblock.checked = false;
        if (dotInput) dotInput.checked = false;
        providerInputs.forEach(input => { input.checked = false; });
        if (customToggle) customToggle.checked = false;
        if (customLabel) customLabel.hidden = true;
        if (customInput) customInput.value = '';
        if (readout) readout.textContent = 'Resolver status unavailable.';
        if (target) target.innerHTML = '<p>Resolver status unavailable.</p>';
        return;
      }
      const hasUpstreams = Array.isArray(status.upstreams);
      const hasDot = typeof status.dnsOverTls === 'boolean';
      const upstreams = hasUpstreams ? status.upstreams.map(String) : [];
      const normalized = dnsNormalizeUpstreams(upstreams);
      if (adblock) adblock.checked = typeof status.adblockEnabled === 'boolean' ? status.adblockEnabled : false;
      if (dotInput) dotInput.checked = hasDot ? status.dnsOverTls : false;
      const selectedProviders = [];
      providerInputs.forEach(input => {
        const addresses = dnsProviders[input.value] || [];
        const selected = hasUpstreams && addresses.length > 0 && addresses.every(address => normalized.includes(address));
        input.checked = selected;
        if (selected) selectedProviders.push(input.value);
      });
      const known = new Set(selectedProviders.flatMap(name => dnsProviders[name] || []));
      const custom = hasUpstreams ? normalized.filter(address => !known.has(address)) : [];
      if (customToggle) customToggle.checked = hasUpstreams && custom.length > 0;
      if (customLabel) customLabel.hidden = !hasUpstreams || custom.length === 0;
      if (customInput) customInput.value = custom.join(', ');
      if (readout) { if (hasUpstreams && hasDot) renderDnsUpstreamReadout(); else readout.textContent = `Upstreams: ${hasUpstreams ? normalized.join(', ') || 'None' : 'unavailable'} · DoT: ${hasDot ? (status.dnsOverTls ? 'on' : 'off') : 'unavailable'}`; }
      const updated = status.blocklistLastUpdate === null ? 'Never' : Number.isFinite(status.blocklistLastUpdate) ? new Date(status.blocklistLastUpdate * 1000).toLocaleString() : 'Unavailable';
      const count = Number.isFinite(status.blocklistDomainCount) ? status.blocklistDomainCount.toLocaleString() : 'Unavailable';
      const active = typeof status.unboundActive === 'boolean' ? (status.unboundActive ? 'Active' : 'Inactive') : 'Unavailable';
      if (target) target.innerHTML = [['Upstreams', hasUpstreams ? upstreams.join(', ') || 'None' : 'Unavailable'], ['Blocklist domains', count], ['Last update', updated], ['Unbound service', active]].map(([label, value]) => `<div class="dns-info-row"><span>${escapeHtml(label)}</span><strong>${escapeHtml(dnsValue(value))}</strong></div>`).join('');
    }
    function dnsCalendarRange() { const boundary = identityState?.boundary?.boundary || identityState?.boundary || {}; const start = dnsOctet(boundary.start || boundary.first || boundary.pool_start || boundary.reservation_start); const end = dnsOctet(boundary.end || boundary.last || boundary.pool_end || boundary.reservation_end); return start !== null && end !== null && start <= end ? { start, end } : null; }
    function dnsHolderMac(row) { return String(row?.mac || row?.macAddress || row?.['hw-address'] || row?.hwAddress || row?.device_mac || row?.clientId || row?.client_id || '').toLowerCase(); }
    function dnsNormalizeHolderLabel(value) { return String(value).toLowerCase().replace(/\.$/, '').replace(/\.home\.arpa$/, ''); }
    function dnsHolderLabels(row) { return [row?.hostname, row?.hostName, row?.device_name, row?.name, row?.client_name].filter(Boolean).map(dnsNormalizeHolderLabel); }
    function dnsDeviceLabels(device) { return [device?.hostname, device?.name, device?.declared_reservation?.hostname, device?.observed_lease?.hostname, ...dnsArray(device?.dns_names)].filter(Boolean).map(dnsNormalizeHolderLabel); }
    function dnsHolderBelongsToDevice(row, device, address) { const heldMac = dnsHolderMac(row); const deviceMac = String(device?.mac || '').toLowerCase(); if (heldMac && deviceMac) return heldMac === deviceMac; const holderLabels = dnsHolderLabels(row); const deviceLabels = dnsDeviceLabels(device); if (holderLabels.length && deviceLabels.some(label => holderLabels.includes(label))) return true; return !heldMac && !holderLabels.length && dnsAddress(device) === address; }
    function dnsAddressHolders(address) { return [...dnsArray(identityState?.leases), ...dnsArray(identityState?.reservations)].filter(row => String(row?.ip || row?.['ip-address'] || row?.ipAddress || row?.address || '') === address); }
    function dnsIpAvailability(address, device) { const holders = dnsAddressHolders(address); const foreign = holders.filter(row => !dnsHolderBelongsToDevice(row, device, address)); const names = [...new Set(holders.map(row => String(row?.hostname || row?.hostName || row?.device_name || row?.name || row?.mac || row?.device_mac || 'DHCP holder')))]; return { available: foreign.length === 0, heldByPicked: holders.length > 0 && foreign.length === 0, holder: names.join(', ') }; }
    function dnsSelectedIp() { const device = dnsRosterDevice(); const octet = dnsUiState.selectedOctet; const range = dnsCalendarRange(); if (!device || !range || !Number.isInteger(octet) || octet < range.start || octet > range.end) return ''; const address = `192.168.123.${octet}`; return dnsIpAvailability(address, device).available ? address : ''; }
    function dnsUpdateAddSave() { const save = document.querySelector('[data-dns-add-save]'); if (!save) return; const name = document.querySelector('[data-dns-new-name]')?.value.trim(); save.disabled = !dnsUiState.identityReady || !dnsUiState.pickedMac || !dnsSelectedIp() || !name || dnsUiState.mutationBusy; }
    function renderDnsIpCalendar() {
      const calendar = document.querySelector('[data-dns-ip-calendar]');
      const grid = calendar?.querySelector('[data-dns-ip-grid]');
      const rangeLabel = calendar?.querySelector('[data-dns-ip-range]');
      const readout = calendar?.querySelector('[data-dns-ip-readout]');
      if (!calendar || !grid) return;
      const device = dnsRosterDevice();
      calendar.hidden = !device;
      if (!device) { grid.replaceChildren(); if (readout) readout.textContent = 'Pick a device to load available addresses.'; dnsUpdateAddSave(); return; }
      const range = dnsCalendarRange();
      if (!range || !dnsUiState.identityReady) { grid.replaceChildren(); if (rangeLabel) rangeLabel.textContent = 'Unavailable'; if (readout) readout.textContent = 'DHCP boundary, leases, and reservations must be available before selecting an address.'; calendar.querySelector('[data-dns-ip-prev]')?.setAttribute('disabled', ''); calendar.querySelector('[data-dns-ip-next]')?.setAttribute('disabled', ''); dnsUpdateAddSave(); return; }
      dnsUiState.calendarStart = range.start;
      dnsUiState.calendarEnd = range.end;
      const pageSize = 32;
      const pageCount = Math.ceil((range.end - range.start + 1) / pageSize);
      dnsUiState.calendarPage = Math.max(0, Math.min(pageCount - 1, dnsUiState.calendarPage));
      const first = range.start + dnsUiState.calendarPage * pageSize;
      const last = Math.min(range.end, first + pageSize - 1);
      if (rangeLabel) rangeLabel.textContent = `${first} - ${last}`;
      const previous = calendar.querySelector('[data-dns-ip-prev]');
      const next = calendar.querySelector('[data-dns-ip-next]');
      if (previous) previous.disabled = dnsUiState.calendarPage <= 0;
      if (next) next.disabled = last >= range.end;
      grid.replaceChildren(...Array.from({ length: Math.max(0, last - first + 1) }, (_, offset) => {
        const octet = first + offset;
        const address = `192.168.123.${octet}`;
        const availability = dnsIpAvailability(address, device);
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'ui-lan-ip-calendar__octet';
        button.dataset.dnsIpOctet = String(octet);
        button.textContent = String(octet);
        button.disabled = !availability.available || dnsUiState.mutationBusy;
        button.setAttribute('role', 'gridcell');
        button.setAttribute('aria-pressed', String(dnsUiState.selectedOctet === octet));
        button.setAttribute('aria-label', availability.holder ? `${address}, held by ${availability.holder}${availability.heldByPicked ? ' (picked device)' : ''}` : `Select ${address}`);
        if (availability.holder) button.title = availability.heldByPicked ? `${availability.holder} — picked device` : availability.holder;
        return button;
      }));
      const selected = dnsSelectedIp();
      const selectedStatus = selected ? dnsIpAvailability(selected, device) : null;
      if (readout) readout.textContent = selected ? `Selected address: ${selected}${selectedStatus?.heldByPicked ? ` (held by ${dnsDeviceLabel(device)})` : ''}` : 'Select an available address. Addresses held by other devices are unavailable.';
      dnsUpdateAddSave();
    }
    function selectDnsDevice(mac) {
      const device = dnsRosterDevice(mac);
      if (!device || !dnsUiState.identityReady || dnsUiState.mutationBusy) return;
      dnsUiState.pickedMac = String(device.mac || '');
      const range = dnsCalendarRange();
      const ownOctet = dnsOctet(dnsAddress(device));
      let selected = ownOctet !== null && range && ownOctet >= range.start && ownOctet <= range.end && dnsIpAvailability(`192.168.123.${ownOctet}`, device).available ? ownOctet : null;
      if (selected === null && range) { for (let octet = range.start; octet <= range.end; octet += 1) if (dnsIpAvailability(`192.168.123.${octet}`, device).available) { selected = octet; break; } }
      dnsUiState.selectedOctet = selected;
      dnsUiState.calendarPage = selected !== null && range ? Math.floor((selected - range.start) / 32) : 0;
      const selectedNode = document.querySelector('[data-dns-picked-device]');
      if (selectedNode) selectedNode.textContent = `Selected device: ${dnsDeviceLabel(device)} (${dnsValue(device.mac)}) · current address ${dnsValue(dnsAddress(device))}`;
      const nameInput = document.querySelector('[data-dns-new-name]');
      const suggestion = String(dnsArray(device.dns_names)[0] || device.declared_reservation?.hostname || device.observed_lease?.hostname || device.hostname || device.name || '').replace(/\.home\.arpa\.?$/i, '');
      if (nameInput && (!nameInput.value.trim() || nameInput.value.trim() === dnsUiState.nameDefault)) { nameInput.value = suggestion; dnsUiState.nameDefault = suggestion; }
      renderDnsModalRoster();
      renderDnsIpCalendar();
      dnsUpdateAddSave();
    }
    function openDnsModal() { if (!dnsUiState.identityReady || dnsUiState.mutationBusy) return; const modal = document.querySelector('[data-dns-add-modal]'); if (!modal) return; modal.hidden = false; renderDnsModalRoster(); renderDnsModalRecords(); if (dnsUiState.pickedMac) renderDnsIpCalendar(); document.querySelector('[data-dns-new-name]')?.focus(); }
    function closeDnsModal() { const modal = document.querySelector('[data-dns-add-modal]'); if (modal) modal.hidden = true; dnsUiState.pickedMac = null; dnsUiState.selectedOctet = null; dnsUiState.nameDefault = ''; const nameInput = document.querySelector('[data-dns-new-name]'); if (nameInput) nameInput.value = ''; const picked = document.querySelector('[data-dns-picked-device]'); if (picked) picked.textContent = 'Choose a device for this name.'; const calendar = document.querySelector('[data-dns-ip-calendar]'); if (calendar) calendar.hidden = true; dnsUpdateAddSave(); }
    function dnsNormalizeBareLabel(value) { const label = String(value ?? '').trim().toLowerCase().replace(/\.$/, '').replace(/\.home\.arpa$/, ''); if (!label) return { label: '', error: 'Enter a hostname.' }; if (label.includes('.')) return { label: '', error: 'Use a single hostname label; additional dots are not allowed.' }; if (label.length > 63) return { label: '', error: 'Hostname must be 63 characters or fewer.' }; if (label.match(/^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/)?.[0] !== label) return { label: '', error: /^[a-z0-9]/.test(label) && /[a-z0-9]$/.test(label) ? 'Hostname may contain only letters, numbers, and hyphens.' : 'Hostname must start and end with a letter or number; hyphens are allowed only inside the label.' }; return { label, error: '' }; }
    async function saveDnsName() {
      const input = document.querySelector('[data-dns-new-name]');
      const result = document.querySelector('[data-dns-add-result]');
      const normalized = dnsNormalizeBareLabel(input?.value);
      if (normalized.error) { if (result) result.textContent = normalized.error; return; }
      const hostname = normalized.label;
      const device = dnsRosterDevice();
      const address = dnsSelectedIp();
      if (!device || !address || !dnsUiState.identityReady || dnsUiState.mutationBusy) { if (result) result.textContent = 'Choose a device and available DHCP address.'; return; }
      if (result) result.textContent = 'Adding name…';
      dnsSetMutationBusy(true);
      try {
        const receipt = await dnsJson('/api/v1/network/dns/device-name/create', { method: 'POST', body: JSON.stringify({ hostname, ip: address }) });
        dnsRequireAccepted(receipt);
        showCoronatioToast('LAN name added.', 'success');
        closeDnsModal();
        await hydrateDns();
      } catch (failure) { if (result) result.textContent = `Unable to add name: ${failure.message}`; showCoronatioToast(failure?.message || 'DNS request failed', 'error'); await hydrateDns(); }
      finally { dnsSetMutationBusy(false); }
    }
    async function removeDnsName(button) {
      const name = button?.dataset?.dnsNameRemove;
      const address = button?.dataset?.dnsIp;
      const row = dnsNameRows().find(item => item.name === name && item.address === address && item.removable);
      const normalized = dnsNormalizeBareLabel(row?.name);
      if (!row || !row.removable || !normalized.label || !dnsUiState.identityReady || dnsUiState.mutationBusy) return;
      dnsSetMutationBusy(true);
      try {
        const receipt = await dnsJson('/api/v1/network/dns/device-name/remove', { method: 'POST', body: JSON.stringify({ hostname: normalized.label, ip: row.address }) });
        dnsRequireAccepted(receipt);
        showCoronatioToast('Owned A and PTR records removed.', 'success');
        await hydrateDns();
      } catch (failure) { showCoronatioToast(failure?.message || 'Unable to remove DNS name', 'error'); await hydrateDns(); }
      finally { dnsSetMutationBusy(false); }
    }
    async function updateDnsBlocklist() {
      if (!dnsUiState.resolverReady || dnsUiState.mutationBusy) return;
      dnsSetMutationBusy(true);
      try { const receipt = await dnsJson('/api/v1/network/dns/blocklist/update', { method: 'POST' }); dnsRequireAccepted(receipt); showCoronatioToast('Blocklist update accepted.', 'success'); await hydrateDns(); }
      catch (failure) { showCoronatioToast(failure?.message || 'Blocklist update failed', 'error'); await hydrateDns(); }
      finally { dnsSetMutationBusy(false); }
    }
    async function setDnsAdblock(enabled) {
      if (!dnsUiState.resolverReady || typeof dnsUiState.resolver?.adblockEnabled !== 'boolean' || dnsUiState.mutationBusy) return;
      dnsSetMutationBusy(true);
      try { const receipt = await dnsJson('/api/v1/network/dns/adblock', { method: 'POST', body: JSON.stringify({ enabled }) }); dnsRequireAccepted(receipt); showCoronatioToast(`Ad blocking ${enabled ? 'enabled' : 'disabled'}.`, 'success'); await hydrateDns(); }
      catch (failure) { showCoronatioToast(failure?.message || 'Ad blocking change failed', 'error'); await hydrateDns(); }
      finally { dnsSetMutationBusy(false); }
    }
    async function saveDnsUpstream() {
      if (!dnsUiState.resolverReady || !Array.isArray(dnsUiState.resolver?.upstreams) || typeof dnsUiState.resolver?.dnsOverTls !== 'boolean' || dnsUiState.mutationBusy) return;
      const providers = [...new Set([...document.querySelectorAll('[data-dns-upstream-provider]:checked')].map(input => input.value).filter(name => dnsProviders[name]))];
      const customToggle = document.querySelector('[data-dns-upstream-custom-toggle]');
      const typedCustom = customToggle?.checked ? (document.querySelector('[data-dns-upstream-custom-addresses]')?.value || '').split(',').map(dnsUpstreamBase).filter(Boolean) : [];
      const providerAddresses = [...new Set(providers.flatMap(name => dnsProviders[name]))];
      const custom = [...new Set(typedCustom)].filter(address => !providerAddresses.includes(address));
      const addresses = [...new Set([...providerAddresses, ...custom])];
      const result = addresses.length ? null : 'Choose at least one upstream resolver.';
      if (result) { dnsSetState(result); return; }
      if (addresses.length > 8) { dnsSetState('Caduceus accepts at most eight upstream addresses.'); return; }
      const dot = Boolean(document.querySelector('[data-dns-dot]')?.checked);
      const payload = providers.length === 1 && custom.length === 0 ? { preset: providers[0], custom: null, dot } : { preset: null, custom: addresses, dot };
      dnsSetMutationBusy(true);
      try { const receipt = await dnsJson('/api/v1/network/dns/upstream', { method: 'POST', body: JSON.stringify(payload) }); dnsRequireAccepted(receipt); showCoronatioToast('Resolver settings saved.', 'success'); await hydrateDns(); }
      catch (failure) { showCoronatioToast(failure?.message || 'Resolver change failed', 'error'); await hydrateDns(); }
      finally { dnsSetMutationBusy(false); }
    }
    async function hydrateDns() {
      if (!viewportFamilyAdmitted('unbound')) return false;
      dnsUiState.identityReady = false;
      dnsUiState.resolverReady = false;
      dnsUiState.resolver = null;
      dnsSetResolverControls();
      dnsSetIdentityControls();
      if (dnsUiState.pickedMac) renderDnsIpCalendar();
      renderResolverStatus(null);
      dnsSetState('Reading local names and resolver status…');
      const identityRead = hydrateIdentity();
      const resolverRead = dnsJson('/api/v1/network/dns/resolver/status').then(envelope => { if (!envelope || typeof envelope !== 'object') throw new Error('resolver-status-body-unavailable'); return resolverReceipt(envelope); });
      const [identityResult, resolverResult] = await Promise.allSettled([identityRead, resolverRead]);
      const identityError = identityResult.status === 'rejected' ? String(identityResult.reason?.message || identityResult.reason) : identityResult.value === true ? null : 'network-identity-read-unavailable';
      if (!identityError) {
        dnsUiState.identityReady = true;
        if (dnsUiState.pickedMac && !dnsRosterDevice()) { dnsUiState.pickedMac = null; dnsUiState.selectedOctet = null; }
        renderDnsNames();
        renderDnsModalRoster();
        renderDnsModalRecords();
        renderDnsIpCalendar();
      } else {
        dnsUiState.identityReady = false;
        identityState.roster = [];
        identityState.boundary = {};
        identityState.leases = [];
        identityState.reservations = [];
        identityState.dns = {};
        dnsUiState.pickedMac = null;
        dnsUiState.selectedOctet = null;
        dnsUiState.calendarStart = null;
        dnsUiState.calendarEnd = null;
        const nameInput = document.querySelector('[data-dns-new-name]');
        if (nameInput && nameInput.value.trim() === dnsUiState.nameDefault) nameInput.value = '';
        dnsUiState.nameDefault = '';
        const selected = document.querySelector('[data-dns-picked-device]');
        if (selected) selected.textContent = 'Choose a device for this name.';
        renderDnsNames();
        renderDnsModalRoster();
        renderDnsModalRecords();
        renderDnsIpCalendar();
      }
      let resolverError = null;
      let resolverPartial = false;
      if (resolverResult.status === 'fulfilled') {
        const status = resolverResult.value || {};
        dnsUiState.resolver = status;
        dnsUiState.resolverReady = true;
        resolverPartial = typeof status.adblockEnabled !== 'boolean' || !Array.isArray(status.upstreams) || typeof status.dnsOverTls !== 'boolean' || !Number.isFinite(status.blocklistDomainCount) || !(status.blocklistLastUpdate === null || Number.isFinite(status.blocklistLastUpdate)) || typeof status.unboundActive !== 'boolean';
        renderResolverStatus(status);
      } else {
        resolverError = String(resolverResult.reason?.message || resolverResult.reason);
        dnsUiState.resolver = null;
        dnsUiState.resolverReady = false;
        renderResolverStatus(null);
      }
      dnsSetResolverControls();
      dnsSetIdentityControls();
      const identityMessage = identityError ? `Local names unavailable: ${identityError}.` : 'Local names are current.';
      const resolverMessage = resolverError ? `Resolver status unavailable: ${resolverError}.` : resolverPartial ? 'Resolver status loaded; some optional fields are unavailable.' : 'Resolver status is current.';
      dnsSetState(`${identityMessage} ${resolverMessage}`);
      return !identityError && !resolverError;
    }
    document.addEventListener('click', async event => {
      if (event.target.closest?.('[data-dns-add-name]')) return openDnsModal();
      if (event.target.closest?.('[data-dns-add-close]')) return closeDnsModal();
      if (event.target.closest?.('[data-dns-add-save]')) return saveDnsName();
      const pick = event.target.closest?.('[data-dns-device-pick]');
      if (pick) { selectDnsDevice(pick.dataset.dnsDevicePick); return; }
      const octet = event.target.closest?.('[data-dns-ip-octet]');
      if (octet && !octet.disabled) { dnsUiState.selectedOctet = Number(octet.dataset.dnsIpOctet); renderDnsIpCalendar(); return; }
      if (event.target.closest?.('[data-dns-ip-prev]')) { dnsUiState.calendarPage = Math.max(0, dnsUiState.calendarPage - 1); renderDnsIpCalendar(); return; }
      if (event.target.closest?.('[data-dns-ip-next]')) { dnsUiState.calendarPage += 1; renderDnsIpCalendar(); return; }
      const remove = event.target.closest?.('[data-dns-name-remove]');
      if (remove) return removeDnsName(remove);
      if (event.target.closest?.('[data-dns-blocklist-update]')) return updateDnsBlocklist();
      if (event.target.closest?.('[data-dns-upstream-save]')) return saveDnsUpstream();
      if (event.target.closest?.('[data-dns-refresh]')) return hydrateDns();
    }, true);
    document.addEventListener('change', event => {
      const adblock = event.target.closest?.('[data-dns-adblock]');
      if (adblock) { void setDnsAdblock(adblock.checked); return; }
      if (event.target.closest?.('[data-dns-upstream-provider], [data-dns-upstream-custom-toggle], [data-dns-dot]')) {
        const customToggle = document.querySelector('[data-dns-upstream-custom-toggle]');
        const custom = document.querySelector('[data-dns-upstream-custom]');
        if (custom && customToggle) custom.hidden = !customToggle.checked;
        renderDnsUpstreamReadout();
      }
    }, true);
    document.addEventListener('input', event => {
      if (event.target.closest?.('[data-dns-new-name]')) { if (event.target.value.trim() !== dnsUiState.nameDefault) dnsUiState.nameDefault = ''; dnsUpdateAddSave(); }
      if (event.target.closest?.('[data-dns-upstream-custom-addresses]')) renderDnsUpstreamReadout();
    }, true);
    document.addEventListener('keydown', event => {
      const current = event.target.closest?.('[data-dns-ip-octet]');
      if (!current || !['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End'].includes(event.key)) return;
      const buttons = [...current.closest('[data-dns-ip-grid]').querySelectorAll('[data-dns-ip-octet]:not(:disabled)')];
      const index = buttons.indexOf(current);
      if (index < 0) return;
      const offsets = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: -8, ArrowDown: 8 };
      const next = event.key === 'Home' ? 0 : event.key === 'End' ? buttons.length - 1 : Math.max(0, Math.min(buttons.length - 1, index + (offsets[event.key] || 0)));
      event.preventDefault();
      buttons[next]?.focus();
    });
"####
}