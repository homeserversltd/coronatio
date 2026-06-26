fn shell_document_1() -> &'static str {
    r####"<!doctype html>
<html lang="en" class="theme-loaded">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="coronatio-port-doctrine" content="one-to-one port; homeserver.json before any Coronatio-local fallback or firmware default">
  <title>Coronatio</title>
  <style>
    :root {
      color-scheme: dark;
      --theme-color-primary: #323840;
      --theme-color-secondary: #9CA3AF;
      --theme-bg-primary: #0A0A0A;
      --theme-bg-secondary: #0A0A0A;
      --theme-bg-tertiary: #1E293B;
      --theme-bg-hover: #6B7280;
      --theme-bg-active: #6B7280;
      --theme-text-primary: #E0E0E0;
      --theme-text-secondary: #9CA3AF;
      --theme-text-tertiary: #A0AEC0;
      --theme-text-disabled: #94A3B8;
      --theme-text-accent: #A78BFA;
      --theme-status-success: #10B981;
      --theme-status-error: #F87171;
      --theme-status-warning: #FBBF24;
      --theme-status-info: #A78BFA;
      --theme-spacing-xxs: 0.125rem;
      --theme-spacing-xs: 0.25rem;
      --theme-spacing-sm: 0.5rem;
      --theme-spacing-md: 1rem;
      --theme-spacing-lg: 1.5rem;
      --theme-spacing-xl: 2rem;
      --theme-spacing-2xl: 3rem;
      --theme-font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --theme-font-mono: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
      --theme-font-size-xs: 12px;
      --theme-font-size-sm: 0.875rem;
      --theme-font-size-base: 16px;
      --theme-font-size-md: 16px;
      --theme-font-size-lg: 1.125rem;
      --theme-font-size-xl: 24px;
      --theme-font-size-2xl: 32px;
      --theme-font-weight-normal: 400;
      --theme-font-weight-medium: 500;
      --theme-font-weight-bold: 600;
      --theme-line-height-tight: 1.2;
      --theme-line-height-normal: 1.5;
      --theme-line-height-loose: 1.8;
      --theme-transition-fast: 150ms ease;
      --theme-transition-normal: 250ms ease;
      --theme-transition-slow: 350ms ease;
      --theme-shadow-sm: 0 1px 2px rgba(0,0,0,0.1);
      --theme-shadow-md: 0 2px 4px rgba(0,0,0,0.1);
      --theme-shadow-lg: 0 4px 8px rgba(0,0,0,0.1);
      --theme-radius: 4px;
      --theme-radius-sm: 4px;
      --theme-radius-md: 6px;
      --theme-radius-lg: 8px;
      --theme-radius-pill: 999px;
      --theme-border-width: 1px;
      --theme-focus-ring: 0 0 0 2px color-mix(in srgb, var(--theme-color-primary) 30%, transparent);
      --theme-header-height: 48px;
      --theme-tab-height: 48px;
      --theme-control-height: 34px;
      --theme-control-padding-x: 0.9rem;
      --theme-control-padding-y: 0.55rem;
      --theme-content-padding: var(--theme-content-padding);
      --theme-card-padding: var(--theme-card-padding);
      --theme-card-min-height: var(--theme-card-min-height);
      --theme-card-radius: 8px;
      --theme-modal-radius: 10px;
      --theme-portal-icon-size: 96px;
      --theme-chart-height: var(--theme-chart-height);
      --theme-grid-gap: var(--theme-grid-gap);
      --theme-background: #0A0A0A;
      --theme-text: #E0E0E0;
      --theme-primary: #323840;
      --theme-primaryHover: #6B7280;
      --theme-secondary: #9CA3AF;
      --theme-accent: #A78BFA;
      --theme-error: #df0a3f;
      --theme-success: #10B981;
      --theme-warning: #FBBF24;
      --theme-border: #1E293B;
      --theme-statusUp: #10B981;
      --theme-statusDown: #F87171;
      --theme-statusPartial: #FBBF24;
      --theme-statusUnknown: #94A3B8;
      --theme-hiddenTabBackground: #1E293B;
      --theme-hiddenTabText: #A0AEC0;
      --theme-role-primary: #323840;
      --theme-role-on-primary: #E0E0E0;
      --theme-role-primary-container: #1E293B;
      --theme-role-on-primary-container: #E0E0E0;
      --theme-role-secondary: #9CA3AF;
      --theme-role-tertiary: #A78BFA;
      --theme-surface-0: #0A0A0A;
      --theme-surface-1: #111827;
      --theme-surface-2: #1E293B;
      --theme-surface-3: #323840;
      --theme-on-surface: #E0E0E0;
      --theme-on-surface-muted: #9CA3AF;
      --theme-outline: #6B7280;
      --theme-outline-variant: #1E293B;
      --theme-highlight-subtle: rgba(167,139,250,0.16);
      --theme-highlight-strong: rgba(167,139,250,0.34);
      --theme-highlight-ring: 0 0 0 3px rgba(167,139,250,0.34);
      --theme-accent-warm: #FBBF24;
      --theme-accent-cool: #90cff3;
      --theme-accent-neutral: #9CA3AF;
      --theme-accent-critical: #df0a3f;
      --theme-gradient-primary: linear-gradient(135deg, #323840 0%, #6B7280 100%);
      --theme-gradient-accent: linear-gradient(135deg, #A78BFA 0%, #90cff3 100%);
      --theme-gradient-surface: linear-gradient(180deg, #111827 0%, #0A0A0A 100%);
      --theme-gradient-highlight: radial-gradient(circle at 20% 20%, rgba(167,139,250,0.34), transparent 55%);
      --theme-elevation-1: 0 1px 4px rgba(0,0,0,0.28);
      --theme-elevation-2: 0 6px 16px rgba(0,0,0,0.34);
      --theme-elevation-3: 0 18px 40px rgba(0,0,0,0.44);
      --theme-overlay-scrim: rgba(0,0,0,0.68);
      --theme-overlay-tint: rgba(167,139,250,0.10);
      --theme-focus-color: #A78BFA;
      --theme-focus-width: 2px;
      --theme-focus-offset: 2px;
      --theme-contrast-minimum: 4.5;
      --theme-density: comfortable;
      --theme-component-button-container: #323840;
      --theme-component-button-on-container: #E0E0E0;
      --theme-component-button-hover-container: #6B7280;
      --theme-component-card-container: #111827;
      --theme-component-card-outline: #1E293B;
      --theme-flag-gradients: enabled;
      --theme-flag-highlights: enabled;
      --theme-flag-accent-stripes: enabled;
      --theme-flag-state-layers: enabled;
      --theme-flag-density-scale: enabled;
      --background: var(--theme-background);
      --surface: var(--theme-bg-primary);
      --surface-soft: var(--theme-bg-tertiary);
      --text: var(--theme-text);
      --text-secondary: var(--theme-secondary);
      --accent: var(--theme-accent);
      --accent-soft: color-mix(in srgb, var(--theme-accent) 16%, transparent);
      --primary: var(--theme-primary);
      --border: var(--theme-border);
      --error: var(--theme-error);
      --warning: var(--theme-warning);
      --success: var(--theme-success);
      --shadow: var(--theme-shadow-md);
      --primaryHover: var(--theme-primaryHover);
      --hiddenTabBackground: var(--theme-hiddenTabBackground);
      --hiddenTabText: var(--theme-hiddenTabText);
      --status-up: var(--theme-statusUp);
      --status-down: var(--theme-statusDown);
      --status-partial: var(--theme-statusPartial);
      --status-unknown: var(--theme-statusUnknown);
      font-family: var(--theme-font-family);
      background: var(--background);
      color: var(--text);
    }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: var(--background); color: var(--text); }
    .app { min-height: 100vh; display: flex; flex-direction: column; }
    .top-bar {
      min-height: var(--theme-header-height);
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      padding: 0 var(--theme-content-padding);
      background: var(--surface);
      border-bottom: 1px solid var(--border);
      box-shadow: var(--shadow);
    }
    .header-top-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; min-width: 0; flex: 1 1 auto; }
    .header-left, .header-center, .header-right { display: flex; align-items: center; gap: .5rem; }
    .header-center { justify-content: center; flex: 2 1 auto; min-width: 0; }
    .header-right { justify-content: flex-end; flex-wrap: wrap; }
    .uptime { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: .82rem; padding: .24rem .5rem; border-radius: var(--theme-radius-md); background: rgba(255,255,255,.06); color: var(--text); }
    .status-indicators { display: flex; flex-direction: row; align-items: center; justify-content: center; gap: 8px; }
    .indicator { cursor: pointer; padding: .25rem; display: flex; align-items: center; justify-content: center; border: 0; border-radius: var(--theme-radius-md); background: transparent; color: var(--text); transition: transform .2s ease, background-color .2s ease, box-shadow .2s ease; }
    .indicator:hover { background-color: var(--primary-hover); transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,0,0,.15); }
    .indicator:active { transform: translateY(1px); box-shadow: 0 1px 2px rgba(0,0,0,.1); }
    .indicator-icon { width: 1.15rem; height: 1.15rem; display: block; fill: currentColor; }
    .indicator .indicator-icon-spinner { display: none; animation: indicator-spin .9s linear infinite; }
    .indicator.loading .indicator-icon-plug { display: none; }
    .indicator.loading .indicator-icon-spinner { display: block; }
    .power-indicator { gap: .2rem; }
    .power-value-small { display: inline-flex; align-items: baseline; gap: .05rem; color: currentColor; font-size: .72rem; font-weight: 700; line-height: 1; min-width: 2.25rem; }
    .power-value-small-unit { font-size: .62rem; }
    .indicator.ok .indicator-icon { color: var(--success); }
    .indicator.warn .indicator-icon { color: var(--warning); }
    .indicator.error .indicator-icon { color: var(--error); }
    .status-text.connected { color: var(--success); }
    .status-text.disconnected, .status-text.error { color: var(--error); }
    .status-text.loading { color: var(--text); }
    .error-message { color: var(--error); font-size: .9rem; }
    @keyframes indicator-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
    .modal-body { display: grid; gap: .75rem; color: var(--text-secondary); }
    .modal-body ul { margin: .25rem 0 0; padding-left: 1.15rem; }
    .modal-section, .status-section, .config-section, .credentials-section, .service-controls, .power-history-section, .speed-test-section { display: grid; gap: .55rem; }
    .modal-grid { display: grid; gap: .55rem; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); }
    .modal-body input { width: 100%; padding: .55rem .65rem; border: 1px solid var(--border); border-radius: var(--theme-radius-md); background: var(--background); color: var(--text); }
    .status-item, .power-average-row, .current-tailnet { display: flex; justify-content: space-between; gap: .75rem; border-bottom: 1px solid rgba(255,255,255,.08); padding-bottom: .35rem; }
    .status-value, .power-value, .power-average-value, .value { color: var(--text); font-weight: 700; }
    .tailscale-status-modal { padding: 0 .5rem; max-width: 400px; }
    .tailscale-status-modal .status-text { font-size: .95rem; font-weight: 600; display: flex; align-items: center; gap: .5rem; padding: .25rem .5rem; border-radius: 4px; }
    .tailscale-status-modal .status-text.connected { justify-content: space-between; color: var(--success); padding: 12px; background-color: color-mix(in srgb, var(--success) 15%, transparent); border: 1px solid color-mix(in srgb, var(--success) 30%, transparent); }
    .tailscale-status-modal .status-text.disconnected, .tailscale-status-modal .status-text.error { justify-content: space-between; color: var(--error); padding: 12px; background-color: color-mix(in srgb, var(--error) 15%, transparent); border: 1px solid color-mix(in srgb, var(--error) 30%, transparent); }
    .tailscale-status-modal .status-text.login-required { color: var(--warning); background-color: color-mix(in srgb, var(--warning) 15%, transparent); border-color: color-mix(in srgb, var(--warning) 30%, transparent); }
    .tailscale-status-modal .connection-buttons, .tailscale-status-modal .service-controls { display: grid; grid-template-columns: 1fr 1fr; gap: .75rem; margin-bottom: .5rem; }
    .tailscale-status-modal .current-tailnet { display: flex; gap: .5rem; align-items: baseline; margin-bottom: 1rem; font-size: .9rem; }
    .tailscale-status-modal .current-tailnet .label { color: color-mix(in srgb, var(--text) 60%, transparent); }
    .tailscale-status-modal .config-form { display: grid; gap: .75rem; }
    .tailscale-status-modal .tooltip-text, .tailscale-status-modal .authkey-help { white-space: pre-line; font-size: .8rem; color: color-mix(in srgb, var(--text) 60%, transparent); line-height: 1.4; padding-bottom: .75rem; }
    .tailscale-status-modal .login-required-section { background-color: color-mix(in srgb, var(--warning) 8%, transparent); border: 1px solid color-mix(in srgb, var(--warning) 20%, transparent); border-radius: 8px; padding: 1rem; margin: 1rem 0; }
    .tailscale-status-modal .login-message strong { color: var(--warning); font-size: 1.1rem; display: block; margin-bottom: .5rem; }
    .tailscale-status-modal .login-url-container { display: flex; flex-direction: column; gap: .5rem; margin-bottom: 1rem; padding: .75rem; background-color: color-mix(in srgb, var(--background) 50%, transparent); border-radius: 6px; border: 1px solid color-mix(in srgb, var(--text) 10%, transparent); }
    .tailscale-status-modal .login-url-link { color: var(--primary); text-decoration: none; font-family: monospace; font-size: .85rem; word-break: break-all; padding: .5rem; background-color: color-mix(in srgb, var(--primary) 5%, transparent); border-radius: 4px; border: 1px solid color-mix(in srgb, var(--primary) 15%, transparent); }
    .tailscale-status-modal .copy-url-button { align-self: flex-start; font-size: .85rem; }
    .tailscale-status-modal .login-instructions { font-size: .85rem; line-height: 1.4; }
    .tailscale-status-modal .authkey-section { margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid color-mix(in srgb, var(--text) 10%, transparent); }
    .tailscale-status-modal .authkey-form { display: grid; gap: .75rem; margin-bottom: .75rem; }
    .tailscale-status-modal .authkey-input { font-family: monospace; font-size: .85rem; }
    .pending-operation { opacity: .82; cursor: progress; }
    .action-output:empty { display: none; }
    .theme-choice-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-top: .5rem; }
    .theme-choice-row[hidden] { display: none !important; }
    .theme-choice { min-width: 90px; }
    .header-control, .admin-button, .theme-button, .change-admin-pin-button { min-height: var(--theme-control-height); min-width: 120px; padding: 0 var(--theme-control-padding-x); border: none; border-radius: var(--theme-radius-md); background: var(--primary); color: var(--text); font-size: var(--theme-font-size-sm); font-weight: 700; box-shadow: inset 0 2px 4px rgba(0,0,0,.2); }
    .header-control:hover, .admin-button:hover, .theme-button:hover, .change-admin-pin-button:hover { filter: brightness(1.08); transform: translateY(-1px); }
    .modal-backdrop { position: fixed; inset: 0; display: none; place-items: center; background: rgba(0,0,0,.55); z-index: 2000; padding: var(--theme-card-padding); }
    .modal-backdrop.open { display: grid; }
    .modal { width: min(420px, 100%); background: var(--surface); border: 1px solid var(--border); border-radius: var(--theme-modal-radius); box-shadow: 0 16px 40px rgba(0,0,0,.45); padding: var(--theme-card-padding); }
    .modal h2 { margin: 0 0 .75rem; font-size: 1.05rem; }
    .pin-modal { display: flex; flex-direction: column; gap: var(--theme-grid-gap); padding: 16px 0 0; }
    .pin-modal input { padding: 8px 12px; border: 1px solid var(--border); border-radius: 4px; background: var(--background); color: var(--text); font-size: 14px; outline: none; }
    .pin-modal input:focus { border-color: var(--primary); }
    .modal-actions { display: flex; justify-content: flex-end; gap: .5rem; margin-top: 1rem; }
    .toast-slot { min-height: 1.2rem; color: var(--warning); font-size: .84rem; margin-top: .5rem; }
    .status-strip { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: .45rem; color: var(--text-secondary); font-size: .82rem; }
    .status-pill { border: 1px solid var(--border); border-radius: 999px; padding: .18rem .55rem; background: rgba(255,255,255,.045); }
    .status-pill.ok { color: var(--success); border-color: color-mix(in srgb, var(--success) 45%, transparent); }
    .tab-bar {
      min-height: var(--theme-header-height);
      display: flex;
      align-items: center;
      gap: .5rem;
      padding: 0 var(--theme-content-padding);
      background: var(--surface-soft);
      border-bottom: 1px solid var(--border);
      overflow-x: auto;
    }
    .tab-bar.hidden { display: none; }
    .tab {
      display: grid;
      grid-template-columns: 28px minmax(max-content, 1fr) 28px;
      align-items: center;
      min-height: 38px;
      gap: .35rem;
      padding: .35rem .45rem;
      border-radius: var(--theme-card-radius);
      border: 1px solid transparent;
      color: var(--text-secondary);
      font-weight: 600;
      cursor: pointer;
      transition: background .18s ease, color .18s ease, border-color .18s ease, opacity .18s ease;
      user-select: none;
    }
    .tab:hover, .tab.active, .tab[aria-selected="true"] {
      background: var(--accent-soft);
      color: var(--text);
      border-color: color-mix(in srgb, var(--accent) 42%, transparent);
    }
    .tab[data-visibility="hidden"] { opacity: .48; border-style: dashed; }
    .tab[data-visibility="hidden"] .tab-name { text-decoration: line-through; }
    [data-admin-mode="false"] .tab[data-visibility="hidden"] { display: none; }
    [data-admin-mode="true"] .tab[data-visibility="hidden"] { display: grid; }
    .tab-visibility-column, .tab-star-column { display: grid; place-items: center; min-width: 24px; }
    .visibility-toggle, .star-button {
      display: grid;
      place-items: center;
      width: 24px;
      height: 24px;
      padding: 0;
      border: 1px solid transparent;
      border-radius: 999px;
      background: transparent;
      color: var(--text-secondary);
      line-height: 1;
    }
    .visibility-toggle:hover, .star-button:hover { background: rgba(255,255,255,.1); color: var(--text); }
    .visibility-toggle[data-visible="false"] { color: var(--warning); }
    .star-button.fas { color: var(--warning); }
    .star-button.far { color: rgba(255,255,255,.42); }
    .tab-name { white-space: nowrap; }
    .content { flex: 1; padding: var(--theme-content-padding); }
    .pane { display: none; max-width: 1180px; margin: 0 auto; }
    .pane.active { display: block; }
    .pane-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(270px, 1fr)); gap: var(--theme-grid-gap); align-items: start; }
    .portal-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: var(--theme-grid-gap); align-items: stretch; }
    .portal-card[data-portal-card] { min-height: 220px; align-items: center; text-align: center; cursor: pointer; }
    .portal-card[data-portal-card]:hover { transform: translateY(-2px); border-color: color-mix(in srgb, var(--accent) 48%, var(--border)); }
    .portal-card-header { display: flex; flex-direction: column; align-items: center; gap: .45rem; }
    .portal-icon { width: var(--theme-portal-icon-size); height: var(--theme-portal-icon-size); object-fit: contain; border-radius: var(--theme-card-radius); }
    .portal-name { margin: .25rem 0 0; font-size: 1.05rem; font-weight: 700; color: var(--text); }
    .portal-description { margin: 0; color: var(--text-secondary); font-size: .9rem; line-height: 1.35; }
    .portal-service-row { display: flex; gap: .35rem; justify-content: center; flex-wrap: wrap; color: var(--text-secondary); font-size: .78rem; }
    .portal-chip { border: 1px solid var(--border); border-radius: 999px; padding: .18rem .45rem; background: rgba(255,255,255,.05); }

    [data-admin-mode="false"] [data-portal-element][data-visible="false"] { display: none !important; }
    [data-admin-mode="true"] [data-portal-element][data-visible="false"] { display: block; }
    [data-portal-element][data-visible="false"] .portal-card { opacity: .62; border-style: dashed; }
    .portal-admin-controls { width: 100%; display: flex; flex-direction: column; gap: .3rem; }
    .admin-controls-row { display: grid; grid-template-columns: repeat(3, 1fr); gap: .25rem; width: 100%; }
    .admin-controls-row button { min-height: 32px; padding: .35rem .45rem; font-size: .82rem; }


    .card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--theme-card-radius);
      padding: var(--theme-card-padding);
      box-shadow: var(--shadow);
      min-height: var(--theme-card-min-height);
    }
    .portal-card { min-height: 180px; display: flex; flex-direction: column; justify-content: space-between; gap: .9rem; }
    .card h2, .card h3 { margin: 0 0 .65rem; font-size: 1rem; color: var(--text); }
    .card p { margin: .25rem 0; color: var(--text-secondary); font-size: .92rem; }
    .metric { font-size: 1.65rem; font-weight: 750; color: var(--accent); line-height: 1.1; }
    .stats-viewport { display: grid; gap: var(--theme-grid-gap); }
    .stats-section { background: var(--surface); border: 1px solid var(--border); border-radius: var(--theme-card-radius); padding: var(--theme-card-padding); box-shadow: var(--shadow); }
    .stats-section h2 { margin: 0 0 .75rem; font-size: 1rem; }
    .stats-resource-grid, .drives-grid, .network-grid, .services-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: .75rem; }
    .stats-resource-card, .drive-info, .network-interface, .service-info, .connections-summary { background: rgba(0,0,0,.18); border: 1px solid rgba(255,255,255,.08); border-radius: var(--theme-card-radius); padding: .75rem; }
    .stats-resource-card h3, .drive-info h3, .network-interface h3, .service-info h3 { margin: 0 0 .45rem; font-size: .95rem; }
    .chart-container { position: relative; width: 100%; height: var(--theme-chart-height); margin: .5rem 0 1rem; }
    .cpu-temp { font-weight: 800; color: var(--accent); }
    .cpu-details { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: .35rem; color: var(--text-secondary); font-size: .78rem; text-align: center; }
    .drive-checkboxes { display: flex; flex-wrap: wrap; gap: .55rem; margin: .35rem 0 .75rem; color: var(--text-secondary); font-size: .84rem; }
    .drive-checkboxes label { display: inline-flex; align-items: center; gap: .25rem; }
    .io-chart-legend { display: flex; flex-wrap: wrap; gap: .75rem; margin-top: .5rem; color: var(--text-secondary); font-size: .82rem; }
    .chart-fallback { color: var(--warning); font-size: .82rem; margin: .5rem 0; }
    .progress-bar { width: 100%; height: 8px; background: var(--background); border-radius: 999px; overflow: hidden; margin: .55rem 0; }
    .progress-bar .progress { height: 100%; width: 0%; background: var(--accent); transition: width .18s ease; }
    .details, .traffic, .counts, .service-info .details { display: flex; justify-content: space-between; gap: .65rem; color: var(--text-secondary); font-size: .84rem; flex-wrap: wrap; }
    .status-dot { display: inline-block; width: .55rem; height: .55rem; border-radius: 50%; margin-right: .35rem; background: var(--warning); }
    .status-dot.up, .status-dot.running { background: var(--success); }
    .status-dot.down, .status-dot.stopped { background: var(--error); }
    .stats-transport-card .button-row { margin-top: .65rem; }
    .muted { color: var(--text-secondary); }
    .readout { margin-top: .75rem; padding: .75rem; border-radius: var(--theme-radius-md); background: rgba(0,0,0,.22); border: 1px solid rgba(255,255,255,.08); font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: .78rem; color: #cdefff; overflow: auto; max-height: 220px; white-space: pre-wrap; }
    .button-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-top: .85rem; }
    .admin-tablet { display: flex; flex-direction: column; gap: .5rem; color: var(--text); }
    .admin-visual-port { display: flex; flex-direction: column; gap: .5rem; }
    .admin-tablet .system-controls { display: flex; flex-wrap: wrap; gap: 1rem; width: 100%; margin: 0 auto; box-shadow: 0 2px 0 var(--border); justify-content: center; padding-bottom: .7rem; }
    .system-controls-container, .admin-tablet .key-manager, .admin-tablet .disk-manager, .admin-modal-shelf { background-color: var(--background); border-radius: var(--theme-card-radius); box-shadow: 0 2px 4px var(--primary), 0 2px 4px var(--border); padding: 15px; color: var(--text); }
    .system-controls-btn { display: flex; justify-content: center; align-items: center; gap: 10px; padding: 10px 15px; border-radius: 4px; border: none; background: var(--primary); color: #fff; font-size: .9rem; font-weight: 500; min-width: 180px; width: 180px; flex: 0 0 auto; transition: background-color .2s, transform .1s, box-shadow .2s; position: relative; overflow: hidden; box-shadow: inset 0 2px 4px rgba(0,0,0,.1), 0 2px 4px rgba(0,0,0,.1); }
    .system-controls-btn::before { content: ''; position: absolute; inset: 0; background: linear-gradient(to bottom, rgba(255,255,255,.15) 0%, rgba(0,0,0,.1) 100%); mix-blend-mode: overlay; pointer-events: none; opacity: .6; }
    .system-controls-btn:hover { transform: translateY(-2px); background-color: var(--primaryHover); box-shadow: inset 0 2px 4px rgba(0,0,0,.1), 0 4px 8px rgba(0,0,0,.15); }
    .system-controls-btn span { position: relative; text-shadow: 0 1px 1px rgba(0,0,0,.2); }
    .admin-tablet h3, .admin-tablet h4 { color: var(--text); margin: 0 0 .75rem; }
    .admin-tablet .key-manager { padding: var(--theme-content-padding); margin-bottom: .5rem; }
    .key-manager-content { display: flex; gap: 30px; }
    .key-manager-left { flex: 2; }
    .key-manager-right { flex: 1; min-width: 250px; }
    .security-status, .key-actions { display: flex; flex-direction: column; gap: 15px; }
    .key-actions { position: sticky; top: 20px; }
    .status-item { display: flex; align-items: flex-start; gap: 12px; padding: 15px; background-color: var(--border); border-radius: var(--theme-card-radius); border-left: 4px solid transparent; border-color: var(--border); transition: transform .2s ease, box-shadow .2s ease; }
    .status-item:hover { transform: translateX(5px); }
    .status-icon { font-size: 1.5rem; margin-top: 2px; color: var(--success); }
    .key-manager .action-button, .disk-actions .action-button, .modal-button, .refresh-button, .tab-button { color: var(--text); cursor: pointer; font-weight: bold; transition: all .2s ease; }
    .key-manager .action-button { width: 100%; padding: 15px; border-radius: var(--theme-card-radius); border: none; display: flex; align-items: center; justify-content: center; gap: 10px; background-color: var(--primary); }
    .key-manager .action-button:hover { background-color: var(--primaryHover); transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,.2); }
    .info-button { display: inline-flex; width: auto; margin-left: .5rem; padding: 6px 10px; border-radius: var(--theme-radius-md); background: var(--primary); color: var(--text); border: 1px solid var(--border); }
    .admin-tablet .disk-manager { padding: var(--theme-content-padding); }
    .disk-manager-container { display: flex; gap: 20px; margin-bottom: 20px; }
    .disk-column { flex: 1; min-width: 0; }
    .disk-list { display: flex; flex-direction: column; gap: 10px; }
    .disk-item { background-color: var(--primary); border-radius: var(--theme-card-radius); padding: 15px; position: relative; transition: all .2s ease; border: 2px solid transparent; }
    .disk-item:hover, .disk-item.mounted { background-color: var(--primaryHover); }
    .disk-item.selected { border-color: var(--accent); background-color: var(--primaryHover); }
    .disk-item.available { border-color: var(--success); box-shadow: 0 0 8px var(--success); }
    .disk-item.locked-pair { border-color: var(--warning); }
    .disk-item.nas-compatible { border-left: 4px solid var(--success); }
    .disk-icon { font-size: 1.5rem; margin-right: 15px; color: var(--text); }
    .disk-info { display: inline-block; vertical-align: top; }
    .disk-name { color: var(--text); font-weight: bold; font-size: 1.1rem; margin-bottom: 5px; display: flex; align-items: center; }
    .nas-badge, .nas-role-badge { font-size: .7rem; padding: 2px 6px; border-radius: 4px; margin-left: 8px; font-weight: 600; background-color: var(--success); color: var(--background); }
    .nas-role-backup { background-color: var(--info); }
    .disk-details, .disk-mount-info { font-size: .85rem; color: var(--secondary); margin: 5px 0; }
    .disk-serial { font-size: .8rem; color: var(--secondary); margin-bottom: 5px; font-family: monospace; }
    .disk-space-usage { font-size: .85rem; color: var(--secondary); margin-top: 5px; padding: 3px 6px; background-color: var(--hiddenTabBackground); border-radius: 3px; border-left: 3px solid var(--info); }
    .disk-actions { display: flex; flex-wrap: wrap; gap: 10px; justify-content: center; margin-top: 20px; }
    .disk-actions .action-button { padding: 10px 15px; border-radius: 5px; background-color: var(--primary); border: 1px solid var(--border); display: flex; align-items: center; gap: 8px; }
    .disk-actions .action-button:hover { background-color: var(--primaryHover); }
    .action-button.danger-button, .modal-button-danger { background-color: var(--error); color: var(--text); border-color: var(--error); }
    .action-button.success-button, .modal-button-primary { background-color: var(--accent); color: var(--background); border-color: var(--accent); }
    .admin-modal-shelf { display: none; }
    .modal-window { background-color: var(--background); border: 1px solid var(--border); border-radius: var(--theme-card-radius); box-shadow: 0 4px 16px rgba(0,0,0,.25); overflow: hidden; flex-direction: column; min-height: 190px; }
    .modal-titlebar { background-color: var(--primaryHover); border-bottom: 1px solid var(--border); padding: 10px 14px; font-weight: 700; display: flex; align-items: center; justify-content: space-between; gap: 10px; }
    .modal-body-port { padding: 14px; display: flex; flex-direction: column; gap: 12px; }

    /* Coronatio composable UX library: stock TestTab composes these primitives. */
    .ux-surface { color: var(--text); font-family: var(--theme-font-family); }
    .ux-stack { display: grid; gap: var(--theme-grid-gap); }
    .ux-row { display: flex; flex-wrap: wrap; align-items: center; gap: var(--theme-spacing-sm); }
    .ux-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: var(--theme-grid-gap); }
    .ux-tabs { display: flex; flex-wrap: wrap; gap: var(--theme-spacing-xs); border-bottom: var(--theme-border-width) solid var(--border); padding-bottom: var(--theme-spacing-xs); margin-bottom: var(--theme-spacing-md); }
    .ux-interactive, .ux-tab, .ux-button, .ux-card-button, .ux-toggle, .ux-checkbox, .ux-field, .ux-select, .ux-textbox, .ux-badge-button { transition: background-color var(--theme-transition-fast), border-color var(--theme-transition-fast), box-shadow var(--theme-transition-fast), color var(--theme-transition-fast), transform var(--theme-transition-fast), opacity var(--theme-transition-fast); }
    .ux-tab { border: var(--theme-border-width) solid var(--border); border-radius: var(--theme-radius-md) var(--theme-radius-md) 0 0; min-height: var(--theme-tab-height); padding: 0 var(--theme-control-padding-x); background: var(--hiddenTabBackground); color: var(--text); cursor: pointer; font-weight: var(--theme-font-weight-medium); box-shadow: inset 0 0 0 1px transparent; }
    .ux-tab:hover { background: var(--theme-surface-2); border-color: var(--theme-outline); transform: translateY(-1px); }
    .ux-tab[aria-selected="true"], .ux-tab.active { background: var(--primary); border-color: var(--primaryHover); box-shadow: inset 0 -2px 0 var(--accent), var(--theme-elevation-1); }
    .ux-tab:focus-visible, .ux-button:focus-visible, .ux-card-button:focus-visible, .ux-field:focus-visible, .ux-select:focus-visible, .ux-textbox:focus-visible, .ux-toggle:focus-within, .ux-checkbox:focus-within { outline: var(--theme-focus-width) solid var(--theme-focus-color); outline-offset: var(--theme-focus-offset); box-shadow: var(--theme-highlight-ring); }
    .ux-tab[disabled], .ux-disabled { opacity: .45; cursor: not-allowed; transform: none; box-shadow: none; }
    .ux-tab-strip .ux-tab { display: inline-flex; align-items: center; gap: var(--theme-spacing-xs); }
    .ux-tab-star, .ux-tab-eye { display: inline-grid; place-items: center; min-width: 1.2em; color: var(--accent); font-weight: var(--theme-font-weight-bold); }
    .ux-tab-eye { color: var(--text-secondary); }
    .ux-tab-star.muted, .ux-tab-eye.muted { opacity: .52; color: var(--text-secondary); }
    .ux-tab-faded, .ux-tab[data-hidden-tab="true"] { opacity: .48; background: var(--hiddenTabBackground); color: var(--hiddenTabText); border-style: dashed; }
    .ux-button { min-height: var(--theme-control-height); padding: var(--theme-control-padding-y) var(--theme-control-padding-x); border: var(--theme-border-width) solid var(--theme-component-card-outline); border-radius: var(--theme-radius-md); background: var(--theme-component-button-container); color: var(--theme-component-button-on-container); font-weight: var(--theme-font-weight-bold); cursor: pointer; text-decoration: none; display: inline-flex; align-items: center; justify-content: center; gap: var(--theme-spacing-xs); box-shadow: var(--theme-elevation-1); position: relative; overflow: hidden; }
    .ux-button:hover { background: var(--theme-component-button-hover-container); border-color: var(--theme-outline); transform: translateY(-1px); box-shadow: var(--theme-elevation-2); }
    .ux-button:active { transform: translateY(0); box-shadow: var(--theme-elevation-1); }
    .ux-button.secondary { background: var(--theme-surface-1); color: var(--text); }
    .ux-button.secondary:hover { background: var(--theme-component-button-hover-container); border-color: var(--theme-outline); color: var(--theme-component-button-on-container); transform: translateY(-1px); box-shadow: var(--theme-elevation-2); }
    .ux-button.danger { background: var(--error); color: var(--background); }
    .ux-button.warning { background: var(--warning); color: var(--background); }
    .ux-button.success { background: var(--success); color: var(--background); }
    .ux-button.small { min-height: 28px; font-size: var(--theme-font-size-xs); padding: var(--theme-spacing-xs) var(--theme-spacing-sm); }
    .ux-button.large { min-height: 44px; font-size: var(--theme-font-size-lg); }
    .ux-card { background: var(--theme-component-card-container); border: var(--theme-border-width) solid var(--theme-component-card-outline); border-radius: var(--theme-card-radius); padding: var(--theme-card-padding); min-height: var(--theme-card-min-height); box-shadow: var(--theme-elevation-1); }
    .ux-card-button { width: 100%; text-align: left; color: var(--text); cursor: pointer; appearance: none; font: inherit; }
    .ux-card.clickable, .ux-card-button { cursor: pointer; border-color: var(--theme-outline-variant); box-shadow: var(--theme-elevation-1); }
    .ux-card.clickable:hover, .ux-card-button:hover { transform: translateY(-2px); border-color: var(--theme-outline); box-shadow: var(--theme-elevation-2); background: color-mix(in srgb, var(--theme-component-card-container) 88%, var(--theme-highlight-subtle)); }
    .ux-card.clickable:active, .ux-card-button:active { transform: translateY(0); box-shadow: var(--theme-elevation-1); }
    .ux-card.active { position: relative; border-color: var(--accent); background: color-mix(in srgb, var(--theme-component-card-container) 82%, var(--theme-highlight-strong)); box-shadow: inset 4px 0 0 var(--accent), var(--theme-elevation-2); }
    .ux-card.active::before { content: 'Selected'; display: inline-flex; width: max-content; margin-bottom: var(--theme-spacing-xs); padding: 2px 8px; border-radius: var(--theme-radius-pill); background: var(--accent); color: var(--background); font-size: var(--theme-font-size-xs); font-weight: var(--theme-font-weight-bold); letter-spacing: .06em; text-transform: uppercase; }
    .ux-card.error { border-color: var(--error); }
    .ux-field, .ux-select, .ux-textbox { width: 100%; padding: var(--theme-control-padding-y) var(--theme-control-padding-x); border: var(--theme-border-width) solid var(--theme-component-card-outline); border-radius: var(--theme-radius-md); background: var(--theme-surface-0); color: var(--text); font: inherit; box-shadow: inset 0 1px 0 rgba(255,255,255,.04); }
    .ux-field:hover, .ux-select:hover, .ux-textbox:hover { border-color: var(--theme-outline); background: var(--theme-surface-1); }
    .ux-field:focus, .ux-select:focus, .ux-textbox:focus { outline: none; box-shadow: var(--theme-highlight-ring); border-color: var(--theme-focus-color); }
    .ux-toggle, .ux-checkbox { display: inline-flex; align-items: center; gap: var(--theme-spacing-sm); min-height: 32px; padding: var(--theme-spacing-xs) var(--theme-spacing-sm); border-radius: var(--theme-radius-pill); cursor: pointer; color: var(--text); }
    .ux-toggle:hover, .ux-checkbox:hover { background: var(--theme-highlight-subtle); }
    .ux-toggle input, .ux-checkbox input { accent-color: var(--accent); cursor: pointer; }
    .ux-badge { display: inline-flex; align-items: center; justify-content: center; min-height: 24px; border-radius: var(--theme-radius-pill); border: var(--theme-border-width) solid var(--border); padding: 2px 8px; font-size: var(--theme-font-size-xs); text-transform: uppercase; letter-spacing: .06em; }
    .ux-badge-button { appearance: none; font: inherit; cursor: pointer; box-shadow: inset 0 0 0 1px transparent, var(--theme-elevation-1); }
    .ux-badge-button:hover { border-color: var(--theme-outline); transform: translateY(-1px); box-shadow: inset 0 0 0 1px var(--theme-highlight-strong), var(--theme-elevation-2); filter: brightness(1.08); }
    .ux-badge-button:active { transform: translateY(0); box-shadow: inset 0 0 0 1px var(--theme-outline), var(--theme-elevation-1); }
    .ux-badge-button:focus-visible { outline: var(--theme-focus-width) solid var(--theme-focus-color); outline-offset: var(--theme-focus-offset); box-shadow: var(--theme-highlight-ring); }
    .ux-badge-button[aria-pressed="true"] { border-color: var(--theme-focus-color); box-shadow: inset 0 0 0 1px var(--theme-focus-color), var(--theme-highlight-ring); }
    .ux-badge.primary { background: var(--primary); color: var(--text); }
    .ux-badge.secondary { background: var(--hiddenTabBackground); color: var(--text-secondary); }
    .ux-badge.success { background: var(--success); color: var(--background); }
    .ux-badge.warning { background: var(--warning); color: var(--background); }
    .ux-badge.danger { background: var(--error); color: var(--background); }
    .ux-badge.info { background: var(--accent); color: var(--background); }
    .ux-progress { width: 100%; height: 10px; overflow: hidden; border-radius: var(--theme-radius-pill); background: var(--hiddenTabBackground); border: var(--theme-border-width) solid var(--border); }
    .ux-progress > span { display: block; height: 100%; background: var(--accent); }
    .ux-table-shell { width: 100%; overflow: auto; border: var(--theme-border-width) solid var(--border); border-radius: var(--theme-card-radius); background: var(--surface); box-shadow: var(--theme-elevation-1); }
    .ux-table-toolbar { display: grid; grid-template-columns: minmax(180px, 1fr) auto auto; gap: var(--theme-spacing-sm); align-items: center; margin-bottom: var(--theme-spacing-sm); }
    .ux-table { width: 100%; border-collapse: collapse; background: var(--surface); overflow: hidden; }
    .ux-table caption { text-align: left; padding: var(--theme-spacing-sm) var(--theme-spacing-md); color: var(--text-secondary); font-weight: var(--theme-font-weight-bold); background: var(--theme-surface-1); border-bottom: var(--theme-border-width) solid var(--border); }
    .ux-table th, .ux-table td { padding: var(--theme-spacing-sm); border-bottom: var(--theme-border-width) solid var(--border); text-align: left; vertical-align: middle; }
    .ux-table th { color: var(--text-secondary); font-size: var(--theme-font-size-xs); text-transform: uppercase; letter-spacing: .06em; background: var(--theme-surface-1); }
    .ux-table tbody tr:hover { background: var(--theme-highlight-subtle); }
    .ux-table-dense th, .ux-table-dense td { padding: var(--theme-spacing-xs) var(--theme-spacing-sm); }
    .ux-table-actions td:last-child { text-align: right; white-space: nowrap; }
    .ux-table-subtext { display: block; color: var(--text-secondary); font-size: var(--theme-font-size-xs); margin-top: 2px; }
    .ux-table-sort { appearance: none; border: 0; background: transparent; color: inherit; display: inline-flex; align-items: center; gap: var(--theme-spacing-xs); font: inherit; font-weight: var(--theme-font-weight-bold); cursor: pointer; padding: 0; }
    .ux-table-sort:hover { color: var(--text); }
    .ux-row-selected { background: color-mix(in srgb, var(--theme-highlight-subtle) 72%, var(--theme-surface-1)); box-shadow: inset 3px 0 0 var(--accent); }
    .ux-table meter { width: min(140px, 100%); accent-color: var(--accent); }
    .ux-chart { display: grid; gap: var(--theme-spacing-sm); min-height: 280px; border: var(--theme-border-width) solid var(--border); border-radius: var(--theme-card-radius); padding: var(--theme-card-padding); background: linear-gradient(180deg, var(--theme-component-card-container), var(--theme-surface-1)); box-shadow: var(--theme-elevation-1); }
    .ux-chart figcaption { display: flex; justify-content: space-between; gap: var(--theme-spacing-sm); color: var(--text); }
    .ux-chart figcaption span, .ux-chart-legend { color: var(--text-secondary); font-size: var(--theme-font-size-sm); }
    .ux-chart-frame { width: 100%; min-height: 180px; overflow: visible; }
    .ux-chart-grid line { stroke: var(--border); stroke-dasharray: 4 4; opacity: .85; }
    .ux-chart-axis line, .ux-chart-axis text, .ux-chart-labels text { stroke: var(--text-secondary); fill: var(--text-secondary); font-size: 11px; }
    .ux-chart-area { fill: var(--theme-highlight-subtle); opacity: .72; }
    .ux-chart-line-path, .ux-sparkline polyline { fill: none; stroke: var(--accent); stroke-width: 4; stroke-linecap: round; stroke-linejoin: round; vector-effect: non-scaling-stroke; }
    .ux-chart-point { fill: var(--accent); stroke: var(--background); stroke-width: 2; }
    .ux-chart-bars rect { fill: var(--primary); rx: 6; transition: opacity var(--theme-transition-fast), transform var(--theme-transition-fast); }
    .ux-chart-bars rect:hover { opacity: .78; transform: translateY(-2px); }
    .ux-chart-legend { display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: var(--theme-spacing-sm); }
    .ux-chart-legend span { display: inline-flex; align-items: center; gap: var(--theme-spacing-xs); }
    .ux-chart-legend i { width: 10px; height: 10px; border-radius: 999px; background: var(--accent); display: inline-block; }
    .ux-chart-legend i.secondary { background: var(--primary); }
    .ux-chart-legend i.muted { background: var(--hiddenTabBackground); border: 1px solid var(--border); }
    .ux-donut { --ux-donut-a: 60%; --ux-donut-b: 78%; width: min(180px, 70vw); aspect-ratio: 1; border-radius: 50%; margin: 0 auto; display: grid; place-items: center; background: conic-gradient(var(--accent) 0 var(--ux-donut-a), var(--primary) var(--ux-donut-a) var(--ux-donut-b), var(--hiddenTabBackground) var(--ux-donut-b) 100%); position: relative; box-shadow: var(--theme-elevation-2); }
    .ux-donut::after { content: ''; position: absolute; inset: 22%; border-radius: 50%; background: var(--theme-component-card-container); border: var(--theme-border-width) solid var(--border); }
    .ux-donut span { position: relative; z-index: 1; font-size: var(--theme-font-size-xl); font-weight: var(--theme-font-weight-bold); color: var(--text); }
    .ux-sparkline { width: 100%; min-height: 44px; background: var(--theme-surface-1); border-radius: var(--theme-radius-md); padding: var(--theme-spacing-xs); }
    .ux-sparkline.warning polyline { stroke: var(--warning); }
    .ux-readout { white-space: pre-wrap; background: var(--hiddenTabBackground); border: var(--theme-border-width) solid var(--border); border-radius: var(--theme-radius-md); padding: var(--theme-card-padding); color: var(--text); font-family: var(--theme-font-mono); max-height: 240px; overflow: auto; }
    .ux-breadcrumbs { display: flex; flex-wrap: wrap; gap: var(--theme-spacing-xs); color: var(--text-secondary); }
    .ux-modal-sample { border: var(--theme-border-width) solid var(--border); border-radius: var(--theme-modal-radius); padding: var(--theme-card-padding); background: var(--surface); box-shadow: 0 16px 40px rgba(0,0,0,.35); }
    .test-tablet { padding: var(--theme-content-padding); background: var(--background); min-height: 70vh; }
    .test-tablet-content, .component-showcase { max-width: 1400px; margin: 0 auto; }
    .ux-panel { display: none; }
    .ux-panel.active { display: grid; gap: var(--theme-grid-gap); }
    .ux-component-card { align-content: start; }
    .ux-component-sample { display: grid; gap: var(--theme-spacing-sm); }

    .ux-gradient-swatch { min-height: 84px; border-radius: var(--theme-radius-lg); border: 1px solid var(--theme-outline-variant); box-shadow: var(--theme-elevation-1); }
    .ux-gradient-swatch.primary { background: var(--theme-gradient-primary); }
    .ux-gradient-swatch.accent { background: var(--theme-gradient-accent); }
    .ux-gradient-swatch.surface { background: var(--theme-gradient-surface); }
    .ux-gradient-swatch.highlight { background: var(--theme-gradient-highlight), var(--theme-surface-1); }
    .ux-highlight-card { border-color: var(--theme-highlight-strong); background: var(--theme-highlight-subtle); box-shadow: var(--theme-highlight-ring); }
    .ux-accent-strip { display: grid; grid-template-columns: repeat(4, minmax(0,1fr)); min-height: 44px; overflow: hidden; border-radius: var(--theme-radius-md); border: 1px solid var(--theme-outline-variant); }
    .ux-accent-strip span:nth-child(1) { background: var(--theme-accent-warm); }
    .ux-accent-strip span:nth-child(2) { background: var(--theme-accent-cool); }
    .ux-accent-strip span:nth-child(3) { background: var(--theme-accent-neutral); }
    .ux-accent-strip span:nth-child(4) { background: var(--theme-accent-critical); }
    .ux-role-pair { display: grid; gap: var(--theme-spacing-xs); padding: var(--theme-spacing-md); border-radius: var(--theme-radius-md); border: 1px solid var(--theme-outline-variant); background: var(--theme-component-card-container); color: var(--theme-on-surface); }
    .ux-role-pair.primary { background: var(--theme-role-primary); color: var(--theme-role-on-primary); }
    .ux-state-layer { position: relative; overflow: hidden; }
    .ux-state-layer::after { content: ''; position: absolute; inset: 0; background: var(--theme-focus-color); opacity: var(--theme-state-hover-opacity); pointer-events: none; }
    .showcase-item h4, .ux-card h3, .ux-card h4 { margin: 0 0 var(--theme-spacing-sm); }
    .theme-token-table { font-family: var(--theme-font-mono); font-size: var(--theme-font-size-xs); }
"####
}
