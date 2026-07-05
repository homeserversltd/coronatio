const CROWN_HTMX_SCRIPT_PATH: &str = "/static/vendor/htmx.min.js";
const CROWN_CHROME_SCRIPT_PATH: &str = "/static/crown/chrome.js";
const CROWN_CONTENT_SECURITY_POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; base-uri 'self'; frame-ancestors 'self'";
const CROWN_HTMX_JS: &str = include_str!("../../../static/vendor/htmx.min.js");

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
    let shell = [shell_document_1(), shell_document_2(), shell_document_3(), shell_document_4()].concat();
    let shell = shell
        .replace("__NAV__", &nav)
        .replace("__TESTTAB__", &render_testtab())
        .replace("__ADMIN_SSH_PASSWORD_CARD__", &render_admin_service_card_html("ssh-password-authentication"))
        .replace("__ADMIN_SSH_SERVICE_CARD__", &render_admin_service_card_html("ssh-service"))
        .replace("__ADMIN_SAMBA_SERVICE_CARD__", &render_admin_service_card_html("samba-file-sharing"))
        .replace("__ADMIN_AVAILABLE_DEVICES__", &render_admin_available_devices_html())
        .replace("__ADMIN_MOUNT_DESTINATIONS__", &render_admin_mount_destinations_html())
        .replace("__UPLOAD_TREE_FRAGMENT__", &render_upload_tree_fragment(None, None));
    remove_inline_chrome_script(shell)
}
