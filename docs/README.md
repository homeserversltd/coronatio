# Coronatio documentation

Coronatio is a Rust web interface that gathers a self-hosted appliance's controls and local web services into one tabbed screen.

## Use Coronatio

- [Loadable cartridges](cartridges.md) — add, remove, and troubleshoot web services loaded as tabs.
- [Architecture](architecture.md) — understand the shell, native tabs, runtime cartridges, configuration, privileged actions, and live updates.

## Contribute

- [Theme tokens](development/theme-tokens.md) — extend the visual token catalog without creating a second theme system.

## Engineering notes

- [Page-load fetch flood](engineering/incidents/pageload-fetch-flood.md) — how two refresh loops created a request storm and how the regression is tested.
