#!/usr/bin/env node
/**
 * Dependency-free Chromium/CDP proof for CORONATIO-BUTTERY-CONVERGENCE-001.
 * Requires Node 22+ and /usr/bin/chromium; it starts only an isolated fixture
 * Coronatio server and removes every temporary surface in finally.
 */
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn } from 'node:child_process';
import net from 'node:net';

const root = resolve(new URL('../..', import.meta.url).pathname);
const binary = process.env.CORONATIO_BROWSER_BIN || join(root, 'target/debug/coronatio');
const chromium = process.env.CHROMIUM || '/usr/bin/chromium';
const timeoutMs = Number(process.env.BUTTERY_TIMEOUT_MS || 45_000);
const summary = { schema: 'coronatio.buttery.browser.v1', assertions: {}, counts: {}, routes: [], metrics: {}, ok: false };
let server, browser, caduceus, temp;

function assert(name, condition, detail = '') {
  summary.assertions[name] = { ok: Boolean(condition), detail };
  if (!condition) throw new Error(`${name}: ${detail || 'assertion failed'}`);
}
function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }
function freePort() { return new Promise((resolve, reject) => { const s = net.createServer(); s.once('error', reject); s.listen(0, '127.0.0.1', () => { const { port } = s.address(); s.close(error => error ? reject(error) : resolve(port)); }); }); }
async function eventually(label, predicate, ms = 8_000) { const end = Date.now() + ms; let last; while (Date.now() < end) { try { last = await predicate(); if (last) return last; } catch (error) { last = error; } await sleep(80); } throw new Error(`${label}: ${last?.message || last || 'timed out'}`); }
async function stop(child) {
  if (!child || child.exitCode !== null || child.pid === undefined) return;
  const exited = new Promise(resolve => { child.once('exit', resolve); child.once('close', resolve); child.once('error', resolve); });
  child.kill('SIGTERM');
  const killer = setTimeout(() => { if (child.exitCode === null) child.kill('SIGKILL'); }, 1_000);
  await exited;
  clearTimeout(killer);
}
async function closeServer(server) { if (server) await new Promise(resolve => server.close(resolve)); }
async function startCaduceusFixture(port) {
  const current = new Map(), requests = [];
  const fixture = net.createServer(socket => {
    let raw = Buffer.alloc(0);
    socket.on('data', chunk => {
      raw = Buffer.concat([raw, chunk]);
      const marker = raw.indexOf('\r\n\r\n');
      if (marker < 0) return;
      const head = raw.subarray(0, marker).toString('utf8');
      const length = Number(/^content-length:\s*(\d+)$/im.exec(head)?.[1] || 0);
      if (raw.length < marker + 4 + length) return;
      const path = head.split('\r\n')[0].split(' ')[1];
      let body = {}; try { body = JSON.parse(raw.subarray(marker + 4, marker + 4 + length).toString('utf8')); } catch (_) {}
      const documentId = body.documentId, supplied = body.attendance;
      let ok = false, attendance;
      if (path === '/api/v1/attendance/open' && body.pin === '1234' && documentId && body.documentIncarnation === documentId) {
        attendance = 'browser-attendance'; current.set(documentId, attendance); ok = true;
      } else if (path === '/api/v1/attendance/change-pin') {
        ok = current.get(documentId) === supplied && Boolean(body.currentPin) && Boolean(body.newPin);
      } else if (path === '/api/v1/attendance/invalidate') {
        ok = current.get(documentId) === supplied; if (ok) current.delete(documentId);
      } else if (['/api/v1/attendance/validate', '/api/v1/attendance/touch'].includes(path)) {
        ok = current.get(documentId) === supplied;
      }
      requests.push({ path, ok, documentBound: Boolean(documentId && body.documentIncarnation === documentId), attendancePresented: Boolean(supplied) });
      const payload = JSON.stringify(ok ? { ok: true, code: 'none', ...(attendance ? { attendance } : {}) } : { ok: false, code: 'caduceus-attendance-refused' });
      socket.end(`HTTP/1.1 ${ok ? '200 OK' : '401 Unauthorized'}\r\nContent-Type: application/json\r\nContent-Length: ${Buffer.byteLength(payload)}\r\nConnection: close\r\n\r\n${payload}`);
    });
  });
  await new Promise((resolve, reject) => { fixture.once('error', reject); fixture.listen(port, '127.0.0.1', resolve); });
  return { fixture, requests };
}
function run(label, command, args, options = {}) {
  const child = spawn(command, args, { cwd: root, stdio: ['ignore', 'pipe', 'pipe'], ...options });
  child.once('error', error => finish(new Error(`${label} spawn: ${error.message}`)));
  return child;
}

class Cdp {
  constructor(url) { this.ws = new WebSocket(url); this.next = 1; this.pending = new Map(); this.events = new Map(); }
  async open() { await new Promise((resolve, reject) => { this.ws.addEventListener('open', resolve, { once: true }); this.ws.addEventListener('error', reject, { once: true }); }); this.ws.addEventListener('message', event => { const m = JSON.parse(event.data); if (m.id) { const p = this.pending.get(m.id); this.pending.delete(m.id); if (p) m.error ? p.reject(new Error(m.error.message)) : p.resolve(m.result); return; } for (const fn of this.events.get(m.method) || []) fn(m.params || {}); }); }
  send(method, params = {}) { const id = this.next++; return new Promise((resolve, reject) => { this.pending.set(id, { resolve, reject }); this.ws.send(JSON.stringify({ id, method, params })); }); }
  on(method, fn) { const list = this.events.get(method) || []; list.push(fn); this.events.set(method, list); }
  async eval(expression) { const out = await this.send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true }); if (out.exceptionDetails) throw new Error(out.exceptionDetails.text); return out.result.value; }
  close() { this.ws.close(); }
}

async function main() {
  temp = await mkdtemp(join(tmpdir(), 'coronatio-buttery-'));
  const [port, debugPort, caduceusPort] = await Promise.all([freePort(), freePort(), freePort()]);
  caduceus = await startCaduceusFixture(caduceusPort);
  const config = join(temp, 'homeserver.json'), systemctl = join(temp, 'systemctl.json'), tabs = join(temp, 'tabs'), uploadRoot = join(temp, 'uploads'), profile = join(temp, 'chromium');
  await Promise.all([mkdir(tabs, { recursive: true }), mkdir(uploadRoot, { recursive: true })]);
  await writeFile(config, JSON.stringify({ global: { admin: { pin: '1234' }, cors: { allowed_origins: [`http://127.0.0.1:${port}`] } }, tabs: { starred: 'portals', portals: { config: { displayName: 'Portals', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: { Jellyfin: true, Transmission: true, Relay: true, Docs: true } }, data: { portals: [ { name: 'Jellyfin', description: 'Media', type: 'systemd', localURL: 'https://jellyfin.home.arpa', port: 8096, services: ['jellyfin'] }, { name: 'Transmission', description: 'Downloads', type: 'systemd', localURL: 'https://transmission.home.arpa', port: 9091, services: ['transmission'] }, { name: 'Relay', description: 'Mixed', type: 'systemd', localURL: 'https://relay.home.arpa', port: 4040, services: ['relay', 'vpn'] }, { name: 'Docs', description: 'Reference', type: 'link', localURL: 'https://docs.home.arpa', services: [] } ] } }, stats: { config: { displayName: 'Stats', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: {} } }, upload: { config: { displayName: 'Upload', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: {} }, data: { isPinRequired: true } }, admin: { config: { displayName: 'Admin', isEnabled: true, adminOnly: true }, visibility: { tab: true, elements: {} } }, test: { config: { displayName: 'Test', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: {} } }, dhcp: { config: { displayName: 'DHCP', isEnabled: true, adminOnly: true }, visibility: { tab: true, elements: {} } }, backblaze: { config: { displayName: 'Chia Mining', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: {} } }, 'wake-on-lan': { config: { displayName: 'YouTube', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: {} } } } }));
  await writeFile(systemctl, JSON.stringify({ jellyfin: 'active', transmission: 'inactive', relay: 'inactive', vpn: 'active' }));
  server = run('coronatio', binary, [], { env: { ...process.env, CORONATIO_PORT: String(port), CORONATIO_TAB_ROOT: tabs, CORONATIO_HOMESERVER_JSON: config, CORONATIO_SYSTEMCTL_FIXTURE: systemctl, CORONATIO_STATIC_ROOT: join(root, 'static'), CORONATIO_UPLOAD_ROOT: uploadRoot, CADUCEUS_ACCESS_BASE_URL: `http://127.0.0.1:${caduceusPort}` } });
  let serverLog = ''; server.stdout.on('data', d => { serverLog += d; }); server.stderr.on('data', d => { serverLog += d; });
  await eventually('server health', async () => (await fetch(`http://127.0.0.1:${port}/health`)).ok);
  browser = run('chromium', chromium, [`--headless=new`, '--no-sandbox', '--disable-dev-shm-usage', '--disable-gpu', '--disable-background-networking', '--disable-component-update', '--disable-default-apps', '--disable-sync', '--metrics-recording-only', '--no-first-run', `--user-data-dir=${profile}`, `--remote-debugging-port=${debugPort}`, 'about:blank']);
  let chromiumLog = ''; browser.stdout.on('data', d => { chromiumLog += d; }); browser.stderr.on('data', d => { chromiumLog += d; });
  const version = await eventually('chromium CDP', async () => { const r = await fetch(`http://127.0.0.1:${debugPort}/json/version`); return r.ok ? r.json() : false; });
  const target = await fetch(`http://127.0.0.1:${debugPort}/json/new?about:blank`, { method: 'PUT' }).then(r => r.json());
  const cdp = new Cdp(target.webSocketDebuggerUrl); await cdp.open();
  const routes = [], debugEmits = [], consoleErrors = [], responses = [];
  cdp.on('Network.requestWillBeSent', event => { const pathname = new URL(event.request.url).pathname; routes.push(pathname); if (pathname === '/api/debug/emit') debugEmits.push(event.request.postData || ''); });
  cdp.on('Network.responseReceived', event => { const url = new URL(event.response.url); responses.push({ path: url.pathname, status: event.response.status, method: event.response.requestHeadersText?.split(' ')[0] || '' }); });
  cdp.on('Runtime.exceptionThrown', event => consoleErrors.push(event.exceptionDetails?.text || event.exceptionDetails?.exception?.description || 'uncaught-exception'));
  cdp.on('Runtime.consoleAPICalled', event => { if (event.type === 'error') consoleErrors.push(event.args?.map(arg => arg.value || arg.description || '').join(' ') || 'console-error'); });
  await Promise.all(['Page.enable', 'Runtime.enable', 'Network.enable', 'Performance.enable'].map(method => cdp.send(method)));
  try { await cdp.send('PerformanceTimeline.enable', { eventTypes: ['layout-shift', 'longtask'] }); } catch (_) {}
  await cdp.send('Page.addScriptToEvaluateOnNewDocument', { source: `(() => { window.__butteryMetrics = { longtasks: 0, layoutShifts: 0, shiftValue: 0 }; window.__butteryObserverCallbacks = []; const NativePerformanceObserver = window.PerformanceObserver; if (NativePerformanceObserver) window.PerformanceObserver = class { constructor(callback) { this.callback = callback; this.native = new NativePerformanceObserver(callback); } observe(options) { window.__butteryObserverCallbacks.push({ type: options.type, callback: this.callback }); return this.native.observe(options); } disconnect() { return this.native.disconnect(); } takeRecords() { return this.native.takeRecords(); } }; try { new PerformanceObserver(list => { for (const e of list.getEntries()) window.__butteryMetrics.longtasks++; }).observe({ type: 'longtask', buffered: true }); new PerformanceObserver(list => { for (const e of list.getEntries()) if (!e.hadRecentInput) { window.__butteryMetrics.layoutShifts++; window.__butteryMetrics.shiftValue += e.value || 0; } }).observe({ type: 'layout-shift', buffered: true }); } catch (_) {} })();` });
  routes.length = 0;
  await cdp.send('Page.navigate', { url: `http://127.0.0.1:${port}/` });
  await eventually('initial portals seated', () => cdp.eval(`document.documentElement.dataset.immortalFloorState === 'Seated' && window.getImmortalFloorState?.() === 'Seated' && document.querySelectorAll('[data-portals-grid] [data-portal-card]').length === 4`));
  const initial = routes.filter(p => p === '/api/portals/elements' || p === '/api/portals/currentness');
  summary.counts.initialPortalsElements = initial.filter(p => p === '/api/portals/elements').length;
  summary.counts.initialPortalsCurrentness = initial.filter(p => p === '/api/portals/currentness').length;
  assert('initial_portals_one_owner', summary.counts.initialPortalsElements === 1, JSON.stringify(initial));
  assert('initial_portals_currentness_cadence_lawful', summary.counts.initialPortalsCurrentness === 1, JSON.stringify(initial));
  const attendanceRuntime = await cdp.eval(`(() => { window.__butteryDocument = document; renderThemeChoices(); renderThemeChoices(); const runtime = globalThis[Symbol.for('coronatio.attendance.runtime.v1')]; return { fetchDepth: window.fetch.__coronatioAttendanceDecoratorDepth, fetchDecorationCount: runtime?.fetchDecorationCount, htmxHandlerInstallCount: runtime?.htmxHandlerInstallCount, activityCensusInstallCount: runtime?.activityCensusInstallCount, fetchIsOwner: window.fetch === runtime?.decoratedFetch, hasAttendance: Boolean(runtime?.currentAttendance) }; })()`);
  assert('attendance_runtime_single_owner_at_guest_load', attendanceRuntime.fetchDepth === 1 && attendanceRuntime.fetchDecorationCount === 1 && attendanceRuntime.htmxHandlerInstallCount === 1 && attendanceRuntime.activityCensusInstallCount === 1 && attendanceRuntime.fetchIsOwner && !attendanceRuntime.hasAttendance, JSON.stringify(attendanceRuntime));
  const diagnostics = await cdp.eval(`(() => { const controls = [...document.querySelectorAll('[data-crown-diagnostic-toggle]')]; controls.find(x => x.dataset.crownDiagnosticToggle === 'requests')?.click(); controls.find(x => x.dataset.crownDiagnosticToggle === 'layout')?.click(); const enabled = JSON.parse(localStorage.getItem('coronatioDiagnostics')).enabled; return { sections: controls.map(x => x.dataset.crownDiagnosticToggle), requests: enabled.includes('requests'), layout: enabled.includes('layout') }; })()`);
  assert('diagnostics_sections', diagnostics.sections.length === 6 && diagnostics.sections.includes('layout') && diagnostics.sections.includes('requests'), JSON.stringify(diagnostics));
  assert('diagnostics_sections_enabled', diagnostics.requests && diagnostics.layout, JSON.stringify(diagnostics));
  await sleep(120); debugEmits.length = 0;
  const syntheticLayout = await cdp.eval(`(() => { const ownerFetch = window.fetch; const emits = []; window.fetch = (input, init = {}) => { if (String(input).includes('/api/debug/emit')) emits.push(init.body || ''); return ownerFetch(input, init); }; for (const observer of window.__butteryObserverCallbacks.filter(observer => ['paint', 'layout-shift', 'longtask'].includes(observer.type))) { const entry = observer.type === 'layout-shift' ? { name: 'layout-shift', duration: 0, value: 0.02, hadRecentInput: false } : { name: observer.type === 'paint' ? 'first-contentful-paint' : 'self', duration: 12.5 }; observer.callback({ getEntries: () => [entry] }); } window.fetch = ownerFetch; return { observers: window.__butteryObserverCallbacks.map(observer => observer.type), enabled: window.crownDebug.enabled('crown-layout'), emits }; })()`);
  summary.metrics.syntheticLayout = { observers: syntheticLayout.observers, emitCount: syntheticLayout.emits.length, enabled: syntheticLayout.enabled };
  const layoutEvents = syntheticLayout.emits.map(raw => { try { return JSON.parse(raw); } catch (_) { return null; } }).filter(event => event?.kind === 'crown-layout');
  assert('layout_observer_installation', ['layout-shift', 'longtask'].every(type => syntheticLayout.observers.includes(type)), JSON.stringify(syntheticLayout.observers));
  assert('layout_bounded_schema', layoutEvents.every(event => event.event === 'performance-entry' && event.attributes && !('sources' in event.attributes) && !('attribution' in event.attributes)) && layoutEvents.some(event => event.attributes.entryType === 'layout-shift' && event.attributes.hadRecentInput === false && Number(event.attributes.value) === 0.02) && layoutEvents.some(event => event.attributes.entryType === 'longtask' && Number(event.attributes.duration) === 12.5), JSON.stringify(layoutEvents));
  debugEmits.length = 0;
  const emitted = await cdp.eval(`window.crownDebug.emit('crown-layout', 'browser-redaction-proof', { phase:'seated', safeField:'survives', token:'browser-sentinel', nested:{ pin:'browser-sentinel', body:'browser-sentinel', requestBody:'browser-sentinel', responseBody:'browser-sentinel', raw_body:'browser-sentinel', payload:'browser-sentinel', payloadData:'browser-sentinel', nestedPayload:'browser-sentinel', headers:{ authorization:'browser-sentinel' }, capability:'browser-sentinel', connectionString:'browser-sentinel', localStorage:'browser-sentinel', domSnapshot:'browser-sentinel', sourceHtml:'browser-sentinel' }, array:[{ secret:'browser-sentinel' }, { safeNested:'survives' }] })`);
  assert('diagnostics_emit_enabled', emitted === true, 'production crownDebug.emit was not enabled');
  await eventually('production crownDebug redaction payload', () => debugEmits.some(raw => { try { const event = JSON.parse(raw); return event.kind === 'crown-layout' && event.event === 'browser-redaction-proof'; } catch (_) { return false; } }));
  const redactionEvent = debugEmits.map(raw => { try { return JSON.parse(raw); } catch (_) { return null; } }).find(event => event?.kind === 'crown-layout' && event.event === 'browser-redaction-proof');
  const redactionText = JSON.stringify(redactionEvent || {});
  const redacted = redactionEvent?.attributes || {};
  assert('diagnostics_redaction_actual_client_payload', redacted.phase === 'seated' && redacted.safeField === 'survives' && redacted.array?.[1]?.safeNested === 'survives' && !('token' in redacted) && !('pin' in (redacted.nested || {})) && !('body' in (redacted.nested || {})) && !('requestBody' in (redacted.nested || {})) && !('responseBody' in (redacted.nested || {})) && !('raw_body' in (redacted.nested || {})) && !('payload' in (redacted.nested || {})) && !('payloadData' in (redacted.nested || {})) && !('nestedPayload' in (redacted.nested || {})) && !('headers' in (redacted.nested || {})) && !('capability' in (redacted.nested || {})) && !('connectionString' in (redacted.nested || {})) && !('localStorage' in (redacted.nested || {})) && !('domSnapshot' in (redacted.nested || {})) && !('sourceHtml' in (redacted.nested || {})) && !('secret' in (redacted.array?.[0] || {})) && !redactionText.includes('browser-sentinel'), 'actual crownDebug payload redaction failed');
  debugEmits.length = 0;
  routes.length = 0;
  await sleep(3_100);
  const idle = routes.filter(p => p === '/api/portals/elements' || p === '/api/portals/currentness');
  summary.counts.idlePortalsElements = idle.filter(p => p === '/api/portals/elements').length;
  summary.counts.idlePortalsCurrentness = idle.filter(p => p === '/api/portals/currentness').length;
  assert('idle_portals_no_rehydration', summary.counts.idlePortalsElements === 0, JSON.stringify(idle));
  assert('idle_portals_cadence_lawful', summary.counts.idlePortalsCurrentness === 0, JSON.stringify(idle));
  // Initial browser layout is intentionally observed; only crossing-induced shift is a wall.
  await cdp.eval(`window.__butteryMetrics = { longtasks: 0, layoutShifts: 0, shiftValue: 0 }`);
  const crossings = [];
  const cross = async (from, to, mode = 'public') => {
    routes.length = 0;
    const result = await cdp.eval(`(async () => { const before = { state: window.getImmortalFloorState(), guest: [...document.querySelectorAll('[data-pane-panel].active')].map(x => x.dataset.panePanel) }; const p = showPane(${JSON.stringify(to)}, { refresh: true }); await new Promise(requestAnimationFrame); const during = { state: window.getImmortalFloorState(), guest: [...document.querySelectorAll('[data-pane-panel].active')].map(x => x.dataset.panePanel) }; await p; return { before, during, after: { state: window.getImmortalFloorState(), guest: [...document.querySelectorAll('[data-pane-panel].active')].map(x => x.dataset.panePanel), floor2: document.querySelectorAll('[data-immortal-floor-layer="2"] .pane.active').length } }; })()`);
    if (to === 'portals') await eventually(`${from}-to-portals cards`, () => cdp.eval(`document.querySelectorAll('[data-portals-grid] [data-portal-card]').length === 4`));
    const portalRoutes = routes.filter(p => p === '/api/portals/elements' || p === '/api/portals/currentness');
    const counts = { from, to, elements: portalRoutes.filter(p => p === '/api/portals/elements').length, currentness: portalRoutes.filter(p => p === '/api/portals/currentness').length };
    crossings.push(counts);
    assert(`cross_${mode}_${from}_to_${to}_single_guest`, result.after.state === 'Seated' && result.after.guest.length === 1 && result.after.floor2 === 1, JSON.stringify(result));
    assert(`cross_${mode}_${from}_to_${to}_source_matches`, result.before.guest[0] === from && result.after.guest[0] === to, JSON.stringify(result));
    assert(`cross_${mode}_${from}_to_${to}_elements_lawful`, counts.elements === (to === 'portals' ? 1 : 0), JSON.stringify(counts));
    assert(`cross_${mode}_${from}_to_${to}_currentness_lawful`, counts.currentness === (to === 'portals' ? 1 : 0), JSON.stringify(counts));
    if (mode === 'public' && from === 'portals' && to === 'test') {
      await eventually('production request diagnostics', () => debugEmits.some(raw => { try { const event = JSON.parse(raw); return event.kind === 'crown-requests' && event.attributes?.phase === 'before-request' && event.attributes?.pathname === '/admit/test'; } catch (_) { return false; } }));
      const requestEvents = debugEmits.map(raw => { try { return JSON.parse(raw); } catch (_) { return null; } }).filter(event => event?.kind === 'crown-requests');
      assert('request_phase_reaches_debug_emit_without_recursion', requestEvents.some(event => event.attributes?.phase === 'before-request' && event.attributes?.pathname === '/admit/test' && event.attributes?.method === 'GET') && requestEvents.every(event => event.attributes?.pathname !== '/api/debug/emit'), JSON.stringify(requestEvents));
    }
    return result;
  };
  const first = await cross('portals', 'test');
  assert('outgoing_guest_retained_until_reveal', first.before.guest[0] === 'portals' && first.during.guest[0] === 'portals', JSON.stringify(first));
  await cross('test', 'portals'); await cross('portals', 'test'); await cross('test', 'portals');
  await cdp.send('Emulation.setEmulatedMedia', { features: [{ name: 'prefers-reduced-motion', value: 'reduce' }] });
  await cross('portals', 'test'); await cross('test', 'portals');
  assert('reduced_motion_settles', await cdp.eval(`matchMedia('(prefers-reduced-motion: reduce)').matches && window.getImmortalFloorState() === 'Seated'`));
  await sleep(120);
  debugEmits.length = 0;
  const ttlExpired = await cdp.eval(`(() => { localStorage.setItem('coronatioDiagnostics', JSON.stringify({ enabled: ['requests', 'layout'], expiresAt: Date.now() - 1 })); const observer = window.__butteryObserverCallbacks.find(observer => observer.type === 'paint'); observer?.callback({ getEntries: () => [{ name: 'first-paint', duration: 1 }] }); return !window.crownDebug.enabled('crown-layout') && !window.crownDebug.enabled('crown-requests'); })()`);
  await sleep(120);
  assert('diagnostics_ttl_expiry_stops_emissions', ttlExpired && debugEmits.length === 0, JSON.stringify(debugEmits));
  summary.metrics.public = await cdp.eval(`window.__butteryMetrics`);
  assert('public_longtask_telemetry_honest', Number.isInteger(Number(summary.metrics.public.longtasks)) && Number(summary.metrics.public.longtasks) >= 0, JSON.stringify(summary.metrics.public));
  assert('public_no_unintended_layout_shift', Number(summary.metrics.public.shiftValue || 0) === 0, JSON.stringify(summary.metrics.public));
  await cross('portals', 'stats');
  await eventually('stats pulse stream open', () => cdp.eval(`Boolean(pulseStreamId && coreStreamId)`));
  responses.length = 0;
  await cdp.eval(`(() => { document.querySelector('[data-admin-button]')?.click(); const pin = document.querySelector('[data-pin-current]'); if (!pin) throw new Error('admin-pin-input-missing'); pin.value = '1234'; document.querySelector('[data-pin-confirm-button]')?.click(); })()`);
  const adminSession = await eventually('admin browser session', async () => { const state = await cdp.eval(`(() => { const runtime = globalThis[Symbol.for('coronatio.attendance.runtime.v1')]; return { established: document.querySelector('[data-product="Coronatio"]')?.dataset.adminMode === 'true' && Boolean(runtime?.currentAttendance), sameDocument: window.__butteryDocument === document, floor0: document.querySelectorAll('[data-immortal-floor-layer="0"]').length, floor1: document.querySelectorAll('[data-immortal-floor-layer="1"]').length, activeFloor2: document.querySelectorAll('[data-immortal-floor-layer="2"] .pane.active').length, state: window.getImmortalFloorState?.(), fetchDepth: window.fetch.__coronatioAttendanceDecoratorDepth, fetchDecorationCount: runtime?.fetchDecorationCount, htmxHandlerInstallCount: runtime?.htmxHandlerInstallCount, activityCensusInstallCount: runtime?.activityCensusInstallCount }; })()`); return state.established && state.state === 'Seated' ? state : false; });
  summary.counts.adminSessionEstablished = adminSession.established;
  assert('admin_session_established_through_pin_ui', adminSession.established === true, JSON.stringify(adminSession));
  assert('admin_projection_is_same_crown', adminSession.floor0 === 1 && adminSession.floor1 === 1 && adminSession.activeFloor2 === 1 && adminSession.state === 'Seated' && adminSession.sameDocument, JSON.stringify(adminSession));
  assert('attendance_runtime_remains_single_after_pin', adminSession.fetchDepth === 1 && adminSession.fetchDecorationCount === 1 && adminSession.htmxHandlerInstallCount === 1 && adminSession.activityCensusInstallCount === 1, JSON.stringify(adminSession));
  await sleep(750);
  const adminInventoryAtStats = await cdp.eval(`(() => { const labels = [...document.querySelectorAll('[data-tab-id] .tab-name')].map(node => node.textContent.trim()); return { labels, tabEyes: document.querySelectorAll('[data-tab-visibility-toggle]').length, statEyes: document.querySelectorAll('[data-stat-visibility-toggle]').length, active: [...document.querySelectorAll('[data-pane-panel].active')].map(node => node.dataset.panePanel) }; })()`);
  assert('admin_stats_projection_and_inventory', ['Admin','DHCP','YouTube','Chia Mining'].every(label => adminInventoryAtStats.labels.includes(label)) && adminInventoryAtStats.tabEyes >= 6 && adminInventoryAtStats.statEyes >= 1, JSON.stringify(adminInventoryAtStats));
  const adminProjection = await cdp.eval(`(async () => { const labels = [...document.querySelectorAll('[data-tab-id] .tab-name')].map(node => node.textContent.trim()); const statsResponse = await fetch('/api/stats', { cache: 'no-store' }); const stats = await statsResponse.json(); return { labels, tabEyes: document.querySelectorAll('[data-tab-visibility-toggle]').length, statEyes: document.querySelectorAll('[data-stat-visibility-toggle]').length, statsStatus: statsResponse.status, statsAdminFields: Array.isArray(stats.processes) && Array.isArray(stats.leases), header: [...document.querySelectorAll('[data-change-pin-button], [data-admin-button]')].map(node => node.textContent.trim()) }; })()`);
  assert('admin_inventory_and_stats_eyes_visible', ['Admin','DHCP','YouTube','Chia Mining'].every(label => adminProjection.labels.includes(label)) && adminProjection.tabEyes >= 6 && adminProjection.statEyes >= 1, JSON.stringify(adminProjection));
  assert('admin_stats_endpoint_projection', adminProjection.statsStatus === 200 && adminProjection.statsAdminFields, JSON.stringify(adminProjection));
  assert('admin_header_controls_visible', adminProjection.header.some(label => label.includes('Change PIN')) && adminProjection.header.some(label => label.includes('Exit Admin Mode')), JSON.stringify(adminProjection.header));
  await eventually('stream upgrades accepted', () => ['core','stats'].every(family => responses.some(response => response.path === `/api/${family}/pulse/upgrade` && response.status === 200)));
  await cdp.eval(`window.__butteryStreamIds = { stats: pulseStreamId, core: coreStreamId }`);
  await cdp.eval(`document.dispatchEvent(new PointerEvent('pointerdown', { bubbles: true }))`);
  await eventually('eligible activity touched attendance', async () => caduceus.requests.some(request => request.path === '/api/v1/attendance/touch' && request.ok));
  await sleep(250); // Let the genuine session projection settle before opening its idle census window.
  await cdp.eval(`window.__butteryMetrics = { longtasks: 0, layoutShifts: 0, shiftValue: 0 }`);
  routes.length = 0;
  await sleep(3_100);
  const adminIdle = routes.filter(p => p === '/api/portals/elements' || p === '/api/portals/currentness');
  summary.counts.adminIdlePortalsElements = adminIdle.filter(p => p === '/api/portals/elements').length;
  summary.counts.adminIdlePortalsCurrentness = adminIdle.filter(p => p === '/api/portals/currentness').length;
  assert('admin_idle_portals_no_rehydration', summary.counts.adminIdlePortalsElements === 0 && summary.counts.adminIdlePortalsCurrentness === 0, JSON.stringify(adminIdle));
  await cross('stats', 'portals', 'admin');
  const portalProjection = await eventually('admin portal controls', async () => { const state = await cdp.eval(`(() => ({ eyes: document.querySelectorAll('[data-portal-visibility-toggle]').length, actions: [...document.querySelectorAll('[data-service-action]')].map(node => node.dataset.serviceAction), deletes: document.querySelectorAll('[data-portal-delete]').length, addPortal: document.querySelectorAll('[data-add-portal-open]').length, portalNameHeadings: document.querySelectorAll('[data-portal-card] .portal-name, [data-portal-card] h3.portal-name').length }))()`); return state.eyes >= 4 && state.actions.length >= 6 && state.deletes >= 1 && state.addPortal === 1 && state.portalNameHeadings === 0 ? state : false; });
  assert('admin_portals_projection_controls_and_hidden_names', portalProjection.eyes >= 4 && ['start','stop','restart','enable','disable','status'].every(action => portalProjection.actions.includes(action)) && portalProjection.deletes >= 1 && portalProjection.addPortal === 1 && portalProjection.portalNameHeadings === 0, JSON.stringify(portalProjection));
  const adminFirst = await cross('portals', 'test', 'admin');
  assert('admin_outgoing_guest_retained_until_reveal', adminFirst.before.guest[0] === 'portals' && adminFirst.during.guest[0] === 'portals', JSON.stringify(adminFirst));
  await cross('test', 'portals', 'admin');
  const statusModalViewport = await cdp.eval(`(() => {
    const grid = document.querySelector('[data-portals-grid]');
    const status = grid?.querySelector('[data-service-action="status"]');
    if (!grid || !status) throw new Error('admin-status-control-missing');
    grid.style.minHeight = '2800px';
    window.scrollTo(0, 1200);
    if (window.scrollY < 200) throw new Error('portals-grid-did-not-scroll');
    status.click();
    return true;
  })()`);
  const statusModalGeometry = await eventually('status modal centered in scrolled viewport', async () => {
    const geometry = await cdp.eval(`(() => {
      const overlay = document.querySelector('[data-service-status-modal]');
      const content = overlay?.querySelector('.portal-modal-content');
      if (!overlay || overlay.hidden || !content) return false;
      const overlayBox = overlay.getBoundingClientRect();
      const contentBox = content.getBoundingClientRect();
      return {
        bodyChild: overlay.parentElement === document.body,
        scrollY: window.scrollY,
        overlayTop: overlayBox.top,
        overlayHeight: overlayBox.height,
        viewportHeight: window.innerHeight,
        contentCenterY: contentBox.top + contentBox.height / 2,
        viewportCenterY: window.innerHeight / 2,
      };
    })()`);
    return geometry?.bodyChild && geometry.scrollY > 200 && Math.abs(geometry.overlayTop) <= 1 && Math.abs(geometry.overlayHeight - geometry.viewportHeight) <= 1 && Math.abs(geometry.contentCenterY - geometry.viewportCenterY) <= 1 ? geometry : false;
  });
  assert('status_modal_opens_at_current_scrolled_viewport_center', statusModalViewport && statusModalGeometry.bodyChild && statusModalGeometry.scrollY > 200, JSON.stringify(statusModalGeometry));
  summary.metrics.statusModalViewport = statusModalGeometry;
  await sleep(120);
  await cdp.eval(`window.__butteryMetrics = { longtasks: 0, layoutShifts: 0, shiftValue: 0 }`);
  summary.metrics.admin = await cdp.eval(`window.__butteryMetrics`);
  assert('admin_longtask_telemetry_honest', Number.isInteger(Number(summary.metrics.admin.longtasks)) && Number(summary.metrics.admin.longtasks) >= 0, JSON.stringify(summary.metrics.admin));
  assert('admin_no_unintended_layout_shift', Number(summary.metrics.admin.shiftValue || 0) === 0, JSON.stringify(summary.metrics.admin));
  responses.length = 0;
  await cdp.eval(`document.querySelector('[data-admin-button]')?.click()`);
  const guestAfterExit = await eventually('guest restored after exit', async () => { const state = await cdp.eval(`(() => { const runtime = globalThis[Symbol.for('coronatio.attendance.runtime.v1')]; const labels = [...document.querySelectorAll('[data-tab-id] .tab-name')].map(node => node.textContent.trim()); return { guest: document.querySelector('[data-product="Coronatio"]')?.dataset.adminMode === 'false' && !runtime?.currentAttendance, sameDocument: window.__butteryDocument === document, labels, tabEyes: document.querySelectorAll('[data-tab-visibility-toggle]').length, portalEyes: document.querySelectorAll('[data-portal-visibility-toggle]').length, actions: document.querySelectorAll('[data-service-action]').length, deletes: document.querySelectorAll('[data-portal-delete]').length, addPortal: document.querySelectorAll('[data-add-portal-open]').length, portalNames: document.querySelectorAll('[data-portal-card] .portal-name').length, fetchDepth: window.fetch.__coronatioAttendanceDecoratorDepth }; })()`); return state.guest && !state.labels.includes('Admin') && state.portalEyes === 0 && state.actions === 0 && state.deletes === 0 && state.addPortal === 0 && state.portalNames === 4 ? state : false; });
  assert('exit_restores_guest_baseline_on_same_document', guestAfterExit.guest && guestAfterExit.sameDocument && !guestAfterExit.labels.includes('Admin') && guestAfterExit.tabEyes === 0 && guestAfterExit.portalEyes === 0 && guestAfterExit.actions === 0 && guestAfterExit.deletes === 0 && guestAfterExit.addPortal === 0 && guestAfterExit.portalNames === 4 && guestAfterExit.fetchDepth === 1, JSON.stringify(guestAfterExit));
  const invalidatesBeforeUploadPin = caduceus.requests.filter(request => request.path === '/api/v1/attendance/invalidate').length;
  await cross('portals', 'upload', 'upload-pin');
  await eventually('upload PIN controls seated', () => cdp.eval(`Boolean(document.querySelector('[data-upload-file]') && document.querySelector('[data-upload-submit]') && document.querySelector('[data-upload-pin-modal]'))`));
  await cdp.eval(`(() => { const input = document.querySelector('[data-upload-file]'); const transfer = new DataTransfer(); transfer.items.add(new File(['scoped upload proof'], 'scoped-upload-proof.txt', { type: 'text/plain' })); input.files = transfer.files; input.dispatchEvent(new Event('change', { bubbles: true })); document.querySelector('[data-upload-submit]').click(); })()`);
  await eventually('per-upload PIN modal open', () => cdp.eval(`document.querySelector('[data-upload-pin-modal]')?.hidden === false`));
  await cdp.eval(`(() => { const input = document.querySelector('[data-upload-pin-input]'); input.value = '1234'; document.querySelector('[data-upload-pin-confirm]').click(); })()`);
  const uploadPinInvalidate = await eventually('per-upload scoped attendance invalidated', () => {
    const invalidates = caduceus.requests.filter(request => request.path === '/api/v1/attendance/invalidate');
    return invalidates.length > invalidatesBeforeUploadPin ? invalidates.at(-1) : false;
  });
  assert('per_upload_pin_scoped_invalidate_document_bound', uploadPinInvalidate.ok && uploadPinInvalidate.documentBound && uploadPinInvalidate.attendancePresented, JSON.stringify(uploadPinInvalidate));
  assert('per_upload_pin_flow_has_no_console_error', consoleErrors.length === 0, JSON.stringify(consoleErrors));
  const refusedUpgrades = await cdp.eval(`Promise.all(['stats','core'].map(async family => ({ family, status: (await fetch('/api/' + family + '/pulse/upgrade?streamId=' + encodeURIComponent(window.__butteryStreamIds[family]), { method: 'POST', cache: 'no-store' })).status })))`);
  assert('exit_refuses_stream_upgrades_again', refusedUpgrades.every(result => result.status === 401), JSON.stringify(refusedUpgrades));
  summary.counts.attendanceAuthority = caduceus.requests.reduce((counts, request) => { counts[request.path] = (counts[request.path] || 0) + 1; return counts; }, {});
  assert('attendance_authority_saw_open_touch_invalidate', ['/api/v1/attendance/open','/api/v1/attendance/touch','/api/v1/attendance/invalidate'].every(path => caduceus.requests.some(request => request.path === path && request.ok)), JSON.stringify(summary.counts.attendanceAuthority));
  assert('zero_uncaught_console_errors', consoleErrors.length === 0, JSON.stringify(consoleErrors));
  summary.counts.crossings = crossings;
  summary.routes = ['/api/portals/currentness', '/api/portals/elements', '/api/stats', '/api/core/pulse/upgrade', '/api/stats/pulse/upgrade']; summary.counts.routeCensus = crossings.length; summary.ok = true;
  cdp.close();
}

const watchdog = setTimeout(() => { finish(new Error(`harness timeout after ${timeoutMs}ms`)); }, timeoutMs);
let finishing = false;
async function finish(error) {
  if (finishing) return;
  finishing = true;
  clearTimeout(watchdog);
  await Promise.all([stop(browser), stop(server), closeServer(caduceus?.fixture)]);
  if (temp) await rm(temp, { recursive: true, force: true, maxRetries: 4, retryDelay: 100 });
  if (error) { summary.error = error.message; console.log(JSON.stringify(summary)); process.exitCode = 1; } else console.log(JSON.stringify(summary));
}
process.on('SIGINT', () => finish(new Error('SIGINT'))); process.on('SIGTERM', () => finish(new Error('SIGTERM')));
main().then(() => finish()).catch(finish);
