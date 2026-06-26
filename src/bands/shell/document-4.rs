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
    const uploadTree = document.querySelector('[data-upload-tree]');
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
          <div class="progress-section"><div class="progress-bar-container"><div class="progress-bar" style="width:${upload.progress}%;background-color:${uploadStatusColor(upload.status)}"><span class="progress-text">${upload.progress.toFixed(1)}%</span></div></div><div class="upload-stats"><span class="size">${uploadFormatSize(upload.uploaded)} / ${uploadFormatSize(upload.total)}</span>${upload.status === 'uploading' ? `<span class="speed">${uploadFormatSize(upload.speed)}/s</span>` : ''}</div>${upload.status === 'error' ? `<div class="error-message">${upload.error || 'Upload failed'}</div>` : ''}</div>
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
    }
    function renderUploadBreadcrumbs(path) {
      if (!uploadBreadcrumbs) return;
      const parts = path.split('/').filter(Boolean).slice(1);
      let current = '/mnt/nas';
      const crumbs = [{ name: 'nas', path: '/mnt/nas' }];
      parts.slice(1).forEach(part => { current += '/' + part; crumbs.push({ name: part, path: current }); });
      uploadBreadcrumbs.innerHTML = crumbs.map((crumb, index) => `<span class="breadcrumb-item ${crumb.path === path ? 'current' : ''}" data-path="${crumb.path}">${crumb.name}</span>${index < crumbs.length - 1 ? '<span class="breadcrumb-separator"> / </span>' : ''}`).join('');
      uploadBreadcrumbs.querySelectorAll('.breadcrumb-item').forEach(item => item.addEventListener('click', () => selectUploadPath(item.dataset.path || '/mnt/nas')));
    }
    function selectUploadPath(path) {
      uploadState.currentPath = path || '/mnt/nas';
      renderUploadBreadcrumbs(uploadState.currentPath);
      uploadTree?.querySelectorAll('.directory-entry').forEach(row => {
        const selected = row.dataset.directoryPath === uploadState.currentPath;
        row.classList.toggle('selected', selected);
        row.setAttribute('aria-selected', selected ? 'true' : 'false');
        const mark = row.querySelector('.entry-selected');
        if (mark) mark.hidden = !selected;
      });
    }
    function renderDirectoryEntries(entries, depth = 0) {
      return (entries || []).map(entry => `<div class="directory-entry ${entry.path === uploadState.currentPath ? 'selected' : ''}" data-directory-path="${entry.path}" role="treeitem" aria-selected="${entry.path === uploadState.currentPath}" aria-expanded="${entry.hasChildren ? !!entry.isExpanded : 'false'}" style="padding-left:${24 * depth + 12}px">${depth > 0 ? '<div class="tree-line horizontal"></div>' : ''}<span class="expand-control" aria-label="${entry.isExpanded ? 'Collapse' : 'Expand'}">${entry.hasChildren ? (entry.isLoading ? '⟳' : (entry.isExpanded ? '▼' : '▶')) : ''}</span><span class="entry-icon">📁</span><span class="entry-name">${entry.name}</span><span class="entry-selected" aria-hidden="true" ${entry.path === uploadState.currentPath ? '' : 'hidden'}>✓</span></div>${entry.isExpanded ? renderDirectoryEntries(entry.children || [], depth + 1) : ''}`).join('');
    }
    async function loadUploadDirectory(path = '/mnt/nas', expand = false) {
      const loading = document.querySelector('[data-upload-directory-loading]');
      const error = document.querySelector('[data-upload-directory-error]');
      if (loading) loading.hidden = false;
      if (error) error.hidden = true;
      try {
        const data = await fetch('/api/files/browse-hierarchical?path=' + encodeURIComponent(path) + '&expand=' + expand).then(r => r.json());
        const entries = data.entries && data.entries.length ? data.entries : [{ name: 'nas', path: '/mnt/nas', type: 'directory', hasChildren: true, isExpanded: false, children: [] }];
        if (uploadTree) uploadTree.innerHTML = renderDirectoryEntries(entries);
        uploadTree?.querySelectorAll('.directory-entry').forEach(row => row.addEventListener('click', event => { event.stopPropagation(); selectUploadPath(row.dataset.directoryPath || '/mnt/nas'); }));
        selectUploadPath(uploadState.currentPath);
      } catch (err) {
        if (error) { error.hidden = false; error.textContent = '⚠️ NAS Storage Unavailable'; }
      } finally { if (loading) loading.hidden = true; }
    }
    async function uploadOneFile(file) {
      setUpload(file.name, { filename: file.name, progress: 0, speed: 0, uploaded: 0, total: file.size, status: 'pending' });
      const form = new FormData();
      form.append('file', file);
      form.append('path', uploadState.currentPath);
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
    document.querySelector('[data-upload-refresh]')?.addEventListener('click', () => loadUploadDirectory('/mnt/nas', false));
    document.querySelector('[data-upload-force-allow]')?.addEventListener('click', async () => { if (!confirm(`WARNING: This will override security settings for ${uploadState.currentPath}. 
Only continue if you understand the risks.`)) return; await fetch('/api/upload/force-permissions', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ directory: uploadState.currentPath }) }); });
    document.querySelector('[data-upload-set-default]')?.addEventListener('click', () => fetch('/api/upload/default-directory', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ directory: uploadState.currentPath }) }));
    document.querySelector('[data-upload-history]')?.addEventListener('click', async () => { openUploadModal('[data-upload-history-modal]'); await refreshUploadHistory(); });
    document.querySelector('[data-upload-blacklist]')?.addEventListener('click', async () => { openUploadModal('[data-upload-blacklist-modal]'); await refreshUploadBlacklist(); });
    document.querySelector('[data-upload-blacklist-add]')?.addEventListener('click', () => { const input = document.querySelector('[data-upload-blacklist-input]'); const next = input?.value?.trim(); if (!next || uploadState.blacklist.includes(next)) return; uploadState.blacklist.push(next); input.value = ''; refreshUploadBlacklistDomOnly(); });
    document.querySelector('[data-upload-blacklist-submit]')?.addEventListener('click', () => fetch('/api/upload/blacklist/update', { method: 'PUT', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ blacklist: uploadState.blacklist }) }).then(() => loadUploadDirectory('/mnt/nas', true)));
    document.querySelector('[data-upload-clear-history]')?.addEventListener('click', () => fetch('/api/upload/history/clear', { method: 'POST' }).then(refreshUploadHistory));
    document.querySelector('[data-upload-pin-toggle]')?.addEventListener('click', async event => { uploadState.pinRequired = !uploadState.pinRequired; event.currentTarget.classList.toggle('active', uploadState.pinRequired); event.currentTarget.setAttribute('aria-label', `Toggle PIN requirement (currently ${uploadState.pinRequired ? 'enabled' : 'disabled'})`); await fetch('/api/upload/pin-required-status', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ isPinRequired: uploadState.pinRequired }) }); });
    fetch('/api/upload/pin-required-status').then(r => r.json()).then(data => { uploadState.pinRequired = !!data.isPinRequired; const b = document.querySelector('[data-upload-pin-toggle]'); b?.classList.toggle('active', uploadState.pinRequired); }).catch(() => {});
    loadUploadDirectory('/mnt/nas', false);

    async function hydrateUptime() {
      const uptime = document.querySelector('[data-uptime-indicator]');
      if (!uptime) return;
      try {
        const data = await fetch('/uptime').then(r => r.json()).catch(() => null);
        uptime.textContent = data?.uptime ? data.uptime + 's' : 'connecting...';
      } catch (_) { uptime.textContent = navigator.onLine ? 'connecting...' : 'disconnected'; }
    }
    function fmtBytes(value) {
      if (value === null || value === undefined) return '—';
      const units = ['B', 'KB', 'MB', 'GB', 'TB'];
      let next = Number(value);
      let unit = 0;
      while (next >= 1024 && unit < units.length - 1) { next = next / 1024; unit += 1; }
      return next.toFixed(next >= 10 || unit === 0 ? 0 : 1) + ' ' + units[unit];
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
    function seedStatsHistory(label, data) {
      if (statsChartState.seeded) return;
      const cpu = loadToPercent(data.resources?.load?.one);
      const temp = Number(data.resources?.load?.cpuTemperatureCelsius || 0);
      const now = Date.now();
      statsChartState.labels = Array.from({ length: 24 }, (_, index) => new Date(now - (23 - index) * 5000).toLocaleTimeString());
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
      const totalRx = (data.network?.interfaces || []).reduce((sum, iface) => sum + Number(iface.rxBytes || 0), 0);
      const totalTx = (data.network?.interfaces || []).reduce((sum, iface) => sum + Number(iface.txBytes || 0), 0);
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
    function points(values, width, height, maxValue) {
      const max = Math.max(maxValue || 0, ...values, 1);
      const step = values.length > 1 ? width / (values.length - 1) : width;
      return values.map((value, index) => `${(index * step).toFixed(1)},${(height - (Number(value || 0) / max) * (height - 32) - 12).toFixed(1)}`).join(' ');
    }
    function axisTicks(maxValue, count = 4) {
      return Array.from({ length: count + 1 }, (_, index) => Math.round((maxValue / count) * index));
    }
    function renderRechartsLine(containerId, datasets, opts = {}) {
      const container = document.getElementById(containerId);
      if (!container) return;
      const width = 760;
      const height = opts.height || 200;
      const plot = { left: 52, right: 24, top: 18, bottom: 34 };
      const plotWidth = width - plot.left - plot.right;
      const plotHeight = height - plot.top - plot.bottom;
      const maxValue = opts.maxValue || Math.max(...datasets.flatMap(ds => ds.values), 1);
      const grid = axisTicks(maxValue).map(value => {
        const y = plot.top + plotHeight - (value / Math.max(maxValue, 1)) * plotHeight;
        return `<g class="recharts-cartesian-grid-horizontal"><line x1="${plot.left}" x2="${width - plot.right}" y1="${y.toFixed(1)}" y2="${y.toFixed(1)}"></line><text x="${plot.left - 8}" y="${(y + 4).toFixed(1)}" text-anchor="end" class="recharts-cartesian-axis-tick-value">${opts.formatTick ? opts.formatTick(value) : value}</text></g>`;
      }).join('') + [0, .25, .5, .75, 1].map(ratio => {
        const x = plot.left + ratio * plotWidth;
        return `<line x1="${x.toFixed(1)}" x2="${x.toFixed(1)}" y1="${plot.top}" y2="${height - plot.bottom}" class="recharts-cartesian-grid-vertical"></line>`;
      }).join('');
      const lines = datasets.map(ds => `<polyline class="recharts-line-curve" points="${points(ds.values, plotWidth, plotHeight, maxValue)}" fill="none" stroke="${ds.color}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" stroke-dasharray="${ds.dash || ''}" transform="translate(${plot.left} ${plot.top})" data-series="${ds.name}"></polyline>`).join('');
      const legend = datasets.map(ds => ds.icon === 'square'
        ? `<div class="recharts-legend-item"><svg width="20" height="20"><rect x="5" y="5" width="10" height="10" fill="none" stroke="${ds.color}" stroke-width="2"></rect></svg><span>${ds.name}</span></div>`
        : `<div class="recharts-legend-item"><svg width="20" height="20"><circle cx="10" cy="10" r="5" fill="${ds.color}"></circle></svg><span>${ds.name}</span></div>`).join('');
      const xLabels = statsChartState.labels.filter((_, index) => index % Math.max(1, Math.ceil(statsChartState.labels.length / 4)) === 0).slice(-4).map((label, index) => `<text x="${(plot.left + index * (plotWidth / 3)).toFixed(1)}" y="${height - 10}" text-anchor="middle" class="recharts-cartesian-axis-tick-value">${label.split(' ')[0]}</text>`).join('');
      container.innerHTML = `<div class="recharts-wrapper"><svg class="recharts-surface" viewBox="0 0 ${width} ${height}" role="img" aria-label="${opts.label || 'chart'}"><g class="recharts-cartesian-grid">${grid}</g><g class="recharts-cartesian-axis recharts-xAxis"><line x1="${plot.left}" x2="${width - plot.right}" y1="${height - plot.bottom}" y2="${height - plot.bottom}" stroke="var(--hiddenTabText)"></line>${xLabels}</g><g class="recharts-cartesian-axis recharts-yAxis"><line x1="${plot.left}" x2="${plot.left}" y1="${plot.top}" y2="${height - plot.bottom}" stroke="var(--hiddenTabText)"></line></g><g class="recharts-line">${lines}</g></svg><div class="recharts-legend-wrapper custom-legend">${legend}</div></div>`;
    }
    function renderCpuChart(data) {
      renderRechartsLine('cpu-chart-container', [
        { name: 'CPU Usage', values: statsChartState.cpu, color: 'var(--secondary)' },
        { name: 'Temperature', values: statsChartState.temp, color: 'var(--accent)' }
      ], { label: 'CPU Usage & Load', maxValue: 100, formatTick: value => value + '%' });
      document.getElementById('load-1min').textContent = metricPercent(loadToPercent(data.resources?.load?.one));
      document.getElementById('load-5min').textContent = metricPercent(loadToPercent(data.resources?.load?.five));
      document.getElementById('load-15min').textContent = metricPercent(loadToPercent(data.resources?.load?.fifteen));
    }
    function interfaceLabel(name) {
      if (name === 'wan0') return 'WAN';
      if (name === 'lan0') return 'LAN';
      if (name === 'tailscale0') return 'Tailscale VPN';
      if (name === 'veth0') return 'Transmission';
      return name;
    }
    function renderNetwork(data) {
      renderRechartsLine('network-chart-container', [
        { name: 'Download Speed', values: statsChartState.download, color: 'var(--secondary)' },
        { name: 'Upload Speed', values: statsChartState.upload, color: 'var(--accent)' }
      ], { label: 'Network Traffic (WAN)', formatTick: value => fmtBytes(value) });
      const tbody = document.querySelector('[data-network-interfaces]');
      if (tbody) tbody.innerHTML = (data.network?.interfaces || []).map(iface => `<tr><td><span class="interface-name">${interfaceLabel(iface.name)}</span><span class="interface-label"> (${iface.name})</span></td><td class="data-cell">${fmtBytes(iface.rxBytes)}</td><td class="data-cell">${fmtBytes(iface.txBytes)}</td></tr>`).join('') || '<tr><td colspan="3">Loading network data...</td></tr>';
    }
    function renderDiskIo(data) {
      const controls = document.querySelector('[data-device-controls]');
      const devices = data.io?.devices || [];
      if (controls) controls.innerHTML = devices.map(device => `<div class="device-control"><div class="device-name">${device.device}</div><div class="device-checkboxes"><label><input type="checkbox" name="read-${device.device}" checked>Read</label><label><input type="checkbox" name="write-${device.device}" checked>Write</label></div></div>`).join('') || '<div class="io-loading"><p>Loading disk I/O data...</p></div>';
      const datasets = devices.flatMap((device, index) => {
        const series = statsChartState.ioSeries[device.device] || { read: [0], write: [0] };
        const colors = ['var(--secondary)', 'var(--accent)', 'var(--warning)', 'var(--success)', 'var(--error)'];
        const color = colors[index % colors.length];
        return [
          { name: `${device.device} (Read)`, values: series.read, color },
          { name: `${device.device} (Write)`, values: series.write, color, dash: '3 3', icon: 'square' }
        ];
      });
      renderRechartsLine('disk-io-chart-container', datasets.length ? datasets : [{ name: 'disk (Read)', values: [0], color: 'var(--secondary)' }, { name: 'disk (Write)', values: [0], color: 'var(--secondary)', dash: '3 3' }], { label: 'Disk I/O', formatTick: value => fmtBytes(value) + '/s' });
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
      target.innerHTML = (data.storage || []).map(drive => `<div class="disk-usage-item"><div class="disk-usage-header"><div class="disk-device">${drive.name} (${Number(drive.usagePercent || 0).toFixed(1)}%)</div><div class="disk-mountpoint">Mount: ${drive.mount}</div></div><div class="disk-usage-bar"><div class="disk-usage-fill" style="width:${drive.usagePercent || 0}%"></div></div><div class="disk-usage-details"><div>Used: ${fmtBytes(drive.usedBytes)}</div><div>Free: ${fmtBytes(drive.freeBytes)}</div><div>Total: ${fmtBytes(drive.totalBytes)}</div></div></div>`).join('') || '<div class="disk-usage-loading"><p>Loading disk usage data...</p></div>';
    }
    function renderKeaLeases(data) {
      const tbody = document.querySelector('[data-kea-leases]');
      if (!tbody) return;
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
        const data = await fetch('/api/stats').then(r => r.json());
        const label = new Date().toLocaleTimeString();
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
      const factory = factoryNames.includes(portal.name);
      const services = (portal.services || []).map(service => `<span class="portal-chip">${escapeHtml(service)}</span>`).join('');
      const port = portal.port ? `<span class="portal-chip">:${escapeHtml(portal.port)}</span>` : '';
      const serviceData = encodeURIComponent(JSON.stringify(portal.services || []));
      const adminControls = portal.type === 'link' ? '' : `<div class="portal-admin-controls" data-admin-only data-admin-viewport="portals" data-portal-services="${serviceData}">
        <div class="admin-controls-row"><button data-service-action="start">Start</button><button data-service-action="stop">Stop</button><button data-service-action="restart">Restart</button></div>
        <div class="admin-controls-row"><button data-service-action="enable">Enable</button><button data-service-action="disable">Disable</button><button data-service-action="status">Status</button></div>
      </div>`;
      const isVisible = portal.visible !== false;
      const visibilityToggle = `<button type="button" class="visibility-toggle" data-admin-only data-admin-viewport="portals" data-portal-visibility-toggle data-visible="${isVisible}" aria-label="${isVisible ? 'Hide' : 'Show'} ${escapeHtml(portal.name)}">${isVisible ? '👁' : '🙈'}</button>`;
      return `<div class="portal-element" data-portal-element data-visible="${isVisible}" style="position:relative">
        ${visibilityToggle}
        <article class="card portal-card ${escapeHtml(portal.status || 'unknown')}" data-portal-card data-portal-name="${escapeHtml(portal.name)}" data-portal-url="${escapeHtml(destination)}" role="link" tabindex="0">
          <div class="portal-card-header">
            <img src="/api/portals/images/${encodeURIComponent(portal.name)}.png" alt="${escapeHtml(portal.name)} icon" class="portal-icon" onerror="this.onerror=null;this.src='/api/portals/images/default.png';">
            <h2 class="portal-name">${escapeHtml(portal.name)}</h2>
            <p class="portal-description">${escapeHtml(portal.description || '')}</p>
          </div>
          <div class="portal-service-row">${factory ? '<span class="portal-chip">factory</span>' : '<span class="portal-chip">custom</span>'}${isVisible ? '' : '<span class="portal-chip">hidden</span>'}${port}${services}</div>
          ${adminControls}
        </article>
      </div>`;
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
        if (readout) readout.textContent = 'No services specified for this portal.';
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
    }

    async function hydratePortals() {
      const grid = document.querySelector('[data-portals-grid]');
      if (!grid) return;
      try {
        const data = await fetch(grid.dataset.portalsSource || '/api/portals').then(r => r.json());
        const portals = data.portals || [];
        const factoryNames = data.factoryPortals || [];
        grid.innerHTML = portals.length ? portals.map(portal => renderPortalCard(portal, factoryNames)).join('') : '<article class="card portal-card"><h2>No portals configured</h2><p>homeserver.json has no portal entries.</p></article>';
        grid.querySelectorAll('[data-portal-card]').forEach(card => {
          const open = () => { const url = card.dataset.portalUrl; if (url && url !== '#') window.open(url, '_blank', 'noopener,noreferrer'); };
          card.addEventListener('click', open);
          card.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); open(); } });
        });
        grid.querySelectorAll('[data-service-action]').forEach(button => button.addEventListener('click', handlePortalServiceAction));
        setAdminMode(headerState.isAdmin);
      } catch (error) {
        grid.innerHTML = '<article class="card portal-card error"><h2>Portals unavailable</h2><p>homeserver.json could not be read.</p></article>';
      }
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


    function switchScopedTabs(tabSelector, panelSelector, attrName, selectedName) {
      document.querySelectorAll(tabSelector).forEach(tab => {
        const selected = tab.dataset[attrName] === selectedName;
        tab.classList.toggle('active', selected);
        tab.setAttribute('aria-selected', String(selected));
      });
      document.querySelectorAll(panelSelector).forEach(panel => panel.classList.toggle('active', panel.dataset[attrName.replace('Tab','Panel')] === selectedName));
    }
    document.querySelectorAll('[data-testtab-tab]').forEach(tab => tab.addEventListener('click', () => switchScopedTabs('[data-testtab-tab]', '[data-testtab-panel]', 'testtabTab', tab.dataset.testtabTab)));
    document.querySelectorAll('[data-showcase-tab]').forEach(tab => tab.addEventListener('click', () => switchScopedTabs('[data-showcase-tab]', '[data-showcase-panel]', 'showcaseTab', tab.dataset.showcaseTab)));
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
    document.querySelectorAll('[data-testtab-health-check]').forEach(button => button.addEventListener('click', () => {
      const out = document.querySelector('[data-testtab-health-output]');
      if (out) out.textContent = JSON.stringify({ schema: 'coronatio.testtab.health.v1', status: 'ready', dependencies: { rust_shell: true, theme_catalog: Boolean(themeCatalog?.themes), ux_library: true }, theme: headerState.theme }, null, 2);
    }));

    hydrateFavoriteManifest();
    hydrateThemeTruth();
    hydrateUptime();
    hydrateStats();
    hydratePortals();
    setInterval(hydrateStats, 5000);
  </script>
</body>
</html>"####
}
