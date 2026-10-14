fn shell_unbound_client() -> &'static str {
    r####"    const dnsUiState = { resolver: null, resolverReady: false, identityReady: false, pickedMac: null, selectedOctet: null, calendarStart: null, calendarEnd: null, calendarPage: 0, nameDefault: '', mutationBusy: false, binding: null, resolverDirty: new Set() };
    function dnsCurrent(binding = dnsUiState.binding) { return Boolean(binding && dnsUiState.binding === binding && headerState.isAdmin && binding.guest === 'unbound' && window.scaffoldRegistry?.current(binding)); }
    function dnsIdentityCurrent(binding = dnsUiState.binding) { return dnsCurrent(binding) && identityState.binding === binding && ['roster','boundary','leases','reservations','dns'].every(key => identityState.ready[key] === true); }
    const dnsArray = value => Array.isArray(value) ? value : [];
    const dnsValue = (value, fallback = '—') => value === undefined || value === null || value === '' ? fallback : String(value);
    const dnsAddress = row => row?.address || row?.ip || row?.target || row?.value || row?.declared_reservation?.ip || row?.declared_reservation?.['ip-address'] || row?.observed_lease?.ip || '—';
    const dnsProviders = Object.freeze({ quad9: ['9.9.9.9', '149.112.112.112'], cloudflare: ['1.1.1.1', '1.0.0.1'], google: ['8.8.8.8', '8.8.4.4'] });
    function dnsEnvelopeNodes(value, depth = 0) { if (!value || typeof value !== 'object' || depth > 6) return []; if (Array.isArray(value)) return value.flatMap(item => dnsEnvelopeNodes(item, depth + 1)); const nodes = [value]; for (const key of ['payload', 'result', 'data', 'receipt', 'readback', 'caduceus']) nodes.push(...dnsEnvelopeNodes(value[key], depth + 1)); return nodes; }
    function dnsFindObject(value, predicate) { return dnsEnvelopeNodes(value).find(node => predicate(node)) || null; }
    function dnsRefusal(value) { for (const node of dnsEnvelopeNodes(value)) { if (node.ok === false || node.success === false) return node.firstMissingSignal || node.first_failing_boundary || node.first_missing_signal || node.error || 'dns-caduceus-refused'; } return null; }
    function dnsRequireAccepted(value) { const refusal = dnsRefusal(value); if (refusal) throw new Error(refusal); if (!dnsFindObject(value, node => node.ok === true || node.success === true || node.accepted === true)) throw new Error('dns-mutation-receipt-missing-success'); return value; }
    async function dnsJson(route, options = {}) { const response = await fetch(route, { cache: 'no-store', headers: options.body ? {'content-type':'application/json'} : {}, ...options }); const body = await response.json().catch(() => null); const refusal = dnsRefusal(body); if (!response.ok || refusal) throw new Error(refusal || body?.first_failing_boundary || body?.firstMissingSignal || body?.error || `Request failed (${response.status})`); return body; }
    function dnsSetState(message) { const binding=dnsUiState.binding; if(!dnsCurrent(binding))return; binding.root.querySelectorAll('[data-dns-state]').forEach(node => { node.textContent=message; }); }
    function dnsSetResolverControls() { const status=dnsUiState.resolver,binding=dnsUiState.binding,root=binding?.root;
        if(!root)return;
        root.querySelectorAll('[data-dns-resolver-control], [data-dns-upstream-save]').forEach(node=>{let available=dnsCurrent(binding)&&dnsUiState.resolverReady;
          if(node.matches('[data-dns-adblock]'))available=available&&typeof status?.adblockEnabled==='boolean';
          else if(node.matches('[data-dns-dot], [data-dns-upstream-provider], [data-dns-upstream-custom-toggle], [data-dns-upstream-custom-addresses], [data-dns-upstream-save]'))available=available&&Array.isArray(status?.upstreams)&&typeof status?.dnsOverTls==='boolean';
          node.disabled=!available||dnsUiState.mutationBusy;
          });
        }
    function dnsSetIdentityControls() { const binding=dnsUiState.binding;dnsUiState.identityReady=dnsIdentityCurrent(binding);const root=binding?.root;if(!root)return;root.querySelectorAll('[data-dns-add-name], [data-dns-device-pick], [data-dns-name-remove], [data-dns-add-save]').forEach(node=>{node.disabled=!dnsUiState.identityReady||dnsUiState.mutationBusy;});dnsUpdateAddSave(); }
    function dnsSetMutationBusy(busy) { const binding=dnsUiState.binding;if(!dnsCurrent(binding))return;dnsUiState.mutationBusy=Boolean(busy);dnsSetResolverControls();dnsSetIdentityControls();renderDnsNames();renderDnsModalRoster();renderDnsModalRecords();if(dnsUiState.pickedMac)renderDnsIpCalendar(); }
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
    function renderDnsNames() { const target=dnsUiState.binding?.root?.querySelector('[data-dns-names]');
        if(!target||!dnsCurrent())return;
        const rows=dnsNameRows();
        if(!rows.length&&!identityState.ready.roster&&!identityState.ready.dns)return;
        const html=rows.length?`<table class="ui-table dns-name-table"><thead><tr><th>Name</th><th>Address</th><th>PTR</th><th>Provenance</th><th>Action</th></tr></thead><tbody>${rows.map(row=>{const key=`${row.name.toLowerCase()}-${row.address.toLowerCase()}`;
          return `<tr data-scaffold-key="dns-name:${escapeHtml(key)}"><td><code>${escapeHtml(dnsValue(row.name))}</code></td><td><code>${escapeHtml(dnsValue(row.address))}</code></td><td><code>${escapeHtml(dnsValue(row.ptr))}</code></td><td>${escapeHtml(Array.isArray(row.provenance)?row.provenance.join(', '):dnsValue(row.provenance))}</td><td>${row.removable?`<button type="button" class="ui-button ui-button--danger ui-button--small" data-dns-name-remove="${escapeHtml(row.name)}" data-dns-ip="${escapeHtml(row.address)}" aria-label="Remove owned A and PTR records for ${escapeHtml(row.name)}"${!dnsUiState.identityReady||dnsUiState.mutationBusy?' disabled':''}>Remove</button>`:'—'}</td></tr>`;
          }).join('')}</tbody></table>`:'<p>No LAN names are registered.</p>';
        morphLivePane(target,html);
        }
    function dnsDeviceLabel(device) { return String(device?.hostname || device?.name || device?.declared_reservation?.hostname || device?.observed_lease?.hostname || dnsArray(device?.dns_names)[0] || device?.mac || 'Unknown device'); }
    function dnsRosterDevice(mac = dnsUiState.pickedMac) { return dnsArray(identityState?.roster).find(device => String(device?.mac || '') === String(mac || '')) || null; }
    function renderDnsModalRoster() { const target=dnsUiState.binding?.root?.querySelector('[data-dns-device-roster]');
        if(!target||!dnsCurrent()||!identityState.ready.roster)return;
        const devices=dnsArray(identityState?.roster);
        const html=`<table class="identity-roster"><thead><tr><th>Device</th><th>Address</th><th>Names</th><th>Pick</th></tr></thead><tbody>${devices.map(device=>{const mac=String(device?.mac||''),selected=mac===dnsUiState.pickedMac;
          return `<tr data-device-mac="${escapeHtml(mac)}" data-scaffold-key="dns-device:${escapeHtml(mac)}"><td><code>${escapeHtml(dnsValue(dnsDeviceLabel(device)))}</code><br><small>${escapeHtml(dnsValue(mac))}</small></td><td><code>${escapeHtml(dnsValue(dnsAddress(device)))}</code></td><td>${escapeHtml(dnsArray(device?.dns_names).join(', ')||'—')}</td><td><button type="button" class="ui-button ui-button--secondary ui-button--small" data-dns-device-pick="${escapeHtml(mac)}" aria-pressed="${selected}"${!dnsUiState.identityReady||dnsUiState.mutationBusy?' disabled':''}>${selected?'Selected':'Pick'}</button></td></tr>`;
          }).join('')||'<tr data-scaffold-key="dns-devices-empty"><td colspan="4">No devices reported.</td></tr>'}</tbody></table>`;
        morphLivePane(target,html);
        }
    function renderDnsModalRecords() { const target=dnsUiState.binding?.root?.querySelector('[data-dns-modal-records]');
        if(!target||!dnsCurrent()||!identityState.ready.roster&&!identityState.ready.dns)return;
        const rows=dnsNameRows();
        const html=`<h3>Records</h3><table class="ui-table"><thead><tr><th>Name</th><th>Address</th><th>PTR</th><th>Provenance</th></tr></thead><tbody>${rows.map(row=>`<tr data-scaffold-key="dns-modal-record:${escapeHtml(`${row.name.toLowerCase()}-${row.address.toLowerCase()}`)}"><td><code>${escapeHtml(dnsValue(row.name))}</code></td><td><code>${escapeHtml(dnsValue(row.address))}</code></td><td><code>${escapeHtml(dnsValue(row.ptr))}</code></td><td>${escapeHtml(Array.isArray(row.provenance)?row.provenance.join(', '):dnsValue(row.provenance))}</td></tr>`).join('')||'<tr data-scaffold-key="dns-modal-records-empty"><td colspan="4">No records reported.</td></tr>'}</tbody></table>`;
        morphLivePane(target,html);
        }
    function resolverReceipt(envelope) { const known=['adblockEnabled','upstreams','dnsOverTls','blocklistDomainCount','blocklistLastUpdate','unboundActive'];return dnsFindObject(envelope,node=>known.some(key=>Object.prototype.hasOwnProperty.call(node,key)))||null; }
    function dnsUpstreamBase(address) { return String(address ?? '').trim().split(/[@#]/, 1)[0].trim(); }
    function dnsNormalizeUpstreams(addresses) { return [...new Set(dnsArray(addresses).map(dnsUpstreamBase).filter(Boolean))]; }
    function dnsSelectedUpstreamAddresses() { const root=dnsUiState.binding?.root;if(!root)return [];const selected=[...root.querySelectorAll('[data-dns-upstream-provider]:checked')].flatMap(input=>dnsProviders[input.value]||[]),customToggle=root.querySelector('[data-dns-upstream-custom-toggle]'),custom=customToggle?.checked?(root.querySelector('[data-dns-upstream-custom-addresses]')?.value||'').split(',').map(dnsUpstreamBase).filter(Boolean):[];return [...new Set([...selected,...custom])]; }
    function renderDnsUpstreamReadout() { const root=dnsUiState.binding?.root,target=root?.querySelector('[data-dns-upstream-readout]');if(!target||!dnsCurrent())return;const dot=Boolean(root.querySelector('[data-dns-dot]')?.checked),addresses=dnsSelectedUpstreamAddresses().map(address=>dot?`${address}@853`:address);target.textContent=`Upstreams: [${addresses.join(', ')}] · DoT: ${dot?'on':'off'}`; }
    function renderResolverStatus(status) {
      const root=dnsUiState.binding?.root;
        if(!root||!dnsCurrent())return;
        const target=root.querySelector('[data-dns-resolver-status]'),adblock=root.querySelector('[data-dns-adblock]'),dotInput=root.querySelector('[data-dns-dot]'),customToggle=root.querySelector('[data-dns-upstream-custom-toggle]'),customLabel=root.querySelector('[data-dns-upstream-custom]'),customInput=root.querySelector('[data-dns-upstream-custom-addresses]'),readout=root.querySelector('[data-dns-upstream-readout]'),providerInputs=[...root.querySelectorAll('[data-dns-upstream-provider]')];

      if(!status){if(target&&!dnsUiState.resolver)target.textContent='Resolver status not loaded.';if(readout&&!dnsUiState.resolver)readout.textContent='Resolver status unavailable.';dnsSetResolverControls();return;}
      const hasUpstreams=Array.isArray(status.upstreams),hasDot=typeof status.dnsOverTls==='boolean',upstreams=hasUpstreams?status.upstreams.map(String):[],normalized=dnsNormalizeUpstreams(upstreams);
      if(adblock&&typeof status.adblockEnabled==='boolean'&&!dnsUiState.resolverDirty.has('adblock'))adblock.checked=status.adblockEnabled;if(dotInput&&hasDot&&!dnsUiState.resolverDirty.has('upstreams'))dotInput.checked=status.dnsOverTls;
      if(hasUpstreams&&!dnsUiState.resolverDirty.has('upstreams')){const selectedProviders=[];
          providerInputs.forEach(input=>{const addresses=dnsProviders[input.value]||[],selected=addresses.length>0&&addresses.every(address=>normalized.includes(address));
            input.checked=selected;
            if(selected)selectedProviders.push(input.value);
            });
          const known=new Set(selectedProviders.flatMap(name=>dnsProviders[name]||[])),custom=normalized.filter(address=>!known.has(address));
          if(customToggle)customToggle.checked=custom.length>0;
          if(customLabel)customLabel.hidden=custom.length===0;
          if(customInput)customInput.value=custom.join(', ');
          }
      if(readout){if(hasUpstreams&&hasDot)renderDnsUpstreamReadout();else readout.textContent=`Upstreams: ${hasUpstreams?normalized.join(', ')||'None':'unavailable'} · DoT: ${hasDot?(status.dnsOverTls?'on':'off'):'unavailable'}`;}
      const updated=status.blocklistLastUpdate===null?'Never':Number.isFinite(status.blocklistLastUpdate)?new Date(status.blocklistLastUpdate*1000).toLocaleString():'Unavailable',count=Number.isFinite(status.blocklistDomainCount)?status.blocklistDomainCount.toLocaleString():'Unavailable',active=typeof status.unboundActive==='boolean'?(status.unboundActive?'Active':'Inactive'):'Unavailable';
        if(target){const values=[['Upstreams',hasUpstreams?upstreams.join(', ')||'None':'Unavailable'],['Blocklist domains',count],['Last update',updated],['Unbound service',active]];
          morphLivePane(target,values.map(([label,value])=>`<div class="dns-info-row" data-scaffold-key="resolver:${label}"><span>${escapeHtml(label)}</span><strong>${escapeHtml(dnsValue(value))}</strong></div>`).join(''));
          }dnsSetResolverControls();

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
    function dnsUpdateAddSave() { const root=dnsUiState.binding?.root,save=root?.querySelector('[data-dns-add-save]');if(!save)return;const name=root.querySelector('[data-dns-new-name]')?.value.trim();save.disabled=!dnsIdentityCurrent()||!dnsUiState.pickedMac||!dnsSelectedIp()||!name||dnsUiState.mutationBusy; }
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
        if(readout)readout.textContent=selected?`Selected address: ${selected}${selectedStatus?.heldByPicked?` (held by ${dnsDeviceLabel(device)})`:''}`:'Select an available address. Addresses held by other devices are unavailable.';
        dnsUpdateAddSave();

    }
    function selectDnsDevice(mac) { const device=dnsRosterDevice(mac);
        if(!device||!dnsIdentityCurrent()||dnsUiState.mutationBusy)return;
        dnsUiState.pickedMac=String(device.mac||'');
        const range=dnsCalendarRange(),ownOctet=dnsOctet(dnsAddress(device));
        let selected=ownOctet!==null&&range&&ownOctet>=range.start&&ownOctet<=range.end&&dnsIpAvailability(`192.168.123.${ownOctet}`,device).available?ownOctet:null;
        if(selected===null&&range)for(let octet=range.start;
        octet<=range.end;
        octet+=1)if(dnsIpAvailability(`192.168.123.${octet}`,device).available){selected=octet;
          break;
          }dnsUiState.selectedOctet=selected;
        dnsUiState.calendarPage=selected!==null&&range?Math.floor((selected-range.start)/32):0;
        const root=dnsUiState.binding.root,node=root.querySelector('[data-dns-picked-device]');
        if(node)node.textContent=`Selected device: ${dnsDeviceLabel(device)} (${dnsValue(device.mac)}) · current address ${dnsValue(dnsAddress(device))}`;
        const nameInput=root.querySelector('[data-dns-new-name]'),suggestion=String(dnsArray(device.dns_names)[0]||device.declared_reservation?.hostname||device.observed_lease?.hostname||device.hostname||device.name||'').replace(/\.home\.arpa\.?$/i,'');
        if(nameInput&&(!nameInput.value.trim()||nameInput.value.trim()===dnsUiState.nameDefault)){nameInput.value=suggestion;
          dnsUiState.nameDefault=suggestion;
          }renderDnsModalRoster();
        renderDnsIpCalendar();
        dnsUpdateAddSave();
        }
      function openDnsModal() { if(!dnsIdentityCurrent()||dnsUiState.mutationBusy)return;const modal=dnsUiState.binding.root.querySelector('[data-dns-add-modal]');if(!modal)return;modal.hidden=false;renderDnsModalRoster();renderDnsModalRecords();if(dnsUiState.pickedMac)renderDnsIpCalendar();dnsUiState.binding.root.querySelector('[data-dns-new-name]')?.focus(); }
    function closeDnsModal() { const root=dnsUiState.binding?.root,modal=root?.querySelector('[data-dns-add-modal]');
        if(modal)modal.hidden=true;
        dnsUiState.pickedMac=null;
        dnsUiState.selectedOctet=null;
        dnsUiState.nameDefault='';
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
      return scaffoldRegistry.run(binding,`dns-write-${key}`,async ctx=>{if(!ctx.current()||!dnsCurrent(binding))return false;
          const response=await fetch(route,{cache:'no-store',credentials:'same-origin',signal:ctx.signal,headers:options.body?{'content-type':'application/json'}:{},...options});
          const receipt=await response.json().catch(()=>null);
          if(!response.ok||!receipt)throw new Error(receipt?.first_failing_boundary||receipt?.firstMissingSignal||receipt?.error||`Request failed (${response.status})`);
          dnsRequireAccepted(receipt);
          if(!ctx.current()||!dnsCurrent(binding))return false;
          ctx.ready(slot);
          return receipt;
          },{slots:[slot],retry:false,timeoutMs:15000,onFailure:({current,reason})=>{if(current()&&dnsCurrent(binding))dnsSetState(`DNS change unavailable: ${reason||'Read failed'}`);
            }});

    }
    async function saveDnsName() {
      const binding=dnsUiState.binding,root=binding?.root,input=root?.querySelector('[data-dns-new-name]'),result=root?.querySelector('[data-dns-add-result]'),normalized=dnsNormalizeBareLabel(input?.value);
        if(!dnsIdentityCurrent()||dnsUiState.mutationBusy)return;
        if(normalized.error){if(result)result.textContent=normalized.error;
          return;
          }const hostname=normalized.label,device=dnsRosterDevice(),address=dnsSelectedIp();
        if(!device||!address){if(result)result.textContent='Choose a device and available DHCP address.';
          return;
          }if(result)result.textContent='Adding name…';
        dnsSetMutationBusy(true);

      try{const receipt=await dnsMutation(binding,'name-create','/api/v1/network/dns/device-name/create',{method:'POST',body:JSON.stringify({hostname,ip:address})});
          if(!receipt||!dnsCurrent(binding))return;
          showCoronatioToast('LAN name added.','success');
          closeDnsModal();
          await hydrateDns({supersede:true});
          }catch(failure){if(dnsCurrent(binding)){if(result)result.textContent=`Unable to add name: ${failure.message}`;
            showCoronatioToast(failure?.message||'DNS request failed','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
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
    async function updateDnsBlocklist() {
      const binding=dnsUiState.binding;
        if(!dnsCurrent(binding)||!dnsUiState.resolverReady||dnsUiState.mutationBusy)return;
        dnsSetMutationBusy(true);
        try{const receipt=await dnsMutation(binding,'blocklist','/api/v1/network/dns/blocklist/update',{method:'POST'});
          if(receipt&&dnsCurrent(binding)){showCoronatioToast('Blocklist update accepted.','success');
            await hydrateDns({supersede:true});
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
            }}catch(failure){if(dnsCurrent(binding)){showCoronatioToast(failure?.message||'Ad blocking change failed','error');
            await hydrateDns({supersede:true});
            }}finally{if(dnsCurrent(binding))dnsSetMutationBusy(false);
          }
    }
    async function saveDnsUpstream() {
      const binding=dnsUiState.binding;
        if(!dnsCurrent(binding)||!Array.isArray(dnsUiState.resolver?.upstreams)||typeof dnsUiState.resolver?.dnsOverTls!=='boolean'||dnsUiState.mutationBusy)return;
        const root=binding.root,
          providers=[...new Set([...root.querySelectorAll('[data-dns-upstream-provider]:checked')].map(input=>input.value).filter(name=>dnsProviders[name]))],
          customToggle=root.querySelector('[data-dns-upstream-custom-toggle]'),
          typedCustom=customToggle?.checked?(root.querySelector('[data-dns-upstream-custom-addresses]')?.value||'').split(',').map(dnsUpstreamBase).filter(Boolean):[],
          providerAddresses=[...new Set(providers.flatMap(name=>dnsProviders[name]))],
          custom=[...new Set(typedCustom)].filter(address=>!providerAddresses.includes(address)),
          addresses=[...new Set([...providerAddresses,...custom])];
        if(!addresses.length){dnsSetState('Choose at least one upstream resolver.');
          return;
          }if(addresses.length>8){dnsSetState('Caduceus accepts at most eight upstream addresses.');
          return;
          }const dot=Boolean(root.querySelector('[data-dns-dot]')?.checked),payload=providers.length===1&&custom.length===0?{preset:providers[0],custom:null,dot}:{preset:null,custom:addresses,dot};
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
          dnsSetResolverControls();
          dnsSetState('Refreshing resolver status; last successful values remain visible.');
          const envelope=await ctx.fetchJson('/api/v1/network/dns/resolver/status');
          if(!ctx.current()||!dnsCurrent(binding))return false;
          if(!envelope||envelope.ok!==true||!envelope.payload||envelope.payload.ok!==true)throw new Error(dnsRefusal(envelope)||'Resolver response is incomplete');
          const source=resolverReceipt(envelope);
          if(!source)throw new Error('Resolver status fields are unavailable');
          const status=Object.fromEntries(['adblockEnabled','upstreams','dnsOverTls','blocklistDomainCount','blocklistLastUpdate','unboundActive'].filter(key=>Object.prototype.hasOwnProperty.call(source,key)).map(key=>[key,source[key]]));
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
            dnsSetResolverControls();
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
            binding.root?.querySelectorAll('[data-scaffold-slot]').forEach(slot=>{if(slot.dataset.scaffoldUpdatedAt)slot.dataset.scaffoldStale='true';
              slot.setAttribute('aria-busy','false');
              });
            dnsSetResolverControls();
            dnsSetIdentityControls();
            });
          }if(!dnsCurrent(binding))return false;
        dnsSetResolverControls();
        dnsSetIdentityControls();
        renderDnsNames();
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
      dnsSetIdentityControls();
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
      dnsSetIdentityControls();
      dnsSetState('Local identity is stale; last successful names and selection remain visible.');
    });
    document.addEventListener('click',async event=>{const binding=dnsUiState.binding,target=event.target instanceof Element?event.target:null;
        if(!target||!dnsCurrent(binding)||!binding.root.contains(target))return;
        if(target.closest('[data-dns-add-name]'))return openDnsModal();
        if(target.closest('[data-dns-add-close]'))return closeDnsModal();
        if(target.closest('[data-dns-add-save]'))return saveDnsName();
        const pick=target.closest('[data-dns-device-pick]');
        if(pick){if(!pick.disabled)return selectDnsDevice(pick.dataset.dnsDevicePick);
          return;
          }const octet=target.closest('[data-dns-ip-octet]');
        if(octet&&!octet.disabled){dnsUiState.selectedOctet=Number(octet.dataset.dnsIpOctet);
          renderDnsIpCalendar();
          return;
          }if(target.closest('[data-dns-ip-prev]')){dnsUiState.calendarPage=Math.max(0,dnsUiState.calendarPage-1);
          renderDnsIpCalendar();
          return;
          }if(target.closest('[data-dns-ip-next]')){dnsUiState.calendarPage+=1;
          renderDnsIpCalendar();
          return;
          }const remove=target.closest('[data-dns-name-remove]');
        if(remove){if(!remove.disabled)return removeDnsName(remove);
          return;
          }if(target.closest('[data-dns-blocklist-update]'))return updateDnsBlocklist();
        if(target.closest('[data-dns-upstream-save]'))return saveDnsUpstream();
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
          renderDnsUpstreamReadout();
          }},true);

    document.addEventListener('input',event=>{
      const binding=dnsUiState.binding,target=event.target instanceof Element?event.target:null;
      if(!target||!dnsCurrent(binding)||!binding.root.contains(target))return;
      if(target.matches('[data-dns-new-name]')){
        if(target.value.trim()!==dnsUiState.nameDefault)dnsUiState.nameDefault='';
        dnsUpdateAddSave();
      }
      if(target.matches('[data-dns-upstream-custom-addresses]')){
        dnsUiState.resolverDirty.add('upstreams');
        renderDnsUpstreamReadout();
      }
    },true);
    document.addEventListener('keydown',event=>{const binding=dnsUiState.binding,current=event.target.closest?.('[data-dns-ip-octet]');
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
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-names], [data-pane-panel="unbound"] [data-dns-device-roster], [data-pane-panel="unbound"] [data-dns-modal-records], [data-pane-panel="unbound"] [data-dns-resolver-status]').forEach(node=>node.replaceChildren());
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-new-name], [data-pane-panel="unbound"] [data-dns-upstream-custom-addresses]').forEach(node=>{node.value='';
          });
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-adblock], [data-pane-panel="unbound"] [data-dns-dot], [data-pane-panel="unbound"] [data-dns-upstream-provider], [data-pane-panel="unbound"] [data-dns-upstream-custom-toggle], [data-pane-panel="unbound"] [data-dns-resolver-control], [data-pane-panel="unbound"] [data-dns-add-name], [data-pane-panel="unbound"] [data-dns-add-save]').forEach(node=>{if('checked'in node)node.checked=false;
          node.disabled=true;
          });
        document.querySelectorAll('[data-pane-panel="unbound"] [data-dns-add-modal]').forEach(node=>{node.hidden=true;
          });
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
