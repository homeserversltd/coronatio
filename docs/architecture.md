# Coronatio architecture

Coronatio is a Rust process that presents a self-hosted appliance through one web interface. A stable shell provides the header, tab bar, session controls, and pane frame.

## Native tabs

Native tabs are compiled into Coronatio. The current set is Admin, Portals, Upload, Stats, backBlaze, Wake on LAN, Test, Linker, DHCP, Firewall, and DNS. Admin, Wake on LAN, Linker, Firewall, and DNS require admin mode. DHCP is hidden by default.

Each native tab has an explicit route and state source. Native tabs are the right place for features that need direct integration with Coronatio's rendering, session filtering, or typed routes.

## Runtime cartridges

Runtime cartridges add tabs without recompiling Coronatio. Their registry is `/etc/appliance/cartridges.json`, using schema `appliance.cartridges.v1`. Each row contains `id`, `title`, `url`, `guest_class: "iframe"`, and `admin_only`.

Coronatio reads valid, unique rows into the tab list. Opening a cartridge tab requests `/admit/<id>`, which returns a sandboxed iframe for the configured URL. The iframe permits scripts, same-origin access within the framed service, and forms. It does not grant popups, downloads, or top-level navigation.

Adding and removing cartridges is available from the `+` button in admin mode. The browser sends the request through Coronatio to the privileged actuator. An attended admin session is required, and Coronatio does not edit the registry file itself.

See [Loadable cartridges](cartridges.md) for the registry format, validation rules, and framing requirements.

## Configuration

The installed configuration is `/etc/appliance/config.json`. It stores shared choices such as the selected theme, favorite and visible tabs, portal settings, and upload defaults. Development runs can select another file with `CORONATIO_HOMESERVER_JSON`. Factory portal readback uses `/etc/appliance/config.factory`.

The browser may retain short-lived presentation state, but persistent appliance settings come from server-side configuration.

## Reads and privileged actions

Status and configuration reads use ordinary HTTP routes. Actions that need host privileges use a separate actuator with a limited set of named operations. Coronatio checks the admin session before forwarding protected actions; it does not turn browser requests into unrestricted shell commands.

## Live updates

Coronatio uses Server-Sent Events for live notifications. The stream sends small topic changes, such as tabs or statistics changing. The browser then reloads the affected route or fragment instead of receiving the entire application state over the event stream.

This keeps live panes current without constant polling and lets abandoned browser sessions expire.
