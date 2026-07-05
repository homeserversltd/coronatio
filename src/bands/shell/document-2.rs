fn shell_document_2() -> &'static str {
    r####"    .modal-actions, .modal-toolbar, .modal-pager, .update-actions { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: 10px; margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--border); }
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
    .admin-quarry-note { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; border: 0; }

    .stats-tablet { display: flex; flex-direction: column; gap: 24px; padding: 16px; }
    .stat-element { position: relative; padding: 8px; border-radius: 8px; background-color: var(--background); transition: all 0.2s ease-out; box-shadow: 0 2px 4px var(--primary), 0 2px 4px var(--border); }
    .stat-element[data-visible="true"] { background-color: var(--background); }
    .stat-element[data-visible="false"] { background-color: var(--hiddenTabBackground); }
    .stat-header { display: flex; align-items: center; gap: 8px; margin-bottom: 16px; }
    .stat-title { margin: 0; color: var(--text); font-size: 1rem; font-weight: 500; flex-grow: 1; text-align: center; }
    .stat-content { display: flex; flex-direction: column; gap: 16px; }
    .stat-info { padding: 8px; background-color: var(--hiddenTabBackground); border-radius: 4px; font-size: .9rem; color: var(--text); }
    .visibility-toggle { padding: .35rem .45rem; min-width: 34px; }
    .cpu-stats-container, .network-stats-container, .disk-io-chart, .memory-stats, .disk-usage-stats, .kea-leases-table, .process-usage-list { width: 100%; }
    .cpu-chart, .network-speed-chart, .stat-chart { width: 100%; height: 180px; margin: 0; }
    .chart-container { position: relative; width: 100%; height: 180px; margin: 0; }
    .coronatio-chart-canvas { display: block; width: 100% !important; height: 180px !important; }
    .recharts-wrapper { width: 100%; height: 180px; position: relative; }
    .recharts-surface { width: 100%; height: 180px; overflow: visible; display: block; }
    .recharts-cartesian-grid line { stroke: var(--border) !important; stroke-opacity: .8; stroke-dasharray: 3 3; }
    .recharts-cartesian-axis text { fill: var(--hiddenTabText); font-size: 11px; }
    .recharts-line-curve { vector-effect: non-scaling-stroke; }
    .recharts-legend-wrapper, .custom-legend { display: flex; justify-content: center; gap: 16px; flex-wrap: wrap; color: var(--text); font-size: .85rem; padding: 8px; }
    .recharts-default-tooltip { background-color: var(--hiddenTabBackground) !important; border: 1px solid var(--border) !important; border-radius: 4px !important; padding: 8px !important; box-shadow: 0 2px 4px rgba(0,0,0,.1) !important; }
    .load-averages { display: flex; flex-direction: column; gap: .5rem; padding: .5rem; border-radius: 4px; }
    .load-average-values { display: flex; justify-content: space-around; gap: 2rem; padding: .5rem; color: var(--text); }
    .load-average-item { display: flex; gap: .5rem; align-items: center; }
    .load-label { color: var(--text); font-weight: 500; }
    .load-value { color: var(--text); margin-left: .5rem; }
    .network-interfaces { background: var(--tabContentBackground); border-radius: 4px; padding: 1rem; max-height: 260px; overflow: auto; }
    .network-interfaces-table, .kea-leases-table table { width: 100%; border-collapse: collapse; color: var(--text); }
    .network-interfaces-table th { background-color: var(--hiddenTabBackground); text-align: left; padding: .5rem; font-weight: 500; color: var(--accent); }
    .network-interfaces-table th, .network-interfaces-table td { border: 1px solid var(--border); }
    .network-interfaces-table td { padding: .5rem; }
    .interface-name { font-weight: 500; }
    .interface-label { color: var(--hiddenTabText); font-size: .9em; }
    .data-cell { font-family: var(--monoFont); }
    .device-controls { display: flex; flex-wrap: wrap; gap: 12px; margin-bottom: 16px; padding: 8px; background: var(--hiddenTabBackground); border-radius: 4px; }
    .device-control { display: flex; align-items: center; gap: 8px; padding: 4px 8px; background: var(--background); border-radius: 4px; font-size: .9rem; }
    .device-name { font-weight: 500; margin-right: 8px; color: var(--text); }
    .device-checkboxes { display: flex; gap: 8px; }
    .device-control label { display: flex; align-items: center; gap: 4px; color: var(--text); cursor: pointer; }
    .memory-stats { display: flex; flex-direction: column; gap: 16px; }
    .memory-current { display: flex; flex-direction: column; gap: 8px; border-bottom: 1px solid var(--border); }
    .memory-current:last-child { border-bottom: none; }
    .memory-label { font-size: .85rem; font-weight: 500; color: var(--text); margin-bottom: -4px; }
    .memory-bar, .disk-usage-bar, .process-bar { width: 100%; height: 24px; background-color: var(--hiddenTabBackground); border-radius: 12px; overflow: hidden; position: relative; }
    .memory-bar-fill, .disk-usage-fill { height: 100%; background-color: var(--secondary); border-radius: 12px; transition: width .3s ease-out; display: flex; align-items: center; justify-content: center; min-width: 40px; }
    .memory-bar-fill-swap { background-color: var(--accent); }
    .memory-text { color: var(--background); font-size: .85rem; font-weight: 500; text-shadow: 0 1px 2px rgba(0,0,0,.1); }
    .memory-details, .disk-usage-details { display: flex; justify-content: space-between; gap: 8px; font-size: .85rem; color: var(--text); padding: 4px 8px; }
    .disk-usage-stats { display: flex; flex-direction: column; gap: 16px; padding: 8px; }
    .disk-usage-item { display: flex; flex-direction: column; gap: 8px; border-bottom: 1px solid var(--border); }
    .disk-usage-item:last-child { border-bottom: none; padding-bottom: 0; }
    .disk-usage-header { display: flex; justify-content: space-between; align-items: center; padding: 0 4px; margin-bottom: 4px; }
    .disk-device { font-weight: 500; color: var(--text); font-size: .9rem; }
    .disk-mountpoint { font-style: italic; color: var(--hiddenTabText); font-size: .85rem; }
    .kea-leases-table { max-height: 360px; overflow: auto; }
    .kea-leases-table th, .kea-leases-table td { border: 1px solid var(--border); padding: 8px; text-align: left; min-width: 120px; }
    .kea-leases-table th { background-color: var(--hiddenTabBackground); color: var(--text); }
    .kea-leases-table th:nth-child(4), .kea-leases-table td:nth-child(4) { min-width: 150px !important; width: auto !important; white-space: nowrap; }
    .device-note-cell { display: flex; align-items: center; gap: 8px; min-width: 200px; min-height: 32px; padding: 4px 0; }
    .note-text { flex: 1; color: var(--text); font-size: .9rem; min-height: 24px; display: flex; align-items: center; }
    .edit-note-button { padding: 4px 8px; background: none; border: none; color: var(--text); opacity: .6; cursor: pointer; transition: opacity .2s; }
    .edit-note-button:hover { opacity: 1; }
    .process-usage-list { display: flex; flex-direction: column; gap: 8px; margin-top: 8px; position: relative; z-index: 1; }
    .process-bar { width: 100%; height: 24px; background-color: var(--hiddenTabBackground); border-radius: 12px; overflow: hidden; position: relative; z-index: 1; }
    .process-bar-fill { height: 100%; background-color: var(--primaryHover); border-radius: 12px; transition: width .3s ease-out; position: absolute; left: 0; top: 0; }
    .process-text-container { position: absolute; left: 0; top: 0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: space-between; padding: 0 8px; z-index: 1; }
    .process-name { color: var(--text); font-size: .85rem; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    .process-usage { color: var(--text); font-size: .85rem; font-weight: 500; margin-left: 8px; }
    .stat-element[data-visible="false"] .stat-title, .stat-element[data-visible="false"] .stat-info { color: var(--secondary); opacity: .7; }
    .cpu-chart, .network-speed-chart, .stat-chart { height: 180px; min-height: 180px; overflow: visible; }
    .disk-io-chart .chart-container { min-height: 180px; }
    .recharts-wrapper { height: 180px; min-height: 180px; overflow: visible; }
    .recharts-surface { height: 180px; max-height: 180px; }
    .recharts-legend-wrapper { margin-top: 6px; min-height: 28px; align-items: center; }
    .recharts-legend-item { display: inline-flex; align-items: center; gap: 4px; color: var(--text); }
    .recharts-cartesian-axis-tick-value, .recharts-surface text { fill: var(--hiddenTabText) !important; color: var(--hiddenTabText); }
    .network-stats-container, .disk-io-chart { display: flex; flex-direction: column; gap: 16px; }
    .network-interfaces { margin-top: 0; }
    @media (max-width: 768px) { .key-manager-content, .disk-manager-container { flex-direction: column; } .system-controls-btn { width: 100%; max-width: 180px; } .admin-modal-shelf { grid-template-columns: 1fr; } }
    button, .action-link { border: 1px solid var(--border); border-radius: 6px; padding: .55rem .7rem; background: var(--primary); color: var(--text); font-weight: 700; cursor: pointer; text-decoration: none; }
    button.secondary, .action-link.secondary { background: transparent; color: var(--text); }
    .warning { color: var(--warning); }
    .error { color: var(--error); }
    .success { color: var(--success); }
    .upload-tablet { display: flex; flex-direction: column; height: 100%; min-height: 0; overflow-y: auto; gap: 12px; scrollbar-width: inherit; }
    .upload-controls { display: flex; flex-direction: column; gap: 16px; overflow: visible; }
    .upload-progress-list { display: flex; flex-direction: column; gap: 12px; width: 100%; overflow: visible; }
    .upload-progress { background: var(--hiddenTabBackground); border-radius: 8px; padding: 12px; margin: 8px 0; box-shadow: 0 2px 4px rgba(0,0,0,.1); border: 1px solid var(--border); transition: transform .2s ease, box-shadow .2s ease; }
    .upload-header { display: flex; align-items: center; margin-bottom: 8px; gap: 8px; }
    .status-icon { font-size: 1.2em; }
    .filename { flex: 1; font-weight: 500; color: var(--text); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    .remove-button { background: none; border: none; color: var(--secondary); font-size: 1.2em; cursor: pointer; padding: 4px 8px; border-radius: 8px; transition: all .2s ease; }
    .remove-button:hover { color: var(--statusDown); background: rgba(239,68,68,.1); }
    .progress-section { display: flex; flex-direction: column; gap: 8px; }
    .progress-bar-container { width: 100%; height: 20px; background: var(--hiddenTabBackground); color: var(--text); border-radius: 10px; overflow: hidden; position: relative; }
    .progress-bar { height: 100%; border-radius: 10px; background: var(--hiddenTabBackground); color: var(--text); position: relative; display: flex; align-items: center; justify-content: center; min-width: 24px; }
    .progress-text { color: var(--text); font-size: .8em; font-weight: 500; text-shadow: 0 1px 2px rgba(0,0,0,.3); z-index: 1; }
    .upload-stats { display: flex; justify-content: space-between; font-size: .9em; color: var(--secondary); }
    .speed { color: var(--accent); font-weight: 500; }
    .error-message { color: var(--statusDown); font-size: .9em; padding: 8px; background: rgba(239,68,68,.1); border-radius: 4px; border-left: 3px solid var(--statusDown); }
    .upload-progress.pending .progress-bar, .upload-progress.uploading .progress-bar { background-image: linear-gradient(45deg, rgba(255,255,255,.15) 25%, transparent 25%, transparent 50%, rgba(255,255,255,.15) 50%, rgba(255,255,255,.15) 75%, transparent 75%, transparent); background-size: 1rem 1rem; animation: progress-stripes 1s linear infinite; }
    @keyframes progress-stripes { from { background-position: 1rem 0; } to { background-position: 0 0; } }
    .file-upload-section { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
    .file-upload-section button { background: var(--primary); border: none; border-radius: var(--border-radius, 8px); padding: 8px 12px; color: var(--text); cursor: pointer; transition: background var(--transition-fast); font-size: var(--font-size-sm); margin-right: 8px; margin-bottom: 8px; }
    .file-upload-section input[type="file"] { flex: 1; min-width: 200px; padding: 8px 12px; border: 1px solid var(--border); border-radius: var(--border-radius, 8px); background-color: var(--background); color: var(--text); font-size: 14px; cursor: default; transition: border-color .2s ease; }
    .file-upload-section button:hover { background: var(--primaryHover); }
    .file-upload-section input[type="file"]:focus { outline: none; border-color: var(--primary); }
    .file-upload-section button[disabled] { opacity: .6; cursor: not-allowed; background: var(--disabled); }
    .directory-browser { display: flex; flex-direction: column; border: 1px solid var(--border); border-radius: 8px; overflow: hidden; background-color: var(--background); box-shadow: 0 2px 4px var(--primary), 0 2px 4px var(--border); }
    .directory-browser-header { display: flex; flex-wrap: wrap; align-items: center; padding: 8px 12px; background-color: var(--hiddenTabBackground); border-bottom: 1px solid var(--border); gap: 8px; }
    .directory-browser-header button { padding: 6px 10px; border: 1px solid var(--border); border-radius: 6px; background-color: var(--primary); color: var(--text); cursor: pointer; font-size: .9em; transition: background-color .2s, border-color .2s; display: inline-flex; align-items: center; gap: 4px; margin: 0; }
    .directory-browser-header button:hover:not(:disabled) { background-color: var(--primaryHover); border-color: var(--border); }
    .directory-browser-header button:disabled { opacity: .6; cursor: not-allowed; }
    .directory-breadcrumb-container { padding: 8px 12px; background-color: var(--hiddenTabBackground); border-bottom: 1px solid var(--border); font-size: .9em; }
    .breadcrumb-navigation { display: flex; align-items: center; flex-wrap: wrap; gap: 2px; }
    .breadcrumb-item { color: var(--text); padding: 2px 6px; border-radius: 4px; transition: background-color .2s ease, color .2s ease; }
    .breadcrumb-item:not(.current) { color: var(--secondary); cursor: pointer; }
    .breadcrumb-item:not(.current):hover { background-color: var(--primaryHover); color: var(--text); }
    .breadcrumb-item.current { color: var(--primary); background-color: var(--primaryHover); font-weight: bold; cursor: default; }
    .breadcrumb-separator { color: var(--secondary); user-select: none; }
    .directory-tree-container { flex-grow: 1; overflow-y: auto; padding: 8px; max-height: 70vh; width: 100%; }
    .directory-entry { padding: 4px 8px; cursor: pointer; border-radius: 4px; transition: background-color .15s ease-in-out; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; position: relative; display: flex; align-items: center; gap: 4px; }
    .directory-entry:hover { background-color: var(--primaryHover); }
    .directory-entry.selected { background-color: var(--primaryHover); font-weight: bold; color: var(--text); }
    .directory-entry.loading { opacity: .7; pointer-events: none; }
    .tree-line { background-color: var(--border); position: absolute; }
    .tree-line.vertical { width: 1px; }
    .tree-line.horizontal { height: 1px; }
    .expand-control { cursor: pointer; margin-right: 4px; user-select: none; font-size: 12px; width: 24px; height: 24px; display: inline-flex; align-items: center; justify-content: center; border-radius: 4px; transition: background-color .2s ease; flex: 0 0 24px; }
    .entry-icon { color: var(--primary); font-size: 1.1em; }
    .entry-name { color: var(--text); font-size: .95em; }
    .entry-selected { color: var(--primary); font-size: 1.1em; margin-left: auto; }
    .directory-error, .directory-empty { padding: 16px; text-align: center; color: var(--secondary); }
    .directory-error.nas-unavailable, .directory-error[data-nas-unavailable="true"] { background: rgba(255,193,7,.1); border: 2px solid var(--warning, #ffc107); border-radius: 8px; color: var(--warning, #ffc107); font-weight: bold; font-size: 1.1em; }
    .directory-loading-initial { display: flex; justify-content: center; align-items: center; min-height: 100px; padding: 20px; }
    .directory-loading-initial[hidden] { display: none; }
    .toggle-pin-button { position: relative; width: 46px !important; height: 24px !important; padding: 0 !important; border-radius: 24px !important; transition: all .3s ease !important; overflow: hidden; border: none !important; display: flex !important; align-items: center !important; justify-content: center !important; background-color: var(--error) !important; }
    .toggle-pin-button::before { content: ""; position: absolute; height: 18px; width: 18px; left: 3px; top: 3px; background-color: var(--text) !important; border-radius: 50%; transition: transform .3s ease; }
    .toggle-pin-button.active { background-color: var(--success) !important; }
    .toggle-pin-button.active::before { transform: translateX(22px); }
    .blacklist-manager { padding: 20px; display: flex; flex-direction: column; gap: 20px; min-width: 400px; max-width: 600px; background: var(--background); border-radius: 8px; }
    .blacklist-entries { display: flex; flex-direction: column; gap: 8px; padding-right: 8px; }
    .blacklist-entry { display: flex; align-items: center; padding: 8px 12px; background: var(--hiddenTabBackground); border-radius: 8px; gap: 12px; }
    .entry-path { flex: 1; font-family: monospace; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .remove-entry { background: none; border: none; color: var(--text); font-size: 1.2em; cursor: pointer; padding: 4px 8px; border-radius: 8px; }
    .blacklist-controls, .add-entry { display: flex; gap: 8px; }
    .blacklist-controls { flex-direction: column; }
    .entry-input { flex: 1; padding: 8px 12px; border: 1px solid var(--border); border-radius: 4px; background: var(--background); color: var(--text); font-family: monospace; }
    .add-button, .submit-button, .clear-history-button { padding: 8px 12px; background: var(--primary); color: var(--text); border: none; border-radius: 8px; cursor: pointer; font-size: .9em; }
    .submit-button, .clear-history-button { width: 100%; }
    .clear-history-button { background: var(--error); margin-top: 4px; }
    .upload-history-loading { display: flex; justify-content: center; align-items: center; min-height: 200px; width: 100%; }
    .upload-history-modal-content { display: flex; flex-direction: column; max-height: 90vh; min-height: 200px; min-width: 300px; width: 100%; }
    .upload-history-list { flex: 1; overflow-y: auto; padding: 4px; height: calc(90vh - 130px); }
    .uploadHistoryModal.empty { padding: 10px; width: auto; height: auto; display: flex; justify-content: center; align-items: center; }
    .upload-history-empty-message { font-size: 1.2rem; text-align: center; color: #666; }
    .history-item { padding: 8px; margin: 4px 0; border-radius: 4px; font-size: .9em; font-family: monospace; word-break: break-word; width: 100%; box-sizing: border-box; }
    .history-item.success { background: color-mix(in srgb, var(--status-up) 20%, transparent); border-left: 3px solid var(--status-up); color: var(--status-up); }
    .history-item.error { background: color-mix(in srgb, var(--status-down) 20%, transparent); border-left: 3px solid var(--status-down); color: var(--status-down); }
    .pin-modal-form { display: flex; flex-direction: column; gap: 15px; }
    .pin-modal-form p { margin: 0; color: var(--secondary); line-height: 1.4; }
    .pin-input { padding: 10px; border-radius: 4px; border: 1px solid var(--border); background: var(--background); color: var(--text); font-size: 1rem; }
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
            <button type="button" class="indicator loading internet-indicator" data-indicator="internet" data-modal-kind="internet" data-modal-title="Internet Status" data-internet-status-indicator aria-label="Checking Internet Status" title="Checking internet connection..."><svg class="indicator-icon indicator-icon-plug" data-packed-icon="plug" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 2h2v5h2V2h2v5h2V2h2v7a5 5 0 0 1-4 4.9V17h3v2h-3v3h-2v-3H8v-2h3v-3.1A5 5 0 0 1 7 9V2z"/></svg><svg class="indicator-icon indicator-icon-spinner" data-packed-icon="spinner" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2a10 10 0 0 1 10 10h-3a7 7 0 0 0-7-7V2z"/></svg></button>
            <button type="button" class="indicator warn openvpn-indicator" data-indicator="openvpn" data-modal-kind="openvpn" data-modal-title="VPN & Transmission Configuration" aria-label="VPN & Transmission Configuration" title="VPN & Transmission Configuration"><svg class="indicator-icon" data-packed-icon="lock" viewBox="0 0 24 24" aria-hidden="true"><path d="M7 10V8a5 5 0 0 1 10 0v2h2v11H5V10h2zm2 0h6V8a3 3 0 0 0-6 0v2zm2 4v3h2v-3h-2z"/></svg></button>
            <button type="button" class="indicator warn services-indicator" data-indicator="services" data-modal-kind="services" data-modal-title="Services Status" aria-label="Services Status" title="Services Status"><svg class="indicator-icon" data-packed-icon="server" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 3h16v8H4V3zm2 2v4h12V5H6zm-2 8h16v8H4v-8zm2 2v4h12v-4H6zm9-8h2v2h-2V7zm0 10h2v2h-2v-2z"/></svg></button>
            <button type="button" class="indicator power-indicator" data-indicator="power-meter" data-modal-kind="power-meter" data-modal-title="Power Consumption" aria-label="Measuring Power Usage" title="Measuring power usage"><svg class="indicator-icon" data-packed-icon="bolt" viewBox="0 0 24 24" aria-hidden="true"><path d="M13 2 4 14h7l-1 8 10-13h-7l1-7z"/></svg><span class="power-value-small" data-power-indicator-value><span class="power-value-small-number">—</span><span class="power-value-small-unit">W</span></span></button>
          </div>
        </div>
        <div class="header-right">
          <button type="button" class="theme-button" data-theme-button data-admin-only="true" hidden title="Current theme: light. Click to switch theme."><span>light</span></button>
          <button type="button" class="change-admin-pin-button" data-change-pin-button data-admin-only="true" hidden>Change PIN</button>
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
      <section class="pane" id="pane-admin" data-pane-panel="admin" role="tabpanel" aria-label="Admin">

        <div class="admin-tablet admin-visual-port" data-admin-quarry="flask-react-admin" data-admin-quarry-button-total="74" data-admin-only="true" data-admin-viewport="admin" data-admin-visual-port="one-to-one-best-effort">
          <section class="system-controls-container" data-admin-quarry-group="system-controls" aria-label="System controls">
            <span class="admin-quarry-note" data-admin-quarry-count-readback hidden>74 buttons</span>
            <div class="system-controls" data-admin-action-strip="single-row" data-admin-action-strip-count="7">
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="1" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="1" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">▣</span><span>Hard Drive Test</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="2" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">⬇</span><span>Update</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="3" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">⟳</span><span>Restart</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="4" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">⏻</span><span>Shutdown</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="5" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="5" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">↻</span><span>Restart Website</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="6" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="6" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">▤</span><span>View Logs</span></button>
              <button type="button" class="system-controls-btn admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="7" data-admin-quarry-source="components/SystemControls.tsx" data-admin-quarry-placement="main admin action row" data-admin-quarry-local-index="7" data-stub-action="true" aria-disabled="true" ><span class="admin-action-icon">◆</span><span>Install Certificate</span></button>
            </div>
            <span class="admin-quarry-note" hidden>7 buttons · system-controls</span>
            <div class="system-service-controls" data-admin-service-controls data-state-source="/api/services/data">
              <div class="ssh-controls">
                <div class="ssh-control" data-service-card="ssh-password-authentication" data-state-field="ssh.password_auth_enabled" data-state-source="/api/services/data">
                  __ADMIN_SSH_PASSWORD_CARD__
                </div>
                <div class="ssh-control" data-service-card="ssh-service" data-state-field="sshd.running" data-state-source="/api/services/data">
                  __ADMIN_SSH_SERVICE_CARD__
                </div>
              </div>
              <div class="samba-control" data-service-card="samba-file-sharing" data-state-field="samba.running" data-state-source="/api/services/data">
                __ADMIN_SAMBA_SERVICE_CARD__
              </div>
            </div>
          </section>

          <section class="key-manager" data-admin-quarry-group="key-manager">
            <h3><span class="admin-action-icon">⚿</span> Key Management</h3>
            <div class="key-manager-content">
              <div class="key-manager-left">
                <div class="security-status">
                  <div class="status-item"><span class="status-icon secure">🛡</span><div class="status-details"><p>This is the key to your vault. When you boot your HOMESERVER and visit home.arpa, this is what unlocks your encrypted storage system - just like unlocking your smartphone. Your /vault partition contains the sensitive keys stored on the device. Unlock the vault and everything HOMESERVER specifically stores is accessible. This is the device's master key.</p></div></div>
                </div>
              </div>
              <div class="key-manager-right"><div class="key-actions">
                <button type="button" class="action-button create-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="21" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="2" data-stub-action="true" aria-disabled="true" ><span>+ Create New Key</span></button>
                <button type="button" class="action-button update-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="22" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="3" data-stub-action="true" aria-disabled="true" ><span>⟳ Update Key on Drive</span></button>
                <button type="button" class="action-button admin-password-button admin-quarry-button" data-admin-quarry-button data-admin-quarry-index="23" data-admin-quarry-source="components/KeyManager.tsx" data-admin-quarry-placement="key manager card" data-admin-quarry-local-index="4" data-stub-action="true" aria-disabled="true" ><span>🔒 Admin Password</span></button>
              </div></div>
            </div>
            <span class="admin-quarry-note" hidden>3 buttons · key-manager</span>
          </section>

          <section class="disk-manager" data-admin-quarry-group="disk-manager">
            <div class="disk-manager-container">
              <div class="disk-column"><h4>Available Devices</h4><div class="disk-list" data-admin-devices-readback="/api/services/data">__ADMIN_AVAILABLE_DEVICES__</div></div>
              <div class="disk-column"><h4>Mount Destinations</h4><div class="disk-list" data-admin-mounts-readback="/api/services/data">__ADMIN_MOUNT_DESTINATIONS__</div></div>
            </div>
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
      <section class="pane active" id="pane-stats" data-pane-panel="stats" role="tabpanel" aria-label="Stats">
        <div class="stats-tablet" data-stats-viewport data-react-quarry="StatsTablet" data-identity-standard="one-to-one">
          <div class="stat-element" data-stat-element-id="cpu-chart" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="cpu-chart" data-visible="true" aria-label="Hide CPU Usage & Load">👁</button><h3 class="stat-title">CPU Usage &amp; Load</h3></div>
            <div class="stat-content"><div class="cpu-stats-container"><div class="cpu-chart" data-chartjs-chart="cpu" data-chart-authority="serverbox-original-homeserver-stats"><div class="chart-container" id="cpu-chart-container"><canvas id="cpuChart" class="coronatio-chart-canvas" data-full-width-canvas="true" data-chart-left-axis="percent-suffix" data-chart-right-axis="celsius-suffix"></canvas></div></div><div class="load-averages"><div class="load-average-values"><div class="load-average-item"><span class="load-label">1 min:</span><span class="load-value" id="load-1min">—</span></div><div class="load-average-item"><span class="load-label">5 min:</span><span class="load-value" id="load-5min">—</span></div><div class="load-average-item"><span class="load-label">15 min:</span><span class="load-value" id="load-15min">—</span></div></div></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="network" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="network" data-visible="true" aria-label="Hide Network Traffic (WAN)">👁</button><h3 class="stat-title">Network Traffic (WAN)</h3></div>
            <div class="stat-content"><div class="network-stats-container"><div class="network-speed-chart" data-chartjs-chart="network" data-chart-authority="serverbox-original-homeserver-stats"><div class="chart-container" id="network-chart-container"><canvas id="networkChart" class="coronatio-chart-canvas" data-full-width-canvas="true" data-chart-left-axis="byte-rate-suffix" data-chart-right-axis="byte-rate-suffix" data-synchronized-axes="true"></canvas></div></div><div class="network-interfaces"><table class="network-interfaces-table"><thead><tr><th>Interface</th><th>Total Received</th><th>Total Sent</th></tr></thead><tbody data-network-interfaces></tbody></table></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="io-section" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="io-section" data-visible="true" aria-label="Hide Disk I/O">👁</button><h3 class="stat-title">Disk I/O</h3></div>
            <div class="stat-content"><div class="disk-io-chart"><div class="device-controls" id="io-drive-selector" data-device-controls data-original-control="drive-checkbox"></div><div class="chart-container" id="disk-io-chart-container"><canvas id="io-chart" class="coronatio-chart-canvas" data-full-width-canvas="true"></canvas></div><div class="io-chart-legend" id="io-chart-legend"></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="memory" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="memory" data-visible="true" aria-label="Hide Memory Usage">👁</button><h3 class="stat-title">Memory Usage</h3></div>
            <div class="stat-content"><div class="memory-stats"><div class="memory-current"><div class="memory-label">RAM</div><div class="memory-bar"><div class="memory-bar-fill" id="memory-bar-fill"><span class="memory-text" id="memory-percent">—</span></div></div><div class="memory-details"><div id="memory-used">Used: —</div><div id="memory-available">Available: —</div><div id="memory-total">Total: —</div></div></div><div class="memory-current"><div class="memory-label">Swap</div><div class="memory-bar"><div class="memory-bar-fill memory-bar-fill-swap" id="swap-bar-fill"><span class="memory-text" id="swap-percent">—</span></div></div><div class="memory-details"><div id="swap-used">Used: —</div><div id="swap-free">Free: —</div><div id="swap-total">Total: —</div></div></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="disk-usage" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="disk-usage" data-visible="true" aria-label="Hide Disk Usage">👁</button><h3 class="stat-title">Disk Usage</h3></div>
            <div class="stat-content"><div class="disk-usage-stats" data-disk-usage-stats></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="kea-leases" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="kea-leases" data-visible="true" aria-label="Hide DHCP Leases">👁</button><h3 class="stat-title">DHCP Leases</h3></div>
            <div class="stat-content"><div class="kea-leases-table"><table><thead><tr><th>Device Note</th><th>Hostname</th><th>IP Address</th><th>MAC Address</th></tr></thead><tbody data-kea-leases><tr><td colspan="4">Loading Kea leases...</td></tr></tbody></table></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="process-usage" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="process-usage" data-visible="true" aria-label="Hide CPU Usage by Process">👁</button><h3 class="stat-title">CPU Usage by Process</h3></div>
            <div class="stat-content"><div class="process-usage-list" data-process-usage-list><p>Loading process usage...</p></div></div>
          </div>
        </div>
      </section>
      <section class="pane" id="pane-portals" data-pane-panel="portals" role="tabpanel" aria-label="Portals">
        <div class="portal-grid" data-portals-grid data-portals-source="/api/portals">
          <article class="card portal-card portal-loading" data-portals-loading><div><h2>Admitted services</h2><p>Reading homeserver.json portal entries.</p></div></article>
        </div>
      </section>
      <section class="pane" id="pane-upload" data-pane-panel="upload" role="tabpanel" aria-label="Upload">
        <div class="upload-tablet" data-upload-viewport data-react-quarry="UploadTablet" data-identity-standard="one-to-one">
          <div class="upload-progress-list" data-upload-progress-list hidden></div>
          <div class="upload-controls">
            <div class="directory-browser" data-upload-regular="directory-browser" data-directory-browser>
              <div class="directory-browser-header">
                <button type="button" class="refresh-button" data-upload-refresh title="Refresh Directory Tree">🔄</button>
                <button type="button" class="admin-button force-allow-button" data-admin-only data-admin-viewport="upload" data-upload-force-allow title="Force Allow Upload (Admin)">🛡️ Allow</button>
                <button type="button" class="admin-button set-default-button" data-admin-only data-admin-viewport="upload" data-upload-set-default title="Set as Default Directory (Admin)">📌 Default</button>
                <button type="button" class="admin-button blacklist-button" data-admin-only data-admin-viewport="upload" data-upload-blacklist title="Manage Blacklist (Admin)">🚫 Blacklist</button>
                <button type="button" class="admin-button upload-history-button" data-admin-only data-admin-viewport="upload" data-upload-history title="View Upload History (Admin)">📜 History</button>
                <button type="button" class="toggle-pin-button" data-admin-only data-admin-viewport="upload" data-upload-pin-toggle title="Enable PIN requirement for uploads (Currently Off)" aria-label="Toggle PIN requirement (currently disabled)"></button>
              </div>
              <div class="directory-breadcrumb-container">
                <div class="breadcrumb-navigation" data-upload-breadcrumbs><span class="breadcrumb-item current" data-path="/mnt/nas">nas</span></div>
              </div>
              <div class="directory-error nas-unavailable" data-nas-unavailable="true" data-upload-directory-error hidden>⚠️ NAS Storage Unavailable</div>
              <div class="directory-loading-initial" data-upload-directory-loading hidden>Loading directory tree…</div>
              <div class="directory-tree-container" data-upload-tree role="tree">
                <div class="directory-entry selected" data-directory-path="/mnt/nas" role="treeitem" aria-selected="true" aria-expanded="false" style="padding-left: 12px">
                  <span class="expand-control" aria-label="Expand">▶</span><span class="entry-icon">📁</span><span class="entry-name">nas</span><span class="entry-selected" aria-hidden="true">✓</span>
                </div>
              </div>
            </div>
            <div class="file-upload-section" data-upload-regular="file-ingress" data-upload-file-section>
              <input type="file" multiple data-upload-file>
              <button type="button" data-upload-submit disabled>Upload Selected Files</button>
            </div>
          </div>
          <div class="modal-window" data-upload-history-modal hidden>
            <div class="modal-titlebar">Upload History</div>
            <div class="upload-history-modal-content"><div class="uploadHistoryModal empty"><div class="upload-history-empty-message">No upload history available</div></div><div class="upload-history-list" hidden></div><button type="button" class="clear-history-button" data-upload-clear-history disabled>Clear History</button></div>
          </div>
          <div class="modal-window" data-upload-blacklist-modal hidden>
            <div class="modal-titlebar">Manage Blacklist</div>
            <div class="blacklist-manager"><div class="blacklist-entries" data-upload-blacklist-entries></div><div class="blacklist-controls"><form class="add-entry" data-upload-blacklist-form><input type="text" class="entry-input" placeholder="Enter path to blacklist" data-upload-blacklist-input><button type="button" class="add-button" data-upload-blacklist-add>New</button></form><button type="button" class="submit-button" data-upload-blacklist-submit>Submit</button></div></div>
          </div>
          <div class="modal-window" data-upload-pin-modal hidden>
            <div class="modal-titlebar">Admin PIN Required</div>
            <form class="pin-modal-form" data-upload-pin-form><p>Please enter the admin PIN to proceed with the upload.</p><input type="text" name="username" autocomplete="username" style="position:absolute;left:-9999px;opacity:0" tabindex="-1" aria-hidden="true"><input type="password" placeholder="Admin PIN" class="pin-input" autocomplete="new-password" data-upload-pin-input></form>
            <div class="modal-actions"><button type="button" class="secondary" data-upload-pin-cancel>Cancel</button><button type="button" data-upload-pin-confirm>Confirm</button></div>
          </div>
          <pre class="readout admin-quarry-note" id="upload-readout" data-upload-receipt-readout>Upload Caduceus receipts are diagnostic evidence, not the visible progress UI.</pre>
        </div>
      </section>
      <section class="pane" id="pane-backblaze" data-pane-panel="backblaze" role="tabpanel" aria-label="backBlaze">
        <article class="card og-stub-pane" data-og-stub-pane="backblaze"><h2>backBlaze</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-wake-on-lan" data-pane-panel="wake-on-lan" role="tabpanel" aria-label="Wake on LAN">
        <article class="card og-stub-pane" data-og-stub-pane="wake-on-lan"><h2>Wake on LAN</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-test" data-pane-panel="test" role="tabpanel" aria-label="Test">
        <article class="card og-stub-pane" data-og-stub-pane="test"><h2>Test</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-chia-mining" data-pane-panel="chia-mining" role="tabpanel" aria-label="Chia Mining">
        <article class="card og-stub-pane" data-og-stub-pane="chia-mining"><h2>Chia Mining</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-dhcp" data-pane-panel="dhcp" role="tabpanel" aria-label="DHCP">
        <article class="card og-stub-pane" data-og-stub-pane="dhcp"><h2>DHCP</h2><p>not yet ported</p></article>
      </section>
      <section class="pane" id="pane-youtube" data-pane-panel="youtube" role="tabpanel" aria-label="YouTube">
        <article class="card og-stub-pane" data-og-stub-pane="youtube"><h2>YouTube</h2><p>not yet ported</p></article>
      </section>
      __TESTTAB__
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
    const tabState = Object.assign({ starredTab: 'stats', hiddenTabs: ['chia-mining', 'dhcp', 'youtube'] }, loadTabState());
    const headerStateKey = 'coronatio.flask-react-header.v1';
    const preferredThemeKey = 'preferred-theme';
    const themeDataKey = 'themeData';
    let themeCatalog = { default: 'light', themes: {} };
    let themes = [];
    const savedHeaderState = (() => { try { return JSON.parse(localStorage.getItem(headerStateKey) || '{}'); } catch (_) { return {}; } })();
    const savedPreferredTheme = localStorage.getItem(preferredThemeKey);
    const headerState = { theme: savedPreferredTheme || savedHeaderState.theme || 'light', isAdmin: false };
    const saveHeaderState = () => {
      localStorage.setItem(headerStateKey, JSON.stringify({ theme: headerState.theme }));
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
    const internetIndicator = document.querySelector('[data-internet-status-indicator]');
    const internetState = { status: 'loading', publicIp: undefined, ipDetails: undefined, speedTestResults: null, speedTestError: null, isSpeedTesting: false };
    let modalMode = 'enter';
    function themeToCss(theme) {
      if (!theme) return '';
      const aliasMap = { background: '--background', text: '--text', primary: '--primary', primaryHover: '--primaryHover', secondary: '--secondary', accent: '--accent', error: '--error', success: '--success', warning: '--warning', border: '--border', statusUp: '--status-up', statusDown: '--status-down', statusPartial: '--status-partial', statusUnknown: '--status-unknown', hiddenTabBackground: '--hiddenTabBackground', hiddenTabText: '--hiddenTabText' };
      const lines = [];
      Object.entries(theme).forEach(([key, value]) => {
        lines.push('  --theme-' + key + ': ' + value + ';');
        if (aliasMap[key]) lines.push('  ' + aliasMap[key] + ': ' + value + ';');
      });
      return ':root {\n' + lines.join('\n') + '\n}';
    }
    function themeLabel(name) {
      return name;
    }
    function renderThemeChoices() {
      if (!themeChoiceRow) return;
      themeChoiceRow.innerHTML = '';
      themes.forEach(name => {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'theme-choice';"####
}
