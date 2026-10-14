fn shell_pane_structure_owner_client() -> &'static str {
    r####"    const scaffoldSlotStructures = new WeakMap();
    const scaffoldDynamicAttributes = new Set(['data-scaffold-state', 'data-scaffold-pending', 'data-scaffold-error', 'data-scaffold-stale', 'data-scaffold-updated-at', 'aria-busy', 'aria-invalid', 'aria-selected', 'aria-checked', 'aria-pressed', 'checked', 'selected', 'value', 'open']);
    function scaffoldStructureSignature(node) {
      if (node.nodeType === Node.TEXT_NODE) return '#text';
      if (node.nodeType === Node.COMMENT_NODE) return '#comment';
      if (node.nodeType !== Node.ELEMENT_NODE) return `#${node.nodeType}`;
      const attrs = Array.from(node.attributes)
        .filter(attribute => !scaffoldDynamicAttributes.has(attribute.name) && attribute.name !== 'data-scaffold-status')
        .map(attribute => [attribute.name, attribute.value])
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([name, value]) => `${name}=${JSON.stringify(value)}`).join(';');
      const children = Array.from(node.childNodes)
        .filter(child => !(child.nodeType === Node.ELEMENT_NODE && child.hasAttribute('data-scaffold-status')))
        .map(scaffoldStructureSignature).join('|');
      return `<${node.tagName}[${attrs}]>${children}</${node.tagName}>`;
    }
    function rememberScaffoldSlotStructures(root) {
      root?.querySelectorAll('[data-scaffold-slot]').forEach(slot => {
        if (!scaffoldSlotStructures.has(slot)) scaffoldSlotStructures.set(slot, scaffoldStructureSignature(slot));
      });
    }
    function morphLivePane(target, html, options = {}) {
      const template = document.createElement('template');
      if (html instanceof DocumentFragment) template.content.appendChild(html.cloneNode(true));
      else template.innerHTML = String(html || '');
      let mutated = false;
      const sameKind = (left, right) => left.nodeType === right.nodeType && (left.nodeType !== Node.ELEMENT_NODE || left.tagName === right.tagName);
      const nodeKey = node => {
        if (node.nodeType !== Node.ELEMENT_NODE) return null;
        if (node.id) return `id:${node.id}`;
        if (node.hasAttribute('data-scaffold-slot')) return `slot:${node.dataset.scaffoldSlot}`;
        if (node.hasAttribute('data-scaffold-key')) return `scaffold:${node.dataset.scaffoldKey}`;
        if (node.hasAttribute('data-linker-path')) return `linker:${node.dataset.linkerPath}`;
        if (node.hasAttribute('data-stat-element-id')) return `stat:${node.dataset.statElementId}`;
        return null;
      };
      const morphAttributes = (current, next) => {
        Array.from(current.attributes).forEach(attribute => {
          if (!next.hasAttribute(attribute.name)) { current.removeAttribute(attribute.name); mutated = true; }
        });
        Array.from(next.attributes).forEach(attribute => {
          if (current.getAttribute(attribute.name) !== attribute.value) { current.setAttribute(attribute.name, attribute.value); mutated = true; }
        });
      };
      const morphChildren = (current, next) => {
        const previous = Array.from(current.childNodes);
        const keyed = new Map(previous.map(child => [nodeKey(child), child]).filter(([key]) => key));
        const used = new Set();
        Array.from(next.childNodes).forEach((nextChild, index) => {
          const key = nodeKey(nextChild);
          let currentChild = key ? keyed.get(key) : previous[index];
          if (currentChild && used.has(currentChild)) currentChild = null;
          if (!currentChild) {
            currentChild = nextChild.cloneNode(true);
            current.insertBefore(currentChild, current.childNodes[index] || null);
            if (currentChild.nodeType === Node.ELEMENT_NODE && Array.isArray(options.insertedNodes)) options.insertedNodes.push(currentChild);
            mutated = true;
            if (currentChild.nodeType === Node.ELEMENT_NODE && currentChild.hasAttribute('data-scaffold-slot')) scaffoldSlotStructures.set(currentChild, scaffoldStructureSignature(nextChild));
            used.add(currentChild);
            return;
          }
          if (currentChild !== current.childNodes[index]) { current.insertBefore(currentChild, current.childNodes[index] || null); mutated = true; }
          const result = morphNode(currentChild, nextChild);
          used.add(result);
        });
        Array.from(current.childNodes).forEach(child => {
          if (!used.has(child)) { child.remove(); mutated = true; }
        });
      };
      const morphNode = (current, next) => {
        if (!sameKind(current, next)) {
          const replacement = next.cloneNode(true);
          current.replaceWith(replacement); mutated = true;
          if (replacement.nodeType === Node.ELEMENT_NODE && Array.isArray(options.insertedNodes)) options.insertedNodes.push(replacement);
          if (replacement.nodeType === Node.ELEMENT_NODE && replacement.hasAttribute('data-scaffold-slot')) scaffoldSlotStructures.set(replacement, scaffoldStructureSignature(next));
          return replacement;
        }
        if (current.nodeType === Node.TEXT_NODE || current.nodeType === Node.COMMENT_NODE) {
          if (current.nodeValue !== next.nodeValue) { current.nodeValue = next.nodeValue; mutated = true; }
          return current;
        }
        if (current.hasAttribute('data-scaffold-slot') && next.hasAttribute('data-scaffold-slot') && current.dataset.scaffoldSlot === next.dataset.scaffoldSlot) {
          const incomingShape = scaffoldStructureSignature(next);
          const liveShape = scaffoldSlotStructures.get(current) || scaffoldStructureSignature(current);
          if (liveShape === incomingShape) {
            scaffoldSlotStructures.set(current, incomingShape);
            return current;
          }
        }
        morphAttributes(current, next);
        // Stats owns chart canvases and live readouts between element-fact pulls. Its
        // element wrapper carries the fact attributes; preserve its live descendants.
        if (current.hasAttribute('data-stat-element-id')) {
          const currentHeader = Array.from(current.children).find(child => child.classList.contains('stat-header'));
          const nextHeader = Array.from(next.children).find(child => child.classList.contains('stat-header'));
          if (currentHeader && nextHeader) morphNode(currentHeader, nextHeader);
          return current;
        }
        morphChildren(current, next);
        if (current.hasAttribute('data-scaffold-slot')) scaffoldSlotStructures.set(current, scaffoldStructureSignature(next));
        return current;
      };
      if (options.outerRoot) {
        const roots = Array.from(template.content.children);
        if (roots.length !== 1 || roots[0].tagName !== target.tagName || roots[0].id !== target.id) return false;
        const preservedRootAttributes = (options.preserveOuterRootAttributes || [])
          .map(name => [name, target.getAttribute(name)])
          .filter(([, value]) => value !== null);
        morphAttributes(target, roots[0]);
        preservedRootAttributes.forEach(([name, value]) => {
          if (!roots[0].hasAttribute(name)) target.setAttribute(name, value);
        });
        morphChildren(target, roots[0]);
      } else {
        morphChildren(target, template.content);
      }
      return mutated;
    }
    const scaffoldRegistry = (() => {
      const guests = new Map();
      const bindings = new Map();
      const validStates = new Set(['pending', 'ready', 'fault']);
      const DEFAULT_READ_TIMEOUT_MS = 8000;
      const DEFAULT_SLOW_READ_MS = 750;
      const DEFAULT_BODY_MAX_BYTES = 4 * 1024 * 1024;
      function slotsFor(binding) {
        if (!binding.root) return [];
        const candidates = [...binding.root.querySelectorAll('[data-scaffold-slot]')];
        const declared = nativePaneContracts[binding.guest]?.slots;
        return Array.isArray(declared) ? candidates.filter(node => declared.includes(node.dataset.scaffoldKey || node.dataset.scaffoldSlot)) : candidates;
      }
      function slotAllowed(binding, target) {
        const declared = nativePaneContracts[binding.guest]?.slots;
        return !Array.isArray(declared) || slotsFor(binding).includes(target);
      }
      function rootFor(registration) {
        try { return typeof registration.root === 'function' ? registration.root() : document.querySelector(registration.root); }
        catch (_) { return null; }
      }
      function stateFor(slot) { return validStates.has(slot.dataset.scaffoldState) ? slot.dataset.scaffoldState : 'pending'; }
      function removeStatus(slot) {
        slot.querySelector('[data-scaffold-status]')?.remove();
        slot.removeAttribute('aria-invalid');
        slot.removeAttribute('data-scaffold-error');
        delete slot.dataset.scaffoldStale;
      }
      function statusNode(slot, state, guest, readKey, message, retry = false) {
        let status = slot.querySelector('[data-scaffold-status]');
        if (!status) { status = document.createElement('div'); status.dataset.scaffoldStatus = state; status.className = `scaffold-slot-${state}`; status.setAttribute('role', state === 'fault' ? 'alert' : 'status'); slot.appendChild(status); }
        status.dataset.scaffoldStatus = state;
        status.className = `scaffold-slot-${state}`;
        status.textContent = '';
        const text = document.createElement('span'); text.textContent = message; status.appendChild(text);
        if (retry && readKey) {
          const button = document.createElement('button');
          button.type = 'button'; button.className = 'scaffold-slot-retry';
          button.dataset.scaffoldRetry = guest; button.dataset.scaffoldReadKey = readKey;
          button.textContent = 'Retry'; status.appendChild(button);
        }
      }
      function setPending(slot, key, guest, readKey = null) {
        slot.setAttribute('aria-busy', 'true');
        if (stateFor(slot) === 'ready') {
          slot.dataset.scaffoldStale = 'true';
          statusNode(slot, 'stale', guest, readKey, `Refreshing ${slot.dataset.scaffoldPlaceholder || key || 'content'}; showing the last successful read${slot.dataset.scaffoldUpdatedAt ? ` from ${new Date(Number(slot.dataset.scaffoldUpdatedAt)).toLocaleTimeString()}` : ''}.`, Boolean(readKey));
          return;
        }
        removeStatus(slot);
        slot.dataset.scaffoldState = 'pending';
        slot.dataset.scaffoldPending = 'true';
        if (!slot.hasAttribute('aria-label')) slot.setAttribute('aria-label', 'Loading ' + (slot.dataset.scaffoldPlaceholder || key || 'content'));
      }
      function setReady(slot) {
        removeStatus(slot);
        slot.dataset.scaffoldState = 'ready';
        slot.dataset.scaffoldUpdatedAt = String(Date.now());
        delete slot.dataset.scaffoldPending;
        slot.setAttribute('aria-busy', 'false');
      }
      function setFault(slot, guest, detail, readKey = null) {
        const message = detail || 'Content unavailable';
        if (stateFor(slot) === 'ready') { setStale(slot, guest, message, readKey); return; }
        slot.dataset.scaffoldState = 'fault';
        delete slot.dataset.scaffoldPending;
        slot.setAttribute('aria-busy', 'false');
        slot.setAttribute('aria-invalid', 'true');
        slot.dataset.scaffoldError = message;
        statusNode(slot, 'fault', guest, readKey, `${message}. `, Boolean(readKey));
      }
      function setStale(slot, guest, detail, readKey = null) {
        const message = detail || 'Showing last known data';
        slot.dataset.scaffoldState = 'ready';
        slot.dataset.scaffoldStale = 'true';
        slot.setAttribute('aria-busy', 'false');
        slot.removeAttribute('aria-invalid');
        slot.dataset.scaffoldError = message;
        const age = slot.dataset.scaffoldUpdatedAt ? ` Last successful read: ${new Date(Number(slot.dataset.scaffoldUpdatedAt)).toLocaleTimeString()}.` : '';
        statusNode(slot, 'stale', guest, readKey, `${message}.${age} `, Boolean(readKey));
      }
      function current(binding, run = null, allowTimedOut = false) {
        if (!binding || binding.retired || bindings.get(binding.guest) !== binding || guests.get(binding.guest) !== binding.registration || !(binding.root instanceof Element) || !binding.root.isConnected || document.visibilityState !== 'visible') return false;
        if (typeof viewportFamilyAdmitted === 'function' && !viewportFamilyAdmitted(binding.guest)) return false;
        if (window.immortalFloor?.activeGuest !== binding.guest) return false;
        if (typeof sessionLawfulTab === 'function' && !sessionLawfulTab(binding.guest)) return false;
        try { if (typeof binding.registration.authorize === 'function' && !binding.registration.authorize(binding.guest, binding)) return false; }
        catch (_) { return false; }
        return !run || (!run.retired && (!run.controller.signal.aborted || (allowTimedOut && (run.timedOut || run.externalAbort))) && binding.runs.get(run.key) === run);
      }
      function resolveSlots(binding, selected) {
        binding.slots = slotsFor(binding);
        const values = Array.isArray(selected) ? selected : binding.slots;
        const resolved = values.map(value => value instanceof Element ? value : slot(binding, value)).filter(node => node instanceof Element && binding.root.contains(node) && slotAllowed(binding, node));
        return [...new Set(resolved)];
      }
      function stopRun(binding, run, reason = 'Read retired') {
        if (!run || run.retired) return false;
        run.retired = true;
        window.clearTimeout(run.timer); window.clearTimeout(run.slowTimer);
        run.timer = null; run.slowTimer = null;
        if (run.upstream) run.upstream.removeEventListener('abort', run.relay);
        if (binding.runs.get(run.key) === run) binding.runs.delete(run.key);
        try { run.controller.abort(new DOMException(reason, 'AbortError')); } catch (_) { run.controller.abort(); }
        try { run.cancel?.(new DOMException(reason, 'AbortError')); } catch (_) {}
        return true;
      }
      function retireBinding(binding, reason = 'Guest retired') {
        if (!binding || binding.retired) return false;
        binding.retired = true;
        [...binding.runs.values()].forEach(run => stopRun(binding, run, reason));
        [...binding.readOwners.values()].forEach(owner => { try { owner.retire?.(reason); } catch (_) {} });
        [...binding.retireCallbacks].forEach(callback => { try { callback({ guest: binding.guest, binding, reason }); } catch (_) {} });
        binding.retireCallbacks.clear();
        binding.retryTasks.clear(); binding.readOwners.clear(); binding.slots = [];
        if (bindings.get(binding.guest) === binding) bindings.delete(binding.guest);
        binding.root = null; binding.registration = null;
        return true;
      }
      function registerGuest(registration) {
        if (!registration || typeof registration.guest !== 'string' || !registration.root || !(typeof registration.root === 'function' || typeof registration.root === 'string')) return false;
        const active = bindings.get(registration.guest);
        if (active) retireBinding(active, 'Guest registration replaced');
        guests.set(registration.guest, registration);
        return true;
      }
      function hasGuest(guest) { return guests.has(String(guest)); }
      function bind(guest) {
        guest = String(guest);
        let registration = guests.get(guest);
        if (!registration) {
          registration = { guest, root: () => document.querySelector(`[data-pane-panel="${CSS.escape(guest)}"]`), authorize: () => typeof sessionLawfulTab !== 'function' || sessionLawfulTab(guest) };
          guests.set(guest, registration);
        }
        retireBinding(bindings.get(guest), 'Guest binding replaced');
        const root = rootFor(registration);
        if (!(root instanceof Element)) return null;
        const binding = { guest, registration, root, slots: [], runs: new Map(), retireCallbacks: new Set(), retryTasks: new Map(), readOwners: new Map(), retired: false };
        binding.slots = slotsFor(binding);
        rememberScaffoldSlotStructures(binding.root);
        bindings.set(guest, binding);
        if (!current(binding)) { retireBinding(binding, 'Guest is not currently admitted'); return null; }
        binding.slots.forEach(slot => {
          if (!validStates.has(slot.dataset.scaffoldState)) setPending(slot, slot.dataset.scaffoldKey || slot.dataset.scaffoldSlot, guest);
          else if (stateFor(slot) === 'fault') setFault(slot, guest, slot.dataset.scaffoldError || 'Content unavailable');
          else if (stateFor(slot) === 'pending') setPending(slot, slot.dataset.scaffoldKey || slot.dataset.scaffoldSlot, guest);
          else if (stateFor(slot) === 'ready') slot.setAttribute('aria-busy', 'false');
        });
        return binding;
      }
      function capture(guest) {
        guest = String(guest);
        let binding = bindings.get(guest);
        if (!binding && window.immortalFloor?.activeGuest === guest) binding = bind(guest);
        return current(binding) ? binding : null;
      }
      function signal(binding, readKey) { return binding?.runs.get(String(readKey || 'default'))?.controller.signal || null; }
      function onRetire(binding, callback) {
        if (typeof callback !== 'function') return () => {};
        if (!current(binding)) { try { callback({ guest: binding?.guest || null, binding, reason: 'Already retired' }); } catch (_) {} return () => {}; }
        binding.retireCallbacks.add(callback);
        return () => binding.retireCallbacks.delete(callback);
      }
      function slot(binding, key, run = null) {
        if (!current(binding, run) || !(binding.root instanceof Element)) return null;
        const value = key instanceof Element ? null : String(key);
        let target = key instanceof Element ? key : binding.root.querySelector(`[data-scaffold-slot="${CSS.escape(value)}"], [data-scaffold-key="${CSS.escape(value)}"]`);
        if (target && (!slotAllowed(binding, target) || (run && !run.slots.includes(target)))) return null;
        return target;
      }
      function markRunFailure(run, target) { if (run && target) run.failedSlots.add(target); }
      function setSlot(binding, key, state, detail = '', run = null, allowTimedOut = false) {
        if (!current(binding, run, allowTimedOut)) return false;
        const target = slot(binding, key, run);
        if (!target || !binding.root.contains(target)) return false;
        if (state === 'ready') { setReady(target); run?.failedSlots.delete(target); }
        else if (state === 'fault') { setFault(target, binding.guest, safeFailureText(detail, 'Content unavailable'), run?.key); markRunFailure(run, target); }
        else if (state === 'stale') { setStale(target, binding.guest, safeFailureText(detail, 'Showing last known data'), run?.key); markRunFailure(run, target); }
        else setPending(target, key, binding.guest, run?.key || null);
        return true;
      }
      function publish(binding, payload, options = {}) {
        const run = options.run || null;
        if (!current(binding, run)) return false;
        const selected = run ? run.slots : resolveSlots(binding, options.slots);
        selected.forEach(target => {
          if (!current(binding, run) || !binding.root.contains(target)) return;
          const key = target.dataset.scaffoldKey || target.dataset.scaffoldSlot;
          const fillerName = target.dataset.scaffoldFill || key;
          const filler = binding.registration.fillers?.[fillerName];
          if (!filler || typeof filler.present !== 'function' || typeof filler.fill !== 'function') { setFault(target, binding.guest, 'This slot is not configured', run?.key); markRunFailure(run, target); return; }
          let present = false;
          try { present = Boolean(filler.present(payload)); }
          catch (_) { setFault(target, binding.guest, 'Slot data is invalid', run?.key); markRunFailure(run, target); return; }
          if (!present) { setFault(target, binding.guest, 'Required data is unavailable in this response', run?.key); markRunFailure(run, target); return; }
          try {
            if (!current(binding, run)) return;
            filler.fill(payload, { guest: binding.guest, key, slot: target, binding, run });
            setReady(target); run?.failedSlots.delete(target);
          } catch (_) { setFault(target, binding.guest, 'Slot render failed', run?.key); markRunFailure(run, target); }
        });
        return true;
      }
      function failureReason(error) {
        if (error?.name === 'AbortError') return 'Read cancelled';
        const message = String(error?.message || '');
        if (message === 'Read timed out') return message;
        if (message === 'response-body-too-large') return 'Response exceeded the 4 MiB read limit';
        if (message === 'Read returned invalid JSON') return message;
        if (message === 'Read helper accepts bodyless GET only') return message;
        const status = message.match(/^Read failed \(HTTP (\d{3})\)$/);
        if (status) return `Read failed (HTTP ${status[1]})`;
        return 'Read failed';
      }
      function safeFailureText(detail, fallback = 'Read failed') {
        const raw = typeof detail === 'string' ? detail : String(detail?.message || '');
        if (['Content unavailable', 'Showing last known data', 'This slot is not configured', 'Slot data is invalid', 'Required data is unavailable in this response', 'Slot render failed'].includes(raw)) return raw;
        return failureReason(typeof detail === 'string' ? new Error(raw) : detail) || fallback;
      }
      function fail(binding, detail = 'Content unavailable', options = {}) {
        const run = options.run || null;
        const allowTimedOut = options.allowTimedOut === true;
        if (!current(binding, run, allowTimedOut)) return false;
        const selected = run ? run.slots : resolveSlots(binding, options.slots);
        const message = safeFailureText(detail, 'Content unavailable');
        selected.forEach(target => {
          if (!current(binding, run, allowTimedOut) || !binding.root.contains(target)) return;
          if (stateFor(target) === 'ready') setStale(target, binding.guest, message, run?.key);
          else setFault(target, binding.guest, message, run?.key);
          markRunFailure(run, target);
        });
        return true;
      }
      async function responseText(response, signal, maxBytes = DEFAULT_BODY_MAX_BYTES) {
        const declared = Number(response.headers?.get('content-length'));
        if (Number.isFinite(declared) && declared > maxBytes) {
          try { await response.body?.cancel(); } catch (_) {}
          throw new Error('response-body-too-large');
        }
        if (!response.body) return '';
        const reader = response.body.getReader();
        const chunks = [];
        let total = 0;
        const cancelReader = () => { try { void reader.cancel().catch(() => {}); } catch (_) {} };
        if (signal?.aborted) { cancelReader(); throw new DOMException('Read aborted', 'AbortError'); }
        signal?.addEventListener('abort', cancelReader, { once: true });
        try {
          while (true) {
            if (signal?.aborted) throw new DOMException('Read aborted', 'AbortError');
            const part = await reader.read();
            if (part.done) break;
            total += part.value.byteLength;
            if (total > maxBytes) { cancelReader(); throw new Error('response-body-too-large'); }
            chunks.push(part.value);
          }
        } finally {
          signal?.removeEventListener('abort', cancelReader);
          if (signal?.aborted) { try { await reader.cancel(); } catch (_) {} }
        }
        const joined = new Uint8Array(total);
        let offset = 0;
        chunks.forEach(chunk => { joined.set(chunk, offset); offset += chunk.byteLength; });
        return new TextDecoder().decode(joined);
      }
      async function fetchText(binding, url, options = {}) {
        const { run = null, signal: suppliedSignal = null, maxBytes: requestedMax = DEFAULT_BODY_MAX_BYTES, method, body, ...init } = options;
        if (method && String(method).toUpperCase() !== 'GET' || body != null) throw new Error('Read helper accepts bodyless GET only');
        const signal = suppliedSignal || run?.controller.signal;
        if (!current(binding, run)) throw new DOMException('Read retired', 'AbortError');
        const response = await fetch(url, { ...init, method: 'GET', cache: 'no-store', signal });
        if (!current(binding, run)) { try { await response.body?.cancel(); } catch (_) {} throw new DOMException('Read retired', 'AbortError'); }
        if (!response.ok) { try { await response.body?.cancel(); } catch (_) {} throw new Error(`Read failed (HTTP ${response.status})`); }
        const candidate = Number(requestedMax);
        const maxBytes = Number.isSafeInteger(candidate) && candidate > 0 ? Math.min(candidate, DEFAULT_BODY_MAX_BYTES) : DEFAULT_BODY_MAX_BYTES;
        const text = await responseText(response, signal, maxBytes);
        if (!current(binding, run)) throw new DOMException('Read retired', 'AbortError');
        return text;
      }
      async function fetchJson(binding, url, options = {}) {
        const text = await fetchText(binding, url, options);
        try { return JSON.parse(text); }
        catch (_) { throw new Error('Read returned invalid JSON'); }
      }
      function registerRetry(binding, key, task, options = {}) {
        if (!current(binding) || typeof task !== 'function') return false;
        key = String(key);
        if (!binding.retryTasks.has(key) && binding.retryTasks.size >= 128) binding.retryTasks.delete(binding.retryTasks.keys().next().value);
        binding.retryTasks.set(key, { task, options });
        return true;
      }
      function clearRetry(binding, key) {
        if (!binding || binding.retired) return false;
        return binding.retryTasks.delete(String(key));
      }
      function retry(binding, key) {
        if (!current(binding)) return Promise.resolve(false);
        const retryTask = binding.retryTasks.get(String(key));
        if (!retryTask) return Promise.resolve(false);
        return run(binding, String(key), retryTask.task, { ...retryTask.options, supersede: false });
      }
      function run(guestOrBinding, readKey, task, options = {}) {
        const binding = typeof guestOrBinding === 'object' ? guestOrBinding : capture(guestOrBinding);
        if (!current(binding) || typeof task !== 'function') return Promise.resolve(false);
        const key = String(readKey || 'default');
        const existing = binding.runs.get(key);
        if (existing && !options.supersede) return existing.promise;
        if (existing) stopRun(binding, existing, 'Read superseded');
        const controller = new AbortController();
        const timeoutCandidate = Number(options.timeoutMs);
        const timeoutMs = Number.isSafeInteger(timeoutCandidate) && timeoutCandidate > 0 ? Math.min(timeoutCandidate, 30000) : DEFAULT_READ_TIMEOUT_MS;
        const slowCandidate = Number(options.slowMs);
        const slowMs = Number.isSafeInteger(slowCandidate) && slowCandidate > 0 ? Math.min(slowCandidate, Math.max(1, timeoutMs - 1)) : Math.min(DEFAULT_SLOW_READ_MS, Math.max(1, timeoutMs - 1));
        const slots = resolveSlots(binding, options.slots);
        const upstream = options.signal || null;
        if (upstream?.aborted) return Promise.resolve(false);
        const record = { key, controller, timer: null, slowTimer: null, retired: false, timedOut: false, promise: null, slots, failedSlots: new Set(), upstream, relay: null, cancel: null, failureNotified: false };
        const retryOptions = { slots: Array.isArray(options.slots) ? options.slots.slice() : undefined, timeoutMs, slowMs, onFailure: options.onFailure, onRetire: options.onRetire };
        let retryTask = null;
        if (options.retry !== false) {
          if (!registerRetry(binding, key, task, retryOptions)) return Promise.resolve(false);
          retryTask = binding.retryTasks.get(key);
        }
        binding.runs.set(key, record);
        slots.forEach(target => setPending(target, target.dataset.scaffoldKey || target.dataset.scaffoldSlot, binding.guest, key));
        record.relay = () => { record.externalAbort = true; controller.abort(); record.cancel?.(new DOMException('Read cancelled', 'AbortError')); };
        if (upstream) { if (upstream.aborted) record.relay(); else upstream.addEventListener('abort', record.relay, { once: true }); }
        let rejectCancel;
        const cancelled = new Promise((_, reject) => { rejectCancel = reject; });
        record.cancel = rejectCancel;
        const reportFailure = (error, allowTimedOut = false) => {
          if (record.failureNotified || !current(binding, record, allowTimedOut)) return;
          record.failureNotified = true;
          const reason = failureReason(error);
          fail(binding, reason, { run: record, allowTimedOut });
          try {
            options.onFailure?.({ guest: binding.guest, readKey: key, reason, errorName: String(error?.name || 'Error'), slots: record.slots.slice(), setSlot: (target, state = 'fault', detail = reason) => setSlot(binding, target, state, detail, record, allowTimedOut), current: () => current(binding, record, allowTimedOut) });
          } catch (_) {}
        };
        const context = {
          binding, run: record, key, guest: binding.guest, slots: slots.slice(), signal: controller.signal,
          current: () => current(binding, record),
          slot: keyName => slot(binding, keyName, record),
          setSlot: (target, state, detail = '') => setSlot(binding, target, state, detail, record),
          ready: target => setSlot(binding, target, 'ready', '', record),
          fail: detail => { if (!current(binding, record)) return false; reportFailure(detail instanceof Error ? detail : new Error(typeof detail === 'string' ? detail : 'Read failed')); return true; },
          publish: payload => publish(binding, payload, { run: record }),
          fetchJson: (url, fetchOptions = {}) => fetchJson(binding, url, { ...fetchOptions, run: record }),
          fetchText: (url, fetchOptions = {}) => fetchText(binding, url, { ...fetchOptions, run: record })
        };
        const operation = Promise.resolve().then(() => current(binding, record) ? task(context) : false);
        const deadline = new Promise((_, reject) => {
          record.timer = window.setTimeout(() => { record.timedOut = true; controller.abort(); reject(new Error('Read timed out')); }, timeoutMs);
          record.slowTimer = window.setTimeout(() => {
            if (!current(binding, record)) return;
            slots.forEach(target => {
              if (!binding.root.contains(target)) return;
              if (stateFor(target) === 'ready') setPending(target, target.dataset.scaffoldKey || target.dataset.scaffoldSlot, binding.guest, key);
              else statusNode(target, 'pending', binding.guest, key, `Still waiting for ${target.dataset.scaffoldPlaceholder || target.dataset.scaffoldSlot || 'this region'}.`, true);
            });
          }, slowMs);
        });
        record.promise = Promise.race([operation, deadline, cancelled]).then(result => {
          if (!current(binding, record)) return false;
          if (result === false) { reportFailure(new Error('Read failed')); return false; }
          if (!record.failedSlots.size && binding.retryTasks.get(key) === retryTask) binding.retryTasks.delete(key);
          return result === undefined ? true : result;
        }).catch(error => {
          reportFailure(error, record.timedOut || record.externalAbort);
          return false;
        }).finally(() => {
          window.clearTimeout(record.timer); window.clearTimeout(record.slowTimer);
          record.timer = null; record.slowTimer = null;
          if (upstream) upstream.removeEventListener('abort', record.relay);
          if (binding.runs.get(key) === record) binding.runs.delete(key);
          record.slots = []; record.cancel = null; record.relay = null;
        });
        return record.promise;
      }
      function read(guest, options = {}) {
        const binding = capture(guest);
        if (!binding || typeof binding.registration.read !== 'function') return Promise.resolve(false);
        return run(binding, options.readKey || 'default', async context => {
          const payload = await binding.registration.read({ ...context, guest: binding.guest, root: binding.root });
          if (!context.current()) return false;
          return publish(binding, payload, { run: context.run });
        }, { ...options, timeoutMs: options.timeoutMs || binding.registration.timeoutMs });
      }
      function rebind(guest) { return bind(guest); }
      function retire(guestOrBinding, reason = 'Guest retired') {
        const binding = typeof guestOrBinding === 'object' ? guestOrBinding : bindings.get(guestOrBinding);
        return retireBinding(binding, reason);
      }
      document.addEventListener('click', event => {
        const target = event.target instanceof Element ? event.target : null;
        const retryButton = target?.closest('[data-scaffold-retry]');
        const htmxRetryButton = target?.closest('[data-native-read-retry]');
        if (retryButton) {
          const binding = capture(retryButton.dataset.scaffoldRetry);
          if (!binding || !retryButton.dataset.scaffoldReadKey) return;
          event.preventDefault();
          retry(binding, retryButton.dataset.scaffoldReadKey).catch(() => {});
          return;
        }
        if (htmxRetryButton) {
          const binding = capture(htmxRetryButton.dataset.nativeReadGuest);
          if (!binding || !htmxRetryButton.dataset.nativeReadRetry) return;
          event.preventDefault();
          retry(binding, htmxRetryButton.dataset.nativeReadRetry).catch(() => {});
        }
      });
      return Object.freeze({ registerGuest, hasGuest, bind, capture, current, signal, onRetire, slot, setSlot, publish, fail, run, read, retry, registerRetry, clearRetry, rebind, retire, fetchJson, fetchText });
    })();
    window.scaffoldRegistry = scaffoldRegistry;
    const elementsChangedConsumers = new Map();
    function consumeElementsChanged({ paneId, route, target, afterReplace = () => {} }) {
      const pull = () => {
        const binding = scaffoldRegistry.capture(paneId);
        if (!binding) return Promise.resolve(false);
        return scaffoldRegistry.run(binding, `structure-${paneId}`, async context => {
          const fragment = target();
          if (!fragment || !context.current() || !binding.root.contains(fragment)) return false;
          const html = await context.fetchText(route);
          if (!context.current() || !binding.root.contains(fragment)) return false;
          const changed = morphLivePane(fragment, html);
          if (!context.current()) return false;
          if (changed) {
            afterReplace(fragment, binding, context);
            applyAdminDomState();
          }
          fragment.removeAttribute('data-structure-read-error');
          fragment.removeAttribute('data-structure-read-stale');
          return { changed };
        }, {
          slots: [], supersede: false,
          onFailure: ({ current, reason }) => {
            const fragment = target();
            if (!current() || !fragment || !binding.root?.contains(fragment)) return;
            fragment.dataset.structureReadStale = 'true';
            fragment.dataset.structureReadError = reason || 'Structure refresh failed';
          }
        }).then(result => result && typeof result === 'object' && Object.prototype.hasOwnProperty.call(result, 'changed') ? result.changed : result);
      };
      elementsChangedConsumers.set(paneId, pull);
      return pull;
    }
    window.consumeElementsChanged = consumeElementsChanged;
    const nativePaneContracts = Object.freeze({
      admin: { root: '[data-admin-viewport="admin"]', required: ['[data-admin-action-strip]', '[data-admin-service-controls]'] },
      portals: { root: '[data-portals-viewport]', required: ['[data-portals-grid]'] },
      upload: { root: '[data-upload-viewport]', required: ['[data-upload-tree]'] },
      stats: { root: '[data-stats-viewport]', required: ['[data-stat-element-id]'] },
      backblaze: { root: '[data-backblaze-tablet]', required: ['[data-backblaze-ribbon]', '[data-backblaze-panel]'] },
      'wake-on-lan': { root: '[data-wake-on-lan-tablet]', required: ['[data-wake-on-lan-rows]'] },
      test: { root: '[data-native-stock-test="true"]', required: ['.ui-tab-group', '.test-tablet-content'], slots: [] },
      linker: {
        root: '#linker-fragment[data-linker-state][data-linker-path]',
        swap: 'outerHTML',
        preserveOuterRootAttributes: ['data-linker-readback', 'hx-get', 'hx-trigger', 'hx-target', 'hx-swap', 'hx-request'],
        required: ['[data-linker-tree]', '.linker-list', '[data-linker-read-status]']
      },
      dhcp: { root: '[data-dhcp-tablet]', required: ['[data-dhcp-refresh]', '[data-identity-roster-host="dhcp"]'] },
      firewall: { root: '[data-firewall-tablet]', required: ['[data-firewall-refresh]', '[data-firewall-observed]', '[data-firewall-children]'] },
      unbound: { root: '[data-dns-tablet]', required: ['[data-dns-state]', '[data-dns-names]', '[data-dns-resolver-status]'] }
    });
    function nativePanePayload(id, fragment) {
      const contract = nativePaneContracts[id];
      if (!contract || !fragment?.querySelector) return null;
      const root = fragment.querySelector(contract.root);
      if (!root || contract.required?.some(selector => !root.querySelector(selector))) return null;
      return root;
    }
    function nativePaneMorphTarget(pane, id) {
      const contract = nativePaneContracts[id];
      return contract?.swap === 'outerHTML' ? pane?.querySelector(contract.root) || null : pane;
    }
    function healthyCartridgePayload(id, fragment) {
      const host = fragment.querySelector(`.cartridge-viewport[data-cartridge-id="${CSS.escape(id)}"]`);
      if (host) {
        const iframe = host.querySelector(':scope > iframe');
        if (iframe) return Boolean(iframe.getAttribute('src') && iframe.getAttribute('sandbox') === 'allow-scripts allow-same-origin allow-forms');
        return host.hasAttribute('hx-get') && host.getAttribute('hx-trigger') === 'load' && host.getAttribute('hx-swap') === 'innerHTML';
      }
      if (fragment.querySelector('iframe')) return false;
      return Array.from(fragment.children).some(root => {
        if (['SCRIPT', 'STYLE', 'LINK', 'TEMPLATE'].includes(root.tagName)) return false;
        return root.childElementCount > 0 || Boolean(root.textContent.trim());
      });
    }
"####
}
