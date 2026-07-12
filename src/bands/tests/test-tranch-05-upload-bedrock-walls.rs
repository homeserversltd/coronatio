#[test]
fn test_tranch_05_upload_components_prove_library_and_domain_pack_vocabulary() {
    let html = render_crown_shell();
    let start = html.find(r#"id="showcase-upload-components""#).expect("Upload Components catalog section");
    let end = html[start..].find(r#"id="showcase-progress-bar""#).expect("Progress Bar section") + start;
    let upload_test = &html[start..end];

    for library_marker in [
        "ui-breadcrumbs",
        "ui-breadcrumbs__item--current",
        "ui-icon-button--small ui-icon-button--square",
        "ui-icon-button--medium ui-icon-button--circle",
        "ui-icon-button--large ui-icon-button--square",
        "ui-file-input__input",
        "ui-file-input__display",
        "data-test-file-submit disabled",
        "aria-invalid=\"true\"",
    ] {
        assert!(upload_test.contains(library_marker), "TEST-TRANCH-05 library specimen missing {library_marker}");
    }

    for domain_marker in [
        "data-test-upload-domain-pack",
        "class=\"upload-tablet\"",
        "class=\"upload-progress-list\"",
        "class=\"directory-browser\"",
        "class=\"directory-browser-header\"",
        "class=\"breadcrumb-navigation\"",
        "class=\"breadcrumb-item current\"",
        "class=\"breadcrumb-separator\"",
        "class=\"directory-tree-container\"",
        "class=\"directory-entry selected\"",
        "class=\"file-upload-section\"",
        "data-test-domain-submit disabled",
    ] {
        assert!(upload_test.contains(domain_marker), "TEST-TRANCH-05 domain pack missing {domain_marker}");
    }
}

#[test]
fn test_tranch_05_progress_catalog_carries_low_mid_high_and_all_upload_states() {
    let html = render_crown_shell();
    let start = html.find(r#"id="showcase-progress-bar""#).expect("Progress Bar catalog section");
    let end = html[start..].find(r#"id="showcase-table""#).expect("Table section") + start;
    let progress = &html[start..end];

    for state in ["pending", "uploading", "completed", "error"] {
        assert!(progress.contains(&format!("upload-progress {state}")), "missing upload progress state {state}");
    }
    for percentage in ["2%", "50%", "64%", "100%", "18%"] {
        assert!(progress.contains(&format!("class=\"progress-text\">{percentage}")), "missing legible progress label {percentage}");
    }
    for marker in ["progress-bar-container", "progress-bar", "progress-text", "upload-stats", "error-message"] {
        assert!(progress.contains(marker), "progress specimen missing production marker {marker}");
    }
}

#[test]
fn test_tranch_05_interactions_update_breadcrumb_and_file_selection_readbacks() {
    let html = render_crown_shell();
    for marker in [
        "const testBreadcrumb = event.target.closest('[data-test-breadcrumb-path]')",
        "ui-breadcrumbs__item--current",
        "out.textContent = path",
        "const testDirectory = event.target.closest('[data-test-directory-path]')",
        "const names = Array.from(event.target.files || []).map(file => file.name)",
        "if (submit) submit.disabled = names.length === 0",
        "data-test-domain-file-name",
    ] {
        assert!(html.contains(marker), "TEST-TRANCH-05 interaction wall missing {marker}");
    }
}

#[test]
fn test_tranch_05_protects_live_upload_landmarks_and_shared_pack() {
    let html = render_crown_shell();
    let start = html.find(r#"id="pane-upload""#).expect("live Upload pane");
    let end = html[start..].find(r#"id="pane-backblaze""#).expect("next pane") + start;
    let upload = &html[start..end];
    for marker in [
        "class=\"upload-tablet\"",
        "class=\"upload-progress-list\"",
        "class=\"directory-browser\"",
        "class=\"breadcrumb-navigation\"",
        "class=\"directory-tree-container\"",
        "class=\"file-upload-section\"",
        "data-upload-file",
        "data-upload-submit disabled",
    ] {
        assert!(upload.contains(marker), "live Upload landmark regressed: {marker}");
    }
    assert!(html.contains("UXPORT-001 LIBRARY band: og src/tablets/upload upload domain pack"));
    assert!(html.matches(".upload-progress { background: var(--hiddenTabBackground)").count() >= 1);
}
