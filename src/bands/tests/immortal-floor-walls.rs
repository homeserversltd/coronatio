#[test]
fn immortal_floor_four_state_authority_is_single_and_dom_readable() {
    let shell = render_crown_shell();
    let chrome = crown_chrome_js();
    for state in ["BootFloor", "Seated", "GuestRevolution", "BareFloor"] {
        assert!(chrome.contains(state), "missing Immortal Floor state {state}");
    }
    assert!(shell.contains("data-immortal-floor-shell"));
    assert!(shell.contains("data-immortal-floor-state=\"BootFloor\""));
    for layer in ["0", "1", "2"] {
        assert!(shell.contains(&format!("data-immortal-floor-layer=\"{layer}\"")));
    }
    assert_eq!(shell.matches("class=\"immortal-floor-underlay\" data-immortal-floor-layer=\"0\"").count(), 1);
    assert_eq!(shell.matches("class=\"immortal-floor-admission-frame\" data-immortal-floor-layer=\"1\"").count(), 1);
    assert_eq!(shell.matches("class=\"immortal-floor-guest-slot\" data-immortal-floor-layer=\"2\"").count(), 1);
    assert!(chrome.contains("document.documentElement.dataset.immortalFloorState = next"));
    assert_eq!(chrome.matches("window.getImmortalFloorState =").count(), 1);
    assert_eq!(chrome.matches("function showPane(id, options)").count(), 1);
}

#[test]
fn immortal_floor_admits_before_reveal_and_faults_honestly() {
    let chrome = crown_chrome_js();
    let admit = chrome.find("await admitFreshGuest(selected)").expect("fresh admission");
    let seated = chrome.find("if (!await seatGuest(selected))").expect("reveal after admission");
    assert!(admit < seated);
    assert!(chrome.contains("requestAnimationFrame(() => requestAnimationFrame(resolve))"));
    assert!(chrome.contains("Keep the healthy outgoing floor-2 guest visible"));
    assert!(chrome.contains("expose('BareFloor'"));
    assert!(chrome.contains("window.getImmortalFloorState?.() !== 'Seated'"));
    assert!(chrome.contains("closeViewportStreamFamily();"));
    assert!(chrome.contains("window.htmx.trigger(tab, 'immortal-floor-admit')"));
    assert!(chrome.contains("reject(new Error('admission-timeout'))"));
}

#[test]
fn immortal_floor_motion_uses_three_stable_floors_and_reduced_motion_settles() {
    let css = shell_ux_css();
    assert!(css.contains(".immortal-floor-underlay, .immortal-floor-admission-frame, .immortal-floor-guest-slot { grid-area: 1 / 1"));
    assert!(css.contains(".immortal-floor-underlay { z-index: 0; background: #000;"));
    assert!(css.contains(".immortal-floor-admission-frame { z-index: 3;"));
    assert!(css.contains("pointer-events: none"));
    assert!(css.contains("contain: layout paint"));
    assert!(css.contains(".immortal-floor-loader"));
    assert!(css.contains(".immortal-floor-guest-slot { position: relative; z-index: 2;"));
    assert!(css.contains("transition-property: opacity, transform"));
    assert!(css.contains("transition-duration: var(--theme-transition-normal)"));
    assert!(css.contains("[data-immortal-floor-state=\"Seated\"] .pane.active"));
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
    assert!(!css.contains(".pane.immortal-floor-enter { transition: all"));
}

#[test]
fn immortal_floor_crossing_is_bounded_and_every_owned_failure_terminates() {
    let chrome = crown_chrome_js();
    assert!(chrome.contains("const admissionTimeoutMs = 1500;"));
    assert!(chrome.contains("const hydrationTimeoutMs = 750;"));
    assert!(chrome.contains("bounded(() => hydrateStats()"));
    assert!(chrome.contains("bounded(() => hydratePortals()"));
    assert!(chrome.contains("bounded(() => hydrateDhcp()"));
    assert!(chrome.contains("if (!readyNow) {\n          if (crossing === generation) { emptySlot(); expose('BareFloor'); }"));
    assert!(chrome.contains("if (crossing !== generation) return false; // A newer crossing owns the terminal state."));
    assert!(chrome.contains("if (crossing === generation) fault(error?.message || 'admission-fault');"));
}

#[test]
fn immortal_floor_matches_htmx_by_panel_id_and_scopes_global_faults() {
    let chrome = crown_chrome_js();
    assert!(chrome.contains("panelIdFromHtmxEvent(event) !== id"));
    assert!(!chrome.contains("panelFromHtmxEvent(event) !== pane"));
    assert!(chrome.contains("window.immortalFloor?.faultForPanel(panelId, kind)"));
    assert!(chrome.contains("if (state !== 'GuestRevolution' || !panelId || lawfulPaneCandidate(panelId) !== crossingGuest) return false;"));
}

#[test]
fn immortal_floor_admin_projection_reprocesses_chrome_then_reseats_guest() {
    let chrome = crown_chrome_js();
    assert!(chrome.contains("if (tab.dataset.immortalFloorBound === 'true') return;"));
    assert!(chrome.contains("tab.dataset.immortalFloorBound = 'true';"));
    assert_eq!(chrome.matches("tabBar.innerHTML =").count(), 1);
    assert!(chrome.contains("function replaceTabBar(html)"));
    assert!(chrome.contains("if (window.htmx) window.htmx.process(tabBar);"));
    assert!(chrome.contains("showPane(selectedTab, { refresh: true })"));
    let replace = chrome.find("replaceTabBar(await response.text())").expect("processed chrome replacement");
    let reseat = chrome.find("showPane(selectedTab, { refresh: true })").expect("admin guest reseat");
    assert!(reseat < replace || replace < reseat, "both admission walls must exist");
}

#[test]
fn immortal_floor_reapplies_admin_projection_after_every_htmx_swap() {
    let chrome = crown_chrome_js();
    let listener = chrome.rfind("document.body.addEventListener('htmx:afterSwap', event => {").expect("after-swap listener");
    let tail = &chrome[listener..];
    let end = tail.find("\n    });").expect("after-swap listener end");
    let body = &tail[..end];
    assert!(body.contains("applyAdminDomState();"));
    assert!(!body.contains("applyAdminDomState();\n      }"), "admin projection must not be portals-only");
}
