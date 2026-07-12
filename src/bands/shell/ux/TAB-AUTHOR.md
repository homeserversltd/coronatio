# Theme Net tab author cookbook

Audience: humans and small open-source-trained models adding a Coronatio tab or cartridge.

The Theme Net has two levels. The shell owns the full system lattice and its beauty materials. A tab sees the shallow author face only. This is progressive disclosure, not a second theme system: see `pali:agentics-theme-net-indra-skin-tablet`, `pali:agentics-theme-net-projection-membrane-tablet`, and `pali:agentics-theme-net-tab-author-face-tablet`.

## Build ladder

1. Compose existing `.ux-*` and `ui-*` library classes first (`.ux-stack`, `.ux-row`, `.ux-grid`, `.ux-card`, `.ux-button`, `.ux-tab`, `.ux-field`, `.ux-badge`, `.ui-progress-bar` and its `__container`, `__fill`, `__labels`, `__text`, size, variant, state, and indeterminate classes, plus other focused library components). Progress semantics and the upload-domain reflection are governed by `pali:agentics-ux-progress-constitution-tablet` and `pali:workflow-coronatio-progress-primitive-binding-tablet`.
2. Add small pack CSS only when composition cannot express the domain need.
3. In pack CSS, use `var(--name)` only from the author face below.
4. If a domain needs a local emphasis, rebind one author-face name at its root:

```css
[data-domain="storage"] {
  --accent: var(--info);
  --surface-soft: var(--background-alt);
}
```

Do not invent hex colors. Do not use `--theme-role-*`, `--theme-surface-*`, `--theme-gradient-*`, `--theme-elevation-*`, flags, components, or another token catalog in a tab pack. Shell CSS owns deep materials. A pack may not call any variable outside this list.

## Author-face allowlist

```text
background
background-alt
background-alt-hover
surface
surface-soft
text
text-secondary
text-muted
secondary
primary
primaryHover
accent
accent-soft
border
error
warning
success
info
status-up
status-down
status-partial
status-unknown
hiddenTabBackground
hiddenTabText
shadow
primary-rgb
background-rgb
statusUp
statusDown
statusPartial
statusUnknown
border-radius
font-size-sm
transition-fast
monoFont
tabBorder
tabContentBackground
disabled
border-color
```

The machine-readable source is `author-face.json`; the focused Rust wall tests enforce it across `packs/**/*.css`. If a repeated quarry-shaped name is needed, add one shell rebind and this list in the same change. Rewrite one-off names to an existing author-face variable instead.

## One-to-one port

Coronatio remains the HOMESERVER one-to-one port (`pali:coronatio-original-website-firmware-port-law`). Familiar paint names are allowed here so small authors can build a tab without learning the deep lattice. The shallow face is the only public paint membrane; the Theme Net remains alive beneath it.
