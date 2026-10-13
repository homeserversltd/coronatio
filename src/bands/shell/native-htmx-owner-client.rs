fn shell_native_htmx_owner_client() -> &'static str {
    r####"    const nativeReadGuests = Object.keys(nativePaneContracts);
    nativeReadGuests.forEach(guest => scaffoldRegistry.registerGuest({
      guest,
      root: () => document.querySelector(`[data-pane-panel="${CSS.escape(guest)}"]`),
      authorize: () => sessionLawfulTab(guest)
    }));

    const nativeHtmxOwners = new WeakMap();
    const nativeHtmxTargetOwners = new WeakMap();
    const nativeHtmxTargetIds = new WeakMap();
    const nativeHtmxSwapOwners = new Map();
    const nativeMorphSwapStyle = 'crown-native-morph';
    const nativeHtmxOobMarker = 'crown-htmx-oob-owner:';
    const nativeHtmxOobSelector = '[hx-swap-oob], [data-hx-swap-oob]';
    let nextNativeHtmxTargetId = 0;
    let nextNativeHtmxSwapId = 0;

    function nativeHtmxVisitNodes(root, visit) {
      visit(root);
      if (root.nodeType === Node.ELEMENT_NODE && root.tagName === 'TEMPLATE') {
        nativeHtmxVisitNodes(root.content, visit);
        return;
      }
      Array.from(root.childNodes || []).forEach(node => nativeHtmxVisitNodes(node, visit));
    }

    function nativeHtmxRemoveOutOfBand(fragment) {
      let found = false;
      function removeFrom(parent) {
        let removed = false;
        Array.from(parent.childNodes || []).forEach(node => {
          if (node.nodeType === Node.ELEMENT_NODE && node.matches(nativeHtmxOobSelector)) {
            node.remove();
            removed = true;
            return;
          }
          if (node.nodeType === Node.ELEMENT_NODE && node.tagName === 'TEMPLATE') {
            if (removeFrom(node.content)) {
              node.remove();
              removed = true;
            }
            return;
          }
          if (removeFrom(node)) removed = true;
        });
        return removed;
      }
      found = removeFrom(fragment);
      return found;
    }

    function nativeHtmxTagOutOfBand(html, token) {
      const template = document.createElement('template');
      template.innerHTML = String(html || '');
      let tagged = false;
      nativeHtmxVisitNodes(template.content, node => {
        if (node.nodeType === Node.ELEMENT_NODE && node.matches(nativeHtmxOobSelector)) {
          node.insertBefore(document.createComment(`${nativeHtmxOobMarker}${token}`), node.firstChild);
          tagged = true;
        }
      });
      return tagged ? template.innerHTML : String(html || '');
    }

    function nativeHtmxStripOobMarkers(fragment) {
      const tokens = new Set();
      if (!fragment) return tokens;
      nativeHtmxVisitNodes(fragment, node => {
        if (node.nodeType !== Node.COMMENT_NODE || !node.data.startsWith(nativeHtmxOobMarker)) return;
        const token = node.data.slice(nativeHtmxOobMarker.length);
        if (!token.startsWith(`${nativeMorphSwapStyle}-`)) return;
        tokens.add(token);
        node.remove();
      });
      return tokens;
    }

    function nativeHtmxOwner(event) {
      const xhr = event.detail?.xhr;
      return xhr ? nativeHtmxOwners.get(xhr) : null;
    }

    function nativeHtmxSwapToken(owner) {
      for (const [token, reference] of nativeHtmxSwapOwners) {
        if (!reference.deref()) nativeHtmxSwapOwners.delete(token);
      }
      const nonce = window.crypto?.randomUUID?.() || `${Date.now().toString(36)}-${Math.random().toString(36).slice(2)}`;
      const token = `${nativeMorphSwapStyle}-${++nextNativeHtmxSwapId}-${nonce}`;
      nativeHtmxSwapOwners.set(token, new WeakRef(owner.xhr));
      return token;
    }

    function clearNativeHtmxSwapToken(owner) {
      if (!owner?.swapToken) return;
      nativeHtmxSwapOwners.delete(owner.swapToken);
      owner.swapToken = null;
    }


    function nativeReadTargetKey(target, guest) {
      if (target.id) return `id:${target.id}`;
      for (const [attribute, name] of [
        ['data-service-card', 'service'],
        ['data-linker-readback', 'linker'],
        ['data-upload-tree', 'upload-tree'],
        ['data-portals-grid', 'portals-grid'],
        ['data-scaffold-slot', 'slot'],
        ['data-scaffold-key', 'key']
      ]) {
        if (target.hasAttribute(attribute)) return `${name}:${target.getAttribute(attribute) || guest}`;
      }
      let identity = nativeHtmxTargetIds.get(target);
      if (identity) return `node:${identity}`;
      identity = ++nextNativeHtmxTargetId;
      nativeHtmxTargetIds.set(target, identity);
      return `node:${identity}`;
    }

    function nativeReadStatusHost(owner) {
      if (owner.guest === 'admin') {
        const card = owner.target.closest('[data-service-card]');
        const label = card?.querySelector('.toggle-label');
        if (label) return { container: label.parentElement || card, label };
      }
      if (owner.guest === 'linker') {
        const status = owner.target.querySelector('[data-linker-read-status]') || owner.pane.querySelector('[data-linker-read-status]');
        if (status) return { container: status.parentElement || owner.target, label: status };
      }
      if (owner.guest === 'upload') {
        const status = owner.pane.querySelector('[data-upload-tree-read-status]');
        if (status) return { container: status, label: status };
      }
      if (owner.guest === 'portals') {
        const status = owner.pane.querySelector('[data-portals-loading]');
        if (status) return { container: status, label: status.querySelector('p') || status };
      }
      let status = owner.target.querySelector('[data-native-read-status]');
      if (!status) {
        status = document.createElement('div');
        status.dataset.nativeReadStatus = owner.readKey;
        status.setAttribute('role', 'status');
        owner.target.appendChild(status);
      }
      return { container: status, label: status };
    }

    function displayNativeReadState(owner, message, withRetry = false) {
      if (!nativeHtmxOwnerCurrent(owner) || !owner.target.isConnected) return;
      const host = nativeReadStatusHost(owner);
      if (!host?.container || !host.label) return;
      if (owner.originalStatusText === undefined) owner.originalStatusText = host.label.textContent || '';
      host.label.textContent = message;
      host.label.setAttribute('aria-live', 'polite');
      let retry = host.container.querySelector('[data-native-read-retry]');
      if (withRetry) {
        if (!retry) {
          retry = document.createElement('button');
          retry.type = 'button';
          retry.className = 'scaffold-slot-retry';
          retry.textContent = 'Retry';
          host.container.appendChild(retry);
        }
        retry.dataset.nativeReadRetry = owner.readKey;
        retry.dataset.nativeReadGuest = owner.guest;
      } else {
        retry?.remove();
      }
    }

    function clearNativeReadState(owner, { restoreStatus = true } = {}) {
      if (!owner?.target?.isConnected) return;
      const host = nativeReadStatusHost(owner);
      if (!host?.container || !host.label) return;
      const retry = host.container.querySelector('[data-native-read-retry]');
      if (retry?.dataset.nativeReadRetry === owner.readKey && retry.dataset.nativeReadGuest === owner.guest) retry.remove();
      if (host.container.matches('[data-native-read-status]')
          && host.container.dataset.nativeReadStatus === owner.readKey) host.container.remove();
      else if (restoreStatus && owner.originalStatusText !== undefined) host.label.textContent = owner.originalStatusText;
    }

    function nativeHtmxOwnerCurrent(owner, requireSource = true) {
      if (!owner || owner.retired
          || nativeHtmxOwners.get(owner.xhr) !== owner
          || nativeHtmxTargetOwners.get(owner.target) !== owner
          || owner.binding.readOwners.get(owner.readKey) !== owner
          || !scaffoldRegistry.current(owner.binding)
          || owner.binding.root !== owner.pane
          || !owner.pane.contains(owner.target)
          || !owner.target.isConnected) return false;
      return !requireSource || (owner.source?.isConnected && owner.pane.contains(owner.source));
    }

    function retireNativeHtmxOwner(owner, reason = 'Read completed', { abort = false, keepRetry = false } = {}) {
      if (!owner || owner.retired) return false;
      owner.retired = true;
      clearNativeHtmxSwapToken(owner);
      window.clearTimeout(owner.timer);
      window.clearTimeout(owner.slowTimer);
      owner.timer = null;
      owner.slowTimer = null;
      if (owner.release) {
        const release = owner.release;
        owner.release = null;
        release();
      }
      if (owner.binding.readOwners.get(owner.readKey) === owner) owner.binding.readOwners.delete(owner.readKey);
      if (nativeHtmxTargetOwners.get(owner.target) === owner) nativeHtmxTargetOwners.delete(owner.target);
      if (!keepRetry) scaffoldRegistry.clearRetry(owner.binding, owner.readKey);
      if (abort) {
        try { if (owner.xhr.readyState !== XMLHttpRequest.DONE) owner.xhr.abort(); }
        catch (_) {}
      }
      return true;
    }

    function failNativeHtmxOwner(owner, reason = 'Read unavailable') {
      if (!nativeHtmxOwnerCurrent(owner, false)) return false;
      owner.responseHandled = true;
      displayNativeReadState(owner, reason, false);
      const binding = owner.binding;
      const pane = owner.pane;
      const request = owner.request;
      const guest = owner.guest;
      const path = owner.path;
      const target = owner.target;
      const swap = owner.swap;
      scaffoldRegistry.registerRetry(binding, owner.readKey, () => {
        if (!scaffoldRegistry.current(binding) || binding.root !== pane
            || !target?.isConnected || !pane.contains(target)) return false;
        const retrySource = guest === 'linker' ? target
          : request?.isConnected && pane.contains(request) ? request
          : null;
        if (!(retrySource instanceof Element)) return false;
        return window.htmx?.ajax('GET', path, { source: retrySource, target, swap });
      }, { slots: [], timeoutMs: owner.timeoutMs, slowMs: DEFAULT_NATIVE_SLOW_READ_MS });
      displayNativeReadState(owner, reason, true);
      retireNativeHtmxOwner(owner, reason, { abort: reason === 'Read timed out', keepRetry: true });
      return true;
    }

    function nativeHtmxResponseFailure(owner, html) {
      const source = String(html || '');
      if (/data-(?:cartridge-fault|cartridge-fault-kind|service-card-fault-kind|admin-membrane-refusal)=/.test(source)) {
        return 'Read returned an upstream refusal';
      }
      const template = document.createElement('template');
      template.innerHTML = source;
      if (template.content.querySelector('script')) return 'Read returned forbidden script markup';
      if (template.content.querySelector('[data-cartridge-fault], [data-cartridge-fault-kind], [data-service-card-fault-kind], [data-admin-membrane-refusal]')) {
        return 'Read returned an upstream refusal';
      }
      const primary = template.content.cloneNode(true);
      nativeHtmxRemoveOutOfBand(primary);
      if (primary.childElementCount === 0) return 'Read returned no renderable content';
      if (owner.swapStyle !== 'innerHTML' && owner.swapStyle !== 'outerHTML') return 'Read requested an unsupported native swap';
      if (owner.swapStyle === 'outerHTML') {
        const mainNodes = Array.from(primary.childNodes).filter(node => {
          if (node.nodeType === Node.COMMENT_NODE) return false;
          if (node.nodeType === Node.TEXT_NODE) return Boolean(node.textContent.trim());
          return true;
        });
        if (mainNodes.length !== 1 || mainNodes[0].nodeType !== Node.ELEMENT_NODE
            || mainNodes[0].tagName !== owner.target.tagName || mainNodes[0].id !== owner.target.id) {
          return 'Read returned an incompatible outer fragment';
        }
      }
      return '';
    }

    function nativeMorphExtensionEnabled(pane) {
      if (!pane) return false;
      const names = (pane.getAttribute('hx-ext') || '').split(',').map(name => name.trim()).filter(Boolean);
      if (!names.includes(nativeMorphSwapStyle)) names.push(nativeMorphSwapStyle);
      pane.setAttribute('hx-ext', names.join(','));
      return true;
    }

    function nativeHtmxReadSourceAllowed(guest, request, source, path) {
      if (!(request instanceof Element) && !(source instanceof Element)) return false;
      if (guest === 'admin') {
        return (request?.matches('[data-service-card]') && /^\/admit\/admin\/service\/[a-z0-9-]+$/.test(path))
          || (request?.matches('[data-admin-action-id="view-logs"]') && path === '/admit/admin/action/view-logs');
      }
      if (guest === 'linker') {
        const linkerSource = request?.closest('#linker-fragment') || source?.closest?.('#linker-fragment');
        return Boolean(linkerSource && (path === '/admit/linker' || path.startsWith('/admit/linker?')));
      }
      if (guest === 'upload') {
        return request?.matches('[data-upload-tree-read-status]') && path === '/admit/upload/tree';
      }
      if (guest === 'portals') {
        return request?.matches('[data-portals-grid]') && path === '/api/portals/elements';
      }
      return false;
    }

    function nativeHtmxReadTargetAllowed(guest, request, source, target, pane) {
      if (!(target instanceof Element)) return false;
      if (guest === 'linker') {
        return target.matches('#linker-fragment')
          && ((request instanceof Element && pane.contains(request) && request.closest('#linker-fragment') === target)
            || (source === target && pane.contains(source)));
      }
      if (!(request instanceof Element) || !pane.contains(request)) return false;
      if (guest === 'portals') return request.matches('[data-portals-grid]') && target === request;
      if (guest === 'admin') {
        if (request.matches('[data-service-card]')) return target === request;
        return request.matches('[data-admin-action-id="view-logs"]')
          && target === pane.querySelector('[data-admin-action-result]');
      }
      if (guest === 'upload') {
        return request.matches('[data-upload-tree-read-status]') && target.matches('[data-upload-tree]');
      }
      return false;
    }

    if (window.htmx?.defineExtension) {
      window.htmx.defineExtension(nativeMorphSwapStyle, {
        isInlineSwap(style) { return style === nativeMorphSwapStyle || style.startsWith(`${nativeMorphSwapStyle}-`); },
        handleSwap(style, target, fragment) {
          if (style !== nativeMorphSwapStyle && !style.startsWith(`${nativeMorphSwapStyle}-`)) return false;
          const reference = nativeHtmxSwapOwners.get(style);
          nativeHtmxSwapOwners.delete(style);
          const xhr = reference?.deref();
          const owner = xhr ? nativeHtmxOwners.get(xhr) : null;
          if (owner?.swapToken === style) owner.swapToken = null;
          if (!nativeHtmxOwnerCurrent(owner) || owner.xhr !== xhr) return [];
          const insertedNodes = [];
          const changed = morphLivePane(target, fragment, {
            outerRoot: owner.swapStyle === 'outerHTML',
            preserveOuterRootAttributes: nativePaneContracts[owner.guest]?.preserveOuterRootAttributes,
            insertedNodes
          });
          if (!nativeHtmxOwnerCurrent(owner, false)) return [];
          owner.morphed = true;
          owner.changed = changed;
          insertedNodes.forEach(node => {
            if (!nativeHtmxOwnerCurrent(owner, false) || !node.isConnected || !owner.pane.contains(node)) return;
            try { window.htmx?.process(node); } catch (_) {}
          });
          if (owner.swapStyle === 'outerHTML' && nativeHtmxOwnerCurrent(owner, false)) {
            try { window.htmx?.process(target); } catch (_) {}
          }
          return [];
        }
      });
    }

    const DEFAULT_NATIVE_READ_TIMEOUT_MS = 8000;
    const DEFAULT_NATIVE_SLOW_READ_MS = 750;

    document.body.addEventListener('htmx:beforeRequest', event => {
      const detail = event.detail || {};
      const source = detail.elt instanceof Element ? detail.elt : null;
      const target = detail.target instanceof Element ? detail.target : null;
      if (source?.closest('[data-pane]')) return;
      const request = source?.closest('[hx-get], [hx-post], [hx-put], [hx-patch], [hx-delete]');
      const configuredVerb = String(detail.requestConfig?.verb || detail.requestConfig?.method || '').toLowerCase();
      const verb = configuredVerb || (request?.hasAttribute('hx-get') ? 'get'
        : request?.hasAttribute('hx-post') ? 'post'
        : request?.hasAttribute('hx-put') ? 'put'
        : request?.hasAttribute('hx-patch') ? 'patch'
        : request?.hasAttribute('hx-delete') ? 'delete' : '');
      if (verb !== 'get' || !detail.xhr) return;
      const pane = source?.closest('[data-pane-panel]') || target?.closest('[data-pane-panel]');
      const guest = pane?.dataset.panePanel;
      if (!pane || !guest || !Object.prototype.hasOwnProperty.call(nativePaneContracts, guest)) return;
      const path = String(detail.requestConfig?.path || request?.getAttribute('hx-get') || '');
      if (!nativeHtmxReadSourceAllowed(guest, request, source, path)) return;
      if (!nativeHtmxReadTargetAllowed(guest, request, source, target, pane)) {
        event.preventDefault();
        return;
      }
      const binding = scaffoldRegistry.capture(guest);
      if (!binding || !scaffoldRegistry.current(binding) || binding.root !== pane || !target || !binding.root.contains(target)) {
        event.preventDefault();
        return;
      }
      if (!path) {
        event.preventDefault();
        return;
      }
      const readKey = `htmx:${nativeReadTargetKey(target, guest)}`;
      const prior = binding.readOwners.get(readKey);
      if (prior && !prior.retired) {
        if (!nativeHtmxOwnerCurrent(prior)) {
          retireNativeHtmxOwner(prior, 'Read owner retired', { abort: true });
        } else if (prior.path === path) {
          event.preventDefault();
          return;
        } else {
          retireNativeHtmxOwner(prior, 'Read superseded', { abort: true });
        }
      }
      const requestedTimeout = Number(detail.requestConfig?.timeout || detail.timeout);
      const timeoutMs = Number.isSafeInteger(requestedTimeout) && requestedTimeout > 0
        ? Math.min(requestedTimeout, 30000)
        : DEFAULT_NATIVE_READ_TIMEOUT_MS;
      const swap = String(request?.getAttribute('hx-swap') || detail.requestConfig?.swap || 'innerHTML');
      const owner = {
        guest, pane, source, request, target, binding, xhr: detail.xhr, path, swap,
        swapStyle: swap.trim().split(/\s+/)[0], readKey, timeoutMs,
        retired: false, release: null, timer: null, originalStatusText: undefined,
        retire: reason => retireNativeHtmxOwner(owner, reason, { abort: true })
      };
      binding.readOwners.set(readKey, owner);
      // Keep the weak XHR tombstone so a delayed swap cannot become default HTMX work.
      nativeHtmxOwners.set(owner.xhr, owner);
      nativeHtmxTargetOwners.set(target, owner);
      scaffoldRegistry.clearRetry(binding, readKey);
      clearNativeReadState(owner);

      nativeMorphExtensionEnabled(pane);
      try { owner.xhr.timeout = timeoutMs; }
      catch (_) {}
      owner.release = scaffoldRegistry.onRetire(binding, retirement => {
        retireNativeHtmxOwner(owner, retirement?.reason || 'Guest retired', { abort: true });
      });
      owner.timer = window.setTimeout(() => failNativeHtmxOwner(owner, 'Read timed out'), timeoutMs + 25);
      owner.slowTimer = window.setTimeout(() => displayNativeReadState(owner, `${guest} is still loading.`), Math.min(DEFAULT_NATIVE_SLOW_READ_MS, timeoutMs - 1));
    });

    document.body.addEventListener('htmx:oobBeforeSwap', event => {
      const tokens = nativeHtmxStripOobMarkers(event.detail?.fragment);
      if (!tokens.size) return;
      const token = tokens.size === 1 ? tokens.values().next().value : '';
      const reference = token && nativeHtmxSwapOwners.get(token);
      const xhr = reference?.deref();
      const owner = xhr ? nativeHtmxOwners.get(xhr) : null;
      if (tokens.size !== 1 || !nativeHtmxOwnerCurrent(owner) || owner.xhr !== xhr) {
        event.detail.shouldSwap = false;
      }
    });

    document.body.addEventListener('htmx:beforeSwap', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner) return;
      if (!nativeHtmxOwnerCurrent(owner) || event.detail?.target !== owner.target) {
        event.detail.shouldSwap = false;
        retireNativeHtmxOwner(owner, 'Read owner retired', { abort: true });
        return;
      }
      const status = Number(owner.xhr?.status || 0);
      const declaredSwap = String(event.detail?.swapOverride == null ? owner.swap : event.detail.swapOverride).trim();
      const declaredParts = declaredSwap.split(/\s+/);
      const effectiveOwner = { ...owner, swapStyle: declaredParts[0] };
      const failure = status < 200 || status >= 300
        ? `Read failed (HTTP ${status || 'unavailable'})`
        : nativeHtmxResponseFailure(effectiveOwner, event.detail?.serverResponse || owner.xhr?.responseText || '');
      if (failure) {
        event.detail.shouldSwap = false;
        failNativeHtmxOwner(owner, failure);
        return;
      }
      owner.swapStyle = declaredParts[0];
      owner.swapAuthorized = true;
      owner.swapToken = nativeHtmxSwapToken(owner);
      event.detail.serverResponse = nativeHtmxTagOutOfBand(event.detail.serverResponse || owner.xhr?.responseText || '', owner.swapToken);
      event.detail.swapOverride = [owner.swapToken, ...declaredParts.slice(1)].join(' ');
    });

    document.body.addEventListener('htmx:afterSwap', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || event.detail?.target !== owner.target) return;
      if (!nativeHtmxOwnerCurrent(owner, false)) {
        retireNativeHtmxOwner(owner, 'Read owner retired');
      }
    });

    document.body.addEventListener('htmx:afterSettle', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || event.detail?.target !== owner.target) return;
      if (!nativeHtmxOwnerCurrent(owner, false)) {
        retireNativeHtmxOwner(owner, 'Read owner retired');
        return;
      }
      if (!owner.morphed) {
        failNativeHtmxOwner(owner, 'Read did not produce a native swap');
        return;
      }
      if (owner.guest === 'portals') bindPortalFragmentControls(owner.target);
      if (owner.guest === 'upload') syncUploadTreeSelection();
      applyAdminDomState();
      clearNativeReadState(owner, { restoreStatus: false });
      retireNativeHtmxOwner(owner, 'Read completed');
    });

    document.body.addEventListener('htmx:afterRequest', event => {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || (event.detail?.target && event.detail.target !== owner.target)) return;
      if (event.detail?.successful === true) {
        if (!owner.swapAuthorized) failNativeHtmxOwner(owner, 'Read returned no renderable content');
        return;
      }
      failNativeHtmxOwner(owner, 'Read unavailable');
    });

    function reportNativeHtmxFailure(event) {
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || !scaffoldRegistry.current(owner.binding)) return;
      const reason = event.type === 'htmx:timeout' ? 'Read timed out'
        : event.type === 'htmx:sendError' ? 'Connection unavailable'
        : 'Read unavailable';
      failNativeHtmxOwner(owner, reason);
    }

    ['htmx:timeout', 'htmx:responseError', 'htmx:sendError'].forEach(name => {
      document.body.addEventListener(name, reportNativeHtmxFailure);
    });
"####
}
