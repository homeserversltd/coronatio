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
      if (!scopes.length) return;
      const memory = explainMemory();
      scopes.forEach(scope => { if (!scope.hasAttribute('data-explain-on')) explainApply(scope, memory[scope.dataset.explainScope] === true); });
    }
    explainInitialize(document);
    new MutationObserver(records => records.forEach(record => record.addedNodes.forEach(node => { if (node instanceof Element) explainInitialize(node); }))).observe(document.documentElement, {childList: true, subtree: true});
    const explainDotSelector = 'button[type="button"][data-explain-dot][data-explain-hint]';
    function explainCloseDots(state = '[data-explain-open="true"]') {
      document.querySelectorAll(`${explainDotSelector}${state}`).forEach(dot => {
        delete dot.dataset.explainOpen;
        dot.setAttribute('aria-expanded', 'false');
        dot.setAttribute('data-explain-dismissed', '');
      });
    }
    document.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      const dot = target?.closest(explainDotSelector);
      const shouldOpen = dot && dot.dataset.explainOpen !== 'true';
      explainCloseDots();
      if (dot) {
        event.preventDefault();
        if (shouldOpen) {
          dot.removeAttribute('data-explain-dismissed');
          dot.dataset.explainOpen = 'true';
          dot.setAttribute('aria-expanded', 'true');
        }
        return;
      }
      const button = target?.closest('[data-explain-toggle]');
      const scope = button?.closest('[data-explain-scope]');
      if (!scope) return;
      const on = scope.dataset.explainOn !== 'true';
      explainApply(scope, on);
      try { const memory = explainMemory(); memory[scope.dataset.explainScope] = on; localStorage.setItem(explainKey, JSON.stringify(memory)); } catch (_) { /* Storage is optional. */ }
    });
    document.addEventListener('keydown', event => {
      if (event.key === 'Escape') explainCloseDots(':is([data-explain-open="true"], :hover, :focus-visible)');
    }, true);
    function explainPreviewAgain(event) {
      const dot = event.target instanceof Element ? event.target.closest(explainDotSelector) : null;
      if (dot && !(event.relatedTarget instanceof Node && dot.contains(event.relatedTarget))) {
        dot.removeAttribute('data-explain-dismissed');
      }
    }
    ['pointerover', 'focusin'].forEach(type => {
      document.addEventListener(type, explainPreviewAgain);
    });
"####
}
