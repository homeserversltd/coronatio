# Coronatio theme projection

Coronatio is a one-to-one port of the old Flask/React HOMESERVER surface. Theme behavior is therefore governed first by the original HomeServer state model and the central HomeServer config file, `homeserver.json`.

Runtime theme selection SHALL come from `homeserver.json`, specifically `global.theme.name`, before any Coronatio-local fallback or firmware default. On the installed homeserver the primary authority is `/etc/homeserver.json`; the legacy live HomeServer source path is a fallback/quarry read surface for migration and tests.

Coronatio may carry firmware theme token defaults so the Rust shell can render safely, but those defaults are implementation substrate, not user state authority. `/api/themes` projects the selected `homeserver.json` theme and firmware token defaults into the browser-visible one-to-one membrane:

```text
preferred-theme
browser themeData
style[data-theme-styles]
<html data-theme>
--theme-* CSS variables
```

The visible theme control and persistence behavior must match the original Flask/React Header and ThemeComponent behavior unless an explicit divergence is recorded.
