# shell ux hierarchy

The shell UX library now obeys infinite-infinite file hierarchy law: ordered children live in `index.json`, and serve-time concatenation follows that list exactly. Adding a file means adding it to `index.json` in the intended byte order.

## Three-kind law

- `library/` holds og `styles/common/ui` mirrors, one file per og source. Og underscore names are kept one-to-one for traceability.
- `packs/` holds per-pane absorbed domain packs. Og tab-local declarations stay byte-identical, plus a clearly labeled `og inline-style fold` section when a tranche folds TSX inline styles or element defaults into the pack. Og truth = tab CSS + TSX inline styles + element defaults.
- `shell/` holds crown chrome/base CSS that is neither a library primitive nor a pane pack. Legacy pane declarations that predate their future packs remain in `shell/document-2-css.css` until a later extraction promotes them to a named pack.

## How to add

1. Shared primitive from `styles/common/ui`: copy the og CSS into `library/<og-name>.css`, keep the underscore filename, then add that path to `index.json`.
2. Pane/domain absorption: copy the tab-local declarations byte-identically into `packs/<pane>.css`; if the source included inline styles or element defaults, add a labeled `og inline-style fold` section; then add that path to `index.json`.
3. Crown shell/base chrome: place non-pane, non-library shell substrate under `shell/<concern>.css`, then add that path to `index.json`.

Markup vocabulary law for adding a pane lives in `src/bands/README.md` under `Adding a crown pane`.

For tab and cartridge authors, read [TAB-AUTHOR.md](TAB-AUTHOR.md). It is the shallow Theme Net author face; `author-face.json` is its machine-readable allowlist.
