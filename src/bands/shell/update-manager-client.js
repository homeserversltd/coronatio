    function systemUpdateManager() {
      if (!headerState.isAdmin) return;
      const names = ['Overview', 'Modules', 'Schedule', 'Interactables', 'History'];
      const body = `<nav aria-label="Update manager views">${names.map(name => `<button type="button" class="ui-button ui-button--secondary ui-button--small" data-update-view="${name}">${name}</button>`).join(' ')}</nav><div data-update-content aria-live="polite"></div>`;
      const modal = managerModal('updates', 'System Update Manager', body);

      modal.querySelector('[data-manager-confirm]').remove();
      const close = modal.querySelector('.modal-actions [data-manager-close]');
      if (close) close.textContent = 'Close';
      const content = modal.querySelector('[data-update-content]');
      const esc = value => escapeDiskHtml(String(value ?? '')).replace(/"/g, '&quot;').replace(/'/g, '&#39;');
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
          if (result?.readback?.status === 404 || response.status === 404) showCoronatioToast(path === 'update/service/toggle' ? 'Automatic updates unavailable — Caduceus is not deployed yet.' : path === 'update/now' ? 'Update pending — Caduceus is not deployed yet.' : 'Caduceus is not deployed yet.', 'warning');
          else if (result?.accepted) showCoronatioToast(success, 'success');
          else if (result?.readback?.status === 409) showCoronatioToast(path === 'update/now' ? 'Update is already running.' : path === 'update/service/toggle' ? 'Automatic updates could not be changed yet.' : 'This action could not be completed yet.', 'warning');
          else showCoronatioToast('This action could not be completed.', 'warning');
        } catch (_) { showCoronatioToast('This action could not be completed.', 'warning'); }
      };
      let view = 'Overview', generation = 0;
      async function render() {
        const current = ++generation, selected = view;
        content.textContent = 'Reading updates…';
        const [status, history, interactables, modules, service, stats] = await Promise.all([
          read('update/status'), read('update/history'), read('interactables'),
          selected === 'Modules' ? read('update/modules') : null,
          selected === 'Schedule' ? read('update/service/status') : null,
          selected === 'Overview' ? read('appliance/stats') : null
        ]);
        if (!modal.isConnected || generation !== current || !headerState.isAdmin) return;
        const runs = recent(history), rows = list(interactables, 'interactables').filter(row => typeof row?.target_path === 'string' && row.target_path.length > 0);
        if (selected === 'Overview') {
          const latest = runs[0];
          const stored = stats?.update, answer = stored?.answer;
          const pendingCount = answer?.modules?.pending ?? answer?.pending_software_steps;
          const validCount = value => Number.isSafeInteger(value) && value >= 0;
          const ready = answer?.ok === true && typeof answer.pending === 'boolean' && (!answer.pending || validCount(pendingCount));
          const failed = latest?.outcome === 'failed';
          const headline = !ready ? 'Update status unavailable' : failed ? 'Last update failed' : answer.pending ? `${pendingCount} updates waiting` : 'Up to date';
          const checked = typeof stored?.checked_at === 'number' ? new Date(stored.checked_at * 1000) : new Date(stored?.checked_at);
          const details = [];
          if (stored?.checked_at != null && !Number.isNaN(checked.valueOf())) details.push(`checked ${checked.toLocaleTimeString([], {hour:'2-digit', minute:'2-digit'})}`);
          if (validCount(answer?.modules?.unobserved) && answer.modules.unobserved > 0) details.push(`${answer.modules.unobserved} modules unread`);
          const proposals = answer?.config?.proposals ?? answer?.config_proposals;
          if (validCount(proposals) && proposals > 0) details.push(`${proposals} files to review`);
          const failures = failed && Array.isArray(latest.failures) ? latest.failures.slice(0,3) : [];
          content.innerHTML = `<p><strong>${esc(headline)}</strong></p>${details.length ? `<p>${esc(details.join(' · '))}</p>` : ''}${latest?.finished_at ? `<p>Last update: ${esc(localTime(latest.finished_at))}</p>` : ''}<div aria-label="Recent outcomes">${runs.map(run => `<span>${esc(run.outcome)} </span>`).join('')}</div>${failures.map(f => `<p>${esc(f.module)}: ${esc(f.signal)}</p>`).join('')}<p><button type="button" class="ui-button ui-button--secondary ui-button--small" data-update-action="check">Check for updates</button> <button type="button" class="ui-button ui-button--primary ui-button--small" data-update-action="now">Update now</button></p>`;
        } else if (selected === 'Modules') {
          content.innerHTML = list(modules, 'modules').map(m => `<article><strong>${esc(m.title || String(m.id || '').replace(/[-_]+/g, ' '))}</strong>${m.summary ? `<p>${esc(m.summary)}</p>` : ''}<label>Enabled <input type="checkbox" data-update-module="${esc(m.id)}" ${m.enabled === true ? 'checked' : ''} ${typeof m.enabled === 'boolean' ? '' : 'disabled'}></label></article>`).join('') || '<p>No modules available.</p>';
        } else if (selected === 'Schedule') {
          const known = typeof service?.enabled === 'boolean';
          const cadence = status?.cadence ?? status?.schedule;
          const timerStatus = typeof service?.active === 'boolean' ? `Timer ${service.active ? 'active' : 'inactive'}` : 'Timer status unavailable';
          content.innerHTML = `<label>Automatic updates <input type="checkbox" data-update-schedule ${service?.enabled ? 'checked' : ''} ${known ? '' : 'disabled'}></label><p>${known ? timerStatus : 'Automatic updates status unavailable'}</p>${typeof cadence === 'string' && cadence ? `<p>${esc(cadence)}</p>` : ''}`;
        } else if (selected === 'Interactables') {
          const draw = (items, silenced) => items.map(row => {
            const diff = typeof row.diff === 'string' && row.diff ? `<pre class="ux-readout">${row.diff.split('\n').map(line => `<span${line.startsWith('-') && !line.startsWith('---') ? ' class="error"' : line.startsWith('+') && !line.startsWith('+++') ? ' class="success"' : ''}>${esc(line)}</span>`).join('\n')}</pre>` : '<p>Diff unavailable</p>';
            return `<article><strong>${esc(row.target_path)}</strong>${row.slice?.sentence ? `<p>${esc(row.slice.sentence)}</p>` : ''}${diff}${silenced ? `<button type="button" data-interactable-action="unsilence" data-interactable-id="${esc(row.id)}">Unsilence</button>` : `<button type="button" data-interactable-action="run" data-interactable-id="${esc(row.id)}">Yes</button> <button type="button" data-interactable-action="silence" data-interactable-id="${esc(row.id)}">Silence</button>`}</article>`;
          }).join('');
          const pending = rows.filter(row => !row.silenced), muted = rows.filter(row => row.silenced);
          content.innerHTML = !(Array.isArray(interactables) || Array.isArray(interactables?.interactables)) ? '<p>Updates unavailable</p>' : rows.length ? draw(pending, false) + (muted.length ? `<p>Silenced</p>${draw(muted, true)}` : '') : '<p>Nothing waiting</p>';
        } else {
          content.innerHTML = runs.map(run => `<article><p>${esc(localTime(run.finished_at))} · ${esc(run.outcome)} · ${esc(run.duration_ms)} ms · ${esc(run.module_count)} modules · ${esc(run.updated_count)} updated</p>${(Array.isArray(run.failures) ? run.failures : []).map(f => `<p>${esc(f.module)}: ${esc(f.signal)} <button type="button" data-copy-failure="${esc(`${f.module}: ${f.signal}`)}">Copy</button></p>`).join('')}</article>`).join('') || '<p>No update history available.</p>';
        }
      }
      modal.addEventListener('click', async event => {
        if (!headerState.isAdmin || !modal.isConnected) return;
        const target = event.target instanceof Element ? event.target : null;
        const tab = target?.closest('[data-update-view]');
        if (tab) { view = tab.dataset.updateView; await render(); return; }
        const action = target?.closest('[data-update-action]');
        if (action) { action.disabled = true; const kind = action.dataset.updateAction; await post('update/' + kind, undefined, kind === 'now' ? 'Update started — the crown will restart when it completes' : 'Update check started.'); if (kind === 'check') await render(); if (action.isConnected) action.disabled = false; return; }
        const item = target?.closest('[data-interactable-action]');
        if (item) { item.disabled = true; await post('interactables/' + encodeURIComponent(item.dataset.interactableId) + '/' + item.dataset.interactableAction, undefined, 'Interactable updated.'); await render(); return; }
        const copy = target?.closest('[data-copy-failure]');
        if (copy) { try { await navigator.clipboard.writeText(copy.dataset.copyFailure); showCoronatioToast('Copied.', 'success'); } catch (_) { showCoronatioToast('Could not copy.', 'warning'); } }
      });
      modal.addEventListener('change', async event => {
        if (!headerState.isAdmin || !modal.isConnected) return;
        const input = event.target;
        if (!(input instanceof HTMLInputElement)) return;
        if (input.matches('[data-update-schedule]')) { const state = input.checked ? 'on' : 'off'; input.disabled = true; await post('update/service/toggle', {state}, `Automatic updates turned ${state}.`); await render(); }
        else if (input.matches('[data-update-module]')) { const enabled = input.checked; input.disabled = true; await post('update/modules/' + encodeURIComponent(input.dataset.updateModule), {enabled}, 'Module updated.'); await render(); }
      });
      void render();
    }
    window.addEventListener('coronatio:privileged-state-retired', () => {
      document.querySelectorAll('[data-manager-modal="updates"]').forEach(modal => modal.remove());
    });
    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      if (target?.closest('[data-update-manager-open]')) { event.preventDefault(); systemUpdateManager(); }
    });
