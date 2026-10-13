fn shell_backblaze_client() -> &'static str {
    r####"
    const state = { binding: null, data: null, bucket: null, ledger: [], ledgerBucket: null, statusReady: false, statusStale: false, snapshotsReady: false, snapshotsStale: false, selectionGeneration: 0, managerDrafts: {} };
    let statusPollTimer = null;
    let managerDraftBucket = null;
    let managerDraftBeforeOpen = null;
    const esc = v => String(v ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
    const admitted = () => typeof viewportFamilyAdmitted === 'function' && viewportFamilyAdmitted('backblaze');
    const roleAuthorized = () => Boolean(headerState?.isAdmin && coronatioAttendanceRuntime?.currentAttendance);
    const root = (binding = state.binding) => binding?.root instanceof Element ? binding.root : null;
    const el = (selector, binding = state.binding) => root(binding)?.querySelector(selector) || null;
    const current = (binding = state.binding) => Boolean(binding && state.binding === binding && admitted() && scaffoldRegistry.current(binding));
    const canUse = (binding = state.binding) => Boolean(current(binding) && state.statusReady && !state.statusStale && state.data);
    const health = b => b?.status?.running ? 'RUNNING' : (b?.status?.last_run_ok === true ? 'GOOD' : (b?.status?.last_run_ok === false ? 'FAILED' : 'IDLE'));
    const forgejo = b => b?.managers?.forgejo || {};
    const actions = new Set();
    const secretDialogs = new Set();
    function clearStatusPoll() { if (statusPollTimer !== null) window.clearTimeout(statusPollTimer); statusPollTimer = null; }
    function cancelSecretDialogs() { secretDialogs.forEach(dialog => dialog.cancel()); secretDialogs.clear(); }
    function renderReadStatus(selector, message, stateName, retry = false) {
      const host = el(selector); if (!host) return;
      let node = host.querySelector(':scope > [data-backblaze-read-state]');
      if (!message) { node?.remove(); return; }
      if (!node) { node = document.createElement('p'); node.dataset.backblazeReadState = ''; node.setAttribute('role', 'status'); host.prepend(node); }
      node.dataset.state = stateName || 'stale'; node.textContent = message;
      node.querySelectorAll('[data-backblaze-retry]').forEach(button => button.remove());
      if (retry) { const button = document.createElement('button'); button.type = 'button'; button.dataset.backblazeRetry = selector === '[data-backblaze-ledger]' ? 'backblaze-snapshots' : 'backblaze-status'; button.textContent = 'Retry'; node.append(document.createTextNode(' '), button); }
    }
    function projectStatus(data) {
      if (!data || typeof data !== 'object' || Array.isArray(data) || data.ok === false || data.success === false || typeof data.running !== 'boolean' || !Array.isArray(data.buckets)) throw new Error('Backblaze status response is incomplete');
      const buckets = data.buckets.map(bucket => {
        if (!bucket || typeof bucket !== 'object' || Array.isArray(bucket) || typeof bucket.bucket !== 'string' || !bucket.bucket) throw new Error('Backblaze bucket row is malformed');
        const items = Array.isArray(bucket.items) ? bucket.items.map(item => {
          if (!item || typeof item !== 'object' || Array.isArray(item) || typeof item.path !== 'string') throw new Error('Backblaze item row is malformed');
          return { ...item, path: item.path };
        }) : [];
        return { bucket: bucket.bucket, prefix: typeof bucket.prefix === 'string' ? bucket.prefix : '', encrypted: bucket.encrypted === true,
          items, connected: typeof bucket.connected === 'boolean' ? bucket.connected : null,
          status: { running: bucket.status?.running === true, last_run_ok: typeof bucket.status?.last_run_ok === 'boolean' ? bucket.status.last_run_ok : null },
          managers: { ...(bucket.managers && typeof bucket.managers === 'object' && !Array.isArray(bucket.managers) ? bucket.managers : {}),
            forgejo: { ...(bucket.managers?.forgejo && typeof bucket.managers.forgejo === 'object' && !Array.isArray(bucket.managers.forgejo) ? bucket.managers.forgejo : {}),
              localNas: bucket.managers?.forgejo?.localNas === true, backblazeBackup: bucket.managers?.forgejo?.backblazeBackup === true } } };
      });
      return { running: data.running, last_run_at: data.last_run_at ?? null, last_run_ok: typeof data.last_run_ok === 'boolean' ? data.last_run_ok : null, buckets };
    }
    function renderBucketList(selector) {
      const host = el(selector); if (!host) return;
      if (!state.statusReady) return;
      const buckets = Array.isArray(state.data?.buckets) ? state.data.buckets : [];
      const html = buckets.map(bucket => `<div class="backblaze-bucket-row" data-backblaze-bucket-row="${esc(bucket.bucket)}" data-scaffold-key="backblaze-bucket-${esc(bucket.bucket)}"><button type="button" class="backblaze-bucket-select" data-backblaze-select="${esc(bucket.bucket)}" aria-pressed="${state.bucket === bucket.bucket}" ${canUse() ? '' : 'disabled'}><span>${esc(bucket.bucket)}</span><span class="ui-badge">${health(bucket)}</span></button><button type="button" class="backblaze-bucket-remove" data-backblaze-remove="${esc(bucket.bucket)}" title="Remove ${esc(bucket.bucket)}" ${canUse() ? '' : 'disabled'}>×</button></div>`).join('') || '<p class="backblaze-empty" data-backblaze-empty-buckets>No buckets configured.</p>';
      morphLivePane(host, html);
    }
    function renderBuckets() { renderBucketList('[data-backblaze-buckets]'); renderBucketList('[data-backblaze-ledger-buckets]'); }
    function renderManager(bucket) {
      const indicator = el('[data-backblaze-manager-indicator]'), open = el('[data-backblaze-manager-open]');
      const local = el('[data-backblaze-local-nas]'), backup = el('[data-backblaze-forgejo-backup]');
      if (!bucket) { if (indicator) indicator.classList.remove('is-lit'); if (open) open.disabled = true; [local, backup].forEach(control => { if (control) control.disabled = true; }); return; }
      const model = state.managerDrafts[bucket.bucket] || forgejo(bucket), on = model.localNas === true || model.backblazeBackup === true;
      if (indicator) indicator.classList.toggle('is-lit', on);
      if (open) open.disabled = !canUse();
      [local, backup].forEach(control => { if (control) control.disabled = !canUse(); });
    }
    function renderSelected() {
      const binding = state.binding, bucket = state.data?.buckets?.find(candidate => candidate.bucket === state.bucket);
      const addBucket = el('[data-backblaze-add]'); if (addBucket) addBucket.disabled = !canUse(binding);
      const title = el('[data-backblaze-selected]'), save = el('[data-backblaze-save]'), verify = el('[data-backblaze-verify]'), update = el('[data-backblaze-update]'), run = el('[data-backblaze-run]'), encrypted = el('[data-backblaze-encrypted]'), path = el('[data-backblaze-path]'), add = el('[data-backblaze-item-add]');
      if (!state.statusReady || !bucket) {
        if (title) title.textContent = state.statusReady ? 'Select a bucket' : 'Loading buckets…';
        const status = el('[data-backblaze-bucket-status]'); if (status) status.textContent = state.statusReady ? 'Select a bucket.' : 'Waiting for current status…';
        [save, verify, update, run, encrypted, path, add].forEach(control => { if (control) control.disabled = true; });
        const host = el('[data-backblaze-items]'); if (host && state.statusReady) morphLivePane(host, '<p class="backblaze-empty">Select a bucket to manage paths.</p>');
        const name = el('[data-backblaze-bucket-name]'); if (name) name.value = '';
        renderManager(null); return;
      }
      const enabled = canUse();
      if (title) title.textContent = bucket.bucket;
      const status = el('[data-backblaze-bucket-status]'); if (status) status.textContent = health(bucket);
      const name = el('[data-backblaze-bucket-name]'); if (name) name.value = bucket.bucket;
      const connected = el('[data-backblaze-connected]'); if (connected) { connected.textContent = bucket.connected === true ? 'Connected' : bucket.connected === false ? 'Not connected' : 'Unknown'; connected.classList.toggle('is-connected', bucket.connected === true); connected.classList.toggle('is-disconnected', bucket.connected === false); }
      if (encrypted) { encrypted.checked = bucket.encrypted === true; encrypted.disabled = !enabled; }
      if (path) path.disabled = !enabled; if (add) add.disabled = !enabled;
      if (save) save.disabled = !enabled; if (verify) verify.disabled = !enabled; if (update) update.disabled = !enabled; if (run) run.disabled = !enabled;
      const host = el('[data-backblaze-items]'); if (host) {
        const items = Array.isArray(bucket.items) ? bucket.items : [];
        morphLivePane(host, items.map(item => `<div class="backblaze-path-row" data-backblaze-path-row="${esc(item.path)}" data-scaffold-key="backblaze-path-${esc(item.path)}"><span>${esc(item.path)}</span><button type="button" class="backblaze-path-remove" data-backblaze-path-remove="${esc(item.path)}" ${enabled ? '' : 'disabled'}>×</button></div>`).join('') || '<p class="backblaze-empty" data-backblaze-empty-items>No files configured.</p>');
      }
      renderManager(bucket);
    }
    function renderLedger(binding = state.binding) {
      if (!current(binding)) return;
      const host = el('[data-backblaze-ledger]', binding); if (!host) return;
      let content = host.querySelector(':scope > [data-backblaze-ledger-content]');
      if (!content) { content = document.createElement('div'); content.dataset.backblazeLedgerContent = ''; host.append(content); }
      let html;
      if (!state.bucket) html = '<p class="backblaze-empty">Select a bucket</p>';
      else if (state.ledgerBucket !== state.bucket) html = '<p class="backblaze-empty">Loading snapshots…</p>';
      else if (!state.ledger.length && !state.snapshotsReady) html = '<p class="backblaze-empty">Snapshots unavailable.</p>';
      else html = state.ledger.map((row, index) => `<div class="backblaze-ledger-row" data-backblaze-snapshot="${esc(row.id || `${state.bucket}-${index}`)}"><time>${esc(row.time || row.status || 'Snapshot')}</time><span>${esc(row.id || '—')}</span></div>`).join('') || '<p class="backblaze-empty" data-backblaze-empty-snapshots>No restic snapshots available.</p>';
      morphLivePane(content, html);
    }
    function setView(view) {
      const binding = state.binding; if (!current(binding)) return;
      root(binding).querySelectorAll('[data-backblaze-view]').forEach(button => { const on = button.dataset.backblazeView === view; button.classList.toggle('is-active', on); button.setAttribute('aria-selected', String(on)); });
      root(binding).querySelectorAll('[data-backblaze-panel]').forEach(panel => { panel.hidden = panel.dataset.backblazePanel !== view; });
      if (view === 'ledger') { if (state.statusReady) void loadSnapshots(binding); else renderReadStatus('[data-backblaze-ledger]', 'Waiting for status before reading snapshots.', 'pending'); }
    }
    function adoptBackblazeBinding() {
      const binding = admitted() ? scaffoldRegistry.capture('backblaze') : null;
      if (!binding) { clearStatusPoll(); if (!roleAuthorized()) { if (state.binding) retireBackblaze(state.binding, true); else scrubBackblazeProtectedState(document.querySelector('[data-pane-panel="backblaze"]')); } return null; }
      if (state.binding !== binding) {
        const previous = state.binding;
        if (previous) scaffoldRegistry.retire(previous, 'Backblaze binding replaced');
        state.binding = binding; state.statusStale = true; state.snapshotsStale = state.snapshotsReady || state.snapshotsStale;
        clearStatusPoll();
        scaffoldRegistry.onRetire(binding, () => retireBackblaze(binding));
        renderBuckets(); renderSelected();
        if (state.statusReady) renderReadStatus('[data-backblaze-ribbon]', 'Showing last known status while the current view refreshes.', 'stale');
      }
      return binding;
    }
    function scrubBackblazeProtectedState(pane) {
      state.data = null; state.bucket = null; state.ledger = []; state.ledgerBucket = null;
      state.statusReady = false; state.statusStale = true; state.snapshotsReady = false; state.snapshotsStale = true; state.managerDrafts = {};
      if (!(pane instanceof Element) || !pane.isConnected) return;
      pane.querySelectorAll('[data-backblaze-buckets], [data-backblaze-ledger-buckets], [data-backblaze-items], [data-backblaze-ledger]').forEach(node => node.replaceChildren());
      pane.querySelectorAll('[data-backblaze-state], [data-backblaze-last], [data-backblaze-duration], [data-backblaze-next], [data-backblaze-selected], [data-backblaze-bucket-status], [data-backblaze-connected], [data-backblaze-result]').forEach(node => { node.textContent = ''; });
      pane.querySelectorAll('[data-backblaze-bucket-name], [data-backblaze-path]').forEach(node => { node.value = ''; });
      pane.querySelectorAll('[data-backblaze-encrypted], [data-backblaze-local-nas], [data-backblaze-forgejo-backup]').forEach(node => { node.checked = false; });
      pane.querySelectorAll('[data-backblaze-manager-modal]').forEach(node => { node.hidden = true; });
      pane.querySelectorAll('[data-backblaze-add], [data-backblaze-select], [data-backblaze-remove], [data-backblaze-save], [data-backblaze-verify], [data-backblaze-update], [data-backblaze-run], [data-backblaze-manager-open], [data-backblaze-item-add], [data-backblaze-path-remove], [data-backblaze-encrypted], [data-backblaze-local-nas], [data-backblaze-forgejo-backup]').forEach(node => { node.disabled = true; });
    }
    function retireBackblaze(binding, privilegedLoss = false) {
      if (state.binding !== binding) return;
      const roleLost = privilegedLoss || !roleAuthorized();
      clearStatusPoll(); cancelSecretDialogs(); actions.forEach(controller => controller.abort()); actions.clear();
      state.binding = null; state.statusStale = true; state.snapshotsStale = true;
      const pane = binding.root;
      if (roleLost) { scrubBackblazeProtectedState(pane); return; }
      if (!(pane instanceof Element) || !pane.isConnected) return;
      pane.querySelectorAll('[data-backblaze-add], [data-backblaze-select], [data-backblaze-remove], [data-backblaze-save], [data-backblaze-verify], [data-backblaze-update], [data-backblaze-run], [data-backblaze-manager-open], [data-backblaze-item-add], [data-backblaze-path-remove], [data-backblaze-encrypted], [data-backblaze-local-nas], [data-backblaze-forgejo-backup]').forEach(node => { node.disabled = true; });
      const status = pane.querySelector('[data-backblaze-state]'); if (status) { status.textContent = 'STALE · showing last known status'; status.dataset.state = 'stale'; }
      const ribbon = pane.querySelector('[data-backblaze-ribbon]');
      if (ribbon) { let node = ribbon.querySelector(':scope > [data-backblaze-read-state]'); if (!node) { node = document.createElement('p'); node.dataset.backblazeReadState = ''; node.setAttribute('role', 'status'); ribbon.prepend(node); } node.dataset.state = 'stale'; node.textContent = 'This view is stale; reconnecting will refresh it.'; }
    }
    window.addEventListener('coronatio:privileged-state-retired', () => {
      if (state.binding) { retireBackblaze(state.binding, true); return; }
      scrubBackblazeProtectedState(document.querySelector('[data-pane-panel="backblaze"]'));
    });
    function scheduleStatusPoll(binding) {
      clearStatusPoll();
      if (!current(binding) || state.data?.running !== true) return;
      statusPollTimer = window.setTimeout(() => { statusPollTimer = null; if (current(binding)) void hydrateBackblaze(binding); else clearStatusPoll(); }, 5000);
    }
    function normalizeSnapshot(row) {
      if (!row || typeof row !== 'object' || Array.isArray(row)) throw new Error('Backblaze snapshot row is malformed');
      return { time: typeof row.time === 'string' || typeof row.time === 'number' ? row.time : '', status: typeof row.status === 'string' ? row.status : '', id: typeof row.id === 'string' || typeof row.id === 'number' ? row.id : '' };
    }
    async function loadSnapshots(binding = state.binding, options = {}) {
      if (!canUse(binding)) return false;
      const bucket = state.bucket, generation = state.selectionGeneration;
      if (!bucket) { state.ledger = []; state.ledgerBucket = null; state.snapshotsReady = true; renderLedger(binding); return true; }
      if (state.ledgerBucket !== bucket) { state.ledger = []; state.ledgerBucket = bucket; state.snapshotsReady = false; state.snapshotsStale = false; renderLedger(binding); }
      renderReadStatus('[data-backblaze-ledger]', state.snapshotsReady ? '' : 'Loading snapshots…', state.snapshotsStale ? 'stale' : 'pending');
      return scaffoldRegistry.run(binding, 'backblaze-snapshots', async context => {
        const body = await context.fetchJson('/api/backblaze/snapshots?bucket=' + encodeURIComponent(bucket));
        if (!context.current() || !current(binding)) return false;
        if (!body || typeof body !== 'object' || Array.isArray(body) || body.ok === false || !Array.isArray(body.snapshots)) throw new Error('Backblaze snapshots response is incomplete');
        const ledger = body.snapshots.map(normalizeSnapshot);
        if (!context.current() || !current(binding)) return false;
        if (state.bucket !== bucket || state.selectionGeneration !== generation) return true;
        state.ledger = ledger; state.ledgerBucket = bucket; state.snapshotsReady = true; state.snapshotsStale = false;
        renderLedger(binding); renderReadStatus('[data-backblaze-ledger]', '', 'ready');
        return true;
      }, {
        slots: [], supersede: options.supersede === true, timeoutMs: 8000, slowMs: 750,
        onFailure: ({ current: runCurrent, reason }) => {
          if (!runCurrent() || !current(binding) || state.bucket !== bucket || state.selectionGeneration !== generation) return;
          state.snapshotsStale = true;
          renderReadStatus('[data-backblaze-ledger]', `${state.snapshotsReady ? 'Showing last known snapshots; refresh' : 'Snapshots'} unavailable: ${reason || 'Read failed'}.`, 'stale', true);
          renderLedger(binding);
        }
      });
    }
    async function hydrateBackblaze(binding = adoptBackblazeBinding()) {
      if (!binding || !current(binding)) return false;
      return scaffoldRegistry.run(binding, 'backblaze-status', async context => {
        if (!context.current() || !current(binding)) return false;
        state.statusStale = true;
        renderReadStatus('[data-backblaze-ribbon]', 'Refreshing status; controls are disabled.', 'pending');
        renderBuckets(); renderSelected();
        const body = await context.fetchJson('/api/backblaze/status');
        if (!context.current() || !current(binding)) return false;
        const data = projectStatus(body);
        const currentBucket = state.bucket;
        const nextBucket = data.buckets.some(bucket => bucket.bucket === currentBucket) ? currentBucket : (data.buckets[0]?.bucket || null);
        if (nextBucket !== currentBucket) { state.selectionGeneration += 1; state.bucket = nextBucket; state.ledger = []; state.ledgerBucket = null; state.snapshotsReady = false; state.snapshotsStale = false; }
        state.data = data; state.statusReady = true; state.statusStale = false;
        const status = el('[data-backblaze-state]', binding);
        if (status) { status.textContent = health({ status: { running: data.running, last_run_ok: data.last_run_ok } }); status.dataset.state = 'ready'; }
        const last = el('[data-backblaze-last]', binding); if (last) last.textContent = data.last_run_at ? new Date(Number(data.last_run_at) * 1000).toLocaleString() : 'Never';
        const duration = el('[data-backblaze-duration]', binding); if (duration) duration.textContent = 'duration unavailable';
        const next = el('[data-backblaze-next]', binding); if (next) next.textContent = 'Not scheduled';
        renderReadStatus('[data-backblaze-ribbon]', '', 'ready'); renderBuckets(); renderSelected();
        if (!el('[data-backblaze-panel="ledger"]', binding)?.hidden) void loadSnapshots(binding);
        scheduleStatusPoll(binding);
        return true;
      }, {
        slots: [], timeoutMs: 8000, slowMs: 750,
        onFailure: ({ current: runCurrent, reason }) => {
          if (!runCurrent() || !current(binding)) return;
          state.statusStale = true;
          const status = el('[data-backblaze-state]', binding);
          if (status) { status.dataset.state = 'stale'; status.textContent = state.statusReady ? `STALE · ${health({ status: { running: state.data?.running, last_run_ok: state.data?.last_run_ok } })}` : 'UNAVAILABLE'; }
          renderReadStatus('[data-backblaze-ribbon]', `Status refresh unavailable: ${reason || 'Read failed'}. ${state.statusReady ? 'Showing last known status.' : ''}`, 'stale', true);
          renderBuckets(); renderSelected(); scheduleStatusPoll(binding);
        }
      });
    }
    function currentAction(binding, bucket = state.bucket) { return canUse(binding) && state.bucket === bucket; }
    function actionController(binding) { const controller = new AbortController(); actions.add(controller); scaffoldRegistry.onRetire(binding, () => controller.abort()); return controller; }
    async function getJson(url, init, binding = state.binding) { const response = await fetch(url, { ...init, cache: 'no-store' }); const body = await response.json().catch(() => ({ ok: false, reason: 'Invalid response' })); if (!response.ok) return body; return body; }
    function setResult(binding, text) { if (current(binding)) { const result = el('[data-backblaze-result]', binding); if (result) result.textContent = text; } }
    function setViewEvent(view) { setView(view); }
    function secretInput(label, binding) {
      return new Promise(resolve => {
        if (!current(binding)) { resolve(''); return; }
        const wrap = document.createElement('div'); wrap.className = 'backblaze-secret-dialog'; wrap.innerHTML = `<div class="backblaze-secret-card"><label>${esc(label)}<input type="password" autocomplete="new-password"></label><div><button type="button" data-cancel>Cancel</button><button type="button" data-submit>Continue</button></div></div>`;
        document.body.append(wrap); const input = wrap.querySelector('input'); input.focus();
        let settled = false; const finish = value => { if (settled) return; settled = true; input.value = ''; wrap.remove(); secretDialogs.delete(dialog); resolve(value); };
        const dialog = { cancel: () => finish('') }; secretDialogs.add(dialog);
        wrap.querySelector('[data-cancel]').onclick = () => finish('');
        wrap.querySelector('[data-submit]').onclick = () => finish(input.value);
      });
    }
    async function addBucket(binding = state.binding) {
      if (!canUse(binding)) return;
      const bucket = prompt('B2 bucket name'); if (!bucket || !currentAction(binding)) return;
      const keyId = prompt('B2 key ID'); if (!keyId || !currentAction(binding)) return;
      let applicationKey = await secretInput('B2 application key', binding); if (!applicationKey || !currentAction(binding)) { applicationKey = ''; return; }
      const controller = actionController(binding);
      try {
        const body = await getJson('/api/backblaze/buckets', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ bucket, prefix: '', encrypted: true, items: [], managers: { forgejo: { localNas: false, backblazeBackup: false } }, keyId, applicationKey }), signal: controller.signal }, binding);
        applicationKey = ''; if (!current(binding)) return;
        setResult(binding, body.ok === false ? (body.reason || 'Bucket could not be added') : 'Bucket added and verified'); await hydrateBackblaze(binding);
      } catch (_) { applicationKey = ''; if (current(binding)) setResult(binding, 'Bucket could not be added'); }
      finally { actions.delete(controller); }
    }
    async function updateCredentials(binding = state.binding) {
      if (!canUse(binding)) return; const bucket = state.data?.buckets?.find(row => row.bucket === state.bucket); if (!bucket) return;
      const selectedBucket = bucket.bucket, keyId = prompt('B2 key ID'); if (!keyId || !currentAction(binding, selectedBucket)) return;
      let applicationKey = await secretInput('B2 application key', binding); if (!applicationKey || !currentAction(binding, selectedBucket)) { applicationKey = ''; return; }
      const controller = actionController(binding);
      try {
        const body = await getJson('/api/backblaze/buckets', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ bucket: bucket.bucket, prefix: bucket.prefix || '', encrypted: bucket.encrypted === true, items: bucket.items || [], managers: bucket.managers || { forgejo: { localNas: false, backblazeBackup: false } }, keyId, applicationKey }), signal: controller.signal }, binding);
        applicationKey = ''; if (!currentAction(binding, selectedBucket)) return;
        setResult(binding, body.ok === false ? (body.reason || 'Credentials could not be updated') : 'Credentials updated and verified'); await hydrateBackblaze(binding);
      } catch (_) { applicationKey = ''; if (currentAction(binding, selectedBucket)) setResult(binding, 'Credentials could not be updated'); }
      finally { actions.delete(controller); }
    }
    async function verifyBucket(bucket = state.bucket, binding = state.binding) {
      if (!canUse(binding) || !bucket) return;
      const controller = actionController(binding);
      try {
        const body = await getJson('/api/backblaze/buckets/' + encodeURIComponent(bucket) + '/verify', { method: 'POST', signal: controller.signal }, binding);
        if (!currentAction(binding, bucket)) return;
        const selected = state.data?.buckets?.find(row => row.bucket === bucket); if (selected && typeof body.connected === 'boolean') selected.connected = body.connected;
        renderBuckets(); renderSelected(); setResult(binding, body.ok === false ? (body.reason || 'Connection verification failed') : (body.connected === true ? 'Connected' : 'Not connected'));
      } catch (_) { if (currentAction(binding, bucket)) setResult(binding, 'Connection verification failed'); }
      finally { actions.delete(controller); }
    }
    async function removeBucket(bucket, binding = state.binding) {
      if (!currentAction(binding, bucket) || !confirm(`Remove ${bucket}?`)) return;
      const controller = actionController(binding);
      try {
        await getJson('/api/backblaze/buckets/' + encodeURIComponent(bucket), { method: 'DELETE', signal: controller.signal }, binding);
        if (!current(binding)) return;
        if (state.bucket === bucket) { state.bucket = null; state.selectionGeneration += 1; }
        await hydrateBackblaze(binding);
      } catch (_) { if (current(binding)) setResult(binding, 'Bucket could not be removed'); }
      finally { actions.delete(controller); }
    }
    async function mutateItem(path, method, binding = state.binding) {
      const bucket = state.bucket; if (!currentAction(binding, bucket)) return;
      const controller = actionController(binding);
      try { await getJson('/api/backblaze/buckets/' + encodeURIComponent(bucket) + '/items', { method, headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ path }), signal: controller.signal }, binding); if (currentAction(binding, bucket)) await hydrateBackblaze(binding); }
      catch (_) { if (currentAction(binding, bucket)) setResult(binding, 'File path could not be updated'); }
      finally { actions.delete(controller); }
    }
    async function saveConfig(binding = state.binding) {
      const bucket = state.data?.buckets?.find(row => row.bucket === state.bucket); if (!bucket || !canUse(binding)) return;
      const selectedBucket = bucket.bucket, controller = actionController(binding);
      const body = await getJson('/api/backblaze/config', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ buckets: state.data.buckets.map(row => ({ bucket: row.bucket, prefix: row.prefix || '', encrypted: row.encrypted === true, items: Array.isArray(row.items) ? row.items : [], managers: state.managerDrafts[row.bucket] ? { ...(row.managers || {}), forgejo: state.managerDrafts[row.bucket] } : (row.managers || { forgejo: { localNas: false, backblazeBackup: false } }) })) }), signal: controller.signal }, binding).catch(() => ({ ok: false }));
      try {
        if (!currentAction(binding, selectedBucket)) return;
        setResult(binding, body.ok ? 'Config saved without credentials' : (body.reason || 'Config could not be saved'));
        if (body.ok && await hydrateBackblaze(binding) && current(binding)) { state.managerDrafts = {}; renderSelected(); }
      } finally { actions.delete(controller); }
    }
    function openManager(binding = state.binding) {
      const bucket = state.data?.buckets?.find(row => row.bucket === state.bucket); if (!bucket || !canUse(binding)) return;
      managerDraftBucket = bucket.bucket; managerDraftBeforeOpen = Object.prototype.hasOwnProperty.call(state.managerDrafts, bucket.bucket) ? { ...state.managerDrafts[bucket.bucket] } : null;
      const model = state.managerDrafts[bucket.bucket] || forgejo(bucket); state.managerDrafts[bucket.bucket] = { localNas: model.localNas === true, backblazeBackup: model.backblazeBackup === true };
      const local = el('[data-backblaze-local-nas]', binding), backup = el('[data-backblaze-forgejo-backup]', binding); if (local) local.checked = state.managerDrafts[bucket.bucket].localNas; if (backup) backup.checked = state.managerDrafts[bucket.bucket].backblazeBackup;
      renderManager(bucket); const modal = el('[data-backblaze-manager-modal]', binding); if (modal) modal.hidden = false;
    }
    function closeManager(save, binding = state.binding) {
      const bucket = state.data?.buckets?.find(row => row.bucket === managerDraftBucket);
      if (!save && managerDraftBucket !== null) { if (managerDraftBeforeOpen) state.managerDrafts[managerDraftBucket] = { ...managerDraftBeforeOpen }; else delete state.managerDrafts[managerDraftBucket]; }
      if (bucket && current(binding)) renderManager(bucket);
      const modal = el('[data-backblaze-manager-modal]', binding); if (modal) modal.hidden = true;
      managerDraftBucket = null; managerDraftBeforeOpen = null;
    }
    function selectBucket(bucket, binding = state.binding) {
      if (!canUse(binding) || !state.data?.buckets?.some(row => row.bucket === bucket)) return;
      if (state.bucket !== bucket) { state.bucket = bucket; state.selectionGeneration += 1; state.ledger = []; state.ledgerBucket = null; state.snapshotsReady = false; state.snapshotsStale = false; }
      renderBuckets(); renderSelected(); renderLedger(binding);
      if (!el('[data-backblaze-panel="ledger"]', binding)?.hidden) void loadSnapshots(binding, { supersede: true });
    }
    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null; if (!target) return;
      const binding = state.binding; if (!binding || !root(binding)?.contains(target)) return;
      const view = target.closest('[data-backblaze-view]'); if (view) { setViewEvent(view.dataset.backblazeView); return; }
      const retry = target.closest('[data-backblaze-retry]'); if (retry && current(binding)) { event.preventDefault(); scaffoldRegistry.retry(binding, retry.dataset.backblazeRetry).catch(() => {}); return; }
      const select = target.closest('[data-backblaze-select]'); if (select) { selectBucket(select.dataset.backblazeSelect, binding); return; }
      const remove = target.closest('[data-backblaze-remove]'); if (remove) { void removeBucket(remove.dataset.backblazeRemove, binding); return; }
      const pathRemove = target.closest('[data-backblaze-path-remove]'); if (pathRemove) { void mutateItem(pathRemove.dataset.backblazePathRemove, 'DELETE', binding); return; }
      if (target.closest('[data-backblaze-add]')) { void addBucket(binding); return; }
      if (target.closest('[data-backblaze-save]')) { void saveConfig(binding); return; }
      if (target.closest('[data-backblaze-verify]')) { void verifyBucket(state.bucket, binding); return; }
      if (target.closest('[data-backblaze-update]')) { void updateCredentials(binding); return; }
      if (target.closest('[data-backblaze-manager-open]')) { openManager(binding); return; }
      if (target.closest('[data-backblaze-manager-cancel]')) { closeManager(false, binding); return; }
      if (target.closest('[data-backblaze-manager-close]')) { closeManager(true, binding); return; }
      if (target.closest('[data-backblaze-item-add]')) { const path = el('[data-backblaze-path]', binding)?.value.trim(); if (path) void mutateItem(path, 'POST', binding); return; }
      if (target.closest('[data-backblaze-run]') && currentAction(binding)) {
        const bucket = state.bucket, controller = actionController(binding);
        getJson('/api/backblaze/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ bucket }), signal: controller.signal }, binding).then(() => { if (currentAction(binding, bucket)) return hydrateBackblaze(binding); }).catch(() => { if (currentAction(binding, bucket)) setResult(binding, 'Backup could not be started'); }).finally(() => actions.delete(controller));
      }
    });
    document.body.addEventListener('change', event => {
      const target = event.target instanceof Element ? event.target : null; const binding = state.binding;
      if (!target || !binding || !root(binding)?.contains(target) || !canUse(binding)) return;
      const bucket = state.data?.buckets?.find(row => row.bucket === state.bucket); if (!bucket) return;
      if (target.matches('[data-backblaze-encrypted]')) {
        const selectedBucket = bucket.bucket, controller = actionController(binding);
        getJson('/api/backblaze/buckets/' + encodeURIComponent(bucket.bucket) + '/toggle', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ encrypted: target.checked }), signal: controller.signal }, binding).then(() => { if (currentAction(binding, selectedBucket)) return hydrateBackblaze(binding); }).catch(() => { if (currentAction(binding, selectedBucket)) setResult(binding, 'Encryption setting could not be updated'); }).finally(() => actions.delete(controller));
      }
      if (target.matches('[data-backblaze-local-nas]') || target.matches('[data-backblaze-forgejo-backup]')) {
        const currentDraft = state.managerDrafts[bucket.bucket] || forgejo(bucket);
        state.managerDrafts[bucket.bucket] = { ...currentDraft, localNas: el('[data-backblaze-local-nas]', binding)?.checked === true, backblazeBackup: el('[data-backblaze-forgejo-backup]', binding)?.checked === true };
        renderManager(bucket);
      }
    });
    function hydrateBackblazeEntry() { return hydrateBackblaze(adoptBackblazeBinding()); }
    if (typeof viewportFamilyAdmitted === 'function') void hydrateBackblazeEntry();
"####
}