fn shell_unbound_client() -> &'static str {
    r####"    const dnsUiState = { resolver: null, resolverReady: false, identityReady: false, pickedMac: null, selectedOctet: null, calendarStart: null, calendarEnd: null, calendarPage: 0, nameDefault: '', moveConfirmation: '', mutationBusy: false, binding: null, resolverDirty: new Set(), planReady: false, planText: '', plannedIp: '', planSequence: 0, planTimer: null, namesSort: { key: 'address', dir: 1 } };
    function dnsCurrent(binding = dnsUiState.binding) { return Boolean(binding && dnsUiState.binding === binding && headerState.isAdmin && binding.guest === 'unbound' && window.scaffoldRegistry?.current(binding)); }
    function dnsIdentityCurrent(binding = dnsUiState.binding) { return dnsCurrent(binding) && identityState.binding === binding && ['roster','boundary','leases','reservations','dns'].every(key => identityState.ready[key] === true); }
    const dnsArray = value => Array.isArray(value) ? value : [];
    const dnsValue = (value, fallback = '—') => value === undefined || value === null || value === '' ? fallback : String(value);
    const dnsAddress = row => row?.address || row?.ip || row?.target || row?.value || row?.declared_reservation?.ip || row?.declared_reservation?.['ip-address'] || row?.observed_lease?.ip || '—';
    const dnsProviders = Object.freeze({ quad9: ['9.9.9.9', '149.112.112.112'], cloudflare: ['1.1.1.1', '1.0.0.1'], google: ['8.8.8.8', '8.8.4.4'] });
    const dnsBlocklistTimeoutMs = 300000;
    const dnsBlocklistPresets = Object.freeze([
      ['StevenBlack unified', 'https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts', 'A long-running community list of ad, tracking and malware sites. Safe default; rarely breaks anything.'],
      ['HaGeZi Multi normal', 'https://raw.githubusercontent.com/hagezi/dns-blocklists/main/wildcard/multi-onlydomains.txt', 'Blocks ads, trackers, telemetry and scam sites. Balanced: broader than StevenBlack without breaking common sites.'],
      ['OISD big', 'https://big.oisd.nl/domainswild2', 'A large combined list built from many other lists. Blocks the most; may occasionally block a site you want.']
    ]);
    function dnsBlocklistLabel(url) { const preset = dnsBlocklistPresets.find(([, presetUrl]) => presetUrl === url); if (preset) return { name: preset[0], detail: preset[2] }; let host = url; try { host = new URL(url).hostname || url; } catch (_) {} return { name: `Custom list (${host})`, detail: 'A list you added by its web address.' }; }
    function dnsIpKey(value) { const match = /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/.exec(String(value ?? '').trim()); return match ? match.slice(1).reduce((sum, octet) => sum * 256 + Number(octet), 0) : null; }
    function dnsCompareAddress(a, b) { const left = dnsIpKey(a), right = dnsIpKey(b); if (left !== null && right !== null) return left - right; if (left !== null) return -1; if (right !== null) return 1; const blankA = a === '—' || !a, blankB = b === '—' || !b; if (blankA !== blankB) return blankA ? 1 : -1; return String(a ?? '').localeCompare(String(b ?? '')); }
    function dnsEnvelopeNodes(value, depth = 0) { if (!value || typeof value !== 'object' || depth > 6) return []; if (Array.isArray(value)) return value.flatMap(item => dnsEnvelopeNodes(item, depth + 1)); const nodes = [value]; for (const key of ['payload', 'result', 'data', 'receipt', 'readback', 'caduceus']) nodes.push(...dnsEnvelopeNodes(value[key], depth + 1)); return nodes; }
    function dnsChordNodes(value, depth = 0) { if (!value || typeof value !== 'object' || depth > 20) return []; if (Array.isArray(value)) return value.flatMap(item => dnsChordNodes(item, depth + 1)); return [value, ...Object.values(value).flatMap(item => dnsChordNodes(item, depth + 1))]; }
    function dnsChordStage(value, stageId) { return dnsChordNodes(value).find(node => node.stage_id === stageId) || null; }
    function dnsChordFailure(value) { return dnsChordNodes(value).reverse().find(node => node.failedStage && node.firstMissingSignal && node.firstMissingSignal !== 'none') || null; }
    function dnsFindObject(value, predicate) { return dnsEnvelopeNodes(value).find(node => predicate(node)) || null; }
    function dnsRefusal(value) { for (const node of dnsEnvelopeNodes(value)) { if (node.ok === false || node.success === false) return node.firstMissingSignal || node.first_failing_boundary || node.first_missing_signal || node.error || 'dns-caduceus-refused'; } return null; }
    function dnsRequireAccepted(value) { const refusal = dnsRefusal(value); if (refusal) throw new Error(refusal); if (!dnsFindObject(value, node => node.ok === true || node.success === true || node.accepted === true)) throw new Error('dns-mutation-receipt-missing-success'); return value; }
    async function dnsJson(route, options = {}) { const response = await fetch(route, { cache: 'no-store', headers: options.body ? {'content-type':'application/json'} : {}, ...options }); const body = await response.json().catch(() => null); const refusal = dnsRefusal(body); if (!response.ok || refusal) throw new Error(refusal || body?.first_failing_boundary || body?.firstMissingSignal || body?.error || `Request failed (${response.status})`); return body; }
    function dnsSetState(message) { const binding=dnsUiState.binding; if(!dnsCurrent(binding))return; binding.root.querySelectorAll('[data-dns-state]').forEach(node => { node.textContent=message; }); }
    function dnsSetResolverControls(preserveReadySlots = false) { const status=dnsUiState.resolver,binding=dnsUiState.binding,root=binding?.root;
        if(!root)return;
        root.querySelectorAll('[data-dns-resolver-control], [data-dns-upstream-save]').forEach(node=>{if(!scaffoldReadProjectionAllowed(node,preserveReadySlots))return;let available=dnsCurrent(binding)&&dnsUiState.resolverReady;
          if(node.matches('[data-dns-adblock]'))available=available&&typeof status?.adblockEnabled==='boolean';
          else if(node.matches('[data-dns-blocklist-source-remove], [data-dns-blocklist-preset], [data-dns-blocklist-url], [data-dns-blocklist-add]'))available=available&&Array.isArray(status?.blocklistSources);
          else if(node.matches('[data-dns-dot], [data-dns-upstream-provider], [data-dns-upstream-custom-toggle], [data-dns-upstream-custom-addresses], [data-dns-upstream-save]'))available=available&&Array.isArray(status?.upstreams)&&typeof status?.dnsOverTls==='boolean';
          node.disabled=!available||dnsUiState.mutationBusy;
          });
        }
    function dnsSetIdentityControls(preserveReadySlots = false) { const binding=dnsUiState.binding;dnsUiState.identityReady=dnsIdentityCurrent(binding);const root=binding?.root;if(!root)return;root.querySelectorAll('[data-dns-add-name], [data-dns-device-pick], [data-dns-name-remove], [data-dns-add-save]').forEach(node=>{if(scaffoldReadProjectionAllowed(node,preserveReadySlots))node.disabled=!dnsUiState.identityReady||dnsUiState.mutationBusy;});dnsUpdateAddSave(); }
    function dnsSetMutationBusy(busy) { const binding=dnsUiState.binding;if(!dnsCurrent(binding))return;dnsUiState.mutationBusy=Boolean(busy);dnsSetResolverControls();dnsSetIdentityControls();renderDnsNames();renderDnsModalRoster();renderDnsModalRecords();if(dnsUiState.pickedMac)renderDnsIpCalendar(); }
    function dnsProvenanceHas(value, wanted) { const values = Array.isArray(value) ? value : [value]; return values.some(item => String(item || '').toLowerCase() === wanted); }
    function dnsOctet(value) { const match = String(value || '').match(/^192\.168\.123\.(\d{1,3})$/); if (!match) return null; const octet = Number(match[1]); return Number.isInteger(octet) && octet >= 2 && octet <= 254 ? octet : null; }
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
        if (existing) { if (!existing.mac && row.mac) existing.mac = row.mac; if (!existing.ptr && row.ptr) existing.ptr = row.ptr; if (!existing.where && row.where) existing.where = row.where; existing.owned ||= row.owned === true; existing.adoptable ||= row.adoptable === true; if (source === 'dns-owned-pair' && row.removable === true) { existing.provenance = row.provenance; existing.removable = true; if (row.ptr) existing.ptr = row.ptr; if (row.mac) existing.mac = row.mac; } return; }
        const provenance = row?.provenance ?? (source === 'alias' ? 'alias (read-only)' : source === 'roster' ? 'device projection' : 'observed');
        rows.set(key, { name, address, ptr: row?.ptr || '', mac: row?.mac || '', where: row?.where || '', owned: row?.owned === true || dnsProvenanceHas(provenance, 'owned'), adoptable: row?.adoptable === true, provenance, removable: source === 'dns-owned-pair' && row?.removable === true });
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
          add({ name, address, ptr: ptrRecord?.name || '', where: record?.where, owned: aOwned, adoptable: device?.adoptable === true, provenance: aOwned && ptrOwned ? 'owned A/PTR pair' : provenance, removable: aOwned && ptrOwned && Boolean(dnsNormalizeBareLabel(name).label) }, aOwned && ptrOwned ? 'dns-owned-pair' : 'dns-device');
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
      return [...rows.values()].sort((a, b) => dnsCompareAddress(a.address, b.address) || a.name.localeCompare(b.name));
    }
    function renderDnsNames(preserveReadySlots = false) {
      const target=dnsUiState.binding?.root?.querySelector('[data-dns-names]');
      if(!target||!dnsCurrent()||!scaffoldReadProjectionAllowed(target,preserveReadySlots))return;
      const deviceLabelFor=row=>{const targetName = /(?:^|\.)home\.arpa\.?$/i.test(row.address) && row.address.toLowerCase().replace(/\.$/, '') !== row.name.toLowerCase().replace(/\.$/, '') ? row.address : '';
        const device = !targetName && row.address !== '—' ? dnsArray(identityState?.roster).find(item => dnsAddress(item) === row.address) : null;
        const label = targetName ? `→ ${targetName}` : device ? dnsDeviceLabel(device) : '—';return label === 'Unknown device' ? '—' : label;};
      const sort=dnsUiState.namesSort,blank=value=>!value||value==='—';
      const sortValue=row=>sort.key==='device'?deviceLabelFor(row):sort.key==='name'?row.name:row.address;
      const rows=dnsNameRows().sort((a,b)=>{const va=sortValue(a),vb=sortValue(b);if(blank(va)!==blank(vb))return blank(va)?1:-1;
        const order=sort.key==='address'?dnsCompareAddress(va,vb):String(va).localeCompare(String(vb));
        return sort.dir*order||dnsCompareAddress(a.address,b.address)||a.name.localeCompare(b.name);});
      if(!rows.length&&!identityState.ready.roster&&!identityState.ready.dns)return;
      const heading=(key,label)=>{const active=sort.key===key,arrow=active?(sort.dir>0?' ▲':' ▼'):'';return `<button type="button" class="dns-sort-button" data-dns-names-sort="${key}" aria-sort="${active?(sort.dir>0?'ascending':'descending'):'none'}" title="Sort by ${label.toLowerCase()}">${label}${arrow}</button>`;};
      const html=rows.length?`<div class="dns-name-list"><div class="dns-name-row dns-name-heading">${heading('name','Name')}${heading('address','Address')}${heading('device','Device')}<span>Action</span></div>${rows.map(row=>{
        const key=`${row.name.toLowerCase()}-${row.address.toLowerCase()}`;
        const deviceLabel = deviceLabelFor(row);
        const provenance=Array.isArray(row.provenance)?row.provenance.join(', '):String(row.provenance||'observed');
        const marker=({'owned':'yours','alias':'alias','known-good':'known good','old-staff-file':'old staff file','hand-written':'hand-written'})[row.where] ?? (row.removable?'':(/legacy/i.test(provenance)?'legacy':'observed'));
        const action=row.adoptable?`<button type="button" class="ui-button ui-button--secondary ui-button--small" data-dns-name-adopt="${escapeHtml(row.name)}" ${!dnsUiState.identityReady||dnsUiState.mutationBusy?'disabled':''}>Adopt</button>`:row.removable?`<button type="button" class="ui-button ui-button--danger ui-button--small" data-dns-name-remove="${escapeHtml(row.name)}" data-dns-ip="${escapeHtml(row.address)}" ${!dnsUiState.identityReady||dnsUiState.mutationBusy?'disabled':''}>Remove</button>`:'';
        return `<div class="dns-name-row" data-scaffold-key="dns-name:${escapeHtml(key)}"><span>${escapeHtml(dnsValue(row.name))}${marker?` <small>(${escapeHtml(marker)})</small>`:''}</span><span>${escapeHtml(dnsValue(row.address))}</span><span>${escapeHtml(deviceLabel)}</span><span>${action}</span></div>`;
      }).join('')}</div>`:'<p>No LAN names are registered.</p>';
      morphLivePane(target,html);
    }
    function dnsDeviceLabel(device) { return String(device?.hostname || device?.name || device?.declared_reservation?.hostname || device?.observed_lease?.hostname || dnsArray(device?.dns_names)[0] || device?.mac || 'Unknown device'); }
    function dnsRosterDevice(mac = dnsUiState.pickedMac) { return dnsArray(identityState?.roster).find(device => String(device?.mac || '') === String(mac || '')) || null; }
    function renderDnsModalRoster() { const target=dnsUiState.binding?.root?.querySelector('[data-dns-device-roster]');
        if(!target||!dnsCurrent()||!identityState.ready.roster)return;
        const devices=[...dnsArray(identityState?.roster)].sort((a,b)=>dnsCompareAddress(dnsAddress(a),dnsAddress(b))||dnsDeviceLabel(a).localeCompare(dnsDeviceLabel(b)));
        const html=`<table class="identity-roster"><thead><tr><th>Device</th><th>Address</th><th>Names</th><th>Pick</th></tr></thead><tbody>${devices.map(device=>{const mac=String(device?.mac||''),selected=mac===dnsUiState.pickedMac;
          return `<tr data-device-mac="${escapeHtml(mac)}" data-scaffold-key="dns-device:${escapeHtml(mac)}"><td data-label="Device"><code>${escapeHtml(dnsValue(dnsDeviceLabel(device)))}</code><br><small>${escapeHtml(dnsValue(mac))}</small></td><td data-label="Address"><code>${escapeHtml(dnsValue(dnsAddress(device)))}</code></td><td data-label="Names">${escapeHtml(dnsArray(device?.dns_names).join(', ')||'—')}</td><td data-label="Pick"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-dns-device-pick="${escapeHtml(mac)}" aria-pressed="${selected}"${!dnsUiState.identityReady||dnsUiState.mutationBusy?' disabled':''}>${selected?'Selected':'Pick'}</button></td></tr>`;
          }).join('')||'<tr data-scaffold-key="dns-devices-empty"><td data-label="Devices" colspan="4">No devices reported.</td></tr>'}</tbody></table>`;
        morphLivePane(target,html);
        }
    function renderDnsModalRecords() { const target=dnsUiState.binding?.root?.querySelector('[data-dns-modal-records]');
        if(!target||!dnsCurrent()||!identityState.ready.roster&&!identityState.ready.dns)return;
        const rows=dnsNameRows();
        const html=`<h3>Records</h3><div class="dns-table-scroll" data-dns-records-scroll><table class="ui-table"><thead><tr><th>Name</th><th>Address</th><th>PTR</th><th>Provenance</th></tr></thead><tbody>${rows.map(row=>`<tr data-scaffold-key="dns-modal-record:${escapeHtml(`${row.name.toLowerCase()}-${row.address.toLowerCase()}`)}"><td data-label="Name"><code>${escapeHtml(dnsValue(row.name))}</code></td><td data-label="Address"><code>${escapeHtml(dnsValue(row.address))}</code></td><td data-label="PTR"><code>${escapeHtml(dnsValue(row.ptr))}</code></td><td data-label="Provenance">${escapeHtml(Array.isArray(row.provenance)?row.provenance.join(', '):dnsValue(row.provenance))}</td></tr>`).join('')||'<tr data-scaffold-key="dns-modal-records-empty"><td data-label="Records" colspan="4">No records reported.</td></tr>'}</tbody></table></div>`;
        morphLivePane(target,html);
        }
    function resolverReceipt(envelope) { const known=['adblockEnabled','upstreams','dnsOverTls','blocklistDomainCount','blocklistLastUpdate','blocklistCheckedAt','blocklistSources','unboundActive'];return dnsFindObject(envelope,node=>known.some(key=>Object.prototype.hasOwnProperty.call(node,key)))||null; }
    function renderDnsBlocklistSources(status) {
      const root=dnsUiState.binding?.root, list=root?.querySelector('[data-dns-blocklist-sources]'), presets=root?.querySelector('[data-dns-blocklist-presets]');
      if(!list||!presets||!Array.isArray(status?.blocklistSources))return;
      const sources=status.blocklistSources.filter(row=>typeof row?.url==='string');
      const disabled=!dnsUiState.resolverReady||dnsUiState.mutationBusy?' disabled':'';
      list.innerHTML=sources.map(row=>{const info=dnsBlocklistLabel(row.url),url=escapeHtml(row.url);return `<div class="dns-blocklist-source"><span class="dns-blocklist-source__text" title="${escapeHtml(info.detail)}"><strong>${escapeHtml(info.name)}</strong><small>${escapeHtml(info.detail)}</small><small class="dns-blocklist-source__url">${url}</small></span><button type="button" class="ui-button ui-button--danger ui-button--small" data-dns-blocklist-source-remove="${url}" data-dns-resolver-control${disabled}>Remove</button></div>`;}).join('');
      presets.innerHTML=dnsBlocklistPresets.filter(([,url])=>!sources.some(source=>source.url===url)).map(([name,url,detail])=>`<button type="button" class="ui-button ui-button--secondary ui-button--small" data-dns-blocklist-preset="${escapeHtml(url)}" title="${escapeHtml(`Add ${name}: ${detail}`)}" data-dns-resolver-control${disabled}>+ ${escapeHtml(name)}</button>`).join('');
    }
    function dnsUpstreamBase(address) { return String(address ?? '').trim().split(/[@#]/, 1)[0].trim(); }
    function dnsNormalizeUpstreams(addresses) { return [...new Set(dnsArray(addresses).map(dnsUpstreamBase).filter(Boolean))]; }
    function dnsResolverChoice(addresses) {
      const normalized=dnsNormalizeUpstreams(addresses);
      const preset=Object.keys(dnsProviders).find(name=>dnsProviders[name].length===normalized.length&&dnsProviders[name].every(address=>normalized.includes(address)));
      return {preset:preset||null, custom:preset?[]:normalized};
    }
    function renderResolverStatus(status) {
      const root=dnsUiState.binding?.root;
      if(!root||!dnsCurrent())return;
      if(!status){dnsSetResolverControls();return;}
      const adblock=root.querySelector('[data-dns-adblock]'),dotInput=root.querySelector('[data-dns-dot]'),customToggle=root.querySelector('[data-dns-upstream-custom-toggle]'),customArea=root.querySelector('[data-dns-upstream-custom]'),customInputs=[...root.querySelectorAll('[data-dns-upstream-custom-addresses]')],providerInputs=[...root.querySelectorAll('[data-dns-upstream-provider]')];
      const hasUpstreams=Array.isArray(status.upstreams),hasDot=typeof status.dnsOverTls==='boolean',choice=dnsResolverChoice(status.upstreams);
      if(adblock&&typeof status.adblockEnabled==='boolean'&&!dnsUiState.resolverDirty.has('adblock'))adblock.checked=status.adblockEnabled;
      if(!dnsUiState.resolverDirty.has('upstreams')){
        if(dotInput&&hasDot)dotInput.checked=status.dnsOverTls;
        if(hasUpstreams){
          providerInputs.forEach(input=>{input.checked=input.value===choice.preset;});
          if(customToggle)customToggle.checked=!choice.preset;
          if(customArea)customArea.hidden=Boolean(choice.preset);
          customInputs.forEach((input,index)=>{input.value=index===0?(choice.custom[0]||''):choice.custom.slice(1).join(', ');});
        }
      }
      const updated=status.blocklistLastUpdate===null?'Never':Number.isFinite(status.blocklistLastUpdate)?new Date(status.blocklistLastUpdate*1000).toLocaleString():'Unavailable';
      const count=Number.isFinite(status.blocklistDomainCount)?status.blocklistDomainCount.toLocaleString():'Unavailable';
      const blocklist=root.querySelector('[data-dns-blocklist-status]');
      if(blocklist)blocklist.textContent=`${count} domains blocked · updated ${updated}`;
      renderDnsBlocklistSources(status);
      const active=typeof status.unboundActive==='boolean'?(status.unboundActive?'running':'stopped'):'unavailable';
      const name=hasUpstreams?(choice.preset?({quad9:'Quad9',cloudflare:'Cloudflare',google:'Google'}[choice.preset]):'Custom'):'unavailable';
      const service=root.querySelector('[data-dns-service-status]');
      if(service)service.textContent=`Unbound ${active} · ${name} · encryption ${hasDot?(status.dnsOverTls?'on':'off'):'unavailable'}`;
      dnsSetResolverControls();
    }
    function dnsCalendarRange() { const boundary = identityState?.boundary?.boundary || identityState?.boundary || {}; const rawStart = boundary.start || boundary.first || boundary.pool_start || boundary.reservation_start; const start = String(rawStart) === '192.168.123.1' ? 2 : dnsOctet(rawStart); const end = dnsOctet(boundary.end || boundary.last || boundary.pool_end || boundary.reservation_end); return start !== null && end !== null && start <= end ? { start, end } : null; }
    function dnsHolderMac(row) { return String(row?.mac || row?.macAddress || row?.['hw-address'] || row?.hwAddress || row?.device_mac || row?.clientId || row?.client_id || '').toLowerCase(); }
    function dnsNormalizeHolderLabel(value) { return String(value).toLowerCase().replace(/\.$/, '').replace(/\.home\.arpa$/, ''); }
    function dnsHolderLabels(row) { return [row?.hostname, row?.hostName, row?.device_name, row?.name, row?.client_name].filter(Boolean).map(dnsNormalizeHolderLabel); }
    function dnsDeviceLabels(device) { return [device?.hostname, device?.name, device?.declared_reservation?.hostname, device?.observed_lease?.hostname, ...dnsArray(device?.dns_names)].filter(Boolean).map(dnsNormalizeHolderLabel); }
    function dnsHolderBelongsToDevice(row, device, address) { const heldMac = dnsHolderMac(row); const deviceMac = String(device?.mac || '').toLowerCase(); if (heldMac && deviceMac) return heldMac === deviceMac; const holderLabels = dnsHolderLabels(row); const deviceLabels = dnsDeviceLabels(device); if (holderLabels.length && deviceLabels.some(label => holderLabels.includes(label))) return true; return !heldMac && !holderLabels.length && dnsAddress(device) === address; }
    function dnsAddressHolders(address) { return [...dnsArray(identityState?.leases), ...dnsArray(identityState?.reservations)].filter(row => String(row?.ip || row?.['ip-address'] || row?.ipAddress || row?.address || '') === address); }
    function dnsIpAvailability(address, device) { const holders = dnsAddressHolders(address); const foreign = holders.filter(row => !dnsHolderBelongsToDevice(row, device, address)); const dnsHeld=dnsArray(dnsReadRoot(identityState?.dns).devices).some(device=>dnsArray(device?.a_records?.length?device.a_records:device?.a).some(row=>String(typeof row==='string'?row:row?.address||row?.ip||row?.target||'')===address)); const names = [...new Set(holders.map(row => String(row?.hostname || row?.hostName || row?.device_name || row?.name || row?.mac || row?.device_mac || 'DHCP holder')))]; if(dnsHeld)names.push('DNS name'); return { available: dnsOctet(address)!==null && !dnsHeld && foreign.length === 0, heldByPicked: !dnsHeld && holders.length > 0 && foreign.length === 0, holder: names.join(', ') }; }
    function dnsSelectedIp() { const device = dnsRosterDevice(); const octet = dnsUiState.selectedOctet; const range = dnsCalendarRange(); if (!device || !range || !Number.isInteger(octet) || octet < range.start || octet > range.end) return ''; const address = `192.168.123.${octet}`; return dnsIpAvailability(address, device).available ? address : ''; }
    function dnsUpdateAddSave() { const root=dnsUiState.binding?.root,save=root?.querySelector('[data-dns-add-save]');if(!save)return;const name=root.querySelector('[data-dns-new-name]')?.value.trim();save.disabled=!dnsIdentityCurrent()||!dnsUiState.pickedMac||!name||!dnsUiState.planReady||dnsUiState.mutationBusy; }
    function dnsSchedulePlan(delay=0) {
      clearTimeout(dnsUiState.planTimer);const sequence=++dnsUiState.planSequence,binding=dnsUiState.binding,device=dnsRosterDevice();
      dnsUiState.planReady=false;dnsUiState.plannedIp='';dnsUiState.planText=device?'Staff plan: loading…':'';renderDnsIpCalendar();
      if(!device||!dnsIdentityCurrent())return;
      const hostname=binding.root.querySelector('[data-dns-new-name]')?.value.trim()||dnsDeviceLabel(device),ip=dnsUiState.selectedOctet===null?null:dnsSelectedIp();
      if(dnsUiState.selectedOctet!==null&&!ip){dnsUiState.planText='Staff plan: selected address is unavailable';renderDnsIpCalendar();return;}
      dnsUiState.planTimer=setTimeout(async()=>{try{
        const response=await fetch('/api/v1/network/identity/device/plan',{method:'POST',cache:'no-store',credentials:'same-origin',headers:{'content-type':'application/json'},body:JSON.stringify({mac:device.mac,hostname,ip,move:false,settle:false})});
        const receipt=await response.json().catch(()=>null);if(sequence!==dnsUiState.planSequence||!dnsCurrent(binding))return;
        const plan=dnsFindObject(receipt,node=>node.plan&&typeof node.plan==='object')?.plan;
        const signal=plan?.signal&&plan.signal!=='none'?plan.signal:dnsRefusal(receipt);
        if(!response.ok||signal||!plan?.ip||!plan?.reason){dnsUiState.planText=`Staff plan: ${signal||'plan unavailable'}`;dnsUiState.planReady=ip===null&&Boolean(signal&&String(signal).endsWith('name-elsewhere')&&dnsNameRows().some(row=>dnsNormalizeBareLabel(row.name).label===dnsNormalizeBareLabel(hostname).label&&(row.where==='owned'||(!row.where&&row.owned))));}
        else{dnsUiState.planText=`Staff plan: ${plan.ip} (${plan.reason})`;dnsUiState.plannedIp=plan.ip;dnsUiState.planReady=true;}
      }catch(error){if(sequence!==dnsUiState.planSequence||!dnsCurrent(binding))return;dnsUiState.planText=`Staff plan: ${error.message||'unavailable'}`;dnsUiState.planReady=false;}
      renderDnsIpCalendar();},delay);
    }
    function renderDnsIpCalendar() {
      const root=dnsUiState.binding?.root,calendar=root?.querySelector('[data-dns-ip-calendar]'),grid=calendar?.querySelector('[data-dns-ip-grid]'),rangeLabel=calendar?.querySelector('[data-dns-ip-range]'),readout=calendar?.querySelector('[data-dns-ip-readout]');if(!calendar||!grid||!dnsCurrent())return;const device=dnsRosterDevice();calendar.hidden=!device;if(!device){morphLivePane(grid,'');if(readout)readout.textContent='Pick a device to load available addresses.';dnsUpdateAddSave();return;}
      const range=dnsCalendarRange();if(!range||!dnsUiState.identityReady){morphLivePane(grid,'');if(rangeLabel)rangeLabel.textContent='Unavailable';if(readout)readout.textContent='DHCP boundary, leases, and reservations must be available before selecting an address.';calendar.querySelector('[data-dns-ip-prev]')?.setAttribute('disabled','');calendar.querySelector('[data-dns-ip-next]')?.setAttribute('disabled','');dnsUpdateAddSave();return;}
      dnsUiState.calendarStart=range.start;
        dnsUiState.calendarEnd=range.end;
        const pageSize=32,pageCount=Math.ceil((range.end-range.start+1)/pageSize);
        dnsUiState.calendarPage=Math.max(0,Math.min(pageCount-1,dnsUiState.calendarPage));
        const first=range.start+dnsUiState.calendarPage*pageSize,last=Math.min(range.end,first+pageSize-1);
        if(rangeLabel)rangeLabel.textContent=`${first} - ${last}`;
        const previous=calendar.querySelector('[data-dns-ip-prev]'),next=calendar.querySelector('[data-dns-ip-next]');
        if(previous)previous.disabled=dnsUiState.calendarPage<=0||dnsUiState.mutationBusy;
        if(next)next.disabled=last>=range.end||dnsUiState.mutationBusy;

      const buttons=Array.from({length:Math.max(0,last-first+1)},(_,offset)=>{const octet=first+offset,address=`192.168.123.${octet}`,availability=dnsIpAvailability(address,device),selected=dnsUiState.selectedOctet===octet,disabled=!availability.available||dnsUiState.mutationBusy,label=availability.holder?`${address}, held by ${availability.holder}${availability.heldByPicked?' (picked device)':''}`:`Select ${address}`;
          return `<button type="button" class="ui-lan-ip-calendar__octet" data-dns-ip-octet="${octet}" data-scaffold-key="dns-octet:${octet}" ${disabled?'disabled':''} role="gridcell" aria-pressed="${selected}" aria-label="${escapeHtml(label)}"${availability.holder?` title="${escapeHtml(availability.heldByPicked?`${availability.holder} — picked device`:availability.holder)}"`:''}>${octet}</button>`;
          }).join('');
        morphLivePane(grid,buttons);
        const selected=dnsSelectedIp(),selectedStatus=selected?dnsIpAvailability(selected,device):null;
        if(readout)readout.textContent=dnsUiState.planText || (selected?`Selected address: ${selected}${selectedStatus?.heldByPicked?` (held by ${dnsDeviceLabel(device)})`:''}. Select it again to let staff choose.`:'No address selected; staff will choose.');
        dnsUpdateAddSave();

    }
    function selectDnsDevice(mac) { const device=dnsRosterDevice(mac);
        if(!device||!dnsIdentityCurrent()||dnsUiState.mutationBusy)return;
        dnsUiState.pickedMac=String(device.mac||''); dnsUiState.moveConfirmation='';
        dnsUiState.selectedOctet=null;
        dnsUiState.calendarPage=0;
        const root=dnsUiState.binding.root,node=root.querySelector('[data-dns-picked-device]');
        if(node)node.textContent=`Selected device: ${dnsDeviceLabel(device)} (${dnsValue(device.mac)}) · current address ${dnsValue(dnsAddress(device))}`;
        const nameInput=root.querySelector('[data-dns-new-name]'),suggestion=String(dnsArray(device.dns_names)[0]||device.declared_reservation?.hostname||device.observed_lease?.hostname||device.hostname||device.name||'').replace(/\.home\.arpa\.?$/i,'');
        if(nameInput&&(!nameInput.value.trim()||nameInput.value.trim()===dnsUiState.nameDefault)){nameInput.value=suggestion;
          dnsUiState.nameDefault=suggestion;
          }renderDnsModalRoster();
        renderDnsIpCalendar();
        dnsSchedulePlan();
        }
      function openDnsModal(opener) { if(!dnsIdentityCurrent()||dnsUiState.mutationBusy)return;const modal=dnsUiState.binding.root.querySelector('[data-dns-add-modal]');if(!modal)return;const result=modal.querySelector('[data-dns-add-result]');if(result)result.textContent='';renderDnsModalRoster();renderDnsModalRecords();if(dnsUiState.pickedMac)renderDnsIpCalendar();openCrownDialog(modal,opener);modal.querySelector('[data-dns-new-name]')?.focus(); }
    function closeDnsModal() { const root=dnsUiState.binding?.root,modal=root?.querySelector('[data-dns-add-modal]') || document.querySelector('[data-dns-add-modal]');
        if(modal)closeCrownDialog(modal);
        clearTimeout(dnsUiState.planTimer);dnsUiState.planSequence++;dnsUiState.planReady=false;dnsUiState.planText='';dnsUiState.pickedMac=null;
        dnsUiState.selectedOctet=null;
        dnsUiState.nameDefault=''; dnsUiState.moveConfirmation='';
        const input=root?.querySelector('[data-dns-new-name]');
        if(input)input.value='';
        const picked=root?.querySelector('[data-dns-picked-device]');
        if(picked)picked.textContent='Choose a device for this name.';
        const calendar=root?.querySelector('[data-dns-ip-calendar]');
        if(calendar)calendar.hidden=true;
        dnsUpdateAddSave();
        }
    function dnsNormalizeBareLabel(value) { const label = String(value ?? '').trim().toLowerCase().replace(/\.$/, '').replace(/\.home\.arpa$/, '');
        if (!label) return { label: '', error: 'Enter a hostname.' };
        if (label.includes('.')) return { label: '', error: 'Use a single hostname label; additional dots are not allowed.' };
        if (label.length > 63) return { label: '', error: 'Hostname must be 63 characters or fewer.' };
        if (label.match(/^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/)?.[0] !== label) return { label: '', error: /^[a-z0-9]/.test(label) && /[a-z0-9]$/.test(label) ? 'Hostname may contain only letters, numbers, and hyphens.' : 'Hostname must start and end with a letter or number; hyphens are allowed only inside the label.' };
        return { label, error: '' };
        }
    function dnsMutation(binding,key,route,options) {
      const slot=binding.root.querySelector('[data-scaffold-slot="dns-mutation"]')||binding.root.querySelector('[data-scaffold-slot="identity-dns"]');if(!slot)return Promise.resolve(false);
      let failure=null;
      return scaffoldRegistry.run(binding,`dns-write-${key}`,async ctx=>{try{if(!ctx.current()||!dnsCurrent(binding))return false;
          const response=await fetch(route,{cache:'no-store',credentials:'same-origin',signal:ctx.signal,headers:options.body?{'content-type':'application/json'}:{},...options});
          const receipt=await response.json().catch(()=>null);
          if(!response.ok||!receipt||dnsRefusal(receipt)){const failure=new Error(dnsRefusal(receipt)||receipt?.first_failing_boundary||receipt?.firstMissingSignal||receipt?.error||`Request failed (${response.status})`);failure.receipt=receipt;throw failure;}
          dnsRequireAccepted(receipt);
          if(!ctx.current()||!dnsCurrent(binding))return false;
          ctx.ready(slot);
          return receipt;
          }catch(error){failure=error;throw error;}},{slots:[slot],retry:false,timeoutMs:route.startsWith('/api/v1/network/dns/blocklist/')?dnsBlocklistTimeoutMs:100000,onFailure:({current,reason})=>{if(current()&&dnsCurrent(binding))dnsSetState(`DNS change unavailable: ${failure?.message||reason||'Read failed'}`);
            }}).then(result=>{if(!result&&failure)throw failure;return result;});

    }
    async function saveDnsName() {
      const binding=dnsUiState.binding,root=binding?.root,input=root?.querySelector('[data-dns-new-name]'),result=root?.querySelector('[data-dns-add-result]'),normalized=dnsNormalizeBareLabel(input?.value);
        if(!dnsIdentityCurrent()||dnsUiState.mutationBusy||!dnsUiState.planReady)return;
        if(normalized.error){if(result)result.textContent=normalized.error;
          return;
          }const hostname=normalized.label,device=dnsRosterDevice(),address=dnsSelectedIp();
        if(!device){if(result)result.textContent='Choose a device.';return;}
        if(dnsUiState.selectedOctet!==null&&!address){if(result)result.textContent='That address is no longer available.';return;}
        const matches=dnsNameRows().filter(row=>dnsNormalizeBareLabel(row.name).label===hostname);
        if(matches.some(row=>row.where==='known-good'||row.where==='hand-written')){dnsUiState.moveConfirmation='';if(result)result.textContent='This name already exists. Adopt it first if available.';return;}
        const plannedAddress=address||dnsUiState.plannedIp;
        const owned=matches.filter(row=>(row.where==='owned'||(!row.where&&row.owned)) && (!plannedAddress || row.address!==plannedAddress));
        const confirmation=JSON.stringify([hostname,device.mac,address,owned.map(row=>row.address).sort()]);
        const move=owned.length>0;
        if(move&&dnsUiState.moveConfirmation!==confirmation){dnsUiState.moveConfirmation=confirmation;if(result)result.textContent=`This name belongs to ${owned.map(row=>row.address).join(', ')}. Select Add name again to confirm moving it.`;return;}
        dnsUiState.moveConfirmation=''; if(result)result.textContent='Adding name…';
        dnsSetMutationBusy(true);

      try{const receipt=await dnsMutation(binding,'name-create','/api/v1/network/identity/device/add',{method:'POST',body:JSON.stringify({mac:dnsUiState.pickedMac,hostname,ip:address||null,move})});
          if(!receipt||!dnsCurrent(binding))return;
          const reserved=dnsChordStage(receipt,'reserve')?.receipt?.result?.['ip-address']||address||'address unavailable';
          showCoronatioToast(`${hostname}.home.arpa reserved at ${reserved}.`,'success');
          closeDnsModal();
          await hydrateDns({supersede:true});
          }catch(failure){if(dnsCurrent(binding)){const chord=dnsChordFailure(failure.receipt)||failure.receipt||{};const removed=dnsChordStage(failure.receipt,'remove')?.outcome==='stage-completed';const detail=`${chord.failedStage?`Stage ${chord.failedStage}: `:''}${chord.firstMissingSignal||failure.message}${removed?'. The reservation was removed.':''}`;if(result)result.textContent=`Unable to add name: ${detail}`;
            showCoronatioToast(failure?.message||'DNS request failed','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
    }
    async function adoptDnsName(button) {
      const binding=dnsUiState.binding,row=dnsNameRows().find(item=>item.name===button?.dataset?.dnsNameAdopt&&item.adoptable),hostname=dnsNormalizeBareLabel(row?.name).label;
      if(!dnsIdentityCurrent()||dnsUiState.mutationBusy||!hostname)return;
      dnsSetMutationBusy(true);
      try { const receipt=await dnsMutation(binding,'adopt','/api/v1/network/dns/device-name/create',{method:'POST',body:JSON.stringify({hostname,adopt:true})}); if(receipt&&dnsCurrent(binding)){showCoronatioToast('Name adopted.','success');await hydrateDns({supersede:true});} }
      catch(error){if(dnsCurrent(binding)){const messages={'dns-adopt-not-adoptable':'This name cannot be adopted.','dns-adopt-not-cleanly-squared':'This name cannot be cleanly adopted.','dns-adopt-source-moved':'The source changed; refresh and try again.'};showCoronatioToast(messages[error.message]||error.message,'error');await hydrateDns({supersede:true});}}
      finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);}
    }
    async function removeDnsName(button) {
      const binding=dnsUiState.binding,name=button?.dataset?.dnsNameRemove,address=button?.dataset?.dnsIp,row=dnsNameRows().find(item=>item.name===name&&item.address===address&&item.removable),normalized=dnsNormalizeBareLabel(row?.name);if(!dnsIdentityCurrent()||dnsUiState.mutationBusy||!row?.removable||!normalized.label)return;dnsSetMutationBusy(true);
      try{const receipt=await dnsMutation(binding,'name-remove','/api/v1/network/dns/device-name/remove',{method:'POST',body:JSON.stringify({hostname:normalized.label,ip:row.address})});
          if(!receipt||!dnsCurrent(binding))return;
          showCoronatioToast('Owned A and PTR records removed.','success');
          await hydrateDns({supersede:true});
          }catch(failure){if(dnsCurrent(binding)){showCoronatioToast(failure?.message||'Unable to remove DNS name','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
    }
    function dnsBlocklistOutcome(receipt) {
      const result=dnsFindObject(receipt,node=>typeof node.outcome==='string');
      if(result?.outcome==='held')dnsSetState(`kept the last list: ${result.firstFailingSource||'source'} unreachable`);
      else if(result?.outcome==='unchanged')dnsSetState('already current');
      else if(result?.outcome==='failed')dnsSetState(`DNS change unavailable: ${result.firstMissingSignal||'Blocklist update failed'}`);
    }
    async function changeDnsBlocklistSource(action,url) {
      const binding=dnsUiState.binding,input=binding?.root?.querySelector('[data-dns-blocklist-url]');
      if(!dnsCurrent(binding)||!dnsUiState.resolverReady||!Array.isArray(dnsUiState.resolver?.blocklistSources)||dnsUiState.mutationBusy)return;
      if(action==='add') {
        let parsed;
        try { if(!/^https:\/\//i.test(url))throw new Error(); parsed=new URL(url); if(parsed.protocol!=='https:')throw new Error(); }
        catch(_) { dnsSetState('Enter a valid HTTPS URL.'); return; }
        if(parsed.username||parsed.password) { dnsSetState('Use a URL without a username or password.'); return; }
      }
      dnsSetMutationBusy(true);
      try {
        const body={url};
        const receipt=await dnsMutation(binding,`blocklist-source-${action}`,`/api/v1/network/dns/blocklist/source/${action}`,{method:'POST',body:JSON.stringify(body)});
        if(!receipt||!dnsCurrent(binding))return;
        const result=dnsFindObject(receipt,node=>typeof node.outcome==='string');
        if(action==='add'&&['applied','held','unchanged'].includes(result?.outcome)&&input&&input.value.trim()===url)input.value='';
        await hydrateDns({supersede:true});
        dnsBlocklistOutcome(receipt);
      } catch(failure) { if(dnsCurrent(binding)) { await hydrateDns({supersede:true}); dnsSetState(`DNS change unavailable: ${failure?.message||'Blocklist change failed'}`); } }
      finally { if(dnsCurrent(binding))dnsSetMutationBusy(false); }
    }
    async function updateDnsBlocklist() {
      const binding=dnsUiState.binding;
        if(!dnsCurrent(binding)||!dnsUiState.resolverReady||dnsUiState.mutationBusy)return;
        dnsSetMutationBusy(true);
        try{const receipt=await dnsMutation(binding,'blocklist','/api/v1/network/dns/blocklist/update',{method:'POST'});
          if(receipt&&dnsCurrent(binding)){
            await hydrateDns({supersede:true});
            dnsBlocklistOutcome(receipt);
            }}catch(failure){if(dnsCurrent(binding)){showCoronatioToast(failure?.message||'Blocklist update failed','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
    }
    async function setDnsAdblock(enabled) {
      const binding=dnsUiState.binding;
        if(!dnsCurrent(binding)||!dnsUiState.resolverReady||typeof dnsUiState.resolver?.adblockEnabled!=='boolean'||dnsUiState.mutationBusy)return;
        dnsSetMutationBusy(true);
        try{const receipt=await dnsMutation(binding,'adblock','/api/v1/network/dns/adblock',{method:'POST',body:JSON.stringify({enabled})});
          if(receipt&&dnsCurrent(binding)){dnsUiState.resolverDirty.delete('adblock');
            showCoronatioToast(`Ad blocking ${enabled?'enabled':'disabled'}.`,'success');
            await hydrateDns({supersede:true});
            }}catch(failure){if(dnsCurrent(binding)){dnsUiState.resolverDirty.delete('adblock');showCoronatioToast(failure?.message||'Ad blocking change failed','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
    }
    async function saveDnsUpstream() {
      const binding=dnsUiState.binding;
        if(!dnsCurrent(binding)||!Array.isArray(dnsUiState.resolver?.upstreams)||typeof dnsUiState.resolver?.dnsOverTls!=='boolean'||dnsUiState.mutationBusy)return;
        const root=binding.root, selected=root.querySelector('[data-dns-upstream-provider]:checked'), customToggle=root.querySelector('[data-dns-upstream-custom-toggle]');
        const addresses=customToggle?.checked?[...new Set([...root.querySelectorAll('[data-dns-upstream-custom-addresses]')].flatMap(input=>input.value.split(',')).map(dnsUpstreamBase).filter(Boolean))]:[];
        if(!selected&&!customToggle?.checked){dnsSetState('Choose an upstream resolver.');return;}
        if(customToggle?.checked&&!addresses.length){dnsSetState('Enter at least one upstream address.');return;}
        if(addresses.length>8){dnsSetState('Caduceus accepts at most eight upstream addresses.');return;}
        const dot=Boolean(root.querySelector('[data-dns-dot]')?.checked),payload=selected?{preset:selected.value,custom:null,dot}:{preset:null,custom:addresses,dot};
        dnsSetMutationBusy(true);

      try{const receipt=await dnsMutation(binding,'upstream','/api/v1/network/dns/upstream',{method:'POST',body:JSON.stringify(payload)});
          if(receipt&&dnsCurrent(binding)){dnsUiState.resolverDirty.delete('upstreams');
            showCoronatioToast('Resolver settings saved.','success');
            await hydrateDns({supersede:true});
            }}catch(failure){if(dnsCurrent(binding)){showCoronatioToast(failure?.message||'Resolver change failed','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
    }
    function dnsReadResolver(binding,options={}) {
      const slot=binding.root.querySelector('[data-scaffold-slot="dns-resolver"]');
        if(!slot)return Promise.resolve(false);
        return scaffoldRegistry.run(binding,'dns-resolver',async ctx=>{if(!ctx.current()||!dnsCurrent(binding))return false;
          dnsUiState.resolverReady=false;
          dnsSetResolverControls(true);
          dnsSetState('Refreshing resolver status; last successful values remain visible.');
          const envelope=await ctx.fetchJson('/api/v1/network/dns/resolver/status');
          if(!ctx.current()||!dnsCurrent(binding))return false;
          if(!envelope||envelope.ok!==true||!envelope.payload||envelope.payload.ok!==true)throw new Error(dnsRefusal(envelope)||'Resolver response is incomplete');
          const source=resolverReceipt(envelope);
          if(!source)throw new Error('Resolver status fields are unavailable');
          const status=Object.fromEntries(['adblockEnabled','upstreams','dnsOverTls','blocklistDomainCount','blocklistLastUpdate','blocklistCheckedAt','blocklistSources','unboundActive'].filter(key=>Object.prototype.hasOwnProperty.call(source,key)).map(key=>[key,source[key]]));
          if(!ctx.current()||!dnsCurrent(binding))return false;
          dnsUiState.resolver=status;
          dnsUiState.resolverReady=true;
          renderResolverStatus(status);
          dnsSetResolverControls();
          ctx.ready(slot);
          dnsSetState('Resolver status is current.');
          return true;
          },{slots:[slot],supersede:options.supersede===true,onFailure:({current,reason})=>{if(!current()||!dnsCurrent(binding))return;
            dnsUiState.resolverReady=false;
            dnsSetResolverControls(true);
            dnsSetState(`Resolver status unavailable; showing the last successful values. ${reason||'Read failed'}`);
            }});

    }
    async function hydrateDns(options={}) {
      const binding=scaffoldRegistry.capture('unbound');
        if(!binding||!headerState.isAdmin||!viewportFamilyAdmitted('unbound'))return false;
        if(dnsUiState.binding!==binding){dnsUiState.binding=binding;
          dnsUiState.identityReady=false;
          dnsUiState.resolverReady=false;
          dnsUiState.mutationBusy=false;
          scaffoldRegistry.onRetire(binding,()=>{if(dnsUiState.binding!==binding)return;
            dnsUiState.identityReady=false;
            dnsUiState.resolverReady=false;
            dnsUiState.mutationBusy=false;
            dnsSetResolverControls();
            dnsSetIdentityControls();
            });
          }if(!dnsCurrent(binding))return false;
        dnsSetResolverControls(true);
        dnsSetIdentityControls(true);
        renderDnsNames(true);
        renderDnsModalRoster();
        renderDnsModalRecords();
        if(dnsUiState.pickedMac)renderDnsIpCalendar();
        const outcomes=await Promise.all([hydrateIdentity(options),dnsReadResolver(binding,options)]);
        return outcomes.every(Boolean);

    }
    document.addEventListener('coronatio:network-identity-started',event=>{
      const binding=event.detail?.binding;
      if(!dnsCurrent(binding)||dnsUiState.binding!==binding||identityState.binding!==binding)return;
      dnsUiState.identityReady=false;
      dnsSetIdentityControls(true);
      dnsSetState('Local identity is refreshing; last successful names and selection remain visible.');
    });
    document.addEventListener('coronatio:network-identity-updated',event=>{const binding=event.detail?.binding;
        if(!dnsCurrent(binding)||dnsUiState.binding!==binding||identityState.binding!==binding)return;
        dnsUiState.identityReady=dnsIdentityCurrent(binding);
        if(event.detail?.key==='roster'&&dnsUiState.pickedMac&&!dnsRosterDevice()){dnsUiState.pickedMac=null;
          dnsUiState.selectedOctet=null;
          }renderDnsNames();
        renderDnsModalRoster();
        renderDnsModalRecords();
        if(dnsUiState.pickedMac)renderDnsIpCalendar();
        dnsSetIdentityControls();
        dnsSetState(dnsUiState.identityReady?'Local names and address occupancy are current.':'Local names are loading; last successful records remain visible.');
        });

    document.addEventListener('coronatio:network-identity-failed',event=>{
      const binding=event.detail?.binding;
      if(!dnsCurrent(binding)||dnsUiState.binding!==binding)return;
      dnsUiState.identityReady=false;
      dnsSetIdentityControls(true);
      dnsSetState('Local identity is stale; last successful names and selection remain visible.');
    });
    document.addEventListener('click',async event=>{const binding=dnsUiState.binding,target=event.target instanceof Element?event.target:null;
        if(!target||!dnsCurrent(binding)||!binding.root.contains(target))return;
        const sortButton=target.closest('[data-dns-names-sort]');
        if(sortButton){const key=sortButton.dataset.dnsNamesSort,current=dnsUiState.namesSort;dnsUiState.namesSort={key,dir:current.key===key?-current.dir:1};renderDnsNames();return;}
        if(target.closest('[data-dns-add-name]'))return openDnsModal(target.closest('[data-dns-add-name]'));
        if(target.closest('[data-dns-add-close]'))return closeDnsModal();
        if(target.closest('[data-dns-add-save]'))return saveDnsName();
        const pick=target.closest('[data-dns-device-pick]');
        if(pick){if(!pick.disabled)return selectDnsDevice(pick.dataset.dnsDevicePick);
          return;
          }const octet=target.closest('[data-dns-ip-octet]');
        if(octet&&!octet.disabled){const selected=Number(octet.dataset.dnsIpOctet);dnsUiState.selectedOctet=dnsUiState.selectedOctet===selected?null:selected;dnsUiState.moveConfirmation='';
          renderDnsIpCalendar();
          dnsSchedulePlan();
          return;
          }if(target.closest('[data-dns-ip-prev]')){dnsUiState.calendarPage=Math.max(0,dnsUiState.calendarPage-1);
          renderDnsIpCalendar();
          return;
          }if(target.closest('[data-dns-ip-next]')){dnsUiState.calendarPage+=1;
          renderDnsIpCalendar();
          return;
          }const adopt=target.closest('[data-dns-name-adopt]');if(adopt){if(!adopt.disabled)return adoptDnsName(adopt);return;}const remove=target.closest('[data-dns-name-remove]');
        if(remove){if(!remove.disabled)return removeDnsName(remove);
          return;
          }const sourceRemove=target.closest('[data-dns-blocklist-source-remove]');
        if(sourceRemove&&!sourceRemove.disabled)return changeDnsBlocklistSource('remove',sourceRemove.dataset.dnsBlocklistSourceRemove);
        const preset=target.closest('[data-dns-blocklist-preset]');
        if(preset&&!preset.disabled)return changeDnsBlocklistSource('add',preset.dataset.dnsBlocklistPreset);
        if(target.closest('[data-dns-blocklist-add]'))return changeDnsBlocklistSource('add',binding.root.querySelector('[data-dns-blocklist-url]')?.value.trim()||'');
        if(target.closest('[data-dns-blocklist-update]'))return updateDnsBlocklist();
        if(target.closest('[data-dns-upstream-save]'))return saveDnsUpstream();
        if(target.closest('[data-dns-upstream-cancel]')){dnsUiState.resolverDirty.delete('upstreams');renderResolverStatus(dnsUiState.resolver);return;}
        if(target.closest('[data-dns-refresh]'))return hydrateDns({supersede:true});
        },true);

    document.addEventListener('change',event=>{const binding=dnsUiState.binding,target=event.target instanceof Element?event.target:null;
        if(!target||!dnsCurrent(binding)||!binding.root.contains(target))return;
        const adblock=target.closest('[data-dns-adblock]');
        if(adblock){dnsUiState.resolverDirty.add('adblock');
          void setDnsAdblock(adblock.checked);
          return;
          }if(target.closest('[data-dns-upstream-provider], [data-dns-upstream-custom-toggle], [data-dns-dot]')){dnsUiState.resolverDirty.add('upstreams');
          const toggle=binding.root.querySelector('[data-dns-upstream-custom-toggle]'),custom=binding.root.querySelector('[data-dns-upstream-custom]');
          if(custom&&toggle)custom.hidden=!toggle.checked;

          }},true);

    document.addEventListener('input',event=>{
      const binding=dnsUiState.binding,target=event.target instanceof Element?event.target:null;
      if(!target||!dnsCurrent(binding)||!binding.root.contains(target))return;
      if(target.matches('[data-dns-new-name]')){
        dnsUiState.moveConfirmation='';
        if(target.value.trim()!==dnsUiState.nameDefault)dnsUiState.nameDefault='';
        dnsSchedulePlan(300);
      }
      if(target.matches('[data-dns-upstream-custom-addresses]')){
        dnsUiState.resolverDirty.add('upstreams');

      }
    },true);
    document.addEventListener('keydown',event=>{const binding=dnsUiState.binding;
        if(event.key==='Enter'&&event.target.matches?.('[data-dns-blocklist-url]')&&dnsCurrent(binding)&&binding.root.contains(event.target)&&!event.target.disabled){event.preventDefault();void changeDnsBlocklistSource('add',event.target.value.trim());return;}
        const current=event.target.closest?.('[data-dns-ip-octet]');
        if(!current||!dnsCurrent(binding)||!binding.root.contains(current)||!['ArrowLeft','ArrowRight','ArrowUp','ArrowDown','Home','End'].includes(event.key))return;
        const buttons=[...current.closest('[data-dns-ip-grid]').querySelectorAll('[data-dns-ip-octet]:not(:disabled)')],index=buttons.indexOf(current);
        if(index<0)return;
        const offsets={ArrowLeft:-1,ArrowRight:1,ArrowUp:-8,ArrowDown:8},next=event.key==='Home'?0:event.key==='End'?buttons.length-1:Math.max(0,Math.min(buttons.length-1,index+(offsets[event.key]||0)));
        event.preventDefault();
        buttons[next]?.focus();
        });

    window.addEventListener('coronatio:privileged-state-retired',()=>{window.scaffoldRegistry?.retire('unbound','Admin role retired');
        dnsUiState.resolver=null;
        dnsUiState.resolverReady=false;
        dnsUiState.identityReady=false;
        dnsUiState.pickedMac=null;
        dnsUiState.selectedOctet=null;
        dnsUiState.calendarStart=null;
        dnsUiState.calendarEnd=null;
        dnsUiState.nameDefault='';
        dnsUiState.mutationBusy=false;
        dnsUiState.binding=null;
        dnsUiState.resolverDirty.clear();
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-names], [data-pane-panel="unbound"] [data-dns-device-roster], [data-pane-panel="unbound"] [data-dns-modal-records], [data-pane-panel="unbound"] [data-dns-service-status]').forEach(node=>node.replaceChildren());
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-new-name], [data-pane-panel="unbound"] [data-dns-upstream-custom-addresses]').forEach(node=>{node.value='';
          });
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-adblock], [data-pane-panel="unbound"] [data-dns-dot], [data-pane-panel="unbound"] [data-dns-upstream-provider], [data-pane-panel="unbound"] [data-dns-upstream-custom-toggle], [data-pane-panel="unbound"] [data-dns-resolver-control], [data-pane-panel="unbound"] [data-dns-add-name], [data-pane-panel="unbound"] [data-dns-add-save]').forEach(node=>{if('checked'in node)node.checked=false;
          node.disabled=true;
          });
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-add-modal]').forEach(node=>closeCrownDialog(node));
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-ip-calendar]').forEach(node=>{node.hidden=true;
          node.querySelector('[data-dns-ip-grid]')?.replaceChildren();
          });
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-state]').forEach(node=>{node.textContent='Admin access ended.';
          });
        });
    window.addEventListener('coronatio:native-pane-reset',event=>{const detail=event.detail||{},pane=detail.pane,binding=dnsUiState.binding;
        if(detail.id!=='unbound'||!(pane instanceof Element)||!pane.isConnected||document.querySelector('[data-pane-panel="unbound"]')!==pane)return;
        if(!binding||binding.guest!=='unbound'||window.scaffoldRegistry?.current(binding))return;
        dnsUiState.resolver=null;dnsUiState.resolverReady=false;dnsUiState.identityReady=false;dnsUiState.pickedMac=null;dnsUiState.selectedOctet=null;
        dnsUiState.calendarStart=null;dnsUiState.calendarEnd=null;dnsUiState.calendarPage=0;dnsUiState.nameDefault='';dnsUiState.mutationBusy=false;dnsUiState.binding=null;dnsUiState.resolverDirty.clear();
        });

"####
}
