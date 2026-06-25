# Coronatio theme JSON authority

Coronatio reads themes at runtime from:

```text
static/themes/theme.json
```

On the installed homeserver this resolves through `/opt/coronatio/source/static/themes/theme.json` unless `CORONATIO_THEME_JSON` names another file.

The file is one JSON catalog with this shape:

```json
{
  "schema": "coronatio.theme-catalog.v1",
  "default": "dark",
  "themes": {
    "theme-name": {
      "color-primary": "#00f2fe"
    }
  }
}
```

Every theme under `themes` must contain the complete required variable set used by `src/bands/crown-law.rs`. Theme names are the object keys and propagate into `/api/themes`, `<html data-theme>`, localStorage `preferred-theme`, browser `themeData`, the header theme button, and `style[data-theme-styles]`.

To add a user theme, add a new object under `themes`, preserve all required keys as CSS value strings, save the file, and reload Coronatio. No Rust source edit is required for a new theme entry.

Theme behavior remains governed by the one-to-one port doctrine: the JSON catalog supplies the Rust implementation substrate, while the visible theme control and persistence behavior must match the original Flask/React Header and ThemeComponent behavior unless an explicit divergence is recorded.
