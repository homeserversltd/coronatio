fn shell_backblaze_client() -> &'static str {
    r####"
    const state = {
      binding: null,
      data: null,
      statusReady: false,
      statusStale: true,
      drives: [],
      drivesReady: false,
      summaryRows: [],
      summaryReady: false,
      ledgerRows: [],
      ledgerNext: null,
      ledgerLoading: false,
      ledgerFilter: 'all',
      ledgerError: '',
      skipBucket: null,
      skipDraft: [],
      skipChecks: new Map(),
      folderBucket: null,
      folderPath: '/mnt/nas',
      folderNames: [],
      folderDraft: new Set(),
      folderBaseline: [],
      folderTruncated: false,
      scheduleBaseline: null,
      snapshots: [],
      activeModal: null,
      returnFocus: null,
      addMode: 'add',
      editBucket: null,
      menuBucket: null,
      readErrors: {}
    };
    const actions = new Set();
    const esc = value => String(value ?? '').replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[char]));
    const isRecord = value => Boolean(value && typeof value === 'object' && !Array.isArray(value));
    const validSkip = value => typeof value === 'string' && /^[A-Za-z0-9_.-]+(?:\/[A-Za-z0-9_.-]+)?$/.test(value);
    const admitted = () => typeof viewportFamilyAdmitted === 'function' && viewportFamilyAdmitted('backblaze');
    const roleAuthorized = () => Boolean(headerState?.isAdmin && coronatioAttendanceRuntime?.currentAttendance);
    const root = (binding = state.binding) => binding?.root instanceof Element ? binding.root : null;
    const el = (selector, binding = state.binding) => root(binding)?.querySelector(selector) || null;
    const current = (binding = state.binding) => Boolean(binding && state.binding === binding && admitted() && scaffoldRegistry.current(binding));
    const canUse = (binding = state.binding) => current(binding) && state.statusReady && !state.statusStale && isRecord(state.data);
    const buckets = () => Array.isArray(state.data?.buckets) ? state.data.buckets : [];
    const jobs = () => Array.isArray(state.data?.jobs) ? state.data.jobs : [];
    const clone = value => JSON.parse(JSON.stringify(value));
    const stable = value => {
      if (Array.isArray(value)) return `[${value.map(stable).join(',')}]`;
      if (!isRecord(value)) return JSON.stringify(value);
      return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${stable(value[key])}`).join(',')}}`;
    };
    function configRow(bucket) {
      if (!isRecord(bucket)) return null;
      const row = clone(bucket);
      delete row.status;
      delete row.connected;
      return row;
    }
    function bucketWritable(bucket) {
      if (!isRecord(bucket) || typeof bucket.bucket !== 'string' || !bucket.bucket || bucket.bucket !== bucket.bucket.trim() || bucket.bucket.includes('/')) return false;
      if (typeof bucket.prefix !== 'string' || bucket.prefix !== bucket.prefix.trim() || bucket.prefix.replace(/^\/+|\/+$/g, '') !== bucket.prefix) return false;
      if (!Array.isArray(bucket.items) || bucket.items.some(item => !isRecord(item) || typeof item.path !== 'string' || item.path !== item.path.trim() || !item.path.startsWith('/'))) return false;
      const manager = bucket.managers?.forgejo;
      if (!isRecord(manager) || typeof manager.localNas !== 'boolean' || typeof manager.backblazeBackup !== 'boolean') return false;
      if (manager.skip !== undefined && (!Array.isArray(manager.skip) || manager.skip.some(name => !validSkip(name)) || new Set(manager.skip).size !== manager.skip.length)) return false;
      return true;
    }
    function canRewriteConfig(binding = state.binding) {
      return canUse(binding) && buckets().every(bucketWritable);
    }
    function bucketFor(name) { return buckets().find(bucket => bucket.bucket === name) || null; }
    function scheduleFor(job) {
      const schedule = job?.pid2?.schedule;
      if (!isRecord(schedule)) return null;
      const calendar = isRecord(schedule.calendar) ? schedule.calendar : schedule;
      return {
        enabled: schedule.enabled === true,
        frequency: calendar.frequency === 'weekly' ? 'weekly' : 'daily',
        time: typeof calendar.time === 'string' ? calendar.time : '02:00',
        weekday: typeof calendar.weekday === 'string' ? calendar.weekday : 'Mon'
      };
    }
    function projectStatus(body) {
      if (!isRecord(body) || body.ok === false || !Array.isArray(body.buckets) || !Array.isArray(body.jobs)) throw new Error('Backup status response is incomplete');
      const checkedBuckets = body.buckets.map(bucket => {
        if (!isRecord(bucket) || typeof bucket.bucket !== 'string' || !bucket.bucket) throw new Error('Backup bucket row is malformed');
        return bucket;
      });
      const checkedJobs = body.jobs.map(job => {
        if (!isRecord(job) || typeof job.schedule_id !== 'string') throw new Error('Backup schedule row is malformed');
        return job;
      });
      return { ...body, buckets: checkedBuckets, jobs: checkedJobs };
    }
    function normalizeDriveResponse(body) {
      if (!isRecord(body) || body.ok === false || !Array.isArray(body.drives)) throw new Error('Drive response is incomplete');
      return body.drives.filter(drive => isRecord(drive) && (drive.role === 'main' || drive.role === 'backup'));
    }
    function normalizeLedgerResponse(body) {
      if (!isRecord(body) || body.ok === false || !Array.isArray(body.rows)) throw new Error('Ledger response is incomplete');
      return body.rows.filter(isRecord);
    }
    function formatDate(value, fallback = '—') {
      if (value == null || value === '') return fallback;
      let date;
      if (typeof value === 'number') date = new Date(value > 1000000000000 ? value : value * 1000);
      else date = new Date(value);
      return Number.isNaN(date.getTime()) ? fallback : new Intl.DateTimeFormat(undefined, { month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' }).format(date);
    }
    function formatBytes(value) {
      if (value == null || value === '') return '—';
      const bytes = Number(value);
      if (!Number.isFinite(bytes) || bytes < 0) return '—';
      if (bytes === 0) return '0 B';
      const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
      const power = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
      const amount = bytes / (1024 ** power);
      return `${new Intl.NumberFormat(undefined, { maximumFractionDigits: power < 2 ? 0 : 1 }).format(amount)} ${units[power]}`;
    }
    function formatDuration(value) {
      const ms = Number(value);
      if (!Number.isFinite(ms) || ms < 1000) return '—';
      const seconds = Math.floor(ms / 1000);
      const minutes = Math.floor(seconds / 60);
      const hours = Math.floor(minutes / 60);
      if (hours) return `${hours} hr ${minutes % 60} min`;
      if (minutes) return `${minutes} min ${seconds % 60} s`;
      return `${seconds} s`;
    }
    function setText(selector, value, binding = state.binding) {
      const node = el(selector, binding);
      if (node) node.textContent = String(value ?? '');
    }
    function setMainResult(message, binding = state.binding) {
      setText('[data-backups-result]', message, binding);
    }
    function modalResult(name, message, binding = state.binding) {
      const modal = root(binding)?.querySelector(`[data-backups-modal="${CSS.escape(name)}"]`);
      if (!modal) return;
      let note = modal.querySelector('[data-backup-local-result]');
      if (!note) { note = document.createElement('p'); note.dataset.backupLocalResult = ''; note.setAttribute('role', 'status'); modal.querySelector('.backup-dialog')?.append(note); }
      note.textContent = message || '';
    }
    function actionController(binding = state.binding) {
      const controller = new AbortController();
      actions.add(controller);
      return controller;
    }
    async function getJson(url, init = {}, binding = state.binding) {
      const response = await fetch(url, { ...init, cache: 'no-store', credentials: 'same-origin' });
      const body = await response.json().catch(() => ({ ok: false, reason: 'Invalid response' }));
      if (!response.ok && body?.ok !== false) return { ...body, ok: false, reason: body?.reason || `Request failed (${response.status})` };
      return body;
    }
    function readJson(context, url, body = null) {
      const init = body === null ? {} : { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) };
      return context.fetchJson(url, init);
    }
    function renderStatusNote(message, binding = state.binding) {
      const ribbon = el('[data-backblaze-ribbon]', binding);
      if (!ribbon) return;
      let note = ribbon.querySelector('[data-backups-read-note]');
      if (!message) { note?.remove(); return; }
      if (!note) { note = document.createElement('span'); note.dataset.backupsReadNote = ''; note.setAttribute('role', 'status'); ribbon.append(note); }
      note.textContent = message;
    }
    function scheduleSnapshot(sourceJobs = jobs()) {
      return sourceJobs.map(job => {
        const schedule = scheduleFor(job);
        return [job.schedule_id, schedule ? { enabled: schedule.enabled, frequency: schedule.frequency, time: schedule.time, weekday: schedule.weekday } : null];
      }).sort((a, b) => a[0].localeCompare(b[0]));
    }
    function applyStatus(body) {
      const data = projectStatus(body);
      state.data = data;
      state.statusReady = true;
      state.statusStale = false;
      state.scheduleBaseline = scheduleSnapshot(data.jobs);
      return data;
    }
    function renderMain(binding = state.binding) {
      if (!current(binding)) return;
      renderDriveCards(binding);
      renderOffsiteCards(binding);
      renderRibbon(binding);
      renderSnapshotPicker(binding);
      const sync = el('[data-backups-sync]', binding);
      const hasNas = jobs().some(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      if (sync) { sync.hidden = !hasNas; sync.disabled = !canUse(binding) || state.data?.running === true; }
      const noNas = el('[data-backups-no-nas]', binding);
      if (noNas) noNas.hidden = !state.statusReady || hasNas;
      root(binding)?.querySelectorAll('[data-backups-open]').forEach(button => { button.disabled = button.dataset.backupsOpen === 'add' ? !canRewriteConfig(binding) : false; });
      renderStatusNote(Object.values(state.readErrors).some(Boolean) ? 'Some backup details are unavailable. Reopen this tab to try again.' : '', binding);
    }
    async function loadMain(binding = adoptBackblazeBinding()) {
      if (!binding || !current(binding)) return false;
      state.statusStale = true;
      state.readErrors = {};
      renderStatusNote('Refreshing this box’s backup details…', binding);
      return scaffoldRegistry.run(binding, 'backblaze-status', async context => {
        const results = await Promise.allSettled([
          readJson(context, '/api/backblaze/status'),
          readJson(context, '/api/backups/drives', {}),
          readJson(context, '/api/backups/ledger', { limit: 100 })
        ]);
        if (!context.current() || !current(binding)) return false;
        try {
          if (results[0].status !== 'fulfilled') throw results[0].reason;
          applyStatus(results[0].value);
        } catch (_) { state.statusStale = true; state.readErrors.status = true; }
        try {
          if (results[1].status !== 'fulfilled') throw results[1].reason;
          state.drives = normalizeDriveResponse(results[1].value);
          state.drivesReady = true;
        } catch (_) { state.drives = []; state.drivesReady = false; state.readErrors.drives = true; }
        try {
          if (results[2].status !== 'fulfilled') throw results[2].reason;
          state.summaryRows = normalizeLedgerResponse(results[2].value);
          state.summaryReady = true;
        } catch (_) { state.summaryRows = []; state.summaryReady = false; state.readErrors.ledger = true; }
        renderMain(binding);
        return true;
      }, { slots: [], timeoutMs: 12000, slowMs: 1500, retry: false });
    }
    async function refreshStatus(binding = state.binding) {
      if (!current(binding)) return null;
      const controller = actionController(binding);
      try {
        const body = await getJson('/api/backblaze/status', { signal: controller.signal }, binding);
        if (!current(binding) || body?.ok === false) throw new Error(body?.reason || 'Backup status is unavailable');
        const data = applyStatus(body);
        renderMain(binding);
        return data;
      } finally { actions.delete(controller); }
    }
    function driveHealth(drive) {
      if (drive?.health === 'passed') return ['Healthy', 'good'];
      if (drive?.health === 'failed') return ['Drive health check failed', 'bad'];
      return ['Health check unavailable', 'unknown'];
    }
    function driveCard(drive, role, index, binding) {
      const isBackup = role === 'backup';
      if (!drive) {
        if (isBackup) return '<article class="backup-drive-card backup-drive-card--empty"><strong>No backup drive in this box yet</strong></article>';
        return '<article class="backup-drive-card"><strong>Main drive unavailable</strong><p>Drive details could not be read from this box.</p></article>';
      }
      const [health, healthState] = driveHealth(drive);
      const size = Number(drive.sizeBytes), used = Number(drive.usedBytes), free = Number(drive.freeBytes);
      const percent = Number.isFinite(size) && size > 0 && Number.isFinite(used) ? Math.max(0, Math.min(100, (used / size) * 100)) : 0;
      const paused = isBackup && jobs().some(job => (job.schedule_id === 'nas-sync' || job.kind === 'nas') && scheduleFor(job)?.enabled === false);
      const lastMirror = newestNasSync(state.summaryRows);
      const model = typeof drive.model === 'string' && drive.model ? drive.model : 'Model unavailable';
      const mount = typeof drive.mountpoint === 'string' && drive.mountpoint ? drive.mountpoint : 'Mount unavailable';
      const transport = typeof drive.transport === 'string' && drive.transport ? drive.transport.toUpperCase() : 'Connection unavailable';
      const readErrors = drive.readErrors == null || !Number.isFinite(Number(drive.readErrors)) ? '—' : Number(drive.readErrors);
      return `<article class="backup-drive-card${isBackup ? ' backup-drive-card--backup' : ''}" data-drive-role="${role}" data-drive-index="${index}">
        <div class="backup-card-heading"><div><strong>${isBackup ? 'Backup drive' : 'Main drive'} · in the HomeServer</strong><span class="backup-health backup-health--${healthState}">${esc(paused ? 'Paused after a drive swap' : health)}</span></div></div>
        <p class="backup-drive-model">${esc(model)} · ${esc(formatBytes(drive.sizeBytes))}</p>
        <div class="backup-meter" role="img" aria-label="${esc(formatBytes(drive.usedBytes))} used of ${esc(formatBytes(drive.sizeBytes))}"><span style="width:${percent.toFixed(1)}%"></span></div>
        <p class="backup-drive-capacity">${esc(formatBytes(drive.usedBytes))} used <span>${esc(formatBytes(drive.freeBytes))} free</span></p>
        <p>Drive health check <strong class="backup-health backup-health--${healthState}">${esc(drive.health === 'passed' ? 'Passed' : drive.health === 'failed' ? 'Failed' : 'Health check unavailable')}</strong></p>
        <p>Read errors this boot <strong>${esc(readErrors)}</strong></p>
        ${isBackup ? `<p>Last mirror <strong>${esc(formatDate(lastMirror?.at, '—'))}</strong></p>` : ''}
        <p>Connected by ${esc(transport)} <span>${esc(mount)}</span></p>
      </article>`;
    }
    function renderDriveCards(binding = state.binding) {
      const host = el('[data-backups-drives]', binding);
      if (!host) return;
      if (!state.drivesReady) { host.innerHTML = '<p class="backup-empty">Drive details are unavailable.</p>'; return; }
      const mains = state.drives.filter(drive => drive.role === 'main' && drive.present === true);
      const backups = state.drives.filter(drive => drive.role === 'backup' && drive.present === true);
      const cards = [driveCard(mains[0], 'main', 0, binding), ...(backups.length ? backups.map((drive, index) => driveCard(drive, 'backup', index, binding)) : [driveCard(null, 'backup', 0, binding)])];
      host.innerHTML = cards.join('');
    }
    function newestNasSync(rows = state.summaryRows) {
      return rows.find(row => row.target === 'nas-sync' || String(row.verb || '').startsWith('nas-sync-')) || null;
    }
    function newestBucketRun(bucketName, rows = state.summaryRows) {
      return rows.find(row => row.verb === 'backblaze-bucket-run' && row.target === bucketName && String(row.outcome || '').toLowerCase() === 'ok') || null;
    }
    function runResult(bucket) {
      const row = newestBucketRun(bucket.bucket);
      if (bucket.status?.last_run_ok === false) return { state: 'Last run failed', row };
      if (bucket.status?.last_run_ok === true || row) return { state: 'Last run OK', row };
      return { state: 'No run recorded', row: null };
    }
    function renderOffsiteCards(binding = state.binding) {
      const host = el('[data-backups-buckets]', binding);
      if (!host) return;
      if (!state.statusReady || state.statusStale) { host.innerHTML = '<p class="backup-empty">Off-site copies are unavailable until this box’s status can be read.</p>'; return; }
      if (!buckets().length) { host.innerHTML = '<p class="backup-empty">No off-site copies yet.</p>'; return; }
      host.innerHTML = buckets().map(bucket => {
        const manager = bucket.managers?.forgejo || {};
        const skipCount = Array.isArray(manager.skip) ? manager.skip.length : 0;
        const items = Array.isArray(bucket.items) ? bucket.items : [];
        const connected = bucket.connected === true ? 'Connected' : bucket.connected === false ? 'Not connected' : 'Connection status unavailable';
        const latest = runResult(bucket);
        const localRun = newestBucketRun(bucket.bucket);
        const size = localRun ? formatBytes(localRun.total_bytes) : '—';
        const added = localRun ? formatBytes(localRun.added_bytes) : '—';
        const menuOpen = state.menuBucket === bucket.bucket;
        const editable = canRewriteConfig(binding) && bucketWritable(bucket);
        return `<article class="backup-offsite-card" data-offsite-card="${esc(bucket.bucket)}">
          <div class="backup-card-heading backup-offsite-heading"><div><strong>${esc(bucket.bucket)}</strong><span>Backblaze B2 bucket · ${esc(connected.toLowerCase())}</span></div><div class="backup-settings"><button type="button" data-settings-toggle="${esc(bucket.bucket)}" aria-expanded="${menuOpen}" aria-haspopup="true">Settings ▾</button><div class="backup-settings-menu" role="group" aria-label="Settings for ${esc(bucket.bucket)}" ${menuOpen ? '' : 'hidden'}>
            <button type="button" data-bucket-action="run" data-bucket="${esc(bucket.bucket)}" ${canUse(binding) ? '' : 'disabled'}>Run now</button><button type="button" data-bucket-action="verify" data-bucket="${esc(bucket.bucket)}" ${canUse(binding) ? '' : 'disabled'}>Check connection</button><button type="button" data-bucket-action="credentials" data-bucket="${esc(bucket.bucket)}" ${editable ? '' : 'disabled'}>Update credentials</button><button type="button" data-bucket-action="remove" data-bucket="${esc(bucket.bucket)}" ${editable ? '' : 'disabled'}>Remove this copy</button>
          </div></div><span class="backup-run-state">${esc(latest.state)}</span></div>
          <div class="backup-figures"><div><strong>${esc(size)}</strong><span>Size of your copy</span><small>as of the last run</small></div><div><strong>${esc(added)}</strong><span>Added last run</span><small>only what changed</small></div><div><strong>${esc(formatDate(localRun?.at, '—'))}</strong><span>Last run</span><small>took ${esc(formatDuration(localRun?.duration_ms))}</small></div></div>
          <p class="backup-costless">Counted by this box after every run, so checking it never asks Backblaze and never costs anything.</p>
          <div class="backup-includes"><strong>What this copy includes</strong>
            <label class="backup-check-row"><input type="checkbox" data-git-toggle="${esc(bucket.bucket)}" ${manager.backblazeBackup === true ? 'checked' : ''} ${editable ? '' : 'disabled'}><span><strong>Git server</strong><small>Everything on your Git server: repositories, database and settings. Skipping ${skipCount} names you chose.</small></span></label><button type="button" class="backup-subaction" data-open-skip="${esc(bucket.bucket)}" ${editable ? '' : 'disabled'}>Choose what to skip</button>
            <label class="backup-check-row"><input type="checkbox" aria-label="Folders from your drive included" ${items.length ? 'checked' : ''} disabled><span><strong>Folders from your drive</strong><small>Pick the folders worth keeping off-site, like photos and documents.</small></span></label><button type="button" class="backup-subaction" data-open-folders="${esc(bucket.bucket)}" ${editable ? '' : 'disabled'}>Choose folders</button>
          </div>
        </article>`;
      }).join('');
    }
    function jobLabel(job) {
      return job?.kind === 'nas' || job?.schedule_id === 'nas-sync' ? 'Backup drive' : `Off-site to ${job?.bucket || 'your bucket'}`;
    }
    function nextFireDate(job) {
      const raw = job?.pid2?.next_fire;
      if (typeof raw === 'number' || typeof raw === 'string') {
        const date = typeof raw === 'number' ? new Date(raw > 1000000000000 ? raw : raw * 1000) : new Date(raw);
        if (!Number.isNaN(date.getTime())) return date;
      }
      return null;
    }
    function renderRibbon(binding = state.binding) {
      if (!state.statusReady || state.statusStale) {
        setText('[data-backups-happened]', 'Unavailable', binding);
        setText('[data-backups-next]', 'Unavailable', binding);
        return;
      }
      const latest = state.summaryRows[0];
      const happened = latest ? `${ledgerWhat(latest)} · ${formatDate(latest.at, 'Earlier')} · ${String(latest.outcome || '').toLowerCase() === 'ok' ? 'OK' : String(latest.outcome || 'Unknown')}${latest.duration_ms == null ? '' : ` · ${formatDuration(latest.duration_ms)}`}` : (state.summaryReady ? 'Nothing recorded yet' : 'Ledger unavailable');
      const nasJob = jobs().find(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      const paused = nasJob && scheduleFor(nasJob)?.enabled === false;
      const pauseSuffix = paused ? ` · Backup drive paused since ${formatDate(newestNasSync()?.at, 'the last drive swap')}` : '';
      setText('[data-backups-happened]', `${happened}${pauseSuffix}`, binding);
      const scheduledJobs = jobs().filter(job => scheduleFor(job)?.enabled).map(job => ({ job, date: nextFireDate(job) })).filter(item => item.date).sort((a, b) => a.date - b.date);
      if (scheduledJobs.length) {
        const next = scheduledJobs[0];
        const when = new Intl.DateTimeFormat(undefined, { weekday: 'short', hour: 'numeric', minute: '2-digit' }).format(next.date);
        const labels = [];
        if (nasJob) labels.push(paused ? 'Backup drive (paused, skipped)' : 'Backup drive');
        jobs().filter(job => job !== nasJob && scheduleFor(job)?.enabled).sort((a, b) => String(a.bucket || '').localeCompare(String(b.bucket || ''))).forEach(job => labels.push(jobLabel(job)));
        setText('[data-backups-next]', `${when}${labels.length ? ` · ${labels.join(' · ')}` : ` · ${jobLabel(next.job)}`}`, binding);
      } else if (jobs().length && jobs().every(job => scheduleFor(job)?.enabled !== true)) setText('[data-backups-next]', 'Only when I press Sync', binding);
      else setText('[data-backups-next]', 'Not scheduled', binding);
    }
    function scheduleForBucket(bucketName) { return jobs().find(job => job.bucket === bucketName || job.schedule_id === `backblaze-${bucketName}`) || null; }
    function renderSnapshotPicker(binding = state.binding) {
      const select = el('[data-snapshot-bucket]', binding);
      const ask = el('[data-snapshots-ask]', binding);
      if (!select || !ask) return;
      const current = select.value;
      select.replaceChildren();
      buckets().forEach(bucket => { const option = document.createElement('option'); option.value = bucket.bucket; option.textContent = bucket.bucket; select.append(option); });
      if (buckets().some(bucket => bucket.bucket === current)) select.value = current;
      select.hidden = buckets().length <= 1;
      ask.disabled = !buckets().length;
    }
    function renderLedgerRows(binding = state.binding) {
      const host = el('[data-ledger-rows]', binding);
      if (!host) return;
      const rows = state.ledgerRows.filter(row => {
        if (state.ledgerFilter === 'drives') return ledgerKind(row) === 'drives';
        if (state.ledgerFilter === 'offsite') return ledgerKind(row) === 'offsite';
        if (state.ledgerFilter === 'problems') return String(row.outcome || '').toLowerCase() !== 'ok';
        return true;
      });
      if (!rows.length) {
        host.innerHTML = `<tr><td colspan="4" class="backup-empty">${state.ledgerLoading ? 'Loading…' : state.ledgerError || 'No backup activity recorded.'}</td></tr>`;
        return;
      }
      host.innerHTML = rows.map(row => {
        const outcome = String(row.outcome || '').toLowerCase();
        const status = outcome === 'ok' ? 'OK' : outcome === 'paused' ? 'Paused' : 'Failed';
        const reason = outcome === 'ok' ? '' : signalReason(row.signal);
        return `<tr><td data-label="When">${esc(formatDate(row.at, 'Earlier'))}</td><td data-label="What">${esc(ledgerWhat(row))}</td><td data-label="Result"><strong class="backup-result backup-result--${outcome === 'ok' ? 'ok' : outcome === 'paused' ? 'paused' : 'failed'}">${status}</strong>${reason ? `<small>${esc(reason)}</small>` : ''}</td><td data-label="Took">${esc(formatDuration(row.duration_ms))}</td></tr>`;
      }).join('');
    }
    async function loadLedgerPage(reset = false, binding = state.binding) {
      if (!current(binding)) return false;
      const modal = root(binding)?.querySelector('[data-backups-modal="ledger"]');
      if (!modal || modal.hidden) return false;
      const before = reset ? null : state.ledgerNext;
      if (!reset && before == null) return false;
      state.ledgerLoading = true;
      state.ledgerError = '';
      renderLedgerRows(binding);
      const button = el('[data-ledger-more]', binding); if (button) button.disabled = true;
      return scaffoldRegistry.run(binding, 'backblaze-ledger-page', async context => {
        const body = await readJson(context, '/api/backups/ledger', { ...(before == null ? {} : { before }), limit: 50 });
        if (!context.current() || !current(binding) || modal.hidden) return false;
        try {
          const page = normalizeLedgerResponse(body);
          state.ledgerRows = reset ? page : [...state.ledgerRows, ...page];
          state.ledgerNext = Number.isInteger(body.next) ? body.next : null;
          state.ledgerError = '';
        } catch (error) { state.ledgerError = error.message || 'Ledger could not be read.'; }
        state.ledgerLoading = false;
        renderLedgerRows(binding);
        const more = el('[data-ledger-more]', binding); if (more) { more.hidden = state.ledgerNext == null; more.disabled = false; }
        return true;
      }, { slots: [], timeoutMs: 10000, slowMs: 1200, retry: false, onFailure: ({ reason }) => {
        if (!current(binding) || modal.hidden) return;
        state.ledgerError = reason || 'Ledger could not be read.';
        state.ledgerLoading = false;
        renderLedgerRows(binding);
      } }).finally(() => {
        state.ledgerLoading = false;
        const more = el('[data-ledger-more]', binding); if (more) more.disabled = false;
      });
    }
    function ledgerKind(row) {
      const verb = String(row.verb || '');
      return row.job === 'drives' || row.target === 'nas-sync' || verb.startsWith('nas-sync') || verb === 'pid2-fire-interrupted-by-boot' ? 'drives' : 'offsite';
    }
    function ledgerWhat(row) {
      const verb = String(row.verb || ''), target = String(row.target || '');
      if (verb === 'backblaze-bucket-run') {
        const bucket = bucketFor(target), manager = bucket?.managers?.forgejo || {};
        const included = [];
        if (manager.backblazeBackup === true) included.push('Git server');
        if (Array.isArray(bucket?.items) && bucket.items.length) included.push('folders');
        return `Off-site copy to ${target}${included.length ? ` · ${included.join(' · ')}` : ''}`;
      }
      if (verb === 'backblaze-forgejo-b2-push') return `Git server copied to ${target}`;
      if (verb === 'backblaze-bucket-add') return `Off-site copy added: ${target}`;
      if (verb === 'backblaze-bucket-save') return `Off-site settings saved for ${target}`;
      if (verb === 'backblaze-bucket-verify') return `Connection checked: ${target}`;
      if (verb === 'backblaze-bucket-remove') return `Off-site copy removed: ${target}`;
      if (verb === 'backblaze-bucket-credentials') return `Credentials updated for ${target}`;
      if (verb === 'backblaze-restic-unlock') return `Off-site copy unlocked for ${target}`;
      if (verb === 'backblaze-restic-rekey') return `Off-site copy key changed for ${target}`;
      if (verb === 'nas-sync-sync' || (target === 'nas-sync' && verb.endsWith('sync'))) return 'Backup drives mirrored';
      if (verb.includes('sync-hold')) return 'Backup drives swapped · nightly mirror paused';
      if (verb.includes('sync-release')) return 'Backup drives turned back on';
      if (verb === 'pid2-fire-interrupted-by-boot') return 'Re-run after a restart or power loss';
      return verb ? verb.replaceAll('-', ' ') : 'Backup activity';
    }
    function signalReason(signal) {
      if (typeof signal !== 'string' || !signal) return '';
      const known = {
        'bucket-unreachable': "The key works, but it can't see a bucket with that name.",
        'bucket-not-found': "The key works, but it can't see a bucket with that name.",
        'invalid-credentials': 'The key or application key could not be used.',
        'access-denied': 'Access was denied by the storage provider.',
        'nas-unavailable': 'A backup drive was not available.',
        'drive-unavailable': 'A backup drive was not available.'
      };
      return known[signal] || signal;
    }
    function openModal(name, trigger = null, binding = state.binding) {
      if (!current(binding)) return;
      if (name === 'add' && !canRewriteConfig(binding)) return;
      state.returnFocus = trigger instanceof HTMLElement ? trigger : document.activeElement;
      const modal = root(binding)?.querySelector(`[data-backups-modal="${CSS.escape(name)}"]`);
      if (!modal) return;
      modal.hidden = false;
      state.activeModal = name;
      if (name === 'add') resetAddModal('add', null, binding);
      if (name === 'skip') void openSkipModal(trigger?.dataset.bucket || trigger?.dataset.openSkip, binding);
      if (name === 'folders') void openFolderModal(trigger?.dataset.bucket || trigger?.dataset.openFolders, binding);
      if (name === 'schedule') void openScheduleModal(binding);
      if (name === 'ledger') {
        state.ledgerRows = []; state.ledgerNext = null; state.ledgerFilter = 'all'; state.ledgerError = ''; state.ledgerLoading = false; state.snapshots = [];
        el('[data-snapshots-result]', binding)?.replaceChildren();
        const filter = el('[data-ledger-filter]', binding); if (filter) filter.value = 'all';
        const more = el('[data-ledger-more]', binding); if (more) more.hidden = true;
        renderLedgerRows(binding); renderSnapshotPicker(binding); void loadLedgerPage(true, binding);
      }
      if (name === 'confirm') {
        const yes = modal.querySelector('[data-snapshots-yes]');
        if (yes) yes.disabled = false;
        window.setTimeout(() => { if (current(binding) && !modal.hidden) yes?.focus(); }, 0);
        return;
      }
      const focus = modal.querySelector('input:not([type="checkbox"]):not([disabled]), select:not([disabled]), button:not([disabled])');
      window.setTimeout(() => { if (current(binding) && !modal.hidden) focus?.focus(); }, 0);
    }
    function closeModal(name, binding = state.binding) {
      const modal = root(binding)?.querySelector(`[data-backups-modal="${CSS.escape(name)}"]`);
      if (modal) modal.hidden = true;
      if (name === 'confirm') {
        const ledger = root(binding)?.querySelector('[data-backups-modal="ledger"]');
        state.activeModal = ledger && !ledger.hidden ? 'ledger' : null;
        return;
      }
      if (name === 'add') {
        scrubAddModal(binding); state.addMode = 'add'; state.editBucket = null;
      }
      if (name === 'skip') scrubSkipModal(binding);
      if (name === 'folders') scrubFolderModal(binding);
      if (name === 'ledger') {
        state.ledgerRows = []; state.ledgerNext = null; state.ledgerError = ''; state.snapshots = [];
        el('[data-snapshots-result]', binding)?.replaceChildren();
      }
      state.activeModal = null;
      if (state.returnFocus instanceof HTMLElement && state.returnFocus.isConnected) state.returnFocus.focus();
      state.returnFocus = null;
    }
    function resetAddModal(mode, bucketName, binding = state.binding) {
      const modal = root(binding)?.querySelector('[data-backups-modal="add"]'); if (!modal) return;
      state.addMode = mode; state.editBucket = bucketName;
      const title = modal.querySelector('h2'), description = modal.querySelector('h2 + p'), link = modal.querySelector('a'), submit = modal.querySelector('[data-add-submit]');
      if (submit) submit.disabled = false;
      const bucketLabel = modal.querySelector('[data-add-bucket]')?.closest('label');
      const extras = modal.querySelectorAll('.backup-muted');
      const disclaimer = [...modal.querySelectorAll('p')].find(node => node.textContent.includes('HomeServer is not affiliated with Backblaze.'));
      if (mode === 'add') {
        if (title) title.textContent = 'Backblaze B2 — Recommended';
        if (description) description.textContent = 'Encrypted cloud storage for a copy away from home.';
        if (link) link.hidden = false;
        if (bucketLabel) bucketLabel.hidden = false;
        extras.forEach(node => { node.hidden = false; });
        if (disclaimer) disclaimer.hidden = false;
        if (submit) submit.textContent = 'Add';
      } else {
        if (title) title.textContent = 'Update credentials';
        if (description) description.textContent = `Enter new Backblaze credentials for ${bucketName}.`;
        if (link) link.hidden = true;
        if (bucketLabel) bucketLabel.hidden = true;
        extras.forEach(node => { node.hidden = true; });
        if (disclaimer) disclaimer.hidden = true;
        if (submit) submit.textContent = 'Save credentials';
      }
      modalResult('add', '', binding);
    }
    function scrubAddModal(binding = state.binding) {
      const modal = root(binding)?.querySelector('[data-backups-modal="add"]'); if (!modal) return;
      modal.querySelectorAll('[data-add-bucket], [data-add-key], [data-add-secret]').forEach(input => { input.value = ''; });
      modalResult('add', '', binding);
    }
    function scrubSkipModal(binding = state.binding) {
      const modal = root(binding)?.querySelector('[data-backups-modal="skip"]');
      modal?.querySelector('[data-skip-input]') && (modal.querySelector('[data-skip-input]').value = '');
      modal?.querySelector('[data-skip-chips]')?.replaceChildren();
      if (modal) modal.querySelectorAll('[data-backup-local-result]').forEach(node => node.remove());
      state.skipBucket = null; state.skipDraft = []; state.skipChecks.clear();
    }
    function scrubFolderModal(binding = state.binding) {
      const modal = root(binding)?.querySelector('[data-backups-modal="folders"]');
      modal?.querySelector('[data-folder-list]')?.replaceChildren();
      if (modal) modal.querySelectorAll('[data-backup-local-result]').forEach(node => node.remove());
      state.folderBucket = null; state.folderPath = '/mnt/nas'; state.folderNames = []; state.folderDraft = new Set(); state.folderBaseline = []; state.folderTruncated = false;
    }
    function scrubProtectedState(pane) {
      state.data = null; state.statusReady = false; state.statusStale = true; state.drives = []; state.drivesReady = false;
      state.summaryRows = []; state.summaryReady = false; state.ledgerRows = []; state.ledgerNext = null; state.ledgerLoading = false; state.ledgerFilter = 'all'; state.ledgerError = '';
      state.readErrors = {}; state.scheduleBaseline = null; state.menuBucket = null; state.snapshots = []; state.activeModal = null; state.returnFocus = null; state.addMode = 'add'; state.editBucket = null;
      scrubAddModal({ root: pane }); scrubSkipModal({ root: pane }); scrubFolderModal({ root: pane });
      if (!(pane instanceof Element) || !pane.isConnected) return;
      pane.querySelectorAll('[data-backups-drives], [data-backups-buckets], [data-ledger-rows], [data-folder-list], [data-skip-chips], [data-snapshots-result]').forEach(node => node.replaceChildren());
      pane.querySelectorAll('[data-backups-happened], [data-backups-next], [data-backups-result], [data-folder-selected], [data-folder-path]').forEach(node => { node.textContent = ''; });
      pane.querySelectorAll('[data-add-bucket], [data-add-key], [data-add-secret], [data-skip-input]').forEach(node => { node.value = ''; });
      pane.querySelectorAll('[data-snapshot-bucket]').forEach(node => node.replaceChildren());
      pane.querySelectorAll('[data-backups-modal] [data-schedule-frequency]').forEach(node => { node.value = 'daily'; });
      pane.querySelectorAll('[data-backups-modal] [data-schedule-time]').forEach(node => { node.value = '02:00'; });
      pane.querySelectorAll('[data-backups-modal] [data-schedule-day]').forEach(node => { node.value = 'Mon'; });
      pane.querySelectorAll('[data-backups-modal] [data-schedule-on]').forEach(node => { node.checked = false; });
      pane.querySelectorAll('[data-backups-modal]').forEach(node => { node.hidden = true; });
      pane.querySelectorAll('[data-backups-open], [data-backups-sync], [data-settings-toggle], [data-bucket-action], [data-open-skip], [data-open-folders], [data-git-toggle], [data-folder-up], [data-folder-save], [data-skip-add], [data-skip-save], [data-ledger-more], [data-snapshots-ask], [data-snapshots-yes]').forEach(node => { if ('disabled' in node) node.disabled = true; });
    }
    function retireBackblaze(binding, privilegedLoss = false) {
      if (state.binding !== binding) return;
      const roleLost = privilegedLoss || !roleAuthorized();
      actions.forEach(controller => controller.abort()); actions.clear();
      state.binding = null;
      if (roleLost) { scrubProtectedState(binding.root); return; }
      state.statusStale = true;
      state.drivesReady = false;
      state.summaryReady = false;
      if (binding.root instanceof Element) {
        binding.root.querySelectorAll('[data-add-secret], [data-add-key]').forEach(input => { input.value = ''; });
        binding.root.querySelectorAll('[data-backups-modal]').forEach(modal => { modal.hidden = true; });
      }
    }
    function adoptBackblazeBinding() {
      const binding = admitted() && roleAuthorized() ? scaffoldRegistry.capture('backblaze') : null;
      if (!binding) {
        if (!roleAuthorized()) { if (state.binding) retireBackblaze(state.binding, true); else scrubProtectedState(document.querySelector('[data-pane-panel="backblaze"]')); }
        return null;
      }
      if (state.binding !== binding) {
        const previous = state.binding;
        if (previous) { scaffoldRegistry.retire(previous, 'Backups binding replaced'); retireBackblaze(previous); }
        state.binding = binding;
        scaffoldRegistry.onRetire(binding, () => retireBackblaze(binding));
      }
      return binding;
    }
    function paneActive() {
      const pane = document.querySelector('[data-pane-panel="backblaze"]');
      return Boolean(pane?.classList.contains('active') && admitted());
    }
    function observeBackblazeActivity() {
      const pane = document.querySelector('[data-pane-panel="backblaze"]');
      if (!(pane instanceof Element)) return;
      let wasActive = false;
      const check = () => {
        const active = paneActive();
        if (active && !wasActive) void hydrateBackblazeEntry();
        wasActive = active;
      };
      new MutationObserver(check).observe(pane, { attributes: true, attributeFilter: ['class'] });
      check();
    }
    window.addEventListener('coronatio:privileged-state-retired', () => {
      if (state.binding) { retireBackblaze(state.binding, true); return; }
      scrubProtectedState(document.querySelector('[data-pane-panel="backblaze"]'));
    });
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane, binding = state.binding;
      if (detail.id !== 'backblaze' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="backblaze"]') !== pane) return;
      if (binding && (!pane.contains(binding.root) || scaffoldRegistry.current(binding))) return;
      actions.forEach(controller => controller.abort()); actions.clear();
      if (binding) { scaffoldRegistry.retire(binding, 'Backups pane reset'); state.binding = null; }
      scrubProtectedState(pane);
      state.activeModal = null; state.returnFocus = null;
    });
    async function hydrateBackblazeEntry() {
      const binding = adoptBackblazeBinding();
      if (!binding || !current(binding)) return false;
      return loadMain(binding);
    }
    function safeMainDrivePath(path) {
      return typeof path === 'string' && (path === '/mnt/nas' || path.startsWith('/mnt/nas/')) && !path.split('/').some(part => part === '.' || part === '..');
    }
    function nearestFolderPath(path) {
      if (path === '/mnt/nas') return '/mnt/nas';
      const parts = path.split('/').filter(Boolean);
      parts.pop();
      return `/${parts.join('/')}` || '/mnt/nas';
    }
    function safeFolderChild(parent, name) {
      if (typeof name !== 'string' || !name || name === '.' || name === '..' || name.includes('/') || name.includes('\\') || name.includes('\0')) return null;
      return `${parent.replace(/\/$/, '')}/${name}`;
    }
    function renderFolderList(binding = state.binding) {
      const host = el('[data-folder-list]', binding); if (!host) return;
      const paths = state.folderNames.map(name => safeFolderChild(state.folderPath, name)).filter(Boolean);
      if (!paths.length) { host.innerHTML = '<p class="backup-empty">No folders here.</p>'; return; }
      host.innerHTML = paths.map(path => {
        const name = path.slice(path.lastIndexOf('/') + 1);
        return `<div class="backup-folder-row"><label><input type="checkbox" data-folder-select="${esc(path)}" ${state.folderDraft.has(path) ? 'checked' : ''}><span>${esc(name)}</span></label><button type="button" data-folder-enter="${esc(path)}" aria-label="Pick inside ${esc(name)}">Pick inside</button></div>`;
      }).join('');
    }
    function updateFolderSummary(binding = state.binding) {
      setText('[data-folder-selected]', `Selected: ${state.folderDraft.size} folders`, binding);
      setText('[data-folder-path]', state.folderPath, binding);
      const up = el('[data-folder-up]', binding); if (up) up.hidden = state.folderPath === '/mnt/nas';
    }
    async function loadFolders(path, binding = state.binding) {
      if (!current(binding) || !state.folderBucket || !safeMainDrivePath(path)) return false;
      state.folderPath = path;
      state.folderNames = [];
      renderFolderList(binding); updateFolderSummary(binding);
      modalResult('folders', 'Reading this folder from the main drive…', binding);
      return scaffoldRegistry.run(binding, 'backblaze-folders', async context => {
        const body = await readJson(context, '/api/backups/folders', { path });
        if (!context.current() || !current(binding) || state.folderPath !== path) return false;
        if (!isRecord(body) || body.ok === false || !Array.isArray(body.folders) || body.path !== path) throw new Error(body?.reason || 'Folders could not be read');
        state.folderNames = body.folders.filter(name => typeof name === 'string' && name && !name.includes('/') && name !== '.' && name !== '..');
        state.folderTruncated = body.truncated === true;
        renderFolderList(binding); updateFolderSummary(binding);
        modalResult('folders', state.folderTruncated ? 'Some folders are not shown.' : '', binding);
        return true;
      }, { slots: [], timeoutMs: 10000, slowMs: 1200, retry: false, onFailure: () => { if (current(binding) && state.folderPath === path) modalResult('folders', 'Folders could not be read from the main drive.', binding); } }).catch(() => {
        if (current(binding) && state.folderPath === path) modalResult('folders', 'Folders could not be read from the main drive.', binding);
        return false;
      });
    }
    async function openFolderModal(bucketName, binding = state.binding) {
      const bucket = bucketFor(bucketName);
      if (!bucket || !canRewriteConfig(binding) || !bucketWritable(bucket)) { closeModal('folders', binding); return; }
      state.folderBucket = bucketName;
      state.folderPath = '/mnt/nas';
      state.folderBaseline = bucket.items.map(item => item.path);
      state.folderDraft = new Set(state.folderBaseline);
      const save = root(binding)?.querySelector('[data-backups-modal="folders"] [data-folder-save]'); if (save) save.disabled = false;
      const up = el('[data-folder-up]', binding); if (up) up.disabled = false;
      await loadFolders('/mnt/nas', binding);
    }
    async function checkSkipName(name, binding = state.binding) {
      if (!current(binding) || !validSkip(name)) return null;
      const result = await scaffoldRegistry.run(binding, 'backblaze-skip-check', async context => {
        try {
          const body = await readJson(context, '/api/backups/skip-check', { name });
          if (!context.current() || !current(binding)) return null;
          if (!isRecord(body) || body.ok === false || body.name !== name || typeof body.matches !== 'boolean') return null;
          return { matches: body.matches };
        } catch (_) { return null; }
      }, { slots: [], timeoutMs: 8000, slowMs: 1000, retry: false });
      return isRecord(result) && typeof result.matches === 'boolean' ? result.matches : null;
    }
    function renderSkipChips(binding = state.binding) {
      const host = el('[data-skip-chips]', binding); if (!host) return;
      if (!state.skipDraft.length) { host.innerHTML = '<p class="backup-empty">No names to skip.</p>'; return; }
      host.innerHTML = state.skipDraft.map((name, index) => {
        const match = state.skipChecks.get(name);
        const kind = match === true ? 'matches' : match === false ? 'misses' : match === null ? 'unavailable' : 'checking';
        const mark = match === true ? '✓' : match === false ? '!' : match === null ? '?' : '…';
        const note = match === false ? `<small>${esc(name)} matches nothing on your Git server. Check the spelling.</small>` : match === null ? '<small>Name check unavailable.</small>' : '';
        return `<div class="backup-skip-chip backup-skip-chip--${kind}"><span aria-hidden="true">${mark}</span><strong>${esc(name)}</strong><button type="button" data-skip-remove="${index}" aria-label="Remove ${esc(name)}">Remove</button>${note}</div>`;
      }).join('');
    }
    async function openSkipModal(bucketName, binding = state.binding) {
      const bucket = bucketFor(bucketName), manager = bucket?.managers?.forgejo;
      if (!bucket || !bucketWritable(bucket) || !canRewriteConfig(binding) || !Array.isArray(manager?.skip)) { closeModal('skip', binding); return; }
      state.skipBucket = bucketName;
      state.skipDraft = [...manager.skip];
      state.skipChecks = new Map(state.skipDraft.map(name => [name, undefined]));
      const add = root(binding)?.querySelector('[data-backups-modal="skip"] [data-skip-add]'); if (add) add.disabled = false;
      const save = root(binding)?.querySelector('[data-backups-modal="skip"] [data-skip-save]'); if (save) save.disabled = false;
      renderSkipChips(binding);
      await scaffoldRegistry.run(binding, 'backblaze-skip-open-checks', async context => {
        for (const name of state.skipDraft) {
          if (!context.current() || !current(binding) || state.skipBucket !== bucketName) return false;
          try {
            const body = await readJson(context, '/api/backups/skip-check', { name });
            if (!context.current() || !current(binding) || state.skipBucket !== bucketName) return false;
            if (isRecord(body) && body.ok !== false && body.name === name && typeof body.matches === 'boolean') state.skipChecks.set(name, body.matches);
            else state.skipChecks.set(name, null);
          } catch (_) { state.skipChecks.set(name, null); }
          renderSkipChips(binding);
        }
        return true;
      }, { slots: [], timeoutMs: 20000, slowMs: 1500, retry: false, onFailure: () => {
        if (!current(binding) || state.skipBucket !== bucketName) return;
        state.skipDraft.forEach(name => { if (state.skipChecks.get(name) === undefined) state.skipChecks.set(name, null); });
        renderSkipChips(binding);
      } });
    }
    function scheduleBaseFromData() {
      const nas = jobs().find(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      const nasSchedule = scheduleFor(nas);
      const scheduled = jobs().map(job => ({ job, schedule: scheduleFor(job) })).filter(item => item.schedule);
      const onlyManual = scheduled.length > 0 && scheduled.every(item => !item.schedule.enabled);
      if (onlyManual) {
        const source = nasSchedule || scheduleFor(scheduled.find(item => item.job.kind !== 'nas')?.job);
        const base = source && !nasSchedule ? addMinutes(source.time, -1, source.weekday) : null;
        return { frequency: 'manual', time: base?.time || source?.time || '02:00', weekday: base?.weekday || source?.weekday || 'Mon', on: false, nasPresent: Boolean(nas) };
      }
      if (nasSchedule) return { frequency: nasSchedule.frequency, time: nasSchedule.time, weekday: nasSchedule.weekday, on: nasSchedule.enabled, nasPresent: true };
      const offsite = scheduled.find(item => item.job.kind !== 'nas')?.schedule;
      if (offsite) {
        const prior = addMinutes(offsite.time, -1, offsite.weekday);
        return { frequency: offsite.frequency, time: prior.time, weekday: prior.weekday, on: false, nasPresent: false };
      }
      return { frequency: 'daily', time: '02:00', weekday: 'Mon', on: false, nasPresent: Boolean(nas) };
    }
    function addMinutes(time, delta, weekday = 'Mon') {
      const match = /^(\d{2}):(\d{2})$/.exec(String(time || ''));
      if (!match) return { time: '02:00', weekday };
      let total = Number(match[1]) * 60 + Number(match[2]) + delta;
      let dayShift = Math.floor(total / 1440);
      total = ((total % 1440) + 1440) % 1440;
      const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
      let index = Math.max(0, days.indexOf(weekday));
      index = (index + dayShift + 7) % 7;
      return { time: `${String(Math.floor(total / 60)).padStart(2, '0')}:${String(total % 60).padStart(2, '0')}`, weekday: days[index] };
    }
    function renderScheduleOrder(binding = state.binding) {
      const order = el('[data-schedule-order]', binding);
      if (!order) return;
      const frequency = el('[data-schedule-frequency]', binding)?.value || 'daily';
      const time = el('[data-schedule-time]', binding)?.value || '02:00';
      const weekday = el('[data-schedule-day]', binding)?.selectedOptions?.[0]?.textContent || 'Monday';
      const nasPresent = jobs().some(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      const lines = [];
      if (nasPresent) lines.push('<li>Backup drives: mirror the main drive to each backup drive</li>');
      buckets().forEach(bucket => lines.push(`<li>Off-site: ${esc(bucket.bucket)} at Backblaze B2</li>`));
      const when = frequency === 'manual' ? 'Only when I press Sync' : frequency === 'weekly' ? `Once a week on ${esc(weekday)} at ${esc(time)}` : `Each night at ${esc(time)}`;
      order.innerHTML = `<p>${when}${lines.length ? ', in this order:' : '.'}</p>${lines.length ? `<ol>${lines.join('')}</ol>` : ''}`;
    }
    function renderScheduleForm(binding = state.binding) {
      const base = scheduleBaseFromData();
      const paused = el('[data-schedule-paused]', binding);
      if (paused) paused.hidden = !base.nasPresent || base.on;
      const on = el('[data-schedule-on]', binding); if (on) on.checked = base.on;
      const frequency = el('[data-schedule-frequency]', binding); if (frequency) frequency.value = base.frequency;
      const time = el('[data-schedule-time]', binding); if (time) time.value = base.time;
      const day = el('[data-schedule-day]', binding); if (day) day.value = base.weekday;
      const dayWrap = el('[data-schedule-day-wrap]', binding); if (dayWrap) dayWrap.hidden = base.frequency !== 'weekly';
      renderScheduleOrder(binding);
      modalResult('schedule', state.statusStale ? 'Current schedule details are unavailable; saving is disabled.' : '', binding);
      const save = root(binding)?.querySelector('[data-backups-modal="schedule"] [data-schedule-save]');
      const hasSchedule = buckets().length > 0 || jobs().some(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      if (save) save.disabled = !canUse(binding) || !hasSchedule;
    }
    async function openScheduleModal(binding = state.binding) {
      state.scheduleBaseline = null;
      renderScheduleForm(binding);
      try {
        await refreshStatus(binding);
        if (current(binding) && state.activeModal === 'schedule') {
          state.scheduleBaseline = scheduleSnapshot(jobs());
          renderScheduleForm(binding);
        }
      } catch (_) { if (current(binding)) { state.statusStale = true; renderScheduleForm(binding); } }
    }
    function renderSnapshotRows(rows, binding = state.binding) {
      const host = el('[data-snapshots-result]', binding); if (!host) return;
      if (!rows.length) { host.textContent = 'No stored copies were returned.'; return; }
      host.innerHTML = `<ul class="backup-snapshot-list">${rows.map(row => `<li><strong>Stored copy</strong><span>${esc(formatDate(row.time, 'Date unavailable'))}</span><span>${esc(row.status || 'Available')}</span></li>`).join('')}</ul>`;
    }
    async function listSnapshotsAfterYes(binding = state.binding) {
      if (!current(binding) || state.activeModal !== 'confirm') return;
      const bucket = el('[data-snapshot-bucket]', binding)?.value;
      if (!bucket || !bucketFor(bucket)) { modalResult('ledger', 'Choose an off-site copy first.', binding); return; }
      closeModal('confirm', binding);
      const result = el('[data-snapshots-result]', binding); if (result) result.textContent = 'Asking Backblaze to list stored copies…';
      const ask = el('[data-snapshots-ask]', binding); if (ask) ask.disabled = true;
      const controller = actionController(binding);
      try {
        const body = await getJson(`/api/backblaze/snapshots?bucket=${encodeURIComponent(bucket)}`, { signal: controller.signal }, binding);
        if (!current(binding) || state.activeModal !== 'ledger') return;
        if (body?.ok !== true || !Array.isArray(body.snapshots)) throw new Error(body?.reason || 'Stored copies could not be listed');
        state.snapshots = body.snapshots.filter(isRecord).map(row => ({ time: row.time, status: row.status }));
        renderSnapshotRows(state.snapshots, binding);
      } catch (_) { if (current(binding)) { if (result) result.textContent = 'Stored copies could not be listed.'; } }
      finally { actions.delete(controller); if (current(binding) && ask) ask.disabled = false; }
    }
    async function ensureFreshConfig(bucket, binding, signal = null) {
      const body = await getJson('/api/backblaze/status', { signal }, binding);
      if (!current(binding) || body?.ok === false) throw new Error('The current configuration could not be read. No changes were saved.');
      const fresh = projectStatus(body), freshBucket = fresh.buckets.find(row => row.bucket === bucket.bucket);
      const baseline = configRow(bucket);
      const currentRow = configRow(freshBucket);
      if (!currentRow || stable(currentRow) !== stable(baseline)) {
        applyStatus(fresh); renderMain(binding);
        throw new Error('This copy changed elsewhere. The view was refreshed; review it before saving again.');
      }
      return { expected: baseline, fresh: freshBucket };
    }
    async function saveBucketConfig(bucketName, edit, binding = state.binding) {
      const bucket = bucketFor(bucketName);
      if (!bucket || !canRewriteConfig(binding) || !bucketWritable(bucket)) return false;
      const generation = state.binding;
      const controller = actionController(binding);
      try {
        const { expected, fresh } = await ensureFreshConfig(bucket, binding, controller.signal);
        if (!current(binding) || generation !== state.binding) return false;
        const edited = configRow(fresh);
        edit(edited);
        const body = await getJson('/api/backblaze/config', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ buckets: [edited], expected: [expected] }), signal: controller.signal }, binding);
        if (!current(binding)) return false;
        if (body?.ok !== true) throw new Error(body?.reason || 'Configuration could not be saved');
        await loadMain(binding);
        return true;
      } finally { actions.delete(controller); }
    }
    async function saveSkipNames(binding = state.binding) {
      const bucketName = state.skipBucket;
      const input = el('[data-skip-input]', binding);
      const raw = input?.value.trim() || '';
      if (!bucketName || !canRewriteConfig(binding)) return;
      if (!validSkip(raw)) { modalResult('skip', 'Use letters, numbers, hyphen, underscore, or period; one optional slash separates owner and repository.', binding); return; }
      if (state.skipDraft.some(name => name.toLowerCase() === raw.toLowerCase())) { modalResult('skip', 'That name is already in the list.', binding); return; }
      const add = root(binding)?.querySelector('[data-backups-modal="skip"] [data-skip-add]'); if (add) add.disabled = true;
      try {
        const matches = await checkSkipName(raw, binding);
        if (!current(binding) || state.skipBucket !== bucketName) return;
        state.skipDraft.push(raw); state.skipChecks.set(raw, matches); if (input) input.value = '';
        renderSkipChips(binding); modalResult('skip', '', binding);
      } finally { if (add && current(binding) && state.skipBucket === bucketName) add.disabled = false; }
    }
    async function commitSkipNames(binding = state.binding) {
      const bucketName = state.skipBucket;
      if (!bucketName || !canRewriteConfig(binding)) return;
      const save = root(binding)?.querySelector('[data-backups-modal="skip"] [data-skip-save]'); if (save) save.disabled = true;
      try {
        const ok = await saveBucketConfig(bucketName, edited => {
          edited.managers = { ...edited.managers, forgejo: { ...edited.managers.forgejo, skip: [...state.skipDraft] } };
        }, binding);
        if (ok && current(binding)) { closeModal('skip', binding); setMainResult('Skip names saved.', binding); }
        else if (current(binding)) modalResult('skip', 'Skip names could not be saved.', binding);
      } catch (error) { if (current(binding)) modalResult('skip', error.message || 'Skip names could not be saved.', binding); }
      finally { if (save && current(binding)) save.disabled = false; }
    }
    async function commitFolders(binding = state.binding) {
      const bucketName = state.folderBucket, bucket = bucketFor(bucketName);
      if (!bucketName || !bucket || !canRewriteConfig(binding)) return;
      const save = root(binding)?.querySelector('[data-backups-modal="folders"] [data-folder-save]'); if (save) save.disabled = true;
      try {
        const preflight = actionController(binding);
        let freshBody;
        try { freshBody = await getJson('/api/backblaze/status', { signal: preflight.signal }, binding); }
        finally { actions.delete(preflight); }
        if (!current(binding) || freshBody?.ok === false) throw new Error('The current configuration could not be read. No changes were saved.');
        const fresh = projectStatus(freshBody), freshBucket = fresh.buckets.find(row => row.bucket === bucketName);
        const currentPaths = Array.isArray(freshBucket?.items) ? freshBucket.items.map(item => item.path) : null;
        if (!currentPaths || stable([...currentPaths].sort()) !== stable([...state.folderBaseline].sort())) {
          applyStatus(fresh); renderMain(binding);
          throw new Error('This copy changed elsewhere. The view was refreshed; review it before saving again.');
        }
        const desired = [...state.folderDraft];
        const remove = state.folderBaseline.filter(path => !state.folderDraft.has(path));
        const add = desired.filter(path => !state.folderBaseline.includes(path));
        for (const [method, paths] of [['DELETE', remove], ['POST', add]]) {
          for (const path of paths) {
            if (!current(binding) || state.folderBucket !== bucketName) return;
            const controller = actionController(binding);
            try {
              const body = await getJson(`/api/backblaze/buckets/${encodeURIComponent(bucketName)}/items`, { method, headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ path }), signal: controller.signal }, binding);
              if (!current(binding) || body?.ok !== true) throw new Error(body?.reason || 'Folder selection could not be saved');
            } finally { actions.delete(controller); }
          }
        }
        await loadMain(binding);
        if (current(binding)) { closeModal('folders', binding); setMainResult('Folders saved.', binding); }
      } catch (error) { if (current(binding)) modalResult('folders', error.message || 'Folders could not be saved.', binding); }
      finally { if (save && current(binding)) save.disabled = false; }
    }
    function schedulePayload(id, frequency, time, weekday, enabled) {
      return { action: 'schedule-save', schedule_id: id, frequency, time, weekday: frequency === 'weekly' ? weekday : 'Mon', enabled, timestamp: new Date().toISOString() };
    }
    async function saveSchedule(binding = state.binding) {
      if (!canUse(binding) || !state.scheduleBaseline) return;
      const frequency = el('[data-schedule-frequency]', binding)?.value || 'daily';
      const time = el('[data-schedule-time]', binding)?.value || '02:00';
      const weekday = el('[data-schedule-day]', binding)?.value || 'Mon';
      const nas = jobs().find(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      const on = el('[data-schedule-on]', binding)?.checked === true;
      const save = root(binding)?.querySelector('[data-backups-modal="schedule"] [data-schedule-save]'); if (save) save.disabled = true;
      const controller = actionController(binding);
      try {
        const freshBody = await getJson('/api/backblaze/status', { signal: controller.signal }, binding);
        if (!current(binding) || freshBody?.ok === false) throw new Error('Current schedules could not be read. Nothing was changed.');
        const fresh = projectStatus(freshBody);
        if (stable(scheduleSnapshot(fresh.jobs)) !== stable(state.scheduleBaseline)) {
          applyStatus(fresh); renderMain(binding); renderScheduleForm(binding);
          throw new Error('A schedule changed elsewhere. Review the current schedule before saving again.');
        }
        if (nas) {
          const nasEnabled = frequency !== 'manual' && on;
          const body = await getJson('/api/backblaze/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(schedulePayload('nas-sync', frequency, time, weekday, nasEnabled)), signal: controller.signal }, binding);
          if (!current(binding) || body?.ok !== true) throw new Error(body?.reason || 'The drive schedule could not be saved.');
        }
        const later = addMinutes(time, 1, weekday);
        for (const bucket of buckets()) {
          const id = scheduleForBucket(bucket.bucket)?.schedule_id || `backblaze-${bucket.bucket}`;
          const enabled = frequency !== 'manual';
          const body = await getJson('/api/backblaze/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(schedulePayload(id, frequency, later.time, later.weekday, enabled)), signal: controller.signal }, binding);
          if (!current(binding) || body?.ok !== true) throw new Error(body?.reason || `The schedule for ${bucket.bucket} could not be saved.`);
        }
        if (current(binding)) { closeModal('schedule', binding); setMainResult('Schedule saved.', binding); await loadMain(binding); }
      } catch (error) { if (current(binding)) modalResult('schedule', error.message || 'Schedule could not be saved.', binding); }
      finally { actions.delete(controller); if (save && current(binding)) save.disabled = false; }
    }
    async function runBucket(bucketName, binding = state.binding) {
      if (!canUse(binding) || !bucketFor(bucketName)) return;
      const controller = actionController(binding);
      setMainResult(`Starting the off-site copy to ${bucketName}…`, binding);
      try {
        const body = await getJson('/api/backblaze/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ bucket: bucketName }), signal: controller.signal }, binding);
        if (!current(binding)) return;
        if (body?.ok !== true) throw new Error(body?.reason || 'Backup could not be started');
        setMainResult('Off-site copy queued.', binding); await loadMain(binding);
      } catch (error) { if (current(binding)) setMainResult(error.message || 'Backup could not be started.', binding); }
      finally { actions.delete(controller); }
    }
    async function syncNow(binding = state.binding) {
      if (!canUse(binding)) return;
      const nas = jobs().find(job => job.schedule_id === 'nas-sync' || job.kind === 'nas');
      if (!nas) return;
      const button = el('[data-backups-sync]', binding); if (button) button.disabled = true;
      const controller = actionController(binding);
      try {
        const first = await getJson('/api/backblaze/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ schedule_id: 'nas-sync' }), signal: controller.signal }, binding);
        if (!current(binding) || first?.ok !== true) throw new Error(first?.reason || 'Backup drive copy could not be started.');
        for (const bucket of buckets()) {
          const next = await getJson('/api/backblaze/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ bucket: bucket.bucket }), signal: controller.signal }, binding);
          if (!current(binding) || next?.ok !== true) throw new Error(next?.reason || `Off-site copy to ${bucket.bucket} could not be started.`);
        }
        if (current(binding)) { setMainResult('Backup drive copy and off-site copies queued in order.', binding); await loadMain(binding); }
      } catch (error) { if (current(binding)) setMainResult(error.message || 'Backup sequence could not be started.', binding); }
      finally { actions.delete(controller); if (current(binding) && button) button.disabled = false; }
    }
    async function addOrUpdateCredentials(binding = state.binding) {
      if (!canRewriteConfig(binding)) return;
      const mode = state.addMode;
      const bucketName = mode === 'update' ? state.editBucket : el('[data-add-bucket]', binding)?.value.trim();
      let keyId = el('[data-add-key]', binding)?.value.trim() || '';
      let applicationKey = el('[data-add-secret]', binding)?.value || '';
      if (!bucketName || !keyId || !applicationKey || (mode === 'update' && !bucketFor(bucketName))) { modalResult('add', 'Enter the bucket and both credentials.', binding); return; }
      const submit = el('[data-add-submit]', binding); if (submit) submit.disabled = true;
      const controller = actionController(binding);
      const payload = { bucket: bucketName, keyId, applicationKey };
      scrubAddModal(binding);
      try {
        const body = await getJson('/api/backblaze/buckets', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), signal: controller.signal }, binding);
        applicationKey = '';
        if (!current(binding)) return;
        if (body?.ok !== true) throw new Error(mode === 'update' ? 'Credentials could not be updated.' : 'Off-site copy could not be added.');
        closeModal('add', binding); setMainResult(mode === 'update' ? 'Credentials updated.' : 'Off-site copy added.', binding); await loadMain(binding);
      } catch (error) { applicationKey = ''; if (current(binding)) modalResult('add', error.message || 'Off-site copy could not be added.', binding); }
      finally { applicationKey = ''; keyId = ''; payload.applicationKey = ''; payload.keyId = ''; actions.delete(controller); if (current(binding) && submit) submit.disabled = false; }
    }
    async function verifyBucket(bucketName, binding = state.binding) {
      if (!canUse(binding) || !bucketFor(bucketName)) return;
      const controller = actionController(binding);
      setMainResult(`Checking the connection for ${bucketName}…`, binding);
      try {
        const body = await getJson(`/api/backblaze/buckets/${encodeURIComponent(bucketName)}/verify`, { method: 'POST', signal: controller.signal }, binding);
        if (!current(binding)) return;
        if (body?.ok !== true) throw new Error(body?.reason || 'Connection check failed.');
        setMainResult(body.connected === true ? 'Connection checked: connected.' : body.connected === false ? 'Connection checked: not connected.' : 'Connection status unavailable.', binding);
        await loadMain(binding);
      } catch (error) { if (current(binding)) setMainResult(error.message || 'Connection check failed.', binding); }
      finally { actions.delete(controller); }
    }
    async function removeBucket(bucketName, binding = state.binding) {
      if (!canRewriteConfig(binding) || !bucketFor(bucketName) || !window.confirm(`Remove the off-site copy ${bucketName}?`)) return;
      const bucket = bucketFor(bucketName);
      const controller = actionController(binding);
      try {
        await ensureFreshConfig(bucket, binding, controller.signal);
        if (!current(binding)) return;
        const body = await getJson(`/api/backblaze/buckets/${encodeURIComponent(bucketName)}`, { method: 'DELETE', signal: controller.signal }, binding);
        if (!current(binding)) return;
        if (body?.ok !== true) throw new Error(body?.reason || 'Off-site copy could not be removed.');
        setMainResult('Off-site copy removed.', binding); await loadMain(binding);
      } catch (error) { if (current(binding)) setMainResult(error.message || 'Off-site copy could not be removed.', binding); }
      finally { actions.delete(controller); }
    }
    async function toggleGitServer(bucketName, checked, binding = state.binding) {
      const bucket = bucketFor(bucketName);
      if (!bucket || !canRewriteConfig(binding) || !bucketWritable(bucket)) return;
      try {
        const ok = await saveBucketConfig(bucketName, edited => { edited.managers = { ...edited.managers, forgejo: { ...edited.managers.forgejo, backblazeBackup: checked } }; }, binding);
        if (current(binding)) setMainResult(ok ? 'Git server inclusion saved.' : 'Git server inclusion could not be saved.', binding);
      } catch (error) { if (current(binding)) { renderOffsiteCards(binding); setMainResult(error.message || 'Git server inclusion could not be saved.', binding); } }
    }
    function openAddUpdate(bucketName, binding = state.binding) {
      if (!canRewriteConfig(binding)) return;
      openModal('add', null, binding);
      resetAddModal('update', bucketName, binding);
    }
    function snapshotScheduleChanged(data) {
      return stable(scheduleSnapshot(data.jobs)) !== stable(state.scheduleBaseline || []);
    }
    function openFolderChild(path, binding = state.binding) {
      if (!safeMainDrivePath(path) || path === '/mnt/nas') return;
      void loadFolders(path, binding);
    }
    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      const binding = state.binding;
      if (!target || !binding || !root(binding)?.contains(target)) return;
      const open = target.closest('[data-backups-open]');
      if (open) { openModal(open.dataset.backupsOpen, open, binding); return; }
      if (target.closest('[data-backups-close]')) {
        const modal = target.closest('[data-backups-modal]'); if (modal) closeModal(modal.dataset.backupsModal, binding); return;
      }
      if (target.closest('[data-backups-sync]')) { void syncNow(binding); return; }
      if (target.closest('[data-settings-toggle]')) {
        const name = target.closest('[data-settings-toggle]').dataset.settingsToggle;
        state.menuBucket = state.menuBucket === name ? null : name; renderOffsiteCards(binding); return;
      }
      if (target.closest('[data-open-skip]')) { openModal('skip', target.closest('[data-open-skip]'), binding); return; }
      if (target.closest('[data-open-folders]')) { openModal('folders', target.closest('[data-open-folders]'), binding); return; }
      if (target.closest('[data-skip-add]')) { void saveSkipNames(binding); return; }
      if (target.closest('[data-skip-remove]')) {
        const index = Number(target.closest('[data-skip-remove]').dataset.skipRemove);
        if (Number.isInteger(index) && index >= 0 && index < state.skipDraft.length) { state.skipChecks.delete(state.skipDraft[index]); state.skipDraft.splice(index, 1); renderSkipChips(binding); }
        return;
      }
      if (target.closest('[data-skip-save]')) { void commitSkipNames(binding); return; }
      if (target.closest('[data-folder-enter]')) { openFolderChild(target.closest('[data-folder-enter]').dataset.folderEnter, binding); return; }
      if (target.closest('[data-folder-up]')) { void loadFolders(nearestFolderPath(state.folderPath), binding); return; }
      if (target.closest('[data-folder-save]')) { void commitFolders(binding); return; }
      if (target.closest('[data-ledger-more]')) { void loadLedgerPage(false, binding); return; }
      if (target.closest('[data-snapshots-ask]')) { openModal('confirm', target.closest('[data-snapshots-ask]'), binding); return; }
      if (target.closest('[data-snapshots-cancel]')) { closeModal('confirm', binding); return; }
      if (target.closest('[data-snapshots-yes]')) { void listSnapshotsAfterYes(binding); return; }
      if (target.closest('[data-add-submit]')) { void addOrUpdateCredentials(binding); return; }
      if (target.closest('[data-schedule-save]')) { void saveSchedule(binding); return; }
      const action = target.closest('[data-bucket-action]');
      if (action) {
        const name = action.dataset.bucket, kind = action.dataset.bucketAction;
        state.menuBucket = null; renderOffsiteCards(binding);
        if (kind === 'run') void runBucket(name, binding);
        if (kind === 'verify') void verifyBucket(name, binding);
        if (kind === 'credentials') openAddUpdate(name, binding);
        if (kind === 'remove') void removeBucket(name, binding);
        return;
      }
    });
    document.body.addEventListener('change', event => {
      const target = event.target instanceof Element ? event.target : null;
      const binding = state.binding;
      if (!target || !binding || !root(binding)?.contains(target) || !current(binding)) return;
      if (target.matches('[data-git-toggle]')) { void toggleGitServer(target.dataset.gitToggle, target.checked, binding); return; }
      if (target.matches('[data-folder-select]')) {
        const path = target.dataset.folderSelect;
        if (target.checked) state.folderDraft.add(path); else state.folderDraft.delete(path);
        updateFolderSummary(binding); return;
      }
      if (target.matches('[data-ledger-filter]')) { state.ledgerFilter = target.value; renderLedgerRows(binding); return; }
      if (target.matches('[data-snapshot-bucket]')) { const result = el('[data-snapshots-result]', binding); if (result) result.replaceChildren(); return; }
      if (target.matches('[data-schedule-frequency]') || target.matches('[data-schedule-time]') || target.matches('[data-schedule-day]')) {
        const frequency = el('[data-schedule-frequency]', binding)?.value || 'daily';
        const day = el('[data-schedule-day-wrap]', binding); if (day) day.hidden = frequency !== 'weekly';
        renderScheduleOrder(binding); return;
      }
    });
    document.body.addEventListener('keydown', event => {
      const target = event.target instanceof Element ? event.target : null;
      const binding = state.binding;
      if (!target || !binding || !root(binding)?.contains(target)) return;
      if (event.key === 'Escape' && state.activeModal) { event.preventDefault(); closeModal(state.activeModal, binding); return; }
      if (event.key === 'Enter' && target.matches('[data-skip-input]')) { event.preventDefault(); void saveSkipNames(binding); }
    });
    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      if (!target || !state.binding || !root()?.contains(target)) return;
      const overlay = target.matches('[data-backups-modal]') ? target : null;
      if (overlay && !overlay.hidden && overlay.dataset.backupsModal !== 'confirm') closeModal(overlay.dataset.backupsModal);
    });
    function scrubBackblazeProtectedState(pane) { scrubProtectedState(pane); }
    observeBackblazeActivity();
"####
}
