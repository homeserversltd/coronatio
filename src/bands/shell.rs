fn render_crown_shell() -> String {
    let nav = render_flask_react_tabbar_quarry();

    let shell = r###"<!doctype html>
<html lang="en" class="theme-loaded">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Coronatio</title>
  <style>
    :root {
      color-scheme: dark;
      --theme-color-primary: #00f2fe;
      --theme-color-secondary: #4CAF50;
      --theme-bg-primary: #2a2a2a;
      --theme-bg-secondary: #1a1a1a;
      --theme-bg-tertiary: #222222;
      --theme-bg-hover: #333333;
      --theme-bg-active: #3a3a3a;
      --theme-text-primary: #ffffff;
      --theme-text-secondary: #dddddd;
      --theme-text-tertiary: #a7a7a7;
      --theme-text-disabled: #777777;
      --theme-text-accent: #00f2fe;
      --theme-status-success: #4CAF50;
      --theme-status-error: #f44336;
      --theme-status-warning: #ff9800;
      --theme-status-info: #2196f3;
      --theme-spacing-xs: 0.25rem;
      --theme-spacing-sm: 0.5rem;
      --theme-spacing-md: 1rem;
      --theme-spacing-lg: 1.5rem;
      --theme-spacing-xl: 2rem;
      --theme-font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --theme-font-mono: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
      --theme-font-size-xs: 12px;
      --theme-font-size-sm: 0.875rem;
      --theme-font-size-base: 16px;
      --theme-font-size-md: 16px;
      --theme-font-size-lg: 1.125rem;
      --theme-font-size-xl: 24px;
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
      --background: var(--theme-bg-secondary);
      --surface: var(--theme-bg-primary);
      --surface-soft: var(--theme-bg-tertiary);
      --text: var(--theme-text-primary);
      --text-secondary: var(--theme-text-secondary);
      --accent: var(--theme-color-primary);
      --accent-soft: color-mix(in srgb, var(--theme-color-primary) 16%, transparent);
      --primary: var(--theme-color-secondary);
      --border: color-mix(in srgb, var(--theme-text-primary) 14%, transparent);
      --error: var(--theme-status-error);
      --warning: var(--theme-status-warning);
      --success: var(--theme-status-success);
      --shadow: var(--theme-shadow-md);
      font-family: var(--theme-font-family);
      background: var(--background);
      color: var(--text);
    }
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; background: var(--background); color: var(--text); }
    .app { min-height: 100vh; display: flex; flex-direction: column; }
    .top-bar {
      min-height: 48px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      padding: 0 20px;
      background: var(--surface);
      border-bottom: 1px solid var(--border);
      box-shadow: var(--shadow);
    }
    .header-top-row { display: flex; align-items: center; justify-content: space-between; gap: 1rem; min-width: 0; flex: 1 1 auto; }
    .header-left, .header-center, .header-right { display: flex; align-items: center; gap: .5rem; }
    .header-center { justify-content: center; flex: 2 1 auto; min-width: 0; }
    .header-right { justify-content: flex-end; flex-wrap: wrap; }
    .uptime { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: .82rem; padding: .24rem .5rem; border-radius: 6px; background: rgba(255,255,255,.06); color: var(--text); }
    .status-indicators { display: flex; flex-direction: row; align-items: center; justify-content: center; gap: 8px; }
    .indicator { cursor: pointer; padding: .25rem; display: flex; align-items: center; justify-content: center; border: 0; border-radius: 6px; background: transparent; color: var(--text); transition: transform .2s ease, background-color .2s ease, box-shadow .2s ease; }
    .indicator:hover { background-color: var(--primary-hover); transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,0,0,.15); }
    .indicator:active { transform: translateY(1px); box-shadow: 0 1px 2px rgba(0,0,0,.1); }
    .indicator-icon { width: 1.15rem; height: 1.15rem; display: block; fill: currentColor; }
    .indicator.ok .indicator-icon { color: var(--success); }
    .indicator.warn .indicator-icon { color: var(--warning); }
    .modal-body { display: grid; gap: .75rem; color: var(--text-secondary); }
    .modal-body ul { margin: .25rem 0 0; padding-left: 1.15rem; }
    .modal-section, .status-section, .config-section, .credentials-section, .service-controls, .power-history-section, .speed-test-section { display: grid; gap: .55rem; }
    .modal-grid { display: grid; gap: .55rem; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); }
    .modal-body input { width: 100%; padding: .55rem .65rem; border: 1px solid var(--border); border-radius: 6px; background: var(--background); color: var(--text); }
    .status-item, .power-average-row, .current-tailnet { display: flex; justify-content: space-between; gap: .75rem; border-bottom: 1px solid rgba(255,255,255,.08); padding-bottom: .35rem; }
    .status-value, .power-value, .power-average-value, .value { color: var(--text); font-weight: 700; }
    .action-output:empty { display: none; }
    .theme-choice-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-top: .5rem; }
    .theme-choice-row[hidden] { display: none !important; }
    .theme-choice { min-width: 90px; }
    .header-control, .admin-button, .theme-button, .change-admin-pin-button { min-height: 34px; min-width: 120px; padding: 0 .9rem; border: none; border-radius: 6px; background: var(--primary); color: #061006; font-size: .86rem; font-weight: 700; box-shadow: inset 0 2px 4px rgba(0,0,0,.2); }
    .header-control:hover, .admin-button:hover, .theme-button:hover, .change-admin-pin-button:hover { filter: brightness(1.08); transform: translateY(-1px); }
    .modal-backdrop { position: fixed; inset: 0; display: none; place-items: center; background: rgba(0,0,0,.55); z-index: 2000; padding: 1rem; }
    .modal-backdrop.open { display: grid; }
    .modal { width: min(420px, 100%); background: var(--surface); border: 1px solid var(--border); border-radius: 10px; box-shadow: 0 16px 40px rgba(0,0,0,.45); padding: 1rem; }
    .modal h2 { margin: 0 0 .75rem; font-size: 1.05rem; }
    .pin-modal { display: flex; flex-direction: column; gap: 16px; padding: 16px 0 0; }
    .pin-modal input { padding: 8px 12px; border: 1px solid var(--border); border-radius: 4px; background: var(--background); color: var(--text); font-size: 14px; outline: none; }
    .pin-modal input:focus { border-color: var(--primary); }
    .modal-actions { display: flex; justify-content: flex-end; gap: .5rem; margin-top: 1rem; }
    .toast-slot { min-height: 1.2rem; color: var(--warning); font-size: .84rem; margin-top: .5rem; }
    .status-strip { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: .45rem; color: var(--text-secondary); font-size: .82rem; }
    .status-pill { border: 1px solid var(--border); border-radius: 999px; padding: .18rem .55rem; background: rgba(255,255,255,.045); }
    .status-pill.ok { color: var(--success); border-color: color-mix(in srgb, var(--success) 45%, transparent); }
    .tab-bar {
      min-height: 48px;
      display: flex;
      align-items: center;
      gap: .5rem;
      padding: 0 20px;
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
      border-radius: 8px;
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
    .content { flex: 1; padding: 20px; }
    .pane { display: none; max-width: 1180px; margin: 0 auto; }
    .pane.active { display: block; }
    .pane-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(270px, 1fr)); gap: 16px; align-items: start; }
    .portal-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px; align-items: stretch; }
    .card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 1rem;
      box-shadow: var(--shadow);
      min-height: 112px;
    }
    .portal-card { min-height: 180px; display: flex; flex-direction: column; justify-content: space-between; gap: .9rem; }
    .card h2, .card h3 { margin: 0 0 .65rem; font-size: 1rem; color: var(--text); }
    .card p { margin: .25rem 0; color: var(--text-secondary); font-size: .92rem; }
    .metric { font-size: 1.65rem; font-weight: 750; color: var(--accent); line-height: 1.1; }
    .stats-viewport { display: grid; gap: 16px; }
    .stats-section { background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 1rem; box-shadow: var(--shadow); }
    .stats-section h2 { margin: 0 0 .75rem; font-size: 1rem; }
    .stats-resource-grid, .drives-grid, .network-grid, .services-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: .75rem; }
    .stats-resource-card, .drive-info, .network-interface, .service-info, .connections-summary { background: rgba(0,0,0,.18); border: 1px solid rgba(255,255,255,.08); border-radius: 8px; padding: .75rem; }
    .stats-resource-card h3, .drive-info h3, .network-interface h3, .service-info h3 { margin: 0 0 .45rem; font-size: .95rem; }
    .chart-container { position: relative; width: 100%; height: 200px; margin: .5rem 0 1rem; }
    .cpu-gauge-container { position: relative; width: 100%; height: 170px; margin: .35rem 0 .75rem; }
    .cpu-temp { position: absolute; inset: auto 0 16px; text-align: center; font-weight: 800; color: var(--accent); pointer-events: none; }
    .cpu-details { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: .35rem; color: var(--text-secondary); font-size: .78rem; text-align: center; }
    .chart-fallback { color: var(--warning); font-size: .82rem; margin: .5rem 0; }
    .progress-bar { width: 100%; height: 8px; background: var(--background); border-radius: 999px; overflow: hidden; margin: .55rem 0; }
    .progress-bar .progress { height: 100%; width: 0%; background: var(--accent); transition: width .18s ease; }
    .details, .traffic, .counts, .service-info .details { display: flex; justify-content: space-between; gap: .65rem; color: var(--text-secondary); font-size: .84rem; flex-wrap: wrap; }
    .status-dot { display: inline-block; width: .55rem; height: .55rem; border-radius: 50%; margin-right: .35rem; background: var(--warning); }
    .status-dot.up, .status-dot.running { background: var(--success); }
    .status-dot.down, .status-dot.stopped { background: var(--error); }
    .stats-transport-card .button-row { margin-top: .65rem; }
    .muted { color: var(--text-secondary); }
    .readout { margin-top: .75rem; padding: .75rem; border-radius: 6px; background: rgba(0,0,0,.22); border: 1px solid rgba(255,255,255,.08); font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: .78rem; color: #cdefff; overflow: auto; max-height: 220px; white-space: pre-wrap; }
    .button-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-top: .85rem; }
    .admin-tablet { display: flex; flex-direction: column; gap: .5rem; color: var(--text); }
    .admin-visual-port { display: flex; flex-direction: column; gap: .5rem; }
    .admin-tablet .system-controls { display: flex; flex-wrap: wrap; gap: 1rem; width: 100%; margin: 0 auto; box-shadow: 0 2px 0 var(--border); justify-content: center; padding-bottom: .7rem; }
    .system-controls-container, .admin-tablet .key-manager, .admin-tablet .disk-manager, .admin-modal-shelf { background-color: var(--background); border-radius: 8px; box-shadow: 0 2px 4px var(--primary), 0 2px 4px var(--border); padding: 15px; color: var(--text); }
    .system-controls-btn { display: flex; justify-content: center; align-items: center; gap: 10px; padding: 10px 15px; border-radius: 4px; border: none; background: var(--primary); color: #fff; font-size: .9rem; font-weight: 500; min-width: 180px; width: 180px; flex: 0 0 auto; transition: background-color .2s, transform .1s, box-shadow .2s; position: relative; overflow: hidden; box-shadow: inset 0 2px 4px rgba(0,0,0,.1), 0 2px 4px rgba(0,0,0,.1); }
    .system-controls-btn::before { content: ''; position: absolute; inset: 0; background: linear-gradient(to bottom, rgba(255,255,255,.15) 0%, rgba(0,0,0,.1) 100%); mix-blend-mode: overlay; pointer-events: none; opacity: .6; }
    .system-controls-btn:hover { transform: translateY(-2px); background-color: var(--primaryHover); box-shadow: inset 0 2px 4px rgba(0,0,0,.1), 0 4px 8px rgba(0,0,0,.15); }
    .system-controls-btn span { position: relative; text-shadow: 0 1px 1px rgba(0,0,0,.2); }
    .admin-tablet h3, .admin-tablet h4 { color: var(--text); margin: 0 0 .75rem; }
    .admin-tablet .key-manager { padding: 20px; margin-bottom: .5rem; }
    .key-manager-content { display: flex; gap: 30px; }
    .key-manager-left { flex: 2; }
    .key-manager-right { flex: 1; min-width: 250px; }
    .security-status, .key-actions { display: flex; flex-direction: column; gap: 15px; }
    .key-actions { position: sticky; top: 20px; }
    .status-item { display: flex; align-items: flex-start; gap: 12px; padding: 15px; background-color: var(--border); border-radius: 8px; border-left: 4px solid transparent; border-color: var(--border); transition: transform .2s ease, box-shadow .2s ease; }
    .status-item:hover { transform: translateX(5px); }
    .status-icon { font-size: 1.5rem; margin-top: 2px; color: var(--success); }
    .key-manager .action-button, .disk-actions .action-button, .modal-button, .refresh-button, .tab-button { color: var(--text); cursor: pointer; font-weight: bold; transition: all .2s ease; }
    .key-manager .action-button { width: 100%; padding: 15px; border-radius: 8px; border: none; display: flex; align-items: center; justify-content: center; gap: 10px; background-color: var(--primary); }
    .key-manager .action-button:hover { background-color: var(--primaryHover); transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,.2); }
    .info-button { display: inline-flex; width: auto; margin-left: .5rem; padding: 6px 10px; border-radius: 6px; background: var(--primary); color: var(--text); border: 1px solid var(--border); }
    .admin-tablet .disk-manager { padding: 20px; }
    .disk-manager-container { display: flex; gap: 20px; margin-bottom: 20px; }
    .disk-column { flex: 1; min-width: 0; }
    .disk-list { display: flex; flex-direction: column; gap: 10px; }
    .disk-item { background-color: var(--primary); border-radius: 8px; padding: 15px; position: relative; transition: all .2s ease; border: 2px solid transparent; }
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
    .modal-window { background-color: var(--background); border: 1px solid var(--border); border-radius: 8px; box-shadow: 0 4px 16px rgba(0,0,0,.25); overflow: hidden; flex-direction: column; min-height: 190px; }
    .modal-titlebar { background-color: var(--primaryHover); border-bottom: 1px solid var(--border); padding: 10px 14px; font-weight: 700; display: flex; align-items: center; justify-content: space-between; gap: 10px; }
    .modal-body-port { padding: 14px; display: flex; flex-direction: column; gap: 12px; }
    .modal-actions, .modal-toolbar, .modal-pager, .update-actions { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: 10px; margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--border); }
    .modal-toolbar { justify-content: flex-start; border-top: none; margin-top: 0; padding-top: 0; }
    .modal-button, .refresh-button { padding: 10px 15px; border-radius: 5px; border: 1px solid var(--border); background-color: var(--primary); display: inline-flex; align-items: center; gap: 8px; }
    .modal-button:hover, .refresh-button:hover { background-color: var(--primaryHover); transform: translateY(-1px); }
    .view-tabs { display: flex; gap: 5px; position: relative; bottom: -1px; }
    .tab-button { padding: 8px 16px; border: 1px solid var(--border); border-bottom: none; background-color: var(--primary); color: var(--text); border-radius: 6px 6px 0 0; }
    .tab-button.active { background-color: var(--background); z-index: 2; }
    .update-status-container { display: flex; align-items: center; gap: 12px; padding: 15px 20px; border-radius: 6px; font-weight: 600; font-size: 1.1rem; background-color: var(--primaryHover); color: var(--warning); border: 1px solid var(--warning); }
    .modules-table, .log-frame { width: 100%; border-collapse: collapse; background-color: var(--primary); border-radius: 6px; overflow: hidden; }
    .modules-table th, .modules-table td { padding: 8px; border-bottom: 1px solid var(--border); text-align: left; font-size: .85rem; }
    .log-frame { min-height: 76px; padding: 10px; font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; color: var(--secondary); }
    .subscription-debug-toggle, .subscription-debug-panel { background-color: var(--background); border: 1px solid var(--border); border-radius: 8px; padding: 12px; box-shadow: 0 2px 4px var(--primary), 0 2px 4px var(--border); }
    .subscription-debug-header { display: flex; align-items: center; justify-content: space-between; gap: 10px; }
    .admin-quarry-note { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; border: 0; }
    @media (max-width: 768px) { .key-manager-content, .disk-manager-container { flex-direction: column; } .system-controls-btn { width: 100%; max-width: 180px; } .admin-modal-shelf { grid-template-columns: 1fr; } }
    button, .action-link { border: 1px solid var(--border); border-radius: 6px; padding: .55rem .7rem; background: var(--primary); color: #061006; font-weight: 700; cursor: pointer; text-decoration: none; }
    button.secondary, .action-link.secondary { background: transparent; color: var(--text); }
    .warning { color: var(--warning); }
    .error { color: var(--error); }
    .success { color: var(--success); }
    .drop-zone { border: 1px dashed color-mix(in srgb, var(--accent) 55%, transparent); border-radius: 8px; padding: 1.2rem; background: rgba(0,242,254,.07); }
    [data-admin-mode="false"] [data-admin-only]:not([data-admin-only="false"]) { display: none !important; }
    [data-admin-mode="true"] [data-admin-only]:not([data-admin-only="false"]) { display: revert; }
    [data-admin-only]:not([data-admin-only="false"]) { }
    @media (max-width: 760px) {
      .top-bar { align-items: stretch; flex-direction: column; padding: .75rem 1rem; }
      .header-top-row { flex-direction: column; align-items: stretch; }
      .header-left, .header-center, .header-right { justify-content: center; }
      .theme-button, .change-admin-pin-button, .admin-button { flex: 1 1 auto; min-width: 0; }
      .tab-bar, .content { padding-left: 12px; padding-right: 12px; }
      .portal-grid, .pane-grid { grid-template-columns: 1fr; }
    }
  </style>
  <script src="/static/vendor/chart.umd.min.js" data-chart-dependency="chartjs-4.4.0"></script>
  <script src="/static/vendor/chartjs-plugin-datalabels.min.js" data-chart-dependency="chartjs-plugin-datalabels-2.2.0"></script>
</head>
<body>
  <main class="app" data-product="Coronatio" data-source-material="homeserver-main-site" data-admin-mode="false">
    <header class="top-bar header" data-flask-react-quarry="Header">
      <div class="header-top-row">
        <div class="header-left"><span class="uptime" data-uptime-indicator title="Server uptime">connecting...</span></div>
        <div class="header-center">
          <div class="status-indicators" aria-label="Status indicators">
            <button type="button" class="indicator ok tailscale-indicator" data-indicator="tailscale" data-modal-kind="tailscale" data-modal-title="Tailscale Status" aria-label="Tailscale Status" title="Tailscale Status"><svg class="indicator-icon" data-packed-icon="network-wired" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 5h16v8H4V5zm2 2v4h12V7H6zm5 8h2v2h7v2h-7v2h-2v-2H4v-2h7v-2z"/></svg></button>
            <button type="button" class="indicator ok internet-indicator" data-indicator="internet" data-modal-kind="internet" data-modal-title="Internet Status" aria-label="Internet Status" title="Internet Status"><svg class="indicator-icon" data-packed-icon="plug" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 2h2v5h2V2h2v5h2V2h2v7a5 5 0 0 1-4 4.9V17h3v2h-3v3h-2v-3H8v-2h3v-3.1A5 5 0 0 1 7 9V2z"/></svg></button>
            <button type="button" class="indicator warn openvpn-indicator" data-indicator="openvpn" data-modal-kind="openvpn" data-modal-title="VPN & Transmission Configuration" aria-label="VPN & Transmission Configuration" title="VPN & Transmission Configuration"><svg class="indicator-icon" data-packed-icon="lock" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 10V8a5 5 0 0 1 10 0v2h2v11H5V10h2zm2 0h6V8a3 3 0 0 0-6 0v2zm2 4v3h2v-3h-2z"/></svg></button>
            <button type="button" class="indicator ok services-indicator" data-indicator="services" data-modal-kind="services" data-modal-title="Services Status" aria-label="Services Status" title="Services Status"><svg class="indicator-icon" data-packed-icon="server" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 3h16v8H4V3zm2 2v4h12V5H6zm-2 8h16v8H4v-8zm2 2v4h12v-4H6zm9-8h2v2h-2V7zm0 10h2v2h-2v-2z"/></svg></button>
            <button type="button" class="indicator warn power-indicator" data-indicator="power-meter" data-modal-kind="power-meter" data-modal-title="Power Consumption" aria-label="Power Consumption" title="Power Consumption"><svg class="indicator-icon" data-packed-icon="bolt" viewBox="0 0 24 24" aria-hidden="true"><path d="M13 2 4 14h7l-1 8 10-13h-7l1-7z"/></svg></button>
          </div>
        </div>
        <div class="header-right">
          <button type="button" class="theme-button" data-theme-button title="Current theme: dark. Click to switch theme."><span>dark</span></button>
          <button type="button" class="change-admin-pin-button" data-change-pin-button hidden>Change PIN</button>
          <button type="button" class="admin-button" data-admin-button data-admin-state="logged-out">Enter Admin Mode</button>
        </div>
      </div>
    </header>
    <div class="modal-backdrop" data-pin-modal-backdrop aria-hidden="true">
      <section class="modal" role="dialog" aria-modal="true" aria-labelledby="pin-modal-title">
        <h2 id="pin-modal-title">Enter Admin Mode</h2>
        <form class="pin-modal" data-pin-modal-form>
          <input type="text" autocomplete="username" value="admin" readonly style="display:none" aria-hidden="true">
          <input data-pin-current type="password" placeholder="Enter PIN" autocomplete="current-password">
          <input data-pin-change-current type="password" placeholder="Current PIN" autocomplete="current-password" hidden>
          <input data-pin-new type="password" placeholder="New PIN" autocomplete="new-password" hidden>
          <input data-pin-confirm type="password" placeholder="Confirm new PIN" autocomplete="new-password" hidden>
        </form>
        <div class="toast-slot" data-pin-modal-message></div>
        <div class="modal-actions"><button type="button" class="secondary" data-pin-cancel>Cancel</button><button type="button" data-pin-confirm-button>Confirm</button></div>
      </section>
    </div>
    <div class="modal-backdrop" data-info-modal-backdrop aria-hidden="true">
      <section class="modal" role="dialog" aria-modal="true" aria-labelledby="info-modal-title">
        <h2 id="info-modal-title">Status</h2>
        <div class="modal-body" data-info-modal-body></div>
        <div class="theme-choice-row" data-theme-choice-row hidden data-theme-json-source="/api/themes"></div>
        <div class="modal-actions"><button type="button" class="secondary" data-info-modal-close>Close</button></div>
      </section>
    </div>
    <nav class="tab-bar" aria-label="Coronatio primary tabs" role="tablist" data-admin-mode="false" data-hidden="false">__NAV__</nav>
    <section class="content">
      <section class="pane active" id="pane-admin" data-pane-panel="admin" role="tabpanel" aria-label="Admin">

        <div class="admin-tablet admin-visual-port" data-admin-quarry="flask-react-admin" data-admin-quarry-button-total="90" data-admin-only="true" data-admin-viewport="admin" data-admin-visual-port="one-to-one-best-effort">
          <section class="system-controls-container" data-admin-quarry-group="system-controls" aria-label="System controls">
            <div class="system-controls">
              <span class="admin-quarry-note" data-admin-quarry-count-readback>90 buttons</span>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="1" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Hard Drive Test</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="2" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Update</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="3" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Restart</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="4" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Shutdown</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="5" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Restart Website</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="6" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>View Logs</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="7" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span>Install Certificate</span></button>
            </div>
            <span class="admin-quarry-note">7 buttons · system-controls</span>
          </section>

          <section class="key-manager" data-admin-quarry-group="key-manager">
            <h3>Key Manager</h3>
            <div class="key-manager-content">
              <div class="key-manager-left">
                <div class="security-status">
                  <div class="status-item"><span class="status-icon">🛡</span><div class="status-details"><p>Service Suite Key protects NAS drives, the system vault, and HOMESERVER stored secrets. <button type="button" class="info-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="20" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>View Full Guide &amp; Critical Warnings</span></button></p></div></div>
                </div>
              </div>
              <div class="key-manager-right"><div class="key-actions">
                <button type="button" class="action-button create-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="21" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Create New Key</span></button>
                <button type="button" class="action-button update-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="22" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Update Key on Drive</span></button>
                <button type="button" class="action-button admin-password-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="23" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Admin Password</span></button>
              </div></div>
            </div>
            <span class="admin-quarry-note">4 buttons · key-manager</span>
          </section>

          <section class="disk-manager" data-admin-quarry-group="disk-manager">
            <h3>Disk Manager</h3>
            <div class="disk-manager-container">
              <div class="disk-column"><h4>Available Drives</h4><div class="disk-list">
                <div class="disk-item selected available nas-compatible"><span class="disk-icon">▣</span><div class="disk-info"><div class="disk-name">/dev/sdb <span class="nas-badge">NAS</span></div><div class="disk-serial">SERIAL HOMESERVER-QUARRY</div><div class="disk-details">ext4 · mounted · encrypted</div><div class="disk-space-usage"><strong>1.8 TB</strong> available</div></div></div>
                <div class="disk-item locked-pair"><span class="disk-icon">🔒</span><div class="disk-info"><div class="disk-name">/dev/sdc <span class="nas-role-badge nas-role-backup">BACKUP</span></div><div class="disk-serial">SERIAL BACKUP-PAIR</div><div class="disk-details">locked pair · needs unlock</div></div></div>
              </div></div>
              <div class="disk-column"><h4>Drive Actions</h4><div class="disk-actions">
                <button type="button" class="action-button danger-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="8" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Format Drive</span></button><button type="button" class="action-button danger-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="9" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Encrypt Drive</span></button><button type="button" class="action-button success-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="10" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Assign as primary NAS</span></button><button type="button" class="action-button success-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="11" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Assign as NAS Backup</span></button><button type="button" class="action-button danger-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="12" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Unassign drive</span></button><button type="button" class="action-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="13" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Import to NAS</span></button><button type="button" class="action-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="14" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span>Fix Permissions</span></button><button type="button" class="action-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="15" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="8" data-stub-action="true" aria-disabled="true" ><span>Unlock</span></button><button type="button" class="action-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="16" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="9" data-stub-action="true" aria-disabled="true" ><span>Mount</span></button><button type="button" class="action-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="17" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="10" data-stub-action="true" aria-disabled="true" ><span>Unmount</span></button><button type="button" class="action-button success-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="18" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="11" data-stub-action="true" aria-disabled="true" ><span>Sync</span></button><button type="button" class="action-button success-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="19" data-admin-quarry-source="components/DiskManager.tsx" data-admin-quarry-placement="drive card action stack" data-admin-quarry-local-index="12" data-stub-action="true" aria-disabled="true" ><span>Auto Sync Schedule</span></button>
              </div></div>
            </div>
            <span class="admin-quarry-note">12 buttons · disk-manager</span>
          </section>

          <section class="subscription-debug-panel" data-admin-quarry-group="debug-subscriptions">
            <div class="subscription-debug-header"><h3>WebSocket Subscriptions</h3><div><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="24" data-admin-quarry-source="components/DebugSubscriptions.tsx" data-admin-quarry-placement="debug drawer controls" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Show Subscriptions</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="25" data-admin-quarry-source="components/DebugSubscriptions.tsx" data-admin-quarry-placement="debug drawer controls" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Refresh</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="26" data-admin-quarry-source="components/DebugSubscriptions.tsx" data-admin-quarry-placement="debug drawer controls" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Hide</span></button></div></div>
            <div class="subscription-stats"><div>Total: 0</div><div>Active: 0</div></div>
            <span class="admin-quarry-note">3 buttons · debug-subscriptions</span>
          </section>

          <section class="admin-modal-shelf" aria-label="Admin modal controls">
            <article class="modal-window" data-admin-quarry-group="admin-password-modal"><div class="modal-titlebar">Admin Password <span class="admin-quarry-note">2 buttons · admin-password-modal</span></div><div class="modal-body-port"><label class="form-group">New password<input class="field" value="••••••" readonly></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="27" data-admin-quarry-source="components/modals/AdminPasswordModal.tsx" data-admin-quarry-placement="admin password modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="28" data-admin-quarry-source="components/modals/AdminPasswordModal.tsx" data-admin-quarry-placement="admin password modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Update Password</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="create-key-modal"><div class="modal-titlebar">Create New Key <span class="admin-quarry-note">2 buttons · create-key-modal</span></div><div class="modal-body-port"><label class="radio-option selected"><input type="radio" checked readonly><span class="radio-label"><span class="radio-label-title">NAS drive key</span></span></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="29" data-admin-quarry-source="components/modals/CreateKeyModal.tsx" data-admin-quarry-placement="create key modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="30" data-admin-quarry-source="components/modals/CreateKeyModal.tsx" data-admin-quarry-placement="create key modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Create Key</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="hard-drive-test-modal"><div class="modal-titlebar">Hard Drive Test <span class="admin-quarry-note">6 buttons · hard-drive-test-modal</span></div><div class="modal-body-port"><div class="modal-toolbar"><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="31" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Refresh Devices</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="32" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Run New Test</span></button></div><div class="update-status-container">Device test ready</div><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="33" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Run New Test</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="34" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>View Results</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="35" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Back to Results</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="36" data-admin-quarry-source="components/modals/HardDriveTestModal.tsx" data-admin-quarry-placement="hard-drive-test modal toolbar and footer" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Start Test</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="log-viewer-modal"><div class="modal-titlebar">Log Viewer <span class="admin-quarry-note">6 buttons · log-viewer-modal</span></div><div class="modal-body-port"><div class="modal-toolbar"><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="37" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Refresh logs</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="38" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Copy logs to clipboard</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="39" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Download logs</span></button> <button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="40" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Clear logs</span></button></div><pre class="log-frame">system log readback preview...</pre><div class="modal-pager"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="41" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Previous</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="42" data-admin-quarry-source="components/modals/LogViewerModal.tsx" data-admin-quarry-placement="log viewer toolbar and pager" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Next</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="password-input-modal"><div class="modal-titlebar">Password Required <span class="admin-quarry-note">3 buttons · password-input-modal</span></div><div class="modal-body-port"><label class="form-group">Password<input class="field" type="password" value="••••" readonly></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="43" data-admin-quarry-source="components/modals/PasswordInputModal.tsx" data-admin-quarry-placement="password prompt controls" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Show/Hide password</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="44" data-admin-quarry-source="components/modals/PasswordInputModal.tsx" data-admin-quarry-placement="password prompt controls" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="45" data-admin-quarry-source="components/modals/PasswordInputModal.tsx" data-admin-quarry-placement="password prompt controls" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Submit</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="premium-tab-modal"><div class="modal-titlebar">Premium Tab Manager <span class="admin-quarry-note">16 buttons · premium-tab-modal</span></div><div class="modal-body-port"><div class="modal-toolbar"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="46" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Back</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="47" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="48" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Validate &amp; Clone</span></button></div><pre class="log-frame">repository clone/install log preview...</pre><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="49" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Copy logs to clipboard</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="50" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Refresh logs</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="51" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Back</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="52" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="53" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="8" data-stub-action="true" aria-disabled="true" ><span>Confirm</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="54" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="9" data-stub-action="true" aria-disabled="true" ><span>Add Repository</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="55" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="10" data-stub-action="true" aria-disabled="true" ><span>Install All</span></button><button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="56" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="11" data-stub-action="true" aria-disabled="true" ><span>Uninstall All</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="57" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="12" data-stub-action="true" aria-disabled="true" ><span>View Logs</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="58" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="13" data-stub-action="true" aria-disabled="true" ><span>Refresh</span></button><button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="59" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="14" data-stub-action="true" aria-disabled="true" ><span>Uninstall</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="60" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="15" data-stub-action="true" aria-disabled="true" ><span>Install</span></button><button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="61" data-admin-quarry-source="components/modals/PremiumTabModal.tsx" data-admin-quarry-placement="premium tab modal modes" data-admin-quarry-local-index="16" data-stub-action="true" aria-disabled="true" ><span>Delete</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="root-ca-modal"><div class="modal-titlebar">Root CA Certificate <span class="admin-quarry-note">5 buttons · root-ca-modal</span></div><div class="modal-body-port"><div class="update-status-container">Certificate installed</div><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="62" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="63" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Confirm Refresh</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="64" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Download Certificate</span></button><button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="65" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Refresh Certificate</span></button><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="66" data-admin-quarry-source="components/modals/RootCAModal.tsx" data-admin-quarry-placement="root CA install/refresh modal" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Close</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="sync-schedule-modal"><div class="modal-titlebar">Auto Sync Schedule <span class="admin-quarry-note">2 buttons · sync-schedule-modal</span></div><div class="modal-body-port"><label class="form-group">Schedule<select class="field"><option>Daily</option></select></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="67" data-admin-quarry-source="components/modals/SyncScheduleModal.tsx" data-admin-quarry-placement="sync schedule modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="68" data-admin-quarry-source="components/modals/SyncScheduleModal.tsx" data-admin-quarry-placement="sync schedule modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Save Schedule</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="system-action-modal"><div class="modal-titlebar">System Action <span class="admin-quarry-note">1 buttons · system-action-modal</span></div><div class="modal-body-port"><p class="system-action-text">Action output</p><pre class="log-frame">waiting...</pre><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="69" data-admin-quarry-source="components/modals/SystemModals.tsx" data-admin-quarry-placement="system progress modal" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Copy Logs</span></button></div></div></article>
            <article class="modal-window" data-admin-quarry-group="update-key-modal"><div class="modal-titlebar">Update Key on Drive <span class="admin-quarry-note">2 buttons · update-key-modal</span></div><div class="modal-body-port"><label class="form-group">Drive<select class="field"><option>/dev/sdb</option></select></label><div class="modal-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="70" data-admin-quarry-source="components/modals/UpdateKeyModal.tsx" data-admin-quarry-placement="update key modal footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Cancel</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="71" data-admin-quarry-source="components/modals/UpdateKeyModal.tsx" data-admin-quarry-placement="update key modal footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Update Key</span></button></div></div></article>
            <article class="modal-window update-manager-modal" data-admin-quarry-group="update-manager-modal"><div class="modal-titlebar">Update Manager <span class="admin-quarry-note">19 buttons · update-manager-modal</span></div><div class="update-manager-header"><div class="view-tabs"><button type="button" class="tab-button active admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="82" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="11" data-stub-action="true" aria-disabled="true" ><span>Overview</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="83" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="12" data-stub-action="true" aria-disabled="true" ><span>Modules</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="84" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="13" data-stub-action="true" aria-disabled="true" ><span>Schedule</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="85" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="14" data-stub-action="true" aria-disabled="true" ><span>Interactives</span></button><button type="button" class="tab-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="86" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="15" data-stub-action="true" aria-disabled="true" ><span>Logs</span></button></div></div><div class="modal-body-port update-manager-content"><div class="update-status-container">Updates available</div><table class="modules-table"><tr><th>Module</th><th>Branch</th><th>Actions</th></tr><tr><td>coronatio</td><td>main</td><td><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="72" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span>Refresh Modules</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="73" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>Toggle Module</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="74" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>Apply Branch</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="75" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>Reset to default</span></button></td></tr></table><div class="modal-toolbar"><button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="76" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span>Refresh Interactives</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="77" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span>Run Interactive</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="78" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span>Save Schedule</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="79" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="8" data-stub-action="true" aria-disabled="true" ><span>Back to Overview</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="80" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="9" data-stub-action="true" aria-disabled="true" ><span>Copy Contents</span></button> <button type="button" class="refresh-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="81" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="10" data-stub-action="true" aria-disabled="true" ><span>Refresh Logfile</span></button></div><div class="update-actions"><button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="87" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="16" data-stub-action="true" aria-disabled="true" ><span>Close</span></button> <button type="button" class="modal-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="88" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="17" data-stub-action="true" aria-disabled="true" ><span>Check Updates</span></button> <button type="button" class="modal-button modal-button-primary admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="89" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="18" data-stub-action="true" aria-disabled="true" ><span>Update</span></button> <button type="button" class="modal-button modal-button-danger admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="90" data-admin-quarry-source="components/modals/UpdateManagerModal.tsx" data-admin-quarry-placement="update manager tabs, tables, logs, footer" data-admin-quarry-local-index="19" data-stub-action="true" aria-disabled="true" ><span>Force Update</span></button></div></div></article>
          </section>
        </div>
      </section>
      <section class="pane" id="pane-stats" data-pane-panel="stats" role="tabpanel" aria-label="Stats">
        <div class="stats-viewport" data-stats-viewport>
          <section class="stats-section resources" aria-label="Resources">
            <h2>Resources</h2>
            <div class="stats-resource-grid">
              <article class="stats-resource-card chart-card" data-chart-card="cpu"><h3>CPU</h3><div class="cpu-gauge-container"><canvas id="cpu-gauge" data-chart-canvas="cpu-gauge"></canvas><div id="cpu-temp" class="cpu-temp">—°C</div></div><div id="cpu-details" class="cpu-details"><span>5s <strong id="cpu-5s">—</strong>%</span><span>1m <strong id="cpu-1m">—</strong>%</span><span>5m <strong id="cpu-5m">—</strong>%</span><span>Cores <strong id="cpu-cores">—</strong></span></div></article>
              <article class="stats-resource-card chart-card" data-chart-card="memory"><h3>Memory</h3><div class="chart-container"><canvas id="memory-chart" data-chart-canvas="memory-chart"></canvas></div><div class="metric" id="stats-memory">—</div><div class="progress-bar"><div class="progress" id="stats-memory-progress"></div></div><div class="details"><span id="stats-memory-used">Used —</span><span id="stats-memory-total">Total —</span></div></article>
              <article class="stats-resource-card"><h3>Swap</h3><div class="metric" id="stats-swap">—</div><div class="progress-bar"><div class="progress" id="stats-swap-progress"></div></div><div class="details"><span id="stats-swap-used">Used —</span><span id="stats-swap-total">Total —</span></div></article>
            </div>
          </section>
          <section class="stats-section drives" aria-label="Storage">
            <h2>Storage</h2>
            <div class="drives-grid" data-stats-drives></div>
          </section>
          <section class="stats-section network" aria-label="Network">
            <h2>Network</h2>
            <div class="chart-container"><canvas id="network-chart" data-chart-canvas="network-chart"></canvas></div>
            <div class="network-grid" data-stats-network></div>
            <div class="connections-summary" data-stats-connections></div>
          </section>
          <section class="stats-section services" aria-label="Services">
            <h2>Services</h2>
            <div class="services-grid" data-stats-services></div>
          </section>
          <section class="stats-section stats-transport-card" aria-label="Stats stream">
            <h2>Stream lane</h2>
            <p id="stats-stream">Stats stream state pending.</p>
            <p id="stats-missing" class="warning">Checking collector status…</p>
            <div class="button-row"><button data-fetch="/api/stats/events" data-target="stats-event">Read event frame</button><button class="secondary" data-admin-only data-admin-viewport="stats" data-fetch="/api/stats/events/renew" data-target="stats-event" data-method="POST">Renew lease</button></div>
            <pre class="readout" id="stats-event">No event readback yet.</pre>
            <pre class="readout" id="stats-readout">Fetching /api/stats…</pre>
          </section>
        </div>
      </section>
      <section class="pane" id="pane-portals" data-pane-panel="portals" role="tabpanel" aria-label="Portals">
        <div class="portal-grid">
          <article class="card portal-card"><div><h2>Admitted services</h2><p>Portal cards follow the main HomeServer service-grid pattern and expose the live config contract.</p></div><div class="button-row"><button data-fetch="/api/services/data" data-target="portals-readout">Read service contract</button><a class="action-link secondary" href="https://home.arpa/">Open main HomeServer</a></div></article>
          <article class="card portal-card"><div><h2>Coronatio</h2><p>Rust crown preview, port 3013.</p></div><div class="button-row" data-admin-only data-admin-viewport="portals"><button data-fetch="/api/portals" data-method="POST" data-target="portals-readout">Add portal</button><button class="secondary" data-fetch="/api/portals/coronatio" data-method="PUT" data-target="portals-readout">Edit portal</button></div><span class="status-pill ok">online</span></article>
          <article class="card portal-card"><div><h2>Caduceus</h2><p>Privileged actuator membrane, port 3014.</p></div><div class="button-row"><button data-fetch="/api/caduceus/status" data-target="portals-readout">Status</button><a class="action-link secondary" href="http://home.arpa:3014/health">Health</a></div></article>
        </div>
        <pre class="readout" id="portals-readout">Service contract readback will appear here.</pre>
      </section>
      <section class="pane" id="pane-upload" data-pane-panel="upload" role="tabpanel" aria-label="Upload">
        <div class="pane-grid upload-viewport" data-upload-viewport>
          <article class="card upload-card"><h2>Safe file ingress</h2><form class="upload-form" data-upload-form><label>Destination <input class="field" name="path" data-upload-path value="/mnt/nas" autocomplete="off"></label><label>File <input class="field" name="file" data-upload-file type="file"></label><div class="button-row"><button type="submit">Upload through Caduceus</button><button type="button" class="secondary" data-admin-only data-admin-viewport="upload" data-fetch="/api/upload/default-directory" data-target="upload-readout">Default directory</button></div></form><div class="drop-zone" data-upload-drop><strong>Choose a file</strong><p>Coronatio reads the browser file and sends upload metadata through the Caduceus staff membrane.</p></div></article>
          <article class="card"><h2>Upload controls</h2><div class="button-row"><button class="secondary" data-fetch="/api/upload/history" data-target="upload-readout">History</button><button data-admin-only data-admin-viewport="upload" data-fetch="/api/upload/pin-required-status" data-target="upload-readout">PIN requirement</button><button class="secondary" data-admin-only data-admin-viewport="upload" data-fetch="/api/upload/blacklist/list" data-target="upload-readout">Blacklist</button></div><pre class="readout" id="upload-readout">Select a file to send the upload intent to Caduceus.</pre></article>
        </div>
      </section>
    </section>
  </main>
  <script>
    const appRoot = document.querySelector('[data-product="Coronatio"]');
    const tabBar = document.querySelector('[role="tablist"]');
    const tabs = [...document.querySelectorAll('[data-pane]')];
    const panes = [...document.querySelectorAll('[data-pane-panel]')];
    const fallbackTab = 'admin';
    const storageKey = 'coronatio.flask-react-tabbar.v1';
    const loadTabState = () => {
      try { return JSON.parse(localStorage.getItem(storageKey) || '{}'); }
      catch (_) { return {}; }
    };
    const saveTabState = state => localStorage.setItem(storageKey, JSON.stringify(state));
    const tabState = Object.assign({ starredTab: 'portals', hiddenTabs: [] }, loadTabState());
    const headerStateKey = 'coronatio.flask-react-header.v1';
    const preferredThemeKey = 'preferred-theme';
    const themeDataKey = 'themeData';
    let themeCatalog = { default: 'dark', themes: {} };
    let themes = [];
    const savedHeaderState = (() => { try { return JSON.parse(localStorage.getItem(headerStateKey) || '{}'); } catch (_) { return {}; } })();
    const savedPreferredTheme = localStorage.getItem(preferredThemeKey);
    const headerState = Object.assign({ theme: savedPreferredTheme || savedHeaderState.theme || 'dark', isAdmin: false }, savedHeaderState, { theme: savedPreferredTheme || savedHeaderState.theme || 'dark' });
    const saveHeaderState = () => {
      localStorage.setItem(headerStateKey, JSON.stringify(headerState));
      localStorage.setItem(preferredThemeKey, headerState.theme);
      localStorage.setItem(themeDataKey, JSON.stringify({ name: headerState.theme, values: themeCatalog.themes[headerState.theme] || {} }));
    };
    const themeButton = document.querySelector('[data-theme-button]');
    const adminButton = document.querySelector('[data-admin-button]');
    const changePinButton = document.querySelector('[data-change-pin-button]');
    const modalBackdrop = document.querySelector('[data-pin-modal-backdrop]');
    const modalTitle = document.getElementById('pin-modal-title');
    const modalMessage = document.querySelector('[data-pin-modal-message]');
    const infoBackdrop = document.querySelector('[data-info-modal-backdrop]');
    const infoTitle = document.getElementById('info-modal-title');
    const infoBody = document.querySelector('[data-info-modal-body]');
    const themeChoiceRow = document.querySelector('[data-theme-choice-row]');
    const currentPinInput = document.querySelector('[data-pin-current]');
    const changeCurrentPinInput = document.querySelector('[data-pin-change-current]');
    const newPinInput = document.querySelector('[data-pin-new]');
    const confirmPinInput = document.querySelector('[data-pin-confirm]');
    let modalMode = 'enter';
    function themeToCss(theme) {
      if (!theme) return '';
      return ':root {\n' + Object.entries(theme).map(([key, value]) => '  --theme-' + key + ': ' + value + ';').join('\n') + '\n}';
    }
    function themeLabel(name) {
      return name.split('-').map(part => part.charAt(0).toUpperCase() + part.slice(1)).join(' ');
    }
    function renderThemeChoices() {
      if (!themeChoiceRow) return;
      themeChoiceRow.innerHTML = '';
      themes.forEach(name => {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'theme-choice';
        button.dataset.themeChoice = name;
        button.textContent = themeLabel(name);
        button.addEventListener('click', () => {
          headerState.theme = name;
          saveHeaderState();
          applyTheme();
          closeInfoModal();
        });
        themeChoiceRow.appendChild(button);
      });
    }
    function ensureThemeStyleElement() {
      let style = document.querySelector('style[data-theme-styles]');
      if (!style) {
        style = document.createElement('style');
        style.setAttribute('data-theme-styles', '');
        document.head.appendChild(style);
      }
      return style;
    }
    function applyTheme() {
      if (!themes.includes(headerState.theme)) headerState.theme = themeCatalog.default || themes[0] || 'dark';
      const theme = themeCatalog.themes[headerState.theme];
      document.documentElement.dataset.theme = headerState.theme;
      ensureThemeStyleElement().textContent = themeToCss(theme);
      saveHeaderState();
      if (themeButton) {
        const label = themeLabel(headerState.theme);
        themeButton.querySelector('span').textContent = label;
        themeButton.title = 'Current theme: ' + label + '. Click to switch theme.';
      }
      document.querySelectorAll('[data-theme-choice]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.themeChoice === headerState.theme)));
    }
    function setAdminMode(value) {
      headerState.isAdmin = Boolean(value);
      saveHeaderState();
      if (adminButton) {
        adminButton.dataset.adminState = headerState.isAdmin ? 'logged-in' : 'logged-out';
        adminButton.textContent = headerState.isAdmin ? 'Exit Admin Mode' : 'Enter Admin Mode';
      }
      if (appRoot) appRoot.dataset.adminMode = headerState.isAdmin ? 'true' : 'false';
      if (tabBar) tabBar.dataset.adminMode = headerState.isAdmin ? 'true' : 'false';
      document.querySelectorAll('[data-admin-only]:not([data-admin-only="false"])').forEach(el => {
        el.hidden = !headerState.isAdmin;
        el.setAttribute('aria-hidden', String(!headerState.isAdmin));
      });
      if (changePinButton) changePinButton.hidden = !headerState.isAdmin;
    }
    function openPinModal(mode) {
      modalMode = mode;
      modalTitle.textContent = mode === 'change' ? 'Change Admin PIN' : 'Enter Admin Mode';
      currentPinInput.hidden = mode === 'change';
      changeCurrentPinInput.hidden = mode !== 'change';
      newPinInput.hidden = mode !== 'change';
      confirmPinInput.hidden = mode !== 'change';
      modalMessage.textContent = '';
      [currentPinInput, changeCurrentPinInput, newPinInput, confirmPinInput].forEach(input => { input.value = ''; });
      modalBackdrop.classList.add('open');
      modalBackdrop.setAttribute('aria-hidden', 'false');
      (mode === 'change' ? changeCurrentPinInput : currentPinInput).focus();
    }
    function closePinModal() {
      modalBackdrop.classList.remove('open');
      modalBackdrop.setAttribute('aria-hidden', 'true');
    }
    function indicatorAdminSection(inner) {
      return headerState.isAdmin ? `<div data-admin-only data-admin-surface="indicator-modal" data-admin-enhanced="true">${inner}</div>` : '';
    }
    function modalTemplate(kind) {
      if (kind === 'tailscale') return `<div class="tailscale-status-modal">
        <div class="status-section"><p class="status-text loading">LOADING...</p></div>
        ${indicatorAdminSection(`<div class="config-section"><div class="current-tailnet"><span class="label">Current Tailnet:</span><span class="value" data-route-read="/api/status/tailscale/config">Loading...</span></div><input data-tailnet-input placeholder="Enter Tailnet name"><div class="button-row"><button data-modal-fetch="/api/status/tailscale/update-tailnet" data-method="POST">Update Tailnet</button><button data-modal-fetch="/api/status/tailscale/connect" data-method="POST">Connect</button><button data-modal-fetch="/api/status/tailscale/disconnect" data-method="POST">Disconnect</button><button data-modal-fetch="/api/status/tailscale/enable" data-method="POST">Enable Service</button><button data-modal-fetch="/api/status/tailscale/disable" data-method="POST">Disable Service</button></div></div>
        <div class="authkey-section"><input class="authkey-input" placeholder="Enter your tskey-auth-... or tskey-client-... key"><div class="button-row"><button data-modal-fetch="/api/status/tailscale/authkey" data-method="POST">Authenticate</button></div></div>`)}<pre class="readout action-output" data-modal-output></pre>
      </div>`;
      if (kind === 'internet') return `<div class="internet-status-modal"><div class="status-section"><p class="status-text loading">CHECKING...</p></div>${indicatorAdminSection(`<div class="admin-details-section" data-admin-details-section><div class="ip-details"><p><strong>Location:</strong> —</p><p><strong>ISP:</strong> —</p><p><strong>Timezone:</strong> —</p></div></div><div class="speed-test-section"><div class="button-row"><button data-modal-fetch="/api/status/internet/speedtest" data-method="POST">Run Speed Test</button></div><div class="speed-results"><p>Download: — Mbps</p><p>Upload: — Mbps</p><p>Latency: — ms</p></div></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'services') return `<div class="services-status-modal"><div class="loading-section">Loading service status...</div><ul class="service-status-list" data-route-read="/api/status/services"><li>No status data available</li></ul>${indicatorAdminSection(`<div class="admin-service-grid"><div class="admin-service-description">Description</div><div class="admin-service-name">Service</div><div class="admin-service-right"><span class="admin-service-status">enabled</span></div></div><div class="button-row"><button data-modal-fetch="/api/status/services">Refresh</button><button data-modal-fetch="/api/services/data">Service Data</button></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'openvpn') return `<div class="vpn-status-modal"><div class="status-section"><div class="service-statuses"><div class="status-item loading"><span>VPN Status:</span><span class="status-value">LOADING</span></div><div class="status-item loading"><span>Transmission Status:</span><span class="status-value">LOADING</span></div>${headerState.isAdmin ? `<div class="status-item" data-admin-only data-admin-surface="indicator-modal"><span>Systemd Service:</span><span class="status-value">LOADING</span></div>` : ''}</div></div>${indicatorAdminSection(`<div class="credentials-section"><div class="modal-grid"><div class="credential-group"><input placeholder="PIA Username"><input type="password" placeholder="PIA Password"><button data-modal-fetch="/api/status/vpn/updatekey/pia" data-method="POST">Create PIA Key</button></div><div class="credential-group"><input placeholder="Transmission Username"><input type="password" placeholder="Transmission Password"><button data-modal-fetch="/api/status/vpn/updatekey/transmission" data-method="POST">Create Transmission</button></div></div></div><div class="service-controls"><div class="button-row"><button data-modal-fetch="/api/status/vpn/enable" data-method="POST">Enable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/disable" data-method="POST">Disable Transmission over PIA VPN</button><button data-modal-fetch="/api/status/vpn/pia/exists">PIA Key Exists</button><button data-modal-fetch="/api/status/vpn/transmission/exists">Transmission Key Exists</button></div></div><div class="restart-notice"><p>Note: Service changes require a restart to take effect.</p></div>`)}<pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'power-meter') return `<div class="power-meter-modal"><div class="power-usage-display"><div class="power-value"><span class="power-value-number">—</span><span class="power-value-unit">Watts</span></div></div><div class="power-history-section"><div class="power-averages"><div class="power-average-row"><div class="power-average-label">5s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">30s average:</div><div class="power-average-value">—W</div></div><div class="power-average-row"><div class="power-average-label">60s average:</div><div class="power-average-value">—W</div></div></div></div><pre class="readout action-output" data-modal-output></pre></div>`;
      if (kind === 'theme') return `<div class="theme-modal"><p>Current theme: ${headerState.theme}.</p><p>Themes are loaded from /api/themes backed by static/themes/theme.json.</p></div>`;
      return '';
    }
    function wireModalFetches() {
      infoBody.querySelectorAll('[data-modal-fetch]').forEach(button => button.addEventListener('click', async () => {
        const output = infoBody.querySelector('[data-modal-output]');
        if (!headerState.isAdmin && button.closest('[data-admin-only]')) { if (output) output.textContent = 'Enter Admin Mode'; return; }
        if (output) output.textContent = 'Loading ' + button.dataset.modalFetch + '…';
        try {
          const response = await fetch(button.dataset.modalFetch, { method: button.dataset.method || 'GET' });
          const text = await response.text();
          if (output) { try { output.textContent = JSON.stringify(JSON.parse(text), null, 2); } catch (_) { output.textContent = text; } }
        } catch (error) { if (output) output.textContent = 'fetch failed: ' + error; }
      }));
    }
    function openInfoModal(title, kind = 'status') {
      infoTitle.textContent = title;
      infoBody.innerHTML = modalTemplate(kind);
      themeChoiceRow.hidden = kind !== 'theme';
      infoBackdrop.classList.add('open');
      infoBackdrop.setAttribute('aria-hidden', 'false');
      wireModalFetches();
    }
    function closeInfoModal() {
      infoBackdrop.classList.remove('open');
      infoBackdrop.setAttribute('aria-hidden', 'true');
    }
    function closeModalOnOutsideClick(backdrop, closeModal) {
      backdrop?.addEventListener('click', event => {
        if (event.target === event.currentTarget) closeModal();
      });
    }
    closeModalOnOutsideClick(modalBackdrop, closePinModal);
    closeModalOnOutsideClick(infoBackdrop, closeInfoModal);
    document.querySelector('[data-info-modal-close]')?.addEventListener('click', closeInfoModal);
    document.querySelectorAll('[data-indicator]').forEach(button => button.addEventListener('click', () => openInfoModal(button.dataset.modalTitle, button.dataset.modalKind)));
    function cycleTheme() {
      if (!themes.length) return;
      const currentIndex = Math.max(0, themes.indexOf(headerState.theme));
      headerState.theme = themes[(currentIndex + 1) % themes.length];
      saveHeaderState();
      applyTheme();
    }
    themeButton?.addEventListener('click', cycleTheme);
    async function loadThemeCatalog() {
      try {
        const catalog = await fetch('/api/themes').then(response => response.json());
        if (!catalog.themes || Object.keys(catalog.themes).length === 0) throw new Error('empty theme catalog');
        themeCatalog = catalog;
        themes = Object.keys(catalog.themes);
        renderThemeChoices();
        applyTheme();
      } catch (error) {
        console.error('theme catalog load failed', error);
        themes = [];
        ensureThemeStyleElement().textContent = '';
        if (themeButton) themeButton.title = 'Theme catalog unavailable: ' + error;
      }
    }
    adminButton?.addEventListener('click', () => {
      if (headerState.isAdmin) setAdminMode(false);
      else openPinModal('enter');
    });
    changePinButton?.addEventListener('click', () => {
      if (!headerState.isAdmin) { modalMessage.textContent = 'Must be in admin mode to change PIN'; return; }
      openPinModal('change');
    });
    document.querySelector('[data-pin-cancel]')?.addEventListener('click', closePinModal);
    document.querySelector('[data-pin-confirm-button]')?.addEventListener('click', async () => {
      if (modalMode === 'change' && (!changeCurrentPinInput.value || !newPinInput.value || !confirmPinInput.value)) { modalMessage.textContent = 'Please fill in all fields'; return; }
      if (modalMode === 'change' && newPinInput.value !== confirmPinInput.value) { modalMessage.textContent = 'New PINs do not match'; return; }
      if (modalMode === 'enter' && !currentPinInput.value) { modalMessage.textContent = 'Enter PIN'; return; }
      setAdminMode(true);
      modalMessage.textContent = modalMode === 'change' ? 'PIN changed successfully' : '';
      if (modalMode === 'enter') closePinModal();
    });
    loadThemeCatalog();
    setAdminMode(headerState.isAdmin);
    function visibleTabs() { return tabs.filter(tab => tab.dataset.visibility !== 'hidden' && tab.dataset.adminOnly !== 'true'); }
    function firstVisibleTab() { return visibleTabs()[0]?.dataset.pane || fallbackTab; }
    function setStarredTab(id) {
      tabState.starredTab = id;
      saveTabState(tabState);
      tabs.forEach(tab => {
        const starred = tab.dataset.pane === id;
        const button = tab.querySelector('[data-tab-star]');
        if (button) {
          button.classList.toggle('fas', starred);
          button.classList.toggle('far', !starred);
          button.setAttribute('aria-pressed', String(starred));
          button.title = starred ? tab.querySelector('.tab-name').textContent + ' tab is starred' : 'Star ' + tab.querySelector('.tab-name').textContent + ' tab';
        }
      });
    }
    function applyVisibilityState() {
      tabs.forEach(tab => {
        const hidden = tabState.hiddenTabs.includes(tab.dataset.pane);
        tab.dataset.visibility = hidden ? 'hidden' : 'visible';
        const button = tab.querySelector('[data-tab-visibility-toggle]');
        if (button) {
          button.dataset.visible = String(!hidden);
          button.title = (hidden ? 'Show ' : 'Hide ') + tab.querySelector('.tab-name').textContent + ' tab';
          button.setAttribute('aria-label', button.title);
          button.querySelector('.eye-icon').textContent = hidden ? '🙈' : '👁';
        }
      });
    }
    function showPane(id) {
      const selected = panes.some(pane => pane.dataset.panePanel === id) ? id : firstVisibleTab();
      tabs.forEach(tab => {
        const active = tab.dataset.pane === selected;
        tab.setAttribute('aria-selected', String(active));
        tab.classList.toggle('active', active);
      });
      panes.forEach(pane => pane.classList.toggle('active', pane.dataset.panePanel === selected));
      if (location.hash !== '#' + selected) history.replaceState(null, '', '#' + selected);
    }
    tabs.forEach(tab => {
      tab.addEventListener('click', event => {
        if (event.target.closest('button')) return;
        event.preventDefault();
        if (tab.dataset.visibility !== 'hidden') showPane(tab.dataset.pane);
      });
      tab.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); showPane(tab.dataset.pane); } });
    });
    document.querySelectorAll('[data-tab-star]').forEach(button => button.addEventListener('click', event => { event.stopPropagation(); setStarredTab(button.dataset.tabStar); }));
    document.querySelectorAll('[data-tab-visibility-toggle]').forEach(button => button.addEventListener('click', event => {
      event.stopPropagation();
      const id = button.dataset.tabVisibilityToggle;
      const hidden = tabState.hiddenTabs.includes(id);
      tabState.hiddenTabs = hidden ? tabState.hiddenTabs.filter(tab => tab !== id) : [...new Set([...tabState.hiddenTabs, id])];
      saveTabState(tabState);
      applyVisibilityState();
      if (!hidden && tabState.starredTab === id) setStarredTab(firstVisibleTab());
      const active = tabs.find(tab => tab.getAttribute('aria-selected') === 'true');
      if (!active || active.dataset.visibility === 'hidden') showPane(firstVisibleTab());
    }));
    applyVisibilityState();
    setStarredTab(tabState.starredTab);
    async function fetchInto(route, target, method = 'GET') {
      const el = document.getElementById(target);
      if (!el) return;
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
    const statsChartState = {
      labels: [],
      cpu: [],
      temp: [],
      memory: [],
      rx: [],
      tx: [],
      lastRx: null,
      lastTx: null,
      lastStamp: null,
      charts: {}
    };
    function chartReady() { return typeof Chart !== 'undefined'; }
    function pushChartPoint(label, data) {
      const now = Date.now();
      const totalRx = (data.network?.interfaces || []).reduce((sum, iface) => sum + Number(iface.rxBytes || 0), 0);
      const totalTx = (data.network?.interfaces || []).reduce((sum, iface) => sum + Number(iface.txBytes || 0), 0);
      const seconds = statsChartState.lastStamp ? Math.max(1, (now - statsChartState.lastStamp) / 1000) : 1;
      const rxRate = statsChartState.lastRx === null ? 0 : Math.max(0, (totalRx - statsChartState.lastRx) / seconds);
      const txRate = statsChartState.lastTx === null ? 0 : Math.max(0, (totalTx - statsChartState.lastTx) / seconds);
      statsChartState.lastRx = totalRx;
      statsChartState.lastTx = totalTx;
      statsChartState.lastStamp = now;
      statsChartState.labels.push(label);
      statsChartState.cpu.push(Number(data.resources?.load?.one || 0));
      statsChartState.temp.push(Number(data.resources?.load?.cpuTemperatureCelsius || 0));
      statsChartState.memory.push(Number(data.resources?.memory?.percent || 0));
      statsChartState.rx.push(rxRate);
      statsChartState.tx.push(txRate);
      if (statsChartState.labels.length > 60) {
        ['labels', 'cpu', 'temp', 'memory', 'rx', 'tx'].forEach(key => statsChartState[key].shift());
      }
    }
    function chartColors(ctx, top, bottom, height = 200) {
      const gradient = ctx.createLinearGradient(0, 0, 0, height);
      gradient.addColorStop(0, top);
      gradient.addColorStop(1, bottom);
      return gradient;
    }
    function ensureStatsCharts(data) {
      if (!chartReady()) {
        document.querySelectorAll('.chart-card').forEach(card => {
          if (!card.querySelector('.chart-fallback')) card.insertAdjacentHTML('beforeend', '<p class="chart-fallback">Chart.js dependency unavailable.</p>');
        });
        return;
      }
      if (window.ChartDataLabels && Chart.register) Chart.register(window.ChartDataLabels);
      const cpuCanvas = document.getElementById('cpu-gauge');
      if (cpuCanvas && !statsChartState.charts.cpuGauge) {
        const ctx = cpuCanvas.getContext('2d');
        statsChartState.charts.cpuGauge = new Chart(ctx, {
          type: 'doughnut',
          data: { datasets: [
            { data: [0, 100], backgroundColor: ['rgba(78, 121, 167, 0.8)', 'rgba(78, 121, 167, 0.2)'], borderWidth: 0, circumference: 180, rotation: -90, cutout: '70%' },
            { data: [0, 100], backgroundColor: ['rgba(242, 142, 44, 0.8)', 'rgba(242, 142, 44, 0.2)'], borderWidth: 0, circumference: 180, rotation: -90, cutout: '55%' },
            { data: [0, 100], backgroundColor: ['rgba(225, 87, 89, 0.8)', 'rgba(225, 87, 89, 0.2)'], borderWidth: 0, circumference: 180, rotation: -90, cutout: '40%' }
          ] },
          options: { responsive: true, maintainAspectRatio: false, plugins: { tooltip: { enabled: false }, legend: { display: false }, datalabels: { color: 'white', formatter: (value, ctx) => ctx.dataIndex === 0 ? Number(value).toFixed(1) + '%' : '', anchor: 'end', align: 'start', offset: 10, font: { size: 10 } } } }
        });
      }
      const memoryCanvas = document.getElementById('memory-chart');
      if (memoryCanvas && !statsChartState.charts.memory) {
        const ctx = memoryCanvas.getContext('2d');
        statsChartState.charts.memory = new Chart(ctx, {
          type: 'line',
          data: { labels: statsChartState.labels, datasets: [{ label: 'Memory Usage', data: statsChartState.memory, borderColor: chartColors(ctx, 'rgba(99, 179, 237, 0.9)', 'rgba(99, 179, 237, 0.25)'), backgroundColor: 'rgba(99, 179, 237, 0.14)', borderWidth: 2, fill: true, tension: 0.4 }] },
          options: { responsive: true, maintainAspectRatio: false, animation: { duration: 0 }, scales: { x: { ticks: { maxTicksLimit: 5, color: '#9ca3af' }, grid: { display: false } }, y: { beginAtZero: true, max: 100, ticks: { callback: value => value + '%', color: '#9ca3af' }, grid: { color: 'rgba(156,163,175,0.1)' } } }, plugins: { legend: { display: false }, datalabels: { display: false } } }
        });
      }
      const networkCanvas = document.getElementById('network-chart');
      if (networkCanvas && !statsChartState.charts.network) {
        const ctx = networkCanvas.getContext('2d');
        statsChartState.charts.network = new Chart(ctx, {
          type: 'line',
          data: { labels: statsChartState.labels, datasets: [
            { label: 'Download', data: statsChartState.rx, borderColor: '#63b3ed', backgroundColor: 'rgba(99, 179, 237, 0.2)', fill: true, tension: 0.4 },
            { label: 'Upload', data: statsChartState.tx, borderColor: '#f6ad55', backgroundColor: 'rgba(246, 173, 85, 0.2)', fill: true, tension: 0.4 }
          ] },
          options: { responsive: true, maintainAspectRatio: false, animation: { duration: 0 }, scales: { x: { display: true, grid: { display: false }, ticks: { maxTicksLimit: 5, color: '#9ca3af' } }, y: { beginAtZero: true, ticks: { callback: value => fmtBytes(value) + '/s', color: '#9ca3af' }, grid: { color: 'rgba(156, 163, 175, 0.1)' } } }, plugins: { legend: { display: false }, datalabels: { display: false }, tooltip: { callbacks: { label: context => context.dataset.label + ': ' + fmtBytes(context.parsed.y) + '/s' } } } }
        });
      }
      updateStatsCharts(data);
    }
    function updateStatsCharts(data) {
      if (!chartReady()) return;
      const load = data.resources?.load || {};
      const cpuNow = Number(load.one || 0);
      const cpu1m = Number(load.five || cpuNow);
      const cpu5m = Number(load.fifteen || cpu1m);
      const gauge = statsChartState.charts.cpuGauge;
      if (gauge) {
        gauge.data.datasets[0].data = [cpuNow, Math.max(0, 100 - cpuNow)];
        gauge.data.datasets[1].data = [cpu1m, Math.max(0, 100 - cpu1m)];
        gauge.data.datasets[2].data = [cpu5m, Math.max(0, 100 - cpu5m)];
        gauge.update();
      }
      if (statsChartState.charts.memory) statsChartState.charts.memory.update();
      if (statsChartState.charts.network) statsChartState.charts.network.update();
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
        document.getElementById('cpu-temp').textContent = (load.cpuTemperatureCelsius ?? '—') + '°C';
        document.getElementById('cpu-5s').textContent = load.one ?? '—';
        document.getElementById('cpu-1m').textContent = load.five ?? '—';
        document.getElementById('cpu-5m').textContent = load.fifteen ?? '—';
        document.getElementById('cpu-cores').textContent = navigator.hardwareConcurrency || '—';
        document.getElementById('stats-memory').textContent = metricPercent(memory.percent);
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
    showPane((location.hash || '#' + (tabState.starredTab || firstVisibleTab())).slice(1));
    hydrateUptime();
    hydrateStats();
    setInterval(hydrateStats, 5000);
  </script>
</body>
</html>"###;
    shell.replace("__NAV__", &nav)
}

fn is_safe_tab_id(tab_id: &str) -> bool {
    !tab_id.is_empty()
        && tab_id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

