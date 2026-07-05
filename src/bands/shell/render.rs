fn render_crown_shell() -> String {
    let nav = render_flask_react_tabbar_quarry();
    let shell = [shell_document_1(), shell_document_2(), shell_document_3(), shell_document_4()].concat();
    shell
        .replace("__NAV__", &nav)
        .replace("__TESTTAB__", &render_testtab())
        .replace("__ADMIN_SSH_PASSWORD_CARD__", &render_admin_service_card_html("ssh-password-authentication"))
        .replace("__ADMIN_SSH_SERVICE_CARD__", &render_admin_service_card_html("ssh-service"))
        .replace("__ADMIN_SAMBA_SERVICE_CARD__", &render_admin_service_card_html("samba-file-sharing"))
        .replace("__ADMIN_AVAILABLE_DEVICES__", &render_admin_available_devices_html())
        .replace("__ADMIN_MOUNT_DESTINATIONS__", &render_admin_mount_destinations_html())
}
