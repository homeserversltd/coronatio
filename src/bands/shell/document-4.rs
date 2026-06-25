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
    function setProgress(id, percent) {
      const el = document.getElementById(id);
      if (el) el.style.width = (percent ?? 0) + '%';
    }
    function metricPercent(value) { return value === null || value === undefined ? '—' : value + '%'; }
    const selectedDrivesKey = 'selectedDrives';
    const statsChartState = {
      labels: [],
      cpu: [],
      temp: [],
      upload: [],
      download: [],
      lastRx: null,
      lastTx: null,
      lastIo: {},
      lastStamp: null,
      selectedDrives: [],
      charts: {}
    };
    function chartReady() { return typeof Chart !== 'undefined'; }
    function smoothData(data, windowSize) {
      return data.map((_, index) => {
        const slice = data.slice(Math.max(0, index - windowSize + 1), index + 1);
        return slice.reduce((sum, value) => sum + value, 0) / Math.max(1, slice.length);
      });
    }
    function reduceDataPoints(data, labels, maxPoints) {
      if (data.length <= maxPoints) return { data, labels };
      const step = Math.ceil(data.length / maxPoints);
      return { data: data.filter((_, index) => index % step === 0), labels: labels.filter((_, index) => index % step === 0) };
    }
    function chartColors(ctx, top, bottom, height = 200) {
      const gradient = ctx.createLinearGradient(0, 0, 0, height);
      gradient.addColorStop(0, top);
      gradient.addColorStop(1, bottom);
      return gradient;
    }
    function selectedDriveMounts(data) {
      const mounts = (data.io?.devices || []).map(device => device.mount);
      if (!statsChartState.selectedDrives.length) {
        try { statsChartState.selectedDrives = JSON.parse(localStorage.getItem(selectedDrivesKey) || '[]'); } catch (_) { statsChartState.selectedDrives = []; }
      }
      if (!statsChartState.selectedDrives.length) statsChartState.selectedDrives = mounts;
      return statsChartState.selectedDrives.filter(mount => mounts.includes(mount));
    }
    function saveSelectedDrives() { localStorage.setItem(selectedDrivesKey, JSON.stringify(statsChartState.selectedDrives)); }
    function renderDriveSelector(data) {
      const selector = document.getElementById('io-drive-selector');
      if (!selector) return;
      const mounts = (data.io?.devices || []).map(device => device.mount);
      const selected = selectedDriveMounts(data);
      selector.innerHTML = mounts.map(mount => `<label class="drive-checkbox"><input type="checkbox" value="${mount}" ${selected.includes(mount) ? 'checked' : ''}>${mount}</label>`).join('');
      selector.querySelectorAll('input').forEach(input => input.addEventListener('change', () => {
        if (input.checked && !statsChartState.selectedDrives.includes(input.value)) statsChartState.selectedDrives.push(input.value);
        if (!input.checked) statsChartState.selectedDrives = statsChartState.selectedDrives.filter(mount => mount !== input.value);
        saveSelectedDrives();
        updateStatsCharts(data);
      }));
    }
    function pushChartPoint(label, data) {
      const now = Date.now();
      const totalRx = (data.network?.interfaces || []).reduce((sum, iface) => sum + Number(iface.rxBytes || 0), 0);
      const totalTx = (data.network?.interfaces || []).reduce((sum, iface) => sum + Number(iface.txBytes || 0), 0);
      const seconds = statsChartState.lastStamp ? Math.max(1, (now - statsChartState.lastStamp) / 1000) : 1;
      const downloadRateKb = statsChartState.lastRx === null ? 0 : Math.max(0, (totalRx - statsChartState.lastRx) / seconds / 1024);
      const uploadRateKb = statsChartState.lastTx === null ? 0 : Math.max(0, (totalTx - statsChartState.lastTx) / seconds / 1024);
      statsChartState.lastRx = totalRx;
      statsChartState.lastTx = totalTx;
      statsChartState.lastStamp = now;
      statsChartState.labels.push(label);
      statsChartState.cpu.push(Number(data.resources?.load?.one || 0));
      statsChartState.temp.push(Number(data.resources?.load?.cpuTemperatureCelsius || 0));
      statsChartState.upload.push(uploadRateKb);
      statsChartState.download.push(downloadRateKb);
      (data.io?.devices || []).forEach(device => {
        const previous = statsChartState.lastIo[device.mount];
        const readSpeed = previous ? Math.max(0, (Number(device.readBytes || 0) - previous.readBytes) / seconds) : 0;
        const writeSpeed = previous ? Math.max(0, (Number(device.writeBytes || 0) - previous.writeBytes) / seconds) : 0;
        statsChartState.lastIo[device.mount] = { readBytes: Number(device.readBytes || 0), writeBytes: Number(device.writeBytes || 0), readSpeed, writeSpeed };
      });
      if (statsChartState.labels.length > 60) {
        ['labels', 'cpu', 'temp', 'upload', 'download'].forEach(key => statsChartState[key].shift());
      }
    }
    function createCPUChart(ctx, labels, cpuData, tempData) {
      const cpuGradient = chartColors(ctx.getContext('2d'), 'rgba(75, 192, 192, 0.8)', 'rgba(75, 192, 192, 0.2)', 400);
      const tempGradient = chartColors(ctx.getContext('2d'), 'rgba(255, 99, 132, 0.8)', 'rgba(255, 99, 132, 0.2)', 400);
      return new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          { label: 'CPU Usage', data: cpuData, borderColor: cpuGradient, backgroundColor: 'rgba(75, 192, 192, 0.1)', borderWidth: 2, fill: true, tension: 0.4, yAxisID: 'y-cpu' },
          { label: 'Temperature', data: tempData, borderColor: tempGradient, backgroundColor: 'rgba(255, 99, 132, 0.1)', borderWidth: 2, fill: true, tension: 0.4, yAxisID: 'y-temp' }
        ] },
        options: { responsive: true, maintainAspectRatio: false, interaction: { mode: 'index', intersect: false }, plugins: { tooltip: { enabled: true, mode: 'index', intersect: false }, legend: { position: 'top' }, datalabels: { display: false } }, scales: { x: { ticks: { maxTicksLimit: 10, autoSkip: true, color: '#888' }, grid: { display: false } }, 'y-cpu': { type: 'linear', display: true, position: 'left', beginAtZero: true, max: 100, title: { display: true, text: 'CPU Usage (%)', color: '#888' }, ticks: { color: '#888' }, grid: { color: 'rgba(200, 200, 200, 0.1)' } }, 'y-temp': { type: 'linear', display: true, position: 'right', beginAtZero: true, max: 100, title: { display: true, text: 'Temperature (°C)', color: '#888' }, ticks: { color: '#888' }, grid: { display: false } } } }
      });
    }
    function createNetworkChart(ctx, labels, uploadData, downloadData) {
      const uploadGradient = chartColors(ctx.getContext('2d'), 'rgba(75, 192, 192, 0.8)', 'rgba(75, 192, 192, 0.2)');
      const downloadGradient = chartColors(ctx.getContext('2d'), 'rgba(255, 99, 132, 0.8)', 'rgba(255, 99, 132, 0.2)');
      return new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          { label: 'Upload Speed', data: uploadData, borderColor: uploadGradient, backgroundColor: 'rgba(75, 192, 192, 0.1)', borderWidth: 2, fill: true, tension: 0.4 },
          { label: 'Download Speed', data: downloadData, borderColor: downloadGradient, backgroundColor: 'rgba(255, 99, 132, 0.1)', borderWidth: 2, fill: true, tension: 0.4 }
        ] },
        options: { responsive: true, maintainAspectRatio: false, interaction: { mode: 'index', intersect: false }, plugins: { tooltip: { enabled: true, mode: 'index', intersect: false }, legend: { position: 'top' }, datalabels: { display: false } }, scales: { x: { ticks: { maxTicksLimit: 10, autoSkip: true, color: '#888' }, grid: { display: false } }, y: { beginAtZero: true, suggestedMin: 0, suggestedMax: 1000, title: { display: true, text: 'Speed (KB/s)', color: '#888' }, ticks: { color: '#888', callback: value => value + ' KB/s' }, grid: { color: 'rgba(200, 200, 200, 0.1)' } } } }
      });
    }
    function createIOChart(ctx) {
      return new Chart(ctx, {
        type: 'line',
        data: { labels: [], datasets: [] },
        options: { responsive: true, maintainAspectRatio: false, layout: { padding: { left: 10, right: 10, top: 20, bottom: 20 } }, scales: { x: { title: { display: true, text: 'Time', padding: { top: 10, bottom: 10 } }, ticks: { maxRotation: 0, autoSkip: true, maxTicksLimit: 6 } }, y: { title: { display: true, text: 'Speed (MB/s)' }, ticks: { callback: value => Number(value).toFixed(2) } } }, plugins: { legend: { display: false }, tooltip: { mode: 'index', intersect: false }, datalabels: { display: false } } }
      });
    }
    function ensureStatsCharts(data) {
      if (!chartReady()) {
        document.querySelectorAll('.chart-card').forEach(card => {
          if (!card.querySelector('.chart-fallback')) card.insertAdjacentHTML('beforeend', '<p class="chart-fallback">Chart.js dependency unavailable.</p>');
        });
        return;
      }
      if (window.ChartDataLabels && Chart.register) Chart.register(window.ChartDataLabels);
      if (!statsChartState.charts.cpu) {
        const ctx = document.getElementById('cpuChart');
        if (ctx) statsChartState.charts.cpu = createCPUChart(ctx, statsChartState.labels, statsChartState.cpu, statsChartState.temp);
      }
      if (!statsChartState.charts.network) {
        const ctx = document.getElementById('networkChart');
        if (ctx) statsChartState.charts.network = createNetworkChart(ctx, statsChartState.labels, statsChartState.upload, statsChartState.download);
      }
      if (!statsChartState.charts.io) {
        const ctx = document.getElementById('io-chart');
        if (ctx) statsChartState.charts.io = createIOChart(ctx);
      }
      renderDriveSelector(data);
      updateStatsCharts(data);
    }
    function updateStatsCharts(data) {
      if (!chartReady()) return;
      if (statsChartState.charts.cpu) statsChartState.charts.cpu.update();
      if (statsChartState.charts.network) statsChartState.charts.network.update();
      const selected = selectedDriveMounts(data);
      const colors = { '/': '#FF6384', '/home': '#36A2EB', '/boot': '#FFCE56', '/boot/efi': '#4BC0C0', '/vault': '#9966FF', '/mnt/elements': '#FF9F40', '/mnt/wd-drive': '#FF6384' };
      const ioChart = statsChartState.charts.io;
      if (ioChart) {
        ioChart.data.labels = statsChartState.labels.slice();
        ioChart.data.datasets = (data.io?.devices || []).flatMap(device => {
          if (!selected.includes(device.mount)) return [];
          const color = colors[device.mount] || '#00f2fe';
          const current = statsChartState.lastIo[device.mount] || { readSpeed: 0, writeSpeed: 0 };
          return [
            { label: `${device.mount} Read`, data: statsChartState.labels.map((_, idx) => idx === statsChartState.labels.length - 1 ? current.readSpeed / (1024 * 1024) : 0), borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0 },
            { label: `${device.mount} Write`, data: statsChartState.labels.map((_, idx) => idx === statsChartState.labels.length - 1 ? current.writeSpeed / (1024 * 1024) : 0), borderColor: color, backgroundColor: color, borderWidth: 2, borderDash: [5, 5], fill: false, pointRadius: 0 }
          ];
        });
        ioChart.update();
        const legend = document.getElementById('io-chart-legend');
        if (legend) legend.innerHTML = selected.map(mount => `<span><span style="display:inline-block;width:20px;border-top:2px solid ${colors[mount] || '#00f2fe'};margin-right:5px"></span>Read <span style="display:inline-block;width:20px;border-top:2px dotted ${colors[mount] || '#00f2fe'};margin:0 5px"></span>Write ${mount}</span>`).join('');
      }
    }
    async function hydrateStats() {
      try {
        const data = await fetch('/api/stats').then(r => r.json());
        const caduceus = await fetch('/api/caduceus/status').then(r => r.json()).catch(() => null);
        const load = data.resources?.load || {};
        const memory = data.resources?.memory || {};
        const swap = data.resources?.swap || {};
        const label = new Date().toLocaleTimeString();
        pushChartPoint(label, data);
        document.getElementById('cpu-current').textContent = load.one ?? '—';
        document.getElementById('cpu-temp').textContent = load.cpuTemperatureCelsius ?? '—';
        document.getElementById('cpu-5m').textContent = load.five ?? '—';
        document.getElementById('cpu-cores').textContent = navigator.hardwareConcurrency || '—';
        document.getElementById('memory-usage').textContent = metricPercent(memory.percent);
        document.getElementById('stats-memory-used').textContent = 'Used ' + fmtBytes(memory.usedBytes);
        document.getElementById('stats-memory-total').textContent = 'Total ' + fmtBytes(memory.totalBytes);
        setProgress('stats-memory-progress', memory.percent);
        document.getElementById('stats-swap').textContent = metricPercent(swap.percent);
        document.getElementById('stats-swap-used').textContent = 'Used ' + fmtBytes(swap.usedBytes);
        document.getElementById('stats-swap-total').textContent = 'Total ' + fmtBytes(swap.totalBytes);
        setProgress('stats-swap-progress', swap.percent);
        const drives = document.querySelector('[data-stats-drives]');
        drives.innerHTML = (data.storage || []).map(drive => `<article class="drive-info"><h3>${drive.mount}</h3><div class="progress-bar"><div class="progress" style="width:${drive.usagePercent ?? 0}%"></div></div><div class="details"><span>${fmtBytes(drive.usedBytes)} / ${fmtBytes(drive.totalBytes)}</span><span>${drive.usagePercent ?? '—'}%</span></div><p class="muted">${drive.name}</p></article>`).join('') || '<article class="drive-info"><h3>No storage readback</h3><p class="muted">df readback unavailable.</p></article>';
        const network = document.querySelector('[data-stats-network]');
        network.innerHTML = (data.network?.interfaces || []).map(iface => `<article class="network-interface"><h3><span class="status-dot ${iface.status}"></span>${iface.name}</h3><div class="traffic"><span>↓ ${fmtBytes(iface.rxBytes)}</span><span>↑ ${fmtBytes(iface.txBytes)}</span></div></article>`).join('') || '<article class="network-interface"><h3>No network readback</h3><p class="muted">/proc/net/dev unavailable.</p></article>';
        const counts = data.network?.connections || {};
        document.querySelector('[data-stats-connections]').innerHTML = `<div class="counts"><span>Established: ${counts.established ?? 0}</span><span>Listening: ${counts.listening ?? 0}</span><span>Total: ${counts.total ?? 0}</span></div>`;
        const services = document.querySelector('[data-stats-services]');
        services.innerHTML = (data.services || []).map(service => `<article class="service-info"><h3><span class="status-dot ${service.status}"></span>${service.name}</h3><div class="details"><span>${service.status}</span><span>${service.route}</span></div><p class="muted">${service.details}</p></article>`).join('');
        ensureStatsCharts(data);
        document.getElementById('stats-stream').textContent = (data.transport?.streamStatus || 'unknown') + ' — ' + (data.transport?.streamReason || '');
        document.getElementById('stats-missing').textContent = caduceus?.firstMissingSignal || data.telemetry?.firstMissingSignal || 'none';
        document.getElementById('stats-readout').textContent = JSON.stringify({ stats: data, caduceus }, null, 2);
      } catch (error) { document.getElementById('stats-readout').textContent = String(error); }
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
