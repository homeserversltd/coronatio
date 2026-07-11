pub(crate) fn tailscale_manifest() -> IndicatorManifest {
    IndicatorManifest { id: "tailscale", topic_id: "tailscale.status", order: 10, title: "Tailscale Status", icon_id: "network-wired", initial_state: "loading", admin_interactive: true, render_indicator: render_tailscale_indicator, render_modal: render_tailscale_modal, collector: Some(tailscale_collector) }
}

fn render_tailscale_indicator(ctx: IndicatorRenderContext) -> String { let _ = ctx.session; r##"<button type="button" class="indicator ok tailscale-indicator" data-indicator="tailscale" data-modal-kind="tailscale" data-modal-title="Tailscale Status" aria-label="Tailscale Status" title="Tailscale Status"><svg class="indicator-icon" data-packed-icon="network-wired" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 5h16v8H4V5zm2 2v4h12V7H6zm5 8h2v2h7v2h-7v2h-2v-2H4v-2h7v-2z"/></svg></button>"##.to_string() }
fn render_tailscale_modal(ctx: IndicatorRenderContext) -> String { let _ = ctx.session; r##"<div class="tailscale-status-modal" data-modal-kind-body="tailscale" data-flask-react-quarry="TailscaleIndicator">
        <div class="status-section"><p class="status-text loading" data-modal-status data-route-read="/api/status/tailscale"><span data-spinner>⟳</span> LOADING...</p>
          <div class="login-required-section" data-tailscale-login-section hidden>
            <div class="login-message"><strong>Authentication Required</strong><p>Tailscale service is running but needs authentication. Click the link below to complete login:</p></div>
            <div class="login-url-container"><a href="#" target="_blank" rel="noopener noreferrer" class="login-url-link" data-tailscale-login-url></a><button class="copy-url-button" data-copy-login-url title="Copy URL to clipboard">Copy URL</button></div>
            <div class="login-instructions"><p><strong>Instructions:</strong></p><ol><li>Click the authentication link above (opens in new tab)</li><li>Sign in to your Tailscale account</li><li>Authorize this device</li><li>Return here - the status should update automatically</li></ol></div>
          </div>
        </div>
        ${indicatorAdminSection(`<div class="controls-section"><div class="connection-buttons"><button class="primary-button" data-modal-fetch="/api/status/tailscale/connect" data-method="POST" data-operation-label="Connecting...">Connect</button><button class="primary-button" data-modal-fetch="/api/status/tailscale/disconnect" data-method="POST" data-operation-label="Disconnecting...">Disconnect</button></div><div class="service-controls"><button class="primary-button" data-modal-fetch="/api/status/tailscale/enable" data-method="POST" data-operation-label="Enabling...">Enable Service</button><button class="primary-button" data-modal-fetch="/api/status/tailscale/disable" data-method="POST" data-operation-label="Disabling...">Disable Service</button></div></div>
        <div class="config-section"><div class="current-tailnet"><span class="label">Current Tailnet:</span><span class="value" data-route-read="/api/status/tailscale/config">Loading...</span></div><div class="config-form"><input data-tailnet-input placeholder="Enter Tailnet name"><button class="primary-button" data-modal-fetch="/api/status/tailscale/update-tailnet" data-method="POST" data-operation-label="Updating...">Update Tailnet</button><div class="tooltip-text">Unique name used for DNS entries and TLS certificates.
          You can find this name on the DNS page of your tailscale dashboard.
          This change will reboot the website and tailscale service.
          Please wait and refresh the page after submitting changes.

          Note: HOMESERVER will automatically regenerate the HTTPS self-signed
          certificate to reference your new tailnet. If you previously
          installed the certificate on any device, open the site in a
          private/incognito window and re-download the certificate before
          returning to normal browsing. Until the new certificate is
          installed, browsers may report a certificate name mismatch for both
          local and remote access.</div></div></div>
        <div class="authkey-section"><div class="authkey-alternative"><p class="alternative-text"><strong>Alternative:</strong> If the login link isn't working, you can use an auth key instead.</p></div><div class="authkey-form"><input class="authkey-input" data-authkey-input placeholder="Enter your tskey-auth-... or tskey-client-... key"><button class="primary-button" data-modal-fetch="/api/status/tailscale/authkey" data-method="POST" data-operation-label="Authenticating...">Authenticate</button></div><div class="authkey-help"><p>Get your auth key from the Tailscale admin console under Settings → Keys.</p></div></div>`)}<pre class="readout action-output" data-modal-output></pre>
      </div>"##.to_string() }
fn tailscale_collector(session: Session) -> Result<serde_json::Value, String> { collect_indicator_topic("tailscale.status", session) }
