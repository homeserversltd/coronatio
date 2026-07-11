# Theme tokens

Coronatio themes are JSON maps projected into CSS custom properties. One catalog supplies the whole crown: shell, native panes, controls, cards, modals, and cartridge-facing visual roles.

The catalog lives at `src/bands/theme/catalog.json`. It currently contains `light`, `dark`, and `radioactive`. Runtime selection comes from `global.theme.name` in `homeserver.json`; the catalog provides the corresponding firmware values. Do not create a second runtime theme authority.

## How a token travels

A catalog key such as `surface-1` becomes the CSS variable `--theme-surface-1`. Rust serves the selected map through the theme route, and the browser projects it through the existing `style[data-theme-styles]` membrane and `<html data-theme>` state.

The flat string map is intentional. Colors, dimensions, shadows, gradients, opacities, and feature flags can travel through the same validated route without a parallel schema per CSS value type.

## Choose a name by purpose

Prefer a role over a literal appearance:

- `surface-*` and `on-surface*` for layered backgrounds and their text;
- `role-*` and `role-on-*` for semantic color pairs;
- `status-*` for success, warning, error, and information;
- `outline*`, `focus-*`, and `state-*-opacity` for interaction;
- `spacing-*`, `font-*`, `radius-*`, and `elevation-*` for shared geometry;
- `component-*` only when a component cannot honestly use a broader role.

Avoid names such as `blue-card` or `dark-border`: they describe one theme rather than the token's job.

## Add or change a token

1. Add the key to every theme in `src/bands/theme/catalog.json`.
2. Keep values as JSON strings, including numbers represented for CSS such as `".12"`.
3. Consume the projected `--theme-<key>` variable in the owning CSS surface.
4. Demonstrate a new visual primitive in the Test pane before spreading it across production panes when the primitive is genuinely new.
5. Run the theme and full proofs.

```bash
cargo test theme_net -- --nocapture
cargo fmt --check
cargo test
```

The tests check catalog families, projection into CSS variables, and representative consumers. Also inspect the rendered light, dark, and radioactive themes: a present variable can still be an unreadable pairing.

## Compatibility

Existing keys are part of the visible appliance grammar. Rename or remove one only after updating every consumer and proving all themes. Adding an alias forever is not preferable to one bounded migration, but silently leaving one pane on an old literal color is worse.

Theme choice belongs to the household memory. Firmware defaults may help Coronatio start, but they must not override the selected `homeserver.json` theme.
