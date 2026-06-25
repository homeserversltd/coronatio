fn render_crown_shell() -> String {
    let (background, primary, text, hidden_tab_background) = first_paint_theme();
    r####"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="__PRIMARY__" />
    <meta
      name="description"
      content="HomeServer Admin Interface"
    />
    <meta name="csrf-token" content="" /> <!-- Will be populated by Flask -->
    <link rel="icon" type="image/x-icon" href="/assets/favicon-CHgY6yiq.ico" />
    <link rel="apple-touch-icon" sizes="180x180" href="/assets/apple-touch-icon-CgumePGS.png" />
    <link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon-32x32-C1pw8DCa.png" />
    <link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon-16x16-B9kc5FdD.png" />
    <link rel="icon" type="image/png" sizes="192x192" href="/assets/android-chrome-192x192-BAMQ6pez.png" />
    <link rel="icon" type="image/png" sizes="512x512" href="/assets/android-chrome-512x512-C9kCmYN6.png" />
    <title>HomeServer</title>
    <style>
      :root {
        --background: __BACKGROUND__;
        --primary: __PRIMARY__;
        --primaryHover: __PRIMARY__;
        --text: __TEXT__;
        --hiddenTabBackground: __HIDDEN_TAB_BACKGROUND__;
      }
      html, body, #root {
        background-color: var(--background);
        margin: 0;
        min-height: 100%;
      }
      .app {
        visibility: hidden;
      }
      html.theme-loaded .app {
        visibility: visible;
      }
    </style>
    <script data-coronatio-identical-socket-bridge="home-arpa">
      (() => {
        const targetWs = 'wss://home.arpa';
        const rewriteWebSocket = (value) => {
          const text = String(value || '');
          if (!text.includes('/socket.io')) return value;
          if (text.startsWith('ws://') || text.startsWith('wss://')) return text.replace(/^wss?:\/\/[^/]+/, targetWs);
          return value;
        };
        const NativeWebSocket = window.WebSocket;
        window.WebSocket = function(url, protocols) {
          const rewritten = rewriteWebSocket(url);
          return protocols === undefined ? new NativeWebSocket(rewritten) : new NativeWebSocket(rewritten, protocols);
        };
        window.WebSocket.prototype = NativeWebSocket.prototype;
        Object.assign(window.WebSocket, NativeWebSocket);
      })();
    </script>
    <script type="module" crossorigin src="/assets/index-BRoXzIjg.js?coronatio-identical=20260625"></script>
    <link rel="stylesheet" crossorigin href="/assets/index-Co-PYpJ8.css">
  </head>
  <body>
    <noscript>You need to enable JavaScript to run this app.</noscript>
    <div id="root"></div>
  </body>
</html> "####
        .replace("__BACKGROUND__", background)
        .replace("__PRIMARY__", primary)
        .replace("__TEXT__", text)
        .replace("__HIDDEN_TAB_BACKGROUND__", hidden_tab_background)
}

fn is_safe_tab_id(tab_id: &str) -> bool {
    !tab_id.is_empty()
        && tab_id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

