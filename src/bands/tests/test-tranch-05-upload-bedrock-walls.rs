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
    for percentage in ["2%", "64%", "100%", "18%"] {
        assert!(progress.contains(&format!("class=\"progress-text\">{percentage}")), "missing legible upload progress label {percentage}");
    }
    assert!(progress.contains("class=\"ui-progress-bar__text\">50%"), "missing library mid-progress label");
    for marker in ["progress-bar-container", "progress-bar", "progress-text", "upload-stats", "error-message"] {
        assert!(progress.contains(marker), "progress specimen missing production marker {marker}");
    }
    for library_marker in [
        "data-test-library-progress", "data-test-progress-sizes", "data-test-progress-variants",
        "data-test-progress-indeterminate", "ui-progress-bar--small", "ui-progress-bar--medium",
        "ui-progress-bar--large", "ui-progress-bar__fill--memory", "ui-progress-bar__fill--swap",
        "ui-progress-bar__fill--process", "ui-progress-bar__fill--disk", "role=\"meter\"",
    ] {
        assert!(progress.contains(library_marker), "progress library showroom missing {library_marker}");
    }
    let indeterminate = &progress[progress.find("data-test-progress-indeterminate").unwrap()..progress.find("Upload domain-pack states").unwrap()];
    assert!(indeterminate.contains("role=\"progressbar\""));
    assert!(!indeterminate.contains("aria-valuenow"), "indeterminate specimen must not claim a fraction");
}

#[test]
fn test_tranch_05_progress_paint_is_tokenized_legible_and_reduced_motion_safe() {
    let html = render_crown_shell();
    let pack = std::fs::read_to_string("src/bands/shell/ux/packs/upload.css").unwrap();
    for marker in [
        "pali:agentics-ux-progress-constitution-tablet", ".ui-progress-bar__container {",
        "background: var(--hiddenTabBackground);", ".ui-progress-bar__fill {",
        "background: var(--secondary);", ".ui-progress-bar--indeterminate .ui-progress-bar__fill",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(html.contains(marker), "progress visual-law marker missing {marker}");
    }
    for marker in [".upload-progress.pending .progress-bar,", ".upload-progress.uploading .progress-bar {", "background-image: linear-gradient(45deg, ", "animation: progress-stripes 1s linear infinite;"] {
        assert!(pack.contains(marker), "OG-carried progress paint marker missing {marker}");
    }
    let script = &html[html.find("function renderUploadProgress").unwrap()..html.find("function setUpload").unwrap()];
    assert!(!script.contains("uploadStatusColor"), "upload status paint must come from Theme Net state classes");
    assert!(!script.contains("background-color:"), "upload renderer must not inline status paint");
}

#[test]
fn test_tranch_05_upload_progress_label_geometry_does_not_falsify_fill_width() {
    let html = render_crown_shell();
    for percentage in ["2", "18", "50", "64", "100"] {
        assert!(
            html.contains(&format!(">{percentage}%</span>")),
            "progress showroom missing {percentage}% label"
        );
    }

    let pack = std::fs::read_to_string("src/bands/shell/ux/packs/upload.css").unwrap();
    for marker in [".progress-bar {", "background: var(--hiddenTabBackground);", "position: relative;", "min-width: 24px;", ".progress-text {"] {
        assert!(pack.contains(marker), "OG-carried progress geometry marker missing {marker}");
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
    let pack = std::fs::read_to_string("src/bands/shell/ux/packs/upload.css").unwrap();
    for marker in [
        ".upload-progress {",
        ".directory-browser {",
        ".file-upload-section input[type=\"file\"] {",
        ".directory-error.nas-unavailable {",
    ] {
        assert!(pack.contains(marker), "absorbed OG Upload pack landmark regressed: {marker}");
    }
    assert!(!pack.contains("UXPORT-001 LIBRARY band"));
}
