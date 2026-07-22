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
      'restart-website': 'Restart Website', 'view-logs': 'View Logs', 'install-certificate': 'Install Certificate'
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
        const token = slider.dataset.themeTokenSlider;
        const row = slider.closest('[data-theme-token-control]');
        const unit = row?.dataset.themeTokenUnit || '';
        const output = document.querySelector(`[data-theme-token-output="${token}"]`);
        const apply = () => {
          const value = `${slider.value}${unit}`;
          root.style.setProperty(token, value);
          if (output) output.textContent = value;
          row?.setAttribute('data-theme-token-current', value);
        };
        slider.addEventListener('input', apply);
        apply();
      });
    }
    __DHCP_CLIENT__
    hydrateFavoriteManifest(); hydrateThemeTruth(); hydrateThemeTokenLab();
    hydrateUptime();
    setInterval(tickUptime, 1000);

  </script>
</body>
</html>"####
}
