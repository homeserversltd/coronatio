const CROWN_HTMX_SCRIPT_PATH: &str = "/static/vendor/htmx.min.js";
const DNS_PANE: &str = r#"<section class="pane" id="pane-unbound" data-pane-panel="unbound" data-view-panel="unbound" role="tabpanel" aria-label="DNS"><article class="dns-tablet" data-dns-tablet data-admin-only="true" data-admin-viewport="unbound"><header class="dns-tablet-header"><div><h2>Local DNS</h2><p>Manage local home.arpa A records through Unbound.</p></div><button type="button" class="ui-button ui-button--secondary ui-button--small" data-dns-refresh>Refresh</button></header><p data-dns-state aria-live="polite">Ready to read local records.</p><form class="dns-form" data-dns-form><label>Hostname<input class="ui-input ui-input--medium" name="name" placeholder="app.home.arpa" required></label><label>Private IPv4<input class="ui-input ui-input--medium" name="address" placeholder="192.168.123.2" required></label><button class="ui-button ui-button--primary" type="submit">Save record</button></form><div class="dns-records" data-dns-records></div><p class="dns-empty" data-dns-empty hidden>No local A records are configured.</p></article></section>"#;
const CROWN_CHROME_SCRIPT_PATH: &str = "/static/crown/chrome.js";
const CROWN_CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; base-uri 'self'; frame-ancestors 'self'";
const CROWN_HTMX_JS: &str = include_str!("../../../static/vendor/htmx.min.js");
#[allow(dead_code)]
const SHELL_UX_INDEX_JSON: &str = include_str!("ux/index.json");
#[allow(dead_code)]
const SHELL_UX_README: &str = include_str!("ux/README.md");
#[allow(dead_code)]
const SHELL_UX_CHILDREN: &[&str] = &[
    "shell/base-and-chrome.css", "library/_badge.css", "library/_breadcrumbs.css", "library/_button.css", "library/_calendar.css", "library/_card.css", "library/_checkbox.css", "library/_collapsible.css", "library/_editable-field.css", "library/_file-input.css", "library/_icon-button.css", "library/_input.css", "library/_loading-spinner.css", "library/_modal.css", "library/_plus-button.css", "library/_progress-bar.css", "library/_row-info-tile.css", "library/_select.css", "library/_slider.css", "library/_table.css", "library/_tabs.css", "library/_text-box.css", "library/_time-picker.css", "library/_toast.css", "library/_toggle.css", "library/_visibility-toggle.css", "packs/upload.css", "packs/stats.css", "packs/portals.css", "packs/test-services.css", "packs/test-animations.css", "packs/test-config.css", "packs/test-health.css", "packs/admin.css", "packs/dhcp.css", "packs/firewall.css", "packs/unbound.css", "shell/document-2-css.css",
];
const SHELL_UX_CONTENTS: &[&str] = &[
    include_str!("ux/shell/base-and-chrome.css"),
    include_str!("ux/library/_badge.css"),
    include_str!("ux/library/_breadcrumbs.css"),
    include_str!("ux/library/_button.css"),
    include_str!("ux/library/_calendar.css"),
    include_str!("ux/library/_card.css"),
    include_str!("ux/library/_checkbox.css"),
    include_str!("ux/library/_collapsible.css"),
    include_str!("ux/library/_editable-field.css"),
    include_str!("ux/library/_file-input.css"),
    include_str!("ux/library/_icon-button.css"),
    include_str!("ux/library/_input.css"),
    include_str!("ux/library/_loading-spinner.css"),
    include_str!("ux/library/_modal.css"),
    include_str!("ux/library/_plus-button.css"),
    include_str!("ux/library/_progress-bar.css"),
    include_str!("ux/library/_row-info-tile.css"),
    include_str!("ux/library/_select.css"),
    include_str!("ux/library/_slider.css"),
    include_str!("ux/library/_table.css"),
    include_str!("ux/library/_tabs.css"),
    include_str!("ux/library/_text-box.css"),
    include_str!("ux/library/_time-picker.css"),
    include_str!("ux/library/_toast.css"),
    include_str!("ux/library/_toggle.css"),
    include_str!("ux/library/_visibility-toggle.css"),
    include_str!("ux/packs/upload.css"),
    include_str!("ux/packs/stats.css"),
    include_str!("ux/packs/portals.css"),
    include_str!("ux/packs/test-services.css"),
    include_str!("ux/packs/test-animations.css"),
    include_str!("ux/packs/test-config.css"),
    include_str!("ux/packs/test-health.css"),
    include_str!("ux/packs/admin.css"),
    include_str!("ux/packs/dhcp.css"),
    include_str!("ux/packs/firewall.css"),
    include_str!("ux/packs/unbound.css"),
    include_str!("ux/shell/document-2-css.css"),
];

fn shell_ux_css() -> String {
    SHELL_UX_CONTENTS.concat()
}

fn crown_chrome_js() -> String {
    let raw = [
        shell_document_2(),
        shell_document_3(),
        shell_document_4(),
        shell_document_4_tail(),
    ]
    .concat()
    .replace(
        "__INDICATOR_MODAL_REGISTRY__",
        &render_indicator_modal_registry(Session::Guest),
    )
    .replace("__DHCP_CLIENT__", shell_dhcp_client())
    .replace("__UNBOUND_CLIENT__", shell_unbound_client())
    .replace("__FIREWALL_CLIENT__", shell_firewall_client());
    extract_between(&raw, "<script>", "</script>").unwrap_or_default()
}

fn extract_between(source: &str, open: &str, close: &str) -> Option<String> {
    let start = source.find(open)? + open.len();
    let tail = &source[start..];
    let end = tail.rfind(close)?;
    Some(tail[..end].to_string())
}

fn remove_inline_chrome_script(shell: String) -> String {
    let Some(start) = shell.find("<script>") else { return shell; };
    let Some(close_relative) = shell[start..].rfind("</script>") else { return shell; };
    let close = start + close_relative + "</script>".len();
    let chrome_source = &shell[start + "<script>".len()..start + close_relative];
    format!(
        "{}  <script defer src=\"{}\" data-htmx-organ=\"2.0.10\"></script>\n  <script defer src=\"{}\" data-crown-chrome=\"og-htmx\"></script>\n  <template data-crown-chrome-source=\"externalized-for-csp\">{}</template>\n{}",
        &shell[..start],
        CROWN_HTMX_SCRIPT_PATH,
        CROWN_CHROME_SCRIPT_PATH,
        chrome_source,
        &shell[close..]
    )
}

fn render_crown_shell() -> String {
    render_crown_shell_for_session(Session::Guest)
}

fn remove_admin_only_pane_from_guest_shell(shell: String, pane: &str) -> String {
    let marker = format!("<section class=\"pane\" id=\"pane-{pane}\"");
    let Some(start) = shell.find(&marker) else { return shell; };
    let mut cursor = start;
    let mut depth = 0usize;
    loop {
        let next_open = shell[cursor..].find("<section").map(|offset| cursor + offset);
        let next_close = shell[cursor..].find("</section>").map(|offset| cursor + offset);
        match (next_open, next_close) {
            (Some(open), Some(close)) if open < close => {
                depth += 1;
                cursor = open + "<section".len();
            }
            (_, Some(close)) if depth > 0 => {
                depth -= 1;
                cursor = close + "</section>".len();
                if depth == 0 {
                    return format!("{}{}", &shell[..start], &shell[cursor..]);
                }
            }
            _ => return shell,
        }
    }
}

fn render_crown_shell_for_session(session: Session) -> String {
    let nav = render_plan_tabbar(session);
    let shell = [
        shell_document_1().to_string(),
        shell_ux_css(),
        shell_document_2().to_string(),
        shell_document_3().to_string(),
        shell_document_4().to_string(),
        shell_document_4_tail().to_string(),
    ]
    .concat();
    let shell = shell
        .replace("__NAV__", &nav)
        .replace("__INDICATOR_SPINE__", &render_indicator_strip(session))
        .replace("__INDICATOR_MODAL_REGISTRY__", &render_indicator_modal_registry(session))
        .replace("__TEST__", &render_test_showcase())
        .replace("__ADMIN_AVAILABLE_DEVICES__", &render_admin_available_devices_html())
        .replace("__ADMIN_MOUNT_DESTINATIONS__", &render_admin_mount_destinations_html())
        .replace("__UPLOAD_TREE_FRAGMENT__", &render_upload_tree_fragment(None, None))
        .replace("__STATS_ELEMENTS_FRAGMENT__", &render_stats_elements_fragment(session))
        .replace("__DHCP_CLIENT__", shell_dhcp_client())
        .replace("__UNBOUND_CLIENT__", shell_unbound_client())
        .replace("__FIREWALL_CLIENT__", shell_firewall_client())
        .replace("__UNBOUND_PANE__", if session == Session::Admin { DNS_PANE } else { "" });
    let shell = if session == Session::Guest {
        ["firewall"]
            .into_iter()
            .fold(shell, remove_admin_only_pane_from_guest_shell)
    } else {
        shell
    };
    remove_inline_chrome_script(shell)
}
