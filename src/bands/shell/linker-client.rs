fn shell_linker_client() -> &'static str {
    r####"    const linkerCursorClient = (() => {
      let currentPath = null;
      let pendingNextPath = null;
      let cursorIndex = 0;
      let pendingEditor = null;
      let checkboxSpace = false;

      function editors(root) {
        return [...root.querySelectorAll('input:not([type="checkbox"]),textarea,[contenteditable]')];
      }

      function sameEditor(a, b) {
        return a.tagName === b.tagName && a.id === b.id &&
          a.getAttribute('name') === b.getAttribute('name') &&
          a.getAttribute('type') === b.getAttribute('type') &&
          a.closest('[data-linker-entry]')?.dataset.linkerPath === b.closest('[data-linker-entry]')?.dataset.linkerPath;
      }

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
        if (!root || !root.isConnected || !activePane(root)) return false;
        const focused = document.activeElement;
        if (focused instanceof Element && root.contains(focused) && editors(root).some(editor => editor === focused || editor.contains(focused))) return false;
        const rows = entries(root);
        if (!rows.length) {
          currentPath = null;
          pendingNextPath = null;
          cursorIndex = 0;
          const drop = root.querySelector('[data-linker-act="drop"]');
          if (drop) drop.focus();
          return Boolean(drop);
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

      document.body.addEventListener('htmx:beforeSwap', event => {
        const root = event.detail?.target;
        if (!(root instanceof Element) || !root.matches('#linker-fragment')) return;
        const focused = document.activeElement;
        pendingEditor = null;
        if (!activePane(root) || !(focused instanceof Element) || !root.contains(focused) || !editors(root).includes(focused)) return;
        pendingEditor = { element: focused, index: editors(root).filter(editor => sameEditor(editor, focused)).indexOf(focused) };
      });

      document.body.addEventListener('htmx:afterSwap', event => {
        const target = event.target;
        if (!(target instanceof Element) || !(target.matches('#linker-fragment') || target.querySelector('#linker-fragment'))) return;
        const root = target.matches('#linker-fragment') ? target : target.querySelector('#linker-fragment');
        if (pendingEditor && root.isConnected && activePane(root)) {
          const replacement = editors(root).filter(editor => sameEditor(editor, pendingEditor.element))[pendingEditor.index];
          if (replacement && !root.contains(document.activeElement)) replacement.focus();
        }
        pendingEditor = null;
        if (root.isConnected && activePane(root) && !editors(root).includes(document.activeElement)) {
          root.querySelector('[data-linker-naming] input[type="text"], [data-linker-naming] input:not([type])')?.focus();
        }
        refresh(target);
      });

      document.body.addEventListener('focusin', event => {
        const target = event.target instanceof Element ? event.target : null;
        const row = target?.closest('[data-linker-entry]');
        const root = row?.closest('#linker-fragment');
        if (!root || !activePane(root)) return;
        const rows = entries(root);
        cursorIndex = rows.indexOf(row);
        if (cursorIndex < 0) return;
        currentPath = row.dataset.linkerPath || null;
        rows.forEach((item, index) => item.classList.toggle('cursor', index === cursorIndex));
      });

      document.addEventListener('keyup', event => {
        if (event.key === ' ' && checkboxSpace) {
          event.preventDefault();
          checkboxSpace = false;
        }
      }, true);

      document.body.addEventListener('keydown', event => {
        const target = event.target instanceof Element ? event.target : null;
        const root = target?.closest('#linker-fragment');
        if (!root || !activePane(root) || event.ctrlKey || event.altKey || event.metaKey) return;
        const key = event.key.toLowerCase();
        const act = name => root.querySelector(`[data-linker-act="${name}"]`);
        if (key === 'escape' || key === 'esc') {
          const cancel = act('cancel');
          if (cancel) { event.preventDefault(); cancel.click(); }
          return;
        }
        if (target.closest('input:not([type="checkbox"]),textarea,select,[contenteditable]')) return;
        const rows = entries(root);
        const currentIndex = Math.max(0, rows.findIndex(row => row.dataset.linkerPath === currentPath));
        const current = rows[currentIndex];
        const review = act('confirm');
        const action = key === 'd' ? 'drop' : key === 'delete' ? 'delete' :
          key === 'h' || key === 'arrowleft' ? 'up' :
          key === 'n' ? (review ? 'cancel' : 'new') : key === 'y' && review ? 'confirm' : null;
        if (action) {
          const control = act(action);
          if (control) { event.preventDefault(); control.click(); }
          return;
        }
        if (key === 'r') {
          const rename = act('rename');
          if (rename && current?.dataset.linkerDirectory === 'true' && current.dataset.linkerPath) {
            event.preventDefault();
            window.htmx?.ajax('POST', '/admit/linker/rename', {
              source: rename, target: root, swap: 'outerHTML', values: { path: current.dataset.linkerPath }
            });
          }
          return;
        }
        if (target !== root && !target.matches('[data-linker-entry], [data-linker-entry] input[type="checkbox"]')) return;
        if (!current) return;
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
        if ((key === 'l' || key === 'arrowright' || key === 'enter') && current.dataset.linkerDirectory === 'true') {
          const name = current.querySelector('.ui-listing-row__name');
          if (name) { event.preventDefault(); name.click(); }
          return;
        }
        if (key === ' ') {
          event.preventDefault();
          if (event.repeat) return;
          if (target.matches('input[type="checkbox"]')) checkboxSpace = true;
          pendingNextPath = current.dataset.linkerPath || null;
          current.querySelector('input[type="checkbox"]')?.click();
        }
      });

      return Object.freeze({ refresh });
    })();
    window.linkerCursorClient = linkerCursorClient;
"####
}
