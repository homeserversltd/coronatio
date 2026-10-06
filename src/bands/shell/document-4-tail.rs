fn shell_document_4_tail() -> &'static str {
    r####"    // modal open/close/backdrop clicks handled by the delegated body click listener above (survives HTMX swaps)
    function dismissCoronatioToast(toast) {
      if (!toast || toast.classList.contains('toast-exit')) return;
      window.clearTimeout(Number(toast.dataset.toastTimer || 0));
      toast.classList.add('toast-exit');
    }
    function startCoronatioToastTimer(toast, duration) {
      window.clearTimeout(Number(toast.dataset.toastTimer || 0));
      toast.dataset.toastRemaining = String(duration);
      toast.dataset.toastStartedAt = String(Date.now());
      toast.dataset.toastTimer = String(window.setTimeout(() => dismissCoronatioToast(toast), duration));
    }
    // Fragment requests use data-coronatio-toast-request textContent, data-toast-kind,
    // and optional data-toast-timeout-ms inside data-coronatio-toast-stack; chrome stays crown-owned.
    function showCoronatioToast(message, variant = 'info', timeout = 3000) {
      const stack = document.querySelector('[data-coronatio-toast-stack]');
      if (!stack || !message) return;
      const allowed = ['info', 'success', 'warning', 'error'];
      const resolvedVariant = allowed.includes(variant) ? variant : 'info';
      const duration = Number.isInteger(timeout) && timeout >= 1 && timeout <= 2147483647 ? timeout : 3000;
      const icons = { info: 'ℹ️', success: '✅', warning: '⚠️', error: '❌' };
      const toast = document.createElement('div');
      toast.className = `toast ${resolvedVariant}`;
      toast.dataset.coronatioToast = '';
      toast.setAttribute('role', 'alert');
      const icon = document.createElement('span'); icon.className = 'toast-icon'; icon.setAttribute('aria-hidden', 'true'); icon.textContent = icons[resolvedVariant];
      const text = document.createElement('span'); text.className = 'toast-message'; text.textContent = String(message);
      toast.append(icon, text); stack.appendChild(toast); startCoronatioToastTimer(toast, duration);
    }
    // UX-MIGRATION-SLICE-09A: delegated so these bindings survive Caduceus HTMX card swaps.
    const adminActionLabels = Object.freeze({
      'hard-drive-test': 'Hard Drive Test', update: 'Update', restart: 'Restart', shutdown: 'Shutdown',
      'restart-website': 'Restart Website', 'view-logs': 'View Logs'
    });
    function adminActionToast(action, success) {
      const label = adminActionLabels[action] || 'System action';
      if (!success) return `${label} could not be started.`;
      return action === 'view-logs' ? 'Logs opened.' : `${label} initiated.`;
    }
    function restoreAdminActionControls() {
      document.querySelectorAll('[data-admin-action-id]').forEach(button => {
        button.disabled = false;
        if (button.dataset.adminActionOriginal) {
          button.innerHTML = button.dataset.adminActionOriginal;
          delete button.dataset.adminActionOriginal;
        }
      });
    }
    function restoreAdminToggle(toggle) {
      const card = toggle?.closest?.('[data-service-card]');
      const spinner = card?.querySelector('[data-admin-toggle-spinner]');
      spinner?.remove();
      const control = card?.querySelector('.ui-toggle');
      if (control) control.hidden = false;
      card?.removeAttribute('aria-busy');
    }
    document.body.addEventListener('htmx:beforeRequest', event => {
      const source = event.detail?.elt;
      if (!(source instanceof Element)) return;
      const toggle = source.closest('[data-service-card] .ui-toggle__input');
      if (toggle) {
        const card = toggle.closest('[data-service-card]');
        const control = card?.querySelector('.ui-toggle');
        if (!card || !control) return;
        card.setAttribute('aria-busy', 'true');
        control.hidden = true;
        const spinner = document.createElement('span');
        spinner.className = 'loading-spinner small';
        spinner.dataset.adminToggleSpinner = '';
        spinner.setAttribute('role', 'progressbar');
        spinner.setAttribute('aria-label', `Updating ${card.querySelector('h3')?.textContent || 'service'}`);
        control.after(spinner);
        return;
      }
      const actionButton = source.closest('[data-admin-action-id]');
      if (!actionButton) return;
      document.querySelectorAll('[data-admin-action-id]').forEach(button => {
        button.disabled = true;
        if (!button.dataset.adminActionOriginal) button.dataset.adminActionOriginal = button.innerHTML;
      });
      actionButton.innerHTML = '<span class="loading-spinner small" role="progressbar" aria-label="Starting action"></span><span>Starting...</span>';
    });
    document.body.addEventListener('htmx:afterSettle', event => {
      document.querySelectorAll('[data-coronatio-toast-stack] [data-coronatio-toast-request]').forEach(request => {
        const message = request.textContent;
        const variant = request.getAttribute('data-toast-kind');
        const timeout = Number(request.getAttribute('data-toast-timeout-ms'));
        request.remove();
        showCoronatioToast(message, variant, timeout);
      });
      const target = event.detail?.target;
      if (!(target instanceof Element)) return;
      const actionResult = target.matches('[data-admin-action-result]') ? target.querySelector('[data-admin-action-result-fragment]') : null;
      if (actionResult) {
        const action = actionResult.dataset.adminActionResultFragment || '';
        showCoronatioToast(adminActionToast(action, actionResult.classList.contains('success')), actionResult.classList.contains('success') ? 'success' : 'error');
        target.replaceChildren(); // OG result grammar is toast feedback, never a durable action-success panel.
      }
      const serviceCard = target.matches('[data-service-card]') ? target : target.closest('[data-service-card]');
      const mutation = serviceCard?.querySelector('[data-admin-mutation-result]');
      if (mutation) {
        const label = serviceCard.querySelector('h3')?.textContent || 'Service';
        const success = mutation.classList.contains('success');
        showCoronatioToast(success ? `${label} change initiated; state re-read.` : `${label} could not be changed.`, success ? 'success' : 'error');
        mutation.remove();
      }
    });
    document.body.addEventListener('htmx:afterRequest', event => {
      const source = event.detail?.elt;
      if (source instanceof Element && source.closest('[data-admin-action-id]')) restoreAdminActionControls();
      if (source instanceof Element && source.closest('[data-service-card] .ui-toggle__input')) restoreAdminToggle(source);
    });
    document.body.addEventListener('htmx:responseError', event => {
      const source = event.detail?.elt;
      if (source instanceof Element && source.closest('[data-admin-action-id]')) { restoreAdminActionControls(); showCoronatioToast('System action could not be started.', 'error'); }
      if (source instanceof Element && source.closest('[data-service-card] .ui-toggle__input')) { restoreAdminToggle(source); showCoronatioToast('Service change could not be started.', 'error'); }
    });
    function toggleLoadingSpinnerDemo(loadingToggle) {
      const specimen = loadingToggle.closest('[data-loading-spinner-catalog]'); const frame = specimen?.querySelector('[data-loading-spinner-frame]'); const result = specimen?.querySelector('[data-loading-spinner-result]'); if (!frame || !result) return;
      const loading = frame.dataset.loadingSpinnerState !== 'loaded'; frame.dataset.loadingSpinnerState = loading ? 'loaded' : 'loading'; loadingToggle.setAttribute('aria-pressed', String(!loading)); loadingToggle.textContent = loading ? 'Show loading state' : 'Show loaded state'; result.textContent = loading ? 'Loaded state active' : 'Loading state active'; frame.innerHTML = loading ? '<p><strong>Network data ready</strong></p>' : '<div class="network-loading"><div class="loading-spinner medium" role="progressbar" aria-label="Loading network data"></div><p>Loading network data...</p></div>';
    }
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
        const themeProperty = slider.dataset.themeTokenSlider;
        const row = slider.closest('[data-theme-token-control]');
        const unit = row?.dataset.themeTokenUnit || '';
        const output = document.querySelector(`[data-theme-token-output="${themeProperty}"]`);
        const apply = () => {
          const value = `${slider.value}${unit}`;
          root.style.setProperty(themeProperty, value);
          if (output) output.textContent = value;
          row?.setAttribute('data-theme-token-current', value);
        };
        slider.addEventListener('input', apply);
        apply();
      });
    }
    const hestiaPlatformDetails = Object.freeze({
      windows: { label: 'Windows', filename: 'homeserver-house-ca-windows.cer', steps: ['Open the downloaded certificate and choose Install Certificate.', 'Install for the Local Machine, then place it in Trusted Root Certification Authorities.', 'Restart open browsers after the import.'] },
      android: { label: 'Android', filename: 'homeserver-house-ca-android.crt', steps: ['Open Settings, then Security or Encryption & credentials.', 'Choose Install a certificate, then CA certificate, and select the downloaded file.', 'Android may display a network-monitoring warning for any user-installed CA.'] },
      chromeos: { label: 'ChromeOS', filename: 'homeserver-house-ca-chromeos.crt', steps: ['Open chrome://settings/certificates and select Authorities.', 'Choose Import, select the downloaded file, and allow it to identify websites.', 'Restart open browser windows after the import.'] },
      linux: { label: 'Linux', filename: 'homeserver-house-ca-linux.crt', steps: ['Copy the file to /usr/local/share/ca-certificates/.', 'Run sudo update-ca-certificates, then restart open browsers.', 'Firefox can use its own store: Settings → Privacy & Security → Certificates → View Certificates → Authorities → Import. Chromium normally uses the system store.'] },
      macos: { label: 'macOS', filename: 'homeserver-house-ca-macos.crt', steps: ['Open Keychain Access and import the file into the System keychain.', 'Open the certificate, expand Trust, and choose Always Trust.', 'Approve the system prompt, close the certificate, and restart open browsers.'] }
    });
    function detectHestiaPlatform() {
      const value = `${navigator.userAgentData?.platform || ''} ${navigator.platform || ''} ${navigator.userAgent || ''}`.toLowerCase();
      if (value.includes('android')) return 'android';
      if (value.includes('cros')) return 'chromeos';
      if (value.includes('win')) return 'windows';
      if (value.includes('mac')) return 'macos';
      return 'linux';
    }
    function openHestiaCertificateModal() {
      const backdrop = document.createElement('div');
      backdrop.className = 'modal-backdrop manager-modal-backdrop hestia-certificate-backdrop';
      backdrop.dataset.hestiaCertificateModal = '';
      const options = Object.entries(hestiaPlatformDetails).map(([value, detail]) => `<option value="${value}">${detail.label}</option>`).join('');
      backdrop.innerHTML = `<section class="modal modal-window manager-modal hestia-certificate-modal" role="dialog" aria-modal="true" aria-labelledby="hestia-certificate-title"><button type="button" class="modal-close" data-hestia-certificate-close aria-label="Close certificate window">×</button><h2 id="hestia-certificate-title">Install Household Certificate</h2><div class="modal-body"><p class="hestia-certificate-promise"><strong>Install once for this household root ring.</strong> Future service certificates beneath it need no new bundle.</p><label for="hestia-certificate-platform">This device</label><select id="hestia-certificate-platform" class="ui-input ui-input--medium" data-hestia-certificate-platform>${options}</select><ol data-hestia-certificate-steps></ol><p class="hestia-browser-note">The file contains public trust material only. Firefox may use its own certificate store; Chromium usually follows the operating system store.</p></div><div class="modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-hestia-certificate-close>Cancel</button><a class="ui-button ui-button--primary ui-button--small" data-hestia-certificate-download>Download Certificate</a></div></section>`;
      const select = backdrop.querySelector('[data-hestia-certificate-platform]');
      const download = backdrop.querySelector('[data-hestia-certificate-download]');
      const steps = backdrop.querySelector('[data-hestia-certificate-steps]');
      const render = () => { const platform = hestiaPlatformDetails[select.value] ? select.value : 'linux'; const detail = hestiaPlatformDetails[platform]; steps.innerHTML = detail.steps.map(step => `<li>${step}</li>`).join(''); download.href = `/api/admin/download-root-crt?platform=${encodeURIComponent(platform)}`; download.download = detail.filename; };
      select.value = detectHestiaPlatform(); select.addEventListener('change', render); render();
      const close = () => backdrop.remove(); backdrop.addEventListener('click', event => { if (event.target === backdrop || event.target.closest('[data-hestia-certificate-close]')) close(); }); document.body.appendChild(backdrop); download.focus();
    }
    document.body.addEventListener('click', event => { const certificate = event.target.closest('[data-hestia-certificate-open]'); if (certificate) { event.preventDefault(); openHestiaCertificateModal(); } });
    // Key Management is carried by the attended Crown-to-Caduceus membrane.
    function managerModal(kind, title, body, confirmLabel = 'Continue') {
      const backdrop = document.createElement('div'); backdrop.className = 'modal-backdrop manager-modal-backdrop'; backdrop.dataset.managerModal = kind;
      backdrop.innerHTML = `<section class="modal modal-window manager-modal" role="dialog" aria-modal="true" aria-labelledby="manager-modal-title"><button type="button" class="modal-close" data-manager-close aria-label="Close modal">×</button><h2 id="manager-modal-title">${title}</h2><div class="modal-body">${body}</div><p class="manager-route-state" data-manager-route-state aria-live="polite"></p><div class="modal-actions"><button type="button" class="ui-button ui-button--secondary ui-button--small" data-manager-close>Cancel</button><button type="button" class="ui-button ui-button--primary ui-button--small" data-manager-confirm disabled>${confirmLabel}</button></div></section>`;
      const close = () => backdrop.remove(); backdrop.addEventListener('click', event => { if (event.target === backdrop || event.target.closest('[data-manager-close]')) close(); }); document.body.appendChild(backdrop);
      return backdrop;
    }
    function keymanReceiptText(result) {
      const receipt = result?.receipt || result || {};
      const family = result?.receiptFamily || receipt?.receiptFamily || receipt?.receipt_family || 'caduceus.keyman.door.v1';
      const signal = result?.firstMissingSignal || receipt?.firstMissingSignal || receipt?.first_missing_signal || (result?.ok ? 'none' : 'caduceus-http-not-ok');
      return `${result?.ok ? 'Door receipt received.' : 'Door receipt returned without completion.'} ${family} · ${signal}`;
    }
    async function submitKeyman(modal, kind) {
      const route = kind === 'create-key' ? '/api/caduceus/keyman/create-key' : kind === 'update-key' ? '/api/caduceus/keyman/update-key' : '/api/caduceus/keyman/admin-password';
      const value = selector => modal.querySelector(selector)?.value || '';
      const planned = Boolean(modal.querySelector('[data-manager-planned]')?.checked);
      const payload = kind === 'create-key'
        ? { target: value('[data-manager-target]'), strategy: value('[data-manager-strategy]'), password: value('[data-manager-password]'), planned }
        : kind === 'update-key'
          ? { device: value('[data-manager-device]'), strategy: value('[data-manager-strategy]'), currentPassword: value('[data-manager-current-password]'), planned }
          : { oldPassword: value('[data-manager-old-password]'), newPassword: value('[data-manager-new-password]'), planned };
      const state = modal.querySelector('[data-manager-route-state]'); const confirm = modal.querySelector('[data-manager-confirm]');
      confirm.disabled = true; state.textContent = 'Sending to the appliance…';
      try {
        const response = await fetch(route, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
        const result = await response.json().catch(() => ({ ok: false, firstMissingSignal: 'caduceus-invalid-receipt' }));
        state.textContent = keymanReceiptText(result);
      } catch (_) {
        state.textContent = 'Door receipt returned without completion. caduceus.keyman.door.v1 · caduceus-unreachable';
      } finally {
        modal.querySelectorAll('input[type="password"]').forEach(input => { input.value = ''; });
        confirm.disabled = false;
      }
    }
    function managerKeyModal(kind) {
      const fields = kind === 'create-key'
        ? `<p>Choose the target, strategy, and password for a new vault/master key.</p><label>Target<select class="ui-input ui-input--medium" data-manager-target><option value="">Choose target</option><option value="external">External Device(s)</option><option value="vault">System Vault</option><option value="both">Both</option></select></label><label>Strategy<select class="ui-input ui-input--medium" data-manager-strategy><option value="safe_rotation">Safe Key Rotation</option><option value="replace_primary">Replace Primary Key</option><option value="flexible_addition">Add New Key</option></select></label><label>Password<input class="ui-input ui-input--medium" type="password" autocomplete="new-password" data-manager-password></label>`
        : kind === 'update-key'
          ? `<p>Apply the current NAS key from the vault to the selected encrypted drive.</p><label>Device<input class="ui-input ui-input--medium" type="text" placeholder="/dev/sdX" data-manager-device></label><label>Strategy<select class="ui-input ui-input--medium" data-manager-strategy><option value="safe_rotation">Safe Key Rotation</option><option value="replace_primary">Replace Primary Key</option><option value="flexible_addition">Flexible Key Addition</option></select></label><label>Current password<input class="ui-input ui-input--medium" type="password" autocomplete="current-password" data-manager-current-password></label>`
          : `<p>Changing the admin password also updates the system administrator access.</p><label>Current Admin Password<input class="ui-input ui-input--medium" type="password" autocomplete="current-password" data-manager-old-password></label><label>New Admin Password<input class="ui-input ui-input--medium" type="password" autocomplete="new-password" data-manager-new-password></label>`;
      const plan = `<label class="manager-plan"><input type="checkbox" data-manager-planned checked> Plan only (no change is made)</label><p data-manager-validation aria-live="polite">Complete every required field.</p>`;
      const modal = managerModal(kind, kind === 'create-key' ? 'Create New Key' : kind === 'update-key' ? 'Update Key on Drive' : 'Admin Password', `${fields}${plan}`, kind === 'create-key' ? 'Create Key' : kind === 'update-key' ? 'Update Key' : 'Update Password');
      const required = kind === 'create-key' ? ['[data-manager-target]', '[data-manager-strategy]', '[data-manager-password]'] : kind === 'update-key' ? ['[data-manager-device]', '[data-manager-strategy]', '[data-manager-current-password]'] : ['[data-manager-old-password]', '[data-manager-new-password]'];
      const validate = () => { const valid = required.every(selector => valueFor(selector)); modal.querySelector('[data-manager-confirm]').disabled = !valid; modal.querySelector('[data-manager-validation]').textContent = valid ? 'Ready to send to the appliance.' : 'Complete every required field.'; };
      const valueFor = selector => (modal.querySelector(selector)?.value || '').trim();
      modal.querySelectorAll('input,select').forEach(field => field.addEventListener('input', validate));
      modal.querySelector('[data-manager-confirm]').addEventListener('click', () => submitKeyman(modal, kind));
    }
    function managerGuideModal() {
      const guide = `<section><h4>🛡 It is Strongly Recommended to use the defaults:</h4><p>Creating a new key with the default settings will replace the onboard service suite key, and set both the vault and nas keys to use the password you provide. Providing you access to your home server via the password that came with the device, and the password you have set.</p></section><section><h4>ⓘ Understanding Key Operations:</h4><p><strong>Create New Key:</strong></p><p>This is used to generate and implement a new primary encryption key for the vault and/or NAS drives. This sets the sole <code>service_suite.key</code> and <code>nas.key</code> files stored in your vault. These are set the same unless specified otherwise by performing other than the default settings. You can add new keys, replace all keys with your new password alone, or add inplace a single slot your prefered password; this is the recommended path.</p><p><strong>Update Key on Drive:</strong></p><p>While the &quot;Create New Key&quot; operation (especially with default settings) aims to update the vault and all currently managed/attached NAS drives with the new Service Suite Key, this &quot;Update Key on Drive&quot; function is primarily for:</p><p>Applying the current <code>nas.key</code> (from the vault) to encrypted drives that were not attached, unlocked, or managed by the system during the initial &quot;Create New Key&quot; process.</p><p>Synchronizing newly introduced encrypted drives with your system's existing NAS key.</p><p>If &quot;Create New Key&quot; (using defaults) has just successfully updated all relevant drives, this step might not be immediately necessary for those drives. However, it remains essential for managing keys on drives added or reconnected later. This function uses the <code>nas.key</code> file stored in your vault to add/update the decryption key on the selected drive, ensuring consistent access.</p></section><section class="warning-section"><h4>⚠ Critical Warning:</h4><p>If you change the vault's password or a primary NAS encryption passphrase without correctly updating the key slots on all associated drives, those drives may become inaccessible. This could lead to data loss or require complex manual recovery procedures. Always verify changes and ensure drive keys are updated. Home server can only unlock your drive if the keys it has stored work on your drive. If you change the keys, you must update the keys on your drive.</p></section>`;
      const modal = managerModal('key-guide', 'Key Management Guide', guide, 'Close'); modal.querySelector('[data-manager-confirm]').disabled = false; modal.querySelector('[data-manager-confirm]').addEventListener('click', () => modal.remove());
    }
    function escapeDiskHtml(value) { const node = document.createElement('span'); node.textContent = String(value); return node.innerHTML; }
    const adminNasSetupRequests = new Set();
    let adminNasSetupInFlight = false;
    function diskCensusDevices(payload) { return Array.isArray(payload?.devices) ? payload.devices : []; }
    function diskCensusDeviceName(device) { return typeof device?.name === 'string' ? device.name : ''; }
    function diskDisplayValue(value) { if (value === null || value === undefined || value === '') return '—'; return typeof value === 'object' ? JSON.stringify(value) : String(value); }
    function diskNameCanNormalize(name) { const bare = name.startsWith('/dev/') ? name.slice(5) : name; return bare !== '.' && bare !== '..' && /^[A-Za-z0-9_.-]+$/.test(bare); }
    function diskRowMounted(device) { return (typeof device?.mountpoint === 'string' && device.mountpoint.length > 0) || device?.mounted === true || device?.isMounted === true; }
    function bareDiskName(name) { return typeof name === 'string' ? (name.startsWith('/dev/') ? name.slice(5) : name) : ''; }
    function diskIdentity(name) { return bareDiskName(name); }
    function mountedNameSharesParent(mounted, device) {
      const mountedName = diskIdentity(mounted);
      const parentName = diskIdentity(diskCensusDeviceName(device));
      if (!mountedName || !parentName) return false;
      if (mountedName === parentName) return true;
      const partitionName = diskIdentity(typeof device?.partition === 'string' ? device.partition : '');
      if (partitionName && mountedName === partitionName) return true;
      if (!mountedName.startsWith(parentName)) return false;
      const suffix = mountedName.slice(parentName.length);
      const digits = suffix.startsWith('p') ? suffix.slice(1) : suffix;
      return digits.length > 0 && [...digits].every(digit => digit >= '0' && digit <= '9');
    }
    function diskMetadataProtected(device) {
      const metadata = [device, device?.device, device?.metadata, device?.partitionMetadata].filter(value => value && typeof value === 'object');
      const protectedFlags = ['protected', 'isProtected', 'is_protected', 'system', 'isSystem', 'is_system', 'isSystemDevice', 'isSystemDisk', 'isRoot', 'is_root', 'root', 'rootDevice', 'root_device', 'isVault', 'is_vault', 'vault', 'vaultDevice', 'vault_device', 'isBoot', 'is_boot', 'boot', 'bootDevice', 'boot_device'];
      if (metadata.some(item => protectedFlags.some(flag => item[flag] === true))) return true;
      const identities = metadata.flatMap(item => ['name', 'label', 'path', 'device', 'role', 'id', 'identity'].map(key => item[key]));
      const text = [device?.name, device?.label, device?.device, device?.path, ...identities]
        .filter(value => typeof value === 'string').join(' ').toLowerCase();
      if (/(^|[^a-z0-9])(system|root|vault|boot|efi)([^a-z0-9]|$)/.test(text) || /homeserver-(primary|backup)-nas/.test(text) || /(^|[^a-z0-9])nas([^a-z0-9]|$)/.test(String(device?.label || '').toLowerCase())) return true;
      const mapper = device?.encryption?.mapper;
      return typeof mapper === 'string' && mapper.trim().length > 0;
    }
    function protectedDiskParents(devices, owner) {
      const protectedParents = new Set();
      for (const device of devices) {
        const name = diskCensusDeviceName(device);
        if (!name) continue;
        if (diskRowMounted(device) || diskMetadataProtected(device)) protectedParents.add(diskIdentity(name));
        const mountedName = diskCensusDeviceName(device);
        if (diskRowMounted(device) && mountedName) {
          for (const candidate of devices) {
            if (mountedNameSharesParent(mountedName, candidate)) protectedParents.add(diskIdentity(diskCensusDeviceName(candidate)));
          }
        }
      }
      const pane = owner?.host?.closest('[data-pane-panel="admin"]');
      const mountedNames = [...(pane?.querySelectorAll('[data-admin-mounts-readback] .disk-item.mounted .device-label') || [])].map(node => node.textContent.trim()).filter(Boolean);
      for (const device of devices) {
        if (mountedNames.some(mounted => mountedNameSharesParent(mounted, device))) protectedParents.add(diskIdentity(diskCensusDeviceName(device)));
      }
      return protectedParents;
    }
    function mountedNasDestinations(owner) {
      const pane = owner?.host?.closest('[data-pane-panel="admin"]');
      const destinations = [...owner.devices].map(device => device?.mountpoint).filter(value => typeof value === 'string' && value.length > 0);
      for (const item of pane?.querySelectorAll('[data-admin-mounts-readback] .disk-item.mounted') || []) {
        const text = item.querySelector('.disk-details')?.textContent || '';
        for (const destination of ['/mnt/nas', '/mnt/nas_backup']) {
          const offset = text.indexOf(destination);
          if (offset >= 0 && !/[a-z0-9_]/i.test(text[offset + destination.length] || '')) destinations.push(destination);
        }
      }
      return [...new Set(destinations)];
    }
    function censusHasPrimaryNas(devices) {
      return devices.some(device => String(device?.label || '').toLowerCase().includes('homeserver-primary-nas') || device?.mountpoint === '/mnt/nas');
    }
    function censusHasMountedNas(devices) {
      return devices.some(device => diskRowMounted(device) && (device?.mountpoint === '/mnt/nas' || device?.mountpoint === '/mnt/nas_backup' || /homeserver-(primary|backup)-nas/.test(String(device?.label || '').toLowerCase())));
    }
    function updateAdminNasStatus(owner) {
      if (!adminDiskSnapshotCurrent(owner)) return;
      const pane = owner.host.closest('[data-pane-panel="admin"]');
      const status = pane?.querySelector('[data-admin-nas-status]');
      if (!status) return;
      const mounts = mountedNasDestinations(owner);
      owner.primaryNasExists = censusHasPrimaryNas(owner.devices) || mounts.includes('/mnt/nas');
      const anyNasMounted = censusHasMountedNas(owner.devices) || mounts.includes('/mnt/nas') || mounts.includes('/mnt/nas_backup');
      status.textContent = anyNasMounted ? 'NAS is ready.' : 'No NAS yet';
      status.dataset.state = anyNasMounted ? 'mounted' : 'empty';
    }
    function setAdminNasSetupStatus(owner, message, state, allowRefresh = false) {
      if (!adminDiskSnapshotCurrent(owner)) return;
      const status = owner.host.closest('[data-pane-panel="admin"]')?.querySelector('[data-admin-nas-status]');
      if (!status) return;
      status.textContent = message;
      status.dataset.state = state;
      if (allowRefresh) {
        const refresh = document.createElement('button');
        refresh.type = 'button';
        refresh.className = 'action-button';
        refresh.dataset.nasCensusRefresh = '';
        refresh.textContent = 'Read disk status again';
        status.append(document.createTextNode(' '), refresh);
      }
    }
    function renderDiskCensusDevice(device, owner) {
      const name = diskCensusDeviceName(device);
      const label = typeof device?.label === 'string' && device.label ? device.label : 'Unlabelled disk';
      const eligible = Boolean(name && diskNameCanNormalize(name) && !owner.protectedParents.has(diskIdentity(name)));
      const role = owner.primaryNasExists ? 'backup' : 'primary';
      const partition = diskDisplayValue(device?.partition);
      const encryption = device?.encryption && typeof device.encryption === 'object' ? device.encryption : {};
      const details = `<div class="disk-census-fields"><span><strong>Device:</strong> ${escapeDiskHtml(name || 'Unknown device')}</span><span><strong>Partition:</strong> ${escapeDiskHtml(partition)}</span><span><strong>Size:</strong> ${escapeDiskHtml(diskDisplayValue(device?.sizeBytes))} bytes</span><span><strong>Filesystem:</strong> ${escapeDiskHtml(diskDisplayValue(device?.fstype))}</span><span><strong>Encryption:</strong> ${escapeDiskHtml(diskDisplayValue(encryption.state))}</span><span><strong>Mapper:</strong> ${escapeDiskHtml(diskDisplayValue(encryption.mapper))}</span><span><strong>Mount:</strong> ${escapeDiskHtml(diskDisplayValue(device?.mountpoint))}</span><span><strong>Space:</strong> ${escapeDiskHtml(diskDisplayValue(device?.space))}</span></div>`;
      const action = eligible
        ? `<div class="disk-census-action"><span>${owner.nasSetupStateUnknown ? 'Refresh disk status before setup' : `Set up as NAS ${role}`}</span><button type="button" class="action-button" data-nas-setup-open data-nas-device="${escapeDiskHtml(name)}" data-nas-role="${role}"${owner.nasSetupStateUnknown || adminNasSetupRequests.has(diskIdentity(name)) || adminNasSetupInFlight ? ' disabled' : ''}>Make this my NAS</button></div>`
        : '';
      const classes = eligible ? 'disk-item disk-census-row' : 'disk-item disk-census-row unavailable';
      return `<article class="${classes}" data-disk-parent="${escapeDiskHtml(name)}"><span class="disk-icon" aria-hidden="true">▣</span><div class="disk-info"><div class="disk-name">${escapeDiskHtml(label)}</div>${details}${action}</div></article>`;
    }
    function admittedAdminDiskHost() {
      const family = adminDiskSnapshotFamily;
      if (adminDiskPageHidden || family.authClass !== 'admin' || !headerState.isAdmin || !coronatioAttendanceRuntime.currentAttendance || !sessionLawfulTab(family.paneId) || !viewportFamilyAdmitted(family.paneId)) return null;
      const pane = document.querySelector(`[data-pane-panel="${family.paneId}"]`);
      if (!pane?.isConnected || !pane.classList.contains('active') || pane.hidden || pane.getAttribute('aria-hidden') === 'true' || pane.dataset.viewportFaulted === 'true') return null;
      return pane.querySelector('[data-disk-census-readback]');
    }
    function adminDiskSnapshotCurrent(owner) {
      return Boolean(owner && adminDiskSnapshotOwner === owner && owner.host.isConnected && admittedAdminDiskHost() === owner.host && owner.attendance === coronatioAttendanceRuntime.currentAttendance);
    }
    function retireAdminDiskSnapshot() {
      const owner = adminDiskSnapshotOwner;
      adminDiskSnapshotOwner = null; // Revoke write ownership before abort can settle an old promise.
      if (!owner) return;
      if (owner.timer !== null) window.clearTimeout(owner.timer);
      owner.timer = null;
      owner.controller.abort();
    }
    function paintAdminDiskMessage(owner, message) {
      if (!adminDiskSnapshotCurrent(owner)) return;
      owner.host.innerHTML = `<div class="disk-item empty"><span class="disk-icon">▣</span><div class="disk-info"><div class="disk-name">${escapeDiskHtml(message)}</div></div></div>`;
    }
    function reconcileAdminDiskSnapshot() {
      const host = admittedAdminDiskHost();
      if (adminDiskSnapshotOwner && (!host || !adminDiskSnapshotCurrent(adminDiskSnapshotOwner))) retireAdminDiskSnapshot();
      if (host) return hydrateDiskCensus();
      return Promise.resolve([]);
    }
    function hydrateDiskCensus() {
      const host = admittedAdminDiskHost();
      if (!host) { retireAdminDiskSnapshot(); return Promise.resolve([]); }
      if (adminDiskSnapshotCurrent(adminDiskSnapshotOwner)) return adminDiskSnapshotOwner.promise;
      retireAdminDiskSnapshot();
      const owner = { host, attendance: coronatioAttendanceRuntime.currentAttendance, controller: new AbortController(), timer: null, timedOut: false, error: '', devices: [], protectedParents: new Set(), primaryNasExists: false, nasSetupStateUnknown: false, promise: null };
      adminDiskSnapshotOwner = owner;
      paintAdminDiskMessage(owner, 'Reading available devices…');
      // Coronatio's upstream has separate 4s write/read waits. Allow 15s for
      // cold reads and transport, without borrowing the pane admission budget.
      owner.timer = window.setTimeout(() => {
        owner.timer = null;
        owner.timedOut = true;
        owner.controller.abort();
      }, adminDiskSnapshotFamily.timeoutMs);
      owner.promise = (async () => {
        try {
          const response = await fetch(adminDiskSnapshotFamily.snapshotRoutes[0], { cache: 'no-store', signal: owner.controller.signal });
          if (!adminDiskSnapshotCurrent(owner)) return [];
          if (!response.ok) throw new Error('Available devices could not be read (HTTP ' + response.status + ').');
          let payload;
          try { payload = await response.json(); }
          catch (_) { throw new Error('Available devices could not be read: invalid response.'); }
          if (!adminDiskSnapshotCurrent(owner)) return [];
          if (owner.controller.signal.aborted) throw new Error('Available devices took too long to respond.');
          if (payload?.schema !== 'caduceus.disk.census.v1') throw new Error('Available devices could not be read: unexpected response schema.');
          if (!Array.isArray(payload?.devices)) throw new Error('Available devices could not be read: device list missing.');
          const devices = diskCensusDevices(payload);
          owner.devices = devices;
          owner.protectedParents = protectedDiskParents(devices, owner);
          updateAdminNasStatus(owner);
          if (devices.length) owner.host.innerHTML = devices.map(device => renderDiskCensusDevice(device, owner)).join('');
          else paintAdminDiskMessage(owner, 'No disks found.');
          return devices;
        } catch (error) {
          if (!adminDiskSnapshotCurrent(owner)) return [];
          owner.error = owner.timedOut ? 'Available devices took too long to respond.' : (error?.message || 'Available devices could not be read.');
          paintAdminDiskMessage(owner, owner.error);
          const nasStatus = owner.host.closest('[data-pane-panel="admin"]')?.querySelector('[data-admin-nas-status]');
          if (nasStatus) { nasStatus.textContent = 'NAS status unavailable.'; nasStatus.removeAttribute('data-state'); }
          return [];
        } finally {
          if (owner.timer !== null) window.clearTimeout(owner.timer);
          owner.timer = null;
        }
      })();
      // Retain the settled promise too: repeat reconciliation/modal opens do
      // not retry failures or fetch again until this activation is retired.
      return owner.promise;
    }
    const adminDiskHostObserver = new MutationObserver(records => {
      const owner = adminDiskSnapshotOwner;
      if (owner && records.some(record => [...record.removedNodes].some(node => node === owner.host || node.contains?.(owner.host)))) retireAdminDiskSnapshot();
      reconcileAdminDiskSnapshot();
    });
    if (immortalFloorGuestSlot) adminDiskHostObserver.observe(immortalFloorGuestSlot, { childList: true, subtree: true, attributes: true, attributeFilter: ['class', 'hidden', 'aria-hidden', 'data-viewport-faulted'] });
    document.body.addEventListener('htmx:beforeCleanupElement', event => {
      const target = event.detail?.elt;
      const host = adminDiskSnapshotOwner?.host;
      if (host && (target === host || target?.contains?.(host))) retireAdminDiskSnapshot();
    });
    window.addEventListener('pagehide', () => { adminDiskPageHidden = true; retireAdminDiskSnapshot(); });
    window.addEventListener('pageshow', () => { adminDiskPageHidden = false; reconcileAdminDiskSnapshot(); });
    function vaultUnlockSucceeded(result) {
      if (!result || typeof result !== 'object') return false;
      const positive = result.ok === true || result.success === true;
      const negative = result.ok === false || result.success === false || result.converged === false || result.rolledBack === true || (typeof result.failure === 'string' && result.failure.length > 0);
      return positive && !negative;
    }
    function vaultUnlockMessage(result, success) {
      const keys = success ? ['message', 'failure', 'error', 'firstMissingSignal'] : ['failure', 'error', 'firstMissingSignal', 'message'];
      const detail = keys.map(key => result?.[key]).find(value => typeof value === 'string' && value.length > 0);
      if (!success) return detail ? `Vault unlock failed: ${detail}` : 'Vault unlock failed.';
      return detail || 'Vault unlocked.';
    }
    async function submitVaultUnlock(form) {
      const input = form.querySelector('[data-vault-unlock-password]');
      const submit = form.querySelector('[data-vault-unlock-submit]');
      const resultNode = form.closest('[data-vault-unlock]')?.querySelector('[data-vault-unlock-result]') || form.parentElement?.querySelector('[data-vault-unlock-result]');
      if (!input || !submit || form.dataset.inFlight === 'true' || input.value.length === 0) return;
      const password = input.value;
      input.value = '';
      form.dataset.inFlight = 'true';
      submit.disabled = true;
      if (resultNode) { resultNode.textContent = 'Unlocking vault…'; resultNode.dataset.state = 'pending'; }
      try {
        const response = await fetch('/api/v1/storage/vault/unlock', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ password }) });
        const result = await response.json().catch(() => null);
        const success = response.ok && vaultUnlockSucceeded(result);
        if (resultNode) { resultNode.textContent = vaultUnlockMessage(result, success); resultNode.dataset.state = success ? 'success' : 'error'; }
      } catch (_) {
        if (resultNode) { resultNode.textContent = 'Vault unlock could not be completed.'; resultNode.dataset.state = 'error'; }
      } finally {
        input.value = '';
        form.dataset.inFlight = 'false';
        submit.disabled = true;
      }
    }
    function nasReceiptHasFailure(result) {
      const negative = result?.ok === false || result?.success === false || result?.converged === false || result?.rolledBack === true;
      const failure = result?.failure !== undefined && result.failure !== null && result.failure !== false && result.failure !== '';
      const failedStep = Array.isArray(result?.steps) && result.steps.some(step => step?.ok === false);
      return negative || failure || failedStep;
    }
    function nasReceiptSucceeded(result, responseOk) {
      return responseOk && result?.schema === 'caduceus.nas.setup.v1' && (result.ok === true || result.success === true) && !nasReceiptHasFailure(result);
    }
    function receiptFieldText(value) {
      if (value === undefined) return 'not reported';
      if (value === null) return 'null';
      return typeof value === 'object' ? JSON.stringify(value) : String(value);
    }
    function renderNasSetupReceipt(owner, result, responseOk, responseStatus) {
      if (!adminDiskSnapshotCurrent(owner)) return;
      const pane = owner.host.closest('[data-pane-panel="admin"]');
      const target = pane?.querySelector('[data-admin-nas-setup-result]');
      if (!target) return;
      const success = nasReceiptSucceeded(result, responseOk);
      target.replaceChildren();
      target.dataset.state = success ? 'success' : 'error';
      const summary = document.createElement('p');
      summary.className = success ? 'nas-setup-outcome success' : 'nas-setup-outcome error';
      summary.textContent = success ? 'NAS is ready.' : 'NAS setup could not finish.';
      target.appendChild(summary);
      const steps = Array.isArray(result?.steps) ? result.steps : [];
      if (steps.length) {
        const list = document.createElement('ol');
        list.className = 'nas-setup-steps';
        for (const item of steps) {
          const row = document.createElement('li');
          row.dataset.state = item?.ok === true ? 'success' : item?.ok === false ? 'failure' : 'unknown';
          const title = document.createElement('strong');
          const stepName = typeof item?.step === 'string' && item.step.trim() ? item.step : 'Unidentified step';
          title.textContent = item?.ok === true ? `Passed — ${stepName}` : item?.ok === false ? `Failed — ${stepName}` : `Not confirmed — ${stepName}`;
          row.appendChild(title);
          list.appendChild(row);
        }
        target.appendChild(list);
      } else {
        const missing = document.createElement('p');
        missing.textContent = 'The receipt contained no step readbacks.';
        target.appendChild(missing);
      }
      const details = document.createElement('details');
      const summaryNode = document.createElement('summary');
      summaryNode.textContent = 'Receipt details';
      const facts = document.createElement('p');
      facts.className = 'nas-setup-root-facts';
      facts.textContent = `HTTP ${responseStatus || 'unavailable'} · ok=${receiptFieldText(result?.ok)} · success=${receiptFieldText(result?.success)} · converged=${receiptFieldText(result?.converged)} · failure=${receiptFieldText(result?.failure)} · rolledBack=${receiptFieldText(result?.rolledBack)}`;
      const raw = document.createElement('pre');
      raw.textContent = result === null ? 'No JSON receipt was returned.' : JSON.stringify(result, null, 2);
      details.append(summaryNode, facts, raw);
      target.appendChild(details);
    }
    function refreshNasSetupButtons(owner) {
      if (!adminDiskSnapshotCurrent(owner)) return;
      owner.host.querySelectorAll('[data-nas-setup-open]').forEach(button => {
        const name = button.dataset.nasDevice || '';
        const role = owner.primaryNasExists ? 'backup' : 'primary';
        button.dataset.nasRole = role;
        button.disabled = adminNasSetupInFlight || owner.nasSetupStateUnknown || adminNasSetupRequests.has(diskIdentity(name)) || !name || owner.protectedParents.has(diskIdentity(name));
        const roleLabel = button.parentElement?.querySelector('span');
        if (roleLabel) roleLabel.textContent = owner.nasSetupStateUnknown ? 'Read disk status before setup' : `Set up as NAS ${role}`;
      });
    }
    function openNasSetupConfirmation(owner, name) {
      if (!adminDiskSnapshotCurrent(owner) || adminNasSetupInFlight || adminNasSetupRequests.has(diskIdentity(name))) return;
      const device = owner.devices.find(item => diskCensusDeviceName(item) === name);
      if (!device || owner.nasSetupStateUnknown || owner.protectedParents.has(diskIdentity(name)) || !diskNameCanNormalize(name)) return;
      const role = owner.primaryNasExists ? 'backup' : 'primary';
      const body = `<p>This will erase all data on <code>${escapeDiskHtml(name)}</code> and prepare it as the ${role} NAS.</p><p>Type the displayed device name exactly to confirm erasure.</p><label for="nas-setup-confirmation">Confirm device name<input id="nas-setup-confirmation" class="ui-input ui-input--medium" type="text" autocomplete="off" data-nas-setup-confirmation></label><p data-nas-setup-state aria-live="polite">Waiting for exact confirmation.</p>`;
      const modal = managerModal('nas-setup', 'Make this my NAS', body, 'Erase and set up NAS');
      const input = modal.querySelector('[data-nas-setup-confirmation]');
      const confirm = modal.querySelector('[data-manager-confirm]');
      const state = modal.querySelector('[data-nas-setup-state]');
      const validate = () => { confirm.disabled = input.value !== name || owner.nasSetupStateUnknown || adminNasSetupInFlight || adminNasSetupRequests.has(diskIdentity(name)); };
      input.addEventListener('input', validate);
      confirm.addEventListener('click', async () => {
        if (modal.dataset.setupStarted === 'true' || input.value !== name || !adminDiskSnapshotCurrent(owner) || owner.nasSetupStateUnknown || adminNasSetupInFlight || adminNasSetupRequests.has(diskIdentity(name))) return;
        modal.dataset.setupStarted = 'true';
        input.disabled = true;
        confirm.disabled = true;
        adminNasSetupRequests.add(diskIdentity(name));
        adminNasSetupInFlight = true;
        refreshNasSetupButtons(owner);
        state.textContent = 'Preparing this disk…';
        let result = null;
        let responseOk = false;
        let responseStatus = 0;
        try {
          const response = await fetch('/api/v1/storage/nas/setup', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ device: name, role, confirmation: input.value }) });
          responseStatus = response.status;
          responseOk = response.ok;
          result = await response.json().catch(() => null);
          const success = nasReceiptSucceeded(result, responseOk);
          state.textContent = success ? 'NAS is ready.' : 'NAS setup could not finish.';
          if (role === 'primary' && success) owner.primaryNasExists = true;
          if (!success) owner.nasSetupStateUnknown = true;
          renderNasSetupReceipt(owner, result, responseOk, responseStatus);
          setAdminNasSetupStatus(owner, success ? 'NAS is ready.' : 'NAS setup could not finish. Disk status is unknown; read it again before another setup.', success ? 'ready' : 'unknown', !success);
        } catch (_) {
          result = { schema: 'coronatio.storage.nas.setup.transport-error.v1', ok: false, failure: 'No setup receipt was returned.' };
          state.textContent = 'NAS setup could not finish.';
          owner.nasSetupStateUnknown = true;
          renderNasSetupReceipt(owner, result, false, 0);
          setAdminNasSetupStatus(owner, 'NAS setup could not finish. Disk status is unknown; read it again before another setup.', 'unknown', true);
        } finally {
          adminNasSetupInFlight = false;
          refreshNasSetupButtons(owner);
          modal.querySelectorAll('[data-manager-close]').forEach(button => { button.disabled = false; });
        }
      });
      input.focus();
    }
    function hardDriveTestModal() { const modal = managerModal('hard-drive-test', 'Hard Drive Test', '<p>Choose a NAS drive and the test depth.</p><label>Device<select class="ui-input ui-input--medium" data-hard-drive-test-device><option value="">Reading available devices…</option></select></label><label>Test type<select class="ui-input ui-input--medium" data-hard-drive-test-type><option value="quick">Quick</option><option value="full">Full</option><option value="ultimate">Ultimate</option></select></label><p data-hard-drive-test-state aria-live="polite">Choose a device to begin.</p>', 'Start Test'); const device = modal.querySelector('[data-hard-drive-test-device]'); const type = modal.querySelector('[data-hard-drive-test-type]'); const state = modal.querySelector('[data-hard-drive-test-state]'); const confirm = modal.querySelector('[data-manager-confirm]'); const ready = () => { confirm.disabled = !device.value; }; const census = hydrateDiskCensus(); const censusOwner = adminDiskSnapshotOwner; census.then(devices => { if (!modal.isConnected || !adminDiskSnapshotCurrent(censusOwner)) return; const message = censusOwner.error || (devices.length ? 'Choose a NAS drive' : 'No NAS drives available'); device.innerHTML = `<option value="">${escapeDiskHtml(message)}</option>${devices.map(item => `<option value="${escapeDiskHtml(diskCensusDeviceName(item))}">${escapeDiskHtml(diskCensusDeviceName(item))}</option>`).join('')}`; if (censusOwner.error || !devices.length) state.textContent = message; ready(); }); device.addEventListener('change', ready); confirm.addEventListener('click', async () => { confirm.disabled = true; state.textContent = 'Starting drive test…'; try { const response = await fetch('/api/admin/hard-drive-test/start', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ device: device.value, testType: type.value }) }); if (!response.ok) throw new Error('start-refused'); state.textContent = 'Drive test started. Reading progress…'; const timer = window.setInterval(async () => { try { const progress = await fetch('/api/admin/hard-drive-test/progress', { cache: 'no-store' }).then(result => result.json()); const results = await fetch('/api/admin/hard-drive-test/results', { cache: 'no-store' }).then(result => result.json()); state.textContent = String(progress?.message || progress?.status || results?.message || results?.status || 'Running'); if (progress?.complete || progress?.done || results?.complete || results?.done) { window.clearInterval(timer); confirm.disabled = false; } } catch (_) {} }, 1000); } catch (_) { state.textContent = 'Hard Drive Test could not be started.'; confirm.disabled = false; } }); }
    document.body.addEventListener('submit', event => {
      const form = event.target instanceof Element ? event.target.closest('[data-vault-unlock-form]') : null;
      if (!form) return;
      event.preventDefault();
      submitVaultUnlock(form);
    });
    document.body.addEventListener('input', event => {
      const input = event.target instanceof Element ? event.target.closest('[data-vault-unlock-password]') : null;
      if (!input) return;
      const form = input.closest('[data-vault-unlock-form]');
      const submit = form?.querySelector('[data-vault-unlock-submit]');
      if (submit) submit.disabled = input.value.length === 0 || form.dataset.inFlight === 'true';
    });
    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      const nasButton = target?.closest('[data-nas-setup-open]');
      if (nasButton) {
        event.preventDefault();
        openNasSetupConfirmation(adminDiskSnapshotOwner, nasButton.dataset.nasDevice || '');
        return;
      }
      if (target?.closest('[data-nas-census-refresh]')) {
        event.preventDefault();
        const owner = adminDiskSnapshotOwner;
        if (adminDiskSnapshotCurrent(owner)) {
          retireAdminDiskSnapshot();
          hydrateDiskCensus();
        }
        return;
      }
      const manager = target?.closest('[data-manager-open]');
      if (manager) { const kind = manager.dataset.managerOpen; if (kind === 'key-guide') managerGuideModal(); else managerKeyModal(kind); return; }
      if (target?.closest('[data-hard-drive-test-open]')) { hardDriveTestModal(); return; }
    });
    __DHCP_CLIENT__
    __UNBOUND_CLIENT__
    __FIREWALL_CLIENT__
    hydrateFavoriteManifest(); hydrateThemeTruth(); hydrateThemeTokenLab();
    hydrateUptime();
    setInterval(tickUptime, 1000);

  </script>
</body>
</html>"####
}
