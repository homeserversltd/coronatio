#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Session {
    Guest,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderState {
    Absent,
    Visible,
    DimmedHidden,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IrisFacts {
    tabs: Vec<IrisTabFact>,
    starred: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IrisTabFact {
    id: String,
    order: i64,
    admin_only: bool,
    is_enabled: bool,
    visibility_tab: Option<bool>,
    elements: Vec<IrisElementFact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IrisElementFact {
    id: String,
    visibility: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderPlan {
    session: Session,
    tabs: Vec<TabGrant>,
    elements: Vec<ElementGrant>,
    starred: StarTarget,
    fallback: FallbackGrant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TabGrant {
    key: String,
    tab_id: String,
    state: RenderState,
    eye: bool,
    star: bool,
    star_eligible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ElementGrant {
    key: String,
    tab_id: String,
    element_id: String,
    state: RenderState,
    eye: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StarTarget {
    Tab(String),
    Fallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FallbackGrant {
    injected: bool,
    active: bool,
    reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StarRefusal {
    AdminOnly,
    Hidden,
    Disabled,
    Missing,
    FallbackWhileRegularEligible,
}

mod iris {
    use super::*;

    pub(super) fn plan(facts: &IrisFacts, session: Session) -> RenderPlan {
        let mut tabs = sorted_tabs(facts);
        let guest_visible_regular_count = tabs.iter().filter(|tab| guest_visible(tab)).count();
        let fallback_needed = guest_visible_regular_count == 0;
        let mut tab_grants = tabs
            .iter_mut()
            .filter_map(|tab| grant_tab(tab, session))
            .collect::<Vec<_>>();
        if fallback_needed {
            tab_grants.push(TabGrant {
                key: "tab:fallback".to_string(),
                tab_id: "fallback".to_string(),
                state: RenderState::Visible,
                eye: false,
                star: true,
                star_eligible: true,
            });
        }
        let elements = tabs
            .iter()
            .flat_map(|tab| grant_elements(tab, session))
            .collect::<Vec<_>>();
        let starred = eligible_star(facts).map_or(StarTarget::Fallback, StarTarget::Tab);
        RenderPlan {
            session,
            tabs: tab_grants,
            elements,
            starred: if fallback_needed { StarTarget::Fallback } else { starred },
            fallback: FallbackGrant {
                injected: fallback_needed,
                active: fallback_needed,
                reason: fallback_needed.then(|| "no_guest_visible_regular_tabs".to_string()),
            },
        }
    }

    pub(super) fn apply_tab_visibility(facts: &IrisFacts, tab: &str, value: bool) -> IrisFacts {
        let mut next = facts.clone();
        if let Some(target) = next.tabs.iter_mut().find(|candidate| candidate.id == tab) {
            target.visibility_tab = Some(value);
        }
        restore_star_invariant(next)
    }

    pub(super) fn apply_element_visibility(
        facts: &IrisFacts,
        tab: &str,
        element: &str,
        value: bool,
    ) -> IrisFacts {
        let mut next = facts.clone();
        if let Some(target_tab) = next.tabs.iter_mut().find(|candidate| candidate.id == tab) {
            if let Some(target_element) = target_tab
                .elements
                .iter_mut()
                .find(|candidate| candidate.id == element)
            {
                target_element.visibility = Some(value);
            } else {
                target_tab.elements.push(IrisElementFact {
                    id: element.to_string(),
                    visibility: Some(value),
                });
            }
        }
        restore_star_invariant(next)
    }

    pub(super) fn apply_star(facts: &IrisFacts, tab: &str) -> Result<IrisFacts, StarRefusal> {
        if tab == "fallback" {
            if first_eligible_star(facts).is_none() {
                let mut next = facts.clone();
                next.starred = "fallback".to_string();
                return Ok(next);
            }
            return Err(StarRefusal::FallbackWhileRegularEligible);
        }
        let candidate = facts
            .tabs
            .iter()
            .find(|candidate| candidate.id == tab)
            .ok_or(StarRefusal::Missing)?;
        if candidate.admin_only {
            return Err(StarRefusal::AdminOnly);
        }
        if !candidate.is_enabled {
            return Err(StarRefusal::Disabled);
        }
        if !tab_visibility(candidate) {
            return Err(StarRefusal::Hidden);
        }
        let mut next = facts.clone();
        next.starred = tab.to_string();
        Ok(restore_star_invariant(next))
    }

    pub(super) fn initial_tab(plan: &RenderPlan) -> String {
        match &plan.starred {
            StarTarget::Tab(tab) if plan_has_visible_tab(plan, tab) => tab.clone(),
            _ if plan.fallback.active => "fallback".to_string(),
            _ => first_visible_regular(plan).unwrap_or_else(|| "fallback".to_string()),
        }
    }

    pub(super) fn landing_after_session_change(
        old_plan: &RenderPlan,
        new_plan: &RenderPlan,
        current_tab: &str,
    ) -> String {
        let _ = old_plan;
        if plan_has_visible_tab(new_plan, current_tab) {
            return current_tab.to_string();
        }
        initial_tab(new_plan)
    }

    pub(super) fn from_coronatio_contracts(
        tabs: &[CoronatioTabContract],
        starred: &str,
    ) -> IrisFacts {
        IrisFacts {
            tabs: tabs
                .iter()
                .map(|tab| IrisTabFact {
                    id: tab.id.clone(),
                    order: tab.order,
                    admin_only: tab.admin_only,
                    is_enabled: tab.enabled,
                    visibility_tab: Some(tab.visibility.tab),
                    elements: tab
                        .visibility
                        .elements
                        .iter()
                        .map(|(id, visibility)| IrisElementFact {
                            id: id.clone(),
                            visibility: Some(*visibility),
                        })
                        .collect(),
                })
                .collect(),
            starred: starred.to_string(),
        }
    }

    fn sorted_tabs(facts: &IrisFacts) -> Vec<IrisTabFact> {
        let mut tabs = facts
            .tabs
            .iter()
            .filter(|tab| tab.id != "fallback")
            .cloned()
            .collect::<Vec<_>>();
        tabs.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
        tabs
    }

    fn grant_tab(tab: &IrisTabFact, session: Session) -> Option<TabGrant> {
        let state = tab_state(tab, session);
        if state == RenderState::Absent {
            return None;
        }
        Some(TabGrant {
            key: format!("tab:{}", tab.id),
            tab_id: tab.id.clone(),
            state,
            eye: session == Session::Admin && !tab.admin_only,
            star: state == RenderState::Visible && !tab.admin_only,
            star_eligible: star_eligible(tab),
        })
    }

    fn grant_elements(tab: &IrisTabFact, session: Session) -> Vec<ElementGrant> {
        tab.elements
            .iter()
            .filter_map(|element| {
                let state = element_state(tab, element, session);
                (state != RenderState::Absent).then(|| ElementGrant {
                    key: format!("{}/element:{}", tab.id, element.id),
                    tab_id: tab.id.clone(),
                    element_id: element.id.clone(),
                    state,
                    eye: session == Session::Admin,
                })
            })
            .collect()
    }

    fn tab_state(tab: &IrisTabFact, session: Session) -> RenderState {
        let visible = tab_visibility(tab);
        match session {
            Session::Guest if guest_visible(tab) => RenderState::Visible,
            Session::Guest => RenderState::Absent,
            Session::Admin if tab.admin_only && visible => RenderState::Visible,
            Session::Admin if tab.admin_only => RenderState::DimmedHidden,
            Session::Admin if tab.is_enabled && visible => RenderState::Visible,
            Session::Admin => RenderState::DimmedHidden,
        }
    }

    fn element_state(tab: &IrisTabFact, element: &IrisElementFact, session: Session) -> RenderState {
        let tab_visible = tab_visibility(tab);
        let element_visible = element_visibility(element);
        match session {
            Session::Guest if tab_visible && element_visible => RenderState::Visible,
            Session::Guest => RenderState::Absent,
            Session::Admin if tab_visible && element_visible => RenderState::Visible,
            Session::Admin => RenderState::DimmedHidden,
        }
    }

    fn guest_visible(tab: &IrisTabFact) -> bool {
        !tab.admin_only && tab.is_enabled && tab_visibility(tab)
    }

    fn star_eligible(tab: &IrisTabFact) -> bool {
        !tab.admin_only && tab.is_enabled && tab_visibility(tab)
    }

    /// E3 pending schema ruling: table section E3 fixes tab visibility absence/malformed as hidden while element entry absence remains visible at the plan layer.
    fn tab_visibility(tab: &IrisTabFact) -> bool {
        tab.visibility_tab.unwrap_or(false)
    }

    fn element_visibility(element: &IrisElementFact) -> bool {
        element.visibility.unwrap_or(true)
    }

    fn eligible_star(facts: &IrisFacts) -> Option<String> {
        facts
            .tabs
            .iter()
            .find(|tab| tab.id == facts.starred && star_eligible(tab))
            .map(|tab| tab.id.clone())
            .or_else(|| first_eligible_star(facts))
    }

    fn first_eligible_star(facts: &IrisFacts) -> Option<String> {
        sorted_tabs(facts)
            .into_iter()
            .find(star_eligible)
            .map(|tab| tab.id)
    }

    fn restore_star_invariant(mut facts: IrisFacts) -> IrisFacts {
        facts.starred = eligible_star(&facts).unwrap_or_else(|| "fallback".to_string());
        facts
    }

    fn plan_has_visible_tab(plan: &RenderPlan, tab: &str) -> bool {
        plan.tabs
            .iter()
            .any(|grant| grant.tab_id == tab && grant.state == RenderState::Visible)
    }

    fn first_visible_regular(plan: &RenderPlan) -> Option<String> {
        plan.tabs
            .iter()
            .find(|grant| grant.tab_id != "fallback" && grant.state == RenderState::Visible)
            .map(|grant| grant.tab_id.clone())
    }
}
