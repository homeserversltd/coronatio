const CROWN_STAGE_SCRIPT_PATH: &str = "/static/crown/stage.js";

const CROWN_STAGE_LOADER_JS: &str = r###"(function () {
  const loaderKey = Symbol.for('coronatio.face-stage.loader.v1');
  if (globalThis[loaderKey]) return;
  globalThis[loaderKey] = true;

  function stageRoot() {
    return document.querySelector('[data-stage-root]');
  }

  function setStageMessage(message, state) {
    const root = stageRoot();
    const result = root?.querySelector('[data-stage-vault-result]');
    if (!result) return;
    result.textContent = message;
    if (state) result.dataset.state = state;
  }

  function unlockSucceeded(result) {
    if (!result || typeof result !== 'object') return false;
    const positive = result.ok === true || result.success === true;
    const negative = result.ok === false || result.success === false || result.converged === false
      || result.rolledBack === true || (typeof result.failure === 'string' && result.failure.length > 0);
    return positive && !negative;
  }

  function unlockMessage(result) {
    const detail = [result?.failure, result?.error, result?.firstMissingSignal, result?.message]
      .find(value => typeof value === 'string' && value.length > 0);
    return detail ? `Vault unlock failed: ${detail}` : 'Vault unlock failed.';
  }

  function loadFaceChrome() {
    if (typeof window.initializeCoronatioFace === 'function') {
      window.initializeCoronatioFace();
      return Promise.resolve();
    }
    return new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = '/static/crown/chrome.js';
      script.async = false;
      script.dataset.crownChrome = 'stage-loader';
      script.onload = () => {
        if (typeof window.initializeCoronatioFace !== 'function') {
          reject(new Error('face-initializer-unavailable'));
          return;
        }
        window.initializeCoronatioFace();
        resolve();
      };
      script.onerror = () => reject(new Error('face-chrome-unavailable'));
      document.head.appendChild(script);
    });
  }

  async function fetchAndInsertFace() {
    const root = stageRoot();
    if (!root) return;
    const response = await fetch('/stage', { cache: 'no-store' });
    if (!response.ok) throw new Error(`face-fragment-http-${response.status}`);
    const markup = await response.text();
    if (markup.includes('data-face-stage="vault"')) {
      setStageMessage('Vault remains locked. Verify its status and try again.', 'error');
      return;
    }
    root.innerHTML = markup;
    window.htmx.process(root);
    await loadFaceChrome();
  }

  document.addEventListener('input', event => {
    const input = event.target instanceof Element ? event.target.closest('[data-stage-vault-password]') : null;
    if (!input) return;
    const form = input.closest('[data-stage-vault-form]');
    const password = form?.querySelector('[data-stage-vault-password]');
    const submit = form?.querySelector('[data-stage-vault-submit]');
    if (!submit) return;
    submit.disabled = !password?.value || form.dataset.inFlight === 'true';
  });

  document.addEventListener('submit', async event => {
    const form = event.target instanceof Element ? event.target.closest('[data-stage-vault-form]') : null;
    if (!form) return;
    event.preventDefault();
    if (form.dataset.inFlight === 'true') return;
    const root = stageRoot();
    const password = form.querySelector('[data-stage-vault-password]');
    const submit = form.querySelector('[data-stage-vault-submit]');
    if (!root || !password || !submit || !password.value) return;

    const payload = { password: password.value };
    const requestBody = JSON.stringify(payload);
    password.value = '';
    form.dataset.inFlight = 'true';
    submit.disabled = true;
    setStageMessage('Unlocking vault…', 'pending');
    try {
      const response = await fetch('/api/v1/storage/vault/unlock', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        cache: 'no-store',
        body: requestBody
      });
      const result = await response.json().catch(() => null);
      if (!response.ok || !unlockSucceeded(result)) {
        setStageMessage(unlockMessage(result), 'error');
        return;
      }
      setStageMessage('Vault unlocked. Opening Coronatio…', 'success');
      await fetchAndInsertFace();
    } catch (_) {
      setStageMessage('Vault unlock could not be completed.', 'error');
    } finally {
      password.value = '';
      form.dataset.inFlight = 'false';
      if (submit.isConnected) submit.disabled = true;
    }
  });
})();"###;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaceStage {
    Face,
    Vault,
}

fn face_stage() -> FaceStage {
    let readback = caduceus_http("GET", "/api/v1/access/gate");
    if !readback.ok
        || readback
            .body
            .get("vault")
            .and_then(|vault| vault.get("unlock_required"))
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return FaceStage::Face;
    }
    FaceStage::Vault
}

async fn face_stage_async() -> FaceStage {
    tokio::task::spawn_blocking(face_stage)
        .await
        .unwrap_or(FaceStage::Face)
}

async fn face_stage_fallback(method: Method, uri: Uri) -> Response {
    if matches!(face_stage_async().await, FaceStage::Vault) {
        return face_stage_locked_response();
    }
    let normalized = uri.path().to_string();
    if normalized.starts_with("/api/") {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "schema": "coronatio.api.error.v1",
                "error": "api route not found",
                "path": normalized,
                "method": method.as_str(),
                "policy": "API clients receive JSON 404; website endpoints must be explicitly registered Rust routes"
            })),
        )
            .into_response();
    }
    match tokio::task::spawn_blocking(render_crown_shell).await {
        Ok(markup) => face_stage_html_response(markup),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "code": "face-stage-render-unavailable" })),
        )
            .into_response(),
    }
}

fn face_stage_exempts_gate(method: &Method, path: &str) -> bool {
    path == "/static"
        || path.starts_with("/static/")
        || (*method == Method::GET && matches!(path, "/" | "/health" | "/stage"))
        || (*method == Method::POST && path == "/api/v1/storage/vault/unlock")
}

async fn face_stage_route_gate(request: axum::http::Request<Body>, next: axum::middleware::Next) -> Response {
    if face_stage_exempts_gate(request.method(), request.uri().path()) {
        return next.run(request).await;
    }
    if matches!(face_stage_async().await, FaceStage::Vault) {
        return face_stage_locked_response();
    }
    next.run(request).await
}

fn face_stage_locked_response() -> Response {
    (
        StatusCode::LOCKED,
        Json(serde_json::json!({ "code": "face-stage-vault" })),
    )
        .into_response()
}

fn face_stage_html_response(markup: String) -> Response {
    let mut response = Html(markup).into_response();
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

async fn crown_stage_route() -> Response {
    let rendered = tokio::task::spawn_blocking(|| match face_stage() {
        FaceStage::Vault => render_stage_fragment(),
        FaceStage::Face => render_crown_face_fragment(),
    })
    .await;
    match rendered {
        Ok(markup) => face_stage_html_response(markup),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "code": "face-stage-render-unavailable" })),
        )
            .into_response(),
    }
}

async fn crown_stage_script_route() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        CROWN_STAGE_LOADER_JS,
    )
}

fn render_stage_fragment() -> String {
    let mut markup = String::from(
        r#"<section class="pane active stage-vault" data-face-stage="vault" aria-labelledby="face-stage-vault-title">
  <div class="vault-unlock-panel" aria-label="Vault unlock">
    <h1 id="face-stage-vault-title">Unlock encrypted storage</h1>
    <p>Unlock the vault to continue to HOMESERVER.</p>
    <form data-stage-vault-form autocomplete="off" method="post">
      <label for="face-stage-vault-password">Vault password</label>
      <input id="face-stage-vault-password" class="ui-input ui-input--medium" type="password" autocomplete="current-password" data-stage-vault-password>
"#,
    );
    markup.push_str(
        r#"      <button type="submit" class="action-button" data-stage-vault-submit disabled>Unlock Vault</button>
    </form>
    <p class="vault-unlock-result" data-stage-vault-result aria-live="polite"></p>
  </div>
</section>"#,
    );
    markup
}

fn render_stage_document() -> String {
    let mut document = shell_document_1().to_string();
    // The locked stage must paint before the face's theme catalog is available.
    // Values are the dark theme in theme/catalog.json; face theme CSS can replace them.
    document.push_str(r#"
    :root {
      --theme-background: #0A0A0A;
      --theme-bg-primary: #0A0A0A;
      --theme-bg-tertiary: #1E293B;
      --theme-bg-hover: #6B7280;
      --theme-text: #E0E0E0;
      --theme-text-secondary: #9CA3AF;
      --theme-text-tertiary: #A0AEC0;
      --theme-border: #1E293B;
      --theme-primary: #323840;
      --theme-primaryHover: #6B7280;
      --theme-secondary: #9CA3AF;
      --theme-accent: #A78BFA;
      --theme-success: #10B981;
      --theme-error: #df0a3f;
    }
    .pane.active.stage-vault { display: block; opacity: 1; transform: none; }
"#);
    document.push_str(&shell_ux_css());
    document.push_str(
        r#"</style>
  <link rel="stylesheet" href="/static/vendor/fontawesome/css/all.min.css" data-icon-substrate="fontawesome-free-5.15.4">
  <script src="/static/vendor/chart.umd.min.js" data-chart-dependency="chartjs-4.4.0"></script>
  <script src="/static/vendor/chartjs-plugin-datalabels.min.js" data-chart-dependency="chartjs-plugin-datalabels-2.2.0"></script>
</head>
<body>
  <div class="stage-app" data-stage-root data-admin-mode="false">
"#,
    );
    document.push_str(&render_stage_fragment());
    document.push_str(
        &format!(
            "\n  </div>\n  <script defer src=\"{}\" data-htmx-organ=\"2.0.10\"></script>\n  <script defer src=\"{}\" data-coronatio-stage-loader></script>\n</body>\n</html>",
            CROWN_HTMX_SCRIPT_PATH, CROWN_STAGE_SCRIPT_PATH
        ),
    );
    document
}

fn render_crown_face_fragment() -> String {
    let shell = render_crown_shell();
    let Some((_, body)) = shell.split_once("<body>") else {
        return String::new();
    };
    let Some((body, _)) = body.rsplit_once("</body>") else {
        return String::new();
    };
    body.replace(
        &format!(
            "<script defer src=\"{}\" data-htmx-organ=\"2.0.10\"></script>",
            CROWN_HTMX_SCRIPT_PATH
        ),
        "",
    )
    .replace(
        &format!(
            "<script defer src=\"{}\" data-crown-chrome=\"og-htmx\"></script>",
            CROWN_CHROME_SCRIPT_PATH
        ),
        "",
    )
    .trim()
    .to_string()
}
