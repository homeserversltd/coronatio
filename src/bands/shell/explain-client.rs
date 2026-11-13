fn shell_explain_client() -> &'static str {
    r####"    // A scope may arrive with an admin role skeleton after face initialization.
    const explainKey = 'coronatio.explain.v1';
    function explainMemory() {
      try { const value = JSON.parse(localStorage.getItem(explainKey) || '{}'); return value && typeof value === 'object' && !Array.isArray(value) ? value : {}; }
      catch (_) { return {}; }
    }
    function explainApply(scope, on) {
      scope.dataset.explainOn = on ? 'true' : 'false';
      scope.querySelectorAll('[data-explain-toggle]').forEach(button => button.setAttribute('aria-pressed', String(on)));
    }
    function explainInitialize(root) {
      const scopes = [];
      if (root instanceof Element && root.matches('[data-explain-scope]')) scopes.push(root);
      root.querySelectorAll?.('[data-explain-scope]').forEach(scope => scopes.push(scope));
      const memory = explainMemory();
      scopes.forEach(scope => { if (!scope.hasAttribute('data-explain-on')) explainApply(scope, memory[scope.dataset.explainScope] === true); });
    }
    explainInitialize(document);
    new MutationObserver(records => records.forEach(record => record.addedNodes.forEach(node => { if (node instanceof Element) explainInitialize(node); }))).observe(document.documentElement, {childList: true, subtree: true});
    document.addEventListener('click', event => {
      const button = event.target instanceof Element ? event.target.closest('[data-explain-toggle]') : null;
      const scope = button?.closest('[data-explain-scope]');
      if (!scope) return;
      const on = scope.dataset.explainOn !== 'true';
      explainApply(scope, on);
      try { const memory = explainMemory(); memory[scope.dataset.explainScope] = on; localStorage.setItem(explainKey, JSON.stringify(memory)); } catch (_) { /* Storage is optional. */ }
    });
"####
}
