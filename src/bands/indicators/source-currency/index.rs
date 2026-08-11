pub(crate) fn source_currency_manifest() -> IndicatorManifest {
    IndicatorManifest {
        id: "source-currency",
        topic_id: "source.currency",
        order: 60,
        title: "Coronatio Currency",
        icon_id: "git-branch",
        initial_state: "unknown",
        admin_interactive: true,
        render_indicator: render_source_currency_indicator,
        render_modal: render_source_currency_modal,
        collector: Some(source_currency_collector),
    }
}

fn render_source_currency_indicator(ctx: IndicatorRenderContext) -> String {
    let _ = ctx.session;
    r##"<button type="button" class="indicator unknown source-currency-indicator" data-indicator="source-currency" data-modal-kind="source-currency" data-modal-title="Coronatio Currency" data-source-currency-status="unknown" aria-label="Source Currency Unknown / unavailable" title="Source currency: Unknown / unavailable"><svg class="indicator-icon" data-packed-icon="git-branch" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 3a3 3 0 1 0 2 5.83V11a3 3 0 0 0 3 3h2a1 1 0 0 1 1 1v1.17A3 3 0 1 0 16 16.17V15a3 3 0 0 0-3-3h-2a1 1 0 0 1-1-1V8.83A3 3 0 0 0 6 3z"/></svg></button>"##.to_string()
}
fn render_source_currency_modal(ctx: IndicatorRenderContext) -> String {
    let _ = ctx.session;
    r##"<div class="source-currency-modal" data-modal-kind-body="source-currency"><p class="status-text unknown" data-source-currency-label>Unknown / unavailable</p><p data-source-currency-relation>The source-currency relation is unavailable.</p><dl><dt>Build SHA</dt><dd class="readout" data-source-currency-build-sha>Unavailable</dd><dt>Origin main SHA</dt><dd class="readout" data-source-currency-origin-main-sha>Unavailable</dd></dl>${indicatorAdminSection(`<div class="source-currency-actions"><button type="button" class="ui-button ui-button--primary" data-source-currency-update hidden disabled>Update now</button></div>`)}</div>"##.to_string()
}
fn source_currency_collector(session: Session) -> Result<serde_json::Value, String> {
    collect_indicator_topic("source.currency", session)
}
