# Coronatio theme projection

Coronatio is a one-to-one port of the old Flask/React HOMESERVER surface. Theme behavior is governed first by the original HomeServer state model and the central HomeServer config file, `homeserver.json`.

Runtime theme selection SHALL come from `/etc/appliance/config.json`, specifically `global.theme.name`, before any Coronatio-local fallback or firmware default. `CORONATIO_HOMESERVER_JSON` is the sole development and test fixture seam.

Coronatio carries exactly one firmware-owned embedded token catalog at `src/bands/theme/catalog.json`; it is Rust build substrate, not user state authority and has no served sidecar. `/api/themes` reads `homeserver.json` only for `global.theme.name`, then projects that selected catalog mode into the browser-visible one-to-one port membrane:

```text
preferred-theme
browser themeData
style[data-theme-styles]
<html data-theme>
--theme-* CSS variables
legacy aliases: --background, --text, --primary, --primaryHover, --hiddenTabBackground, --hiddenTabText
```

The Theme Net is the sole paint speech: `themeToCss` emits every catalog key as `--theme-*`; legacy names are pure CSS `var(--theme-…)` rebinds in the shell, never a copied JavaScript value map. The catalog includes color, status, spacing, sizing, radius, elevation, gradients, flags, transition, font-family, monospace font, font-size, font-weight, and line-height tokens. Production shell chrome binds those materials while preserving the original one-to-one port identity.

The visible theme control and persistence behavior must match the original Flask/React Header and ThemeComponent behavior unless an explicit divergence is recorded.
