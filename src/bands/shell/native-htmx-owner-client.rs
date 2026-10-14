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
    const nativeUploadTreeGeneratedLabels = new WeakMap();
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

    function nativeUploadTreeSlot(pane) {
      if (!(pane instanceof Element) || pane.dataset.panePanel !== 'upload') return null;
      const slots = pane.querySelectorAll('[data-upload-tree][data-scaffold-slot="upload-tree"]');
      return slots.length === 1 ? slots[0] : null;
    }

    function nativeUploadTreePathAllowed(path) {
      return path === '/admit/upload/tree'
        || (path.startsWith('/admit/upload/tree?') && !path.includes('#'));
    }

    function nativeUploadTreeSourceAllowed(request, source, pane, path) {
      const tree = nativeUploadTreeSlot(pane);
      if (!tree || !nativeUploadTreePathAllowed(path)) return false;
      if (request?.matches('[data-upload-tree-read-status]')) {
        return path === '/admit/upload/tree' && tree.contains(request)
          && source instanceof Element && request.contains(source);
      }
      if (source === tree) return true;
      return path.startsWith('/admit/upload/tree?')
        && request instanceof Element && tree.contains(request)
        && source instanceof Element && tree.contains(source);
    }

    function nativeUploadTreeOwnerSlot(owner) {
      const tree = nativeUploadTreeSlot(owner?.pane);
      return owner?.guest === 'upload' && owner.target === tree && owner.binding?.root === owner.pane ? tree : null;
    }

    function setNativeUploadTreePending(binding, target) {
      const key = target.dataset.scaffoldPlaceholder || target.dataset.scaffoldKey || target.dataset.scaffoldSlot || 'content';
      const generated = `Loading ${key}`;
      if (target.dataset.scaffoldState !== 'ready'
          && (!target.hasAttribute('aria-label')
            || (target.dataset.scaffoldState === 'pending'
              && target.dataset.scaffoldPending === 'true'
              && target.getAttribute('aria-label') === generated))) {
        nativeUploadTreeGeneratedLabels.set(target, generated);
      }
      return scaffoldRegistry.setSlot(binding, target, 'pending');
    }

    function clearNativeUploadTreeGeneratedLabel(target) {
      const generated = nativeUploadTreeGeneratedLabels.get(target);
      if (generated && target.getAttribute('aria-label') === generated) target.removeAttribute('aria-label');
      nativeUploadTreeGeneratedLabels.delete(target);
    }

    function addNativeUploadTreeRetry(owner) {
      const status = Array.from(owner.target.children).find(child => child.hasAttribute('data-scaffold-status'));
      if (!status) return false;
      let retry = status.querySelector('[data-native-read-retry]');
      if (!retry) {
        retry = document.createElement('button');
        retry.type = 'button';
        retry.className = 'scaffold-slot-retry';
        retry.textContent = 'Retry';
        status.appendChild(retry);
      }
      retry.dataset.nativeReadRetry = owner.readKey;
      retry.dataset.nativeReadGuest = owner.guest;
      return true;
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
      const binding = owner.binding;
      const pane = owner.pane;
      const request = owner.request;
      const guest = owner.guest;
      const path = owner.path;
      const target = owner.target;
      const swap = owner.swap;
      const uploadTree = nativeUploadTreeOwnerSlot(owner) === target;
      if (guest === 'upload' && !uploadTree) {
        retireNativeHtmxOwner(owner, 'Upload tree slot retired', { abort: true });
        return false;
      }
      if (!uploadTree) displayNativeReadState(owner, reason, false);
      scaffoldRegistry.registerRetry(binding, owner.readKey, () => {
        if (!scaffoldRegistry.current(binding) || binding.root !== pane
            || !target?.isConnected || !pane.contains(target)) return false;
        const retrySource = guest === 'upload' ? (nativeUploadTreeSlot(pane) === target ? target : null)
          : guest === 'linker' ? target
          : request?.isConnected && pane.contains(request) ? request
          : null;
        if (!(retrySource instanceof Element)) return false;
        if (guest === 'upload' && !nativeUploadTreeSourceAllowed(target, retrySource, pane, path)) return false;
        return window.htmx?.ajax('GET', path, { source: retrySource, target, swap });
      }, { slots: [], timeoutMs: owner.timeoutMs, slowMs: DEFAULT_NATIVE_SLOW_READ_MS });
      if (uploadTree) {
        clearNativeReadState(owner);
        scaffoldRegistry.setSlot(binding, target, 'fault', reason);
        clearNativeUploadTreeGeneratedLabel(target);
        addNativeUploadTreeRetry(owner);
      } else displayNativeReadState(owner, reason, true);
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

    function nativeHtmxReadSourceAllowed(guest, request, source, path, pane) {
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
        return nativeUploadTreeSourceAllowed(request, source, pane, path);
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
      if (guest === 'upload') {
        const tree = nativeUploadTreeSlot(pane);
        return target === tree && (source === tree
          || (request instanceof Element && tree.contains(request)
            && source instanceof Element && tree.contains(source)));
      }
      if (!(request instanceof Element) || !pane.contains(request)) return false;
      if (guest === 'portals') return request.matches('[data-portals-grid]') && target === request;
      if (guest === 'admin') {
        if (request.matches('[data-service-card]')) return target === request;
        return request.matches('[data-admin-action-id="view-logs"]')
          && target === pane.querySelector('[data-admin-action-result]');
      }
      return false;
    }

    const guestSlotDeferred = new Map();
    const guestSlotOwnersByXhr = new WeakMap();
    const guestSlotTargetOwners = new WeakMap();
    const guestSlotIdentity = new WeakMap();
    const guestSlotActiveOwners = new Set();
    const GUEST_SLOT_MAX_DEFERRED = 128;
    const GUEST_SLOT_DEFAULT_TIMEOUT_MS = 8000;
    const GUEST_SLOT_MAX_TIMEOUT_MS = 30000;
    let nextGuestSlotIdentity = 0;

    function guestSlotReadKey(slot, guest) {
      let identity = guestSlotIdentity.get(slot);
      if (!identity) { identity = ++nextGuestSlotIdentity; guestSlotIdentity.set(slot, identity); }
      return `guest-slot:${guest}:${identity}`;
    }

    function validGuestPartPath(path, guest) {
      const prefix = `/api/tabs/${guest}/`;
      if (typeof path !== 'string' || !path.startsWith(prefix) || path.includes('#') || path.includes('\\')) return false;
      const pathname = path.split('?', 1)[0];
      const suffix = pathname.slice(prefix.length);
      if (!suffix || suffix.endsWith('/') || suffix.includes('//')) return false;
      try {
        return suffix.split('/').every(segment => {
          const decoded = decodeURIComponent(segment);
          return decoded && decoded !== '.' && decoded !== '..' && !decoded.includes('/') && !decoded.includes('\\');
        });
      } catch (_) { return false; }
    }

    function inspectGuestSlot(slot, path, target = slot) {
      if (!(slot instanceof Element) || !slot.hasAttribute('data-scaffold-slot')) return null;
      const pane = slot.closest('[data-pane-panel]');
      const guest = pane?.dataset.panePanel || '';
      if (!pane || !guest || Object.prototype.hasOwnProperty.call(nativePaneContracts, guest)) return null;
      const root = slot.closest('[data-scaffold-root]');
      let reason = '';
      if (!/^[a-z0-9][a-z0-9-]*$/.test(guest) || !slot.dataset.scaffoldSlot) reason = 'Guest slot identity is invalid';
      else if (!root || root.dataset.guestId !== guest || !root.dataset.scaffoldVersion) reason = 'Guest slot is outside its declared scaffold root';
      else if (!validGuestPartPath(path, guest)) reason = 'Guest slot part must stay under its own /api/tabs/<guest>/ prefix';
      else if (target !== slot || (slot.hasAttribute('hx-target') && slot.getAttribute('hx-target') !== 'this')) reason = 'Guest slot must target its own stable wrapper';
      else if (slot.hasAttribute('hx-swap') && slot.getAttribute('hx-swap') !== 'innerHTML') reason = 'Guest slot must fill its wrapper with innerHTML';
      else if ((slot.getAttribute('hx-trigger') || '').trim() !== 'load') reason = 'Guest slot must load on admission';
      return { slot, pane, guest, root, path, reason };
    }

    function guestSlotRequestInfo(detail) {
      const source = detail.elt instanceof Element ? detail.elt : null;
      const request = source?.closest('[hx-get], [hx-post], [hx-put], [hx-patch], [hx-delete]');
      if (!(request instanceof Element) || !request.hasAttribute('data-scaffold-slot')) return null;
      const pane = request.closest('[data-pane-panel]');
      const guest = pane?.dataset.panePanel || '';
      if (!pane || !guest || Object.prototype.hasOwnProperty.call(nativePaneContracts, guest)) return null;
      const path = String(detail.requestConfig?.path || request.getAttribute('hx-get') || '');
      const inspected = inspectGuestSlot(request, path, detail.target);
      if (!inspected) return null;
      const verb = String(detail.requestConfig?.verb || detail.requestConfig?.method || (request.hasAttribute('hx-get') ? 'get' : '')).toLowerCase();
      if (!inspected.reason && verb !== 'get') inspected.reason = 'Guest slots accept bodyless GET only';
      if (!detail.xhr && !inspected.reason) inspected.reason = 'Guest slot request owner is missing';
      return inspected;
    }

    function guestSlotStatus(slot, state, guest, readKey, message, retry = false) {
      let status = Array.from(slot.children).find(child => child.hasAttribute('data-scaffold-status')) || null;
      if (!status) {
        status = document.createElement('div');
        slot.appendChild(status);
      }
      status.dataset.scaffoldStatus = state;
      status.className = `scaffold-slot-${state}`;
      status.setAttribute('role', state === 'fault' ? 'alert' : 'status');
      status.textContent = '';
      const text = document.createElement('span');
      text.textContent = message;
      status.appendChild(text);
      if (retry && readKey) {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'scaffold-slot-retry';
        button.dataset.scaffoldRetry = guest;
        button.dataset.scaffoldReadKey = readKey;
        button.textContent = 'Retry';
        status.appendChild(button);
      }
    }

    function guestSlotSetPending(slot, guest, readKey, slow = false) {
      if (!(slot instanceof Element) || !slot.isConnected) return;
      slot.setAttribute('aria-busy', 'true');
      if (slot.dataset.scaffoldState === 'ready') {
        slot.dataset.scaffoldStale = 'true';
        guestSlotStatus(slot, 'stale', guest, readKey,
          `Refreshing ${slot.dataset.scaffoldPlaceholder || 'content'}; showing the last successful read${slot.dataset.scaffoldUpdatedAt ? ` from ${new Date(Number(slot.dataset.scaffoldUpdatedAt)).toLocaleTimeString()}` : ''}.`, true);
        return;
      }
      if (slot.dataset.scaffoldState === 'fault' && !slow) {
        Array.from(slot.children).filter(child => child.hasAttribute('data-scaffold-status')).forEach(child => child.remove());
      }
      slot.dataset.scaffoldState = 'pending';
      slot.dataset.scaffoldPending = 'true';
      delete slot.dataset.scaffoldError;
      delete slot.dataset.scaffoldStale;
      slot.removeAttribute('aria-invalid');
      if (slow) guestSlotStatus(slot, 'pending', guest, readKey,
        `Still waiting for ${slot.dataset.scaffoldPlaceholder || 'this region'}.`, true);
    }

    function guestSlotSetReady(slot) {
      if (!(slot instanceof Element) || !slot.isConnected) return;
      Array.from(slot.children).filter(child => child.hasAttribute('data-scaffold-status')).forEach(child => child.remove());
      slot.dataset.scaffoldState = 'ready';
      slot.dataset.scaffoldUpdatedAt = String(Date.now());
      delete slot.dataset.scaffoldPending;
      delete slot.dataset.scaffoldError;
      delete slot.dataset.scaffoldStale;
      slot.removeAttribute('aria-invalid');
      slot.setAttribute('aria-busy', 'false');
    }

    function guestSlotSetFailure(slot, guest, readKey, message, retry = true) {
      if (!(slot instanceof Element) || !slot.isConnected) return;
      const hadGoodContent = slot.dataset.scaffoldState === 'ready';
      slot.setAttribute('aria-busy', 'false');
      delete slot.dataset.scaffoldPending;
      slot.dataset.scaffoldError = message;
      if (hadGoodContent) {
        slot.dataset.scaffoldStale = 'true';
        slot.removeAttribute('aria-invalid');
        const age = slot.dataset.scaffoldUpdatedAt
          ? ` Last successful read: ${new Date(Number(slot.dataset.scaffoldUpdatedAt)).toLocaleTimeString()}.` : '';
        guestSlotStatus(slot, 'stale', guest, readKey, `${message}.${age}`, retry);
      } else {
        slot.dataset.scaffoldState = 'fault';
        delete slot.dataset.scaffoldStale;
        slot.setAttribute('aria-invalid', 'true');
        guestSlotStatus(slot, 'fault', guest, readKey, `${message}. `, retry);
      }
    }

    function guestSlotOwnerCurrent(owner) {
      return Boolean(owner && !owner.retired
        && guestSlotOwnersByXhr.get(owner.xhr) === owner
        && guestSlotTargetOwners.get(owner.slot) === owner
        && owner.binding.readOwners.get(owner.readKey) === owner
        && scaffoldRegistry.current(owner.binding)
        && window.immortalFloor?.state === 'Seated'
        && window.immortalFloor?.activeGuest === owner.guest
        && owner.binding.root === owner.pane
        && owner.pane.isConnected && owner.slot.isConnected && owner.pane.contains(owner.slot)
        && owner.slot.closest('[data-scaffold-root]') === owner.root
        && owner.root.dataset.guestId === owner.guest);
    }

    function retireGuestSlotOwner(owner, reason = 'Guest slot retired', { abort = true } = {}) {
      if (!owner || owner.retired) return false;
      owner.retired = true;
      clearNativeHtmxSwapToken(owner);
      window.clearTimeout(owner.timer);
      window.clearTimeout(owner.slowTimer);
      owner.timer = null;
      owner.slowTimer = null;
      if (owner.release) { const release = owner.release; owner.release = null; release(); }
      if (owner.binding.readOwners.get(owner.readKey) === owner) owner.binding.readOwners.delete(owner.readKey);
      if (guestSlotTargetOwners.get(owner.slot) === owner) guestSlotTargetOwners.delete(owner.slot);
      guestSlotActiveOwners.delete(owner);
      if (abort) {
        try { if (owner.xhr.readyState !== XMLHttpRequest.DONE) owner.xhr.abort(); } catch (_) {}
      }
      return true;
    }

    function retireGuestScaffoldSlots(guest = null, reason = 'Guest slot lifetime retired') {
      const selected = guest == null ? null : String(guest);
      [...guestSlotActiveOwners].forEach(owner => {
        if (selected === null || owner.guest === selected) retireGuestSlotOwner(owner, reason, { abort: true });
      });
      [...guestSlotDeferred.entries()].forEach(([slot, intent]) => {
        if (selected === null || intent.guest === selected) guestSlotDeferred.delete(slot);
      });
      return true;
    }
    window.retireGuestScaffoldSlots = retireGuestScaffoldSlots;

    function guestSlotFailureReason(owner, error = null) {
      if (error === 'Read timed out' || error?.message === 'Read timed out') return 'Read timed out';
      const status = Number(owner?.xhr?.status || 0);
      if (status >= 400) return `Read failed (HTTP ${status})`;
      if (error === 'Guest slot response contained forbidden script markup') return error;
      if (error === 'Guest slot response attempted an out-of-band swap') return error;
      if (error === 'Guest slot response was not renderable') return error;
      return 'Guest slot content unavailable';
    }

    function failGuestSlotOwner(owner, error = 'Guest slot content unavailable') {
      if (!owner || owner.responseHandled) return false;
      if (!guestSlotOwnerCurrent(owner)) {
        retireGuestSlotOwner(owner, 'Guest slot owner retired', { abort: true });
        return false;
      }
      owner.responseHandled = true;
      guestSlotSetFailure(owner.slot, owner.guest, owner.readKey, guestSlotFailureReason(owner, error), true);
      retireGuestSlotOwner(owner, 'Guest slot failed', { abort: error === 'Read timed out' || error?.message === 'Read timed out' });
      return true;
    }

    function guestSlotTimeoutMs(detail, slot) {
      let value = Number(detail.requestConfig?.timeout || 0);
      if (!Number.isSafeInteger(value) || value <= 0) {
        try { value = Number(JSON.parse(slot.getAttribute('hx-request') || '{}').timeout || 0); } catch (_) { value = 0; }
      }
      return Number.isSafeInteger(value) && value > 0
        ? Math.min(value, GUEST_SLOT_MAX_TIMEOUT_MS) : GUEST_SLOT_DEFAULT_TIMEOUT_MS;
    }

    function beginGuestSlotOwner(info, detail) {
      const { slot, pane, guest, root, path } = info;
      const binding = scaffoldRegistry.capture(guest);
      if (!binding || !scaffoldRegistry.current(binding) || window.immortalFloor?.state !== 'Seated'
          || window.immortalFloor?.activeGuest !== guest || binding.root !== pane) return false;
      const xhr = detail.xhr;
      if (!xhr) return false;
      const readKey = guestSlotReadKey(slot, guest);
      const prior = binding.readOwners.get(readKey);
      if (prior && !prior.retired) {
        if (guestSlotOwnerCurrent(prior) && prior.path === path) return false;
        retireGuestSlotOwner(prior, 'Guest slot read superseded', { abort: true });
      }
      const timeoutMs = guestSlotTimeoutMs(detail, slot);
      const owner = { guest, pane, root, slot, request: slot, binding, xhr, path, readKey, timeoutMs,
        retired: false, responseHandled: false, swapAuthorized: false, morphed: false,
        timer: null, slowTimer: null, release: null, swapToken: null };
      owner.retire = reason => retireGuestSlotOwner(owner, reason || 'Guest retired', { abort: true });
      binding.readOwners.set(readKey, owner);
      guestSlotOwnersByXhr.set(xhr, owner);
      guestSlotTargetOwners.set(slot, owner);
      guestSlotActiveOwners.add(owner);
      guestSlotSetPending(slot, guest, readKey);
      try { xhr.timeout = timeoutMs; } catch (_) {}
      owner.release = scaffoldRegistry.onRetire(binding, retirement => {
        retireGuestSlotOwner(owner, retirement?.reason || 'Guest retired', { abort: true });
      });
      owner.timer = window.setTimeout(() => failGuestSlotOwner(owner, 'Read timed out'), timeoutMs + 25);
      owner.slowTimer = window.setTimeout(() => {
        if (guestSlotOwnerCurrent(owner)) guestSlotSetPending(slot, guest, readKey, true);
      }, Math.min(750, Math.max(1, timeoutMs - 1)));
      return owner;
    }

    function deferGuestSlot(info) {
      if (!guestSlotDeferred.has(info.slot) && guestSlotDeferred.size >= GUEST_SLOT_MAX_DEFERRED) {
        guestSlotSetFailure(info.slot, info.guest, null, 'Too many guest slots are waiting for admission', false);
        return false;
      }
      guestSlotDeferred.set(info.slot, info);
      if (info.slot.dataset.scaffoldState !== 'ready') {
        info.slot.dataset.scaffoldState = 'pending';
        info.slot.setAttribute('aria-busy', 'true');
      }
      return true;
    }

    function startGuestScaffoldSlots(guest, pane, binding) {
      guest = String(guest || '');
      if (!guest || Object.prototype.hasOwnProperty.call(nativePaneContracts, guest)
          || !(pane instanceof Element) || !binding || binding.guest !== guest || binding.root !== pane
          || !scaffoldRegistry.current(binding) || window.immortalFloor?.state !== 'Seated'
          || window.immortalFloor?.activeGuest !== guest) return false;
      [...guestSlotDeferred.entries()].forEach(([slot, intent]) => {
        if (intent.guest === guest && !pane.contains(slot)) guestSlotDeferred.delete(slot);
      });
      const slots = [...pane.querySelectorAll('[data-scaffold-slot][hx-get]')];
      if (pane.matches('[data-scaffold-slot][hx-get]')) slots.unshift(pane);
      slots.forEach(slot => {
        const path = slot.getAttribute('hx-get') || '';
        const info = inspectGuestSlot(slot, path, slot);
        if (!info) return;
        const readKey = guestSlotReadKey(slot, guest);
        if (info.reason) {
          guestSlotDeferred.delete(slot);
          guestSlotSetFailure(slot, guest, null, info.reason, false);
          return;
        }
        if (binding.readOwners.has(readKey)) return;
        const queued = guestSlotDeferred.get(slot);
        if (queued?.reason) {
          guestSlotDeferred.delete(slot);
          guestSlotSetFailure(slot, guest, null, queued.reason, false);
          return;
        }
        const state = slot.dataset.scaffoldState;
        if (!queued && state !== 'pending' && slot.dataset.scaffoldStale !== 'true') return;
        guestSlotDeferred.delete(slot);
        if (!window.htmx?.ajax) {
          guestSlotSetFailure(slot, guest, null, 'Guest slot transport is unavailable', false);
          return;
        }
        guestSlotSetPending(slot, guest, readKey);
        try { window.htmx.ajax('GET', path, { source: slot, target: slot, swap: 'innerHTML' }); }
        catch (_) { guestSlotSetFailure(slot, guest, readKey, 'Guest slot transport is unavailable', true); }
      });
      return true;
    }
    window.startGuestScaffoldSlots = startGuestScaffoldSlots;

    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState !== 'visible') retireGuestScaffoldSlots(null, 'Document hidden');
    });

    document.body.addEventListener('click', event => {
      const target = event.target instanceof Element ? event.target : null;
      const retry = target?.closest('[data-scaffold-retry][data-scaffold-read-key^="guest-slot:"]');
      if (!retry) return;
      const slot = retry.closest('[data-scaffold-slot]');
      const guest = retry.dataset.scaffoldRetry || '';
      const path = slot?.getAttribute('hx-get') || '';
      const info = inspectGuestSlot(slot, path, slot);
      const seated = window.immortalFloor?.state === 'Seated' && window.immortalFloor?.activeGuest === guest;
      const binding = info && !info.reason && seated ? scaffoldRegistry.capture(guest) : null;
      if (!info || info.reason || info.guest !== guest || !seated || !binding || !scaffoldRegistry.current(binding)
          || binding.root !== info.pane) {
        event.preventDefault(); event.stopPropagation();
        return;
      }
      event.preventDefault(); event.stopPropagation();
      const key = guestSlotReadKey(slot, guest);
      guestSlotSetPending(slot, guest, key);
      try { window.htmx?.ajax('GET', path, { source: slot, target: slot, swap: 'innerHTML' }); }
      catch (_) { guestSlotSetFailure(slot, guest, key, 'Guest slot transport is unavailable', true); }
    });

    if (window.htmx?.defineExtension) {
      window.htmx.defineExtension(nativeMorphSwapStyle, {
        isInlineSwap(style) { return style === nativeMorphSwapStyle || style.startsWith(`${nativeMorphSwapStyle}-`); },
        handleSwap(style, target, fragment) {
          if (style !== nativeMorphSwapStyle && !style.startsWith(`${nativeMorphSwapStyle}-`)) return false;
          const reference = nativeHtmxSwapOwners.get(style);
          nativeHtmxSwapOwners.delete(style);
          const xhr = reference?.deref();
          const nativeOwner = xhr ? nativeHtmxOwners.get(xhr) : null;
          if (!nativeOwner) {
            const owner = xhr ? guestSlotOwnersByXhr.get(xhr) : null;
            const tokenAuthorized = owner?.swapToken === style;
            if (tokenAuthorized) owner.swapToken = null;
            if (!tokenAuthorized || !guestSlotOwnerCurrent(owner) || owner.xhr !== xhr
                || !owner.swapAuthorized || target !== owner.slot) return [];
            const insertedNodes = [];
            const changed = morphLivePane(target, fragment, {
              outerRoot: false,
              insertedNodes
            });
            if (!guestSlotOwnerCurrent(owner) || target !== owner.slot) return [];
            owner.morphed = true;
            owner.changed = changed;
            insertedNodes.forEach(node => {
              if (!guestSlotOwnerCurrent(owner) || !node.isConnected || !owner.slot.contains(node)) return;
              try { window.htmx?.process(node); } catch (_) {}
            });
            return [];
          }
          const owner = nativeOwner;
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

    function guestSlotResponseFailure(owner, html) {
      const status = Number(owner.xhr?.status || 0);
      if (status < 200 || status >= 300) return `Read failed (HTTP ${status || 'unavailable'})`;
      const template = document.createElement('template');
      template.innerHTML = String(html || '');
      let hasScript = false;
      nativeHtmxVisitNodes(template.content, node => {
        if (node.nodeType === Node.ELEMENT_NODE && node.tagName === 'SCRIPT') hasScript = true;
      });
      if (hasScript) return 'Guest slot response contained forbidden script markup';
      if (template.content.querySelector('[data-cartridge-fault], [data-cartridge-fault-kind], [data-service-card-fault-kind], [data-admin-membrane-refusal]')) return 'Guest slot content unavailable';
      const primary = template.content.cloneNode(true);
      if (nativeHtmxRemoveOutOfBand(primary)) return 'Guest slot response attempted an out-of-band swap';
      if (primary.childElementCount === 0 && !String(primary.textContent || '').trim()) return 'Guest slot response was not renderable';
      return '';
    }

    document.body.addEventListener('htmx:beforeRequest', event => {
      const detail = event.detail || {};
      const guestSlot = guestSlotRequestInfo(detail);
      if (guestSlot) {
        const seated = window.immortalFloor?.state === 'Seated'
          && window.immortalFloor?.activeGuest === guestSlot.guest;
        const binding = seated ? scaffoldRegistry.capture(guestSlot.guest) : null;
        if (guestSlot.reason) {
          event.preventDefault();
          if (seated && binding && scaffoldRegistry.current(binding)) {
            guestSlotSetFailure(guestSlot.slot, guestSlot.guest, null, guestSlot.reason, false);
          } else deferGuestSlot(guestSlot);
          return;
        }
        if (!seated || !binding || !scaffoldRegistry.current(binding) || binding.root !== guestSlot.pane) {
          event.preventDefault();
          deferGuestSlot(guestSlot);
          return;
        }
        const readKey = guestSlotReadKey(guestSlot.slot, guestSlot.guest);
        const prior = binding.readOwners.get(readKey);
        if (prior && !prior.retired && guestSlotOwnerCurrent(prior) && prior.path === guestSlot.path) {
          event.preventDefault();
          return;
        }
        if (!beginGuestSlotOwner(guestSlot, detail)) {
          event.preventDefault();
          guestSlotSetFailure(guestSlot.slot, guestSlot.guest, readKey, 'Guest slot request could not be admitted', true);
          return;
        }
        guestSlotDeferred.delete(guestSlot.slot);
        return;
      }
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
      if (!nativeHtmxReadSourceAllowed(guest, request, source, path, pane)) return;
      if (!nativeHtmxReadTargetAllowed(guest, request, source, target, pane)) {
        event.preventDefault();
        return;
      }
      const binding = scaffoldRegistry.capture(guest);
      if (!binding || !scaffoldRegistry.current(binding) || binding.root !== pane || !target || !binding.root.contains(target)) {
        event.preventDefault();
        return;
      }
      if (guest === 'upload' && scaffoldRegistry.slot(binding, 'upload-tree') !== target) {
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
      if (guest === 'upload' && !setNativeUploadTreePending(binding, target)) {
        event.preventDefault();
        retireNativeHtmxOwner(owner, 'Upload tree slot retired', { abort: true });
        return;
      }

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

    document.body.addEventListener('htmx:beforeSwap', event => {
      const xhr = event.detail?.xhr;
      const owner = xhr && guestSlotOwnersByXhr.get(xhr);
      if (!owner) return;
      if (!guestSlotOwnerCurrent(owner) || event.detail?.target !== owner.slot) {
        event.detail.shouldSwap = false;
        retireGuestSlotOwner(owner, 'Guest slot owner retired', { abort: true });
        return;
      }
      const response = String(event.detail?.serverResponse || owner.xhr?.responseText || '');
      const failure = guestSlotResponseFailure(owner, response);
      if (failure) {
        event.detail.shouldSwap = false;
        failGuestSlotOwner(owner, failure);
        return;
      }
      nativeMorphExtensionEnabled(owner.pane);
      owner.swapToken = nativeHtmxSwapToken(owner);
      owner.swapAuthorized = true;
      event.detail.swapOverride = owner.swapToken;
    });

    document.body.addEventListener('htmx:afterSwap', event => {
      const guestOwner = event.detail?.xhr && guestSlotOwnersByXhr.get(event.detail.xhr);
      if (guestOwner) {
        if (!guestSlotOwnerCurrent(guestOwner) || event.detail?.target !== guestOwner.slot || !guestOwner.swapAuthorized) {
          retireGuestSlotOwner(guestOwner, 'Guest slot swap retired', { abort: true });
          return;
        }
        return;
      }
      const owner = nativeHtmxOwner(event);
      if (!owner || owner.retired || event.detail?.target !== owner.target) return;
      if (!nativeHtmxOwnerCurrent(owner, false)) {
        retireNativeHtmxOwner(owner, 'Read owner retired');
      }
    });

    document.body.addEventListener('htmx:afterSettle', event => {
      const guestOwner = event.detail?.xhr && guestSlotOwnersByXhr.get(event.detail.xhr);
      if (guestOwner) {
        if (!guestSlotOwnerCurrent(guestOwner) || event.detail?.target !== guestOwner.slot) {
          retireGuestSlotOwner(guestOwner, 'Guest slot owner retired', { abort: true });
          return;
        }
        if (!guestOwner.swapAuthorized || !guestOwner.morphed) {
          failGuestSlotOwner(guestOwner, 'Guest slot response was not renderable');
          return;
        }
        guestSlotSetReady(guestOwner.slot);
        retireGuestSlotOwner(guestOwner, 'Guest slot read completed', { abort: false });
        return;
      }
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
      if (owner.guest === 'upload') {
        if (!nativeUploadTreeOwnerSlot(owner) || !scaffoldRegistry.setSlot(owner.binding, owner.target, 'ready')) {
          failNativeHtmxOwner(owner, 'Read did not produce an upload tree slot');
          return;
        }
        clearNativeUploadTreeGeneratedLabel(owner.target);
      }
      applyAdminDomState();
      clearNativeReadState(owner, { restoreStatus: false });
      retireNativeHtmxOwner(owner, 'Read completed');
    });

    document.body.addEventListener('htmx:afterRequest', event => {
      const guestOwner = event.detail?.xhr && guestSlotOwnersByXhr.get(event.detail.xhr);
      if (guestOwner && !guestOwner.retired) {
        if (event.detail?.target && event.detail.target !== guestOwner.slot) {
          failGuestSlotOwner(guestOwner, 'Guest slot response was not renderable');
          return;
        }
        if (event.detail?.successful !== true) {
          failGuestSlotOwner(guestOwner, 'Guest slot content unavailable');
          return;
        }
        if (!guestOwner.swapAuthorized) failGuestSlotOwner(guestOwner, 'Guest slot response was not renderable');
        return;
      }
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

    function reportGuestSlotFailure(event) {
      const owner = event.detail?.xhr && guestSlotOwnersByXhr.get(event.detail.xhr);
      if (!owner || owner.retired) return;
      const reason = event.type === 'htmx:timeout' ? 'Read timed out'
        : event.type === 'htmx:sendError' ? 'Guest slot content unavailable'
        : 'Guest slot content unavailable';
      failGuestSlotOwner(owner, reason);
    }

    ['htmx:timeout', 'htmx:responseError', 'htmx:sendError'].forEach(name => {
      document.body.addEventListener(name, reportNativeHtmxFailure);
      document.body.addEventListener(name, reportGuestSlotFailure);
    });
"####
}
