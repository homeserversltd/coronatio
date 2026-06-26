#[derive(Clone, Copy)]
struct UxComponentSpec {
    id: &'static str,
    category: &'static str,
    title: &'static str,
    primitive: &'static str,
    description: &'static str,
    sample: &'static str,
}

fn ux_component_registry() -> Vec<UxComponentSpec> {
    vec![
        UxComponentSpec { id: "button-primary", category: "buttons", title: "Primary Button", primitive: "ux-button", description: "Default action button; uses primary/primaryHover, never status-success.", sample: r#"<div class="ux-row"><button class="ux-button primary">Primary Button</button><button class="ux-button primary">With Icon</button><button class="ux-button primary" aria-busy="true">Loading</button><button class="ux-button primary ux-disabled" disabled>Disabled</button></div>"# },
        UxComponentSpec { id: "button-secondary", category: "buttons", title: "Secondary Button", primitive: "ux-button secondary", description: "Low-emphasis action button.", sample: r#"<div class="ux-row"><button class="ux-button secondary">Secondary Button</button><button class="ux-button secondary">Icon Right ›</button></div>"# },
        UxComponentSpec { id: "button-semantic", category: "buttons", title: "Semantic Buttons", primitive: "ux-button.{danger,warning,success}", description: "Semantic actions are explicit; green appears only on success.", sample: r#"<div class="ux-row"><button class="ux-button danger">Danger Button</button><button class="ux-button warning">Warning Button</button><button class="ux-button success">Success Button</button></div>"# },
        UxComponentSpec { id: "button-sizes", category: "buttons", title: "Button Sizes", primitive: "ux-button small/large", description: "Shared size scale for all future action rows.", sample: r#"<div class="ux-row"><button class="ux-button small">Small</button><button class="ux-button">Medium</button><button class="ux-button large">Large</button></div>"# },
        UxComponentSpec { id: "toggle-basic", category: "toggles", title: "Toggle Switches", primitive: "ux-toggle", description: "On/off state with accessible native input.", sample: r#"<div class="ux-grid"><label class="ux-toggle"><input type="checkbox"> Small Toggle</label><label class="ux-toggle"><input type="checkbox" checked> Medium Toggle</label><label class="ux-toggle"><input type="checkbox" checked disabled> Disabled On</label><label class="ux-toggle"><input type="checkbox" aria-label="Toggle without label"></label></div>"# },
        UxComponentSpec { id: "tabs-plain", category: "tabs", title: "Plain Tabs", primitive: "ux-tabs / ux-tab", description: "Ordinary navigation tabs with no favorite or visibility affordance.", sample: r#"<div class="ux-card" data-ux-tab-affordance="plain"><h4>Plain tab strip</h4><div class="ux-tabs ux-tab-strip"><button class="ux-tab active" aria-selected="true">Overview</button><button class="ux-tab" aria-selected="false">Stats</button><button class="ux-tab" aria-selected="false">Files</button><button class="ux-tab" aria-selected="false" disabled>Disabled</button></div></div>"# },
        UxComponentSpec { id: "tabs-favorite", category: "tabs", title: "Favorite Tabs", primitive: "ux-tab + ux-tab-star", description: "Tabs can carry a star affordance when the surface supports choosing the favorite/default tab.", sample: r#"<div class="ux-card" data-ux-tab-affordance="favorite"><h4>Favorite tab strip</h4><div class="ux-tabs ux-tab-strip ux-tab-strip-favorite"><button class="ux-tab active" aria-selected="true"><span>Upload</span><span class="ux-tab-star" aria-label="favorite tab">★</span></button><button class="ux-tab" aria-selected="false"><span>Portals</span><span class="ux-tab-star muted" aria-label="mark favorite">☆</span></button><button class="ux-tab" aria-selected="false"><span>Stats</span><span class="ux-tab-star muted" aria-label="mark favorite">☆</span></button></div></div>"# },
        UxComponentSpec { id: "tabs-favorite-visibility", category: "tabs", title: "Favorite + Visibility Tabs", primitive: "ux-tab + ux-tab-star + ux-tab-eye", description: "Admin tab rows combine favorite stars with an eyeball affordance for hiding, showing, and faded hidden state without deleting the tab.", sample: r#"<div class="ux-card" data-ux-tab-affordance="favorite-visibility"><h4>Favorite + hide/fade strip</h4><div class="ux-tabs ux-tab-strip ux-tab-strip-managed"><button class="ux-tab active" aria-selected="true"><span>Portals</span><span class="ux-tab-star" aria-label="favorite tab">★</span><span class="ux-tab-eye" aria-label="visible tab">👁</span></button><button class="ux-tab" aria-selected="false"><span>Stats</span><span class="ux-tab-star muted" aria-label="mark favorite">☆</span><span class="ux-tab-eye" aria-label="visible tab">👁</span></button><button class="ux-tab ux-tab-faded" aria-selected="false" data-hidden-tab="true"><span>Admin</span><span class="ux-tab-star muted" aria-label="mark favorite">☆</span><span class="ux-tab-eye muted" aria-label="hidden tab">◌</span></button></div></div>"# },
        UxComponentSpec { id: "input-basic", category: "inputs", title: "Input Fields", primitive: "ux-field", description: "Text, password, read-only, and validation states.", sample: r#"<div class="ux-grid"><label>Username<input class="ux-field" placeholder="Enter username"></label><label>Email with error<input class="ux-field" value="ab" aria-invalid="true"><span class="error">Input must be at least 3 characters</span></label><label>Password<input class="ux-field" type="password" placeholder="Enter password"></label><label>Display<input class="ux-field" value="homeserver" readonly></label></div>"# },
        UxComponentSpec { id: "card-variants", category: "cards", title: "Cards", primitive: "ux-card", description: "Default, header, clickable, active, and error card states.", sample: r#"<div class="ux-grid"><div class="ux-card">Default card body content.</div><div class="ux-card"><h4>Card Header</h4><p>Card body content goes here.</p></div><button type="button" class="ux-card ux-card-button clickable"><strong>Clickable card</strong><span> opens a focused action or drill-in</span></button><div class="ux-card active">Active card</div><div class="ux-card error">Error card</div></div>"# },
        UxComponentSpec { id: "badge-variants", category: "badges", title: "Badges", primitive: "ux-badge", description: "Primary, secondary, status, warning, danger, and info labels.", sample: r#"<div class="ux-row"><span class="ux-badge primary">Primary</span><span class="ux-badge secondary">Secondary</span><span class="ux-badge success">Success</span><span class="ux-badge warning">Warning</span><span class="ux-badge danger">Danger</span><span class="ux-badge info">Info</span></div>"# },
        UxComponentSpec { id: "checkbox-variants", category: "checkboxes", title: "Checkboxes", primitive: "ux-checkbox", description: "Checked, unchecked, disabled, and disabled checked states.", sample: r#"<div class="ux-grid"><label class="ux-checkbox"><input type="checkbox" checked> Checked checkbox</label><label class="ux-checkbox"><input type="checkbox"> Unchecked checkbox</label><label class="ux-checkbox"><input type="checkbox" disabled> Disabled unchecked</label><label class="ux-checkbox"><input type="checkbox" checked disabled> Disabled checked</label></div>"# },
        UxComponentSpec { id: "utility-row", category: "utilities", title: "Utility Controls", primitive: "ux-row / ux-breadcrumbs", description: "Icon buttons, editable fields, breadcrumb rows, and focusable cards share one interactable grammar.", sample: r#"<div class="ux-grid"><button class="ux-button small" aria-label="Visibility toggle">👁</button><button class="ux-button small">＋</button><div class="ux-card">Editable field <input class="ux-field" value="editable value"></div><div class="ux-breadcrumbs"><span>/mnt</span><span>›</span><span>nas</span><span>›</span><strong>media</strong></div></div>"# },
        UxComponentSpec { id: "calendar-time", category: "calendar-time", title: "Calendar & Time", primitive: "ux-select / ux-field", description: "Weekly, monthly, and time picker grammar.", sample: r#"<div class="ux-grid"><label>Weekly day<select class="ux-select"><option>Monday</option><option>Friday</option></select></label><label>Monthly day<input class="ux-field" type="number" value="15"></label><label>Time<input class="ux-field" type="time" value="03:00"></label></div>"# },
        UxComponentSpec { id: "row-info-tile", category: "row-info-tile", title: "Row Info Tile", primitive: "ux-card + ux-row", description: "Compact row card for device or service state.", sample: r#"<div class="ux-grid"><div class="ux-card"><div class="ux-row"><strong>Device</strong><span class="ux-badge success">Online</span></div><p>Service A · healthy</p></div><div class="ux-card"><div class="ux-row"><strong>Backup</strong><span class="ux-badge warning">Pending</span></div><p>Awaiting next sync window.</p></div></div>"# },
        UxComponentSpec { id: "dropdowns", category: "dropdowns", title: "Dropdowns", primitive: "ux-select", description: "Selection controls that inherit theme and focus tokens.", sample: r#"<div class="ux-grid"><label>Device<select class="ux-select"><option>Device A</option><option>Device B</option></select></label><label>Strategy<select class="ux-select"><option>Safe apply</option><option>Dry run</option></select></label></div>"# },
        UxComponentSpec { id: "slider", category: "slider", title: "Slider", primitive: "range + theme accent", description: "Bounded numeric adjustment control.", sample: r#"<div class="ux-grid"><label>Threshold<input type="range" min="0" max="100" value="50"></label><label>Warning level<input type="range" min="0" max="100" value="75"></label></div>"# },
        UxComponentSpec { id: "textbox", category: "textbox", title: "Text Box", primitive: "ux-textbox", description: "Multi-line text/code/log input.", sample: r#"<textarea class="ux-textbox" rows="8">function calculateTotal(items) { return items.reduce((sum, item) => sum + item.price, 0); }</textarea>"# },
        UxComponentSpec { id: "upload-components", category: "upload-components", title: "Upload Components", primitive: "ux-field / drop-zone / ux-breadcrumbs", description: "File input, drop target, and path breadcrumbs.", sample: r#"<div class="ux-grid"><label>File<input class="ux-field" type="file"></label><div class="drop-zone"><strong>Drop zone</strong><p>Choose files for a target directory.</p></div><div class="ux-breadcrumbs"><span>/root</span><span>›</span><span>path</span><span>›</span><strong>uploads</strong></div></div>"# },
        UxComponentSpec { id: "progress-bar", category: "progress-bar", title: "Progress Bar", primitive: "ux-progress", description: "Deterministic progress visualization.", sample: r#"<div class="ux-stack"><div class="ux-progress"><span style="width:25%"></span></div><div class="ux-progress"><span style="width:75%"></span></div></div>"# },
        UxComponentSpec { id: "table", category: "table", title: "Table", primitive: "ux-table", description: "Tabular state/action grid.", sample: r#"<table class="ux-table"><thead><tr><th>Name</th><th>Status</th><th>Action</th></tr></thead><tbody><tr><td>Service A</td><td><span class="ux-badge success">Active</span></td><td><button class="ux-button small">Inspect</button></td></tr><tr><td>Membrane</td><td><span class="ux-badge warning">Boundary</span></td><td><button class="ux-button small secondary">Read</button></td></tr></tbody></table>"# },
        UxComponentSpec { id: "collapsible", category: "collapsible", title: "Collapsible", primitive: "details + ux-card", description: "Disclosure rows for compact evidence.", sample: r#"<details class="ux-card"><summary>Open details</summary><p>Composable details content.</p></details><details class="ux-card" open><summary>Open by default</summary><pre class="ux-readout">receipt: testtab.component.collapsible.v1</pre></details>"# },
        UxComponentSpec { id: "modals", category: "modals", title: "Modals", primitive: "ux-modal-sample", description: "Small and large modal grammar without live side effects.", sample: r#"<div class="ux-grid"><div class="ux-modal-sample"><h4>Small modal</h4><p>Modal body</p><div class="ux-row"><button class="ux-button secondary">Cancel</button><button class="ux-button">Confirm</button></div></div><div class="ux-modal-sample"><h4>Large modal</h4><pre class="ux-readout">[INFO] component sample rendered successfully</pre></div></div>"# },

        UxComponentSpec { id: "theme-gradients", category: "theme-system", title: "Theme Gradients", primitive: "gradient tokens", description: "Gradient tokens are named by intent so skins can become richer without component rewrites.", sample: r#"<div class="ux-grid"><div class="ux-gradient-swatch primary" title="gradient-primary"></div><div class="ux-gradient-swatch accent" title="gradient-accent"></div><div class="ux-gradient-swatch surface" title="gradient-surface"></div><div class="ux-gradient-swatch highlight" title="gradient-highlight"></div></div>"# },
        UxComponentSpec { id: "theme-highlights", category: "theme-system", title: "Highlight Layers", primitive: "highlight tokens", description: "Highlight and state-layer tokens separate attention from success/status color.", sample: r#"<div class="ux-grid"><div class="ux-card ux-highlight-card">Focused highlight card</div><button class="ux-button ux-state-layer">Hover state layer</button><div class="ux-card" style="box-shadow: var(--theme-elevation-2)">Elevation 2 card</div></div>"# },
        UxComponentSpec { id: "theme-accents", category: "theme-system", title: "Accent Families", primitive: "accent tokens", description: "Warm, cool, neutral, and critical accents give visual range while status colors stay truthful.", sample: r#"<div class="ux-stack"><div class="ux-accent-strip"><span></span><span></span><span></span><span></span></div><div class="ux-row"><span class="ux-badge primary">warm</span><span class="ux-badge info">cool</span><span class="ux-badge secondary">neutral</span><span class="ux-badge danger">critical</span></div></div>"# },
        UxComponentSpec { id: "theme-role-pairs", category: "theme-system", title: "Role Pairs", primitive: "role/on-role tokens", description: "Material-style role pairs keep container/on-container contrast explicit.", sample: r#"<div class="ux-grid"><div class="ux-role-pair primary"><strong>role-primary</strong><span>role-on-primary</span></div><div class="ux-role-pair"><strong>surface-1</strong><span>on-surface / outline</span></div></div>"# },
    ]
}

fn title_for_category(category: &str) -> String {
    category
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn ux_component_categories(registry: &[UxComponentSpec]) -> Vec<&'static str> {
    let mut categories = Vec::new();
    for component in registry {
        if !categories.contains(&component.category) {
            categories.push(component.category);
        }
    }
    categories
}

fn render_ux_component_card(component: &UxComponentSpec) -> String {
    format!(
        r#"<article class="ux-card ux-component-card" data-ux-component="{id}" data-ux-category="{category}" data-ux-primitive="{primitive}"><div class="ux-row"><h4>{title}</h4><span class="ux-badge secondary">{primitive}</span></div><p>{description}</p><div class="ux-component-sample">{sample}</div></article>"#,
        id = component.id,
        category = component.category,
        primitive = component.primitive,
        title = component.title,
        description = component.description,
        sample = component.sample
    )
}

fn render_ux_component_showcase(registry: &[UxComponentSpec]) -> String {
    let categories = ux_component_categories(registry);
    let tabs = categories
        .iter()
        .enumerate()
        .map(|(index, category)| {
            format!(
                r#"<button type="button" class="ux-tab {active}" data-showcase-tab="{category}" aria-selected="{selected}">{title}</button>"#,
                active = if index == 0 { "active" } else { "" },
                selected = if index == 0 { "true" } else { "false" },
                category = category,
                title = title_for_category(category)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let panels = categories
        .iter()
        .enumerate()
        .map(|(index, category)| {
            let cards = registry
                .iter()
                .filter(|component| component.category == *category)
                .map(render_ux_component_card)
                .collect::<Vec<_>>()
                .join("");
            format!(
                r#"<section class="ux-panel {active}" data-showcase-panel="{category}"><div class="ux-row"><h3>{title}</h3><span class="ux-badge primary">{count} component{plural}</span></div><div class="ux-grid">{cards}</div></section>"#,
                active = if index == 0 { "active" } else { "" },
                category = category,
                title = title_for_category(category),
                count = registry.iter().filter(|component| component.category == *category).count(),
                plural = if registry.iter().filter(|component| component.category == *category).count() == 1 { "" } else { "s" },
                cards = cards
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!(
        r#"<section class="ux-panel active" data-testtab-panel="showcase"><div class="ux-row"><h3>Component Showcase</h3><span class="ux-badge info" data-ux-registry-count>{count} registered UX components</span></div><div class="ux-tabs" role="tablist" aria-label="Component showcase categories" data-showcase-tabs>{tabs}</div><div class="ux-stack" data-showcase-panels>{panels}</div></section>"#,
        count = registry.len(),
        tabs = tabs,
        panels = panels
    )
}

fn render_testtab_services() -> String {
    r#"<section class="ux-panel" data-testtab-panel="services"><div class="ux-grid"><article class="ux-card active"><h3>Data Generator</h3><p>Generate anonymized sample datasets for testing and development.</p><div class="ux-row"><span class="ux-badge success">active</span><span class="ux-badge secondary">Random Data</span><span class="ux-badge secondary">User Data</span></div><button class="ux-button small">Run action</button></article><article class="ux-card active"><h3>Analytics Processor</h3><p>Process and analyze anonymized datasets.</p><div class="ux-row"><span class="ux-badge success">active</span><span class="ux-badge secondary">Statistical Analysis</span></div><button class="ux-button small">Run action</button></article><article class="ux-card active"><h3>External API Client</h3><p>Fetch sample data from external APIs with timeout and error handling.</p><div class="ux-row"><span class="ux-badge success">active</span><span class="ux-badge secondary">HTTP Requests</span></div><button class="ux-button small">Run action</button></article><article class="ux-card active"><h3>Health Monitor</h3><p>Monitor TestTab dependencies and UX registry presence.</p><div class="ux-row"><span class="ux-badge success">active</span><span class="ux-badge secondary">Status Reporting</span></div><button class="ux-button small" data-testtab-health-check>Run Health Check</button></article></div></section>"#.to_string()
}

fn render_testtab_config(registry_count: usize) -> String {
    format!(r#"<section class="ux-panel" data-testtab-panel="config"><article class="ux-card"><h3>Configuration</h3><div class="ux-grid"><div><h4>Basic Information</h4><p><strong>Name:</strong> TestTab</p><p><strong>Description:</strong> perpetual anonymized UX component utility</p><p><strong>Version:</strong> registry-native</p></div><div><h4>Capabilities</h4><div class="ux-row"><span class="ux-badge success">component_showcase</span><span class="ux-badge success">health_monitor</span><span class="ux-badge success">theme_truth</span><span class="ux-badge success">auto_registry</span></div></div><div><h4>Settings</h4><table class="ux-table"><tbody><tr><td>install mode</td><td>first-party-native</td></tr><tr><td>ux source</td><td>coronatio-composable-ux.v1</td></tr><tr><td>registered components</td><td>{registry_count}</td></tr></tbody></table></div></div></article></section>"#)
}

fn render_testtab_health(registry_count: usize) -> String {
    format!(r#"<section class="ux-panel" data-testtab-panel="health"><article class="ux-card"><div class="ux-row"><h3>Health Status</h3><button class="ux-button" data-testtab-health-check>Run Health Check</button></div><pre class="ux-readout" data-testtab-health-output>{{ "schema": "coronatio.testtab.health.v1", "status": "ready", "dependencies": {{ "rust_shell": true, "theme_catalog": true, "ux_library": true, "registered_components": {registry_count} }} }}</pre></article></section>"#)
}

fn render_testtab_theme_truth() -> String {
    r#"<section class="ux-panel" data-testtab-panel="theme-truth"><article class="ux-card"><h3>Theme Truth</h3><p>Dark buttons use <code>primary</code>/<code>primaryHover</code>; green is success/status only. Expanded JSON tokens add roles, gradients, highlights, accents, elevation, overlays, state layers, focus, contrast, density, and component slots without abandoning the flat legacy token membrane.</p><div class="ux-grid"><div class="ux-card"><h4>Gradient tokens</h4><div class="ux-grid"><div class="ux-gradient-swatch primary"></div><div class="ux-gradient-swatch accent"></div><div class="ux-gradient-swatch surface"></div><div class="ux-gradient-swatch highlight"></div></div></div><div class="ux-card ux-highlight-card"><h4>Highlight token</h4><p>highlight-subtle / highlight-strong / highlight-ring</p></div><div class="ux-card"><h4>Accent families</h4><div class="ux-accent-strip"><span></span><span></span><span></span><span></span></div></div></div><table class="ux-table theme-token-table"><thead><tr><th>token</th><th>computed</th><th>source</th></tr></thead><tbody data-theme-token-readout><tr><td>--primary</td><td>loading</td><td>dark.json primary #323840</td></tr><tr><td>--theme-gradient-accent</td><td>loading</td><td>expanded JSON gradient-accent</td></tr><tr><td>--theme-highlight-strong</td><td>loading</td><td>expanded JSON highlight-strong</td></tr><tr><td>--theme-role-primary</td><td>loading</td><td>expanded JSON role-primary</td></tr></tbody></table></article></section>"#.to_string()
}

fn render_testtab() -> String {
    let registry = ux_component_registry();
    format!(
        r#"<section class="pane" id="pane-testtab" data-pane-panel="testtab" role="tabpanel" aria-label="TestTab"><div class="test-tablet ux-surface" data-native-stock-testtab="true" data-admin-viewport="testtab" data-react-quarry="premium/testTab" data-ux-library="coronatio-composable-ux.v1" data-ux-registry="rust-native"><div class="ux-tabs" role="tablist" aria-label="TestTab sections" data-testtab-tabs><button type="button" class="ux-tab active" data-testtab-tab="showcase" aria-selected="true">Component Showcase</button><button type="button" class="ux-tab" data-testtab-tab="services" aria-selected="false">Service Tests</button><button type="button" class="ux-tab" data-testtab-tab="config" aria-selected="false">Configuration</button><button type="button" class="ux-tab" data-testtab-tab="health" aria-selected="false">Health Status</button><button type="button" class="ux-tab" data-testtab-tab="theme-truth" aria-selected="false">Theme Truth</button></div><div class="test-tablet-content ux-stack">{showcase}{services}{config}{health}{theme_truth}</div></div></section>"#,
        showcase = render_ux_component_showcase(&registry),
        services = render_testtab_services(),
        config = render_testtab_config(registry.len()),
        health = render_testtab_health(registry.len()),
        theme_truth = render_testtab_theme_truth()
    )
}
