const CROWN_SHELL_STYLESHEET_PATH: &str = "/static/crown/crown.css";
const CROWN_HTMX_SCRIPT_PATH: &str = "/static/vendor/htmx.min.js";
const CROWN_SHELL_SCRIPT_PATH: &str = "/static/crown/chrome.js";
const CROWN_CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self' data:; base-uri 'self'; frame-ancestors 'self'";
const CROWN_HTMX_JS: &str = include_str!("../../../static/vendor/htmx.min.js");

const CROWN_SHELL_CSS: &str = r#"
:root {
  color-scheme: dark;
  --ux-color-garden-0: #07130f;
  --ux-color-garden-1: #0c1f19;
  --ux-color-crown: #d9b66f;
  --ux-color-crown-bright: #f6d992;
  --ux-color-leaf: #3ddc97;
  --ux-color-sky: #7ab7ff;
  --ux-color-danger: #ff7a90;
  --ux-surface-0: #07130f;
  --ux-surface-1: rgba(14, 34, 29, 0.94);
  --ux-surface-2: rgba(23, 51, 43, 0.88);
  --ux-surface-underlay: radial-gradient(circle at top left, rgba(61, 220, 151, 0.2), transparent 35%), linear-gradient(135deg, #091711, #101f1b 70%);
  --ux-text-strong: #f7f1df;
  --ux-text: #d9e7dd;
  --ux-text-muted: #92aa9d;
  --ux-outline: rgba(217, 182, 111, 0.32);
  --ux-outline-soft: rgba(146, 170, 157, 0.22);
  --ux-space-1: 0.25rem;
  --ux-space-2: 0.5rem;
  --ux-space-3: 0.75rem;
  --ux-space-4: 1rem;
  --ux-space-5: 1.5rem;
  --ux-space-6: 2rem;
  --ux-radius-sm: 0.5rem;
  --ux-radius-md: 0.85rem;
  --ux-radius-lg: 1.25rem;
  --ux-radius-pill: 999px;
  --ux-font-body: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  --ux-font-display: "Cinzel", Georgia, serif;
  --ux-type-small: 0.82rem;
  --ux-type-body: 0.95rem;
  --ux-type-title: 1.2rem;
  --ux-type-crown: 1.55rem;
  --ux-shadow-crown: 0 24px 70px rgba(0, 0, 0, 0.35);
}

* { box-sizing: border-box; }
html, body { margin: 0; height: 100%; overflow: hidden; background: var(--ux-surface-0); color: var(--ux-text); font-family: var(--ux-font-body); }
body { height: 100vh; }
button { font: inherit; }
.crown-shell { height: 100vh; min-height: 0; display: grid; grid-template-columns: minmax(14rem, 18rem) minmax(0, 1fr); background: var(--ux-surface-underlay); overflow: hidden; }
.crown-rail { min-height: 0; overflow-y: auto; border-right: 1px solid var(--ux-outline); background: rgba(7, 19, 15, 0.84); padding: var(--ux-space-5); display: flex; flex-direction: column; gap: var(--ux-space-5); }
.crown-mark { display: grid; gap: var(--ux-space-2); }
.crown-mark strong { color: var(--ux-color-crown-bright); font-family: var(--ux-font-display); font-size: var(--ux-type-crown); letter-spacing: 0.04em; }
.crown-mark span { color: var(--ux-text-muted); font-size: var(--ux-type-small); line-height: 1.45; }
.crown-tab-rail { display: grid; gap: var(--ux-space-2); }
.crown-tab { border: 1px solid var(--ux-outline-soft); border-radius: var(--ux-radius-md); background: var(--ux-surface-1); color: var(--ux-text); text-align: left; padding: var(--ux-space-3); display: grid; gap: var(--ux-space-1); cursor: pointer; }
.crown-tab:hover, .crown-tab[aria-selected="true"] { border-color: var(--ux-color-crown); color: var(--ux-text-strong); box-shadow: inset 0 0 0 1px rgba(217, 182, 111, 0.22); }
.crown-tab__title { font-weight: 700; }
.crown-tab__kind { color: var(--ux-text-muted); font-size: var(--ux-type-small); }
.crown-main { min-width: 0; min-height: 0; padding: var(--ux-space-5); display: grid; grid-template-rows: auto minmax(0, 1fr); gap: var(--ux-space-4); overflow: hidden; }
.crown-topline { border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-lg); background: rgba(12, 31, 25, 0.72); box-shadow: var(--ux-shadow-crown); padding: var(--ux-space-4); display: flex; justify-content: space-between; gap: var(--ux-space-4); align-items: center; }
.crown-topline h1 { margin: 0; color: var(--ux-text-strong); font-size: var(--ux-type-title); }
.crown-topline p { margin: var(--ux-space-1) 0 0; color: var(--ux-text-muted); font-size: var(--ux-type-body); }
.crown-stage { position: relative; min-height: 0; height: 100%; border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-lg); overflow-y: auto; overflow-x: hidden; background: rgba(6, 16, 13, 0.72); }
.crown-layer-zero { position: absolute; inset: 0; z-index: 0; display: grid; place-items: center; padding: var(--ux-space-6); background: var(--ux-surface-underlay); opacity: 0; transition: opacity 160ms ease; pointer-events: none; }
.crown-stage[data-underlay-state="visible"] .crown-layer-zero { opacity: 1; pointer-events: auto; }
.crown-underlay-card { max-width: 40rem; border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-lg); background: rgba(7, 19, 15, 0.9); padding: var(--ux-space-5); text-align: left; display: grid; gap: var(--ux-space-3); }
.crown-underlay-card h2 { margin: 0; color: var(--ux-color-crown-bright); font-family: var(--ux-font-display); }
.crown-underlay-card p { margin: 0; color: var(--ux-text-muted); line-height: 1.55; }
.crown-layer-one { position: relative; z-index: 1; min-height: 100%; height: 100%; display: grid; }
.crown-view-panel { min-height: 100%; height: 100%; padding: var(--ux-space-5); }
.crown-view-panel[hidden] { display: none; }
.crown-view-panel[data-empty="true"] { pointer-events: none; }
.crown-fragment { background: var(--ux-surface-1); color: var(--ux-text); border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-lg); padding: var(--ux-space-4); display: grid; gap: var(--ux-space-4); }
.crown-fragment h2, .crown-fragment h3, .crown-fragment h4 { margin: 0; color: var(--ux-text-strong); }
.crown-fragment h2 { font-family: var(--ux-font-display); }
.crown-fragment p { margin: 0; color: var(--ux-text-muted); line-height: 1.45; }
.crown-fragment__button { background: var(--ux-color-crown); color: var(--ux-surface-0); border: 0; border-radius: var(--ux-radius-pill); padding: var(--ux-space-2) var(--ux-space-4); }
.crown-block-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr)); gap: var(--ux-space-3); }
.crown-block-card, .crown-status-strip, .crown-token-sample, .crown-readback { border: 1px solid var(--ux-outline-soft); border-radius: var(--ux-radius-md); background: var(--ux-surface-2); padding: var(--ux-space-3); }
.crown-status-strip { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: var(--ux-space-3); }
.crown-card-head { display: flex; align-items: flex-start; justify-content: space-between; gap: var(--ux-space-3); }
.crown-chip-row { display: flex; flex-wrap: wrap; gap: var(--ux-space-2); align-items: center; }
.crown-chip { border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-pill); padding: var(--ux-space-1) var(--ux-space-2); color: var(--ux-color-crown-bright); background: rgba(61, 220, 151, 0.12); font-size: var(--ux-type-small); overflow-wrap: normal; word-break: normal; white-space: nowrap; }
.crown-chip[data-posture="muted"] { color: var(--ux-text-muted); background: rgba(146, 170, 157, 0.1); }
.crown-definition-grid { display: grid; gap: var(--ux-space-2); }
.crown-definition-row { display: grid; grid-template-columns: minmax(5.5rem, .32fr) minmax(0, 1.35fr); gap: var(--ux-space-3); align-items: start; }
.crown-definition-row dt { color: var(--ux-text-muted); }
.crown-definition-row dd { margin: 0; color: var(--ux-text-strong); min-width: 0; overflow-wrap: normal; word-break: normal; hyphens: manual; }
.crown-service-chip-row { min-width: 0; }
.crown-service-chip { max-width: 100%; overflow: hidden; text-overflow: ellipsis; }
.crown-headline-number { display: block; color: var(--ux-color-crown-bright); font-size: var(--ux-type-crown); font-weight: 800; }
.crown-link-row { display: flex; flex-wrap: wrap; gap: var(--ux-space-2); }
.crown-link-row a { color: var(--ux-color-sky); text-decoration: none; }
.crown-token-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr)); gap: var(--ux-space-3); }
.crown-token-chip { min-height: 4.5rem; display: grid; align-content: end; gap: var(--ux-space-1); border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-md); padding: var(--ux-space-3); background: var(--token-value, var(--ux-surface-2)); color: var(--ux-text-strong); }
.token-surface-0 { --token-value: var(--ux-surface-0); }
.token-surface-1 { --token-value: var(--ux-surface-1); }
.token-crown { --token-value: var(--ux-color-crown); }
.token-leaf { --token-value: var(--ux-color-leaf); }
.token-sky { --token-value: var(--ux-color-sky); }
.token-outline { --token-value: var(--ux-outline); }
.crown-type-sample { display: grid; gap: var(--ux-space-2); }
.crown-radius-samples { display: flex; flex-wrap: wrap; gap: var(--ux-space-3); }
.crown-radius-sample { min-width: 7rem; min-height: 3rem; border: 1px solid var(--ux-outline); background: var(--ux-surface-2); display: grid; place-items: center; padding: var(--ux-space-2); }
.crown-radius-sample[data-radius="sm"] { border-radius: var(--ux-radius-sm); }
.crown-radius-sample[data-radius="md"] { border-radius: var(--ux-radius-md); }
.crown-radius-sample[data-radius="lg"] { border-radius: var(--ux-radius-lg); }
.crown-readback summary { cursor: pointer; color: var(--ux-color-crown-bright); }
.crown-readback pre { max-height: 18rem; overflow: auto; margin: var(--ux-space-3) 0 0; color: var(--ux-text); white-space: pre-wrap; overflow-wrap: anywhere; }
.crown-iframe-guest { display: grid; grid-template-rows: auto minmax(0, 1fr); gap: var(--ux-space-3); min-height: 100%; height: 100%; }
.crown-iframe-guest__chrome { display: flex; align-items: center; justify-content: space-between; gap: var(--ux-space-3); border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-md); background: var(--ux-surface-1); padding: var(--ux-space-3) var(--ux-space-4); color: var(--ux-text); }
.crown-iframe-guest__chrome h2 { margin: 0; color: var(--ux-text-strong); font-size: var(--ux-type-title); }
.crown-iframe-guest__chip { border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-pill); padding: var(--ux-space-1) var(--ux-space-3); color: var(--ux-color-crown-bright); background: rgba(61, 220, 151, 0.12); font-size: var(--ux-type-small); }
.crown-iframe-guest__frame { width: 100%; height: 100%; min-height: 0; border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-lg); background: var(--ux-surface-0); }
@media (max-width: 760px) { .crown-shell { grid-template-columns: 1fr; } .crown-rail { border-right: 0; border-bottom: 1px solid var(--ux-outline); } }
"#;

const CROWN_SHELL_JS: &str = r#"
// @ts-check
(() => {
  const htmxOrgan = window.htmx;
  if (htmxOrgan && htmxOrgan.config) {
    htmxOrgan.config.allowScriptTags = false;
    htmxOrgan.config.includeIndicatorStyles = false;
    htmxOrgan.config.selfRequestsOnly = true;
  }

  /** @param {Event} event */
  function eventViewportPanel(event) {
    const detail = /** @type {{ target?: Element, elt?: Element }} */ (event.detail || {});
    const candidate = detail.target || detail.elt || event.target;
    return candidate instanceof Element ? candidate.closest('[data-view-panel]') : null;
  }

  /** @param {Element | null} panel */
  function panelTabId(panel) {
    return panel instanceof HTMLElement ? panel.dataset.viewPanel || 'unknown' : 'unknown';
  }

  /** @param {'timeout' | 'upstream-error' | 'proxy-unreachable' | string} faultKind */
  function writeUnderlayFault(faultKind) {
    const underlay = document.querySelector('[data-crown-underlay]');
    const underlayFault = underlay ? underlay.querySelector('[data-underlay-fault-kind]') : null;
    if (underlayFault instanceof HTMLElement) {
      underlayFault.dataset.underlayFaultKind = faultKind;
      underlayFault.textContent = faultKind;
    }
  }

  /**
   * Detached response parser: when the crown sends typed fault markup on an HTMX
   * error response, its data-cartridge-fault-kind is stronger than the event kind.
   * @param {Event} event
   * @param {'timeout' | 'upstream-error' | 'proxy-unreachable' | string} fallback
   */
  function faultKindFromResponse(event, fallback) {
    const detail = /** @type {{ xhr?: XMLHttpRequest }} */ (event.detail || {});
    const text = detail.xhr && typeof detail.xhr.responseText === 'string' ? detail.xhr.responseText : '';
    if (!text.includes('data-cartridge-fault-kind')) return fallback;
    const template = document.createElement('template');
    template.innerHTML = text;
    const fault = template.content.querySelector('[data-cartridge-fault-kind]');
    return fault instanceof HTMLElement && fault.dataset.cartridgeFaultKind ? fault.dataset.cartridgeFaultKind : fallback;
  }

  /**
   * CORO-004 typed CartridgeFaultReceipt front seam: guest failure clears only the faulted pane.
   * @param {'timeout' | 'upstream-error' | 'proxy-unreachable' | string} faultKind
   * @param {Event} event
   */
  function emitCartridgeFaultReceipt(faultKind, event) {
    const panel = eventViewportPanel(event);
    if (panel instanceof HTMLElement) {
      panel.replaceChildren();
      panel.dataset.viewportFaulted = 'true';
      panel.dataset.empty = 'true';
      panel.hidden = true;
    }
    const activeTab = panelTabId(panel);
    document.documentElement.dataset.cartridgeFaultReceipt = 'typed';
    document.documentElement.dataset.cartridgeFaultLast = faultKind;
    document.documentElement.dataset.cartridgeFaultTab = activeTab;
    if (stage) {
      stage.dataset.underlayState = 'visible';
      stage.dataset.underlayFaultTab = activeTab;
    }
    writeUnderlayFault(faultKind);
  }

  document.body.addEventListener('htmx:timeout', (event) => emitCartridgeFaultReceipt('timeout', event));
  document.body.addEventListener('htmx:responseError', (event) => emitCartridgeFaultReceipt(faultKindFromResponse(event, 'upstream-error'), event));
  document.body.addEventListener('htmx:sendError', (event) => emitCartridgeFaultReceipt('proxy-unreachable', event));

  /** @type {NodeListOf<HTMLButtonElement>} */
  const tabs = document.querySelectorAll('[data-crown-tab]');
  /** @type {NodeListOf<HTMLElement>} */
  const panels = document.querySelectorAll('[data-view-panel]');
  /** @type {HTMLElement | null} */
  const stage = document.querySelector('[data-crown-stage]');

  /** @param {HTMLElement} panel */
  function panelIsEmptyOrFaulted(panel) {
    return panel.dataset.viewportFaulted === 'true' || panel.innerHTML.trim().length === 0;
  }

  /** @param {string} id */
  function selectViewport(id) {
    let activePanel = null;
    document.body.addEventListener('htmx:afterSwap', (event) => {
    const panel = eventViewportPanel(event);
    if (!(panel instanceof HTMLElement)) return;
    const fault = panel.querySelector('[data-cartridge-fault="true"]');
    if (fault instanceof HTMLElement) {
      const kind = fault.dataset.cartridgeFaultKind || 'upstream-error';
      panel.replaceChildren();
      panel.dataset.viewportFaulted = 'true';
      panel.dataset.empty = 'true';
      panel.hidden = true;
      if (stage) {
        stage.dataset.underlayState = 'visible';
        stage.dataset.underlayFaultTab = panel.dataset.viewPanel || 'unknown';
      }
      writeUnderlayFault(kind);
      return;
    }
    panel.dataset.viewportFaulted = 'false';
    panel.dataset.empty = panelIsEmptyOrFaulted(panel) ? 'true' : 'false';
    if (!panel.hidden && stage) stage.dataset.underlayState = panelIsEmptyOrFaulted(panel) ? 'visible' : 'occupied';
  });

  tabs.forEach((tab) => {
      const active = tab.dataset.crownTab === id;
      tab.setAttribute('aria-selected', active ? 'true' : 'false');
      tab.tabIndex = active ? 0 : -1;
    });
    panels.forEach((panel) => {
      const active = panel.dataset.viewPanel === id;
      panel.hidden = !active;
      panel.dataset.empty = panelIsEmptyOrFaulted(panel) ? 'true' : 'false';
      if (active) activePanel = panel;
    });
    if (stage && activePanel) {
      stage.dataset.underlayState = panelIsEmptyOrFaulted(activePanel) ? 'visible' : 'occupied';
    }
  }

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => selectViewport(tab.dataset.crownTab || ''));
    tab.addEventListener('keydown', (event) => {
      if (event.key !== 'ArrowDown' && event.key !== 'ArrowUp') return;
      event.preventDefault();
      const list = Array.from(tabs);
      const index = list.indexOf(tab);
      const offset = event.key === 'ArrowDown' ? 1 : -1;
      const next = list[(index + offset + list.length) % list.length];
      next.focus();
      selectViewport(next.dataset.crownTab || '');
    });
  });

  const first = document.querySelector('[data-crown-tab][aria-selected="true"]') || tabs[0];
  if (first instanceof HTMLElement) selectViewport(first.dataset.crownTab || '');
})();
"#;

#[derive(Debug, Clone)]
struct CrownShellTab {
    id: String,
    title: String,
    kind: &'static str,
    admin_only: bool,
}

fn admit_route_for_tab(tab_id: &str) -> String {
    format!("/admit/{tab_id}")
}

fn viewport_target_for_tab(tab_id: &str) -> String {
    format!("#viewport-{tab_id}")
}

fn render_crown_shell() -> String {
    render_crown_shell_with_registry(&[])
}

fn render_crown_shell_with_registry(registry_tabs: &[TabManifest]) -> String {
    let mut tabs: Vec<CrownShellTab> = native_crown_panes()
        .into_iter()
        .map(|pane| CrownShellTab {
            id: pane.id,
            title: pane.title,
            kind: "native crown pane",
            admin_only: pane.admin_only,
        })
        .collect();

    tabs.extend(registry_tabs.iter().filter(|tab| tab.enabled).map(|tab| CrownShellTab {
        id: tab.id.clone(),
        title: if tab.title.is_empty() { tab.id.clone() } else { tab.title.clone() },
        kind: "registry tab",
        admin_only: tab.admin_only,
    }));

    render_crown_shell_tabs(&tabs).into_string()
}

fn selected_crown_theme_name() -> String {
    read_first_json(&homeserver_config_candidates())
        .ok()
        .and_then(|(_source, value)| {
            value
                .get("global")
                .and_then(|global| global.get("theme"))
                .and_then(|theme| theme.get("name"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_else(|| "dark".to_string())
}

fn theme_token<'a>(theme: &'a BTreeMap<String, String>, key: &str, fallback: &'a str) -> &'a str {
    theme.get(key).map(String::as_str).unwrap_or(fallback)
}

fn crown_theme_projection_css(theme_name: &str) -> String {
    let catalog = firmware_theme_catalog(theme_name.to_string());
    let selected = catalog.themes.get(&catalog.default).or_else(|| catalog.themes.get("dark"));
    let Some(theme) = selected else { return String::new(); };
    format!(
        ":root{{--ux-surface-0:{surface0};--ux-surface-1:{surface1};--ux-surface-2:{surface2};--ux-surface-underlay:{underlay};--ux-text-strong:{text};--ux-text:{text};--ux-text-muted:{muted};--ux-outline:{outline};--ux-outline-soft:{outline_soft};--ux-color-crown:{crown};--ux-color-crown-bright:{crown_bright};--ux-color-leaf:{leaf};--ux-color-sky:{sky};--ux-color-danger:{danger};--ux-space-1:{space1};--ux-space-2:{space2};--ux-space-3:{space3};--ux-space-4:{space4};--ux-space-5:{space5};--ux-space-6:{space6};--ux-radius-sm:{radius_sm};--ux-radius-md:{radius_md};--ux-radius-lg:{radius_lg};--ux-radius-pill:{radius_pill};--ux-font-body:{font_body};--ux-type-small:{type_small};--ux-type-body:{type_body};--ux-type-title:{type_title};--ux-type-crown:{type_crown};--ux-shadow-crown:{shadow};}}",
        surface0 = theme_token(theme, "surface-0", "#0A0A0A"),
        surface1 = theme_token(theme, "surface-1", "#111827"),
        surface2 = theme_token(theme, "surface-2", "#1E293B"),
        underlay = theme_token(theme, "gradient-surface", "linear-gradient(135deg, #091711, #101f1b 70%)"),
        text = theme_token(theme, "on-surface", "#E0E0E0"),
        muted = theme_token(theme, "on-surface-muted", "#9CA3AF"),
        outline = theme_token(theme, "outline", "#6B7280"),
        outline_soft = theme_token(theme, "outline-variant", "#1E293B"),
        crown = theme_token(theme, "role-primary", "#323840"),
        crown_bright = theme_token(theme, "accent-warm", "#FBBF24"),
        leaf = theme_token(theme, "status-success", "#10B981"),
        sky = theme_token(theme, "accent-cool", "#90cff3"),
        danger = theme_token(theme, "status-error", "#F87171"),
        space1 = theme_token(theme, "spacing-xs", "0.25rem"),
        space2 = theme_token(theme, "spacing-sm", "0.5rem"),
        space3 = theme_token(theme, "spacing-md", "0.75rem"),
        space4 = theme_token(theme, "spacing-md", "1rem"),
        space5 = theme_token(theme, "spacing-lg", "1.5rem"),
        space6 = theme_token(theme, "spacing-xl", "2rem"),
        radius_sm = theme_token(theme, "radius-sm", "0.5rem"),
        radius_md = theme_token(theme, "radius-md", "0.85rem"),
        radius_lg = theme_token(theme, "radius-lg", "1.25rem"),
        radius_pill = theme_token(theme, "radius-pill", "999px"),
        font_body = theme_token(theme, "font-family", "Inter, ui-sans-serif, system-ui, sans-serif"),
        type_small = theme_token(theme, "font-size-xs", "0.82rem"),
        type_body = theme_token(theme, "font-size-base", "0.95rem"),
        type_title = theme_token(theme, "font-size-lg", "1.2rem"),
        type_crown = theme_token(theme, "font-size-xl", "1.55rem"),
        shadow = theme_token(theme, "elevation-3", "0 24px 70px rgba(0,0,0,.35)"),
    )
}

fn render_crown_shell_tabs(tabs: &[CrownShellTab]) -> maud::Markup {
    let active = tabs.first().map(|tab| tab.id.as_str()).unwrap_or("fallback");
    let selected_theme = selected_crown_theme_name();
    let theme_css = crown_theme_projection_css(&selected_theme);
    maud::html! {
        (maud::DOCTYPE)
        html lang="en" data-product="Coronatio" data-crown-law="compiled-vessel" data-theme=(selected_theme) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Coronatio" }
                link rel="stylesheet" href=(CROWN_SHELL_STYLESHEET_PATH);
                style data-crown-theme-projection="homeserver-json-default" { (maud::PreEscaped(theme_css)) }
                script defer src=(CROWN_HTMX_SCRIPT_PATH) {}
                script defer src=(CROWN_SHELL_SCRIPT_PATH) {}
            }
            body data-source-material="homeserver-main-site" {
                div.crown-shell data-crown-shell="maud" data-layer-zero="immortal-underlay" {
                    aside.crown-rail aria-label="Coronatio registry rail" {
                        div.crown-mark {
                            strong { "Coronatio" }
                            span { "The crown gathers the garden: native panes and admitted registry vessels share one immortal shell." }
                        }
                        nav.crown-tab-rail role="tablist" aria-label="Crown panes and registry tabs" {
                            @for tab in tabs {
                                button.crown-tab
                                    type="button"
                                    role="tab"
                                    data-crown-tab=(tab.id)
                                    data-admin-only=(if tab.admin_only { "true" } else { "false" })
                                    aria-selected=(if tab.id == active { "true" } else { "false" })
                                    aria-controls=(format!("viewport-{}", tab.id))
                                    hx-get=(admit_route_for_tab(&tab.id))
                                    hx-target=(viewport_target_for_tab(&tab.id))
                                    hx-swap="innerHTML"
                                    hx-trigger="click" {
                                    span.crown-tab__title { (tab.title) }
                                    span.crown-tab__kind { (tab.kind) }
                                }
                            }
                        }
                    }
                    main.crown-main {
                        header.crown-topline {
                            div {
                                h1 { "Compiled crown vessel" }
                                p { "Layer 0 remains warm beneath every viewport; layer 1 opens one admitted pane at a time." }
                            }
                            span data-crown-receipt="coro-001" { "garden crowned" }
                        }
                        section.crown-stage data-crown-stage="true" data-underlay-state="visible" aria-label="Coronatio viewport stage" {
                            div.crown-layer-zero data-layer="0" data-crown-underlay="fallback" aria-live="polite" {
                                div.crown-underlay-card {
                                    div.crown-mark {
                                        strong { "Coronatio" }
                                        span { "Recovery posture: the crown mark remains warm while a viewport is empty or faulted." }
                                    }
                                    dl.crown-definition-grid {
                                        div.crown-definition-row { dt { "StartupPhase" } dd data-underlay-startup-phase=(startup_readback().current_phase) { (startup_readback().current_phase) } }
                                        div.crown-definition-row { dt { "Service health" } dd { "shell standing; layer-1 admission isolated" } }
                                        div.crown-definition-row { dt { "Fault line" } dd { "Last pane fault: " span data-underlay-fault-kind="none" { "none" } } }
                                    }
                                }
                            }
                            div.crown-layer-one data-layer="1" {
                                @for tab in tabs {
                                    section.crown-view-panel
                                        id=(format!("viewport-{}", tab.id))
                                        role="tabpanel"
                                        data-view-panel=(tab.id)
                                        data-empty="true"
                                        hidden[tab.id != active] {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
