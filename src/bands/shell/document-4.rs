fn shell_document_4() -> &'static str {
    r####"      if (!el) return;
      el.textContent = 'Loading ' + route + '…';
      try {
        const response = await fetch(route, { method });
        const text = await response.text();
        try { el.textContent = JSON.stringify(JSON.parse(text), null, 2); }
        catch (_) { el.textContent = text; }
      } catch (error) { el.textContent = 'fetch failed: ' + error; }
    }
    document.querySelectorAll('[data-fetch]').forEach(button => button.addEventListener('click', () => fetchInto(button.dataset.fetch, button.dataset.target, button.dataset.method || 'GET')));

    const uploadState = { currentPath: '/mnt/nas', selectedFiles: [], activeUploads: new Map(), pinRequired: false, uploading: false, blacklist: [], history: [] };
    const uploadFileInput = document.querySelector('[data-upload-file]');
    const uploadSubmit = document.querySelector('[data-upload-submit]');
    const uploadProgressList = document.querySelector('[data-upload-progress-list]');
    const uploadBreadcrumbs = document.querySelector('[data-upload-breadcrumbs]');
    const uploadReadout = document.getElementById('upload-readout');
    function uploadFormatSize(bytes) {
      const units = ['B', 'KB', 'MB', 'GB'];
      let size = Number(bytes || 0);
      let unit = 0;
      while (size >= 1024 && unit < units.length - 1) { size /= 1024; unit += 1; }
      return size.toFixed(1) + ' ' + units[unit];
    }
    function uploadStatusIcon(status) {
      if (status === 'pending') return '⏳';
      if (status === 'uploading') return '📤';
      if (status === 'completed') return '✅';
      if (status === 'error') return '❌';
      return '❓';
    }
    function uploadStatusColor(status) {
      if (status === 'pending') return '#f59e0b';
      if (status === 'uploading') return '#3b82f6';
      if (status === 'completed') return '#10b981';
      if (status === 'error') return '#ef4444';
      return '#6b7280';
    }
    function renderUploadProgress() {
      if (!uploadProgressList) return;
      const uploads = Array.from(uploadState.activeUploads.values());
      uploadProgressList.hidden = uploads.length === 0;
      uploadProgressList.innerHTML = uploads.map(upload => `
        <div class="upload-progress ${upload.status}" data-upload-progress="${upload.filename}">
          <div class="upload-header"><span class="status-icon">${uploadStatusIcon(upload.status)}</span><span class="filename">${upload.filename}</span><button type="button" class="remove-button" data-upload-remove="${upload.filename}" aria-label="Remove upload">×</button></div>
          <div class="progress-section"><div class="progress-bar-container" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow="${upload.progress.toFixed(1)}"><div class="progress-bar" style="width:${upload.progress}%;background-color:${uploadStatusColor(upload.status)};transition:width 0.3s ease-in-out"><span class="progress-text">${upload.progress.toFixed(1)}%</span></div></div><div class="upload-stats"><span class="size">${uploadFormatSize(upload.uploaded)} / ${uploadFormatSize(upload.total)}</span>${upload.status === 'uploading' ? `<span class="speed">${uploadFormatSize(upload.speed)}/s</span>` : ''}</div>${upload.status === 'error' ? `<div class="error-message">${upload.error || 'Upload failed'}</div>` : ''}</div>
        </div>`).join('');
      uploadProgressList.querySelectorAll('[data-upload-remove]').forEach(button => button.addEventListener('click', () => { uploadState.activeUploads.delete(button.dataset.uploadRemove); renderUploadProgress(); }));
    }
    function setUpload(filename, update) {
      const current = uploadState.activeUploads.get(filename) || { filename, progress: 0, speed: 0, uploaded: 0, total: 0, status: 'pending' };
      uploadState.activeUploads.set(filename, Object.assign(current, update));
      renderUploadProgress();
    }
    function setUploadSelection() {
      uploadState.selectedFiles = Array.from(uploadFileInput?.files || []);
      if (uploadSubmit) uploadSubmit.disabled = uploadState.selectedFiles.length === 0 || uploadState.uploading;
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
      if (!uploadBreadcrumbs) return;
      const parts = String(path || '/mnt/nas').split('/').filter(Boolean).slice(1);
      let current = '/mnt/nas';
      const crumbs = [{ name: 'nas', path: '/mnt/nas' }];
      parts.slice(1).forEach(part => { current += '/' + part; crumbs.push({ name: part, path: current }); });
      uploadBreadcrumbs.innerHTML = crumbs.map((crumb, index) => `<span class="breadcrumb-item ${crumb.path === path ? 'current' : ''}" data-path="${crumb.path}">${crumb.name}</span>${index < crumbs.length - 1 ? '<span class="breadcrumb-separator"> / </span>' : ''}`).join('');
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
    });
    async function uploadOneFile(file) {
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
          if (xhr.status >= 200 && xhr.status < 300) { setUpload(file.name, { progress: 100, uploaded: file.size, total: file.size, status: 'completed' }); if (uploadReadout) uploadReadout.textContent = xhr.responseText; resolve(); }
          else { let msg = 'Upload failed with status ' + xhr.status; try { const body = JSON.parse(xhr.responseText); if (body.error) msg = body.error; } catch (_) {} setUpload(file.name, { status: 'error', error: msg }); reject(new Error(msg)); }
        };
        xhr.onerror = () => { const msg = 'Network error occurred during upload'; setUpload(file.name, { status: 'error', error: msg }); reject(new Error(msg)); };
        xhr.open('POST', '/api/files/upload');
        xhr.send(form);
      });
    }
    async function uploadSelectedFiles() {
      if (!uploadState.selectedFiles.length) return;
      uploadState.uploading = true;
      if (uploadSubmit) { uploadSubmit.disabled = true; uploadSubmit.textContent = 'Uploading...'; }
      let success = 0;
      let failed = 0;
      for (const file of uploadState.selectedFiles) {
        try { await uploadOneFile(file); success += 1; }
        catch (_) { failed += 1; }
      }
      uploadState.uploading = false;
      if (uploadSubmit) { uploadSubmit.textContent = 'Upload Selected Files'; uploadSubmit.disabled = uploadState.selectedFiles.length === 0; }
      if (uploadReadout) uploadReadout.textContent = failed ? `Uploaded ${success} file(s), ${failed} failed` : `Successfully uploaded ${success} file(s)`;
    }
    function openUploadModal(selector) { const modal = document.querySelector(selector); if (modal) modal.hidden = false; }
    async function refreshUploadHistory() {
      const modal = document.querySelector('[data-upload-history-modal]');
      const list = modal?.querySelector('.upload-history-list');
      const empty = modal?.querySelector('.uploadHistoryModal');
      const clear = modal?.querySelector('[data-upload-clear-history]');
      try { const data = await fetch('/api/upload/history').then(r => r.json()); uploadState.history = data.history || []; } catch (_) { uploadState.history = []; }
      if (list) { list.hidden = uploadState.history.length === 0; list.innerHTML = uploadState.history.map(line => `<div class="history-item ${String(line).includes('Successfully') ? 'success' : 'error'}">${line}</div>`).join(''); }
      if (empty) empty.hidden = uploadState.history.length !== 0;
      if (clear) clear.disabled = uploadState.history.length === 0;
    }
    async function refreshUploadBlacklist() {
      const entries = document.querySelector('[data-upload-blacklist-entries]');
      try { const data = await fetch('/api/upload/blacklist/list').then(r => r.json()); uploadState.blacklist = data.blacklist || []; } catch (_) { uploadState.blacklist = []; }
      if (entries) entries.innerHTML = uploadState.blacklist.map((entry, index) => `<div class="blacklist-entry"><span class="entry-path">${entry}</span><button type="button" class="remove-entry" data-blacklist-remove="${index}" aria-label="Remove entry">×</button></div>`).join('');
      entries?.querySelectorAll('[data-blacklist-remove]').forEach(button => button.addEventListener('click', () => { uploadState.blacklist.splice(Number(button.dataset.blacklistRemove), 1); refreshUploadBlacklistDomOnly(); }));
    }
    function refreshUploadBlacklistDomOnly() { const entries = document.querySelector('[data-upload-blacklist-entries]'); if (entries) entries.innerHTML = uploadState.blacklist.map((entry, index) => `<div class="blacklist-entry"><span class="entry-path">${entry}</span><button type="button" class="remove-entry" data-blacklist-remove="${index}" aria-label="Remove entry">×</button></div>`).join(''); }
    uploadFileInput?.addEventListener('change', setUploadSelection);
    uploadSubmit?.addEventListener('click', uploadSelectedFiles);
    document.querySelector('[data-upload-refresh]')?.addEventListener('click', () => { window.htmx?.ajax('GET', '/admit/upload/tree?path=%2Fmnt%2Fnas&depth=0&selected=' + encodeURIComponent(uploadCurrentPath()), { target: '[data-upload-tree]', swap: 'innerHTML' }); });
    document.querySelector('[data-upload-force-allow]')?.addEventListener('click', async () => { if (!confirm(`WARNING: This will override security settings for ${uploadState.currentPath}. 
Only continue if you understand the risks.`)) return; await fetch('/api/upload/force-permissions', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ directory: uploadState.currentPath }) }); });
    document.querySelector('[data-upload-set-default]')?.addEventListener('click', () => fetch('/api/upload/default-directory', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ directory: uploadState.currentPath }) }));
    document.querySelector('[data-upload-history]')?.addEventListener('click', async () => { openUploadModal('[data-upload-history-modal]'); await refreshUploadHistory(); });
    document.querySelector('[data-upload-blacklist]')?.addEventListener('click', async () => { openUploadModal('[data-upload-blacklist-modal]'); await refreshUploadBlacklist(); });
    document.querySelector('[data-upload-blacklist-add]')?.addEventListener('click', () => { const input = document.querySelector('[data-upload-blacklist-input]'); const next = input?.value?.trim(); if (!next || uploadState.blacklist.includes(next)) return; uploadState.blacklist.push(next); input.value = ''; refreshUploadBlacklistDomOnly(); });
    document.querySelector('[data-upload-blacklist-submit]')?.addEventListener('click', () => fetch('/api/upload/blacklist/update', { method: 'PUT', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ blacklist: uploadState.blacklist }) }).then(() => window.htmx?.ajax('GET', '/admit/upload/tree?path=%2Fmnt%2Fnas&depth=0&selected=' + encodeURIComponent(uploadCurrentPath()), { target: '[data-upload-tree]', swap: 'innerHTML' })));
    document.querySelector('[data-upload-clear-history]')?.addEventListener('click', () => fetch('/api/upload/history/clear', { method: 'POST' }).then(refreshUploadHistory));
    document.querySelector('[data-upload-pin-toggle]')?.addEventListener('click', async event => { uploadState.pinRequired = !uploadState.pinRequired; event.currentTarget.classList.toggle('active', uploadState.pinRequired); event.currentTarget.setAttribute('aria-label', `Toggle PIN requirement (currently ${uploadState.pinRequired ? 'enabled' : 'disabled'})`); await fetch('/api/upload/pin-required-status', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ isPinRequired: uploadState.pinRequired }) }); });
    fetch('/api/upload/pin-required-status').then(r => r.json()).then(data => { uploadState.pinRequired = !!data.isPinRequired; const b = document.querySelector('[data-upload-pin-toggle]'); b?.classList.toggle('active', uploadState.pinRequired); }).catch(() => {});
    syncUploadTreeSelection();

    let uptimeBaseSeconds = null;
    let uptimeBaseStamp = 0;
    function secondsFromUptimeText(text) {
      const raw = String(text || '');
      const days = Number((raw.match(/(\d+)d/) || [0, 0])[1]);
      const hours = Number((raw.match(/(\d+)h/) || [0, 0])[1]);
      const minutes = Number((raw.match(/(\d+)m/) || [0, 0])[1]);
      const seconds = Number((raw.match(/(\d+)s/) || [0, 0])[1]);
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
    function tickUptime() {
      const uptime = document.querySelector('[data-uptime-indicator]');
      if (!uptime || uptimeBaseSeconds === null) return;
      const elapsed = Math.floor((Date.now() - uptimeBaseStamp) / 1000);
      uptime.textContent = formatUptimeSeconds(uptimeBaseSeconds + elapsed);
      uptime.dataset.uptimeIncludesSeconds = 'true';
    }
    async function hydrateUptime() {
      const uptime = document.querySelector('[data-uptime-indicator]');
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
    function chartTicks(color, callback) { return { color, maxTicksLimit: 10, autoSkip: true, callback }; }
    function chartGrid() { return { color: 'var(--border)', borderDash: [3, 3] }; }
    function chartTooltip(labelFormatter) {
      return { enabled: true, mode: 'index', intersect: false, callbacks: { label: labelFormatter } };
    }
    function chartCommonOptions() {
      return {
        responsive: true,
        maintainAspectRatio: false,
        interaction: { mode: 'index', intersect: false },
        layout: { padding: { left: 10, right: 10, top: 20, bottom: 20 } },
        animation: { duration: 0 }
      };
    }
    function lineDataset(label, data, color, yAxisID) {
      return { label, data, borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4, yAxisID };
    }
    function createCPUChart(ctx, labels, cpuData, tempData) {
      destroyStatsChart('cpu');
      statsCharts.cpu = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          lineDataset('CPU Usage', cpuData, '#4A5568', 'y-cpu'),
          lineDataset('Temperature', tempData, '#90cff3', 'y-temp')
        ] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { tooltip: chartTooltip(context => context.dataset.label + ': ' + Number(context.parsed.y || 0).toFixed(1) + (context.dataset.yAxisID === 'y-temp' ? '°C' : '%')), legend: { position: 'bottom', align: 'center', labels: { usePointStyle: true, pointStyle: 'circle', boxWidth: 8, boxHeight: 8 } } },
          scales: {
            x: { ticks: chartTicks('var(--hiddenTabText)', value => labels[value] || value), grid: { display: false } },
            'y-cpu': { type: 'linear', display: true, position: 'left', beginAtZero: true, max: 100, ticks: chartTicks('var(--hiddenTabText)', value => Number(value).toFixed(0) + '%'), grid: chartGrid() },
            'y-temp': { type: 'linear', display: true, position: 'right', beginAtZero: true, max: 100, ticks: chartTicks('var(--hiddenTabText)', value => Number(value).toFixed(0) + '°C'), grid: { display: false } }
          }
        })
      });
    }
    function createNetworkChart(ctx, labels, downloadData, uploadData) {
      destroyStatsChart('network');
      const networkMax = Math.max(1, ...downloadData, ...uploadData) * 1.1;
      const networkTicks = { color: 'var(--hiddenTabText)', maxTicksLimit: 10, autoSkip: true, callback: value => fmtBytes(value) + '/s' };
      statsCharts.network = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          lineDataset('Download Speed', downloadData, '#4A5568', 'y'),
          lineDataset('Upload Speed', uploadData, '#90cff3', 'y-right')
        ] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { tooltip: chartTooltip(context => context.dataset.label + ': ' + fmtBytes(context.parsed.y) + '/s'), legend: { position: 'bottom', align: 'center', labels: { usePointStyle: true, pointStyle: 'circle', boxWidth: 8, boxHeight: 8 } } },
          scales: {
            x: { ticks: chartTicks('var(--hiddenTabText)', value => labels[value] || value), grid: { display: false } },
            y: { beginAtZero: true, suggestedMin: 0, max: networkMax, ticks: networkTicks, grid: chartGrid() },
            'y-right': { beginAtZero: true, suggestedMin: 0, max: networkMax, position: 'right', ticks: networkTicks, grid: { display: false } }
          }
        })
      });
    }
    function diskDisplayName(device) {
      const mount = device.mount || '';
      if (mount === '/mnt/nas') return 'nas';
      if (mount === '/mnt/nasbackup') return 'nasbackup';
      if ((device.device || '').includes('sda6')) return 'sda6';
      return mount.replace(/^\/mnt\//, '') || device.device || 'disk';
    }
    function createIOChart(ctx, labels, datasets) {
      destroyStatsChart('io');
      statsCharts.io = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets },
        options: Object.assign(chartCommonOptions(), {
          plugins: { legend: { display: false }, tooltip: { mode: 'index', intersect: false, callbacks: { label: context => context.dataset.label + ': ' + Number(context.parsed.y || 0).toFixed(2) + ' MB/s' } } },
          scales: {
            x: { ticks: { maxRotation: 0, autoSkip: true, maxTicksLimit: 6, callback: value => labels[value] || value } },
            y: { ticks: { callback: value => Number(value).toFixed(2) } }
          }
        })
      });
    }
    function renderCpuChart(data) {
      const ctx = document.getElementById('cpuChart');
      if (ctx && window.Chart) createCPUChart(ctx, statsChartState.labels, statsChartState.cpu, statsChartState.temp);
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
      return true;
    }
    function renderNetwork(data) {
      const ctx = document.getElementById('networkChart');
      if (ctx && window.Chart) createNetworkChart(ctx, statsChartState.labels, statsChartState.download, statsChartState.upload);
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
      const controls = document.querySelector('[data-device-controls]');
      const devices = data.io?.devices || [];
      if (controls) controls.innerHTML = devices.map(device => { const name = diskDisplayName(device); return `<div class="device-control" data-io-device="${name}"><div class="device-name">${name}</div><div class="device-checkboxes"><label class="drive-checkbox"><input type="checkbox" name="read-${name}" value="${name}" checked>Read</label><label class="drive-checkbox"><input type="checkbox" name="write-${name}" value="${name}" checked>Write</label></div></div>`; }).join('') || '<div class="io-loading"><p>Loading disk I/O data...</p></div>';
      const colors = ['#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40'];
      const datasets = devices.flatMap((device, index) => {
        const key = device.device || device.mount;
        const name = diskDisplayName(device);
        const series = statsChartState.ioSeries[key] || { read: [0], write: [0] };
        const color = colors[index % colors.length];
        return [
          { label: `${name} Read`, data: series.read.map(value => Number(value || 0) / (1024 * 1024)), borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0 },
          { label: `${name} Write`, data: series.write.map(value => Number(value || 0) / (1024 * 1024)), borderColor: color, backgroundColor: color, borderWidth: 2, borderDash: [5, 5], fill: false, pointRadius: 0 }
        ];
      });
      const ctx = document.getElementById('io-chart');
      if (ctx && window.Chart) createIOChart(ctx, statsChartState.labels, datasets);
      const legend = document.getElementById('io-chart-legend');
      if (legend) legend.innerHTML = [...new Set(datasets.map(dataset => dataset.label.split(' ')[0]))].map(name => `<span>${name} Read · Write</span>`).join('');
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
    function renderKeaLeases(data) {
      const tbody = document.querySelector('[data-kea-leases]');
      if (!tbody) return;
      if (data.keaLeases && !data.leases) {
        tbody.innerHTML = '<tr><td colspan="4">No Kea leases available.</td></tr>';
        return;
      }
      const leases = data.leases || [];
      tbody.innerHTML = leases.length ? leases.map(lease => `<tr><td class="device-note-cell" data-label="Note:"><span class="note-text">${lease.note || ''}</span><button class="edit-note-button" data-admin-only="true" title="Edit device note">✎</button></td><td data-label="Hostname:">${lease.hostname || 'N/A'}</td><td data-label="IP:">${lease.ip}</td><td data-label="MAC:" title="${lease.mac}">${lease.mac}</td></tr>`).join('') : '<tr><td colspan="4">No Kea leases found.</td></tr>';
    }
    function renderProcesses(data) {
      const target = document.querySelector('[data-process-usage-list]');
      if (!target) return;
      const processes = data.processes || [];
      target.innerHTML = processes.length ? processes.map(process => `<div class="process-bar" title="Process: ${process.name}\nMemory: ${fmtBytes(process.memoryBytes)}\nCPU: ${Number(process.cpuPercent || 0).toFixed(1)}%\nInstances: ${process.processCount || 1}"><div class="process-bar-fill" style="width:${Math.max(Number(process.cpuPercent || 0), 1)}%"></div><div class="process-text-container"><span class="process-name">${process.name}</span><span class="process-usage">${Number(process.cpuPercent || 0).toFixed(1)}%</span></div></div>`).join('') : '<div class="process-usage-empty"><p>No process data available</p></div>';
    }
    async function hydrateStats() {
      try {
        const token = localStorage.getItem('coronatioAdminToken');
        const headers = token ? { 'X-Admin-Token': token } : {};
        // Allowlisted chart data lane: fetch('/api/stats') with session headers and no-store cache.
        const data = await fetch('/api/stats', { headers, cache: 'no-store' }).then(r => r.json());
        const label = formatChartTime();
        pushChartPoint(label, data);
        renderCpuChart(data);
        renderNetwork(data);
        renderDiskIo(data);
        renderMemory(data);
        renderDiskUsage(data);
        renderKeaLeases(data);
        renderProcesses(data);
      } catch (error) {
        document.querySelector('[data-process-usage-list]').innerHTML = '<div class="process-usage-empty"><p>' + String(error) + '</p></div>';
      }
    }

    function escapeHtml(value) {
      return String(value ?? '').replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[char]));
    }
    function portalDestination(portal) {
      return portal.localURL || portal.remoteURL || '#';
    }
    function renderPortalCard(portal, factoryNames) {
      const destination = portalDestination(portal);
      const serviceData = encodeURIComponent(JSON.stringify(portal.services || []));
      const adminControls = portal.type === 'link' ? '' : `<div class="admin-controls" data-admin-only data-admin-viewport="portals" data-portal-services="${serviceData}">
        <div class="admin-controls-row"><button data-service-action="start" title="Start service">Start</button><button data-service-action="stop" title="Stop service">Stop</button><button data-service-action="restart" title="Restart service">Restart</button></div>
        <div class="admin-controls-row"><button data-service-action="enable" title="Enable service at boot">Enable</button><button data-service-action="disable" title="Disable service at boot">Disable</button><button data-service-action="status" title="Check service status">Status</button></div>
      </div>`;
      const isVisible = portal.visible !== false;
      const visibilityToggle = `<button type="button" class="visibility-toggle" data-admin-only data-admin-viewport="portals" data-portal-visibility-toggle data-visible="${isVisible}" aria-label="${isVisible ? 'Hide' : 'Show'} ${escapeHtml(portal.name)}">${isVisible ? '👁' : '🙈'}</button>`;
      return `<div class="portal-element" data-portal-element data-visible="${isVisible}" style="position:relative">
        ${visibilityToggle}
        <article class="portal-card ${escapeHtml(portal.status || 'unknown')}" data-portal-card data-portal-name="${escapeHtml(portal.name)}" data-portal-url="${escapeHtml(destination)}" role="link" tabindex="0">
          <div class="portal-card-header">
            <img src="/api/portals/images/${encodeURIComponent(portal.name)}.png" alt="${escapeHtml(portal.name)} icon" class="portal-icon" onerror="this.onerror=null;this.src='/api/portals/images/default.png';">
            <h2 class="portal-name">${escapeHtml(portal.name)}</h2>
            <p class="portal-description">${escapeHtml(portal.description || '')}</p>
          </div>
          <div class="portal-meta">${adminControls}</div>
        </article>
      </div>`;
    }

    function renderAddPortalCard() {
      return `<div class="portal-card add-portal-card" data-admin-only data-admin-viewport="portals" data-add-portal-open role="button" tabindex="0" aria-label="Add new portal">
        <div class="add-portal-content"><div class="add-portal-icon"><i class="fas fa-plus"></i></div><h3 class="add-portal-title">Add Portal</h3><p class="add-portal-description">Create a new portal for your services</p></div>
      </div>`;
    }
    function openPortalModal(selector) { const modal = document.querySelector(selector); if (modal) modal.hidden = false; }
    function closePortalModals() { document.querySelectorAll('[data-add-portal-modal], [data-service-status-modal]').forEach(modal => { modal.hidden = true; }); }
    function showPortalServiceStatus(results) {
      const modal = document.querySelector('[data-service-status-modal]');
      const content = modal?.querySelector('[data-service-status-content]');
      if (content) content.textContent = JSON.stringify(results, null, 2);
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
        showPortalServiceStatus([{ action, error: 'No services specified for this portal.' }]);
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
          try { results.push(JSON.parse(text)); } catch (_) { results.push({ service, action, raw: text }); }
        } catch (error) {
          results.push({ service, action, error: String(error) });
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
    }
    async function refreshElementFragment(tabId) {
      const token = localStorage.getItem('coronatioAdminToken');
      const headers = token ? { 'X-Admin-Token': token } : {};
      const route = tabId === 'portals' ? '/api/portals/elements' : '/api/stats/elements';
      const target = tabId === 'portals' ? document.querySelector('[data-portals-grid]') : document.querySelector('[data-stats-viewport]');
      if (!target) return;
      const response = await fetch(route, { headers, cache: 'no-store' });
      if (!response.ok) return;
      target.innerHTML = await response.text();
      if (tabId === 'portals') bindPortalFragmentControls(target);
      if (tabId === 'stats') hydrateStats();
      applyAdminDomState();
    }
    async function toggleElementVisibility(tabId, elementId, visible) {
      const token = localStorage.getItem('coronatioAdminToken');
      const response = await fetch('/api/tabs/elements', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...(token ? { 'X-Admin-Token': token } : {}) },
        body: JSON.stringify({ tabId, elementId, visibility: visible })
      });
      const html = await response.text();
      if (!response.ok) return;
      const target = tabId === 'portals' ? document.querySelector('[data-portals-grid]') : document.querySelector('[data-stats-viewport]');
      if (!target) return;
      target.innerHTML = html;
      if (tabId === 'portals') bindPortalFragmentControls(target);
      if (tabId === 'stats') hydrateStats();
      applyAdminDomState();
    }
    async function hydratePortals() {
      const grid = document.querySelector('[data-portals-grid]');
      if (!grid) return;
      try { await refreshElementFragment('portals'); }
      catch (error) { grid.innerHTML = '<article class="portal-card error portal-error"><h2>Portals unavailable</h2><p>homeserver.json could not be read.</p></article>' + renderAddPortalCard(); }
    }

    async function hydrateFavoriteManifest() {
      try {
        const favorite = await fetch('/api/favorites').then(r => r.json());
        if (favorite?.starredTab) {
          tabState.starredTab = favorite.starredTab;
          saveTabState(tabState);
          setStarredTab(favorite.starredTab);
        }
      } catch (_) { setStarredTab(tabState.starredTab); }
      showPane((location.hash || '#' + (tabState.starredTab || firstVisibleTab())).slice(1));
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
    document.body.addEventListener('click', event => {
      const scopedTab = event.target.closest('[data-tab-id]');
      if (scopedTab) return switchScopedTabs(scopedTab);
      const portalEye = event.target.closest('[data-portal-visibility-toggle]');
      if (portalEye) { event.preventDefault(); event.stopPropagation(); toggleElementVisibility('portals', portalEye.dataset.portalVisibilityToggle, portalEye.dataset.visible !== 'true'); return; }
      const statEye = event.target.closest('[data-stat-visibility-toggle]');
      if (statEye) { event.preventDefault(); event.stopPropagation(); toggleElementVisibility('stats', statEye.dataset.statVisibilityToggle, statEye.dataset.visible !== 'true'); return; }
      const addPortal = event.target.closest('[data-add-portal-open]');
      if (addPortal) { openPortalModal('[data-add-portal-modal]'); return; }
      const portalModalClose = event.target.closest('[data-portal-modal-close]');
      if (portalModalClose) { closePortalModals(); return; }
      const portalBackdrop = event.target.closest('[data-add-portal-modal], [data-service-status-modal]');
      if (portalBackdrop && event.target === portalBackdrop) { closePortalModals(); return; }
      const copyStatus = event.target.closest('[data-service-status-copy]');
      if (copyStatus) { navigator.clipboard?.writeText(document.querySelector('[data-service-status-content]')?.textContent || ''); return; }
      const fileButton = event.target.closest('[data-upload-file-button]');
      if (fileButton) { uploadFileInput?.click(); return; }
      const health = event.target.closest('[data-test-health-check]');
      if (health) {
        const out = document.querySelector('[data-test-health-output]');
        if (out) out.textContent = JSON.stringify({ schema: 'coronatio.test.health.v1', status: 'ready', dependencies: { rust_shell: true, theme_catalog: Boolean(themeCatalog?.themes), ux_library: true }, theme: headerState.theme }, null, 2);
        return;
      }
      const reset = event.target.closest('[data-reset-breadcrumbs]');
      if (reset) { const out = document.querySelector('[data-breadcrumb-path]'); if (out) out.textContent = '/mnt/nas'; return; }
      const expand = event.target.closest('[data-test-card-expand]');
      if (expand) { const expanded = expand.closest('.test-card')?.querySelector('.test-card-expanded'); if (expanded) { expanded.hidden = !expanded.hidden; expand.textContent = expanded.hidden ? '+' : '−'; } return; }
      const modalOpen = event.target.closest('[data-ux-modal-open]');
      if (modalOpen) return openUxModalDemo(modalOpen.dataset.uxModalOpen);
      const modalClose = event.target.closest('[data-ux-modal-close]');
      if (modalClose) return closeUxModalDemo();
      const backdrop = event.target.closest('[data-ux-modal-demo-backdrop]');
      if (backdrop && event.target === backdrop) return closeUxModalDemo();
    });
    document.querySelector('[data-portal-add-form]')?.addEventListener('submit', event => { event.preventDefault(); });
    document.body.addEventListener('input', event => {
      const slider = event.target.closest('[data-ui-slider]');
      if (slider) { const out = slider.closest('.showcase-item')?.querySelector('[data-slider-value]'); if (out) out.textContent = slider.value; return; }
      const time = event.target.closest('[data-ui-time-picker]');
      if (time) { const out = document.querySelector('[data-ui-time-output]'); if (out) out.textContent = time.value; }
    });
    document.body.addEventListener('change', event => {
      const box = event.target.closest('.file-input');
      if (box && event.target.matches('input[type="file"]')) { const label = box.querySelector('[data-file-input-label]'); if (label) label.textContent = event.target.files?.[0]?.name || 'Choose file'; }
    });
    function openUxModalDemo(size) {
      const backdrop = document.querySelector('[data-ux-modal-demo-backdrop]');
      const win = document.querySelector('[data-ux-modal-demo-window]');
      const title = document.querySelector('[data-ux-modal-demo-title]');
      const body = document.querySelector('[data-ux-modal-demo-body]');
      if (!backdrop || !win || !title || !body) return;
      const copy = {
        small: ['Small modal', 'Compact confirmation or short choice.'],
        medium: ['Medium modal', 'Regular dialog body for settings, details, and ordinary decisions.'],
        fullscreen: ['Fullscreen modal', 'Full-screen workflow surface for focused multi-step work.']
      }[size] || ['Medium modal', 'Regular dialog body.'];
      win.classList.remove('small', 'medium', 'fullscreen');
      win.classList.add(size === 'small' || size === 'fullscreen' ? size : 'medium');
      title.textContent = copy[0];
      body.textContent = copy[1];
      backdrop.classList.add('open');
      backdrop.setAttribute('aria-hidden', 'false');
    }
    function closeUxModalDemo() {
      const backdrop = document.querySelector('[data-ux-modal-demo-backdrop]');
      if (!backdrop) return;
      backdrop.classList.remove('open');
      backdrop.setAttribute('aria-hidden', 'true');
    }
    // modal open/close/backdrop clicks handled by the delegated body click listener above (survives HTMX swaps)
    function hydrateThemeTruth() {
      const target = document.querySelector('[data-theme-token-readout]');
      if (!target) return;
      const computed = getComputedStyle(document.documentElement);
      const tokens = [
        ['--primary', 'dark.json primary #323840'],
        ['--primaryHover', 'dark.json primaryHover #6B7280'],
        ['--success', 'dark.json success #10B981'],
        ['--status-up', 'dark.json statusUp #10B981'],
        ['--accent', 'dark.json accent #A78BFA'],
        ['--theme-control-height', 'theme sizing token'],
        ['--theme-font-family', 'theme font token']
      ];
      target.innerHTML = tokens.map(([token, source]) => `<tr><td>${token}</td><td>${computed.getPropertyValue(token).trim()}</td><td>${source}</td></tr>`).join('');
    }
    function hydrateThemeTokenLab() {
      const root = document.documentElement;
      document.querySelectorAll('[data-theme-token-slider]').forEach(slider => {
        const token = slider.dataset.themeTokenSlider;
        const row = slider.closest('[data-theme-token-control]');
        const unit = row?.dataset.themeTokenUnit || '';
        const output = document.querySelector(`[data-theme-token-output="${token}"]`);
        const apply = () => {
          const value = `${slider.value}${unit}`;
          root.style.setProperty(token, value);
          if (output) output.textContent = value;
          row?.setAttribute('data-theme-token-current', value);
        };
        slider.addEventListener('input', apply);
        apply();
      });
    }
    hydrateFavoriteManifest();
    hydrateThemeTruth();
    hydrateThemeTokenLab();
    hydrateUptime();
    hydrateStats();
    hydratePortals();
    setInterval(tickUptime, 1000);
    setInterval(hydrateStats, 5000);
  </script>
</body>
</html>"####
}
