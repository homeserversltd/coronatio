fn shell_document_4() -> &'static str {
    r####"      if (!el) return;
      el.textContent = 'Loading ' + route + '…'; try {
        const response = await fetch(route, { method }); const text = await response.text();
        try { el.textContent = JSON.stringify(JSON.parse(text), null, 2); }
        catch (_) { el.textContent = text; }
      } catch (error) { el.textContent = 'fetch failed: ' + error; }
    }
    document.querySelectorAll('[data-fetch]').forEach(button => button.addEventListener('click', () => fetchInto(button.dataset.fetch, button.dataset.target, button.dataset.method || 'GET'))); const uploadState = { currentPath: '/mnt/nas', selectedFiles: [], activeUploads: new Map(), pinRequired: false, uploading: false, blacklist: [], history: [] };
    const uploadFileInput = () => document.querySelector('[data-upload-file]'); const uploadSubmit = () => document.querySelector('[data-upload-submit]');
    const uploadProgressList = () => document.querySelector('[data-upload-progress-list]'); const uploadBreadcrumbs = () => document.querySelector('[data-upload-breadcrumbs]');
    const uploadReadout = () => document.getElementById('upload-readout'); function uploadFormatSize(bytes) {
      const units = ['B', 'KB', 'MB', 'GB']; let size = Number(bytes || 0);
      let unit = 0; while (size >= 1024 && unit < units.length - 1) { size /= 1024; unit += 1; }
      return size.toFixed(1) + ' ' + units[unit]; }
    function uploadStatusIcon(status) {
      if (status === 'pending') return '⏳'; if (status === 'uploading') return '📤';
      if (status === 'completed') return '✅'; if (status === 'error') return '❌';
      return '❓'; }
    function renderUploadProgress() {
      const progressList = uploadProgressList();
      if (!progressList) return;
      const uploads = Array.from(uploadState.activeUploads.values());
      progressList.hidden = uploads.length === 0;
      progressList.innerHTML = uploads.map(upload => `
        <div class="upload-progress ${upload.status}" data-upload-progress="${upload.filename}">
          <div class="upload-header"><span class="status-icon">${uploadStatusIcon(upload.status)}</span><span class="filename">${upload.filename}</span><button type="button" class="remove-button" data-upload-remove="${upload.filename}" aria-label="Remove upload">×</button></div>
          <div class="progress-section"><div class="progress-bar-container" role="progressbar" aria-label="${upload.filename}: ${upload.status}" aria-valuemin="0" aria-valuemax="100" aria-valuenow="${upload.progress.toFixed(1)}"><div class="progress-bar" style="width:${upload.progress}%"><span class="progress-text">${upload.progress.toFixed(1)}%</span></div></div><div class="upload-stats"><span class="size">${uploadFormatSize(upload.uploaded)} / ${uploadFormatSize(upload.total)}</span><span class="speed">${upload.speed ? uploadFormatSize(upload.speed) + '/s' : upload.status}</span></div>${upload.error ? `<div class="error-message" role="alert">${upload.error}</div>` : ''}</div>
        </div>`).join('');
    }
    function setUpload(filename, update) {
      const current = uploadState.activeUploads.get(filename) || { filename, progress: 0, speed: 0, uploaded: 0, total: 0, status: 'pending' };
      uploadState.activeUploads.set(filename, Object.assign(current, update));
      renderUploadProgress();
    }
    function setUploadSelection() {
      uploadState.selectedFiles = Array.from(uploadFileInput()?.files || []);
      const submit = uploadSubmit();
      if (submit) submit.disabled = uploadState.selectedFiles.length === 0 || uploadState.uploading;
      const display = document.querySelector('[data-upload-file-display]');
      if (display) {
        const label = uploadState.selectedFiles.length ? uploadState.selectedFiles.map(file => file.name).join(', ') : 'No files selected';
        display.value = label;
        display.setAttribute('aria-label', 'Selected files: ' + label);
      }
    }
    function uploadCurrentPath() {
      return document.querySelector('[data-upload-current-path]')?.value || uploadState.currentPath || '/mnt/nas';
    }
    function renderUploadBreadcrumbs(path) {
      const breadcrumbs = uploadBreadcrumbs();
      if (!breadcrumbs) return;
      const selectedPath = String(path || '/mnt/nas').replace(/\/+$/, '') || '/mnt/nas';
      const nasRoot = '/mnt/nas'; const treeRoot = document.querySelector('[data-upload-root-path]')?.value || nasRoot; const selectedRoot = selectedPath.startsWith(nasRoot) ? nasRoot : treeRoot;
      const crumbs = [{ name: 'nas', path: selectedRoot }]; let current = selectedRoot;
      selectedPath.slice(selectedRoot.length).split('/').filter(Boolean).forEach(part => { current += '/' + part; crumbs.push({ name: part, path: current }); });
      if (crumbs.length <= 1) { breadcrumbs.innerHTML = ''; return; }
      breadcrumbs.innerHTML = crumbs.map((crumb, index) => `<button type="button" class="breadcrumb-item ${crumb.path === selectedPath ? 'current' : ''}" data-upload-breadcrumb-path="${crumb.path}" aria-current="${crumb.path === selectedPath ? 'page' : 'false'}">${crumb.name}</button>${index < crumbs.length - 1 ? '<span class="breadcrumb-separator" aria-hidden="true"> / </span>' : ''}`).join('');
    }
    function syncUploadTreeSelection() {
      const activeUploadTree = document.querySelector('[data-upload-tree]');
      const selected = activeUploadTree?.querySelector('.directory-entry[aria-selected="true"]')?.dataset.directoryPath || uploadCurrentPath();
      uploadState.currentPath = selected || '/mnt/nas';
      const field = document.querySelector('[data-upload-current-path]');
      if (field) field.value = uploadState.currentPath;
      renderUploadBreadcrumbs(uploadState.currentPath);
    }
    function setUploadDirectoryError(message) {
      const error = document.querySelector('[data-upload-directory-error]');
      if (error) { error.hidden = false; error.textContent = message || '⚠️ NAS Storage Unavailable'; }
      syncUploadTreeSelection();
    }
    document.body.addEventListener('htmx:afterSwap', event => {
      const target = event.detail?.target;
      if (target instanceof Element && (target.matches('[data-upload-tree]') || target.closest('[data-upload-tree]'))) syncUploadTreeSelection();
      if (target instanceof Element && (target.matches('[data-portals-grid]') || target.closest('[data-portals-grid]'))) {
        const grid = target.matches('[data-portals-grid]') ? target : target.closest('[data-portals-grid]');
        if (grid) bindPortalFragmentControls(grid);
      }
      applyAdminDomState();
    });
    async function uploadOneFile(file, scopedAttendance = null) {
      setUpload(file.name, { filename: file.name, progress: 0, speed: 0, uploaded: 0, total: file.size, status: 'pending' });
      const form = new FormData();
      form.append('file', file);
      form.append('path', uploadCurrentPath());
      const xhr = new XMLHttpRequest();
      const start = Date.now();
      return new Promise((resolve, reject) => {
        xhr.upload.onprogress = event => {
          if (event.lengthComputable) {
            const elapsed = Math.max(1, Date.now() - start) / 1000;
            setUpload(file.name, { progress: (event.loaded / event.total) * 100, speed: event.loaded / elapsed, uploaded: event.loaded, total: event.total, status: 'uploading' });
          }
        };
        xhr.onload = () => {
          if (xhr.status >= 200 && xhr.status < 300) { setUpload(file.name, { progress: 100, uploaded: file.size, total: file.size, status: 'completed' }); const readout = uploadReadout(); if (readout) readout.textContent = xhr.responseText; resolve(); }
          else { let msg = 'Upload failed with status ' + xhr.status; let signal = ''; try { const body = JSON.parse(xhr.responseText); signal = body.firstMissingSignal || ''; if (body.error) msg = body.error; } catch (_) {} const error = new Error(msg); if (xhr.status === 428 && signal === 'upload-pin-required') { error.uploadPinRequired = true; reject(error); return; } setUpload(file.name, { status: 'error', error: msg }); reject(error); }
        };
        xhr.onerror = () => { const msg = 'Network error occurred during upload'; setUpload(file.name, { status: 'error', error: msg }); reject(new Error(msg)); };
        xhr.open('POST', '/api/files/upload');
        xhr.setRequestHeader('X-Caduceus-Document', coronatioAttendanceRuntime.documentIncarnation); const uploadAttendance = scopedAttendance || coronatioAttendanceRuntime.currentAttendance; if (uploadAttendance) xhr.setRequestHeader('X-Caduceus-Attendance', uploadAttendance);
        xhr.send(form);
      });
    }
    async function uploadSelectedFiles(scopedAttendance = null) {
      if (!uploadState.selectedFiles.length) { showCoronatioToast('No files selected for upload', 'error'); return; }
      if (uploadState.pinRequired && !headerState.isAdmin && !scopedAttendance) { openUploadModal('[data-upload-pin-modal]'); document.querySelector('[data-upload-pin-input]')?.focus(); return; }
      uploadState.uploading = true;
      const submit = uploadSubmit();
      if (submit) { submit.disabled = true; submit.textContent = 'Uploading...'; }
      let success = 0; let failed = 0; let pinRequired = false;
      for (const file of uploadState.selectedFiles) {
        try { await uploadOneFile(file, scopedAttendance); success += 1; }
        catch (error) { if (error?.uploadPinRequired && !headerState.isAdmin) { pinRequired = true; setUpload(file.name, { status: 'pending', error: '' }); break; } failed += 1; showCoronatioToast(`Failed to upload ${file.name}: ${error?.message || error}`, 'error'); }
      }
      uploadState.uploading = false;
      const submitAfterUpload = uploadSubmit();
      if (submitAfterUpload) { submitAfterUpload.textContent = 'Upload Selected Files'; submitAfterUpload.disabled = uploadState.selectedFiles.length === 0; } if (pinRequired) { openUploadModal('[data-upload-pin-modal]'); document.querySelector('[data-upload-pin-input]')?.focus(); return; }
      const summary = failed ? `Uploaded ${success} file(s), ${failed} failed` : `Successfully uploaded ${success} file(s)`;
      const readout = uploadReadout();
      if (readout) readout.textContent = summary;
      if (success && failed) showCoronatioToast(summary, 'warning'); else if (success) showCoronatioToast(summary, 'success');
    }
    function uploadAdminHeaders(json = false) { return json ? { 'content-type': 'application/json' } : {}; }
    function openUploadModal(selector) { const modal = document.querySelector(selector); if (modal) { modal.hidden = false; modal.classList.add('open'); modal.setAttribute('aria-hidden', 'false'); } }
    function closeUploadModal(modal) { if (modal) { modal.classList.remove('open'); modal.setAttribute('aria-hidden', 'true'); modal.hidden = true; const pin = modal.querySelector('[data-upload-pin-input]'); if (pin) pin.value = ''; const message = modal.querySelector('[data-upload-pin-message]'); if (message) message.textContent = ''; } }
    async function refreshUploadHistory() {
      const modal = document.querySelector('[data-upload-history-modal]'), list = modal?.querySelector('.upload-history-list'), empty = modal?.querySelector('.uploadHistoryModal'), clear = modal?.querySelector('[data-upload-clear-history]');
      try { const data = await fetch('/api/upload/history', { headers: uploadAdminHeaders() }).then(r => r.json()); uploadState.history = data.history || []; } catch (_) { uploadState.history = []; }
      if (list) { list.hidden = uploadState.history.length === 0; list.innerHTML = uploadState.history.map(line => `<div class="history-item ${String(line).includes('Successfully') ? 'success' : 'error'}">${line}</div>`).join(''); }
      if (empty) empty.hidden = uploadState.history.length !== 0;
      if (clear) clear.disabled = uploadState.history.length === 0;
    }
    async function refreshUploadBlacklist() {
      const entries = document.querySelector('[data-upload-blacklist-entries]');
      try { const data = await fetch('/api/upload/blacklist/list', { headers: uploadAdminHeaders() }).then(r => r.json()); uploadState.blacklist = data.blacklist || []; } catch (_) { uploadState.blacklist = []; }
      if (entries) entries.innerHTML = uploadState.blacklist.map((entry, index) => `<div class="blacklist-entry"><span class="entry-path">${entry}</span><button type="button" class="remove-entry" data-blacklist-remove="${index}" aria-label="Remove entry">×</button></div>`).join('');
    }
    function refreshUploadBlacklistDomOnly() { const entries = document.querySelector('[data-upload-blacklist-entries]'); if (entries) entries.innerHTML = uploadState.blacklist.map((entry, index) => `<div class="blacklist-entry"><span class="entry-path">${entry}</span><button type="button" class="remove-entry" data-blacklist-remove="${index}" aria-label="Remove entry">×</button></div>`).join(''); }
    async function verifyUploadPin() { const modal = document.querySelector('[data-upload-pin-modal]'), input = modal?.querySelector('[data-upload-pin-input]'), message = modal?.querySelector('[data-upload-pin-message]'), pin = input?.value?.trim() || ''; if (!pin) { if (message) message.textContent = 'Admin PIN is required.'; input?.focus(); return; } let scopedAttendance = null; try { const response = await fetch('/api/v1/attendance/open', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ pin }) }); const result = await response.json().catch(() => ({})); scopedAttendance = response.ok && result.admin === true && typeof result.attendance === 'string' ? result.attendance : null; if (!scopedAttendance) { if (message) message.textContent = response.status === 401 ? 'Invalid PIN' : (result.firstMissingSignal || 'PIN check unavailable'); input?.focus(); return; } showCoronatioToast('PIN Verified.', 'success'); closeUploadModal(modal); await uploadSelectedFiles(scopedAttendance); } catch (_) { if (message) message.textContent = 'PIN check unavailable'; input?.focus(); } finally { if (scopedAttendance) await fetch('/api/v1/attendance/invalidate', { method: 'POST', headers: { 'X-Caduceus-Document': coronatioAttendanceRuntime.documentIncarnation, 'X-Caduceus-Attendance': scopedAttendance }, cache: 'no-store' }).catch(() => {}); } }
    function refreshUploadTree(path = uploadCurrentPath()) { window.htmx?.ajax('GET', '/admit/upload/tree?path=%2Fmnt%2Fnas&depth=0&selected=' + encodeURIComponent(path), { target: '[data-upload-tree]', swap: 'innerHTML' }); }
    async function postUploadDirectoryAction(url, successMessage) { try { const response = await fetch(url, { method: 'POST', headers: uploadAdminHeaders(true), body: JSON.stringify({ directory: uploadState.currentPath }) }); const data = await response.json().catch(() => ({})); if (!response.ok || !(data.success ?? data.ok)) throw new Error(data.message || data.error || data.firstMissingSignal || `Request failed with status ${response.status}`); showCoronatioToast(successMessage, 'success'); return data; } catch (error) { showCoronatioToast(error?.message || 'Request failed', 'error'); return null; } }
    async function toggleUploadPin(toggle) { uploadState.pinRequired = !uploadState.pinRequired; toggle.classList.toggle('active', uploadState.pinRequired); toggle.setAttribute('aria-label', `Toggle PIN requirement (currently ${uploadState.pinRequired ? 'enabled' : 'disabled'})`); toggle.setAttribute('aria-busy', 'true'); toggle.querySelector('[data-upload-pin-spinner]')?.removeAttribute('hidden'); try { await fetch('/api/upload/pin-required-status', { method: 'POST', headers: uploadAdminHeaders(true), body: JSON.stringify({ isPinRequired: uploadState.pinRequired }) }); } finally { toggle.removeAttribute('aria-busy'); toggle.querySelector('[data-upload-pin-spinner]')?.setAttribute('hidden', ''); } }
    document.body.addEventListener('change', event => { if (event.target.matches('[data-upload-file]')) setUploadSelection(); });
    document.body.addEventListener('submit', event => { if (event.target.matches('[data-upload-pin-form]')) { event.preventDefault(); verifyUploadPin(); } if (event.target.matches('[data-upload-blacklist-form]')) { event.preventDefault(); event.target.querySelector('[data-upload-blacklist-add]')?.click(); } });
    document.body.addEventListener('click', async event => { const target = event.target instanceof Element ? event.target : null; if (!target) return;
      const modal = target.closest('[data-upload-history-backdrop], [data-upload-blacklist-backdrop], [data-upload-pin-backdrop]'); if (modal && target === modal) { closeUploadModal(modal); return; }
      const control = target.closest('[data-upload-remove], [data-blacklist-remove], [data-upload-breadcrumb-path], [data-upload-modal-close], [data-upload-pin-cancel], [data-upload-submit], [data-upload-pin-confirm], [data-upload-refresh], [data-upload-force-allow], [data-upload-set-default], [data-upload-history], [data-upload-blacklist], [data-upload-blacklist-add], [data-upload-blacklist-submit], [data-upload-clear-history], [data-upload-pin-toggle]'); if (!control) return;
      if (control.matches('[data-upload-remove]')) { uploadState.activeUploads.delete(control.dataset.uploadRemove); renderUploadProgress(); return; }
      if (control.matches('[data-blacklist-remove]')) { uploadState.blacklist.splice(Number(control.dataset.blacklistRemove), 1); refreshUploadBlacklistDomOnly(); return; }
      if (control.matches('[data-upload-breadcrumb-path]')) { uploadState.currentPath = control.dataset.uploadBreadcrumbPath || '/mnt/nas'; const field = document.querySelector('[data-upload-current-path]'); if (field) field.value = uploadState.currentPath; renderUploadBreadcrumbs(uploadState.currentPath); refreshUploadTree(uploadState.currentPath); return; }
      if (control.matches('[data-upload-modal-close], [data-upload-pin-cancel]')) { closeUploadModal(control.closest('[data-upload-history-backdrop], [data-upload-blacklist-backdrop], [data-upload-pin-backdrop]')); return; }
      if (control.matches('[data-upload-submit]')) { await uploadSelectedFiles(); return; }
      if (control.matches('[data-upload-pin-confirm]')) { await verifyUploadPin(); return; }
      if (control.matches('[data-upload-refresh]')) { refreshUploadTree(); return; }
      if (control.matches('[data-upload-force-allow]')) { if (confirm(`WARNING: This will override security settings for ${uploadState.currentPath}. \nOnly continue if you understand the risks.`)) await postUploadDirectoryAction('/api/upload/force-permissions', 'Directory permissions updated successfully'); return; }
      if (control.matches('[data-upload-set-default]')) { await postUploadDirectoryAction('/api/upload/default-directory', 'Default directory updated successfully'); return; }
      if (control.matches('[data-upload-history]')) { openUploadModal('[data-upload-history-modal]'); await refreshUploadHistory(); return; }
      if (control.matches('[data-upload-blacklist]')) { openUploadModal('[data-upload-blacklist-modal]'); await refreshUploadBlacklist(); return; }
      if (control.matches('[data-upload-blacklist-add]')) { const input = document.querySelector('[data-upload-blacklist-input]'); const next = input?.value?.trim(); if (next && !uploadState.blacklist.includes(next)) { uploadState.blacklist.push(next); input.value = ''; refreshUploadBlacklistDomOnly(); } return; }
      if (control.matches('[data-upload-blacklist-submit]')) { await fetch('/api/upload/blacklist/update', { method: 'PUT', headers: uploadAdminHeaders(true), body: JSON.stringify({ blacklist: uploadState.blacklist }) }); refreshUploadTree(); return; }
      if (control.matches('[data-upload-clear-history]')) { await fetch('/api/upload/history/clear', { method: 'POST', headers: uploadAdminHeaders() }); await refreshUploadHistory(); return; }
      if (control.matches('[data-upload-pin-toggle]')) await toggleUploadPin(control); });
    fetch('/api/upload/pin-required-status', { headers: uploadAdminHeaders() }).then(r => r.json()).then(data => { uploadState.pinRequired = !!data.isPinRequired; const b = document.querySelector('[data-upload-pin-toggle]'); b?.classList.toggle('active', uploadState.pinRequired); }).catch(() => {}); syncUploadTreeSelection();
    let uptimeBaseSeconds = null; let uptimeBaseStamp = 0; function secondsFromUptimeText(text) {
      const raw = String(text || '');
      const days = Number((raw.match(/(\d+)d/) || [0, 0])[1]); const hours = Number((raw.match(/(\d+)h/) || [0, 0])[1]);
      const minutes = Number((raw.match(/(\d+)m/) || [0, 0])[1]); const seconds = Number((raw.match(/(\d+)s/) || [0, 0])[1]);
      const total = days * 86400 + hours * 3600 + minutes * 60 + seconds;
      return total > 0 ? total : null;
    }
    function formatUptimeSeconds(total) {
      total = Math.max(0, Math.floor(Number(total) || 0));
      const days = Math.floor(total / 86400);
      const hours = Math.floor((total % 86400) / 3600);
      const minutes = Math.floor((total % 3600) / 60);
      const seconds = total % 60;
      const parts = [];
      if (days > 0) parts.push(days + 'd');
      if (hours > 0) parts.push(hours + 'h');
      if (minutes > 0) parts.push(minutes + 'm');
      parts.push(seconds + 's');
      return parts.join(' ');
    }
    function tickUptime() { const uptime = document.querySelector('[data-uptime-indicator]');
      if (!uptime || uptimeBaseSeconds === null) return;
      const elapsed = Math.floor((Date.now() - uptimeBaseStamp) / 1000);
      uptime.textContent = formatUptimeSeconds(uptimeBaseSeconds + elapsed);
      uptime.dataset.uptimeIncludesSeconds = 'true';
    }
    async function hydrateUptime() { const uptime = document.querySelector('[data-uptime-indicator]');
      if (!uptime) return;
      try {
        const data = await fetch('/api/uptime').then(r => r.json()).catch(() => null);
        const seconds = Number(data?.uptimeSeconds ?? data?.seconds ?? NaN);
        uptimeBaseSeconds = Number.isFinite(seconds) ? seconds : secondsFromUptimeText(data?.uptime);
        uptimeBaseStamp = Date.now();
        if (uptimeBaseSeconds !== null) tickUptime();
        else uptime.textContent = 'uptime unavailable';
        uptime.dataset.uptimeLoaded = data?.ok ? 'true' : 'false';
      } catch (_) { uptime.textContent = navigator.onLine ? 'uptime unavailable' : 'disconnected'; uptime.dataset.uptimeLoaded = 'false'; }
    }
    function fmtBytes(value) {
      if (value === null || value === undefined) return '—';
      const units = ['B', 'KB', 'MB', 'GB', 'TB'];
      let next = Number(value);
      let unit = 0;
      while (next >= 1024 && unit < units.length - 1) { next = next / 1024; unit += 1; }
      return next.toFixed(next >= 10 || unit === 0 ? 0 : 1) + ' ' + units[unit];
    }
    function formatChartTime(value = Date.now()) {
      const date = value instanceof Date ? value : new Date(value);
      return date.getMinutes() + ':' + date.getSeconds().toString().padStart(2, '0');
    }
    function metricPercent(value) { return value === null || value === undefined ? '—' : Number(value).toFixed(1) + '%'; }
    function loadToPercent(load) {
      const cores = navigator.hardwareConcurrency || 4;
      return Math.max(0, Math.min(100, (Number(load || 0) / cores) * 100));
    }
    const statsChartState = { labels: [], cpu: [], temp: [], upload: [], download: [], lastRx: null, lastTx: null, lastIo: {}, lastStamp: null, ioSeries: {}, seeded: false };
    function seedSeries(value, count = 24) {
      return Array.from({ length: count }, (_, index) => Math.max(0, Number(value || 0) * (0.92 + (index % 6) * 0.025)));
    }
    function statsNetworkTotals(data) {
      const network = data.network || {};
      if (network.receivedBytes !== undefined || network.sentBytes !== undefined) return { rx: Number(network.receivedBytes || 0), tx: Number(network.sentBytes || 0) };
      return {
        rx: (network.interfaces || []).reduce((sum, iface) => sum + Number(iface.rxBytes || 0), 0),
        tx: (network.interfaces || []).reduce((sum, iface) => sum + Number(iface.txBytes || 0), 0)
      };
    }
    function seedStatsHistory(label, data) {
      if (statsChartState.seeded) return;
      const cpu = loadToPercent(data.resources?.load?.one);
      const temp = Number(data.resources?.load?.cpuTemperatureCelsius || 0);
      const now = Date.now();
      statsChartState.labels = Array.from({ length: 24 }, (_, index) => formatChartTime(now - (23 - index) * 5000));
      statsChartState.cpu = seedSeries(cpu);
      statsChartState.temp = seedSeries(temp);
      statsChartState.upload = seedSeries(0);
      statsChartState.download = seedSeries(0);
      (data.io?.devices || []).forEach(device => {
        const key = device.device || device.mount;
        statsChartState.ioSeries[key] = { read: seedSeries(0), write: seedSeries(0), label: key };
      });
      statsChartState.seeded = true;
    }
    function pushChartPoint(label, data) {
      seedStatsHistory(label, data);
      const now = Date.now();
      const totals = statsNetworkTotals(data);
      const totalRx = totals.rx;
      const totalTx = totals.tx;
      const seconds = statsChartState.lastStamp ? Math.max(1, (now - statsChartState.lastStamp) / 1000) : 1;
      const downloadRate = statsChartState.lastRx === null ? 0 : Math.max(0, (totalRx - statsChartState.lastRx) / seconds);
      const uploadRate = statsChartState.lastTx === null ? 0 : Math.max(0, (totalTx - statsChartState.lastTx) / seconds);
      statsChartState.lastRx = totalRx;
      statsChartState.lastTx = totalTx;
      statsChartState.lastStamp = now;
      statsChartState.labels.push(label);
      statsChartState.cpu.push(loadToPercent(data.resources?.load?.one));
      statsChartState.temp.push(Number(data.resources?.load?.cpuTemperatureCelsius || 0));
      statsChartState.upload.push(uploadRate);
      statsChartState.download.push(downloadRate);
      (data.io?.devices || []).forEach(device => {
        const key = device.device || device.mount;
        const previous = statsChartState.lastIo[key];
        const read = previous ? Math.max(0, (Number(device.readBytes || 0) - previous.readBytes) / seconds) : 0;
        const write = previous ? Math.max(0, (Number(device.writeBytes || 0) - previous.writeBytes) / seconds) : 0;
        statsChartState.lastIo[key] = { readBytes: Number(device.readBytes || 0), writeBytes: Number(device.writeBytes || 0) };
        if (!statsChartState.ioSeries[key]) statsChartState.ioSeries[key] = { read: seedSeries(read), write: seedSeries(write), label: key };
        statsChartState.ioSeries[key].read.push(read);
        statsChartState.ioSeries[key].write.push(write);
      });
      if (statsChartState.labels.length > 60) {
        ['labels', 'cpu', 'temp', 'upload', 'download'].forEach(key => statsChartState[key].shift());
        Object.values(statsChartState.ioSeries).forEach(series => { series.read.shift(); series.write.shift(); });
      }
    }
    const statsCharts = { cpu: null, network: null, io: null };
    function destroyStatsChart(key) {
      if (statsCharts[key]) { statsCharts[key].destroy(); statsCharts[key] = null; }
    }
    function themeCssColor(token, fallback) { const value = getComputedStyle(document.documentElement).getPropertyValue(token).trim(); return value || fallback; }
    function chartTicks(tokenOrColor, callback) { const color = tokenOrColor.startsWith('--') ? themeCssColor(tokenOrColor, '#4A5568') : tokenOrColor; return { color, maxTicksLimit: 10, autoSkip: true, callback }; }
    function chartGrid() { return { color: themeCssColor('--border', '#1E293B'), borderDash: [3, 3] }; }
    function chartTooltip(labelFormatter) {
      return { enabled: true, mode: 'index', intersect: false, callbacks: { label: labelFormatter } };
    }
    function chartCommonOptions() {
      return {
        responsive: true,
        maintainAspectRatio: false,
        interaction: { mode: 'index', intersect: false },
        layout: { padding: { left: 10, right: 10, top: 20, bottom: 20 } },
        animation: { duration: 250 }
      };
    }
    function lineDataset(label, data, color, yAxisID) {
      return { label, data, borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4, yAxisID };
    }
    function createCPUChart(ctx, labels, cpuData, tempData) {
      if (statsCharts.cpu) return statsCharts.cpu;
      statsCharts.cpu = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          lineDataset('CPU Usage', cpuData, themeCssColor('--secondary', '#4A5568'), 'y-cpu'),
          lineDataset('Temperature', tempData, themeCssColor('--accent', '#90cff3'), 'y-temp')
        ] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { tooltip: chartTooltip(context => context.dataset.label + ': ' + Number(context.parsed.y || 0).toFixed(1) + (context.dataset.yAxisID === 'y-temp' ? '°C' : '%')), legend: { position: 'bottom', align: 'center', labels: { usePointStyle: true, pointStyle: 'circle', boxWidth: 8, boxHeight: 8 } } },
          scales: {
            x: { ticks: chartTicks('--hiddenTabText', value => labels[value] || value), grid: { display: false } },
            'y-cpu': { type: 'linear', display: true, position: 'left', beginAtZero: true, max: 100, ticks: chartTicks('--hiddenTabText', value => Number(value).toFixed(0) + '%'), grid: chartGrid() },
            'y-temp': { type: 'linear', display: true, position: 'right', beginAtZero: true, max: 100, ticks: chartTicks('--hiddenTabText', value => Number(value).toFixed(0) + '°C'), grid: { display: false } }
          }
        })
      });
      return statsCharts.cpu;
    }
    function createNetworkChart(ctx, labels, downloadData, uploadData) {
      if (statsCharts.network) return statsCharts.network;
      const networkMax = Math.max(1, ...downloadData, ...uploadData) * 1.1;
      const networkTicks = { color: themeCssColor('--hiddenTabText', '#4A5568'), maxTicksLimit: 10, autoSkip: true, callback: value => fmtBytes(value) + '/s' };
      statsCharts.network = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          lineDataset('Download Speed', downloadData, themeCssColor('--secondary', '#4A5568'), 'y'),
          lineDataset('Upload Speed', uploadData, themeCssColor('--accent', '#90cff3'), 'y-right')
        ] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { tooltip: chartTooltip(context => context.dataset.label + ': ' + fmtBytes(context.parsed.y) + '/s'), legend: { position: 'bottom', align: 'center', labels: { usePointStyle: true, pointStyle: 'circle', boxWidth: 8, boxHeight: 8 } } },
          scales: {
            x: { ticks: chartTicks('--hiddenTabText', value => labels[value] || value), grid: { display: false } },
            y: { beginAtZero: true, suggestedMin: 0, max: networkMax, ticks: networkTicks, grid: chartGrid() },
            'y-right': { beginAtZero: true, suggestedMin: 0, max: networkMax, position: 'right', ticks: networkTicks, grid: { display: false } }
          }
        })
      });
      return statsCharts.network;
    }
    function diskDisplayName(device) {
      const mount = device.mount || '';
      if (mount === '/mnt/nas') return 'nas';
      if (mount === '/mnt/nasbackup') return 'nasbackup';
      if ((device.device || '').includes('sda6')) return 'sda6';
      return mount.replace(/^\/mnt\//, '') || device.device || 'disk';
    }
    function createIOChart(ctx, labels, datasets) {
      if (statsCharts.io) return statsCharts.io;
      statsCharts.io = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets },
        options: Object.assign(chartCommonOptions(), {
          plugins: { legend: { display: false }, tooltip: chartTooltip(context => context.dataset.label + ': ' + fmtBytes(context.parsed.y || 0) + '/s') },
          scales: {
            x: { ticks: chartTicks('--hiddenTabText', value => labels[value] || value), grid: { display: false } },
            y: { beginAtZero: true, suggestedMin: 0, suggestedMax: Math.max(1, ...datasets.flatMap(dataset => dataset.data)) * 1.1, ticks: chartTicks('--hiddenTabText', value => fmtBytes(value) + '/s'), grid: chartGrid() }
          }
        })
      });
      return statsCharts.io;
    }
    function renderCpuChart(data) {
      const ctx = document.getElementById('cpuChart');
      if (ctx && window.Chart) {
        if (statsCharts.cpu && statsCharts.cpu.canvas !== ctx) destroyStatsChart('cpu');
        const existing = statsCharts.cpu;
        const chart = createCPUChart(ctx, statsChartState.labels, statsChartState.cpu, statsChartState.temp);
        if (existing) {
          chart.data.labels = statsChartState.labels;
          chart.data.datasets[0].data = statsChartState.cpu;
          chart.data.datasets[1].data = statsChartState.temp;
          chart.update();
        }
      }
      document.getElementById('load-1min').textContent = metricPercent(loadToPercent(data.resources?.load?.one));
      document.getElementById('load-5min').textContent = metricPercent(loadToPercent(data.resources?.load?.five));
      document.getElementById('load-15min').textContent = metricPercent(loadToPercent(data.resources?.load?.fifteen));
    }
    function interfaceLabel(name) {
      if (name === 'lan0') return 'LAN';
      if (name.startsWith('wl')) return 'Wi-Fi';
      if (name.startsWith('en')) return 'Ethernet';
      if (name.startsWith('veth')) return 'Service';
      return name;
    }
    function meaningfulInterface(iface) {
      const name = iface.name || '';
      if (!name || name === 'lo' || name === 'docker0' || name.startsWith('br-') || name.startsWith('virbr') || name.startsWith('vnet')) return false;
      if (!iface.rxBytes && !iface.txBytes) return false;
      return true;
    }
    function renderNetwork(data) {
      const ctx = document.getElementById('networkChart');
      if (ctx && window.Chart) {
        if (statsCharts.network && statsCharts.network.canvas !== ctx) destroyStatsChart('network');
        const existing = statsCharts.network;
        const chart = createNetworkChart(ctx, statsChartState.labels, statsChartState.download, statsChartState.upload);
        if (existing) {
          const networkMax = Math.max(1, ...statsChartState.download, ...statsChartState.upload) * 1.1;
          chart.data.labels = statsChartState.labels;
          chart.data.datasets[0].data = statsChartState.download;
          chart.data.datasets[1].data = statsChartState.upload;
          chart.options.scales.y.max = networkMax;
          chart.options.scales['y-right'].max = networkMax;
          chart.update();
        }
      }
      const tbody = document.querySelector('[data-network-interfaces]');
      if (!tbody) return;
      const interfaces = data.network?.interfaces;
      if (Array.isArray(interfaces)) {
        tbody.innerHTML = interfaces.filter(meaningfulInterface).map(iface => `<tr><td><span class="interface-name">${interfaceLabel(iface.name)}</span><span class="interface-label"> (${iface.name})</span></td><td class="data-cell">${fmtBytes(iface.rxBytes)}</td><td class="data-cell">${fmtBytes(iface.txBytes)}</td></tr>`).join('') || '<tr><td colspan="3">Loading network data...</td></tr>';
      } else {
        const totals = statsNetworkTotals(data);
        tbody.innerHTML = `<tr><td><span class="interface-name">Network</span></td><td class="data-cell">${fmtBytes(totals.rx)}</td><td class="data-cell">${fmtBytes(totals.tx)}</td></tr>`;
      }
    }
    function renderDiskIo(data) {
      const controls = document.querySelector('[data-device-controls]'), checked = new Map(Array.from(controls?.querySelectorAll('input[type="checkbox"]') || []).map(input => [input.name, input.checked])), devices = data.io?.devices || [];
      if (controls) controls.innerHTML = devices.map(device => {
        const name = diskDisplayName(device), readName = `read-${name}`, writeName = `write-${name}`, readChecked = checked.has(readName) ? checked.get(readName) : true, writeChecked = checked.has(writeName) ? checked.get(writeName) : true;
        return `<div class="device-control" data-io-device="${escapeHtml(name)}"><div class="device-name">${escapeHtml(name)}</div><div class="device-checkboxes"><label class="drive-checkbox"><input type="checkbox" name="${escapeHtml(readName)}" value="${escapeHtml(name)}" ${readChecked ? 'checked' : ''}>Read</label><label class="drive-checkbox"><input type="checkbox" name="${escapeHtml(writeName)}" value="${escapeHtml(name)}" ${writeChecked ? 'checked' : ''}>Write</label></div></div>`;
      }).join('') || '<div class="io-loading"><p>Loading disk I/O data...</p></div>';
      const colors = ['--secondary', '--accent', '--warning', '--success', '--error'];
      const datasets = devices.flatMap((device, index) => {
        const key = device.device || device.mount;
        const name = diskDisplayName(device);
        const series = statsChartState.ioSeries[key] || { read: [0], write: [0] };
        const color = themeCssColor(colors[index % colors.length], '#4A5568');
        return [
          { label: `${name} Read`, data: series.read, borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4 },
          { label: `${name} Write`, data: series.write, borderColor: color, backgroundColor: color, borderWidth: 2, borderDash: [3, 3], fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4 }
        ].filter(dataset => checked.get(`${dataset.label.endsWith(' Read') ? 'read' : 'write'}-${name}`) !== false);
      });
      const ctx = document.getElementById('io-chart');
      if (ctx && window.Chart) {
        if (statsCharts.io && statsCharts.io.canvas !== ctx) destroyStatsChart('io');
        const existing = statsCharts.io;
        const chart = createIOChart(ctx, statsChartState.labels, datasets);
        if (existing) {
          chart.data.labels = statsChartState.labels;
          chart.data.datasets.splice(0, chart.data.datasets.length, ...datasets);
          chart.options.scales.y.suggestedMax = Math.max(1, ...datasets.flatMap(dataset => dataset.data)) * 1.1;
          chart.update();
        }
      }
      const legend = document.getElementById('io-chart-legend');
      if (legend) legend.innerHTML = datasets.map(dataset => `<span data-io-series="${escapeHtml(dataset.label)}">${escapeHtml(dataset.label)}</span>`).join('');
    }
    function renderMemory(data) {
      const memory = data.resources?.memory || {};
      const swap = data.resources?.swap || {};
      const memoryPercent = Number(memory.percent || 0);
      const swapPercent = Number(swap.percent || 0);
      document.getElementById('memory-bar-fill').style.width = memoryPercent + '%';
      document.getElementById('memory-percent').textContent = metricPercent(memoryPercent);
      document.getElementById('memory-used').textContent = 'Used: ' + fmtBytes(memory.usedBytes);
      document.getElementById('memory-available').textContent = 'Available: ' + fmtBytes(memory.freeBytes);
      document.getElementById('memory-total').textContent = 'Total: ' + fmtBytes(memory.totalBytes);
      document.getElementById('swap-bar-fill').style.width = swapPercent + '%';
      document.getElementById('swap-percent').textContent = metricPercent(swapPercent);
      document.getElementById('swap-used').textContent = 'Used: ' + fmtBytes(swap.usedBytes);
      document.getElementById('swap-free').textContent = 'Free: ' + fmtBytes(swap.freeBytes);
      document.getElementById('swap-total').textContent = 'Total: ' + fmtBytes(swap.totalBytes);
    }
    function renderDiskUsage(data) {
      const target = document.querySelector('[data-disk-usage-stats]');
      if (!target) return;
      target.innerHTML = (data.storage || []).map(drive => {
        const label = drive.productLabel || drive.name || 'Storage';
        const mountLine = drive.mount ? `<div class="disk-mountpoint">Mount: ${drive.mount}</div>` : '';
        return `<div class="disk-usage-item"><div class="disk-usage-header"><div class="disk-device">${label} (${Number(drive.usagePercent || 0).toFixed(1)}%)</div>${mountLine}</div><div class="disk-usage-bar"><div class="disk-usage-fill" style="width:${drive.usagePercent || 0}%"></div></div><div class="disk-usage-details"><div>Used: ${fmtBytes(drive.usedBytes)}</div><div>Free: ${fmtBytes(drive.freeBytes)}</div><div>Total: ${fmtBytes(drive.totalBytes)}</div></div></div>`;
      }).join('') || '<div class="disk-usage-loading"><p>Loading disk usage data...</p></div>';
    }
    function renderStatsRoster() { renderIdentityRoster('liveness'); }
    function normalizeNetworkNotes(payload) { const notes = payload?.networkNotes || payload?.notes || payload?.data?.networkNotes || payload?.data?.notes || payload; return notes && typeof notes === 'object' && !Array.isArray(notes) ? notes : {}; }
    function canonicalNetworkNoteMac(value) { const raw = String(value ?? ''), match = raw.match(/^([0-9a-f]{2})([:-])(?:[0-9a-f]{2}\2){4}[0-9a-f]{2}$/i); return match ? raw.split(match[2]).map(octet => octet.toUpperCase()).join(':') : null; }
    function ensureNoteModal() {
      let modal = document.querySelector('[data-note-modal]'); if (modal) return modal;
      modal = document.createElement('div'); modal.className = 'modal-backdrop'; modal.dataset.noteModal = ''; modal.hidden = true; modal.innerHTML = `<div class="modal-window edit-note-modal" role="dialog" aria-modal="true" aria-labelledby="note-modal-title"><h2 id="note-modal-title" data-note-modal-title></h2><textarea class="note-textarea" data-note-textarea rows="4"></textarea><div class="modal-actions"><button type="button" data-note-cancel>Cancel</button><button type="button" data-note-confirm>Confirm</button></div><p class="error-message" data-note-error role="alert" hidden></p></div>`;
      document.body.appendChild(modal); modal.addEventListener('click', event => { if (event.target === modal || event.target.closest('[data-note-cancel]')) closeNoteModal(); }); modal.querySelector('[data-note-confirm]').addEventListener('click', saveDeviceNote); return modal;
    }
    function closeNoteModal() { const modal = document.querySelector('[data-note-modal]'); if (modal) { modal.hidden = true; modal.removeAttribute('data-mac'); } }
    function openNoteModal(mac, note) { const modal = ensureNoteModal(); modal.dataset.mac = mac; modal.querySelector('[data-note-modal-title]').textContent = `Edit Note for ${mac}`; const textarea = modal.querySelector('[data-note-textarea]'); textarea.value = note; modal.querySelector('[data-note-error]').hidden = true; modal.hidden = false; textarea.focus(); }
    async function saveDeviceNote() {
      const modal = document.querySelector('[data-note-modal]'), mac = modal?.dataset.mac || '', textarea = modal?.querySelector('[data-note-textarea]'), errorNode = modal?.querySelector('[data-note-error]'), note = textarea?.value ?? '';
      try { const response = await fetch('/api/network/notes', { method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ mac, note }) }); if (!response.ok) throw new Error(`Save failed (${response.status})`); const canonicalMac = canonicalNetworkNoteMac(mac); if (canonicalMac) identityState.notes[canonicalMac] = note; renderStatsRoster(); closeNoteModal(); showCoronatioToast('Device note saved', 'success'); }
      catch (error) { if (errorNode) { errorNode.textContent = String(error); errorNode.hidden = false; } }
    }
    function renderProcesses(data) {
      const target = document.querySelector('[data-process-usage-list]');
      if (!target) return;
      const processes = data.processes || [];
      target.innerHTML = processes.length ? processes.map(process => `<div class="process-bar" title="Process: ${process.name}\nMemory: ${fmtBytes(process.memoryBytes)}\nCPU: ${Number(process.cpuPercent || 0).toFixed(1)}%\nInstances: ${process.processCount || 1}"><div class="process-bar-fill" style="width:${Math.max(Number(process.cpuPercent || 0), 1)}%"></div><div class="process-text-container"><span class="process-name">${process.name}</span><span class="process-usage">${Number(process.cpuPercent || 0).toFixed(1)}%</span></div></div>`).join('') : '<div class="process-usage-empty"><p>No process data available</p></div>';
    }
    async function hydrateStats() {
      if (statsHydrationInFlight) return; statsHydrationInFlight = true;
      try {
        const statsResponse = await fetch('/api/stats', { cache: 'no-store' }); if (!statsResponse.ok) throw new Error(`Stats unavailable (${statsResponse.status})`);
        const data = await statsResponse.json(), label = formatChartTime(), roster = identityRows(data.keaLeases?.entries); identityState.roster = roster; identityState.notes = Object.fromEntries(roster.map(row => [canonicalNetworkNoteMac(row.mac), row.note ?? '']).filter(([mac]) => mac)); pushChartPoint(label, data); if (data.resources?.load) renderCpuChart(data); if (data.network) renderNetwork(data); if (data.io) renderDiskIo(data); if (data.resources?.memory) renderMemory(data); if (data.storage) renderDiskUsage(data); if (data.keaLeases) renderStatsRoster(); if (data.processes) renderProcesses(data);
      } catch (_) { /* OG has no Stats-family error face; retain the last truthful frame. */ }
      finally { statsHydrationInFlight = false; }
    }
    function escapeHtml(value) {
      return String(value ?? '').replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[char]));
    }
    let factoryPortalNamesPromise;
    function factoryPortalNames() {
      if (!factoryPortalNamesPromise) factoryPortalNamesPromise = fetch('/api/portals/factory', { cache: 'no-store' }).then(response => response.ok ? response.json() : Promise.reject(new Error(`Factory portals unavailable (${response.status})`))).then(payload => new Set(payload.factoryPortals || [])).catch(error => { factoryPortalNamesPromise = undefined; throw error; });
      return factoryPortalNamesPromise;
    }
    function setPortalFormError(form, field, message) { const input = form.elements[field], node = form.querySelector(`[data-portal-error-for="${field}"]`); if (input) input.classList.toggle('error', Boolean(message)); if (node) { node.textContent = message || ''; node.hidden = !message; } }
    async function submitPortalForm(event) {
      event.preventDefault(); const form = event.currentTarget, type = form.elements.type.value;
      const name = form.elements.name.value.trim(), description = form.elements.description.value.trim();
      const servicesText = form.elements.services.value.trim(), port = Number.parseInt(form.elements.port.value, 10), localURL = form.elements.localURL.value.trim();
      const errors = { name: !name ? 'Portal name is required' : '', description: !description ? 'Description is required' : '', services: type !== 'link' && !servicesText ? 'At least one service is required' : '', port: type !== 'link' && (!Number.isInteger(port) || port < 1 || port > 65535) ? 'Port must be a valid number between 1 and 65535' : '', localURL: !localURL ? 'Local URL is required' : (!/^https?:\/\//.test(localURL) ? 'Local URL must start with http:// or https://' : '') };
      Object.entries(errors).forEach(([field, message]) => setPortalFormError(form, field, message));
      if (Object.values(errors).some(Boolean)) return;
      const portal = { name, description, type, localURL, services: type === 'link' ? [] : servicesText.split(',').map(service => service.trim()).filter(Boolean) }; if (type !== 'link') portal.port = port;

      const submit = form.querySelector('[type="submit"]'); if (submit) { submit.disabled = true; submit.setAttribute('aria-busy', 'true'); submit.innerHTML = '<span class="loading-spinner small" aria-hidden="true"></span> Creating...'; }
      try {
        const response = await fetch('/api/portals', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(portal) }); if (!response.ok) { const body = await response.json().catch(() => ({})); throw new Error(body.message || body.error || `Create failed (${response.status})`); }
        form.reset(); closePortalModals(); await refreshElementFragment('portals'); showCoronatioToast(`Portal "${name}" created successfully`, 'success');
      } catch (error) { showCoronatioToast(error.message || 'Failed to create portal', 'error'); }
      finally { if (submit) { submit.disabled = false; submit.removeAttribute('aria-busy'); submit.innerHTML = '<i class="fas fa-plus"></i> Create Portal'; } }
    }
    async function deletePortal(event) {
      event.preventDefault(); event.stopPropagation(); const name = event.currentTarget.dataset.portalName || event.currentTarget.dataset.deletePortal; if (!headerState.isAdmin || !name) return;
      try {
        const factoryNames = await factoryPortalNames(); if (factoryNames.has(name)) { showCoronatioToast('Factory portals cannot be deleted', 'error'); return; }
        if (!window.confirm(`Delete portal "${name}"?`)) return;
        const response = await fetch(`/api/portals/${encodeURIComponent(name)}`, { method: 'DELETE', headers: {} });
        if (!response.ok) { const body = await response.json().catch(() => ({})); throw new Error(body.message || body.error || `Delete failed (${response.status})`); }
        await refreshElementFragment('portals'); showCoronatioToast(`Portal "${name}" deleted`, 'success');
      } catch (error) { showCoronatioToast(error.message || 'Failed to delete portal', 'error'); }
    }
    function renderAddPortalCard() {
      return `<div class="portal-card add-portal-card" data-admin-only data-admin-viewport="portals" data-add-portal-open role="button" tabindex="0" aria-label="Add new portal">
        <div class="portal-card-face"><div class="add-portal-content"><div class="add-portal-icon"><i class="fas fa-plus"></i></div><h3 class="add-portal-title">Add Portal</h3><p class="add-portal-description">Create a new portal for your services</p></div></div>
      </div>`;
    }
    function openPortalModal(selector) { const modal = document.querySelector(selector); if (modal) modal.hidden = false; }
    function closePortalModals() { document.querySelectorAll('[data-add-portal-modal], [data-service-status-modal]').forEach(modal => { modal.hidden = true; }); }
    function cartridgeIdFromTitle(title) { return String(title || '').toLowerCase().trim().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, ''); }
    function cartridgeRows(payload) { const body = payload?.cartridges ?? payload ?? {}; return Array.isArray(body) ? body : (body.cartridges || body.rows || []); }
    async function loadCartridgeManagement() { const list = document.querySelector('[data-cartridge-management-list]'); if (!list) return;
      try { const response = await fetch('/api/v1/cartridges', { cache: 'no-store' }); const payload = await response.json().catch(() => ({})); if (!response.ok) throw new Error(payload.firstMissingSignal || 'Cartridges unavailable'); const rows = cartridgeRows(payload); list.innerHTML = rows.length ? rows.map(row => `<li data-cartridge-row="${escapeHtml(row.id)}"><span>${escapeHtml(row.title || row.id)}</span><button type="button" class="secondary" data-cartridge-remove="${escapeHtml(row.id)}">Remove</button></li>`).join('') : '<li>No loadable cartridges admitted.</li>'; }
      catch (error) { list.textContent = error.message || 'Cartridges unavailable'; } }
    async function refreshCartridgeTabs() { const active = currentActiveTabId(); const selected = await refreshTabBar(active); if (selected) showPane(selected); }
    async function submitCartridgeForm(event) { event.preventDefault(); const form = event.currentTarget; const title = form.elements.title.value.trim(), url = form.elements.url.value.trim(), id = cartridgeIdFromTitle(title); if (!id || !url) { showCoronatioToast('Title and URL are required', 'error'); return; } const submit = form.querySelector('[type="submit"]'); if (submit) submit.disabled = true;
      try { const response = await fetch('/api/v1/cartridges/admit', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ id, title, url, guest_class: 'iframe', admin_only: Boolean(form.elements.adminOnly.checked) }) }); const payload = await response.json().catch(() => ({})); if (!response.ok || payload.ok === false) throw new Error(payload.firstMissingSignal || 'Could not add tab'); form.reset(); closeAddTabModal(); showCoronatioToast(`Added ${title}`, 'success'); await refreshCartridgeTabs(); }
      catch (error) { showCoronatioToast(error.message || 'Could not add tab', 'error'); } finally { if (submit) submit.disabled = false; } }
    async function removeCartridge(id) { if (!id) return;
      try { const response = await fetch('/api/v1/cartridges/remove', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ id }) }); const payload = await response.json().catch(() => ({})); if (!response.ok || payload.ok === false) throw new Error(payload.firstMissingSignal || 'Could not remove tab'); showCoronatioToast(`Removed ${id}`, 'success'); await loadCartridgeManagement(); await refreshCartridgeTabs(); }
      catch (error) { showCoronatioToast(error.message || 'Could not remove tab', 'error'); } }
    function openAddTabModal() { const modal = document.querySelector('[data-add-tab-modal]'); if (!modal) return; modal.hidden = false; modal.setAttribute('aria-hidden', 'false'); void loadCartridgeManagement(); requestAnimationFrame(() => modal.querySelector('[name="title"]')?.focus()); }
    function closeAddTabModal() { const modal = document.querySelector('[data-add-tab-modal]'); if (!modal) return; modal.hidden = true; modal.setAttribute('aria-hidden', 'true'); }
    function showPortalServiceStatus(results) {
      const modal = document.querySelector('[data-service-status-modal]'); const content = modal?.querySelector('[data-service-status-content]');
      if (content) content.textContent = results.map(result => {
        const header = `=== ${result.service || 'service'} ===`; const status = result.output || result.message || result.error || result.raw || 'No status available';
        if (result.error) return `${header}\n⚠️ Error State:\n${status}`;
        const isActive = result.active !== undefined ? result.active : result.success;
        return `${header}\n${!isActive ? '⚠️ Service Inactive/Failed:\n' : ''}${status}`;
      }).join('\n\n');
      if (modal) modal.hidden = false;
    }
    async function handlePortalServiceAction(event) {
      event.preventDefault();
      event.stopPropagation();
      const button = event.currentTarget;
      const controls = button.closest('[data-portal-services]');
      const action = button.dataset.serviceAction;
      let services = [];
      try { services = JSON.parse(decodeURIComponent(controls?.dataset.portalServices || '%5B%5D')); } catch (_) { services = []; }
      if (!services.length) {
        if (action === 'status') showPortalServiceStatus([{ service: 'service', error: 'No services specified for this portal.' }]);
        else showCoronatioToast('No services specified', 'error');
        return;
      }
      const results = [];

      for (const service of services) {
        try {
          const response = await fetch('/api/service/control', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ service, action })
          });
          const text = await response.text();
          let result;
          try { result = JSON.parse(text); } catch (_) { result = { success: response.ok, output: text }; }
          results.push({ service, ...result });
          if (action !== 'status') {
            if (response.ok && result.success !== false) showCoronatioToast(result.message || `Successfully ${action}ed ${service}`, 'success');
            else showCoronatioToast(result.error || result.message || `Failed to ${action} ${service}`, 'error');
          }
        } catch (error) {
          results.push({ service, action, error: String(error) });
          if (action !== 'status') showCoronatioToast(`Failed to ${action} ${service}`, 'error');
        }
      }
      if (action === 'status') showPortalServiceStatus(results);
    }
    function bindPortalFragmentControls(grid) {
      grid.querySelectorAll('[data-portal-card]').forEach(card => {
        const open = () => { const url = card.dataset.portalUrl; if (url && url !== '#') window.open(url, '_blank', 'noopener,noreferrer'); };
        card.addEventListener('click', open);
        card.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); open(); } });
      });
      grid.querySelectorAll('[data-service-action]').forEach(button => button.addEventListener('click', handlePortalServiceAction));
      grid.querySelectorAll('[data-portal-delete]').forEach(button => button.addEventListener('click', deletePortal));
    }
    async function refreshPortalCurrentness() {
      const grid = document.querySelector('[data-portals-grid]');
      if (!grid || grid.offsetParent === null) return;
      try {
        const data = await fetch('/api/portals/currentness', { cache: 'no-store' }).then(response => response.ok ? response.json() : null);
        if (!data?.portals) return;
        grid.querySelectorAll('[data-portal-card]').forEach(card => {
          const next = data.portals[card.dataset.portalName] || 'unknown';
          const statuses = ['up', 'down', 'partial', 'unknown'];
          const current = statuses.find(status => card.classList.contains(status)) || 'unknown';
          if (current !== next) {
            card.classList.remove(...statuses);
            card.classList.add(next);
          }
        });
      } catch (_) {}
    }
    const portalElementsChanged = consumeElementsChanged({
      paneId: 'portals',
      route: '/api/portals/elements',
      target: () => document.querySelector('[data-portals-grid]'),
      afterReplace: target => bindPortalFragmentControls(target)
    });
    const statsElementsChanged = consumeElementsChanged({
      paneId: 'stats',
      route: '/api/stats/elements',
      target: () => document.querySelector('[data-stats-viewport]'),
      afterReplace: () => hydrateStats()
    });
    function refreshElementFragment(tabId) {
      return ({ portals: portalElementsChanged, stats: statsElementsChanged })[tabId]?.();
    }
    async function toggleElementVisibility(tabId, elementId, visible) {
      try {

        const response = await fetch('/api/tabs/elements', {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tabId, elementId, visibility: visible })
        });
        const html = await response.text();
        if (!response.ok) {
          let message = `Failed to toggle visibility for ${elementId}`, refusal = html.includes('data-element-visibility-refusal="admin-session-required"') ? 'admin-session-required' : '';
          try { const error = JSON.parse(html); refusal = error.refusal || error.error || error.code || refusal; message = error.message || error.error || message; } catch (_) {}
          if (refusal === 'admin-session-required') { showCoronatioToast('Admin session expired — re-enter PIN', 'error'); setAdminMode(false); openPinModal('enter'); return; }
          showCoronatioToast(message, 'error'); return;
        }
        const target = tabId === 'portals' ? document.querySelector('[data-portals-grid]') : document.querySelector('[data-stats-viewport]');
        if (!target) return;
        const changed = morphLivePane(target, html);
        if (changed) {
          if (tabId === 'portals') bindPortalFragmentControls(target);
          if (tabId === 'stats') hydrateStats();
          applyAdminDomState();
        }
      } catch (_) {
        showCoronatioToast(`Failed to toggle visibility for ${elementId}`, 'error');
      }
    }
    async function hydrateFavoriteManifest() {
      await runFavoriteLadder({ startAt: 0, useHash: true });
    }
    // TEST-001: og Test UX-library chrome is allowed here: generic scoped tabs, demo modal, demo readbacks.
    function inTabScope(element, scope) {
      return element && element.closest('[data-tab-scope]') === scope;
    }
    function switchScopedTabs(tabButton) {
      const scope = tabButton?.closest('[data-tab-scope]');
      const selectedName = tabButton?.dataset.tabId;
      if (!scope || !selectedName) return;
      scope.querySelectorAll('[data-tab-id]').forEach(tab => {
        if (!inTabScope(tab, scope)) return;
        const selected = tab.dataset.tabId === selectedName;
        tab.classList.toggle('active', selected);
        tab.classList.toggle('ui-tab--active', selected);
        tab.setAttribute('aria-selected', String(selected));
      });
      scope.querySelectorAll('[data-tab-panel]').forEach(panel => {
        if (!inTabScope(panel, scope)) return;
        panel.classList.toggle('active', panel.dataset.tabPanel === selectedName);
      });
    }
    const motionTimers = new Set();
    function setMotionPhase(lifecycle, phase) {
      lifecycle.dataset.motionPhase = phase;
      const readback = lifecycle.querySelector('[data-motion-phase-readback]');
      if (readback) readback.textContent = phase;
    }
    function stillMotionLab(lab) {
      motionTimers.forEach(timer => clearTimeout(timer));
      motionTimers.clear();
      lab?.querySelectorAll('.is-running').forEach(specimen => specimen.classList.remove('is-running'));
      lab?.querySelectorAll('[data-motion-lifecycle]').forEach(lifecycle => setMotionPhase(lifecycle, 'REST'));
      lab?.querySelectorAll('.motion-toggle').forEach(toggle => toggle.setAttribute('aria-checked', 'false'));
    }
    function runMotionLifecycle(lifecycle) {
      stillMotionLab(lifecycle.closest('[data-animation-lab]'));
      const phases = [['ENTER', 0], ['HOLD', 320], ['EXIT', 960], ['REST', 1280]];
      phases.forEach(([phase, delay]) => {
        const timer = setTimeout(() => { setMotionPhase(lifecycle, phase); motionTimers.delete(timer); }, delay);
        motionTimers.add(timer);
      });
    }
    function playMotion(button) {
      const lab = button.closest('[data-animation-lab]');
      if (!lab) return;
      const lifecycle = button.closest('[data-motion-lifecycle]');
      if (lifecycle) return runMotionLifecycle(lifecycle);
      const specimen = button.closest('.animation-specimen');
      if (!specimen) return;
      const target = specimen.querySelector('.motion-progress, .motion-spinner, .motion-modal-stage, .motion-toggle, .motion-toast, .motion-card') || specimen;
      const running = target.classList.toggle('is-running');
      if (target.matches('.motion-toggle')) target.setAttribute('aria-checked', String(running));
      button.textContent = running ? 'Stop demo' : (button.dataset.animationPlay === '' && (target.matches('.motion-progress, .motion-spinner')) ? 'Start demo' : 'Play');
    }
    const testLanIpState = { start: 100, end: 163, taken: new Map([[103, 'Media server'], [120, 'Garden tablet']]), selected: 100, page: 0, live: false };
    const testUpstreamProviders = Object.freeze({ quad9: ['9.9.9.9', '149.112.112.112'], cloudflare: ['1.1.1.1', '1.0.0.1'], google: ['8.8.8.8', '8.8.4.4'] });
    const testDhcpRows = payload => Array.isArray(payload) ? payload : (payload?.leases || payload?.reservations || payload?.data || payload?.items || []);
    function testLanOctet(value) { const match = String(value || '').match(/^192\.168\.123\.(\d{1,3})$/); return match ? Number(match[1]) : null; }
    function renderTestLanIpCalendar() { const specimen = document.querySelector('[data-test-lan-ip-calendar]'); const grid = specimen?.querySelector('[data-lan-ip-grid]'); if (!grid) return; const pageSize = 32; const first = testLanIpState.start + testLanIpState.page * pageSize; const last = Math.min(testLanIpState.end, first + pageSize - 1); specimen.querySelector('[data-lan-ip-range]').textContent = `${first} - ${last}`; specimen.querySelector('[data-lan-ip-prev]').disabled = first <= testLanIpState.start; specimen.querySelector('[data-lan-ip-next]').disabled = last >= testLanIpState.end; grid.replaceChildren(...Array.from({ length: Math.max(0, last - first + 1) }, (_, offset) => { const octet = first + offset, holder = testLanIpState.taken.get(octet), button = document.createElement('button'); button.type = 'button'; button.className = 'ui-lan-ip-calendar__octet'; button.dataset.lanIpOctet = String(octet); button.textContent = String(octet); button.disabled = Boolean(holder); button.setAttribute('role', 'gridcell'); button.setAttribute('aria-pressed', String(testLanIpState.selected === octet)); button.setAttribute('aria-label', holder ? `192.168.123.${octet}, held by ${holder}` : `Select 192.168.123.${octet}`); if (holder) button.title = holder; return button; })); const selected = specimen.querySelector('[data-lan-ip-readout]'); if (selected) selected.textContent = `Selected address: 192.168.123.${testLanIpState.selected}${testLanIpState.live ? '' : ' (demonstration range)'}`; }
    async function hydrateTestNetworkCatalog() { if (!viewportFamilyAdmitted('test')) return; try { const [boundary, leases, reservations] = await Promise.all(['/api/network/dhcp/boundary', '/api/network/dhcp/leases', '/api/network/dhcp/reservations'].map(route => fetch(route, { cache: 'no-store' }).then(response => response.ok ? response.json() : Promise.reject(new Error(route))))); if (!viewportFamilyAdmitted('test')) return; const rawBoundary = boundary?.boundary || boundary || {}; const start = testLanOctet(rawBoundary.start || rawBoundary.first || rawBoundary.pool_start || rawBoundary.reservation_start); const end = testLanOctet(rawBoundary.end || rawBoundary.last || rawBoundary.pool_end || rawBoundary.reservation_end); if (!Number.isInteger(start) || !Number.isInteger(end) || start > end) throw new Error('reservation range unavailable'); const taken = new Map(); [...testDhcpRows(leases), ...testDhcpRows(reservations)].forEach(row => { const octet = testLanOctet(row?.ip || row?.['ip-address'] || row?.ipAddress); if (octet !== null) taken.set(octet, String(row?.hostname || row?.device_name || row?.name || 'Reserved device')); }); testLanIpState.start = start; testLanIpState.end = end; testLanIpState.taken = taken; testLanIpState.live = true; } catch (_) { testLanIpState.start = 100; testLanIpState.end = 163; testLanIpState.taken = new Map([[103, 'Media server'], [120, 'Garden tablet']]); testLanIpState.live = false; } const firstFree = Array.from({ length: testLanIpState.end - testLanIpState.start + 1 }, (_, index) => testLanIpState.start + index).find(octet => !testLanIpState.taken.has(octet)); testLanIpState.selected = firstFree || testLanIpState.start; testLanIpState.page = Math.floor((testLanIpState.selected - testLanIpState.start) / 32); renderTestLanIpCalendar(); }
    const now = new Date();
    const testMonthlyCalendarState = { displayedYear: now.getFullYear(), displayedMonth: now.getMonth(), selected: now };
    function renderTestMonthlyCalendar() { const specimen = document.querySelector('[data-ui-calendar-monthly]'); const grid = specimen?.querySelector('[data-ui-calendar-monthly-grid]'); if (!grid) return; const { displayedYear: year, displayedMonth: month, selected } = testMonthlyCalendarState, offset = (new Date(year, month, 1).getDay() + 6) % 7, days = new Date(year, month + 1, 0).getDate(), selectedInDisplayedMonth = selected.getFullYear() === year && selected.getMonth() === month; specimen.querySelector('[data-ui-calendar-monthly-month]').textContent = new Date(year, month, 1).toLocaleDateString(undefined, { month: 'long', year: 'numeric' }); grid.replaceChildren(...Array.from({ length: offset + days + ((7 - ((offset + days) % 7)) % 7) }, (_, index) => { if (index < offset || index >= offset + days) { const blank = document.createElement('span'); blank.className = 'ui-calendar-monthly__blank'; return blank; } const day = index - offset + 1, button = document.createElement('button'); button.type = 'button'; button.className = 'ui-calendar-monthly__day'; button.dataset.uiCalendarMonthlyDay = String(day); button.textContent = String(day); const isSelected = selectedInDisplayedMonth && day === selected.getDate(); button.setAttribute('aria-pressed', String(isSelected)); button.classList.toggle('is-selected', isSelected); return button; })); const output = specimen.querySelector('[data-ui-calendar-monthly-output]'); if (output) output.textContent = `Selected date: ${selected.toLocaleDateString(undefined, { dateStyle: 'long' })}`; }
    function renderTestUpstreamReadout(specimen) { const addresses = [...specimen.querySelectorAll('[data-upstream-provider]:checked')].flatMap(box => testUpstreamProviders[box.value] || []); const custom = specimen.querySelector('[data-upstream-custom-addresses]')?.value.split(',').map(value => value.trim()).filter(Boolean) || []; const dot = Boolean(specimen.querySelector('[data-upstream-dot]')?.checked); specimen.querySelector('[data-upstream-readout]').textContent = `Upstreams: [${[...addresses, ...custom].join(', ')}] · DoT: ${dot ? 'on' : 'off'}`; }

    document.body.addEventListener('click', event => {
      const toastSpawn = event.target.closest('[data-coronatio-toast-spawn]');
      if (toastSpawn) { showCoronatioToast(toastSpawn.dataset.toastMessage || 'Notification', toastSpawn.dataset.toastVariant || 'info'); return; }
      const sourceCurrencyUpdate = event.target.closest('[data-source-currency-update]');
      if (sourceCurrencyUpdate) { void startSourceCurrencyUpdate(sourceCurrencyUpdate); return; }
      const toastDismiss = event.target.closest('[data-coronatio-toast]'); const loadingToggle = event.target.closest('[data-loading-spinner-toggle]');
      if (toastDismiss) { dismissCoronatioToast(toastDismiss); return; } if (loadingToggle) return toggleLoadingSpinnerDemo(loadingToggle);
      const catalogEye = event.target.closest('[data-ui-visibility-toggle]');
      if (catalogEye) { event.preventDefault(); const visible = catalogEye.dataset.visible !== 'true'; catalogEye.dataset.visible = String(visible); catalogEye.setAttribute('aria-pressed', String(visible)); catalogEye.classList.toggle('ui-visibility-toggle--visible', visible); catalogEye.classList.toggle('ui-visibility-toggle--hidden', !visible); const icon = catalogEye.querySelector('i'); if (icon) icon.className = visible ? 'fas fa-eye' : 'fas fa-eye-slash'; const specimen = catalogEye.closest('[data-visibility-specimen]'); if (specimen) { specimen.dataset.visible = String(visible); const label = specimen.querySelector('[data-visibility-state-label]'); if (label) label.textContent = visible ? 'Visible' : 'Dimmed hidden'; } return; }
      const stillness = event.target.closest('[data-motion-stillness]');
      if (stillness) { stillMotionLab(stillness.closest('[data-animation-lab]')); return; }
      const animationPlay = event.target.closest('[data-animation-play]');
      if (animationPlay) { playMotion(animationPlay); return; }
      const monthlyPrevious = event.target.closest('[data-ui-calendar-monthly-prev]'); if (monthlyPrevious) { testMonthlyCalendarState.displayedMonth -= 1; if (testMonthlyCalendarState.displayedMonth < 0) { testMonthlyCalendarState.displayedMonth = 11; testMonthlyCalendarState.displayedYear -= 1; }; renderTestMonthlyCalendar(); return; }
      const monthlyNext = event.target.closest('[data-ui-calendar-monthly-next]'); if (monthlyNext) { testMonthlyCalendarState.displayedMonth += 1; if (testMonthlyCalendarState.displayedMonth > 11) { testMonthlyCalendarState.displayedMonth = 0; testMonthlyCalendarState.displayedYear += 1; }; renderTestMonthlyCalendar(); return; }
      const monthlyDay = event.target.closest('[data-ui-calendar-monthly-day]'); if (monthlyDay) { testMonthlyCalendarState.selected = new Date(testMonthlyCalendarState.displayedYear, testMonthlyCalendarState.displayedMonth, Number(monthlyDay.dataset.uiCalendarMonthlyDay)); renderTestMonthlyCalendar(); return; }
      const lanIpPrevious = event.target.closest('[data-lan-ip-prev]'); if (lanIpPrevious) { testLanIpState.page = Math.max(0, testLanIpState.page - 1); renderTestLanIpCalendar(); return; }
      const lanIpNext = event.target.closest('[data-lan-ip-next]'); if (lanIpNext) { testLanIpState.page = Math.min(Math.ceil((testLanIpState.end - testLanIpState.start + 1) / 32) - 1, testLanIpState.page + 1); renderTestLanIpCalendar(); return; }
      const lanIpOctet = event.target.closest('[data-lan-ip-octet]'); if (lanIpOctet && !lanIpOctet.disabled) { testLanIpState.selected = Number(lanIpOctet.dataset.lanIpOctet); renderTestLanIpCalendar(); return; }
      const scopedTab = event.target.closest('[data-tab-id]');
      if (scopedTab) return switchScopedTabs(scopedTab);
      const portalEye = event.target.closest('[data-portal-visibility-toggle]');
      if (portalEye) { event.preventDefault(); event.stopPropagation(); toggleElementVisibility('portals', portalEye.dataset.portalVisibilityToggle, portalEye.dataset.visible !== 'true'); return; }
      const editNote = event.target.closest('[data-edit-note-button]');
      if (editNote) { event.preventDefault(); openNoteModal(editNote.dataset.mac || '', editNote.dataset.note || ''); return; }
      const statEye = event.target.closest('[data-stat-visibility-toggle]');
      if (statEye) { event.preventDefault(); event.stopPropagation(); toggleElementVisibility('stats', statEye.dataset.statVisibilityToggle, statEye.dataset.visible !== 'true'); return; }
      const addTab = event.target.closest('[data-add-tab-button]');
      if (addTab) { openAddTabModal(); return; }
      const removeCartridgeButton = event.target.closest('[data-cartridge-remove]');
      if (removeCartridgeButton) { removeCartridge(removeCartridgeButton.dataset.cartridgeRemove); return; }
      const addPortal = event.target.closest('[data-add-portal-open], [data-test-add-portal]');
      if (addPortal) { openPortalModal('[data-add-portal-modal]'); return; }
      const addTabModalClose = event.target.closest('[data-add-tab-modal-close]');
      if (addTabModalClose) { closeAddTabModal(); return; }
      const addTabBackdrop = event.target.closest('[data-add-tab-modal]');
      if (addTabBackdrop && event.target === addTabBackdrop) { closeAddTabModal(); return; }
      const portalModalClose = event.target.closest('[data-portal-modal-close]');
      if (portalModalClose) { closePortalModals(); return; }
      const portalBackdrop = event.target.closest('[data-add-portal-modal], [data-service-status-modal]');
      if (portalBackdrop && event.target === portalBackdrop) { closePortalModals(); return; }
      const copyStatus = event.target.closest('[data-service-status-copy]');
      if (copyStatus) { navigator.clipboard?.writeText(document.querySelector('[data-service-status-content]')?.textContent || ''); showCoronatioToast('Status copied to clipboard', 'success'); return; }
      const fileButton = event.target.closest('[data-upload-file-button]');
      if (fileButton) { uploadFileInput?.click(); return; }
      const health = event.target.closest('[data-test-health-check]');
      if (health) { const out = document.querySelector('[data-test-health-output]'); if (out) out.textContent = JSON.stringify({ schema: 'coronatio.test.health.v1', status: 'ready', dependencies: { rust_shell: true, theme_catalog: Boolean(themeCatalog?.themes), ux_library: true }, theme: headerState.theme }, null, 2); return; }
      const testFileButton = event.target.closest('[data-test-file-input] .ui-file-input__button'); if (testFileButton) { testFileButton.closest('[data-test-file-input]')?.querySelector('[data-ui-file-input]')?.click(); return; }
      const testBreadcrumb = event.target.closest('[data-test-breadcrumb-path]'); if (testBreadcrumb) { const path = testBreadcrumb.dataset.testBreadcrumbPath; const nav = testBreadcrumb.closest('.breadcrumb-navigation, .ui-breadcrumbs'); nav?.querySelectorAll('[data-test-breadcrumb-path]').forEach(crumb => { const current = crumb.dataset.testBreadcrumbPath === path; crumb.classList.toggle('current', current); crumb.classList.toggle('ui-breadcrumbs__item--current', current); if (current) crumb.setAttribute('aria-current', 'page'); else crumb.removeAttribute('aria-current'); }); const specimen = testBreadcrumb.closest('[data-test-upload-domain-pack]'); const out = specimen?.querySelector('[data-test-domain-path]') || testBreadcrumb.closest('.showcase-item')?.querySelector('[data-breadcrumb-path]'); if (out) out.textContent = path; return; }
      const testDirectory = event.target.closest('[data-test-directory-path]'); if (testDirectory) { const specimen = testDirectory.closest('[data-test-upload-domain-pack]'); specimen?.querySelectorAll('[data-test-directory-path]').forEach(entry => { const selected = entry === testDirectory; entry.classList.toggle('selected', selected); entry.setAttribute('aria-selected', String(selected)); }); const out = specimen?.querySelector('[data-test-domain-path]'); if (out) out.textContent = testDirectory.dataset.testDirectoryPath; return; }
      const reset = event.target.closest('[data-reset-breadcrumbs]'); if (reset) { const item = reset.closest('.showcase-item'); const out = item?.querySelector('[data-breadcrumb-path]'); if (out) out.textContent = '/mnt/nas'; item?.querySelectorAll('[data-test-breadcrumb-path]').forEach(crumb => { const current = crumb.dataset.testBreadcrumbPath === '/mnt/nas'; crumb.classList.toggle('current', current); crumb.classList.toggle('ui-breadcrumbs__item--current', current); if (current) crumb.setAttribute('aria-current', 'page'); else crumb.removeAttribute('aria-current'); }); return; }
      const expand = event.target.closest('[data-test-card-expand]');
      if (expand) { const expanded = expand.closest('.test-card')?.querySelector('.test-card-expanded'); if (expanded) { expanded.hidden = !expanded.hidden; expand.textContent = expanded.hidden ? '+' : '−'; } return; }
      const modalOpen = event.target.closest('[data-ux-modal-open]');
      if (modalOpen) return openUxModalDemo(modalOpen.dataset.uxModalOpen, modalOpen);
      const modalClose = event.target.closest('[data-ux-modal-close]');
      if (modalClose) return closeUxModalDemo();
      const backdrop = event.target.closest('[data-ux-modal-demo-backdrop]');
      if (backdrop && event.target === backdrop) return closeUxModalDemo();
    });
    document.addEventListener('keydown', event => { const current = event.target.closest?.('[data-lan-ip-octet]'); if (!current || !['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Home', 'End'].includes(event.key)) return; const buttons = [...current.closest('[data-lan-ip-grid]').querySelectorAll('[data-lan-ip-octet]:not(:disabled)')]; const index = buttons.indexOf(current); if (index < 0) return; const next = event.key === 'Home' ? 0 : event.key === 'End' ? buttons.length - 1 : Math.max(0, Math.min(buttons.length - 1, index + ({ ArrowLeft: -1, ArrowRight: 1, ArrowUp: -8, ArrowDown: 8 }[event.key] || 0))); event.preventDefault(); buttons[next]?.focus(); });
    renderTestMonthlyCalendar();
    document.body.addEventListener('mouseenter', event => {
      const toast = event.target.closest?.('[data-coronatio-toast]'); if (!toast || toast.classList.contains('toast-exit')) return;
      const elapsed = Date.now() - Number(toast.dataset.toastStartedAt || Date.now());
      toast.dataset.toastRemaining = String(Math.max(0, Number(toast.dataset.toastRemaining || 3000) - elapsed));
      window.clearTimeout(Number(toast.dataset.toastTimer || 0));
    }, true);
    document.body.addEventListener('mouseleave', event => {
      const toast = event.target.closest?.('[data-coronatio-toast]');
      if (toast && !toast.classList.contains('toast-exit')) startCoronatioToastTimer(toast, Number(toast.dataset.toastRemaining || 0));
    }, true); document.body.addEventListener('animationend', event => {
      const toast = event.target.closest?.('[data-coronatio-toast]');
      if (toast && toast.classList.contains('toast-exit') && event.animationName === 'toast-slide-out') toast.remove();
    }); document.querySelector('[data-portal-add-form]')?.addEventListener('submit', submitPortalForm); document.querySelector('[data-cartridge-add-form]')?.addEventListener('submit', submitCartridgeForm);
    document.body.addEventListener('input', event => {
      const slider = event.target.closest('[data-ui-slider]');
      if (slider) { const item = slider.closest('.showcase-item'); const out = item?.querySelector('[data-slider-value]'); if (out) out.textContent = slider.value; const min = Number(slider.min); const max = Number(slider.max); const value = Number(slider.value); const percent = max > min ? ((value - min) / (max - min)) * 100 : 0; const fill = slider.closest('.ui-slider__container')?.querySelector('.ui-slider__fill'); if (fill) fill.style.width = `${Math.max(0, Math.min(100, percent))}%`; if (slider.matches('[data-dhcp-example-boundary]')) { const reservedEnd = value + 1; const poolStart = reservedEnd + 1; const reservedOut = item?.querySelector('[data-dhcp-example-reserved-end]'); const poolOut = item?.querySelector('[data-dhcp-example-pool-start]'); if (reservedOut) reservedOut.textContent = String(reservedEnd); if (poolOut) poolOut.textContent = String(poolStart); } return; }
      const upstreamInput = event.target.closest('[data-upstream-custom-addresses], [data-upstream-provider], [data-upstream-dot], [data-upstream-custom-toggle]'); if (upstreamInput) { const specimen = upstreamInput.closest('[data-test-upstream-resolver]'); if (specimen) { const custom = specimen.querySelector('[data-upstream-custom]'); const toggle = specimen.querySelector('[data-upstream-custom-toggle]'); if (custom && toggle) custom.hidden = !toggle.checked; renderTestUpstreamReadout(specimen); } return; }
      const time = event.target.closest('[data-ui-time-picker]');
      if (time) { const out = document.querySelector('[data-ui-time-output]'); if (out) out.textContent = time.value; }
    });
    document.body.addEventListener('change', event => {
      if (event.target.closest('[data-device-controls] input[type="checkbox"]')) { hydrateStats(); return; }
      const box = event.target.closest('.file-input, .ui-file-input');
      if (box && event.target.matches('input[type="file"]')) { const names = Array.from(event.target.files || []).map(file => file.name); const text = names.length ? names.join(', ') : 'No files selected'; const label = box.querySelector('[data-file-input-label]'); if (label) { if ('value' in label) label.value = text; else label.textContent = text; } const item = box.closest('.showcase-item'); const state = item?.querySelector('[data-test-file-state]'); const submit = item?.querySelector('[data-test-file-submit]'); if (state) state.textContent = text; if (submit) submit.disabled = names.length === 0; }
      const domainFile = event.target.closest('[data-test-domain-file]');
      if (domainFile) { const section = domainFile.closest('[data-test-domain-file-section]'); const names = Array.from(domainFile.files || []).map(file => file.name); const readback = section?.querySelector('[data-test-domain-file-name]'); const submit = section?.querySelector('[data-test-domain-submit]'); if (readback) readback.textContent = names.length ? names.join(', ') : 'No files selected'; if (submit) submit.disabled = names.length === 0; }
    }); let uxModalDemoOpener = null;
    const coronatioModalBackdrops = '[data-pin-modal-backdrop], [data-info-modal-backdrop], [data-upload-history-backdrop], [data-upload-blacklist-backdrop], [data-upload-pin-backdrop], [data-dhcp-modal-backdrop], [data-add-portal-modal], [data-add-tab-modal], [data-note-modal], [data-manager-modal], [data-hestia-certificate-modal], [data-ux-modal-demo-backdrop]';
    const coronatioModalCloseControls = '[data-pin-cancel], [data-info-modal-close], [data-upload-modal-close], [data-upload-pin-cancel], [data-dhcp-modal-cancel], [data-portal-modal-close], [data-add-tab-modal-close], [data-note-cancel], [data-manager-close], [data-hestia-certificate-close], [data-ux-modal-close]';
    const coronatioModalConfirmControls = '[data-pin-confirm-button], [data-upload-pin-confirm], [data-dhcp-modal-confirm], [data-note-confirm], [data-manager-confirm]';
    function openCoronatioModal() {
      return [...document.querySelectorAll(coronatioModalBackdrops)].filter(modal => !modal.hidden && modal.getAttribute('aria-hidden') !== 'true' && getComputedStyle(modal).display !== 'none').at(-1);
    }
    document.addEventListener('keydown', event => {
      const modal = openCoronatioModal();
      if (!modal) return;
      const target = event.target instanceof Element ? event.target : null;
      if (event.key === 'Escape') {
        const close = modal.querySelector(coronatioModalCloseControls);
        if (close) { event.preventDefault(); close.click(); }
        return;
      }
      if (event.key !== 'Enter' || !target || target.closest('textarea, button, a')) return;
      const confirm = modal.querySelector(coronatioModalConfirmControls);
      if (confirm && !confirm.disabled) { event.preventDefault(); confirm.click(); }
    });
    function openUxModalDemo(kind, opener) {
      const backdrop = document.querySelector('[data-ux-modal-demo-backdrop]');
      const win = document.querySelector('[data-ux-modal-demo-window]');
      const title = document.querySelector('[data-ux-modal-demo-title]');
      const body = document.querySelector('[data-ux-modal-demo-body]');
      if (!backdrop || !win || !title || !body) return;
      const specimen = {
        small: { size: 'small', title: 'Small confirmation', body: 'A compact confirmation or short choice.' },
        medium: { size: 'medium', title: 'Medium dialog', body: 'Regular dialog body for settings, details, and ordinary decisions.' },
        fullscreen: { size: 'fullscreen', title: 'Fullscreen workflow', body: 'Full-screen workflow surface for focused multi-step work.' },
        inspect: { size: 'medium', title: 'Data Generator inspection', body: 'Service inspection opens as a modal result.' },
        run: { size: 'medium', title: 'Run Health Monitor', body: 'Run confirmation opens as a modal result.' }
      }[kind] || { size: 'medium', title: 'Medium dialog', body: 'Regular dialog body.' };
      uxModalDemoOpener = opener instanceof HTMLElement ? opener : document.activeElement;
      win.classList.remove('small', 'medium', 'fullscreen');
      win.classList.add(specimen.size);
      title.textContent = specimen.title;
      body.textContent = specimen.body;
      backdrop.classList.add('open');
      backdrop.setAttribute('aria-hidden', 'false');
      document.body.classList.add('modal-open');
      requestAnimationFrame(() => win.querySelector('[data-ux-modal-close]')?.focus());
    }
    function closeUxModalDemo() {
      const backdrop = document.querySelector('[data-ux-modal-demo-backdrop]');
      if (!backdrop) return;
      backdrop.classList.remove('open');
      backdrop.setAttribute('aria-hidden', 'true');
      document.body.classList.remove('modal-open');
      const opener = uxModalDemoOpener; uxModalDemoOpener = null;
      if (opener instanceof HTMLElement && document.contains(opener)) opener.focus();
    }
"####
}
