fn shell_dhcp_client() -> &'static str {
    r####"    // One active binding owns this bounded last-good projection; reads stay independent.
    const identityState = { roster: [], boundary: {}, dhcpBoundary: {}, dhcpBoundaryCommitted: null, leases: [], reservations: [], dns: {}, notes: {}, modalMac: null, anonymized: false, reservation: null, binding: null, ready: { roster: false, boundary: false, leases: false, reservations: false, dns: false }, dhcpReady: { statistics: false, pool: false }, statistics: null, writeBusy: false };
    const identityPerspective = Object.freeze({
      address: Object.freeze({ lead: ['MAC', 'Observed lease', 'Declared reservation'], emphasis: 'address', writable: true }),
      name: Object.freeze({ lead: ['MAC', 'Names', 'Claim state'], emphasis: 'name', writable: true }),
      liveness: Object.freeze({ lead: ['MAC', 'Last activity', 'Observed IP'], emphasis: 'liveness', writable: false, notes: true }),
    });
    const identityText = value => String(value ?? '—');
    const identityRows = payload => { let value = payload; for (let depth = 0; depth < 5; depth += 1) { if (Array.isArray(value)) return value; if (!value || typeof value !== 'object') return []; for (const key of ['devices', 'roster', 'leases', 'reservations', 'items']) if (Array.isArray(value[key])) return value[key]; value = value.payload?.result || value.payload || value.result || value.data?.result || value.data || value.receipt; } return []; };
    function identityReadRefusal(value, depth = 0) { if (!value || typeof value !== 'object' || depth > 5) return null; if (value.ok === false || value.success === false) return value.firstMissingSignal || value.first_failing_boundary || value.error || value.first_missing_signal || 'network-identity-read-refused'; for (const key of ['payload', 'result', 'data', 'receipt', 'readback']) { const refusal = identityReadRefusal(value[key], depth + 1); if (refusal) return refusal; } return null; }
    function identityResult(body) { const refusal = identityReadRefusal(body); if (refusal) throw new Error(refusal); if (!body || body.ok !== true || !body.payload || body.payload.ok !== true || !Object.prototype.hasOwnProperty.call(body.payload, 'result')) throw new Error('Network identity response is incomplete'); return body.payload.result; }
    function identityArray(body) { const result = identityResult(body); if (!Array.isArray(result)) throw new Error('Network identity array is unavailable'); return result; }
    function identityJson(route, options = {}) { return fetch(route, { cache: 'no-store', headers: options.body ? {'content-type':'application/json'} : {}, ...options }).then(async response => { const body = await response.json().catch(() => ({})); if (!response.ok) throw new Error(body.first_failing_boundary || body.firstMissingSignal || body.error || `Request failed (${response.status})`); return body; }); }
    function identityCurrent(binding = identityState.binding) { return Boolean(binding && identityState.binding === binding && headerState.isAdmin && ['dhcp', 'unbound'].includes(binding.guest) && window.scaffoldRegistry?.current(binding)); }
    function identityHas(binding, keys) { return identityCurrent(binding) && keys.every(key => identityState.ready[key] === true); }
    function identityMutationSlot(binding) { return binding?.root?.querySelector('[data-scaffold-slot="identity-mutation"]') || binding?.root?.querySelector('[data-scaffold-slot="identity-roster"]') || null; }
    function identitySetControls(binding = identityState.binding, preserveReadySlots = false) {
      const root = binding?.root; if (!(root instanceof Element) || !root.isConnected) return;
      const current = identityCurrent(binding); const addressReady = identityHas(binding, ['roster', 'boundary', 'leases', 'reservations']); const reservationReady = identityHas(binding, ['roster', 'reservations']);
      root.querySelectorAll('[data-dhcp-pin], [data-identity-claim]').forEach(node => { if (scaffoldReadProjectionAllowed(node, preserveReadySlots)) node.disabled = !current || !addressReady || identityState.writeBusy; });
      root.querySelectorAll('[data-dhcp-edit], [data-dhcp-remove]').forEach(node => { if (scaffoldReadProjectionAllowed(node, preserveReadySlots)) node.disabled = !current || !reservationReady || identityState.writeBusy; });
      root.querySelectorAll('[data-edit-note-button]').forEach(node => { if (scaffoldReadProjectionAllowed(node, preserveReadySlots)) node.disabled = !current || !identityState.ready.roster || identityState.writeBusy; });
      root.querySelectorAll('[data-identity-claim-save], [data-identity-reservation-save]').forEach(node => { if (scaffoldReadProjectionAllowed(node, preserveReadySlots)) node.disabled = !current || !addressReady || identityState.writeBusy; });
      root.querySelectorAll('[data-dhcp-boundary], [data-dhcp-boundary-save]').forEach(node => { if (scaffoldReadProjectionAllowed(node, preserveReadySlots)) node.disabled = !current || !identityState.dhcpReady.statistics || !identityState.dhcpReady.pool || identityState.writeBusy; });
    }
    function identityMarkRetired(binding) {
      if (identityState.binding !== binding) return;
      Object.keys(identityState.ready).forEach(key => { identityState.ready[key] = false; });
      identityState.dhcpReady.statistics = false; identityState.dhcpReady.pool = false;
      const root = binding.root; if (!(root instanceof Element) || !root.isConnected) return;
      identitySetControls(binding);
    }
    function identityAdopt(binding) {
      if (!binding || !headerState.isAdmin || !['dhcp', 'unbound'].includes(binding.guest)) return false;
      if (identityState.binding === binding) return identityCurrent(binding);
      const previous=identityState.binding; if(previous&&previous!==binding)window.scaffoldRegistry?.retire(previous,'Identity binding replaced');
      identityState.binding = binding;
      Object.keys(identityState.ready).forEach(key => { identityState.ready[key] = false; });
      identityState.dhcpReady.statistics = false; identityState.dhcpReady.pool = false;
      identityState.writeBusy = false; identityState.modalMac = null; identityState.reservation = null;
      window.scaffoldRegistry.onRetire(binding, () => identityMarkRetired(binding));
      return identityCurrent(binding);
    }
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane, binding = identityState.binding;
      if (!['dhcp', 'unbound'].includes(detail.id) || !(pane instanceof Element) || !pane.isConnected || document.querySelector(`[data-pane-panel="${detail.id}"]`) !== pane) return;
      if (!binding || binding.guest !== detail.id || window.scaffoldRegistry?.current(binding)) return;
      identityState.roster = []; identityState.boundary = {}; identityState.dhcpBoundary = {}; identityState.dhcpBoundaryCommitted = null;
      identityState.leases = []; identityState.reservations = []; identityState.dns = {}; identityState.notes = {};
      identityState.modalMac = null; identityState.anonymized = false; identityState.reservation = null; identityState.writeBusy = false;
      Object.keys(identityState.ready).forEach(key => { identityState.ready[key] = false; });
      identityState.dhcpReady.statistics = false; identityState.dhcpReady.pool = false; identityState.statistics = null; identityState.binding = null;
    });
    function identityRead(binding, key, route, slotName, validate, apply, options = {}) {
      const slot = binding.root.querySelector(`[data-scaffold-slot="${slotName}"]`); if (!slot) return Promise.resolve(false);
      return scaffoldRegistry.run(binding, `identity-${key}`, async ctx => {
        if (!ctx.current() || !identityCurrent(binding)) return false;
        identityState.ready[key] = false;
        identitySetControls(binding, true); if (key === 'boundary' || key === 'leases' || key === 'reservations' || key === 'roster') renderIdentityRoster('address', binding, true);
        document.dispatchEvent(new CustomEvent('coronatio:network-identity-started', { detail: { binding, key } }));
        const body = await ctx.fetchJson(route);
        if (!ctx.current() || !identityCurrent(binding)) return false;
        const value = validate(body);
        if (!ctx.current() || !identityCurrent(binding)) return false;
        apply(value);
        if (!ctx.current() || !identityCurrent(binding)) return false;
        identityState.ready[key] = true;
        identityRender(binding);
        identitySetControls(binding);
        document.dispatchEvent(new CustomEvent('coronatio:network-identity-updated', { detail: { binding, key } }));
        ctx.ready(slot);
        const error = binding.root.querySelector('[data-dhcp-error]'); if (error && key !== 'dns') error.hidden = true;
        return true;
      }, { slots: [slot], supersede: options.supersede === true, onFailure: ({ current, reason }) => { if (!current() || !identityCurrent(binding)) return; identityState.ready[key] = false; identitySetControls(binding, true); if (key === 'boundary' || key === 'leases' || key === 'reservations') renderIdentityRoster('address', binding, true); const error = binding.root.querySelector('[data-dhcp-error]'); if (error) { error.hidden = false; error.textContent = `Network identity unavailable: ${reason || 'Read failed'}`; } document.dispatchEvent(new CustomEvent('coronatio:network-identity-failed', { detail: { binding, key, reason } })); } });
    }
    function identityRender(binding) {
      if (!identityCurrent(binding)) return;
      if (binding.guest === 'dhcp') { const loading=binding.root.querySelector('[data-dhcp-loading]'),slot=binding.root.querySelector('[data-scaffold-slot="identity-roster"]'); if(loading)loading.hidden=identityState.ready.roster||Boolean(slot?.dataset.scaffoldUpdatedAt); renderIdentityRoster('address', binding); }
      identitySetControls(binding);
    }
    function startIdentityReads(binding, options = {}) {
      if (!identityAdopt(binding)) return Promise.resolve(false);
      const reads = [
        identityRead(binding, 'roster', '/api/network/device', 'identity-roster', identityArray, value => { identityState.roster = value; }, options),
        identityRead(binding, 'boundary', '/api/network/dhcp/boundary', 'identity-boundary', body => { const rows = identityArray(body); if (!rows.length || !rows[0] || typeof rows[0] !== 'object' || Array.isArray(rows[0]) || typeof rows[0].start !== 'string' || typeof rows[0].end !== 'string' || !rows[0].start || !rows[0].end) throw new Error('DHCP boundary data is unavailable'); return {start:rows[0].start,end:rows[0].end}; }, value => { identityState.boundary = value; }, options),
        identityRead(binding, 'leases', '/api/network/dhcp/leases', 'identity-leases', identityArray, value => { identityState.leases = value; }, options),
        identityRead(binding, 'reservations', '/api/network/dhcp/reservations', 'identity-reservations', identityArray, value => { identityState.reservations = value; }, options),
        identityRead(binding, 'dns', '/api/network/dns/read', 'identity-dns', body => { const value = identityResult(body); if (!value || typeof value !== 'object' || Array.isArray(value) || !Array.isArray(value.devices) || !Array.isArray(value.aliases)) throw new Error('DNS record data is unavailable'); return {devices:value.devices,aliases:value.aliases}; }, value => { identityState.dns = value; }, options),
      ];
      return Promise.all(reads);
    }
    function identityIp(row, key) { return row?.[key]?.ip || '—'; }
    function dhcpMasked(value, kind) { const raw = String(value || '—'); if (!identityState.anonymized) return escapeHtml(raw); if (kind === 'mac') return raw.split(':').map((part, index) => index < 3 ? part : '••').join(':'); if (kind === 'ip') return raw.replace(/\.\d+$/, '.•••'); return raw ? 'Private device' : '—'; }
    function identityReservation(row) { const reservation = row?.declared_reservation; if (!reservation || typeof reservation !== 'object') return null; const id = reservation.id || reservation.reservationId || reservation.reservation_id || row.reservation_id || row.reservationId || row.mac; return id ? { id: String(id), ip: reservation.ip || reservation['ip-address'] || reservation.ipAddress || '' } : null; }
    function identityLastActivity(row) { return row?.observed_lease?.last_activity || row?.observed_lease?.lastActivity || row?.observed_lease?.hostname || row?.last_activity || row?.lastActivity || '—'; }
        function identityRosterMarkup(row, perspective) { const cfg = identityPerspective[perspective]; const names = (row.dns_names || []).join(', ') || '—'; const marks = (row.mismatches || []).map(identityText).join(', ') || 'None'; const observed = identityIp(row, 'observed_lease'); const declared = identityIp(row, 'declared_reservation'); if (!cfg.writable) { const canonicalMac = canonicalNetworkNoteMac(row.mac), note = canonicalMac ? identityState.notes[canonicalMac] ?? '' : '', pencil = headerState.isAdmin ? `<button type="button" class="edit-note-button" data-edit-note-button data-mac="${escapeHtml(row.mac)}" data-note="${escapeHtml(note)}" title="Edit device note" aria-label="Edit note for ${escapeHtml(row.mac)}"><i class="fas fa-pencil-alt" aria-hidden="true"></i></button>` : ''; return `<tr data-device-mac="${escapeHtml(row.mac)}" data-scaffold-key="identity-row:${escapeHtml(row.mac)}" data-roster-emphasis="${cfg.emphasis}"><td><code>${dhcpMasked(row.mac, 'mac')}</code></td><td>${dhcpMasked(identityLastActivity(row), 'hostname')}</td><td>${dhcpMasked(observed, 'ip')}</td><td>${dhcpMasked(names, 'hostname')}</td><td class="device-note-cell" data-label="Note:"><span class="note-text" data-note-text data-mac="${escapeHtml(row.mac)}">${dhcpMasked(note, 'hostname')}</span>${pencil}</td><td>${escapeHtml(row.claim_state || 'unclaimed')}</td><td>${escapeHtml(marks)}</td></tr>`; } const reservation = identityReservation(row); if (perspective === 'address') { const state = reservation ? `<span class="dhcp-pin-badge" aria-label="Pinned address">Pinned</span>` : '<span class="dhcp-unpinned-state">Available to pin</span>'; const action = reservation ? `<button type="button" class="ui-button ui-button--secondary ui-button--small" data-dhcp-edit="${escapeHtml(reservation.id)}" data-dhcp-ip="${escapeHtml(reservation.ip)}">Edit</button><button type="button" class="ui-button ui-button--danger ui-button--small" data-dhcp-remove="${escapeHtml(reservation.id)}">Unreserve</button>` : `<button type="button" class="ui-button ui-button--primary ui-button--small" data-dhcp-pin="${escapeHtml(row.mac)}">Pin</button>`; return `<tr class="dhcp-device-row ${reservation ? 'dhcp-device-row--pinned' : 'dhcp-device-row--lease'}" data-device-mac="${escapeHtml(row.mac)}" data-scaffold-key="identity-row:${escapeHtml(row.mac)}"><td><code>${dhcpMasked(row.mac, 'mac')}</code></td><td>${dhcpMasked(observed, 'ip')}</td><td>${dhcpMasked(declared, 'ip')}</td><td>${dhcpMasked(names, 'hostname')}</td><td>${state}</td><td class="dhcp-device-actions">${action}</td></tr>`; } const lead = names; const action = reservation ? `<button type="button" class="ui-button ui-button--secondary ui-button--small" data-dhcp-edit="${escapeHtml(reservation.id)}" data-dhcp-ip="${escapeHtml(reservation.ip)}">Edit</button><button type="button" class="ui-button ui-button--danger ui-button--small" data-dhcp-remove="${escapeHtml(reservation.id)}">Remove</button>` : `<button type="button" class="ui-button ui-button--primary ui-button--small" data-identity-claim="${escapeHtml(row.mac)}">Claim</button>`; return `<tr data-device-mac="${escapeHtml(row.mac)}" data-scaffold-key="identity-row:${escapeHtml(row.mac)}" data-roster-emphasis="${cfg.emphasis}"><td><code>${dhcpMasked(row.mac, 'mac')}</code></td><td>${dhcpMasked(lead, 'hostname')}</td><td>${dhcpMasked(observed, 'ip')}</td><td>${dhcpMasked(declared, 'ip')}</td><td>${dhcpMasked(names, 'hostname')}</td><td>${escapeHtml(row.claim_state || 'unclaimed')}</td><td>${escapeHtml(marks)}</td><td>${action}</td></tr>`; }
    function renderIdentityRoster(perspective, suppliedBinding = null, preserveReadySlots = false) {
      const binding = suppliedBinding || (identityCurrent() ? identityState.binding : scaffoldRegistry.capture('stats'));
      if (!binding || !scaffoldRegistry.current(binding)) return;
      const targets = binding.root.querySelectorAll(`[data-identity-roster-host][data-identity-perspective="${perspective}"] [data-identity-roster]`);
      targets.forEach(target => {
        if (!scaffoldReadProjectionAllowed(target, preserveReadySlots)) return;
        const cfg = identityPerspective[perspective]; if (!cfg) return;
        const isDhcpAddress = perspective === 'address';
        const headings = isDhcpAddress ? '<th>MAC</th><th>Observed lease</th><th>Declared reservation</th><th>DNS names</th><th>Status</th><th>Action</th>' : cfg.writable ? `<th>${cfg.lead[0]}</th><th>${cfg.lead[1]}</th><th>Observed lease</th><th>Declared reservation</th><th>DNS names</th><th>${cfg.lead[2]}</th><th>Mismatches</th><th>Action</th>` : `<th>${cfg.lead[0]}</th><th>${cfg.lead[1]}</th><th>${cfg.lead[2]}</th><th>Names</th><th>Note</th><th>Claim state</th><th>Mismatches</th>`;
        const columns = isDhcpAddress ? 6 : cfg.writable ? 8 : 7; const tableClass = isDhcpAddress ? 'identity-roster dhcp-device-table' : 'identity-roster';
        const html = `<table class="${tableClass}"><thead><tr>${headings}</tr></thead><tbody>${identityState.roster.map(row => identityRosterMarkup(row, perspective)).join('') || `<tr data-scaffold-key="identity-empty-${perspective}"><td colspan="${columns}">${identityState.ready.roster ? 'No devices reported.' : 'Reading device identities…'}</td></tr>`}</tbody></table>`;
        morphLivePane(target, html);
      });
      if (identityCurrent(binding)) identitySetControls(binding, preserveReadySlots);
    }
    function identityWrite(binding, key, route, options, dependencies, accept = () => true) {
      const slot = identityMutationSlot(binding); if (!slot) return Promise.resolve(false);
      return scaffoldRegistry.run(binding, `identity-write-${key}`, async ctx => {
        if (!ctx.current() || !identityCurrent(binding) || !headerState.isAdmin || !dependencies.every(dep => dep === 'dhcp.statistics' ? identityState.dhcpReady.statistics : dep === 'dhcp.pool' ? identityState.dhcpReady.pool : identityState.ready[dep] === true)) return false;
        const response = await fetch(route, { cache: 'no-store', credentials: 'same-origin', signal: ctx.signal, headers: options.body ? {'content-type':'application/json'} : {}, ...options });
        const receipt = await response.json().catch(() => ({}));
        const refusal = identityReadRefusal(receipt);
        if (!response.ok || refusal) throw new Error(refusal || receipt.first_failing_boundary || receipt.firstMissingSignal || receipt.error || `Request failed (${response.status})`);
        if (!ctx.current() || !identityCurrent(binding) || !headerState.isAdmin || !accept(receipt)) return false;
        ctx.ready(slot); return receipt;
      }, { slots: [slot], retry: false, timeoutMs: 15000, onFailure: ({ current, reason }) => { if (!current() || !identityCurrent(binding)) return; const error = binding.root.querySelector('[data-dhcp-error]'); if (error) { error.hidden = false; error.textContent = reason || 'Network identity write failed'; } } });
    }
    function identityAddressAvailable(binding) { return identityHas(binding, ['roster', 'boundary', 'leases', 'reservations']); }
    function nextFreeIp() {
      if (!identityAddressAvailable()) return '';
      const used = new Set([...identityState.leases, ...identityState.reservations].map(row => String(row.ip || row['ip-address'] || row.ipAddress || '')));
      const boundary = identityState.boundary?.boundary || identityState.boundary || {}; const start = String(boundary.start || boundary.first || boundary.pool_start || ''); const end = String(boundary.end || boundary.last || boundary.pool_end || '');
      const n = ip => ip.split('.').reduce((acc, part) => acc * 256 + Number(part), 0); const dotted = value => [24,16,8,0].map(shift => (value >>> shift) & 255).join('.');
      if (!/^\d+(\.\d+){3}$/.test(start) || !/^\d+(\.\d+){3}$/.test(end)) return '';
      for (let value=n(start); value<=n(end); value++) { const ip=dotted(value); if (!used.has(ip)) return ip; } return '';
    }
    function openIdentityClaim(mac) {
      const binding = identityState.binding; if (!identityCurrent(binding) || !identityAddressAvailable(binding)) return;
      const row = identityState.roster.find(item => item.mac === mac); if (!row) return;
      identityState.modalMac = row.mac; const modal = binding.root.querySelector('[data-identity-claim-modal]'); const body = binding.root.querySelector('[data-identity-claim-body]'); if (!modal || !body) return;
      const suggested = nextFreeIp(); const taken = [...new Set([...identityState.leases, ...identityState.reservations].map(item => item.ip || item['ip-address'] || item.ipAddress).filter(Boolean))];
      body.innerHTML = `<p>MAC <code>${escapeHtml(row.mac)}</code> is fixed from this roster row.</p><label>Hostname<input class="ui-input ui-input--medium" data-identity-hostname value="${escapeHtml((row.dns_names || [])[0]?.split('.')[0] || '')}"></label><label>Reserved IP <input class="ui-input ui-input--medium" data-identity-ip value="${escapeHtml(suggested)}" placeholder="${escapeHtml(suggested || 'Staff selects automatically')}"></label><p>Suggested next free: <strong>${escapeHtml(suggested || 'staff decides')}</strong></p><p>Taken: ${escapeHtml(taken.join(', ') || 'none')}</p><p data-identity-claim-result aria-live="polite"></p>`;
      modal.classList.add('open'); modal.setAttribute('aria-hidden','false'); identitySetControls(binding);
    }
    function closeIdentityClaim() { const modal=identityState.binding?.root?.querySelector('[data-identity-claim-modal]'); modal?.classList.remove('open'); modal?.setAttribute('aria-hidden','true'); identityState.modalMac=null; }
    function closeIdentityReservation() { const modal=identityState.binding?.root?.querySelector('[data-identity-reservation-modal]'); modal?.classList.remove('open'); modal?.setAttribute('aria-hidden','true'); identityState.reservation=null; }
    function openIdentityReservation(mode, id, ip) {
      const binding=identityState.binding; if (!identityCurrent(binding) || !identityHas(binding, ['roster','reservations'])) return;
      if (!identityState.reservations.some(row => String(row.id || row.reservation_id || row.reservationId || row.mac) === String(id))) return;
      identityState.reservation={mode,id}; const modal=binding.root.querySelector('[data-identity-reservation-modal]'); const title=binding.root.querySelector('[data-identity-reservation-title]'); const body=binding.root.querySelector('[data-identity-reservation-body]'); const save=binding.root.querySelector('[data-identity-reservation-save]'); if (!modal || !title || !body || !save) return;
      title.textContent=mode === 'edit' ? 'Edit reserved address' : 'Remove reservation'; body.innerHTML=mode === 'edit' ? `<label>Reserved IP <input class="ui-input ui-input--medium" data-identity-reservation-ip value="${escapeHtml(ip || '')}"></label>` : '<p>Remove this reserved address?</p>'; save.textContent=mode === 'edit' ? 'Save' : 'Remove'; modal.classList.add('open'); modal.setAttribute('aria-hidden','false'); identitySetControls(binding);
    }
    async function saveIdentityReservation() {
      const binding=identityState.binding, reservation=identityState.reservation; if (!reservation || !identityCurrent(binding) || !identityHas(binding,['roster','reservations']) || identityState.writeBusy) return;
      const route='/api/dhcp/reservations/' + encodeURIComponent(reservation.id); const body=reservation.mode === 'edit' ? {ip: binding.root.querySelector('[data-identity-reservation-ip]')?.value.trim()} : undefined; if (reservation.mode === 'edit' && !body.ip) return;
      identityState.writeBusy=true; identitySetControls(binding);
      try { const receipt=await identityWrite(binding,'reservation',route,{method:reservation.mode === 'edit' ? 'PUT' : 'DELETE',body:body === undefined ? undefined : JSON.stringify(body)},['roster','reservations']); if (receipt && identityCurrent(binding)) { showCoronatioToast(reservation.mode === 'edit' ? 'Reservation updated.' : 'Reservation removed.','success'); closeIdentityReservation(); await startIdentityReads(binding,{supersede:true}); } }
      catch (failure) { if (identityCurrent(binding)) showCoronatioToast(failure?.message || 'DHCP request failed','error'); }
      finally { if (identityState.binding === binding) { identityState.writeBusy=false; identitySetControls(binding); } }
    }
    async function saveIdentityClaim() {
      const binding=identityState.binding, mac=identityState.modalMac; if (!identityCurrent(binding) || !identityAddressAvailable(binding) || identityState.writeBusy) return;
      const hostname=binding.root.querySelector('[data-identity-hostname]')?.value.trim(); const ip=binding.root.querySelector('[data-identity-ip]')?.value.trim(); if (!mac || !hostname || !identityState.roster.some(row => row.mac === mac)) return;
      const payload=ip ? {mac,ip,hostname} : {mac,autoIp:true,hostname}; const result=binding.root.querySelector('[data-identity-claim-result]'); if (result) result.textContent='Saving…';
      identityState.writeBusy=true; identitySetControls(binding);
      try { const receipt=await identityWrite(binding,'claim','/api/network/device/claim',{method:'POST',body:JSON.stringify(payload)},['roster','boundary','leases','reservations'],value => ['applied','noop','blocked'].includes(value?.state || value?.data?.state)); if (!receipt || !identityCurrent(binding)) return;
        const state=receipt.state || receipt.data?.state; const boundary=receipt.first_failing_boundary || receipt.data?.first_failing_boundary; const message=(state === 'applied' || state === 'noop') ? (state === 'noop' ? 'Already current.' : 'Applied.') : `${state || 'blocked'}${boundary ? `: ${boundary}` : ''}`; if (result) result.textContent=message;
        if (state === 'applied' || state === 'noop') { showCoronatioToast(state === 'noop' ? 'Device already current.' : 'Device claim applied.','success'); await startIdentityReads(binding,{supersede:true}); } else showCoronatioToast(message,'error');
      } catch (failure) { if (identityCurrent(binding)) { if (result) result.textContent=`blocked: ${failure.message}`; showCoronatioToast(result?.textContent || 'DHCP request failed','error'); } }
      finally { if (identityState.binding === binding) { identityState.writeBusy=false; identitySetControls(binding); } }
    }
    async function pinDhcpLease(mac) {
      const binding=identityState.binding; if (!identityCurrent(binding) || !identityAddressAvailable(binding) || !identityState.roster.some(row => row.mac === mac) || identityState.writeBusy) return;
      identityState.writeBusy=true; identitySetControls(binding);
      try { const receipt=await identityWrite(binding,'pin','/api/dhcp/reservations',{method:'POST',body:JSON.stringify({mac})},['roster','boundary','leases','reservations']); if (receipt && identityCurrent(binding)) { showCoronatioToast('Pinned to the next available reserved address.','success'); await startIdentityReads(binding,{supersede:true}); } }
      catch (failure) { if (identityCurrent(binding)) showCoronatioToast(failure?.message || 'DHCP pin request failed','error'); }
      finally { if (identityState.binding === binding) { identityState.writeBusy=false; identitySetControls(binding); } }
    }
    const dhcpNumber = value => Number.isSafeInteger(value) && value >= 0 ? value : null;
    function dhcpText(root, selector, value) { root.querySelectorAll(selector).forEach(target => { target.textContent=String(value); }); }
    function paintDhcpStatistics(statistics, binding) {
      if (!identityCurrent(binding)) return;
      const reservations=dhcpNumber(statistics.reservations_count), leases=dhcpNumber(statistics.leases_count), hosts=dhcpNumber(statistics.host_count);
      dhcpText(binding.root,'[data-dhcp-homeserver]',typeof statistics.homeserver_ip === 'string' && statistics.homeserver_ip ? statistics.homeserver_ip : '—');
      dhcpText(binding.root,'[data-dhcp-reservations-count]',reservations === null ? '—' : reservations);
      dhcpText(binding.root,'[data-dhcp-leases-count]',leases === null ? '—' : leases);
      dhcpText(binding.root,'[data-dhcp-hosts-count]',hosts === null ? '—' : hosts);
      identityState.dhcpReady.statistics=reservations !== null && leases !== null;
      const error=binding.root.querySelector('[data-dhcp-error]'); if (error && !identityState.dhcpReady.statistics) { error.hidden=false; error.textContent='DHCP counts are unavailable; controls remain disabled.'; }
      if (identityState.dhcpReady.statistics) { if (error) error.hidden=true; if (identityState.dhcpBoundary && identityState.dhcpReady.pool) paintDhcpBoundary(identityState.dhcpBoundary,binding); }
      identitySetControls(binding);
    }
    function paintDhcpBoundary(boundary, binding) {
      if (!identityCurrent(binding)) return;
      const statistics=identityState.statistics || {}; const reservations=dhcpNumber(statistics.reservations_count), leases=dhcpNumber(statistics.leases_count), configured=dhcpNumber(boundary?.max_reservations);
      if (reservations === null || leases === null || configured === null) { identityState.dhcpReady.pool=false; identitySetControls(binding); return; }
      const minValue=reservations; const maxValue=leases === 0 ? 249 : 249-leases;
      if (maxValue < minValue) { identityState.dhcpReady.pool=false; identitySetControls(binding); return; }
      const value=Math.min(maxValue,Math.max(minValue,configured)); identityState.dhcpBoundary=boundary; identityState.dhcpBoundaryCommitted=value;
      dhcpText(binding.root,'[data-dhcp-boundary-min]',minValue); dhcpText(binding.root,'[data-dhcp-boundary-max]',maxValue); dhcpText(binding.root,'[data-dhcp-boundary-value]',value);
      binding.root.querySelectorAll('[data-dhcp-boundary]').forEach(slider => { slider.min=String(minValue); slider.max=String(maxValue); slider.value=String(value); });
      identitySetControls(binding);
    }
    function dhcpRead(binding, key, route, slotName, validate, apply, options = {}) {
      const slot=binding.root.querySelector(`[data-scaffold-slot="${slotName}"]`); if (!slot) return Promise.resolve(false);
      return scaffoldRegistry.run(binding,key,async ctx => {
        if (!ctx.current() || !identityCurrent(binding)) return false;
        if (key === 'dhcp-statistics') identityState.dhcpReady.statistics=false; else identityState.dhcpReady.pool=false;
        identitySetControls(binding, true);
        const body=await ctx.fetchJson(route); if (!ctx.current() || !identityCurrent(binding)) return false;
        const value=validate(body); if (!ctx.current() || !identityCurrent(binding)) return false;
        apply(value); if (!ctx.current() || !identityCurrent(binding)) return false;
        ctx.ready(slot); return true;
      },{slots:[slot],supersede:options.supersede === true,onFailure:({current,reason})=>{if(!current()||!identityCurrent(binding))return;if(key==='dhcp-statistics')identityState.dhcpReady.statistics=false;else identityState.dhcpReady.pool=false;identitySetControls(binding,true);const error=binding.root.querySelector('[data-dhcp-error]');if(error){error.hidden=false;error.textContent=`DHCP settings unavailable: ${reason || 'Read failed'}`;}}});
    }
    function hydrateDhcpBoundary(options = {}) {
      const binding=scaffoldRegistry.capture('dhcp'); if (!binding || !identityAdopt(binding)) return Promise.resolve(false);
      const stats=dhcpRead(binding,'dhcp-statistics','/api/dhcp/statistics','dhcp-statistics',body=>{if(!body||typeof body!=='object'||Array.isArray(body))throw new Error('DHCP statistics are unavailable');if(!['homeserver_ip','reservations_count','leases_count','host_count'].some(key=>Object.prototype.hasOwnProperty.call(body,key)))throw new Error('DHCP statistics fields are unavailable');return {homeserver_ip:typeof body.homeserver_ip==='string'?body.homeserver_ip:null,reservations_count:dhcpNumber(body.reservations_count),leases_count:dhcpNumber(body.leases_count),host_count:dhcpNumber(body.host_count)};},value=>{identityState.statistics=value;paintDhcpStatistics(value,binding);},{supersede:options.supersede});
      const pool=dhcpRead(binding,'dhcp-pool-boundary','/api/dhcp/pool-boundary','dhcp-pool-boundary',body=>{if(!body||typeof body!=='object'||Array.isArray(body)||body.ok===false||body.success===false)throw new Error(body?.error || body?.firstMissingSignal || 'DHCP pool boundary refused');const value=body.max_reservations !== undefined ? body : body.payload?.result;const configured=dhcpNumber(value?.max_reservations);if(configured===null)throw new Error('DHCP pool boundary is unavailable');return {max_reservations:configured};},value=>{identityState.dhcpBoundary=value;identityState.dhcpReady.pool=true;if(identityState.dhcpReady.statistics)paintDhcpBoundary(value,binding);else identitySetControls(binding);},{supersede:options.supersede});
      return Promise.all([stats,pool]);
    }
    function hydrateDhcp(options = {}) { const binding=scaffoldRegistry.capture('dhcp'); if (!binding || !identityAdopt(binding)) return Promise.resolve(false); return Promise.all([startIdentityReads(binding,options),hydrateDhcpBoundary(options)]); }
    function hydrateIdentity(options = {}) { const guest=viewportFamilyAdmitted('dhcp')?'dhcp':viewportFamilyAdmitted('unbound')?'unbound':null;const binding=guest && scaffoldRegistry.capture(guest);if(!binding||!identityAdopt(binding))return Promise.resolve(false);return startIdentityReads(binding,options); }
    document.addEventListener('change', event => { const anonymize=event.target.closest?.('[data-dhcp-anonymize]'); if(anonymize&&identityCurrent()&&identityState.binding.root.contains(anonymize)){identityState.anonymized=anonymize.checked;renderIdentityRoster('address',identityState.binding);} },true);
    document.addEventListener('input', event => { const binding=identityState.binding;const slider=event.target.closest?.('[data-dhcp-boundary]');if(slider&&identityCurrent(binding)&&binding.root.contains(slider)&&identityState.dhcpReady.statistics&&identityState.dhcpReady.pool)dhcpText(binding.root,'[data-dhcp-boundary-value]',slider.value); },true);
    document.addEventListener('click', async event => {
      const binding=identityState.binding; const target=event.target instanceof Element?event.target:null; if(!target)return;
      const saveBoundary=target.closest('[data-dhcp-boundary-save]');
      if(saveBoundary){if(!binding?.root.contains(saveBoundary)||!identityCurrent(binding)||saveBoundary.disabled||!identityState.dhcpReady.statistics||!identityState.dhcpReady.pool||identityState.writeBusy)return;const slider=binding.root.querySelector('[data-dhcp-boundary]');if(!slider)return;identityState.writeBusy=true;identitySetControls(binding);try{const receipt=await identityWrite(binding,'pool-boundary-save','/api/dhcp/pool-boundary',{method:'POST',body:JSON.stringify({max_reservations:Number(slider.value)})},['dhcp.statistics','dhcp.pool'],value=>value?.success!==false&&value?.ok!==false);if(receipt&&identityCurrent(binding)){await hydrateDhcpBoundary({supersede:true});showCoronatioToast('Reserved-address boundary applied.','success');}}catch(failure){if(identityCurrent(binding)){slider.value=String(identityState.dhcpBoundaryCommitted ?? '');dhcpText(binding.root,'[data-dhcp-boundary-value]',identityState.dhcpBoundaryCommitted ?? '—');showCoronatioToast(failure?.message||'DHCP boundary request failed','error');}}finally{if(identityState.binding===binding){identityState.writeBusy=false;identitySetControls(binding);}}return;}
      const pin=target.closest('[data-dhcp-pin]');if(pin){if(binding?.root.contains(pin)&&!pin.disabled)return pinDhcpLease(pin.dataset.dhcpPin);return;}
      const claim=target.closest('[data-identity-claim]');if(claim){if(binding?.root.contains(claim)&&!claim.disabled)return openIdentityClaim(claim.dataset.identityClaim);return;}
      const edit=target.closest('[data-dhcp-edit]');if(edit){if(binding?.root.contains(edit)&&!edit.disabled)return openIdentityReservation('edit',edit.dataset.dhcpEdit,edit.dataset.dhcpIp);return;}
      const remove=target.closest('[data-dhcp-remove]');if(remove){if(binding?.root.contains(remove)&&!remove.disabled)return openIdentityReservation('remove',remove.dataset.dhcpRemove);return;}
      if(target.closest('[data-identity-claim-cancel]'))return closeIdentityClaim();if(target.closest('[data-identity-reservation-cancel]'))return closeIdentityReservation();
      if(target.closest('[data-identity-claim-save]'))return saveIdentityClaim();if(target.closest('[data-identity-reservation-save]'))return saveIdentityReservation();
      const refresh=target.closest('[data-dhcp-refresh]');if(refresh){if(binding?.root.contains(refresh)&&identityCurrent(binding)&&!refresh.disabled)return hydrateDhcp({supersede:true});}
    },true);
    window.addEventListener('coronatio:privileged-state-retired', () => {
      window.scaffoldRegistry?.retire('dhcp','Admin role retired');
      identityState.roster=[];identityState.boundary={};identityState.dhcpBoundary={};identityState.dhcpBoundaryCommitted=null;identityState.leases=[];identityState.reservations=[];identityState.dns={};identityState.notes={};identityState.statistics=null;identityState.ready={roster:false,boundary:false,leases:false,reservations:false,dns:false};identityState.dhcpReady={statistics:false,pool:false};identityState.binding=null;identityState.modalMac=null;identityState.reservation=null;identityState.writeBusy=false;
      document.querySelectorAll('[data-pane-panel="dhcp"] [data-identity-roster]').forEach(node=>node.replaceChildren());
      document.querySelectorAll('[data-pane-panel="dhcp"] [data-dhcp-homeserver], [data-pane-panel="dhcp"] [data-dhcp-reservations-count], [data-pane-panel="dhcp"] [data-dhcp-hosts-count], [data-pane-panel="dhcp"] [data-dhcp-leases-count], [data-pane-panel="dhcp"] [data-dhcp-boundary-min], [data-pane-panel="dhcp"] [data-dhcp-boundary-max], [data-pane-panel="dhcp"] [data-dhcp-boundary-value]').forEach(node=>node.textContent='—');
      document.querySelectorAll('[data-pane-panel="dhcp"] [data-dhcp-boundary]').forEach(node=>{node.disabled=true;});
      document.querySelectorAll('[data-pane-panel="dhcp"] [data-dhcp-loading]').forEach(node=>{node.hidden=true;});
      document.querySelectorAll('[data-pane-panel="dhcp"] [data-identity-claim-modal], [data-pane-panel="dhcp"] [data-identity-reservation-modal]').forEach(node=>{node.classList.remove('open');node.setAttribute('aria-hidden','true');});
    });
"####
}