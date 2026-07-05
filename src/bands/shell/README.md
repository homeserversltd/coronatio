# shell band

Thin shell face; Rust-delivered document chunks recompose through `render.rs` with runtime helpers separate.

## Shell band law

Coronatio shell documents SHALL stay chunked in document order, with `render.rs` recomposing the chunks, replacing pane `__PLACEHOLDER__` markers, and extracting inline chrome into `/static/crown/chrome.js` so the crown keeps its CSP posture.

Delegated-chrome law: shell interactivity SHALL bind once through body-level delegated listeners keyed by stable `data-*` attributes. Pane fragments SHALL NOT attach init-time listeners, because HTMX swaps replace fragments and orphan those handlers; delegated listeners survive the swap boundary.

Generic tab-scope convention: any pane may declare inner tabs with one markup grammar and zero JavaScript edits. The nearest `[data-tab-scope="<scope>"]` container owns its buttons and panels; buttons use `[data-tab-id="<panel>"]`, panels use `[data-tab-panel="<panel>"]`, and chrome toggles `active`, `ui-tab--active`, and `aria-selected` only inside that nearest scope. Nested tab scopes are lawful when each tab button resolves to its closest scope.

## Shell UX hierarchy

Document CSS for shell documents 1 and 2 lives under `src/bands/shell/ux/`. `ux/index.json` is the serve-time order authority. Use `ux/library/` for og `styles/common/ui` one-to-one files, `ux/packs/` for pane domain packs, and `ux/shell/` for crown chrome/base or legacy shell CSS that has not yet become a pack.
