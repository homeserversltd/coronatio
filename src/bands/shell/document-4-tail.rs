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
    function showCoronatioToast(message, variant = 'info') {
      const stack = document.querySelector('[data-coronatio-toast-stack]');
      if (!stack || !message) return;
      const allowed = ['info', 'success', 'warning', 'error'];
      const resolvedVariant = allowed.includes(variant) ? variant : 'info';
      const icons = { info: 'ℹ️', success: '✅', warning: '⚠️', error: '❌' };
      const toast = document.createElement('div');
      toast.className = `toast ${resolvedVariant}`;
      toast.dataset.coronatioToast = '';
      toast.setAttribute('role', 'alert');
      const icon = document.createElement('span'); icon.className = 'toast-icon'; icon.setAttribute('aria-hidden', 'true'); icon.textContent = icons[resolvedVariant];
      const text = document.createElement('span'); text.className = 'toast-message'; text.textContent = String(message);
      toast.append(icon, text); stack.appendChild(toast); startCoronatioToastTimer(toast, 3000);
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
    function diskSelection(item) { const actions = document.querySelector('[data-disk-actions-state]'); if (!actions) return; document.querySelectorAll('[data-disk-select]').forEach(node => node.classList.toggle('selected', node === item)); const label = item.dataset.diskDevice || item.dataset.diskDestination || 'selection'; actions.dataset.diskActionsState = 'blocked'; actions.querySelector('[data-disk-action-reading]').textContent = `${label} selected. Disk actions are blocked: their og routes exist in quarry but no Crown Caduceus actuator is admitted.`; actions.querySelectorAll('[data-disk-action]').forEach(button => { button.disabled = true; button.title = 'Blocked: no Crown Caduceus route is admitted'; }); }
    document.body.addEventListener('click', event => { const manager = event.target.closest('[data-manager-open]'); if (manager) { const kind = manager.dataset.managerOpen; if (kind === 'key-guide') managerGuideModal(); else managerKeyModal(kind); return; } const disk = event.target.closest('[data-disk-select]'); if (disk) diskSelection(disk); });
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
