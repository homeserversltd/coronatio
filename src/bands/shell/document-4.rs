fn shell_document_4() -> &'static str {
    r####"      if (!el) return;
      el.textContent = 'Loading ' + route + '…'; try {
        const response = await fetch(route, { method }); const text = await response.text();
        try { el.textContent = JSON.stringify(JSON.parse(text), null, 2); }
        catch (_) { el.textContent = text; }
      } catch (error) { el.textContent = 'fetch failed: ' + error; }
    }
    document.querySelectorAll('[data-fetch]').forEach(button => button.addEventListener('click', () => fetchInto(button.dataset.fetch, button.dataset.target, button.dataset.method || 'GET'))); const uploadState = { currentPath: '/mnt/nas', selectedFiles: [], activeUploads: new Map(), pinRequired: false, policyReady: false, policyStale: false, binding: null, uploading: false, blacklist: [], blacklistReady: false, blacklistStale: false, history: [], historyReady: false, historyStale: false };
    const uploadFileInput = () => document.querySelector('[data-upload-file]'); const uploadSubmit = () => document.querySelector('[data-upload-submit]');
    const uploadProgressList = () => document.querySelector('[data-upload-progress-list]'); const uploadBreadcrumbs = () => document.querySelector('[data-upload-breadcrumbs]');
    const uploadReadout = () => document.getElementById('upload-readout'); function uploadFormatSize(bytes) {
      const units = ['B', 'KB', 'MB', 'GB']; let size = Number(bytes || 0);
      let unit = 0; while (size >= 1024 && unit < units.length - 1) { size /= 1024; unit += 1; }
      return size.toFixed(1) + ' ' + units[unit]; }
    function uploadCurrent(binding = uploadState.binding) { return Boolean(binding && uploadState.binding === binding && viewportFamilyAdmitted('upload') && scaffoldRegistry.current(binding)); }
    function uploadBinding() {
      const binding = viewportFamilyAdmitted('upload') ? scaffoldRegistry.capture('upload') : null;
      if (!binding) return null;
      if (uploadState.binding !== binding) {
        uploadState.binding = binding; uploadState.policyReady = false; uploadState.policyStale = false; uploadState.historyReady = false; uploadState.historyStale = false; uploadState.blacklistReady = false; uploadState.blacklistStale = false;
        binding.root.querySelectorAll('[data-upload-submit], [data-upload-pin-toggle]').forEach(node => { node.disabled = true; });
        scaffoldRegistry.onRetire(binding, () => {
          if (uploadState.binding !== binding) return;
          uploadState.binding = null; uploadState.policyReady = false; uploadState.policyStale = true;
          uploadState.history = []; uploadState.historyReady = false; uploadState.historyStale = true; uploadState.blacklist = []; uploadState.blacklistReady = false; uploadState.blacklistStale = true;
          const root = binding.root; root?.querySelectorAll('[data-upload-submit], [data-upload-pin-toggle]').forEach(node => { node.disabled = true; });
          root?.querySelectorAll('[data-upload-history-backdrop], [data-upload-blacklist-backdrop], [data-upload-pin-backdrop]').forEach(node => { node.hidden = true; node.setAttribute('aria-hidden', 'true'); });
          root?.querySelectorAll('[data-upload-history-list], [data-upload-blacklist-entries]').forEach(node => node.replaceChildren());
        });
      }
      return binding;
    }
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane, binding = uploadState.binding;
      if (detail.id !== 'upload' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="upload"]') !== pane) return;
      if (binding && (binding.guest !== detail.id || scaffoldRegistry.current(binding))) return;
      uploadState.activeUploads.forEach(upload => { upload.removed = true; upload.xhr?.abort(); });
      uploadState.activeUploads.clear(); uploadState.currentPath = '/mnt/nas'; uploadState.selectedFiles = []; uploadState.uploading = false;
      uploadState.pinRequired = false; uploadState.policyReady = false; uploadState.policyStale = false; uploadState.binding = null;
      uploadState.blacklist = []; uploadState.blacklistReady = false; uploadState.blacklistStale = false;
      uploadState.history = []; uploadState.historyReady = false; uploadState.historyStale = false;
    });
    function uploadReadStatus(container, key, message, state = 'stale', retry = false) {
      if (!container) return;
      let node = container.querySelector(':scope > [data-upload-read-status]');
      if (!message) { node?.remove(); return; }
      if (!node) { node = document.createElement('p'); node.dataset.uploadReadStatus = key; node.setAttribute('role', 'status'); container.prepend(node); }
      node.replaceChildren(document.createTextNode(message)); node.dataset.state = state;
      if (retry) { const button = document.createElement('button'); button.type = 'button'; button.dataset.uploadReadRetry = key; button.textContent = 'Retry'; node.append(document.createTextNode(' '), button); }
    }
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
    function setUpload(filename, update, binding = uploadState.binding) {
      if (!uploadCurrent(binding)) return;
      const current = uploadState.activeUploads.get(filename) || { filename, progress: 0, speed: 0, uploaded: 0, total: 0, status: 'pending' };
      uploadState.activeUploads.set(filename, Object.assign(current, update));
      renderUploadProgress();
    }
    function setUploadSelection() {
      uploadState.selectedFiles = Array.from(uploadFileInput()?.files || []);
      const submit = uploadSubmit();
      if (submit) submit.disabled = uploadState.selectedFiles.length === 0 || uploadState.uploading || !uploadState.policyReady || uploadState.policyStale || !uploadCurrent();
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
      const binding = uploadBinding(); if (!binding || !uploadCurrent(binding)) return;
      const activeUploadTree = binding.root.querySelector('[data-upload-tree]');
      const selected = activeUploadTree?.querySelector('.directory-entry[aria-selected="true"]')?.dataset.directoryPath || uploadCurrentPath();
      uploadState.currentPath = selected || '/mnt/nas';
      const field = binding.root.querySelector('[data-upload-current-path]');
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
    async function uploadOneFile(file, scopedAttendance = null, binding = uploadState.binding) {
      if (!uploadCurrent(binding) || !uploadState.policyReady || uploadState.policyStale) throw new Error('Upload view is not current');
      const CHUNK_SIZE = 4194304;
      setUpload(file.name, { filename: file.name, progress: 0, speed: 0, uploaded: 0, total: file.size, status: 'pending', uploadId: null, xhr: null, removed: false, attendance: scopedAttendance || coronatioAttendanceRuntime.currentAttendance }, binding);
      const uploadHeaders = (uploadAttendance = scopedAttendance) => {
        const headers = { 'X-Caduceus-Document': coronatioAttendanceRuntime.documentIncarnation };
        if (uploadAttendance || coronatioAttendanceRuntime.currentAttendance) headers['X-Caduceus-Attendance'] = uploadAttendance || coronatioAttendanceRuntime.currentAttendance;
        return headers;
      };
      const uploadFailure = (status, text) => {
        let msg = 'Upload failed with status ' + status; let signal = '';
        try { const payload = JSON.parse(text || '{}'); signal = payload.firstMissingSignal || ''; if (payload.error) msg = payload.error; } catch (_) {}
        const error = new Error(msg); if (status === 428 && signal === 'upload-pin-required') error.uploadPinRequired = true; return error;
      };
      const uploadRecord = uploadState.activeUploads.get(file.name);
      const removed = () => uploadRecord?.removed === true;
      const startUpload = await new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest(); const current = uploadState.activeUploads.get(file.name); if (current) current.xhr = xhr;
        xhr.onload = () => {
          if (removed()) { reject(Object.assign(new Error('Upload removed'), { uploadRemoved: true })); return; }
          if (xhr.status < 200 || xhr.status >= 300) { reject(uploadFailure(xhr.status, xhr.responseText)); return; }
          let result = {}; try { result = JSON.parse(xhr.responseText || '{}'); } catch (_) {}
          const uploadId = result.upload_id || result.uploadId || result.id;
          if (!uploadId) { reject(new Error('Upload start response did not include an upload ID')); return; }
          resolve(String(uploadId));
        };
        xhr.onerror = () => { if (removed()) reject(Object.assign(new Error('Upload removed'), { uploadRemoved: true })); else reject(new Error('Network error occurred during upload')); };
        xhr.onabort = () => reject(Object.assign(new Error('Upload removed'), { uploadRemoved: true }));
        xhr.open('POST', '/api/files/upload/start');
        xhr.setRequestHeader('Content-Type', 'application/json'); const startHeaders = uploadHeaders(); Object.keys(startHeaders).forEach(name => xhr.setRequestHeader(name, startHeaders[name]));
        xhr.send(JSON.stringify({ filename: file.name, total_size: file.size, target_dir: uploadCurrentPath(), chunk_size: CHUNK_SIZE }));
      });
      if (removed()) throw Object.assign(new Error('Upload removed'), { uploadRemoved: true });
      setUpload(file.name, { uploadId: startUpload, status: 'uploading', xhr: null, uploaded: 0, total: file.size }, binding);
      const startedAt = Date.now(); let index = 0; let uploaded = 0;
      while (uploaded < file.size) {
        const offset = uploaded; const end = Math.min(file.size, offset + CHUNK_SIZE); const chunk = file.slice(offset, end);
        await new Promise((resolve, reject) => {
          const xhr = new XMLHttpRequest(); const current = uploadState.activeUploads.get(file.name); if (current) current.xhr = xhr;
          xhr.upload.onprogress = event => {
            if (!event.lengthComputable) return;
            const cumulative = Math.min(file.size, offset + event.loaded); const elapsed = Math.max(1, Date.now() - startedAt) / 1000;
            if (removed()) return;
            setUpload(file.name, { progress: file.size ? (cumulative / file.size) * 100 : 0, speed: cumulative / elapsed, uploaded: cumulative, total: file.size, status: 'uploading', uploadId: startUpload, xhr }, binding);
          };
          xhr.onload = () => {
            if (removed()) { reject(Object.assign(new Error('Upload removed'), { uploadRemoved: true })); return; }
            if (xhr.status < 200 || xhr.status >= 300) reject(uploadFailure(xhr.status, xhr.responseText)); else resolve();
          };
          xhr.onerror = () => { if (removed()) reject(Object.assign(new Error('Upload removed'), { uploadRemoved: true })); else reject(new Error('Network error occurred during upload')); };
          xhr.onabort = () => reject(Object.assign(new Error('Upload removed'), { uploadRemoved: true }));
          xhr.open('POST', '/api/files/upload/' + encodeURIComponent(startUpload) + '/chunk/' + index);
          const chunkHeaders = uploadHeaders(); Object.keys(chunkHeaders).forEach(name => xhr.setRequestHeader(name, chunkHeaders[name]));
          xhr.setRequestHeader('Content-Type', 'application/octet-stream');
          xhr.send(chunk);
        });
        if (removed()) throw Object.assign(new Error('Upload removed'), { uploadRemoved: true });
        uploaded = end; index += 1;
        const elapsed = Math.max(1, Date.now() - startedAt) / 1000;
        setUpload(file.name, { progress: file.size ? (uploaded / file.size) * 100 : 0, speed: uploaded / elapsed, uploaded, total: file.size, status: 'uploading', uploadId: startUpload }, binding);
      }
      const complete = await fetch('/api/files/upload/' + encodeURIComponent(startUpload) + '/complete', { method: 'POST', headers: uploadHeaders(), cache: 'no-store' });
      const completeText = await complete.text();
      if (removed()) throw Object.assign(new Error('Upload removed'), { uploadRemoved: true });
      if (!complete.ok) throw uploadFailure(complete.status, completeText);
      if (removed()) throw Object.assign(new Error('Upload removed'), { uploadRemoved: true });
      setUpload(file.name, { progress: 100, uploaded: file.size, total: file.size, speed: file.size / Math.max(1, Date.now() - startedAt) * 1000, status: 'completed', uploadId: startUpload, xhr: null }, binding);
      const readout = uploadCurrent(binding) ? uploadReadout() : null; if (readout) readout.textContent = completeText;
      return { removed: false };
    }
    async function uploadSelectedFiles(scopedAttendance = null, binding = uploadBinding()) {
      if (!binding || !uploadCurrent(binding) || !uploadState.policyReady || uploadState.policyStale) return;
      if (!uploadState.selectedFiles.length) { showCoronatioToast('No files selected for upload', 'error'); return; }
      if (uploadState.pinRequired && !headerState.isAdmin && !scopedAttendance) { openUploadModal('[data-upload-pin-modal]'); binding.root.querySelector('[data-upload-pin-input]')?.focus(); return; }
      uploadState.uploading = true;
      const submit = uploadSubmit();
      if (submit) { submit.disabled = true; submit.textContent = 'Uploading...'; }
      let success = 0; let failed = 0; let pinRequired = false;
      for (const file of uploadState.selectedFiles) {
        if (!uploadCurrent(binding)) break;
        try { const outcome = await uploadOneFile(file, scopedAttendance, binding); if (!outcome?.removed) success += 1; }
        catch (error) { if (!uploadCurrent(binding)) break; if (error?.uploadRemoved) continue; if (error?.uploadPinRequired && !headerState.isAdmin) { pinRequired = true; setUpload(file.name, { status: 'pending', error: '' }, binding); break; } failed += 1; setUpload(file.name, { status: 'error', error: error?.message || String(error) }, binding); showCoronatioToast(`Failed to upload ${file.name}: ${error?.message || error}`, 'error'); }
      }
      if (uploadState.binding === binding) uploadState.uploading = false;
      if (!uploadCurrent(binding)) return;
      const submitAfterUpload = uploadSubmit();
      if (submitAfterUpload) { submitAfterUpload.textContent = 'Upload Selected Files'; submitAfterUpload.disabled = uploadState.selectedFiles.length === 0 || !uploadState.policyReady || uploadState.policyStale; } if (pinRequired) { openUploadModal('[data-upload-pin-modal]'); binding.root.querySelector('[data-upload-pin-input]')?.focus(); return; }
      const summary = failed ? `Uploaded ${success} file(s), ${failed} failed` : `Successfully uploaded ${success} file(s)`;
      const readout = uploadReadout();
      if (readout) readout.textContent = summary;
      if (success && failed) showCoronatioToast(summary, 'warning'); else if (success) showCoronatioToast(summary, 'success');
    }
    function uploadAdminHeaders(json = false) { return json ? { 'content-type': 'application/json' } : {}; }
    async function hydrateUploadPolicy(binding = uploadBinding()) {
      if (!binding || !uploadCurrent(binding)) return false;
      const host = binding.root.querySelector('[data-upload-viewport]') || binding.root;
      return scaffoldRegistry.run(binding, 'upload-policy', async context => {
        const data = await context.fetchJson('/api/upload/pin-required-status', { headers: uploadAdminHeaders() });
        if (!context.current() || !uploadCurrent(binding)) return false;
        if (!data || typeof data !== 'object' || Array.isArray(data) || data.ok === false || typeof data.isPinRequired !== 'boolean') throw new Error('Upload policy response is incomplete');
        uploadState.pinRequired = data.isPinRequired; uploadState.policyReady = true; uploadState.policyStale = false;
        const toggle = binding.root.querySelector('[data-upload-pin-toggle]');
        if (toggle) { toggle.classList.toggle('active', uploadState.pinRequired); toggle.disabled = !headerState.isAdmin || uploadState.policyStale; }
        uploadReadStatus(host, 'upload-policy', '', 'ready');
        setUploadSelection();
        return true;
      }, {
        slots: [], timeoutMs: 8000, slowMs: 750,
        onFailure: ({ current, reason }) => {
          if (!current() || !uploadCurrent(binding)) return;
          uploadState.policyStale = true;
          binding.root.querySelector('[data-upload-pin-toggle]')?.setAttribute('disabled', '');
          uploadReadStatus(host, 'upload-policy', `Upload policy unavailable: ${reason || 'Read failed'}. ${uploadState.policyReady ? 'Showing last known policy.' : 'Uploads remain disabled.'}`, 'stale', true);
          setUploadSelection();
        }
      });
    }
    function openUploadModal(selector) { const modal = document.querySelector(selector); if (modal) { modal.hidden = false; modal.classList.add('open'); modal.setAttribute('aria-hidden', 'false'); } }
    function closeUploadModal(modal) { if (modal) { modal.classList.remove('open'); modal.setAttribute('aria-hidden', 'true'); modal.hidden = true; const pin = modal.querySelector('[data-upload-pin-input]'); if (pin) pin.value = ''; const message = modal.querySelector('[data-upload-pin-message]'); if (message) message.textContent = ''; } }
    async function refreshUploadHistory(binding = uploadBinding()) {
      const modal = binding?.root.querySelector('[data-upload-history-modal]'), list = modal?.querySelector('.upload-history-list'), empty = modal?.querySelector('.uploadHistoryModal'), clear = modal?.querySelector('[data-upload-clear-history]');
      if (!binding || !uploadCurrent(binding) || !headerState.isAdmin || !modal) return false;
      return scaffoldRegistry.run(binding, 'upload-history', async context => {
        const data = await context.fetchJson('/api/upload/history', { headers: uploadAdminHeaders() });
        if (!context.current() || !uploadCurrent(binding) || !headerState.isAdmin) return false;
        if (!data || typeof data !== 'object' || Array.isArray(data) || data.ok === false || !Array.isArray(data.history)) throw new Error('Upload history response is incomplete');
        uploadState.history = data.history.map(line => String(line)); uploadState.historyReady = true; uploadState.historyStale = false;
        if (!context.current() || !uploadCurrent(binding)) return false;
        if (list) { list.hidden = uploadState.history.length === 0; morphLivePane(list, uploadState.history.map((line, index) => `<div class="history-item ${line.includes('Successfully') ? 'success' : 'error'}" data-upload-history-row="${index}">${escapeHtml(line)}</div>`).join('')); }
        if (empty) empty.hidden = uploadState.history.length !== 0;
        if (clear) clear.disabled = uploadState.history.length === 0 || uploadState.historyStale;
        uploadReadStatus(modal.querySelector('.modal-content'), 'upload-history', '', 'ready');
        return true;
      }, { slots: [], timeoutMs: 8000, slowMs: 750, onFailure: ({ current, reason }) => {
        if (!current() || !uploadCurrent(binding) || !headerState.isAdmin) return;
        uploadState.historyStale = true;
        if (empty) empty.hidden = !uploadState.historyReady || uploadState.history.length !== 0;
        if (list) list.hidden = !uploadState.historyReady || uploadState.history.length === 0;
        if (clear) clear.disabled = true;
        uploadReadStatus(modal.querySelector('.modal-content'), 'upload-history', `History refresh unavailable: ${reason || 'Read failed'}. ${uploadState.historyReady ? 'Showing last known history.' : ''}`, 'stale', true);
      } });
    }
    async function refreshUploadBlacklist(binding = uploadBinding()) {
      const modal = binding?.root.querySelector('[data-upload-blacklist-modal]'), entries = modal?.querySelector('[data-upload-blacklist-entries]');
      if (!binding || !uploadCurrent(binding) || !headerState.isAdmin || !modal) return false;
      return scaffoldRegistry.run(binding, 'upload-blacklist', async context => {
        const data = await context.fetchJson('/api/upload/blacklist/list', { headers: uploadAdminHeaders() });
        if (!context.current() || !uploadCurrent(binding) || !headerState.isAdmin) return false;
        if (!data || typeof data !== 'object' || Array.isArray(data) || data.ok === false || !Array.isArray(data.blacklist) || data.blacklist.some(entry => typeof entry !== 'string')) throw new Error('Upload blacklist response is incomplete');
        uploadState.blacklist = data.blacklist.slice(); uploadState.blacklistReady = true; uploadState.blacklistStale = false;
        if (!context.current() || !uploadCurrent(binding)) return false;
        refreshUploadBlacklistDomOnly(binding);
        uploadReadStatus(modal.querySelector('.modal-content'), 'upload-blacklist', '', 'ready');
        return true;
      }, { slots: [], timeoutMs: 8000, slowMs: 750, onFailure: ({ current, reason }) => {
        if (!current() || !uploadCurrent(binding) || !headerState.isAdmin) return;
        uploadState.blacklistStale = true;
        refreshUploadBlacklistDomOnly(binding);
        uploadReadStatus(modal.querySelector('.modal-content'), 'upload-blacklist', `Blacklist refresh unavailable: ${reason || 'Read failed'}. ${uploadState.blacklistReady ? 'Showing last known entries.' : ''}`, 'stale', true);
      } });
    }
    function refreshUploadBlacklistDomOnly(binding = uploadState.binding) {
      if (!uploadCurrent(binding) || !headerState.isAdmin) return;
      const entries = binding.root.querySelector('[data-upload-blacklist-entries]');
      if (entries) morphLivePane(entries, uploadState.blacklist.map((entry, index) => `<div class="blacklist-entry" data-blacklist-entry="${escapeHtml(entry)}"><span class="entry-path">${escapeHtml(entry)}</span><button type="button" class="remove-entry" data-blacklist-remove="${index}" ${uploadState.blacklistStale ? 'disabled' : ''} aria-label="Remove entry">×</button></div>`).join('') || '<p class="blacklist-empty" data-blacklist-empty>No blocked paths.</p>');
      const modal = binding.root.querySelector('[data-upload-blacklist-modal]');
      modal?.querySelectorAll('[data-upload-blacklist-add], [data-upload-blacklist-submit]').forEach(node => { node.disabled = uploadState.blacklistStale; });
    }
    async function verifyUploadPin(binding = uploadBinding()) {
      if (!binding || !uploadCurrent(binding) || !uploadState.policyReady) return;
      const modal = binding.root.querySelector('[data-upload-pin-modal]'), input = modal?.querySelector('[data-upload-pin-input]'), message = modal?.querySelector('[data-upload-pin-message]'), pin = input?.value?.trim() || '';
      if (!pin) { if (message) message.textContent = 'Admin PIN is required.'; input?.focus(); return; }
      let scopedAttendance = null;
      try {
        const response = await fetch('/api/v1/attendance/open', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ pin }) });
        const result = await response.json().catch(() => ({}));
        scopedAttendance = response.ok && result.admin === true && typeof result.attendance === 'string' ? result.attendance : null;
        if (!uploadCurrent(binding)) return;
        if (!scopedAttendance) { if (message) message.textContent = response.status === 401 ? 'Invalid PIN' : (result.firstMissingSignal || 'PIN check unavailable'); input?.focus(); return; }
        showCoronatioToast('PIN Verified.', 'success'); closeUploadModal(modal); await uploadSelectedFiles(scopedAttendance, binding);
      } catch (_) { if (uploadCurrent(binding) && message) { message.textContent = 'PIN check unavailable'; input?.focus(); } }
      finally { if (scopedAttendance) await fetch('/api/v1/attendance/invalidate', { method: 'POST', headers: { 'X-Caduceus-Document': coronatioAttendanceRuntime.documentIncarnation, 'X-Caduceus-Attendance': scopedAttendance }, cache: 'no-store' }).catch(() => {}); }
    }
    function refreshUploadTree(path = uploadCurrentPath()) { const binding = uploadBinding(), tree = binding?.root.querySelector('[data-upload-tree][data-scaffold-slot="upload-tree"]'); if (!binding || !uploadCurrent(binding) || !tree) return false; return window.htmx?.ajax('GET', '/admit/upload/tree?path=%2Fmnt%2Fnas&depth=0&selected=' + encodeURIComponent(path), { source: tree, target: tree, swap: 'innerHTML' }); }
    async function postUploadDirectoryAction(url, successMessage) { const binding = uploadBinding(); if (!binding || !uploadCurrent(binding) || !headerState.isAdmin) return null; try { const response = await fetch(url, { method: 'POST', headers: uploadAdminHeaders(true), body: JSON.stringify({ directory: uploadState.currentPath }) }); const data = await response.json().catch(() => ({})); if (!uploadCurrent(binding)) return null; if (!response.ok || !(data.success ?? data.ok)) throw new Error(data.message || data.error || data.firstMissingSignal || `Request failed with status ${response.status}`); showCoronatioToast(successMessage, 'success'); return data; } catch (error) { if (uploadCurrent(binding)) showCoronatioToast(error?.message || 'Request failed', 'error'); return null; } }
    async function toggleUploadPin(toggle) { const binding = uploadBinding(); if (!binding || !uploadCurrent(binding) || !headerState.isAdmin || !uploadState.policyReady) return; const previous = uploadState.pinRequired, next = !previous; toggle.disabled = true; toggle.setAttribute('aria-busy', 'true'); toggle.querySelector('[data-upload-pin-spinner]')?.removeAttribute('hidden'); try { const response = await fetch('/api/upload/pin-required-status', { method: 'POST', headers: uploadAdminHeaders(true), body: JSON.stringify({ isPinRequired: next }) }); if (!uploadCurrent(binding)) return; if (!response.ok) throw new Error(`Request failed (${response.status})`); uploadState.pinRequired = next; toggle.classList.toggle('active', next); toggle.setAttribute('aria-label', `Toggle PIN requirement (currently ${next ? 'enabled' : 'disabled'})`); } catch (_) { if (uploadCurrent(binding)) { uploadState.pinRequired = previous; toggle.classList.toggle('active', previous); } } finally { if (uploadCurrent(binding)) { toggle.disabled = false; toggle.removeAttribute('aria-busy'); toggle.querySelector('[data-upload-pin-spinner]')?.setAttribute('hidden', ''); } } }
    document.body.addEventListener('change', event => { const target = event.target instanceof Element ? event.target : null; const binding = uploadBinding(); if (target?.matches('[data-upload-file]') && binding?.root.contains(target) && uploadCurrent(binding)) setUploadSelection(); });
    document.body.addEventListener('submit', event => { const target = event.target instanceof Element ? event.target : null; const binding = uploadBinding(); if (!target || !binding || !binding.root.contains(target) || !uploadCurrent(binding)) return; if (target.matches('[data-upload-pin-form]')) { event.preventDefault(); verifyUploadPin(binding); } if (target.matches('[data-upload-blacklist-form]')) { event.preventDefault(); target.querySelector('[data-upload-blacklist-add]')?.click(); } });
    document.body.addEventListener('click', async event => { const target = event.target instanceof Element ? event.target : null; if (!target) return;
      const retry = target.closest('[data-upload-read-retry]');
      if (retry) { const binding = uploadBinding(); if (!binding || !uploadCurrent(binding) || !headerState.isAdmin) return; event.preventDefault(); retry.disabled = true; scaffoldRegistry.retry(binding, retry.dataset.uploadReadRetry).finally(() => { if (retry.isConnected) retry.disabled = false; }); return; }
      const modal = target.closest('[data-upload-history-backdrop], [data-upload-blacklist-backdrop], [data-upload-pin-backdrop]'); if (modal && target === modal) { closeUploadModal(modal); return; }
      const control = target.closest('[data-upload-remove], [data-blacklist-remove], [data-upload-breadcrumb-path], [data-upload-modal-close], [data-upload-pin-cancel], [data-upload-submit], [data-upload-pin-confirm], [data-upload-refresh], [data-upload-force-allow], [data-upload-set-default], [data-upload-history], [data-upload-blacklist], [data-upload-blacklist-add], [data-upload-blacklist-submit], [data-upload-clear-history], [data-upload-pin-toggle]'); if (!control) return;
      const binding = uploadBinding(); if (!binding || !binding.root.contains(control) || !uploadCurrent(binding)) return;
      if (control.matches('[data-upload-remove]')) {
        const filename = control.dataset.uploadRemove; const upload = uploadState.activeUploads.get(filename);
        let cleanup = null;
        if (upload && (upload.status === 'pending' || upload.status === 'uploading')) {
          upload.removed = true; upload.xhr?.abort();
          if (upload.uploadId) {
            const headers = { 'X-Caduceus-Document': coronatioAttendanceRuntime.documentIncarnation };
            const attendance = upload.attendance || coronatioAttendanceRuntime.currentAttendance; if (attendance) headers['X-Caduceus-Attendance'] = attendance;
            cleanup = fetch('/api/files/upload/' + encodeURIComponent(upload.uploadId), { method: 'DELETE', headers, cache: 'no-store' }).catch(() => {});
          }
        }
        uploadState.activeUploads.delete(filename); renderUploadProgress(); if (cleanup) await cleanup; return;
      }
      if (control.matches('[data-blacklist-remove]')) { if (!headerState.isAdmin || !uploadState.blacklistReady || uploadState.blacklistStale) return; uploadState.blacklist.splice(Number(control.dataset.blacklistRemove), 1); refreshUploadBlacklistDomOnly(binding); return; }
      if (control.matches('[data-upload-breadcrumb-path]')) { uploadState.currentPath = control.dataset.uploadBreadcrumbPath || '/mnt/nas'; const field = document.querySelector('[data-upload-current-path]'); if (field) field.value = uploadState.currentPath; renderUploadBreadcrumbs(uploadState.currentPath); refreshUploadTree(uploadState.currentPath); return; }
      if (control.matches('[data-upload-modal-close], [data-upload-pin-cancel]')) { closeUploadModal(control.closest('[data-upload-history-backdrop], [data-upload-blacklist-backdrop], [data-upload-pin-backdrop]')); return; }
      if (control.matches('[data-upload-submit]')) { await uploadSelectedFiles(null, binding); return; }
      if (control.matches('[data-upload-pin-confirm]')) { await verifyUploadPin(binding); return; }
      if (control.matches('[data-upload-refresh]')) { refreshUploadTree(); return; }
      if (control.matches('[data-upload-force-allow]')) { if (confirm(`WARNING: This will override security settings for ${uploadState.currentPath}. \nOnly continue if you understand the risks.`)) await postUploadDirectoryAction('/api/upload/force-permissions', 'Directory permissions updated successfully'); return; }
      if (control.matches('[data-upload-set-default]')) { await postUploadDirectoryAction('/api/upload/default-directory', 'Default directory updated successfully'); return; }
      if (control.matches('[data-upload-history]')) { if (!headerState.isAdmin) return; openUploadModal('[data-upload-history-modal]'); await refreshUploadHistory(binding); return; }
      if (control.matches('[data-upload-blacklist]')) { if (!headerState.isAdmin) return; openUploadModal('[data-upload-blacklist-modal]'); await refreshUploadBlacklist(binding); return; }
      if (control.matches('[data-upload-blacklist-add]')) { if (!headerState.isAdmin || !uploadState.blacklistReady || uploadState.blacklistStale) return; const input = binding.root.querySelector('[data-upload-blacklist-input]'); const next = input?.value?.trim(); if (next && !uploadState.blacklist.includes(next)) { uploadState.blacklist.push(next); input.value = ''; refreshUploadBlacklistDomOnly(binding); } return; }
      if (control.matches('[data-upload-blacklist-submit]')) { if (!headerState.isAdmin || !uploadState.blacklistReady || uploadState.blacklistStale) return; const response = await fetch('/api/upload/blacklist/update', { method: 'PUT', headers: uploadAdminHeaders(true), body: JSON.stringify({ blacklist: uploadState.blacklist }) }); if (uploadCurrent(binding) && response.ok) refreshUploadTree(); return; }
      if (control.matches('[data-upload-clear-history]')) { if (!headerState.isAdmin || !uploadState.historyReady || uploadState.historyStale) return; const response = await fetch('/api/upload/history/clear', { method: 'POST', headers: uploadAdminHeaders() }); if (uploadCurrent(binding) && response.ok) await refreshUploadHistory(binding); return; }
      if (control.matches('[data-upload-pin-toggle]')) await toggleUploadPin(control); });
    syncUploadTreeSelection();
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
      if (load === null || load === undefined || !Number.isFinite(Number(load))) return null;
      const cores = navigator.hardwareConcurrency || 4;
      return Math.max(0, Math.min(100, (Number(load) / cores) * 100));
    }
    const statsChartState = { points: [], labels: [], cpu: [], temp: [], upload: [], download: [], lastRx: null, lastTx: null, lastIo: {}, lastStamp: null, ioSeries: {} };
    function optionalNumber(value) { return value === null || value === undefined || value === '' || !Number.isFinite(Number(value)) ? null : Number(value); }
    function statsNetworkTotals(data) {
      const network = data.network || {};
      const directRx = optionalNumber(network.receivedBytes), directTx = optionalNumber(network.sentBytes);
      const interfaces = Array.isArray(network.interfaces) ? network.interfaces : [];
      return {
        rx: directRx !== null ? directRx : interfaces.reduce((sum, iface) => { const value = optionalNumber(iface.rxBytes); return value === null ? sum : (sum === null ? value : sum + value); }, null),
        tx: directTx !== null ? directTx : interfaces.reduce((sum, iface) => { const value = optionalNumber(iface.txBytes); return value === null ? sum : (sum === null ? value : sum + value); }, null)
      };
    }
    function holdLastValue(values) {
      let last = null;
      return values.map(value => value === null || value === undefined ? last : (last = value));
    }
    function rebuildStatsChartSeries() {
      const points = statsChartState.points;
      statsChartState.labels = points.map(point => formatChartTime(point.timestamp));
      statsChartState.cpu = holdLastValue(points.map(point => point.cpu));
      statsChartState.temp = holdLastValue(points.map(point => point.temp));
      statsChartState.download = holdLastValue(points.map(point => point.download));
      statsChartState.upload = holdLastValue(points.map(point => point.upload));
      const keys = new Set(points.flatMap(point => Object.keys(point.io || {})));
      statsChartState.ioSeries = Object.fromEntries([...keys].map(key => [key, {
        read: holdLastValue(points.map(point => point.io?.[key]?.read ?? null)),
        write: holdLastValue(points.map(point => point.io?.[key]?.write ?? null)), label: key
      }]));
    }
    function statsTimestamp(value) {
      const numeric = optionalNumber(value);
      if (numeric !== null) return numeric > 100000000000 ? numeric : numeric * 1000;
      const parsed = Date.parse(String(value || ''));
      return Number.isFinite(parsed) ? parsed : Date.now();
    }
    function statsSourceStale(data) {
      const stale = data?.telemetry?.sectionStale;
      return ['cpu', 'memory', 'network', 'storage', 'io', 'processes'].some(section => stale?.[section] === true);
    }
    function statsSectionStale(data, section) { return data?.telemetry?.sectionStale?.[section] === true; }
    function setStatsRendererLoading(elementId, loading, root = document) {
      const header = root.querySelector(`[data-stat-element-id="${elementId}"] .stat-header`);
      if (header) header.classList.toggle('stats-renderer-loading', Boolean(loading));
    }
    function statsRendererPresent(data, section, elementId, contentPresent) {
      const stale = statsSectionStale(data, section);
      const element = document.querySelector(`[data-stat-element-id="${elementId}"]`);
      setStatsRendererLoading(elementId, stale);
      if (stale) {
        const age = section === 'keaLeases' ? data?.telemetry?.rosterSampleAgeMs : data?.telemetry?.sampleAgeMs;
        if (age === null || age === undefined || element?.dataset.scaffoldState !== 'ready') return false;
      }
      return Boolean(contentPresent);
    }
    function pushChartPoint(data, timestamp = Date.now()) {
      if (statsSourceStale(data)) return false;
      const totals = statsNetworkTotals(data);
      const previousStamp = statsChartState.lastStamp;
      if (previousStamp !== null && timestamp < previousStamp) return false;
      const seconds = previousStamp === null || timestamp <= previousStamp ? null : Math.max(0.001, (timestamp - previousStamp) / 1000);
      const download = statsChartState.lastRx === null || totals.rx === null || seconds === null ? null : Math.max(0, (totals.rx - statsChartState.lastRx) / seconds);
      const upload = statsChartState.lastTx === null || totals.tx === null || seconds === null ? null : Math.max(0, (totals.tx - statsChartState.lastTx) / seconds);
      const io = {}, devices = Array.isArray(data.io?.devices) ? data.io.devices : [];
      devices.forEach(device => {
        const key = device.device || device.mount; if (!key) return;
        const previous = statsChartState.lastIo[key], readBytes = optionalNumber(device.readBytes), writeBytes = optionalNumber(device.writeBytes);
        io[key] = {
          read: previous && previous.readBytes !== null && readBytes !== null && seconds !== null ? Math.max(0, (readBytes - previous.readBytes) / seconds) : null,
          write: previous && previous.writeBytes !== null && writeBytes !== null && seconds !== null ? Math.max(0, (writeBytes - previous.writeBytes) / seconds) : null
        };
        if (readBytes !== null || writeBytes !== null) statsChartState.lastIo[key] = { readBytes: readBytes ?? previous?.readBytes ?? null, writeBytes: writeBytes ?? previous?.writeBytes ?? null };
      });
      if (previousStamp === null || timestamp > previousStamp) {
        if (totals.rx !== null) statsChartState.lastRx = totals.rx;
        if (totals.tx !== null) statsChartState.lastTx = totals.tx;
        statsChartState.lastStamp = timestamp;
      }
      const point = { timestamp, cpu: loadToPercent(data.resources?.load?.one), temp: optionalNumber(data.resources?.load?.cpuTemperatureCelsius), download, upload, io };
      const existing = statsChartState.points.findIndex(candidate => candidate.timestamp === timestamp);
      if (existing >= 0) statsChartState.points[existing] = point;
      else statsChartState.points.push(point);
      statsChartState.points.sort((left, right) => left.timestamp - right.timestamp);
      if (statsChartState.points.length > 60) statsChartState.points.splice(0, statsChartState.points.length - 60);
      rebuildStatsChartSeries();
      return true;
    }
    const statsCharts = { cpu: null, network: null, io: null };
    function destroyStatsChart(key) { if (statsCharts[key]) { statsCharts[key].destroy(); statsCharts[key] = null; } }
    function resetStatsChartState() {
      statsChartState.points = []; statsChartState.labels = []; statsChartState.cpu = []; statsChartState.temp = [];
      statsChartState.upload = []; statsChartState.download = []; statsChartState.lastRx = null; statsChartState.lastTx = null;
      statsChartState.lastIo = {}; statsChartState.lastStamp = null; statsChartState.ioSeries = {};
    }
    function themeCssColor(token, fallback) { const value = getComputedStyle(document.documentElement).getPropertyValue(token).trim(); return value || fallback; }
    function chartTicks(tokenOrColor, callback) { const color = tokenOrColor.startsWith('--') ? themeCssColor(tokenOrColor, '#4A5568') : tokenOrColor; return { color, maxTicksLimit: 10, autoSkip: true, callback }; }
    function chartGrid() { return { color: themeCssColor('--border', '#1E293B'), borderDash: [3, 3] }; }
    function chartTooltip(labelFormatter) { return { enabled: true, mode: 'index', intersect: false, callbacks: { label: labelFormatter } }; }
    function chartCommonOptions() { return { responsive: true, maintainAspectRatio: false, interaction: { mode: 'index', intersect: false }, layout: { padding: { left: 10, right: 10, top: 20, bottom: 20 } }, animation: { duration: 250 } }; }
    function lineDataset(label, data, color, yAxisID) { return { label, data, borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4, yAxisID }; }
    function createCPUChart(ctx, labels, cpuData, tempData) {
      if (statsCharts.cpu) return statsCharts.cpu;
      statsCharts.cpu = new Chart(ctx, {
        type: 'line',
        data: { labels, datasets: [
          lineDataset('CPU Usage', cpuData, themeCssColor('--secondary', '#4A5568'), 'y-cpu'),
          lineDataset('Temperature', tempData, themeCssColor('--accent', '#90cff3'), 'y-temp')
        ] },
        options: Object.assign(chartCommonOptions(), {
          plugins: { tooltip: chartTooltip(context => context.dataset.label + ': ' + (context.parsed.y === null || context.parsed.y === undefined ? '—' : Number(context.parsed.y).toFixed(1)) + (context.dataset.yAxisID === 'y-temp' ? '°C' : '%')), legend: { position: 'bottom', align: 'center', labels: { usePointStyle: true, pointStyle: 'circle', boxWidth: 8, boxHeight: 8 } } },
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
          plugins: { legend: { display: false }, tooltip: chartTooltip(context => context.dataset.label + ': ' + fmtBytes(context.parsed.y) + '/s') },
          scales: {
            x: { ticks: chartTicks('--hiddenTabText', value => labels[value] || value), grid: { display: false } },
            y: { beginAtZero: true, suggestedMin: 0, suggestedMax: Math.max(1, ...datasets.flatMap(dataset => dataset.data)) * 1.1, ticks: chartTicks('--hiddenTabText', value => fmtBytes(value) + '/s'), grid: chartGrid() }
          }
        })
      });
      return statsCharts.io;
    }
    function renderCpuChart(data) {
      const stale=statsSectionStale(data,'cpu'); setStatsRendererLoading('cpu-chart',stale); if(stale)return;
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
    function statsDomKey(value, fallback = 'item') {
      const raw = String(value ?? fallback);
      return encodeURIComponent(raw).replace(/[^A-Za-z0-9_-]/g, '_') || fallback;
    }
    function renderNetwork(data) {
      const stale=statsSectionStale(data,'network'); setStatsRendererLoading('network-chart',stale); if(stale)return;
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
        const rows = interfaces.filter(meaningfulInterface).map(iface => `<tr id="network-interface-${statsDomKey(iface.name)}" data-stats-key="${escapeHtml(iface.name)}"><td><span class="interface-name">${interfaceLabel(iface.name)}</span><span class="interface-label"> (${iface.name})</span></td><td class="data-cell">${fmtBytes(iface.rxBytes)}</td><td class="data-cell">${fmtBytes(iface.txBytes)}</td></tr>`).join('') || '<tr id="network-interface-empty"><td colspan="3">No network interfaces reported.</td></tr>';
        morphLivePane(tbody, rows);
      } else {
        const totals = statsNetworkTotals(data);
        morphLivePane(tbody, `<tr id="network-interface-total" data-stats-key="total"><td><span class="interface-name">Network</span></td><td class="data-cell">${fmtBytes(totals.rx)}</td><td class="data-cell">${fmtBytes(totals.tx)}</td></tr>`);
      }
    }
    function renderDiskIo(data) {
      const stale=statsSectionStale(data,'io'); setStatsRendererLoading('io-section',stale); if(stale)return;
      const controls = document.querySelector('[data-device-controls]'), checked = new Map(Array.from(controls?.querySelectorAll('input[type="checkbox"]') || []).map(input => [input.name, input.checked])), devices = data.io?.devices || [];
      if (controls) morphLivePane(controls, devices.map(device => {
        const name = diskDisplayName(device), readName = `read-${name}`, writeName = `write-${name}`, readChecked = checked.has(readName) ? checked.get(readName) : true, writeChecked = checked.has(writeName) ? checked.get(writeName) : true;
        const key = statsDomKey(device.device || device.mount || name);
        return `<div id="io-device-${key}" class="device-control" data-io-device="${escapeHtml(name)}" data-stats-key="${escapeHtml(device.device || device.mount || name)}"><div class="device-name">${escapeHtml(name)}</div><div class="device-checkboxes"><label id="io-read-label-${key}" class="drive-checkbox"><input id="io-read-${key}" type="checkbox" name="${escapeHtml(readName)}" value="${escapeHtml(name)}" ${readChecked ? 'checked' : ''}>Read</label><label id="io-write-label-${key}" class="drive-checkbox"><input id="io-write-${key}" type="checkbox" name="${escapeHtml(writeName)}" value="${escapeHtml(name)}" ${writeChecked ? 'checked' : ''}>Write</label></div></div>`;
      }).join('') || '<div id="io-device-empty" class="io-loading"><p>No disk I/O devices reported.</p></div>');
      const colors = ['--secondary', '--accent', '--warning', '--success', '--error'];
      const datasets = devices.flatMap((device, index) => {
        const key = device.device || device.mount;
        const name = diskDisplayName(device);
        const series = statsChartState.ioSeries[key] || { read: [0], write: [0] };
        const color = themeCssColor(colors[index % colors.length], '#4A5568');
        return [
          { label: `${name} Read`, data: series.read, statsSeriesKey: key, statsIoKind: 'read', borderColor: color, backgroundColor: color, borderWidth: 2, fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4 },
          { label: `${name} Write`, data: series.write, statsSeriesKey: key, statsIoKind: 'write', borderColor: color, backgroundColor: color, borderWidth: 2, borderDash: [3, 3], fill: false, pointRadius: 0, pointHoverRadius: 0, tension: 0.4 }
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
      if (legend) morphLivePane(legend, datasets.map(dataset => `<span id="io-series-${statsDomKey(dataset.label)}" data-io-series="${escapeHtml(dataset.label)}">${escapeHtml(dataset.label)}</span>`).join(''));
    }
    const lastGoodMemory = { memory: null, swap: null };
    function renderMemory(data) {
      if (statsSectionStale(data,'memory')) { setStatsRendererLoading('memory-usage',true); return; }
      // A zero read is a dropped sample, never a real reading: hold the last known good.
      const freshMemory = data.resources?.memory || {};
      const freshSwap = data.resources?.swap || {};
      const memoryValid = Number(freshMemory.totalBytes) > 0 && Number(freshMemory.usedBytes) > 0;
      const swapValid = Number(freshSwap.totalBytes) > 0;
      const holdingInvalidRead = (!memoryValid && Boolean(lastGoodMemory.memory)) || (!swapValid && Boolean(lastGoodMemory.swap));
      setStatsRendererLoading('memory-usage',holdingInvalidRead);
      if (memoryValid) lastGoodMemory.memory = freshMemory;
      if (swapValid) lastGoodMemory.swap = freshSwap;
      const memory = lastGoodMemory.memory || freshMemory;
      const swap = lastGoodMemory.swap || freshSwap;
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
      const stale=statsSectionStale(data,'storage'); setStatsRendererLoading('disk-usage',stale); if(stale)return;
      const target = document.querySelector('[data-disk-usage-stats]');
      if (!target) return;
      morphLivePane(target, (data.storage || []).map((drive, index) => {
        const label = drive.productLabel || drive.name || 'Storage';
        const mountLine = drive.mount ? `<div class="disk-mountpoint">Mount: ${drive.mount}</div>` : '';
        const key = statsDomKey(drive.mount || drive.name || `drive-${index}`);
        return `<div id="disk-usage-${key}" class="disk-usage-item" data-stats-key="${escapeHtml(drive.mount || drive.name || `drive-${index}`)}"><div class="disk-usage-header"><div class="disk-device">${label} (${Number(drive.usagePercent || 0).toFixed(1)}%)</div>${mountLine}</div><div class="disk-usage-bar"><div class="disk-usage-fill" style="width:${drive.usagePercent || 0}%"></div></div><div class="disk-usage-details"><div>Used: ${fmtBytes(drive.usedBytes)}</div><div>Free: ${fmtBytes(drive.freeBytes)}</div><div>Total: ${fmtBytes(drive.totalBytes)}</div></div></div>`;
      }).join('') || '<div id="disk-usage-empty" class="disk-usage-loading"><p>No storage usage reported.</p></div>');
    }
    function statsAuthorizedBinding(binding) { return Boolean(binding && binding.guest === 'stats' && headerState.isAdmin && scaffoldRegistry.current(binding)); }
    function renderStatsRoster(data, projection) {
      const binding = projection?.binding, run = projection?.run;
      if (!binding || binding.guest !== 'stats' || !scaffoldRegistry.current(binding, run)) return;
      const stale=statsSectionStale(data,'keaLeases'); setStatsRendererLoading('kea-leases',stale,binding.root); if(stale)return;
      if (!headerState.isAdmin) {
        const target = binding.root.querySelector('[data-identity-roster-host="stats"] [data-identity-roster]');
        if (target && scaffoldRegistry.current(binding, run)) target.textContent = 'Network activity requires Admin Mode.';
        return;
      }
      const entries = data?.keaLeases?.entries;
      if (!Array.isArray(entries)) throw new Error('Network activity entries are unavailable');
      const roster = identityRows(entries);
      if (!scaffoldRegistry.current(binding, run)) return;
      identityState.roster = roster;
      if (!scaffoldRegistry.current(binding, run)) return;
      identityState.notes = Object.fromEntries(roster.map(row => [canonicalNetworkNoteMac(row.mac), row.note ?? '']).filter(([mac]) => mac));
      if (!scaffoldRegistry.current(binding, run)) return;
      identityState.ready.roster = true;
      renderIdentityRoster('liveness', binding);
    }
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
      const binding = scaffoldRegistry.capture('stats');
      const modal = document.querySelector('[data-note-modal]'), mac = modal?.dataset.mac || '', textarea = modal?.querySelector('[data-note-textarea]'), errorNode = modal?.querySelector('[data-note-error]'), note = textarea?.value ?? '';
      if (!statsAuthorizedBinding(binding) || !modal || !mac) return;
      try {
        const response = await fetch('/api/network/notes', { method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ mac, note }) });
        if (!response.ok) throw new Error(`Save failed (${response.status})`);
        if (!statsAuthorizedBinding(binding) || !modal.isConnected || modal.dataset.mac !== mac) return;
        const canonicalMac = canonicalNetworkNoteMac(mac);
        if (canonicalMac) identityState.notes[canonicalMac] = note;
        renderIdentityRoster('liveness', binding); closeNoteModal(); showCoronatioToast('Device note saved', 'success');
      }
      catch (error) { if (statsAuthorizedBinding(binding) && errorNode) { errorNode.textContent = String(error); errorNode.hidden = false; } }
    }
    function renderProcesses(data) {
      const stale=statsSectionStale(data,'processes'); setStatsRendererLoading('process-usage',stale); if(stale)return;
      const target = document.querySelector('[data-process-usage-list]');
      if (!target) return;
      const processes = data.processes || [];
      target.innerHTML = processes.length ? processes.map(process => `<div class="process-bar" title="Process: ${process.name}\nMemory: ${fmtBytes(process.memoryBytes)}\nCPU: ${Number(process.cpuPercent || 0).toFixed(1)}%\nInstances: ${process.processCount || 1}"><div class="process-bar-fill" style="width:${Math.max(Number(process.cpuPercent || 0), 1)}%"></div><div class="process-text-container"><span class="process-name">${process.name}</span><span class="process-usage">${Number(process.cpuPercent || 0).toFixed(1)}%</span></div></div>`).join('') : '<div class="process-usage-empty"><p>No process data available</p></div>';
    }
    function syncStatsChartsFromState(binding, context) {
      if (!binding || !context?.current()) return false;
      const cpuCanvas = binding.root.querySelector('#cpuChart'), cpu = statsCharts.cpu;
      if (cpu && cpu.canvas === cpuCanvas) {
        if (!context.current()) return false;
        cpu.data.labels = statsChartState.labels;
        cpu.data.datasets[0].data = statsChartState.cpu;
        cpu.data.datasets[1].data = statsChartState.temp;
        if (!context.current()) return false;
        cpu.update();
      }
      const networkCanvas = binding.root.querySelector('#networkChart'), network = statsCharts.network;
      if (network && network.canvas === networkCanvas) {
        if (!context.current()) return false;
        const networkMax = Math.max(1, ...statsChartState.download, ...statsChartState.upload) * 1.1;
        network.data.labels = statsChartState.labels;
        network.data.datasets[0].data = statsChartState.download;
        network.data.datasets[1].data = statsChartState.upload;
        network.options.scales.y.max = networkMax;
        network.options.scales['y-right'].max = networkMax;
        if (!context.current()) return false;
        network.update();
      }
      const ioCanvas = binding.root.querySelector('#io-chart'), io = statsCharts.io;
      if (io && io.canvas === ioCanvas) {
        if (!context.current()) return false;
        io.data.labels = statsChartState.labels;
        io.data.datasets.forEach(dataset => {
          const series = statsChartState.ioSeries[dataset.statsSeriesKey];
          if (series && ['read', 'write'].includes(dataset.statsIoKind)) dataset.data = series[dataset.statsIoKind];
        });
        io.options.scales.y.suggestedMax = Math.max(1, ...io.data.datasets.flatMap(dataset => dataset.data)) * 1.1;
        if (!context.current()) return false;
        io.update();
      }
      return context.current();
    }
    function historyTimestamp(sample, bucket) {
      const raw = optionalNumber(sample?.ts);
      if (raw !== null) return raw > 100000000000 ? raw : raw * 1000;
      const minute = optionalNumber(bucket);
      if (minute === null) return null;
      return minute > 100000000000 ? minute : (minute > 1000000000 ? minute * 1000 : minute * 60000);
    }
    function historyPoint(sample, timestamp) {
      const load = sample?.load || {}, throughput = sample?.network?.throughput || {}, diskThroughput = sample?.disk?.throughput || {}, io = {};
      (Array.isArray(sample?.disk?.io) ? sample.disk.io : []).forEach(device => {
        const key = device?.device || device?.mount;
        if (key) io[key] = { read: optionalNumber(diskThroughput.readBytesPerSecond), write: optionalNumber(diskThroughput.writeBytesPerSecond) };
      });
      return { timestamp, cpu: loadToPercent(load.one), temp: optionalNumber(sample?.temperature?.celsius), download: optionalNumber(throughput.rxBytesPerSecond), upload: optionalNumber(throughput.txBytesPerSecond), io };
    }
    function mergeStatsHistory(points, latestSample, latestTimestamp, context) {
      if (!context.current()) return false;
      const merged = new Map(points.map(point => [point.timestamp, historyPoint(point.sample, point.timestamp)]));
      statsChartState.points.forEach(point => merged.set(point.timestamp, point));
      const next = [...merged.values()].sort((left, right) => left.timestamp - right.timestamp);
      statsChartState.points = next.slice(-60);
      if (!context.current()) return false;
      if (latestSample && (statsChartState.lastStamp === null || latestTimestamp > statsChartState.lastStamp)) {
        statsChartState.lastStamp = latestTimestamp;
        const totals = statsNetworkTotals({ network: latestSample.network });
        if (totals.rx !== null) statsChartState.lastRx = totals.rx;
        if (totals.tx !== null) statsChartState.lastTx = totals.tx;
        (Array.isArray(latestSample.disk?.io) ? latestSample.disk.io : []).forEach(device => {
          const key = device?.device || device?.mount;
          if (!key) return;
          const readBytes = optionalNumber(device.readBytes), writeBytes = optionalNumber(device.writeBytes);
          if (readBytes !== null || writeBytes !== null) {
            const previous = statsChartState.lastIo[key] || {};
            statsChartState.lastIo[key] = { readBytes: readBytes ?? previous.readBytes ?? null, writeBytes: writeBytes ?? previous.writeBytes ?? null };
          }
        });
      }
      if (!context.current()) return false;
      rebuildStatsChartSeries();
      if (!context.current()) return false;
      return syncStatsChartsFromState(context.binding, context);
    }
    const statsSnapshotTimeoutMs = 5500;
    const statsHistoryAttemptedBindings = new WeakSet();
    let statsPaneBinding = null;
    const statsBindingsWatched = new WeakSet();
    function watchStatsPaneBinding(binding) {
      if (!binding) return;
      statsPaneBinding = binding;
      if (statsBindingsWatched.has(binding)) return;
      statsBindingsWatched.add(binding);
      scaffoldRegistry.onRetire(binding, () => { if (statsPaneBinding === binding) statsPaneBinding = null; });
    }
    let statsHistoryLoaded = false;
    function setStatsHistoryStatus(binding, message, state = 'stale', retry = false, hidden = false, preserveReadySlots = false) {
      if (!binding || !scaffoldRegistry.current(binding)) return;
      const status = binding.root.querySelector('[data-stats-history-status]');
      if (!status || !scaffoldReadProjectionAllowed(status, preserveReadySlots)) return;
      status.hidden = hidden;
      status.dataset.state = state;
      status.textContent = message;
      if (retry) {
        const button = document.createElement('button'); button.type = 'button'; button.dataset.statsHistoryRetry = ''; button.textContent = 'Retry history';
        status.append(document.createTextNode(' '), button);
      }
    }
    function startStatsHistory(binding) {
      if (!binding || statsHistoryLoaded || statsHistoryAttemptedBindings.has(binding) || !scaffoldRegistry.current(binding)) return Promise.resolve(false);
      statsHistoryAttemptedBindings.add(binding);
      setStatsHistoryStatus(binding, 'Loading saved chart history…', 'pending', false, false, true);
      return scaffoldRegistry.run(binding, 'stats-history', async context => {
        const history = await context.fetchJson('/api/stats/history');
        if (!context.current()) return false;
        if (!history || typeof history !== 'object' || Array.isArray(history) || history.ok === false
            || !history.tiers || !Array.isArray(history.tiers.raw) || !Array.isArray(history.tiers.minute)) {
          throw new Error('Stats history response is incomplete');
        }
        const raw = history.tiers.raw.map(sample => ({ sample, timestamp: historyTimestamp(sample) })).filter(point => point.timestamp !== null && point.sample && typeof point.sample === 'object');
        const firstRaw = raw.length ? Math.min(...raw.map(point => point.timestamp)) : Infinity;
        const minuteByTimestamp = new Map();
        history.tiers.minute.forEach(row => {
          const sample = row?.last, timestamp = historyTimestamp(sample, row?.bucket);
          if (sample && typeof sample === 'object' && timestamp !== null && timestamp < firstRaw) minuteByTimestamp.set(timestamp, sample);
        });
        const points = [...Array.from(minuteByTimestamp, ([timestamp, sample]) => ({ sample, timestamp })), ...raw].sort((left, right) => left.timestamp - right.timestamp);
        const selected = points.length <= 60 ? points : Array.from({ length: 60 }, (_, index) => points[Math.round(index * (points.length - 1) / 59)]);
        const latestPoint = points[points.length - 1];
        if (!context.current() || !mergeStatsHistory(selected, latestPoint?.sample, latestPoint?.timestamp, context)) return false;
        statsHistoryLoaded = true;
        if (!context.current()) return false;
        setStatsHistoryStatus(binding, points.length ? '' : 'No saved history yet.', 'ready', false, points.length > 0);
        return true;
      }, {
        slots: [], timeoutMs: 8000, slowMs: 750,
        onFailure: ({ current, reason }) => {
          if (!current() || !scaffoldRegistry.current(binding)) return;
          setStatsHistoryStatus(binding, `Saved chart history unavailable: ${reason || 'Read failed'}.`, 'stale', true, false, true);
        }
      });
    }
    const statsScaffoldFillers = Object.freeze({
      'stats.cpu': { present: data => statsRendererPresent(data, 'cpu', 'cpu-chart', Boolean(data?.resources?.load && ['one', 'five', 'fifteen'].every(key => Object.prototype.hasOwnProperty.call(data.resources.load, key)))), fill: data => renderCpuChart(data) },
      'stats.network': { present: data => statsRendererPresent(data, 'network', 'network-chart', Boolean(data?.network && typeof data.network === 'object' && (Array.isArray(data.network.interfaces) || (Object.prototype.hasOwnProperty.call(data.network, 'receivedBytes') && Object.prototype.hasOwnProperty.call(data.network, 'sentBytes'))))), fill: data => renderNetwork(data) },
      'stats.io': { present: data => statsRendererPresent(data, 'io', 'io-section', Boolean(data?.io && Array.isArray(data.io.devices))), fill: data => renderDiskIo(data) },
      'stats.memory': { present: data => statsRendererPresent(data, 'memory', 'memory-usage', Boolean(data?.resources?.memory)), fill: data => renderMemory(data) },
      'stats.storage': { present: data => statsRendererPresent(data, 'storage', 'disk-usage', Array.isArray(data?.storage)), fill: data => renderDiskUsage(data) },
      'stats.kea-leases': { present: data => statsRendererPresent(data, 'keaLeases', 'kea-leases', Array.isArray(data?.keaLeases?.entries)), fill: (data, projection) => renderStatsRoster(data, projection) },
      'stats.processes': { present: data => statsRendererPresent(data, 'processes', 'process-usage', Array.isArray(data?.processes)), fill: data => renderProcesses(data) }
    });
    async function readBundledStats(context) {
      watchStatsPaneBinding(context.binding);
      const data = await context.fetchJson('/api/stats');
      if (!context.current()) return false;
      if (!data || typeof data !== 'object' || Array.isArray(data) || data.ok === false) throw new Error('Stats snapshot is incomplete');
      const chartPresent = statsScaffoldFillers['stats.cpu'].present(data)
        || statsScaffoldFillers['stats.network'].present(data)
        || statsScaffoldFillers['stats.io'].present(data);
      if (chartPresent && context.current()) pushChartPoint(data, statsTimestamp(data.sampledAt));
      return data;
    }
    scaffoldRegistry.registerGuest({
      guest: 'stats',
      root: '[data-scaffold-root][data-guest-id="stats"]',
      read: readBundledStats,
      fillers: statsScaffoldFillers,
      timeoutMs: statsSnapshotTimeoutMs
    });
    async function hydrateStats() {
      const binding = scaffoldRegistry.capture('stats');
      if (!binding) return Promise.resolve(false);
      watchStatsPaneBinding(binding);
      startStatsHistory(binding);
      return scaffoldRegistry.read('stats', { readKey: 'stats-snapshot' });
    }
    document.addEventListener('click', event => {
      const button = event.target instanceof Element ? event.target.closest('[data-stats-history-retry]') : null;
      if (!button) return;
      const binding = scaffoldRegistry.capture('stats');
      if (!binding) return;
      event.preventDefault();
      button.disabled = true;
      scaffoldRegistry.retry(binding, 'stats-history').finally(() => { if (button.isConnected) button.disabled = false; });
    });
    window.addEventListener('coronatio:privileged-state-retired', () => {
      identityState.roster = []; identityState.notes = {}; identityState.ready.roster = false;
      retireTestLanIpCatalogForGuest();
      closeNoteModal();
      document.querySelectorAll('[data-pane-panel="stats"] [data-identity-roster]').forEach(target => { target.replaceChildren(); target.textContent = 'Network activity requires Admin Mode.'; });
    });
    window.addEventListener('coronatio:native-pane-reset', event => {
      const detail = event.detail || {}, pane = detail.pane;
      if (detail.id !== 'stats' || !(pane instanceof Element) || !pane.isConnected || document.querySelector('[data-pane-panel="stats"]') !== pane) return;
      if (statsPaneBinding && scaffoldRegistry.current(statsPaneBinding)) return;
      statsPaneBinding = null;
      ['cpu', 'network', 'io'].forEach(destroyStatsChart);
      resetStatsChartState(); lastGoodMemory.memory = null; lastGoodMemory.swap = null; statsHistoryLoaded = false;
    });
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
    function portalServiceFailureMessage(result, fallback = 'Service action failed') {
      const signal = result?.firstMissingSignal || result?.first_missing_signal;
      const expiredAttendanceSignals = new Set(['caduceus-attendance-refused', 'caduceus-attendance-pin-refused', 'caduceus-attendance-not-current', 'caduceus-attendance-invalid', 'caduceus-attendance-required', 'caduceus-stale-incarnation', 'caduceus-attendance-stale-incarnation']);
      const unreachableSignals = new Set(['caduceus-unreachable', 'caduceus-upstream-failed', 'caduceus-attendance-connect-refused', 'caduceus-attendance-connect-failed', 'caduceus-attendance-timeout', 'caduceus-attendance-socket-config-failed', 'caduceus-attendance-write-failed', 'caduceus-attendance-task-failed']);
      if (expiredAttendanceSignals.has(signal)) return 'Admin session expired. Enter the PIN again.';
      if (unreachableSignals.has(signal)) return 'The appliance service controller is unreachable.';
      return result?.message || result?.error || result?.output || result?.stdout || fallback;
    }
    function showPortalServiceStatus(results) {
      const modal = document.querySelector('[data-service-status-modal]'); const content = modal?.querySelector('[data-service-status-content]');
      if (content) content.textContent = results.map(result => {
        const header = `=== ${result.service || 'service'} ===`; const decoratedError = result.decoratedError || (result.error ? portalServiceFailureMessage(result, result.error) : ''); const status = decoratedError || result.output || result.message || result.stdout || result.raw || 'No status available';
        if (decoratedError) return `${header}\n⚠️ Error State:\n${status}`;
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
          const decoratedError = !response.ok || result.ok === false || result.success === false ? portalServiceFailureMessage(result, `Failed to ${action} ${service}`) : '';
          results.push({ service, ...result, decoratedError });
          if (action !== 'status') {
            if (response.ok && result.ok !== false && result.success !== false) showCoronatioToast(result.message || `Successfully ${action}ed ${service}`, 'success');
            else showCoronatioToast(portalServiceFailureMessage(result, `Failed to ${action} ${service}`), 'error');
          }
        } catch (_) {
          const unreachableMessage = 'The appliance service controller is unreachable.';
          results.push({ service, action, error: unreachableMessage, decoratedError: unreachableMessage });
          if (action !== 'status') showCoronatioToast(unreachableMessage, 'error');
        }
      }
      if (action === 'status') showPortalServiceStatus(results);
    }
    function bindPortalFragmentControls(grid) {
      grid.querySelectorAll('[data-portal-card]').forEach(card => {
        if (card.dataset.portalFragmentControlsBound === 'true') return;
        card.dataset.portalFragmentControlsBound = 'true';
        const open = () => { const url = card.dataset.portalUrl; if (url && url !== '#') window.open(url, '_blank', 'noopener,noreferrer'); };
        card.addEventListener('click', open);
        card.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); open(); } });
      });
      grid.querySelectorAll('[data-service-action]').forEach(button => { if (button.dataset.portalFragmentControlBound === 'true') return; button.dataset.portalFragmentControlBound = 'true'; button.addEventListener('click', handlePortalServiceAction); });
      grid.querySelectorAll('[data-portal-delete]').forEach(button => { if (button.dataset.portalFragmentControlBound === 'true') return; button.dataset.portalFragmentControlBound = 'true'; button.addEventListener('click', deletePortal); });
    }
    async function refreshPortalCurrentness(binding = scaffoldRegistry.capture('portals')) {
      if (!binding || binding.guest !== 'portals' || !viewportFamilyAdmitted('portals') || !scaffoldRegistry.current(binding)) return false;
      const grid = binding.root.querySelector('[data-portals-grid]');
      if (!grid) return false;
      return scaffoldRegistry.run(binding, 'portals-currentness', async context => {
        const data = await context.fetchJson('/api/portals/currentness');
        if (!context.current() || !viewportFamilyAdmitted('portals') || !binding.root.contains(grid)) return false;
        if (!data || typeof data !== 'object' || Array.isArray(data) || !data.portals || typeof data.portals !== 'object' || Array.isArray(data.portals)) throw new Error('Portal currentness response is incomplete');
        const statuses = ['up', 'down', 'partial', 'unknown'];
        for (const [name, value] of Object.entries(data.portals)) if (value !== null && value !== undefined && !statuses.includes(value)) throw new Error('Portal currentness row is malformed');
        for (const card of grid.querySelectorAll('[data-portal-card]')) {
          if (!context.current() || !viewportFamilyAdmitted('portals') || !binding.root.contains(card)) return false;
          const next = statuses.includes(data.portals[card.dataset.portalName]) ? data.portals[card.dataset.portalName] : 'unknown';
          const current = statuses.find(status => card.classList.contains(status)) || 'unknown';
          if (current !== next) { card.classList.remove(...statuses); card.classList.add(next); }
        }
        if (context.current()) { delete grid.dataset.currentnessStale; delete grid.dataset.currentnessError; }
        return true;
      }, {
        slots: [], timeoutMs: 8000, slowMs: 750, retry: false,
        onFailure: ({ current, reason }) => {
          if (!current() || !viewportFamilyAdmitted('portals') || !scaffoldRegistry.current(binding) || !binding.root.contains(grid)) return;
          grid.dataset.currentnessStale = 'true'; grid.dataset.currentnessError = reason || 'Portal currentness unavailable';
        }
      });
    }
    const portalElementsChanged = window.consumeElementsChanged({
      paneId: 'portals',
      route: '/api/portals/elements',
      target: () => document.querySelector('[data-portals-grid]'),
      afterReplace: target => bindPortalFragmentControls(target)
    });
    const statsElementsChanged = window.consumeElementsChanged({
      paneId: 'stats',
      route: '/api/stats/elements',
      target: () => document.querySelector('[data-stats-viewport]'),
      afterReplace: () => { scaffoldRegistry.rebind('stats'); hydrateStats(); }
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
          if (tabId === 'stats') { scaffoldRegistry.rebind('stats'); hydrateStats(); }
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
    const testLanIpState = { start: 100, end: 163, taken: new Map([[103, 'Media server'], [120, 'Garden tablet']]), selected: 100, page: 0, live: false, hasCatalog: false, catalogStarted: false, catalogStale: false, binding: null, cycle: 0, startedCycle: 0, reads: { boundary: { state: 'unknown' }, leases: { state: 'unknown' }, reservations: { state: 'unknown' } } };
    const testUpstreamProviders = Object.freeze({ quad9: ['9.9.9.9', '149.112.112.112'], cloudflare: ['1.1.1.1', '1.0.0.1'], google: ['8.8.8.8', '8.8.4.4'] });
    const testDhcpRows = payload => Array.isArray(payload) ? payload : (payload?.leases || payload?.reservations || payload?.data || payload?.items || []);
    const testLanIpPageSize = 32;
    const testLanIpReadRoutes = Object.freeze({ boundary: '/api/network/dhcp/boundary', leases: '/api/network/dhcp/leases', reservations: '/api/network/dhcp/reservations' });
    const testLanIpReadKeys = Object.freeze({ boundary: 'test-network-boundary', leases: 'test-network-leases', reservations: 'test-network-reservations' });
    function testLanOctet(value) { const match = String(value || '').match(/^192\.168\.123\.(\d{1,3})$/); return match ? Number(match[1]) : null; }
    function testLanIpCatalogStatus() {
      const specimen = document.querySelector('[data-test-lan-ip-calendar]'); if (!specimen) return;
      let status = specimen.querySelector('[data-test-network-catalog-status]');
      if (!status) { status = document.createElement('div'); status.dataset.testNetworkCatalogStatus = ''; status.setAttribute('role', 'status'); status.setAttribute('aria-live', 'polite'); const calendar = specimen.querySelector('.ui-lan-ip-calendar'); if (calendar) calendar.after(status); else specimen.appendChild(status); }
      status.replaceChildren();
      const summary = testLanIpState.live ? 'Current network catalog.' : testLanIpState.hasCatalog ? 'Network catalog is stale; showing the last known range with selection disabled.' : testLanIpState.catalogStarted ? 'Demonstration range only; current network catalog is unavailable or incomplete and selection is disabled.' : 'Demonstration range; waiting for current network data.';
      status.append(document.createTextNode(summary));
      for (const name of ['boundary', 'leases', 'reservations']) {
        const read = testLanIpState.reads[name] || { state: 'unknown' };
        status.append(document.createTextNode(` · ${name}: ${read.state}`));
        if (read.state === 'error') {
          if (read.reason) status.append(document.createTextNode(` (${read.reason})`));
          if (testLanIpState.binding && scaffoldRegistry.current(testLanIpState.binding)) { const retry = document.createElement('button'); retry.type = 'button'; retry.dataset.scaffoldRetry = 'test'; retry.dataset.scaffoldReadKey = testLanIpReadKeys[name]; retry.textContent = `Retry ${name}`; status.append(document.createTextNode(' '), retry); }
        }
      }
    }
    function renderTestLanIpCalendar() {
      const specimen = document.querySelector('[data-test-lan-ip-calendar]'); const grid = specimen?.querySelector('[data-lan-ip-grid]'); if (!grid) return;
      const first = testLanIpState.start + testLanIpState.page * testLanIpPageSize; const last = Math.min(testLanIpState.end, first + testLanIpPageSize - 1);
      specimen.querySelector('[data-lan-ip-range]').textContent = `${first} - ${last}`;
      const blocked = testLanIpState.catalogStarted && !testLanIpState.live;
      specimen.querySelector('[data-lan-ip-prev]').disabled = blocked || first <= testLanIpState.start;
      specimen.querySelector('[data-lan-ip-next]').disabled = blocked || last >= testLanIpState.end;
      const html = Array.from({ length: Math.max(0, last - first + 1) }, (_, offset) => {
        const octet = first + offset, holder = testLanIpState.taken.get(octet), label = holder ? `192.168.123.${octet}, held by ${holder}` : `Select 192.168.123.${octet}`;
        return `<button type="button" class="ui-lan-ip-calendar__octet" data-lan-ip-octet="${octet}" data-scaffold-key="lan-ip-octet-${octet}" ${holder || blocked ? 'disabled' : ''} role="gridcell" aria-pressed="${testLanIpState.selected === octet}" aria-label="${escapeHtml(label)}"${holder ? ` title="${escapeHtml(holder)}"` : ''}>${octet}</button>`;
      }).join('');
      morphLivePane(grid, html);
      const selected = specimen.querySelector('[data-lan-ip-readout]');
      if (selected) selected.textContent = testLanIpState.selected === null ? (testLanIpState.catalogStarted && !testLanIpState.live ? 'No address selected; demonstration range only.' : 'No available address.') : `Selected address: 192.168.123.${testLanIpState.selected}${testLanIpState.live ? '' : testLanIpState.hasCatalog ? ' (last known; stale)' : ' (demonstration range)'}`;
      testLanIpCatalogStatus();
    }
    function retireTestLanIpCatalogForGuest() {
      testLanIpState.cycle += 1;
      testLanIpState.start = 100; testLanIpState.end = 163; testLanIpState.taken = new Map(); testLanIpState.selected = null; testLanIpState.page = 0;
      testLanIpState.live = false; testLanIpState.hasCatalog = false; testLanIpState.catalogStale = false; testLanIpState.catalogStarted = true; testLanIpState.binding = null;
      testLanIpState.reads = { boundary: { state: 'error', reason: 'Admin Mode required' }, leases: { state: 'error', reason: 'Admin Mode required' }, reservations: { state: 'error', reason: 'Admin Mode required' } };
      scaffoldRegistry.retire('test');
      renderTestLanIpCalendar();
    }
    function testLanIpCurrent(binding, context, cycle) { return Boolean(context.current() && scaffoldRegistry.current(binding) && viewportFamilyAdmitted('test') && headerState.isAdmin && testLanIpState.cycle === cycle); }
    function testLanIpEnvelopeRows(payload) {
      if (!payload || payload.ok !== true || !payload.payload || payload.payload.ok !== true || !Array.isArray(payload.payload.result)) throw new Error('Network catalog response is incomplete');
      return payload.payload.result;
    }
    function testLanIpProjectRead(name, rows) {
      if (name === 'boundary') {
        if (rows.length !== 1 || !rows[0] || typeof rows[0] !== 'object' || Array.isArray(rows[0])) throw new Error('Network boundary is unavailable');
        const start = testLanOctet(rows[0].start), end = testLanOctet(rows[0].end);
        if (!Number.isInteger(start) || !Number.isInteger(end) || start > end) throw new Error('Network boundary is invalid');
        return { start, end };
      }
      for (const row of rows) {
        if (!row || typeof row !== 'object' || Array.isArray(row)) throw new Error(`DHCP ${name} row is invalid`);
        const octet = testLanOctet(row.ip || row['ip-address'] || row.ipAddress);
        if (octet === null) throw new Error(`DHCP ${name} row has no valid address`);
      }
      return testDhcpRows(rows);
    }
    function applyTestLanIpCatalog(binding, context, cycle) {
      if (!testLanIpCurrent(binding, context, cycle)) return false;
      const reads = testLanIpState.reads;
      if (!['boundary', 'leases', 'reservations'].every(name => reads[name]?.state === 'ready')) return true;
      const boundary = reads.boundary.value, leaseRows = reads.leases.value, reservationRows = reads.reservations.value;
      const taken = new Map();
      [...leaseRows, ...reservationRows].forEach(row => { const octet = testLanOctet(row.ip || row['ip-address'] || row.ipAddress); const holder = String(row.hostname || row.device_name || row.name || 'Reserved device'); if (octet !== null) taken.set(octet, holder); });
      if (!testLanIpCurrent(binding, context, cycle)) return false;
      const previousSelected = testLanIpState.selected, previousPage = testLanIpState.page;
      const selectedStillFree = Number.isInteger(previousSelected) && previousSelected >= boundary.start && previousSelected <= boundary.end && !taken.has(previousSelected);
      const firstFree = Array.from({ length: boundary.end - boundary.start + 1 }, (_, index) => boundary.start + index).find(octet => !taken.has(octet));
      testLanIpState.start = boundary.start; testLanIpState.end = boundary.end; testLanIpState.taken = taken;
      testLanIpState.selected = selectedStillFree ? previousSelected : (firstFree ?? null);
      const lastPage = Math.max(0, Math.ceil((boundary.end - boundary.start + 1) / testLanIpPageSize) - 1);
      testLanIpState.page = selectedStillFree ? Math.min(lastPage, Math.max(0, previousPage)) : (testLanIpState.selected === null ? Math.min(lastPage, Math.max(0, previousPage)) : Math.floor((testLanIpState.selected - boundary.start) / testLanIpPageSize));
      testLanIpState.hasCatalog = true; testLanIpState.catalogStale = false; testLanIpState.live = true;
      renderTestLanIpCalendar();
      return true;
    }
    function runTestLanIpRead(binding, name, cycle) {
      const key = testLanIpReadKeys[name];
      return scaffoldRegistry.run(binding, key, async context => {
        if (!headerState.isAdmin) { if (testLanIpState.binding === binding) retireTestLanIpCatalogForGuest(); return false; }
        if (!context.current() || !scaffoldRegistry.current(binding) || !viewportFamilyAdmitted('test') || testLanIpState.cycle !== cycle) return false;
        if (testLanIpState.startedCycle !== cycle) {
          testLanIpState.startedCycle = cycle; testLanIpState.catalogStarted = true; testLanIpState.catalogStale = true; testLanIpState.live = false;
          testLanIpState.reads = { boundary: { state: 'waiting' }, leases: { state: 'waiting' }, reservations: { state: 'waiting' } };
        }
        testLanIpState.reads[name] = { state: 'pending' }; testLanIpState.catalogStale = true; testLanIpState.live = false;
        renderTestLanIpCalendar();
        const payload = await context.fetchJson(testLanIpReadRoutes[name]);
        if (!testLanIpCurrent(binding, context, cycle)) return false;
        const value = testLanIpProjectRead(name, testLanIpEnvelopeRows(payload));
        if (!testLanIpCurrent(binding, context, cycle)) return false;
        testLanIpState.reads[name] = { state: 'ready', value };
        if (!applyTestLanIpCatalog(binding, context, cycle)) return false;
        if (!testLanIpState.live) renderTestLanIpCalendar();
        return true;
      }, {
        slots: [], timeoutMs: 8000, slowMs: 750, supersede: false,
        onFailure: ({ current, reason }) => {
          if (!current() || !scaffoldRegistry.current(binding) || !viewportFamilyAdmitted('test') || testLanIpState.cycle !== cycle) return;
          testLanIpState.reads[name] = { state: 'error', reason: reason || 'Read failed' }; testLanIpState.catalogStale = true; testLanIpState.live = false;
          renderTestLanIpCalendar();
        }
      });
    }
    async function hydrateTestNetworkCatalog() {
      if (!viewportFamilyAdmitted('test')) return false;
      if (!headerState.isAdmin) { retireTestLanIpCatalogForGuest(); return false; }
      const binding = scaffoldRegistry.capture('test'); if (!binding) return false;
      if (testLanIpState.binding !== binding) {
        const previous = testLanIpState.binding;
        if (previous) scaffoldRegistry.retire(previous, 'Test catalog binding replaced');
        testLanIpState.binding = binding;
        scaffoldRegistry.onRetire(binding, () => {
          if (testLanIpState.binding !== binding) return;
          testLanIpState.binding = null; testLanIpState.live = false; testLanIpState.catalogStale = true; testLanIpState.catalogStarted = true;
          renderTestLanIpCalendar();
        });
      }
      const keys = Object.values(testLanIpReadKeys);
      if (!keys.some(key => binding.runs.has(key))) testLanIpState.cycle += 1;
      const cycle = testLanIpState.cycle;
      const results = await Promise.all(['boundary', 'leases', 'reservations'].map(name => runTestLanIpRead(binding, name, cycle)));
      return results.every(result => result !== false);
    }
    const now = new Date();
    const testMonthlyCalendarState = { displayedYear: now.getFullYear(), displayedMonth: now.getMonth(), selected: now };
    function renderTestMonthlyCalendar() { const specimen = document.querySelector('[data-ui-calendar-monthly]'); const grid = specimen?.querySelector('[data-ui-calendar-monthly-grid]'); if (!grid) return; const { displayedYear: year, displayedMonth: month, selected } = testMonthlyCalendarState, offset = (new Date(year, month, 1).getDay() + 6) % 7, days = new Date(year, month + 1, 0).getDate(), selectedInDisplayedMonth = selected.getFullYear() === year && selected.getMonth() === month; specimen.querySelector('[data-ui-calendar-monthly-month]').textContent = new Date(year, month, 1).toLocaleDateString(undefined, { month: 'long', year: 'numeric' }); grid.replaceChildren(...Array.from({ length: offset + days + ((7 - ((offset + days) % 7)) % 7) }, (_, index) => { if (index < offset || index >= offset + days) { const blank = document.createElement('span'); blank.className = 'ui-calendar-monthly__blank'; return blank; } const day = index - offset + 1, button = document.createElement('button'); button.type = 'button'; button.className = 'ui-calendar-monthly__day'; button.dataset.uiCalendarMonthlyDay = String(day); button.textContent = String(day); const isSelected = selectedInDisplayedMonth && day === selected.getDate(); button.setAttribute('aria-pressed', String(isSelected)); button.classList.toggle('is-selected', isSelected); return button; })); const output = specimen.querySelector('[data-ui-calendar-monthly-output]'); if (output) output.textContent = `Selected date: ${selected.toLocaleDateString(undefined, { dateStyle: 'long' })}`; }
    function renderTestUpstreamReadout(specimen) { const addresses = [...specimen.querySelectorAll('[data-upstream-provider]:checked')].flatMap(box => testUpstreamProviders[box.value] || []); const custom = specimen.querySelector('[data-upstream-custom-addresses]')?.value.split(',').map(value => value.trim()).filter(Boolean) || []; const dot = Boolean(specimen.querySelector('[data-upstream-dot]')?.checked); specimen.querySelector('[data-upstream-readout]').textContent = `Upstreams: [${[...addresses, ...custom].join(', ')}] · DoT: ${dot ? 'on' : 'off'}`; }

    document.body.addEventListener('click', event => {
      const toastSpawn = event.target.closest('[data-coronatio-toast-spawn]');
      if (toastSpawn) { showCoronatioToast(toastSpawn.dataset.toastMessage || 'Notification', toastSpawn.dataset.toastVariant || 'info'); return; }
      const sourceCurrencyServiceToggle = event.target.closest('[data-source-currency-update-service-toggle]');
      if (sourceCurrencyServiceToggle) { event.preventDefault(); void startSourceCurrencyServiceToggle(sourceCurrencyServiceToggle); return; }
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
      const lanIpOctet = event.target.closest('[data-lan-ip-octet]'); if (lanIpOctet && !lanIpOctet.disabled && (!testLanIpState.catalogStarted || testLanIpState.live)) { testLanIpState.selected = Number(lanIpOctet.dataset.lanIpOctet); renderTestLanIpCalendar(); return; }
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
