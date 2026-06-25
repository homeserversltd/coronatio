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

    const uploadForm = document.querySelector('[data-upload-form]');
    uploadForm?.addEventListener('submit', async event => {
      event.preventDefault();
      const file = document.querySelector('[data-upload-file]')?.files?.[0];
      const destination = document.querySelector('[data-upload-path]')?.value || '/mnt/nas';
      const out = document.getElementById('upload-readout');
      if (!file) { out.textContent = 'Choose a file first.'; return; }
      const form = new FormData();
      form.append('file', file);
      form.append('path', destination);
      out.textContent = 'Uploading ' + file.name + ' through Caduceus…';
      try {
        const response = await fetch('/api/files/upload', { method: 'POST', body: form });
        const text = await response.text();
        try { out.textContent = JSON.stringify(JSON.parse(text), null, 2); }
        catch (_) { out.textContent = text; }
      } catch (error) { out.textContent = 'upload failed: ' + error; }
    });

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
    const statsChartState = { labels: [], cpu: [], temp: [], upload: [], download: [], lastRx: null, lastTx: null, lastIo: {}, lastStamp: null, ioSeries: {} };
    function pushChartPoint(label, data) {
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
        if (!statsChartState.ioSeries[key]) statsChartState.ioSeries[key] = { read: [], write: [], label: key };
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
      return values.map((value, index) => `${(index * step).toFixed(1)},${(height - (Number(value || 0) / max) * (height - 20) - 10).toFixed(1)}`).join(' ');
    }
    function renderRechartsLine(containerId, datasets, opts = {}) {
      const container = document.getElementById(containerId);
      if (!container) return;
      const width = 640;
      const height = opts.height || 200;
      const maxValue = opts.maxValue || Math.max(...datasets.flatMap(ds => ds.values), 1);
      const lines = datasets.map(ds => `<polyline class="recharts-line-curve" points="${points(ds.values, width - 40, height - 40, maxValue)}" fill="none" stroke="${ds.color}" stroke-width="2" stroke-dasharray="${ds.dash || ''}" transform="translate(20 20)" data-series="${ds.name}"></polyline>`).join('');
      const legend = datasets.map(ds => `<span class="recharts-legend-item"><span style="display:inline-block;width:18px;border-top:2px ${ds.dash ? 'dashed' : 'solid'} ${ds.color};margin-right:4px"></span>${ds.name}</span>`).join('');
      container.innerHTML = `<div class="recharts-wrapper"><svg class="recharts-surface" viewBox="0 0 ${width} ${height}" role="img" aria-label="${opts.label || 'chart'}"><g class="recharts-cartesian-grid"><line x1="20" x2="620" y1="180" y2="180" stroke="var(--border)"></line><line x1="20" x2="20" y1="20" y2="180" stroke="var(--border)"></line></g>${lines}</svg><div class="recharts-legend-wrapper">${legend}</div></div>`;
    }
    function renderCpuChart(data) {
      renderRechartsLine('cpu-chart-container', [
        { name: 'CPU Usage', values: statsChartState.cpu, color: 'var(--secondary)' },
        { name: 'Temperature', values: statsChartState.temp, color: 'var(--accent)' }
      ], { label: 'CPU Usage & Load', maxValue: 100 });
      document.getElementById('load-1min').textContent = metricPercent(loadToPercent(data.resources?.load?.one));
      document.getElementById('load-5min').textContent = metricPercent(loadToPercent(data.resources?.load?.five));
      document.getElementById('load-15min').textContent = metricPercent(loadToPercent(data.resources?.load?.fifteen));
    }
    function renderNetwork(data) {
      renderRechartsLine('network-chart-container', [
        { name: 'Download Speed', values: statsChartState.download, color: 'var(--secondary)' },
        { name: 'Upload Speed', values: statsChartState.upload, color: 'var(--accent)' }
      ], { label: 'Network Traffic (WAN)' });
      const tbody = document.querySelector('[data-network-interfaces]');
      if (tbody) tbody.innerHTML = (data.network?.interfaces || []).map(iface => `<tr><td><span class="interface-name">${iface.name}</span></td><td class="data-cell">${fmtBytes(iface.rxBytes)}</td><td class="data-cell">${fmtBytes(iface.txBytes)}</td></tr>`).join('') || '<tr><td colspan="3">Loading network data...</td></tr>';
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
          { name: `${device.device} (Write)`, values: series.write, color, dash: '3 3' }
        ];
      });
      renderRechartsLine('disk-io-chart-container', datasets.length ? datasets : [{ name: 'disk (Read)', values: [0], color: 'var(--secondary)' }, { name: 'disk (Write)', values: [0], color: 'var(--secondary)', dash: '3 3' }], { label: 'Disk I/O' });
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
      return `<article class="card portal-card ${escapeHtml(portal.status || 'unknown')}" data-portal-card data-portal-name="${escapeHtml(portal.name)}" data-portal-url="${escapeHtml(destination)}" role="link" tabindex="0">
        <div class="portal-card-header">
          <img src="/api/portals/images/${encodeURIComponent(portal.name)}.png" alt="${escapeHtml(portal.name)} icon" class="portal-icon" onerror="this.onerror=null;this.src='/api/portals/images/default.png';">
          <h2 class="portal-name">${escapeHtml(portal.name)}</h2>
          <p class="portal-description">${escapeHtml(portal.description || '')}</p>
        </div>
        <div class="portal-service-row">${factory ? '<span class="portal-chip">factory</span>' : '<span class="portal-chip">custom</span>'}${port}${services}</div>
      </article>`;
    }
    async function hydratePortals() {
      const grid = document.querySelector('[data-portals-grid]');
      const readout = document.getElementById('portals-readout');
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
        if (readout) readout.textContent = JSON.stringify({ source: data.source, count: portals.length, firstMissingSignal: data.firstMissingSignal }, null, 2);
      } catch (error) {
        grid.innerHTML = '<article class="card portal-card error"><h2>Portals unavailable</h2><p>homeserver.json could not be read.</p></article>';
        if (readout) readout.textContent = 'portal load failed: ' + error;
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
    hydrateFavoriteManifest();
    hydrateUptime();
    hydrateStats();
    hydratePortals();
    setInterval(hydrateStats, 5000);
  </script>
</body>
</html>"####
}
