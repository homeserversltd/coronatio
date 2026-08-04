# How Coronatio holds the household

Coronatio is the HomeServer crown: a Rust process that presents the machine as one appliance rather than a collection of unrelated services. Its stable shell holds the header, navigation, session state, and pane frame. Inside that frame, the household can open as many useful views as it needs.

## The crown and its panes

A **native pane** is part of Coronatio itself. Admin, Stats, Portals, and Upload are compiled with the binary because they define the appliance face. Their routes, state shapes, rendering, and tests move together.

A **cartridge** is an installed extension. It contributes a validated `tab.json` manifest, static assets, and a local service boundary beneath the configured cartridge root. Coronatio can discover and serve it at runtime. A cartridge therefore adds a tab without pretending to be firmware, and its failure can be contained to its own pane.

A trusted feature that needs deep compile-time integration may instead enter through a source-injection rebuild. That lane is deliberate and proven like any native change; it is not the default for an installed service.

## One shell, typed membranes

The browser does not receive one giant application with silent access to everything. Coronatio serves a stable shell and admits pane fragments through explicit routes. Ordinary reads return typed JSON or server-rendered HTML. HTMX carries pane interactions where a fragment is the honest response.

Admin mode is a leased server session, not a browser claim. Guest and admin views are projected from the same underlying facts, with privileged fields and controls filtered before rendering.

## The household memory

`config.json` is the shared household memory at `/etc/appliance/config.json`. It contains the facts that should survive a browser closing or the crown restarting: selected theme, favorite and visible tabs, portal configuration, upload defaults, and other household choices.

The browser may remember short-lived presentation details, but it does not become a rival configuration authority. Development fixtures can be selected with `CORONATIO_HOMESERVER_JSON`; that override exists to test the same behavior away from a live appliance. Factory portal readback uses `/etc/appliance/config.factory`.

## Reads and actions take different lanes

A status card is a read. Restarting a service, changing a system setting, or writing protected configuration is an action. Coronatio keeps those paths distinct.

For admitted privileged actions, the crown sends a narrow request to **Caduceus**. Caduceus is the privileged actuator: it accepts only named capabilities and returns a result the crown can show. Coronatio does not turn a web request into unrestricted shell access.

**Harmonia** works on a longer rhythm. It owns the installed profile and converges services and configuration toward that declared state. The crown may display the result, but a button click is not allowed to impersonate the system's maintenance authority.

## Pulse: a doorbell, not a moving van

Coronatio uses Server-Sent Events for its pulse. The server keeps one lightweight stream open and sends small topic notifications such as “tabs changed” or “stats changed.” The notification carries the reason to look again, not the entire household state.

When the browser hears the doorbell, it fetches the affected typed route or fragment. This keeps live panes current without constant polling and without coupling every state shape to one permanent stream. Leases and renewals let abandoned browser sessions fall away.

## Rebuilding without making users relearn

The old Flask and React application is a quarry of proven behavior. Coronatio replaces that substrate with Rust, typed state, Caduceus actions, Harmonia convergence, and explicit browser fragments. The visible appliance is meant to remain familiar: the same four primary panes, the same control meanings, and the same class of feedback under the same state.

The quarry teaches what the machine already learned. The crown gives those lessons a structure that can survive.

> Governing design: `pali:coronatio-north-star-contract` and `pali:workflow-coronatio-flask-react-visual-ux-identity-contract`.
