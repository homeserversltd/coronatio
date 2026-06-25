# Coronatio favorite/starred-tab projection

Coronatio is a one-to-one port of the old Flask/React HOMESERVER surface. The favorite/starred-tab system is governed first by the central HomeServer config file, `homeserver.json`, before any Coronatio-local fallback or firmware default.

Original source receipts:

- `flask-0-6-tabstate.md` declares the archived starred/default tab state.
- `flask-0-7-tabdrawing.md` declares first load as `get_starred_tab() or get_first_visible_tab()`.
- `flask-0-8-favoritecomponent.md` declares `/api/get_starred_tab`, `/api/set_starred_tab`, visible star toggles, one active favorite, and no admin-tab favorite.

Coronatio projects that law from `homeserver.json`: `tabs.starred` selects the default route, while `tabs.<tab>.config` and `tabs.<tab>.visibility` define visible/admin tab state. `/api/favorites` and `/api/get_starred_tab` expose that one HomeServer config truth; Caduceus/Harmonia own future writes back into the central config membrane.
