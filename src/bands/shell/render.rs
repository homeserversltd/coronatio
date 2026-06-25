fn render_crown_shell() -> String {
    let nav = render_flask_react_tabbar_quarry();
    let shell = [shell_document_1(), shell_document_2(), shell_document_3(), shell_document_4()].concat();
    shell.replace("__NAV__", &nav)
}
