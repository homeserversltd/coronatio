    function systemUpdateManager() {
      if (!headerState.isAdmin) return;
      const names = ['Overview', 'Modules', 'Schedule', 'Interactables', 'History'];
      const body = `<nav aria-label="Update manager views">${names.map(name => `<button type="button" class="ui-button ui-button--secondary ui-button--small" data-update-view="${name}">${name}</button>`).join(' ')}</nav><div data-update-content aria-live="polite"></div>`;
      const modal = managerModal('updates', 'System Update Manager', body);
      modal.querySelector('[data-manager-confirm]').remove();
      const close = modal.querySelector('.modal-actions [data-manager-close]');
      if (close) close.textContent = 'Close';
      const content = modal.querySelector('[data-update-content]');
      const esc = value => escapeDiskHtml(String(value ?? ''));
      const localTime = value => { const date = new Date(value); return value && !Number.isNaN(date.valueOf()) ? date.toLocaleString() : ''; };
      const read = async path => {
        try {
          const response = await fetch('/api/caduceus/' + path, {cache:'no-store'});
          const result = await response.json();
          return response.ok && result?.ok && result?.readback?.ok ? result.readback.body : null;
        } catch (_) { return null; }
      };
      const list = (value, key) => Array.isArray(value) ? value : Array.isArray(value?.[key]) ? value[key] : [];
      const recent = history => list(history, 'runs').slice().sort((a,b) => String(b.finished_at || '').localeCompare(String(a.finished_at || ''))).slice(0, 10);
      const post = async (path, body, success) => {
        try {
          const options = {method:'POST', cache:'no-store'};
          if (body !== undefined) { options.headers = {'Content-Type':'application/json'}; options.body = JSON.stringify(body); }
          const response = await fetch('/api/caduceus/' + path, options);
          const result = await response.json().catch(() => ({}));
          if (result?.readback?.status === 404 || response.status === 404) showCoronatioToast('Caduceus is not deployed yet.', 'warning');
          else if (result?.accepted) showCoronatioToast(success, 'success');
          else if (result?.readback?.status === 409) showCoronatioToast('This action is already running or cannot be changed yet.', 'warning');
          else showCoronatioToast('This action could not be completed.', 'warning');
        } catch (_) { showCoronatioToast('This action could not be completed.', 'warning'); }
      };
      let view = 'Overview', generation = 0;
      async function render() {
        const current = ++generation, selected = view;
        content.textContent = 'Reading updates…';
        const [status, history, interactables, modules, service] = await Promise.all([
          read('update/status'), read('update/history'), read('interactables'),
          selected === 'Modules' ? read('update/modules') : null,
          selected === 'Schedule' ? read('update/service/status') : null
        ]);
        if (!modal.isConnected || generation !== current || !headerState.isAdmin) return;
        const runs = recent(history), rows = list(interactables, 'interactables');
        if (selected === 'Overview') {
          const latest = runs[0];
          const waiting = status?.updates_available ?? status?.updatesAvailable ?? status?.pending_count ?? status?.pendingCount;
          const failed = latest && ['failed','failure','error'].includes(String(latest.outcome).toLowerCase());
          const headline = failed ? 'Last update failed' : typeof waiting === 'number' ? (waiting > 0 ? `${waiting} updates waiting` : 'Up to date') : 'Update status unavailable';
          const failures = runs.flatMap(run => Array.isArray(run.failures) ? run.failures : []).slice(0,3);
          content.innerHTML = `<p><strong>${esc(headline)}</strong></p>${latest?.finished_at ? `<p>Last update: ${esc(localTime(latest.finished_at))}</p>` : ''}<div aria-label="Recent outcomes">${runs.map(run => `<span>${esc(run.outcome)} </span>`).join('')}</div>${failures.map(f => `<p>${esc(f.module)}: ${esc(f.signal)}</p>`).join('')}<p><button type="button" class="ui-button ui-button--secondary ui-button--small" data-update-action="check">Check for updates</button> <button type="button" class="ui-button ui-button--primary ui-button--small" data-update-action="now">Update now</button></p>`;
        } else if (selected === 'Modules') {
          content.innerHTML = list(modules, 'modules').map(m => `<article><strong>${esc(m.title || String(m.id || '').replace(/[-_]+/g, ' '))}</strong>${m.summary ? `<p>${esc(m.summary)}</p>` : ''}<label>Enabled <input type="checkbox" data-update-module="${esc(m.id)}" ${m.enabled ? 'checked' : ''}></label></article>`).join('') || '<p>No modules available.</p>';
        } else if (selected === 'Schedule') {
          const known = typeof service?.enabled === 'boolean';
          const cadence = status?.cadence ?? status?.schedule;
          content.innerHTML = `<label>Automatic updates <input type="checkbox" data-update-schedule ${service?.enabled ? 'checked' : ''} ${known ? '' : 'disabled'}></label><p>${known ? `Timer ${service.active ? 'active' : 'inactive'}` : 'Automatic updates status unavailable'}</p>${typeof cadence === 'string' && cadence ? `<p>${esc(cadence)}</p>` : ''}`;
        } else if (selected === 'Interactables') {
          const draw = (items, silenced) => items.map(row => `<article><strong>${esc(row.name || row.id)}</strong>${row.summary ? `<p>${esc(row.summary)}</p>` : ''}${silenced ? `<button type="button" data-interactable-action="unsilence" data-interactable-id="${esc(row.id)}">Unsilence</button>` : `<button type="button" data-interactable-action="run" data-interactable-id="${esc(row.id)}">Run</button> <button type="button" data-interactable-action="silence" data-interactable-id="${esc(row.id)}">Silence</button>`}</article>`).join('');
          const pending = rows.filter(row => !row.silenced), muted = rows.filter(row => row.silenced);
          content.innerHTML = draw(pending, false) + (muted.length ? `<p>Silenced</p>${draw(muted, true)}` : '') || '<p>No interactables available.</p>';
        } else {
          content.innerHTML = runs.map(run => `<article><p>${esc(localTime(run.finished_at))} · ${esc(run.outcome)} · ${esc(run.duration_ms)} ms · ${esc(run.module_count)} modules · ${esc(run.updated_count)} updated</p>${(Array.isArray(run.failures) ? run.failures : []).map(f => `<p>${esc(f.module)}: ${esc(f.signal)} <button type="button" data-copy-failure="${esc(`${f.module}: ${f.signal}`)}">Copy</button></p>`).join('')}</article>`).join('') || '<p>No update history available.</p>';
        }
      }
      modal.addEventListener('click', async event => {
        const target = event.target instanceof Element ? event.target : null;
        const tab = target?.closest('[data-update-view]');
        if (tab) { view = tab.dataset.updateView; await render(); return; }
        const action = target?.closest('[data-update-action]');
        if (action) { action.disabled = true; const kind = action.dataset.updateAction; await post('update/' + kind, undefined, kind === 'now' ? 'Update started — the crown will restart when it completes' : 'Update check started.'); if (action.isConnected) action.disabled = false; return; }
        const item = target?.closest('[data-interactable-action]');
        if (item) { item.disabled = true; await post('interactables/' + encodeURIComponent(item.dataset.interactableId) + '/' + item.dataset.interactableAction, undefined, 'Interactable updated.'); await render(); return; }
        const copy = target?.closest('[data-copy-failure]');
        if (copy) { try { await navigator.clipboard.writeText(copy.dataset.copyFailure); showCoronatioToast('Copied.', 'success'); } catch (_) { showCoronatioToast('Could not copy.', 'warning'); } }
      });
      modal.addEventListener('change', async event => {
        const input = event.target;
        if (!(input instanceof HTMLInputElement)) return;
        if (input.matches('[data-update-schedule]')) { const state = input.checked ? 'on' : 'off'; input.disabled = true; await post('update/service/toggle', {state}, `Automatic updates turned ${state}.`); await render(); }
        else if (input.matches('[data-update-module]')) { const enabled = input.checked; input.disabled = true; await post('update/modules/' + encodeURIComponent(input.dataset.updateModule), {enabled}, 'Module updated.'); await render(); }
      });
      void render();
    }
    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      if (target?.closest('[data-update-manager-open]')) { event.preventDefault(); systemUpdateManager(); }
    });
