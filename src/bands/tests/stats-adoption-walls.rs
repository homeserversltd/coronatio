#[test]
fn uxport_008_stats_adoption_walls_keep_og_grammar_and_truthful_absence() {
    let shell = render_crown_shell_for_session(Session::Admin);
    let chrome = crown_chrome_js();
    let css = include_str!("../shell/ux/packs/stats.css");

    for marker in [
        "network-interfaces ui-table",
        "kea-leases-table ui-table ui-table--responsive",
        "ui-progress-bar__container",
        "ui-progress-bar__fill--memory",
        "ui-progress-bar__fill--swap",
        "loading-spinner medium\" role=\"progressbar\" aria-label=\"Loading Kea leases\"",
    ] {
        assert!(shell.contains(marker), "Stats library composition omitted {marker}");
    }
    for marker in [
        "themeCssColor('--secondary', '#4A5568')",
        "themeCssColor('--accent', '#90cff3')",
        "borderDash: [3, 3]",
        "pointRadius: 0",
        "pointHoverRadius: 0",
        "animation: { duration: 250 }",
        "labels.length > 60",
        "checked.get(`${dataset.label.endsWith(' Read') ? 'read' : 'write'}-${name}`) !== false",
        "No Kea leases found.",
        "OG has no Stats-family error face; retain the last truthful frame.",
        "event.target.closest('[data-device-controls] input[type=\"checkbox\"]')",
    ] {
        assert!(chrome.contains(marker), "Stats chart/absence wall omitted {marker}");
    }
    for forbidden in ["#FF6384", "#36A2EB", "borderDash: [5, 5]"] {
        assert!(!chrome.contains(forbidden), "Stats retained counterfeit grammar {forbidden}");
    }
    for marker in ["height: 200px !important", "min-height: 200px", "height: 250px !important"] {
        assert!(css.contains(marker), "Stats chart size wall omitted {marker}");
    }
}
