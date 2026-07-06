const CROWN_HTMX_SCRIPT_PATH: &str = "/static/vendor/htmx.min.js";
const CROWN_CHROME_SCRIPT_PATH: &str = "/static/crown/chrome.js";
const CROWN_CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; base-uri 'self'; frame-ancestors 'self'";
const CROWN_HTMX_JS: &str = include_str!("../../../static/vendor/htmx.min.js");
const SHELL_UX_INDEX_JSON: &str = include_str!("ux/index.json");
const SHELL_UX_README: &str = include_str!("ux/README.md");
const SHELL_UX_CHILDREN: &[&str] = &[
    "shell/base-and-chrome.css",
    "library/_badge.css",
    "library/_breadcrumbs.css",
    "library/_button.css",
    "library/_calendar.css",
    "library/_card.css",
    "library/_checkbox.css",
    "library/_collapsible.css",
    "library/_editable-field.css",
    "library/_file-input.css",
    "library/_icon-button.css",
    "library/_input.css",
    "library/_plus-button.css",
    "library/_progress-bar.css",
    "library/_row-info-tile.css",
    "library/_select.css",
    "library/_slider.css",
    "library/_table.css",
    "library/_tabs.css",
    "library/_text-box.css",
    "library/_time-picker.css",
    "library/_toggle.css",
    "library/_visibility-toggle.css",
    "packs/upload.css",
    "packs/stats.css",
    "packs/portals.css",
    "shell/document-2-css.css",
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
    include_str!("ux/library/_plus-button.css"),
    include_str!("ux/library/_progress-bar.css"),
    include_str!("ux/library/_row-info-tile.css"),
    include_str!("ux/library/_select.css"),
    include_str!("ux/library/_slider.css"),
    include_str!("ux/library/_table.css"),
    include_str!("ux/library/_tabs.css"),
    include_str!("ux/library/_text-box.css"),
    include_str!("ux/library/_time-picker.css"),
    include_str!("ux/library/_toggle.css"),
    include_str!("ux/library/_visibility-toggle.css"),
    include_str!("ux/packs/upload.css"),
    include_str!("ux/packs/stats.css"),
    include_str!("ux/packs/portals.css"),
    include_str!("ux/shell/document-2-css.css"),
];

fn shell_ux_css() -> String {
    SHELL_UX_CONTENTS.concat()
}

fn crown_chrome_js() -> String {
    let raw = [shell_document_2(), shell_document_3(), shell_document_4()].concat();
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
    let nav = render_flask_react_tabbar_quarry();
    let shell = [
        shell_document_1().to_string(),
        shell_ux_css(),
        shell_document_2().to_string(),
        shell_document_3().to_string(),
        shell_document_4().to_string(),
    ]
    .concat();
    let shell = shell
        .replace("__NAV__", &nav)
        .replace("__TEST__", &render_test_showcase())
        .replace("__ADMIN_SSH_PASSWORD_CARD__", &render_admin_service_card_html("ssh-password-authentication"))
        .replace("__ADMIN_SSH_SERVICE_CARD__", &render_admin_service_card_html("ssh-service"))
        .replace("__ADMIN_SAMBA_SERVICE_CARD__", &render_admin_service_card_html("samba-file-sharing"))
        .replace("__ADMIN_AVAILABLE_DEVICES__", &render_admin_available_devices_html())
        .replace("__ADMIN_MOUNT_DESTINATIONS__", &render_admin_mount_destinations_html())
        .replace("__UPLOAD_TREE_FRAGMENT__", &render_upload_tree_fragment(None, None));
    remove_inline_chrome_script(shell)
}
