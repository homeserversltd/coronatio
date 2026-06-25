fn shell_document_1() -> &'static str {
    r####"<!doctype html>
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
    .portal-card[data-portal-card] { min-height: 220px; align-items: center; text-align: center; cursor: pointer; }
    .portal-card[data-portal-card]:hover { transform: translateY(-2px); border-color: color-mix(in srgb, var(--accent) 48%, var(--border)); }
    .portal-card-header { display: flex; flex-direction: column; align-items: center; gap: .45rem; }
    .portal-icon { width: 96px; height: 96px; object-fit: contain; border-radius: 8px; }
    .portal-name { margin: .25rem 0 0; font-size: 1.05rem; font-weight: 700; color: var(--text); }
    .portal-description { margin: 0; color: var(--text-secondary); font-size: .9rem; line-height: 1.35; }
    .portal-service-row { display: flex; gap: .35rem; justify-content: center; flex-wrap: wrap; color: var(--text-secondary); font-size: .78rem; }
    .portal-chip { border: 1px solid var(--border); border-radius: 999px; padding: .18rem .45rem; background: rgba(255,255,255,.05); }

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
    .modal-body-port { padding: 14px; display: flex; flex-direction: column; gap: 12px; }"####
}
