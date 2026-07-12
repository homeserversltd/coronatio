#[test]
fn immortal_floor_four_state_authority_is_single_and_dom_readable() {
    let shell = render_crown_shell();
    let chrome = crown_chrome_js();
    for state in ["BootFloor", "Seated", "GuestRevolution", "BareFloor"] {
        assert!(chrome.contains(state), "missing Immortal Floor state {state}");
    }
    assert!(shell.contains("data-immortal-floor-shell"));
    assert!(shell.contains("data-immortal-floor-state=\"BootFloor\""));
    assert!(shell.contains("data-immortal-floor-layer=\"0\""));
    assert!(shell.contains("data-immortal-floor-layer=\"1\""));
    assert!(chrome.contains("document.documentElement.dataset.immortalFloorState = next"));
    assert_eq!(chrome.matches("window.getImmortalFloorState =").count(), 1);
    assert_eq!(chrome.matches("function showPane(id)").count(), 1);
}

#[test]
fn immortal_floor_crossings_empty_admit_reveal_and_fault_honestly() {
    let chrome = crown_chrome_js();
    let empty = chrome.find("emptySlot();").expect("empty-slot crossing");
    let paint = chrome.find("requestAnimationFrame(resolve)").expect("empty paint witness");
    let admit = chrome.find("await admitFreshGuest(selected)").expect("fresh admission");
    let seated = chrome.rfind("expose('Seated')").expect("seated reveal");
    assert!(empty < paint && paint < admit && admit < seated);
    assert!(chrome.contains("expose('BareFloor'"));
    assert!(chrome.contains("window.getImmortalFloorState?.() !== 'Seated'"));
    assert!(chrome.contains("closeViewportStreamFamily();"));
    assert!(chrome.contains("window.htmx.trigger(tab, 'immortal-floor-admit')"));
    assert!(chrome.contains("reject(new Error('admission-timeout'))"));
}

#[test]
fn immortal_floor_motion_uses_stable_slot_and_reduced_motion_settles() {
    let css = shell_ux_css();
    assert!(css.contains(".immortal-floor-underlay, .immortal-floor-guest-slot { grid-area: 1 / 1"));
    assert!(css.contains("transition-property: opacity, transform"));
    assert!(css.contains("transition-duration: var(--theme-transition-normal)"));
    assert!(css.contains("[data-immortal-floor-state=\"Seated\"] .pane.active"));
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
    assert!(!css.contains(".pane.immortal-floor-enter { transition: all"));
}