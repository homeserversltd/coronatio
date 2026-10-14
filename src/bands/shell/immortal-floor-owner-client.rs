fn shell_immortal_floor_owner_client() -> &'static str {
    r####"    const immortalFloorStates = Object.freeze(['BootFloor', 'Seated', 'GuestRevolution', 'BareFloor']);
    const immortalFloor = (() => {
      let state = 'BootFloor'; let generation = 0; let activeGuest = null;
      let crossingGuest = null;
      let visibilityEpoch = 0; let pageVisible = document.visibilityState === 'visible';
      let activeIntent = null; let interruptedIntent = null;

      const ready = new Promise(resolve => requestAnimationFrame(() => {
        if (immortalFloorShell && tabBar && panes.length) {
          immortalFloorShell.dataset.startupPhase = 'Ready';
          resolve(true);
        } else resolve(false);
      }));
      function expose(next, detail = '') {
        if (!immortalFloorStates.includes(next)) throw new Error('Invalid Immortal Floor state: ' + next);
        state = next;
        if (next !== 'Seated') retireAdminDiskSnapshot();
        document.documentElement.dataset.immortalFloorState = next;
        const activePanes = next === 'GuestRevolution' ? panes.filter(pane => pane.classList.contains('active')) : [];
        const retainedPane = activeGuest && activePanes.length === 1 ? activePanes[0] : null;
        const retainedHealthyGuest = retainedPane?.dataset.panePanel === activeGuest && retainedPane.parentElement === immortalFloorGuestSlot && healthyStructure(retainedPane, activeGuest, retainedPane.innerHTML) ? activeGuest : null;
        if (immortalFloorShell) {
          immortalFloorShell.dataset.immortalFloorState = next;
          if (retainedHealthyGuest) immortalFloorShell.dataset.retainedHealthyGuest = retainedHealthyGuest;
          else delete immortalFloorShell.dataset.retainedHealthyGuest;
        }
        if (immortalFloorGuestSlot) immortalFloorGuestSlot.dataset.slotEmpty = String(!activeGuest && next !== 'Seated');
        const message = immortalFloorShell?.querySelector('[data-immortal-floor-message]');
        if (message) message.textContent = detail || ({ BootFloor: 'Preparing your controls…', GuestRevolution: 'Changing view…', BareFloor: 'Controls remain available. Choose a tab to try again.', Seated: '' }[next]);
      }
      function emptySlot() {
        closeViewportStreamFamily();
        if (activeGuest) scaffoldRegistry.retire(activeGuest);
        panes.forEach(pane => {
          pane.classList.remove('active', 'immortal-floor-enter');
          pane.setAttribute('aria-hidden', 'true');
        });
        activeGuest = null;
        if (immortalFloorShell) delete immortalFloorShell.dataset.retainedHealthyGuest;
        if (immortalFloorGuestSlot) immortalFloorGuestSlot.dataset.slotEmpty = 'true';
      }
      let admissionRequest = null;
      const admissionOwners = new WeakMap();
      const admissionOobMarker = 'crown-htmx-oob-owner:';
      const admissionOobSelector = '[hx-swap-oob], [data-hx-swap-oob]';
      let nextAdmissionOobId = 0;
      function ownsVisibleActivation(crossing, id, epoch) {
        return crossing === generation && crossingGuest === id && epoch === visibilityEpoch && document.visibilityState === 'visible';
      }
      function intentOptions(options) {
        const projection = {};
        if (typeof options?.refresh === 'boolean') projection.refresh = options.refresh;
        if (typeof options?.detail === 'string') projection.detail = options.detail;
        return projection;
      }
      function ownsVisibleAdmission(record) {
        return Boolean(record && record === admissionRequest && !record.retired && !record.settled
          && record.crossing === generation && record.visibilityEpoch === visibilityEpoch
          && crossingGuest === record.id && document.visibilityState === 'visible');
      }
      function admissionVisitNodes(root, visit) {
        visit(root);
        if (root.nodeType === Node.ELEMENT_NODE && root.tagName === 'TEMPLATE') {
          admissionVisitNodes(root.content, visit);
          return;
        }
        Array.from(root.childNodes || []).forEach(node => admissionVisitNodes(node, visit));
      }
      function admissionPrimaryFragment(fragment) {
        let hasOutOfBand = false;
        function removeFrom(parent) {
          let removed = false;
          Array.from(parent.childNodes || []).forEach(node => {
            if (node.nodeType === Node.ELEMENT_NODE && node.matches(admissionOobSelector)) {
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
        const primary = fragment.cloneNode(true);
        hasOutOfBand = removeFrom(primary);
        return { primary, hasOutOfBand };
      }
      function admissionTagOutOfBand(html, token) {
        const template = document.createElement('template');
        template.innerHTML = String(html || '');
        let tagged = false;
        admissionVisitNodes(template.content, node => {
          if (node.nodeType === Node.ELEMENT_NODE && node.matches(admissionOobSelector)) {
            node.insertBefore(document.createComment(`${admissionOobMarker}${token}`), node.firstChild);
            tagged = true;
          }
        });
        return tagged ? template.innerHTML : String(html || '');
      }
      function admissionOobRequestToken() {
        const nonce = window.crypto?.randomUUID?.() || `${Date.now().toString(36)}-${Math.random().toString(36).slice(2)}`;
        return `admission-oob-${++nextAdmissionOobId}-${nonce}`;
      }
      function admissionStripOobMarkers(fragment) {
        const tokens = new Set();
        if (!fragment) return tokens;
        admissionVisitNodes(fragment, node => {
          if (node.nodeType !== Node.COMMENT_NODE || !node.data.startsWith(admissionOobMarker)) return;
          const token = node.data.slice(admissionOobMarker.length);
          if (!token.startsWith('admission-oob-')) return;
          tokens.add(token);
          node.remove();
        });
        return tokens;
      }
      function retireAdmissionRequest(record = admissionRequest, reason = 'Admission retired', { structuralFault = false } = {}) {
        if (!record || record.retired) return false;
        record.retired = true;
        if (structuralFault) record.structuralFault = true;
        if (admissionRequest === record) admissionRequest = null;
        record.cleanup?.();
        record.pendingHtml = null;
        record.oobToken = null;
        const error = structuralFault ? Object.assign(new Error(reason), { structuralFault: true }) : new DOMException(reason, 'AbortError');
        try { record.cancel?.(error); } catch (_) {}
        try { if (record.xhr && record.xhr.readyState !== XMLHttpRequest.DONE) record.xhr.abort(); } catch (_) {}
        return true;
      }
      document.body.addEventListener('htmx:beforeRequest', event => {
        const record = admissionRequest;
        const detail = event.detail || {};
        if (!ownsVisibleAdmission(record) || record.tab !== detail.elt || detail.target !== record.pane || detail.xhr == null) return;
        record.xhr = detail.xhr;
        record.target = detail.target;
        try { record.xhr.timeout = record.timeoutMs; } catch (_) {}
        try { record.xhr.addEventListener('loadstart', () => { record.sent = true; }, { once: true }); } catch (_) {}
        admissionOwners.set(record.xhr, record);
      });
      document.body.addEventListener('htmx:oobBeforeSwap', event => {
        const tokens = admissionStripOobMarkers(event.detail?.fragment);
        if (!tokens.size) return;
        const token = tokens.size === 1 ? tokens.values().next().value : '';
        const owner = admissionRequest;
        if (tokens.size !== 1 || !owner || owner.oobToken !== token || !ownsVisibleAdmission(owner) || !isAdmissionRequest(owner.xhr)) {
          event.detail.shouldSwap = false;
        }
      });
      document.body.addEventListener('htmx:beforeSwap', event => {
        const xhr = event.detail?.xhr;
        const owner = xhr && admissionOwners.get(xhr);
        if (!owner) return;
        if (!ownsVisibleAdmission(owner)) {
          event.detail.shouldSwap = false;
          return;
        }
        const html = String(event.detail.serverResponse || '');
        const payload = inspectAdmissionPayload(owner.pane, owner.id, html);
        if (!payload.ok) {
          owner.failed = true;
          owner.structuralFault = payload.structuralFault;
          event.detail.shouldSwap = false;
          return;
        }
        if (Number(owner.xhr?.status || 0) < 200 || Number(owner.xhr?.status || 0) >= 300) {
          owner.failed = true;
          event.detail.shouldSwap = false;
          return;
        }
        const sourceOwned = Object.prototype.hasOwnProperty.call(nativePaneContracts, owner.id);
        owner.oobToken = null;
        if (sourceOwned && payload.hasOutOfBand) {
          owner.oobToken = admissionOobRequestToken();
          event.detail.serverResponse = admissionTagOutOfBand(html, owner.oobToken);
        }
        if (owner.sameGuestRefresh) {
          owner.pendingHtml = payload.primaryHtml;
          owner.accepted = true;
          if (sourceOwned) event.detail.swapOverride = 'none';
          else event.detail.shouldSwap = false;
        }
      });
      function panelIdFromHtmxEvent(event) {
        const panel = panelFromHtmxEvent(event);
        return panel instanceof HTMLElement ? (panel.dataset.viewPanel || panel.dataset.panePanel || '') : '';
      }
      function inspectAdmissionPayload(pane, id, html) {
        const source = String(html || '');
        if (!pane?.isConnected || !source.trim()) return { ok: false, structuralFault: true, reason: 'admission-structure-invalid' };
        const sourceOwned = Object.prototype.hasOwnProperty.call(nativePaneContracts, id);
        const template = document.createElement('template');
        template.innerHTML = source;
        if (template.content.querySelector('script')) return { ok: false, structuralFault: true, reason: 'admission-forbidden-script' };
        if (template.content.querySelector('[data-cartridge-fault], [data-cartridge-fault-kind], [data-service-card-fault-kind], [data-admin-membrane-refusal]')) {
          return { ok: false, structuralFault: true, reason: 'admission-refused' };
        }
        const primary = sourceOwned ? admissionPrimaryFragment(template.content) : { primary: template.content, hasOutOfBand: false };
        if (primary.primary.childElementCount === 0) return { ok: false, structuralFault: true, reason: 'admission-structure-invalid' };
        const valid = sourceOwned ? Boolean(nativePanePayload(id, primary.primary)) : healthyCartridgePayload(id, template.content);
        if (!valid) return { ok: false, structuralFault: true, reason: 'admission-structure-invalid' };
        let primaryHtml = source;
        if (sourceOwned && primary.hasOutOfBand) {
          const primaryTemplate = document.createElement('template');
          primaryTemplate.content.append(primary.primary);
          primaryHtml = primaryTemplate.innerHTML;
        }
        return { ok: true, structuralFault: false, primaryHtml, hasOutOfBand: sourceOwned && primary.hasOutOfBand };
      }
      function healthyStructure(pane, id, html = pane?.innerHTML) {
        return Boolean(pane?.isConnected && pane.childElementCount > 0 && inspectAdmissionPayload(pane, id, html).ok);
      }
      async function admitFreshGuest(id, crossing, sameGuestRefresh = false, activationEpoch) {
        if (!ownsVisibleActivation(crossing, id, activationEpoch)) throw new DOMException('Document hidden', 'AbortError');
        const pane = panes.find(candidate => candidate.dataset.panePanel === id);
        const tab = tabs.find(candidate => candidate.dataset.pane === id);
        if (!pane) {
          const error = new Error('admission-structure-invalid');
          error.structuralFault = true;
          throw error;
        }
        if (!tab?.getAttribute('hx-get') || typeof window.htmx?.trigger !== 'function') {
          const error = new Error('admission-actuator-missing');
          error.structuralFault = true;
          throw error;
        }
        const timeoutCandidate = Number(tab.dataset.admissionTimeoutMs);
        const admissionTimeoutMs = Number.isSafeInteger(timeoutCandidate) && timeoutCandidate > 0 ? Math.min(timeoutCandidate, 30000) : 1500;
        if (admissionRequest) retireAdmissionRequest(admissionRequest, 'Admission superseded');
        const record = { id, tab, pane, crossing, visibilityEpoch: activationEpoch, sameGuestRefresh, xhr: null, target: null, sent: false, timer: null, timeoutMs: admissionTimeoutMs, accepted: false, failed: false, structuralFault: false, retired: false, settled: false, pendingHtml: null, oobToken: null, cleanup: null, cancel: null, resolve: null, reject: null };
        admissionRequest = record;
        await new Promise((resolve, reject) => {
          record.resolve = resolve; record.reject = reject;
          const cleanup = () => {
            if (record.cleaned) return;
            record.cleaned = true;
            document.body.removeEventListener('htmx:afterSwap', afterSwap);
            document.body.removeEventListener('htmx:afterRequest', afterRequest);
            document.body.removeEventListener('htmx:timeout', failed);
            document.body.removeEventListener('htmx:responseError', failed);
            document.body.removeEventListener('htmx:sendError', failed);
            if (record.timer !== null) window.clearTimeout(record.timer);
            record.timer = null;
            if (admissionRequest === record) admissionRequest = null;
            record.pendingHtml = null;
            record.oobToken = null;
            record.cleanup = null;
          };
          const settle = (error = null, value = undefined) => {
            if (record.settled) return false;
            record.settled = true;
            cleanup();
            const resolveRecord = record.resolve; const rejectRecord = record.reject;
            record.resolve = null; record.reject = null; record.cancel = null;
            if (error) rejectRecord?.(error); else resolveRecord?.(value);
            return true;
          };
          record.cleanup = cleanup;
          record.cancel = error => settle(error || new DOMException('Admission retired', 'AbortError'));
          const isOwnRequest = event => event.detail?.xhr && event.detail.xhr === record.xhr && event.detail.target === pane && ownsVisibleAdmission(record);
          const afterSwap = event => {
            if (!isOwnRequest(event)) return;
            if (!healthyStructure(pane, id, pane.innerHTML)) {
              record.structuralFault = true;
              const error = new Error('admission-structure-invalid');
              error.structuralFault = true;
              settle(error);
              return;
            }
            record.accepted = true;
            pane.dataset.nativeStructureReady = 'true';
            if (!sameGuestRefresh) settle(null, pane);
          };
          const afterRequest = async event => {
            if (!isOwnRequest(event)) return;
            if (record.failed) {
              const error = new Error(record.structuralFault ? 'admission-structure-invalid' : 'admission-fault');
              error.structuralFault = record.structuralFault;
              settle(error);
              return;
            }
            if (!sameGuestRefresh) return;
            if (!record.accepted || !record.pendingHtml) { settle(new Error('admission-fault')); return; }
            await new Promise(resolveFrame => requestAnimationFrame(() => requestAnimationFrame(resolveFrame)));
            if (!ownsVisibleAdmission(record)) { settle(new Error('admission-superseded')); return; }
            const pendingHtml = record.pendingHtml;
            record.pendingHtml = null;
            const morphTarget = nativePaneMorphTarget(pane, id);
            if (!morphTarget) {
              const error = new Error('admission-structure-invalid');
              error.structuralFault = true;
              settle(error);
              return;
            }
            const insertedNodes = [];
            morphLivePane(morphTarget, pendingHtml, {
              outerRoot: nativePaneContracts[id]?.swap === 'outerHTML',
              preserveOuterRootAttributes: nativePaneContracts[id]?.preserveOuterRootAttributes,
              insertedNodes
            });
            if (!healthyStructure(pane, id, pane.innerHTML)) {
              const error = new Error('admission-structure-invalid');
              error.structuralFault = true;
              settle(error);
              return;
            }
            insertedNodes.forEach(node => {
              if (!ownsVisibleAdmission(record) || !node.isConnected || !pane.contains(node)) return;
              try { window.htmx.process(node); } catch (_) {}
            });
            pane.dataset.nativeStructureReady = 'true';
            settle(null, pane);
          };
          const failed = event => {
            if (!record.xhr || event.detail?.xhr !== record.xhr || (event.detail?.target && event.detail.target !== pane)) return;
            const body = record.xhr.responseText || '';
            const payload = body ? inspectAdmissionPayload(pane, id, body) : { ok: false, structuralFault: false };
            const actuatorMissing = !record.sent;
            record.structuralFault = actuatorMissing || payload.structuralFault === true;
            const reason = actuatorMissing ? 'admission-actuator-missing'
              : event.type === 'htmx:timeout' ? 'admission-timeout'
              : record.structuralFault ? payload.reason : 'admission-fault';
            const error = new Error(reason);
            error.structuralFault = record.structuralFault;
            settle(error);
          };
          document.body.addEventListener('htmx:afterSwap', afterSwap);
          document.body.addEventListener('htmx:afterRequest', afterRequest);
          document.body.addEventListener('htmx:timeout', failed);
          document.body.addEventListener('htmx:responseError', failed);
          document.body.addEventListener('htmx:sendError', failed);
          record.timer = window.setTimeout(() => {
            if (!record.sent) {
              retireAdmissionRequest(record, 'admission-actuator-missing', { structuralFault: true });
              return;
            }
            retireAdmissionRequest(record, 'admission-timeout');
          }, admissionTimeoutMs);
          try { window.htmx.trigger(tab, 'immortal-floor-admit'); }
          catch (_) { retireAdmissionRequest(record, 'admission-actuator-missing', { structuralFault: true }); }
        });
        return pane;
      }
      async function seatGuest(id, crossing, epoch, detail = '') {
        const pane = panes.find(candidate => candidate.dataset.panePanel === id);
        if (!pane) return false;
        if (!ownsVisibleActivation(crossing, id, epoch)) return false;
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        if (!ownsVisibleActivation(crossing, id, epoch)) return false;
        if (!healthyStructure(pane, id, pane.innerHTML)) {
          const error = new Error('admission-structure-invalid');
          error.structuralFault = true;
          throw error;
        }
        tabs.forEach(tab => { const active = tab.dataset.pane === id; tab.setAttribute('aria-selected', String(active)); tab.classList.toggle('active', active); });
        panes.forEach(candidate => { const active = candidate === pane; candidate.classList.toggle('active', active); candidate.classList.toggle('immortal-floor-enter', active); candidate.setAttribute('aria-hidden', String(!active)); });
        if (activeGuest && activeGuest !== id) scaffoldRegistry.retire(activeGuest);
        activeGuest = id; crossingGuest = null; expose('Seated', detail); applyAdminDomState(); applyTabBarVisibility();
        scaffoldRegistry.bind(id);
        if (id === 'linker') window.linkerCursorClient?.refresh(pane);
        if (id === 'admin' && window.htmx) {
          if (adminDocumentPatchPendingHydration) {
            adminDocumentPatchPendingHydration = false;
            window.htmx.process(pane);
          }
          pane.querySelectorAll('[data-service-card]').forEach(card => window.htmx.trigger(card, 'admin-ready'));
        }
        if (id === 'linker' && window.htmx) {
          const readback = pane.querySelector('[data-linker-readback]');
          if (readback) { window.htmx.process(readback); window.htmx.trigger(readback, 'linker-readback'); }
        }
        if (id === 'portals' && window.htmx) {
          const grid = pane.querySelector('[data-portals-grid]');
          if (grid) { window.htmx.process(grid); window.htmx.trigger(grid, 'portals-admitted'); }
        }
        if (id === 'upload' && window.htmx) {
          const treeRead = pane.querySelector('[data-upload-tree-read-status]');
          if (treeRead) { window.htmx.process(treeRead); window.htmx.trigger(treeRead, 'upload-tree-admitted'); }
        }
        reconcileViewportStreamFamily();
        return true;
      }
      async function activate(requested, options = {}) {
        const selected = lawfulPaneCandidate(requested);
        if (document.visibilityState !== 'visible') {
          if (state === 'Seated' && activeGuest === selected && !options.refresh) return true;
          interruptedIntent = { id: selected, options: intentOptions(options) };
          return false;
        }
        if (state === 'Seated' && activeGuest === selected && !options.refresh) {
          crossingGuest = null;
          interruptedIntent = null;
          applyAdminDomState();
          return true;
        }
        const outgoing = activeGuest;
        const hadHealthyFloor = state === 'Seated' && Boolean(outgoing);
        const crossing = ++generation;
        const epoch = visibilityEpoch;
        const intent = { id: selected, options: intentOptions(options), crossing, epoch };
        interruptedIntent = null;
        activeIntent = intent;
        if (admissionRequest) retireAdmissionRequest(admissionRequest);
        if (outgoing) scaffoldRegistry.retire(outgoing);
        closeViewportStreamFamily();
        stopPortalCurrentnessCadence();
        crossingGuest = selected;
        const readyNow = await ready;
        if (!readyNow) {
          if (crossing === generation) { emptySlot(); expose('BareFloor'); }
          if (activeIntent === intent && document.visibilityState === 'visible') activeIntent = null;
          return false;
        }
        if (!ownsVisibleActivation(crossing, selected, epoch)) {
          if (activeIntent === intent && document.visibilityState === 'visible') activeIntent = null;
          return false;
        }
        expose('GuestRevolution');
        try {
          const sameGuestRefresh = hadHealthyFloor && outgoing === selected;
          if (!ownsVisibleActivation(crossing, selected, epoch)) return false;
          await admitFreshGuest(selected, crossing, sameGuestRefresh, epoch);
          if (!ownsVisibleActivation(crossing, selected, epoch)) return false;
          if (!await seatGuest(selected, crossing, epoch, options.detail)) {
            if (!ownsVisibleActivation(crossing, selected, epoch)) return false;
            throw new Error('guest-structure-unhealthy');
          }
          return true;
        } catch (error) {
          if (!ownsVisibleActivation(crossing, selected, epoch)) return false;
          retireAdmissionRequest();
          if (error?.structuralFault) {
            fault(error?.message || 'admission-structure-invalid');
          } else if (hadHealthyFloor && activeGuest === outgoing) {
            crossingGuest = null;
            const retainedPane = panes.find(p=>p.dataset.panePanel===outgoing);
            if(retainedPane)retainedPane.dataset.viewportFaulted='false';
            expose('Seated', 'The requested view did not refresh. The last healthy view remains open.');
            applyAdminDomState();
            applyTabBarVisibility();
            reconcileViewportStreamFamily();
          } else {
            fault(error?.message || 'admission-fault');
          }
          return false;
        } finally {
          if (activeIntent === intent && document.visibilityState === 'visible') activeIntent = null;
        }
      }
      function visibilityChanged() {
        const visible = document.visibilityState === 'visible';
        if (visible === pageVisible) return false;
        pageVisible = visible;
        if (!visible) {
          if (activeIntent && activeIntent.crossing === generation && activeIntent.epoch === visibilityEpoch
              && crossingGuest === activeIntent.id) {
            interruptedIntent = { id: activeIntent.id, options: intentOptions(activeIntent.options) };
          }
          visibilityEpoch += 1;
          generation += 1;
          if (admissionRequest) retireAdmissionRequest(admissionRequest, 'Document hidden');
          activeIntent = null;
          crossingGuest = null;
          if (activeGuest) {
            const pane = panes.find(candidate => candidate.dataset.panePanel === activeGuest);
            if (state === 'Seated' || healthyStructure(pane, activeGuest, pane?.innerHTML)) {
              if (state !== 'Seated') {
                expose('Seated');
                applyAdminDomState();
                applyTabBarVisibility();
              }
            } else {
              emptySlot();
              expose('BareFloor');
            }
          } else {
            emptySlot();
            expose('BareFloor');
          }
          return true;
        }
        visibilityEpoch += 1;
        const intent = interruptedIntent;
        interruptedIntent = null;
        if (intent) {
          const selected = lawfulPaneCandidate(intent.id);
          const alreadySeated = state === 'Seated' && activeGuest === selected && !intent.options.refresh;
          void activate(selected, intent.options);
          return alreadySeated;
        }
        return true;
      }
      function fault(kind = 'guest-fault') {
        if (admissionRequest) retireAdmissionRequest(admissionRequest, 'Admission fault');
        generation += 1;
        emptySlot();
        expose('BareFloor', 'This view could not open. Choose a tab to try again.');
        console.warn('[coronatio] view failed to open', { pane: crossingGuest, guest: crossingGuest, kind });
        crossingGuest = null;
        document.documentElement.dataset.immortalFloorFault = kind;
      }
      function admissionOwner(xhr) {
        const record = xhr && admissionOwners.get(xhr);
        if (!ownsVisibleAdmission(record) || record.xhr !== xhr || record.target !== record.pane || !record.pane?.isConnected) return null;
        return { guest: record.id, pane: record.pane };
      }
      function isAdmissionRequest(xhr) { return admissionOwner(xhr) !== null; }
      function failAdmission(xhr, kind = 'admission-fault') {
        const record = xhr && admissionOwners.get(xhr);
        if (!record || !admissionOwner(xhr)) return false;
        const coldFault = !activeGuest;
        retireAdmissionRequest(record, kind);
        // activate() owns the cold-versus-refresh decision.  Do not empty the
        // floor here: it may still be showing a healthy outgoing guest.
        return coldFault;
      }
      function adminLost() {
        if (interruptedIntent?.id === 'admin') interruptedIntent = null;
        if (activeIntent?.id === 'admin') activeIntent = null;
        scaffoldRegistry.retire('admin', 'Admin role retired');
        if (crossingGuest === 'admin') {
          generation += 1;
          retireAdmissionRequest(admissionRequest, 'Admin role retired');
          crossingGuest = null;
          if (activeGuest === 'admin') {
            emptySlot();
            expose('BareFloor', 'Admin access ended. Choose a permitted tab.');
          } else if (activeGuest) {
            expose('Seated', 'Admin access ended.');
            applyAdminDomState(); applyTabBarVisibility();
          } else {
            emptySlot();
            expose('BareFloor', 'Admin access ended. Choose a permitted tab.');
          }
          return true;
        }
        if (activeGuest === 'admin') {
          emptySlot();
          if (crossingGuest && admissionRequest) expose('GuestRevolution', 'Admin access ended. Opening the selected view…');
          else {
            generation += 1;
            retireAdmissionRequest(admissionRequest, 'Admin role retired');
            crossingGuest = null;
            expose('BareFloor', 'Admin access ended. Choose a permitted tab.');
          }
          return true;
        }
        return false;
      }
      expose('BootFloor');
      return Object.freeze({ activate, fault, admissionOwner, isAdmissionRequest, failAdmission, adminLost, visibilityChanged, get visibilityEpoch() { return visibilityEpoch; }, get state() { return state; }, get activeGuest() { return activeGuest; } });
    })();
    window.immortalFloor = immortalFloor;
    window.getImmortalFloorState = () => immortalFloor.state;
    function showPane(id, options) { return immortalFloor.activate(id, options); }
"####
}
