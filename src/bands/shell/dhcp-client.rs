fn shell_dhcp_client() -> &'static str {
    r####"    const dhcpState = { leases: [], reservations: [], statistics: {}, boundary: {}, anonymized: false };
    function dhcpHeaders(json = false) {
      const token = localStorage.getItem('coronatioAdminToken');
      return { ...(json ? { 'content-type': 'application/json' } : {}), ...(token ? { 'X-Admin-Token': token } : {}) };
    }
    function dhcpRows(payload, key) {
      if (Array.isArray(payload)) return payload;
      for (const candidate of [payload?.[key], payload?.data?.[key], payload?.result?.[key], payload?.items]) if (Array.isArray(candidate)) return candidate;
      return [];
    }
    function dhcpField(row, ...names) {
      for (const name of names) if (row?.[name] !== undefined && row?.[name] !== null) return row[name];
      return '';
    }
    function dhcpMasked(value, kind) {
      const raw = String(value || '—');
      if (!dhcpState.anonymized) return escapeHtml(raw);
      if (kind === 'mac') return raw.split(':').map((part, index) => index < 3 ? part : '••').join(':');
      if (kind === 'ip') return raw.replace(/\.\d+$/, '.•••');
      return raw ? 'Private device' : '—';
    }
    function dhcpSet(selector, value) { const node = document.querySelector(selector); if (node) node.textContent = value; }
    function setDhcpBoundaryLabel(value) { dhcpSet('[data-dhcp-boundary-value]', value); }
    function setDhcpBoundaryRange(minimum, maximum) {
      dhcpSet('[data-dhcp-boundary-min]', minimum);
      dhcpSet('[data-dhcp-boundary-max]', maximum);
    }
    function renderDhcp() {
      const reservations = dhcpState.reservations;
      const reservedMacs = new Set(reservations.map(row => String(dhcpField(row, 'hw-address', 'hwAddress', 'mac', 'macAddress')).toLowerCase()));
      const leases = dhcpState.leases.filter(row => !reservedMacs.has(String(dhcpField(row, 'hw-address', 'hwAddress', 'mac', 'macAddress')).toLowerCase()));
      const rows = [...reservations.map(row => ({ row, type: 'reservation' })), ...leases.map(row => ({ row, type: 'lease' }))];
      const target = document.querySelector('[data-dhcp-items]');
      if (target) target.innerHTML = rows.map(({ row, type }) => {
        const mac = dhcpField(row, 'hw-address', 'hwAddress', 'mac', 'macAddress');
        const ip = dhcpField(row, 'ip-address', 'ipAddress', 'ip');
        const hostname = dhcpField(row, 'hostname', 'hostName');
        const identifier = dhcpField(row, 'id', 'reservationId') || mac || ip;
        const action = type === 'reservation'
          ? `<button type="button" class="ui-button ui-button--secondary ui-button--small" data-dhcp-edit="${escapeHtml(identifier)}" data-dhcp-ip="${escapeHtml(ip)}">Edit</button><button type="button" class="ui-button ui-button--danger ui-button--small" data-dhcp-remove="${escapeHtml(identifier)}">Remove</button>`
          : `<button type="button" class="ui-button ui-button--primary ui-button--small" data-dhcp-pin="${escapeHtml(mac)}" data-dhcp-hostname="${escapeHtml(hostname)}">Pin</button>`;
        const badge = type === 'reservation' ? 'ui-badge ui-badge--success ui-badge--small' : 'ui-badge ui-badge--info ui-badge--small';
        return `<div class="dhcp-list-item ${type === 'reservation' ? 'pinned' : 'lease'}"><div class="dhcp-list-item-content"><div class="dhcp-list-item-main"><div class="dhcp-list-item-info"><div class="dhcp-list-item-mac"><span class="info-label">MAC Address:</span><span class="info-value">${dhcpMasked(mac, 'mac')}</span></div><div class="dhcp-list-item-ip"><span class="info-label">IP Address:</span><span class="info-value">${dhcpMasked(ip, 'ip')}</span></div><div class="dhcp-list-item-hostname"><span class="info-label">Name:</span><span class="info-value">${dhcpMasked(hostname, 'hostname')}</span></div></div><div class="dhcp-list-item-badge"><span class="${badge}">${type === 'reservation' ? 'Pinned' : 'Lease'}</span></div></div><div class="dhcp-list-item-actions">${action}</div></div></div>`;
      }).join('');
      const stats = dhcpState.statistics || {};
      const reservationCount = Number(stats.reservations_count ?? stats.reservationsCount ?? reservations.length);
      const reservationTotal = Number(stats.reservations_total ?? stats.reservationsTotal ?? dhcpState.boundary.maxReservations ?? 0);
      const leaseCount = Number(stats.leases_count ?? stats.leasesCount ?? leases.length);
      const leaseTotal = Number(stats.leases_total ?? stats.leasesTotal ?? 0);
      dhcpSet('[data-dhcp-homeserver]', stats.homeserver_ip ?? stats.homeserverIp ?? '192.168.123.1');
      dhcpSet('[data-dhcp-reservations-count]', `${reservationCount}/${reservationTotal}`);
      dhcpSet('[data-dhcp-hosts-count]', reservationCount + leaseCount);
      dhcpSet('[data-dhcp-leases-count]', `${leaseCount}/${leaseTotal}`);
      const empty = document.querySelector('[data-dhcp-empty]'); if (empty) empty.hidden = rows.length !== 0;
      const loading = document.querySelector('[data-dhcp-loading]'); if (loading) loading.hidden = true;
    }
    async function dhcpJson(route, options = {}) {
      const response = await fetch(route, { cache: 'no-store', headers: dhcpHeaders(Boolean(options.body)), ...options });
      const body = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(body.error || body.firstMissingSignal || `DHCP request failed (${response.status})`);
      return body;
    }
    function dhcpBoundaryValue(payload, ...names) {
      for (const name of names) for (const source of [payload, payload?.boundary, payload?.poolBoundary, payload?.data]) if (source?.[name] !== undefined) return Number(source[name]);
      return undefined;
    }
    async function hydrateDhcp() {
      const tablet = document.querySelector('[data-dhcp-tablet]'); if (!tablet || !viewportFamilyAdmitted('dhcp')) return;
      const error = document.querySelector('[data-dhcp-error]'); if (error) error.hidden = true;
      const loading = document.querySelector('[data-dhcp-loading]'); if (loading) loading.hidden = false;
      try {
        const [leases, reservations, statistics, boundary] = await Promise.all([
          dhcpJson('/api/dhcp/leases'), dhcpJson('/api/dhcp/reservations'), dhcpJson('/api/dhcp/statistics'), dhcpJson('/api/dhcp/pool-boundary')
        ]);
        dhcpState.leases = dhcpRows(leases, 'leases');
        dhcpState.reservations = dhcpRows(reservations, 'reservations');
        dhcpState.statistics = statistics.statistics || statistics.data || statistics;
        dhcpState.boundary = boundary.boundary || boundary.poolBoundary || boundary.data || boundary;
        const slider = document.querySelector('[data-dhcp-boundary]');
        if (slider) {
          const current = dhcpBoundaryValue(boundary, 'maxReservations', 'max_reservations') ?? dhcpState.reservations.length;
          const minimum = dhcpBoundaryValue(boundary, 'minimum', 'minReservations', 'min_reservations') ?? dhcpState.reservations.length;
          const maximum = dhcpBoundaryValue(boundary, 'maximum', 'maxAllowed', 'max_allowed') ?? Number(dhcpState.statistics.reservations_total ?? dhcpState.statistics.reservationsTotal ?? 249);
          slider.min = String(minimum); slider.max = String(maximum); slider.value = String(Math.max(minimum, Math.min(maximum, current)));
          setDhcpBoundaryLabel(slider.value); setDhcpBoundaryRange(minimum, maximum);
        }
        renderDhcp();
      } catch (failure) {
        if (error) { error.hidden = false; error.textContent = `DHCP unavailable: ${failure.message}`; }
        if (loading) loading.hidden = true;
      }
    }
    async function dhcpMutate(method, route, body, successMessage) {
      try {
        await dhcpJson(route, { method, body: body === undefined ? undefined : JSON.stringify(body) });
        await hydrateDhcp();
        showCoronatioToast(successMessage, 'success');
        return true;
      } catch (failure) {
        showCoronatioToast(failure?.message || 'DHCP request failed', 'error');
        return false;
      }
    }
    const dhcpModal = document.querySelector('[data-dhcp-modal-backdrop]');
    const dhcpModalTitle = document.querySelector('[data-dhcp-modal-title]');
    const dhcpModalBody = document.querySelector('[data-dhcp-modal-body]');
    const dhcpModalConfirm = document.querySelector('[data-dhcp-modal-confirm]');
    function closeDhcpModal() { if (dhcpModal) { dhcpModal.classList.remove('open'); dhcpModal.setAttribute('aria-hidden', 'true'); } }
    function openDhcpModal(title, body, confirmLabel, onConfirm) {
      if (!dhcpModal || !dhcpModalTitle || !dhcpModalBody || !dhcpModalConfirm) return;
      dhcpModalTitle.textContent = title; dhcpModalBody.innerHTML = body; dhcpModalConfirm.textContent = confirmLabel;
      dhcpModalConfirm.onclick = async () => { const completed = await onConfirm(); if (completed) closeDhcpModal(); };
      dhcpModal.classList.add('open'); dhcpModal.setAttribute('aria-hidden', 'false');
      dhcpModalBody.querySelector('input')?.focus();
    }
    // Delegated DHCP bindings survive pane swaps (Upload cure); no direct node ownership.
    document.body.addEventListener('input', event => {
      const slider = event.target.closest?.('[data-dhcp-boundary]');
      if (slider) setDhcpBoundaryLabel(slider.value);
    });
    document.body.addEventListener('change', event => {
      const anonymize = event.target.closest?.('[data-dhcp-anonymize]');
      if (anonymize) { dhcpState.anonymized = anonymize.checked; renderDhcp(); }
    });
    document.body.addEventListener('submit', async event => {
      const form = event.target.closest?.('[data-dhcp-add-form]');
      if (!form) return;
      event.preventDefault(); const data = new FormData(form); const payload = { mac: data.get('mac'), hostname: data.get('hostname') || undefined };
      if (data.get('ip')) payload.ip = data.get('ip'); if (await dhcpMutate('POST', '/api/dhcp/reservations', payload, 'Reservation added')) form.reset();
    });
    document.addEventListener('click', event => {
      if (event.target.closest?.('[data-dhcp-modal-cancel]')) return closeDhcpModal();
      if (event.target === dhcpModal) return closeDhcpModal();
      if (event.target.closest?.('[data-dhcp-refresh]')) return hydrateDhcp();
      if (event.target.closest?.('[data-dhcp-boundary-save]')) { const slider = document.querySelector('[data-dhcp-boundary]'); if (slider) return dhcpMutate('POST', '/api/dhcp/pool-boundary', { maxReservations: Number(slider.value) }, 'Reservation boundary updated'); }
      const pin = event.target.closest?.('[data-dhcp-pin]');
      if (pin) return dhcpMutate('POST', '/api/dhcp/reservations', { mac: pin.dataset.dhcpPin, hostname: pin.dataset.dhcpHostname || undefined }, 'Reservation pinned');
      const remove = event.target.closest?.('[data-dhcp-remove]');
      if (remove) return openDhcpModal('Remove reservation', '<p class="dhcp-modal-message">Remove this reserved address?</p>', 'Remove', () => dhcpMutate('DELETE', '/api/dhcp/reservations/' + encodeURIComponent(remove.dataset.dhcpRemove), undefined, 'Reservation removed'));
      const edit = event.target.closest?.('[data-dhcp-edit]');
      if (edit) return openDhcpModal('Edit reserved address', `<label class="dhcp-modal-field" for="dhcp-edit-ip"><span>Reserved IP address</span><input id="dhcp-edit-ip" class="ui-input ui-input--medium" type="text" value="${escapeHtml(edit.dataset.dhcpIp || '')}"></label>`, 'Save', () => { const ip = document.querySelector('[data-dhcp-modal-body] #dhcp-edit-ip')?.value.trim(); return ip ? dhcpMutate('PUT', '/api/dhcp/reservations/' + encodeURIComponent(edit.dataset.dhcpEdit), { ip }, 'Reservation updated') : false; });
    }, true);


"####
}
