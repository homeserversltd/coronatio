fn shell_linker_client() -> &'static str {
    r####"    const linkerCursorClient = (() => {
      let currentPath = null;
      let pendingNextPath = null;
      let cursorIndex = 0;

      function entries(root) {
        return [...root.querySelectorAll('[data-linker-entry]')];
      }

      function activePane(root) {
        const pane = root?.closest('[data-pane-panel="linker"]');
        return Boolean(pane?.classList.contains('active') && pane.getAttribute('aria-hidden') !== 'true');
      }

      function cursorAt(root, nextIndex) {
        const rows = entries(root);
        if (!rows.length) return false;
        cursorIndex = (nextIndex + rows.length) % rows.length;
        rows.forEach((row, index) => row.classList.toggle('cursor', index === cursorIndex));
        currentPath = rows[cursorIndex].dataset.linkerPath || null;
        rows[cursorIndex].focus();
        return true;
      }

      function refresh(node) {
        const root = node?.matches?.('#linker-fragment') ? node : node?.querySelector?.('#linker-fragment');
        if (!root || !activePane(root)) return false;
        const rows = entries(root);
        if (!rows.length) {
          currentPath = null;
          pendingNextPath = null;
          cursorIndex = 0;
          return false;
        }
        if (pendingNextPath) {
          const selectedIndex = rows.findIndex(row => row.dataset.linkerPath === pendingNextPath);
          cursorIndex = selectedIndex >= 0 ? selectedIndex + 1 : 0;
          pendingNextPath = null;
        } else if (currentPath) {
          const currentIndex = rows.findIndex(row => row.dataset.linkerPath === currentPath);
          cursorIndex = currentIndex >= 0 ? currentIndex : 0;
        } else {
          cursorIndex = 0;
        }
        return cursorAt(root, cursorIndex);
      }

      function requestPath(root, path) {
        return window.htmx?.ajax('GET', `/admit/linker?path=${encodeURIComponent(path)}`, {
          source: root,
          target: root,
          swap: 'outerHTML'
        });
      }

      document.body.addEventListener('htmx:afterSwap', event => {
        const target = event.detail?.target;
        if (target instanceof Element && (target.matches('#linker-fragment') || target.querySelector('#linker-fragment'))) refresh(target);
      });

      document.body.addEventListener('keydown', event => {
        const target = event.target instanceof Element ? event.target : null;
        const root = target?.closest('#linker-fragment');
        if (!root || !activePane(root) || target.closest('input,textarea,select,button,[contenteditable]')) return;
        const rows = entries(root);
        if (!rows.length) return;
        const currentIndex = Math.max(0, rows.findIndex(row => row.dataset.linkerPath === currentPath));
        const key = event.key.toLowerCase();
        const current = rows[currentIndex];

        if (key === 'j' || key === 'arrowdown') {
          event.preventDefault();
          cursorAt(root, currentIndex + 1);
          return;
        }
        if (key === 'k' || key === 'arrowup') {
          event.preventDefault();
          cursorAt(root, currentIndex - 1);
          return;
        }
        if (key === 'h' || key === 'arrowleft') {
          event.preventDefault();
          const path = root.dataset.linkerPath || '/';
          if (path !== '/') {
            const parent = path.replace(/\/+$/, '').split('/');
            parent.pop();
            requestPath(root, parent.join('/') || '/');
          }
          return;
        }
        if ((key === 'l' || key === 'arrowright' || key === 'enter') && current.dataset.linkerDirectory === 'true') {
          event.preventDefault();
          current.click();
          return;
        }
        if (key === ' ') {
          event.preventDefault();
          pendingNextPath = current.dataset.linkerPath || null;
          current.querySelector('input[type="checkbox"]')?.click();
        }
      });

      return Object.freeze({ refresh });
    })();
    window.linkerCursorClient = linkerCursorClient;
"####
}
