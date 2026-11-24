# shell ux hierarchy

Shared sheets, the theme catalog, and author-face names live in xenia-kit `style/`; changes to them belong in xenia-kit.

The shell UX library now obeys infinite-infinite file hierarchy law: ordered children live in `index.json`, and serve-time concatenation follows that list exactly. Adding a file means adding it to `index.json` in the intended byte order.

## Three-kind law

- xenia-kit `style/library/` holds shared og `styles/common/ui` mirrors, one file per og source. Og underscore names are kept one-to-one for traceability.
- `packs/` holds crown-only per-pane absorbed domain packs; the stats and portals packs live in xenia-kit `style/packs/`. Og tab-local declarations stay byte-identical, plus a clearly labeled `og inline-style fold` section when a tranche folds TSX inline styles or element defaults into the pack. Og truth = tab CSS + TSX inline styles + element defaults.
- `shell/` sheets, including explain CSS, base/chrome and `shell/document-2-css.css`, live in xenia-kit `style/shell/`.

## How to add

1. Shared primitives live in xenia-kit `style/library/`; update the kit and retain the intended relative path in `index.json`.
2. Pane/domain absorption: copy the tab-local declarations byte-identically into `packs/<pane>.css`; if the source included inline styles or element defaults, add a labeled `og inline-style fold` section; then add that path to `index.json`.
3. Crown shell/base chrome: place non-pane, non-library shell substrate under `shell/<concern>.css`, then add that path to `index.json`.

Markup vocabulary law for adding a pane lives in `src/bands/README.md` under `Adding a crown pane`.

For tab and cartridge authors, read [TAB-AUTHOR.md](TAB-AUTHOR.md). It is the shallow Theme Net author face; xenia-kit `style/author-face.json` is its machine-readable allowlist.
