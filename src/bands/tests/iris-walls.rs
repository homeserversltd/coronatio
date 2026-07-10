fn iris_tab(id: &str, order: i64, admin_only: bool, is_enabled: bool, visibility_tab: Option<bool>) -> IrisTabFact {
    IrisTabFact {
        id: id.to_string(),
        order,
        admin_only,
        is_enabled,
        visibility_tab,
        elements: Vec::new(),
    }
}

fn iris_element_tab(tab_visible: bool, element: Option<bool>) -> IrisFacts {
    IrisFacts {
        tabs: vec![IrisTabFact {
            id: "stats".to_string(),
            order: 0,
            admin_only: false,
            is_enabled: true,
            visibility_tab: Some(tab_visible),
            elements: vec![IrisElementFact {
                id: "cpu-chart".to_string(),
                visibility: element,
            }],
        }],
        starred: "stats".to_string(),
    }
}

fn iris_facts_for_tab(admin_only: bool, is_enabled: bool, visibility_tab: Option<bool>) -> IrisFacts {
    IrisFacts {
        tabs: vec![iris_tab("alpha", 0, admin_only, is_enabled, visibility_tab)],
        starred: "alpha".to_string(),
    }
}

fn iris_tab_grant(plan: &RenderPlan, tab: &str) -> Option<TabGrant> {
    plan.tabs.iter().find(|grant| grant.tab_id == tab).cloned()
}

fn iris_element_grant(plan: &RenderPlan, key: &str) -> Option<ElementGrant> {
    plan.elements.iter().find(|grant| grant.key == key).cloned()
}

fn assert_iris_tab_row(
    name: &str,
    admin_only: bool,
    is_enabled: bool,
    visibility_tab: bool,
    session: Session,
    state: RenderState,
    eye: bool,
    star: bool,
    eligible: bool,
) {
    let facts = iris_facts_for_tab(admin_only, is_enabled, Some(visibility_tab));
    let plan = iris::plan(&facts, session);
    let grant = iris_tab_grant(&plan, "alpha");
    if state == RenderState::Absent {
        assert!(grant.is_none(), "{name} expected absent grant");
    } else {
        let grant = grant.unwrap_or_else(|| panic!("{name} expected grant"));
        assert_eq!(grant.state, state, "{name} state");
        assert_eq!(grant.eye, eye, "{name} eye");
        assert_eq!(grant.star, star, "{name} star");
        assert_eq!(grant.star_eligible, eligible, "{name} eligible");
    }
}

#[test]
fn iris_c_tab_row_01_regular_enabled_visible_guest() {
    assert_iris_tab_row("C tab row 01", false, true, true, Session::Guest, RenderState::Visible, false, true, true);
}

#[test]
fn iris_c_tab_row_02_regular_enabled_visible_admin() {
    assert_iris_tab_row("C tab row 02", false, true, true, Session::Admin, RenderState::Visible, true, true, true);
}

#[test]
fn iris_c_tab_row_03_regular_enabled_hidden_guest() {
    assert_iris_tab_row("C tab row 03", false, true, false, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_04_regular_enabled_hidden_admin() {
    assert_iris_tab_row("C tab row 04", false, true, false, Session::Admin, RenderState::DimmedHidden, true, false, false);
}

#[test]
fn iris_c_tab_row_05_regular_disabled_visible_guest() {
    assert_iris_tab_row("C tab row 05", false, false, true, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_06_regular_disabled_visible_admin() {
    assert_iris_tab_row("C tab row 06", false, false, true, Session::Admin, RenderState::DimmedHidden, true, false, false);
}

#[test]
fn iris_c_tab_row_07_regular_disabled_hidden_guest() {
    assert_iris_tab_row("C tab row 07", false, false, false, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_08_regular_disabled_hidden_admin() {
    assert_iris_tab_row("C tab row 08", false, false, false, Session::Admin, RenderState::DimmedHidden, true, false, false);
}

#[test]
fn iris_c_tab_row_09_admin_only_enabled_visible_guest() {
    assert_iris_tab_row("C tab row 09", true, true, true, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_10_admin_only_enabled_visible_admin() {
    assert_iris_tab_row("C tab row 10", true, true, true, Session::Admin, RenderState::Visible, false, false, false);
}

#[test]
fn iris_c_tab_row_11_admin_only_enabled_hidden_guest() {
    assert_iris_tab_row("C tab row 11", true, true, false, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_12_admin_only_enabled_hidden_admin() {
    assert_iris_tab_row("C tab row 12", true, true, false, Session::Admin, RenderState::DimmedHidden, false, false, false);
}

#[test]
fn iris_c_tab_row_13_admin_only_disabled_visible_guest() {
    assert_iris_tab_row("C tab row 13", true, false, true, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_14_admin_only_disabled_visible_admin() {
    assert_iris_tab_row("C tab row 14", true, false, true, Session::Admin, RenderState::Visible, false, false, false);
}

#[test]
fn iris_c_tab_row_15_admin_only_disabled_hidden_guest() {
    assert_iris_tab_row("C tab row 15", true, false, false, Session::Guest, RenderState::Absent, false, false, false);
}

#[test]
fn iris_c_tab_row_16_admin_only_disabled_hidden_admin() {
    assert_iris_tab_row("C tab row 16", true, false, false, Session::Admin, RenderState::DimmedHidden, false, false, false);
}

fn assert_iris_element_row(
    name: &str,
    tab_visible: bool,
    element_visible: Option<bool>,
    session: Session,
    state: RenderState,
    eye: bool,
) {
    let facts = iris_element_tab(tab_visible, element_visible);
    let plan = iris::plan(&facts, session);
    let grant = iris_element_grant(&plan, "stats/element:cpu-chart");
    if state == RenderState::Absent {
        assert!(grant.is_none(), "{name} expected absent element");
    } else {
        let grant = grant.unwrap_or_else(|| panic!("{name} expected element grant"));
        assert_eq!(grant.state, state, "{name} state");
        assert_eq!(grant.eye, eye, "{name} eye");
    }
}

#[test]
fn iris_c_portals_element_row_01_tab_visible_element_true_guest() {
    assert_iris_element_row("C portals row 01", true, Some(true), Session::Guest, RenderState::Visible, false);
}

#[test]
fn iris_c_portals_element_row_02_tab_visible_element_true_admin() {
    assert_iris_element_row("C portals row 02", true, Some(true), Session::Admin, RenderState::Visible, true);
}

#[test]
fn iris_c_portals_element_row_03_tab_visible_element_false_guest() {
    assert_iris_element_row("C portals row 03", true, Some(false), Session::Guest, RenderState::Absent, false);
}

#[test]
fn iris_c_portals_element_row_04_tab_visible_element_false_admin() {
    assert_iris_element_row("C portals row 04", true, Some(false), Session::Admin, RenderState::DimmedHidden, true);
}

#[test]
fn iris_c_portals_element_row_05_tab_visible_element_missing_guest() {
    assert_iris_element_row("C portals row 05", true, None, Session::Guest, RenderState::Visible, false);
}

#[test]
fn iris_c_portals_element_row_06_tab_visible_element_missing_admin() {
    assert_iris_element_row("C portals row 06", true, None, Session::Admin, RenderState::Visible, true);
}

#[test]
fn iris_c_portals_element_row_07_tab_hidden_element_any_guest() {
    assert_iris_element_row("C portals row 07", false, Some(true), Session::Guest, RenderState::Absent, false);
}

#[test]
fn iris_c_portals_element_row_08_tab_hidden_element_any_admin() {
    assert_iris_element_row("C portals row 08", false, Some(true), Session::Admin, RenderState::DimmedHidden, true);
}

#[test]
fn iris_c_stats_element_row_01_tab_visible_true_or_missing_guest() {
    assert_iris_element_row("C stats row 01 true", true, Some(true), Session::Guest, RenderState::Visible, false);
    assert_iris_element_row("C stats row 01 missing", true, None, Session::Guest, RenderState::Visible, false);
}

#[test]
fn iris_c_stats_element_row_02_tab_visible_true_or_missing_admin() {
    assert_iris_element_row("C stats row 02 true", true, Some(true), Session::Admin, RenderState::Visible, true);
    assert_iris_element_row("C stats row 02 missing", true, None, Session::Admin, RenderState::Visible, true);
}

#[test]
fn iris_c_stats_element_row_03_tab_visible_false_guest() {
    assert_iris_element_row("C stats row 03", true, Some(false), Session::Guest, RenderState::Absent, false);
}

#[test]
fn iris_c_stats_element_row_04_tab_visible_false_admin() {
    assert_iris_element_row("C stats row 04", true, Some(false), Session::Admin, RenderState::DimmedHidden, true);
}

#[test]
fn iris_c_stats_element_row_05_tab_hidden_any_guest() {
    assert_iris_element_row("C stats row 05", false, Some(true), Session::Guest, RenderState::Absent, false);
}

#[test]
fn iris_c_stats_element_row_06_tab_hidden_any_admin() {
    assert_iris_element_row("C stats row 06", false, Some(true), Session::Admin, RenderState::DimmedHidden, true);
}

#[test]
fn iris_c_fallback_row_01_normal_success_injects_fallback_when_none_visible() {
    let plan = iris::plan(&iris_facts_for_tab(false, true, Some(false)), Session::Guest);
    assert!(iris_tab_grant(&plan, "fallback").is_some());
}

#[test]
fn iris_c_fallback_row_02_config_missing_returns_only_fallback() {
    let plan = iris::plan(&IrisFacts { tabs: Vec::new(), starred: "fallback".to_string() }, Session::Guest);
    assert_eq!(plan.tabs.iter().map(|grant| grant.tab_id.as_str()).collect::<Vec<_>>(), ["fallback"]);
}

#[test]
fn iris_c_fallback_row_03_initial_state_starts_at_fallback_when_no_regular_tabs() {
    let plan = iris::plan(&IrisFacts { tabs: Vec::new(), starred: "fallback".to_string() }, Session::Guest);
    assert_eq!(iris::initial_tab(&plan), "fallback");
}

#[test]
fn iris_c_fallback_row_04_config_initialization_merges_fallback_regardless_input() {
    let plan = iris::plan(&iris_facts_for_tab(false, true, Some(false)), Session::Admin);
    assert!(plan.fallback.injected);
    assert!(iris_tab_grant(&plan, "fallback").is_some());
}

#[test]
fn iris_c_fallback_row_05_no_visible_tabs_at_initialization_activates_fallback() {
    let plan = iris::plan(&iris_facts_for_tab(true, true, Some(true)), Session::Guest);
    assert!(plan.fallback.active);
    assert_eq!(iris::initial_tab(&plan), "fallback");
}

#[test]
fn iris_c_fallback_row_06_visibility_initialization_zero_visible_forces_fallback_visible() {
    let plan = iris::plan(&iris_facts_for_tab(false, false, Some(false)), Session::Guest);
    assert_eq!(iris_tab_grant(&plan, "fallback").unwrap().state, RenderState::Visible);
}

#[test]
fn iris_c_fallback_row_07_star_fallback_allowed_only_when_no_visible_regular_tabs() {
    let hidden = iris_facts_for_tab(false, true, Some(false));
    assert!(iris::apply_star(&hidden, "fallback").is_ok());
    let visible = iris_facts_for_tab(false, true, Some(true));
    assert_eq!(iris::apply_star(&visible, "fallback"), Err(StarRefusal::FallbackWhileRegularEligible));
}

#[test]
fn iris_d_row_01_hide_currently_starred_regular_tab_rederives_first_visible() {
    let facts = IrisFacts {
        tabs: vec![iris_tab("alpha", 0, false, true, Some(true)), iris_tab("beta", 1, false, true, Some(true))],
        starred: "alpha".to_string(),
    };
    let next = iris::apply_tab_visibility(&facts, "alpha", false);
    assert_eq!(next.starred, "beta");
}

#[test]
fn iris_d_row_02_exit_admin_on_admin_only_tab_lands_on_starred_guest_visible() {
    let facts = IrisFacts {
        tabs: vec![iris_tab("admin", 0, true, true, Some(true)), iris_tab("stats", 1, false, true, Some(true))],
        starred: "stats".to_string(),
    };
    let admin_plan = iris::plan(&facts, Session::Admin);
    let guest_plan = iris::plan(&facts, Session::Guest);
    assert_eq!(iris::landing_after_session_change(&admin_plan, &guest_plan, "admin"), "stats");
}

#[test]
fn iris_d_row_03_exit_admin_on_hidden_regular_tab_lands_on_starred_guest_visible() {
    let facts = IrisFacts {
        tabs: vec![iris_tab("hidden", 0, false, true, Some(false)), iris_tab("stats", 1, false, true, Some(true))],
        starred: "stats".to_string(),
    };
    let admin_plan = iris::plan(&facts, Session::Admin);
    let guest_plan = iris::plan(&facts, Session::Guest);
    assert_eq!(iris::landing_after_session_change(&admin_plan, &guest_plan, "hidden"), "stats");
}

#[test]
fn iris_d_row_04_star_then_hide_never_leaves_ineligible_star() {
    let facts = IrisFacts {
        tabs: vec![iris_tab("alpha", 0, false, true, Some(true)), iris_tab("beta", 1, false, true, Some(true))],
        starred: "beta".to_string(),
    };
    let starred = iris::apply_star(&facts, "alpha").unwrap();
    assert_eq!(starred.starred, "alpha");
    let hidden = iris::apply_tab_visibility(&starred, "alpha", false);
    assert_eq!(hidden.starred, "beta");
}

#[test]
fn iris_d_row_05_hide_all_regular_tabs_derives_fallback() {
    let facts = iris_facts_for_tab(false, true, Some(true));
    let hidden = iris::apply_tab_visibility(&facts, "alpha", false);
    let plan = iris::plan(&hidden, Session::Guest);
    assert_eq!(hidden.starred, "fallback");
    assert!(plan.fallback.active);
}

#[test]
fn iris_d_row_06_enter_admin_plan_delta_adds_dimmed_hidden_and_admin_only_grants() {
    let facts = IrisFacts {
        tabs: vec![iris_tab("hidden", 0, false, true, Some(false)), iris_tab("admin", 1, true, true, Some(true)), iris_tab("stats", 2, false, true, Some(true))],
        starred: "stats".to_string(),
    };
    let guest_plan = iris::plan(&facts, Session::Guest);
    let admin_plan = iris::plan(&facts, Session::Admin);
    assert!(iris_tab_grant(&guest_plan, "hidden").is_none());
    assert!(iris_tab_grant(&guest_plan, "admin").is_none());
    assert_eq!(iris_tab_grant(&admin_plan, "hidden").unwrap().state, RenderState::DimmedHidden);
    assert_eq!(iris_tab_grant(&admin_plan, "admin").unwrap().state, RenderState::Visible);
}

#[test]
fn iris_d_row_07_element_toggle_returns_new_projection_state() {
    let facts = iris_element_tab(true, Some(true));
    let hidden = iris::apply_element_visibility(&facts, "stats", "cpu-chart", false);
    let admin_plan = iris::plan(&hidden, Session::Admin);
    assert_eq!(iris_element_grant(&admin_plan, "stats/element:cpu-chart").unwrap().state, RenderState::DimmedHidden);
}

#[test]
fn iris_d_row_08_session_change_current_if_granted_wins_before_starred() {
    let facts = IrisFacts {
        tabs: vec![iris_tab("alpha", 0, false, true, Some(true)), iris_tab("beta", 1, false, true, Some(true))],
        starred: "beta".to_string(),
    };
    let old_plan = iris::plan(&facts, Session::Admin);
    let new_plan = iris::plan(&facts, Session::Guest);
    assert_eq!(iris::landing_after_session_change(&old_plan, &new_plan, "alpha"), "alpha");
}

#[test]
fn iris_registry_delegation_preserves_tab_accessible_signature() {
    let mut tab = native_tab_contracts()
        .into_iter()
        .find(|tab| tab.id == "dhcp")
        .expect("dhcp tab exists");
    tab.visibility.tab = false;
    assert!(!tab_accessible_in_mode(&tab, false));
    assert!(tab_accessible_in_mode(&tab, true));
}

#[test]
fn iris_totality_invariant_sweep_full_synthetic_domain() {
    let values = [false, true];
    let visibilities = [None, Some(false), Some(true)];
    for first_admin_only in values {
        for first_enabled in values {
            for first_visibility in visibilities {
                for second_admin_only in values {
                    for second_enabled in values {
                        for second_visibility in visibilities {
                            for starred in ["alpha", "beta", "missing", "fallback"] {
                                let facts = IrisFacts {
                                    tabs: vec![
                                        iris_tab("alpha", 0, first_admin_only, first_enabled, first_visibility),
                                        iris_tab("beta", 1, second_admin_only, second_enabled, second_visibility),
                                    ],
                                    starred: starred.to_string(),
                                };
                                for session in [Session::Guest, Session::Admin] {
                                    let plan = iris::plan(&facts, session);
                                    if session == Session::Guest {
                                        assert!(plan.tabs.iter().all(|grant| grant.state != RenderState::DimmedHidden));
                                        assert!(plan.tabs.iter().all(|grant| grant.tab_id == "fallback" || facts.tabs.iter().find(|tab| tab.id == grant.tab_id).is_some_and(|tab| !tab.admin_only)));
                                    }
                                    let guest_visible_regular = facts.tabs.iter().filter(|tab| !tab.admin_only && tab.is_enabled && tab.visibility_tab.unwrap_or(false)).count();
                                    assert_eq!(plan.tabs.iter().any(|grant| grant.tab_id == "fallback"), guest_visible_regular == 0);
                                }
                                for tab in ["alpha", "beta"] {
                                    let after = iris::apply_tab_visibility(&facts, tab, false);
                                    assert!(after.starred == "fallback" || after.tabs.iter().any(|candidate| candidate.id == after.starred && !candidate.admin_only && candidate.is_enabled && candidate.visibility_tab.unwrap_or(false)));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
