# Coronatio favorite manifest

`favorites.json` is the Rust-owned manifest for the old Flask/React favorite/starred-tab system.

Original source receipts:

- `flask-0-6-tabstate.md` declares `upload` as the starred default tab.
- `flask-0-7-tabdrawing.md` declares first load as `get_starred_tab() or get_first_visible_tab()`.
- `flask-0-8-favoritecomponent.md` declares `/api/get_starred_tab`, `/api/set_starred_tab`, visible star toggles, one active favorite, and no admin-tab favorite.

Coronatio first load reads this manifest through `/api/favorites` and `/api/get_starred_tab`; the browser opens the manifest starred tab unless an explicit URL hash names a valid visible tab.
