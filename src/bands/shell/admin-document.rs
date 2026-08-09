// Attendance is the sole content-admission event for this document. This fragment
// is never part of a fresh crown shell.
fn shell_admin_document_patch() -> &'static str {
    r####"      <section class="pane" id="pane-admin" data-pane-panel="admin" data-view-panel="admin" data-admin-document-patch="true" role="tabpanel" aria-label="Admin">

        <div class="admin-tablet" data-admin-only="true" data-admin-viewport="admin">
          <section class="mb-6" style="margin-bottom: 0.5rem">
            <div class="system-controls-container" aria-label="System controls">
              <div class="system-controls" data-admin-action-strip="wrapped-row" data-admin-action-strip-count="7">
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-hard-drive-test-open><span class="admin-action-icon">▣</span><span>Hard Drive Test</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="update" hx-post="/admit/admin/action/update" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">⬇</span><span>Update</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="restart" hx-post="/admit/admin/action/restart" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Restart HOMESERVER now? Active services will be interrupted."><span class="admin-action-icon">⟳</span><span>Restart</span></button>
                <button type="button" class="ui-button ui-button--danger ui-button--medium system-controls-btn" data-admin-action-id="shutdown" hx-post="/admit/admin/action/shutdown" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Shut down HOMESERVER now? Active services will stop."><span class="admin-action-icon">⏻</span><span>Shutdown</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-admin-action-id="restart-website" hx-post="/admit/admin/action/restart-website" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this" hx-confirm="Restart Website now? The web service will reset and this page will reconnect."><span class="admin-action-icon">↻</span><span>Restart Website</span></button>
                <button type="button" class="ui-button ui-button--secondary ui-button--medium system-controls-btn" data-admin-action-id="view-logs" hx-get="/admit/admin/action/view-logs" hx-target="[data-admin-action-result]" hx-swap="innerHTML" hx-disabled-elt="this"><span class="admin-action-icon">▤</span><span>View Logs</span></button>
                <button type="button" class="ui-button ui-button--primary ui-button--medium system-controls-btn" data-hestia-certificate-open><span class="admin-action-icon">◆</span><span>Install Certificate</span></button>
              </div>
              <div class="system-service-controls" data-admin-service-controls data-state-source="/api/services/data">
                <div class="ssh-controls">
                  <div class="ssh-control" data-service-card="ssh-password-authentication" hx-get="/admit/admin/service/ssh-password-authentication" hx-trigger="load" hx-target="this" hx-swap="innerHTML"><div class="ssh-status" data-admin-toggle-card="ssh-password-authentication"><h3>SSH Password Authentication</h3><div class="ssh-toggle"><span class="toggle-label">Loading…</span></div></div></div>
                  <div class="ssh-control" data-service-card="ssh-service" hx-get="/admit/admin/service/ssh-service" hx-trigger="load" hx-target="this" hx-swap="innerHTML"><div class="ssh-status" data-admin-toggle-card="ssh-service"><h3>SSH Service</h3><div class="ssh-toggle"><span class="toggle-label">Loading…</span></div></div></div>
                </div>
                <div class="samba-control" data-service-card="samba-file-sharing" hx-get="/admit/admin/service/samba-file-sharing" hx-trigger="load" hx-target="this" hx-swap="innerHTML"><div class="samba-status" data-admin-toggle-card="samba-file-sharing"><h3>Samba File Sharing</h3><div class="samba-toggle"><span class="toggle-label">Loading…</span></div></div></div>
              </div>
              <div class="update-status-container" data-admin-action-result data-og-affordance="toast-mapped-to-result-strip" aria-live="polite"></div>
            </div>
          </section>

          <section class="mb-6" style="margin-bottom: 0.5rem">
            <div class="key-manager">
              <h3><span class="admin-action-icon">⚿</span> Key Management</h3>
              <div class="key-manager-content">
                <div class="key-manager-left">
                  <div class="security-status">
                    <div class="status-item"><span class="status-icon secure">🛡</span><div class="status-details"><p>This is the key to your vault. When you boot your HOMESERVER and visit home.arpa, this is what unlocks your encrypted storage system - just like unlocking your smartphone. Your /vault partition contains the sensitive keys stored on the device. Unlock the vault and everything HOMESERVER specifically stores is accessible. This is the device's master key.<button type="button" class="action-button info-button" data-manager-open="key-guide" aria-label="View Full Guide &amp; Critical Warnings"><span>ⓘ</span> View Full Guide &amp; Critical Warnings</button></p></div></div>
                  </div>
                </div>
                <div class="key-manager-right"><div class="key-actions">
                  <button type="button" class="action-button create-button" data-manager-open="create-key"><span>+ Create New Key</span></button>
                  <button type="button" class="action-button update-button" data-manager-open="update-key"><span>⟳ Update Key on Drive</span></button>
                  <button type="button" class="action-button admin-password-button" data-manager-open="admin-password"><span>🔒 Admin Password</span></button>
                </div></div>
              </div>
            </div>
          </section>

          <section class="mb-6" style="margin-bottom: 0.5rem">
            <div class="disk-manager">
              <div class="disk-manager-container">
                <div class="disk-column"><h4>Available Devices</h4><div class="disk-list" data-disk-census-readback="/api/v1/disk/census"><div class="disk-item empty"><span class="disk-icon">▣</span><div class="disk-info"><div class="disk-name">Reading available devices…</div></div></div></div></div>
                <div class="disk-column"><h4>Mount Destinations</h4><div class="disk-list" data-admin-mounts-readback="/api/services/data">__ADMIN_MOUNT_DESTINATIONS__</div></div>
              </div>
              <div class="disk-actions" data-disk-actions-state="no-selection" aria-live="polite">
                <p class="manager-action-reading" data-disk-action-reading>Select a device or mount destination to inspect its available actions.</p>
                <button type="button" class="action-button format" data-disk-action="format" disabled title="Select an eligible device first">Format</button>
                <button type="button" class="action-button encrypt" data-disk-action="encrypt" disabled title="Select an eligible device first">Encrypt</button>
                <button type="button" class="action-button assign-primary" data-disk-action="assign-primary" disabled title="Select an eligible device first">Assign as primary NAS</button>
                <button type="button" class="action-button assign-backup" data-disk-action="assign-backup" disabled title="Select an eligible device first">Assign as NAS Backup</button>
                <button type="button" class="action-button unassign-nas" data-disk-action="unassign" disabled title="Select an eligible device first">Unassign drive</button>
                <button type="button" class="action-button import-nas" data-disk-action="import" disabled title="Select an eligible device first">Import to NAS</button>
                <button type="button" class="action-button permissions" data-disk-action="setup-nas" data-disk-action-live title="Setup NAS">Setup NAS</button>
                <button type="button" class="action-button unlock" data-disk-action="unlock" disabled title="Select an eligible device first">Unlock</button>
                <button type="button" class="action-button mount" data-disk-action="mount" disabled title="Select an eligible device first">Mount</button>
                <button type="button" class="action-button unmount" data-disk-action="unmount" disabled title="Select an eligible device first">Unmount</button>
                <button type="button" class="action-button sync" data-disk-action="sync" data-disk-action-live title="Sync Now">Sync Now</button>
                <button type="button" class="action-button auto-sync" data-disk-action="auto-sync" data-disk-action-live title="Auto Sync">Auto Sync</button>
              </div>
            </div>
          </section>
        </div>
      </section>
"####
}
