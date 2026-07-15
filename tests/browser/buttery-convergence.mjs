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
let server, browser, temp;

function assert(name, condition, detail = '') {
  summary.assertions[name] = { ok: Boolean(condition), detail };
  if (!condition) throw new Error(`${name}: ${detail || 'assertion failed'}`);
}
function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }
function freePort() { return new Promise((resolve, reject) => { const s = net.createServer(); s.once('error', reject); s.listen(0, '127.0.0.1', () => { const { port } = s.address(); s.close(error => error ? reject(error) : resolve(port)); }); }); }
async function eventually(label, predicate, ms = 8_000) { const end = Date.now() + ms; let last; while (Date.now() < end) { try { last = await predicate(); if (last) return last; } catch (error) { last = error; } await sleep(80); } throw new Error(`${label}: ${last?.message || last || 'timed out'}`); }
async function stop(child) {
  if (!child || child.exitCode !== null) return;
  const exited = new Promise(resolve => child.once('exit', resolve));
  child.kill('SIGTERM');
  const killer = setTimeout(() => { if (child.exitCode === null) child.kill('SIGKILL'); }, 1_000);
  await exited;
  clearTimeout(killer);
}
function run(command, args, options = {}) { return spawn(command, args, { cwd: root, stdio: ['ignore', 'pipe', 'pipe'], ...options }); }

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
  const [port, debugPort] = await Promise.all([freePort(), freePort()]);
  const config = join(temp, 'homeserver.json'), systemctl = join(temp, 'systemctl.json'), tabs = join(temp, 'tabs'), profile = join(temp, 'chromium');
  await mkdir(tabs, { recursive: true });
  await writeFile(config, JSON.stringify({ global: { admin: { pin: '1234' } }, tabs: { starred: 'portals', portals: { config: { displayName: 'Portals', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: { Jellyfin: true, Transmission: true, Relay: true, Docs: true } }, data: { portals: [ { name: 'Jellyfin', description: 'Media', type: 'systemd', localURL: 'https://jellyfin.home.arpa', port: 8096, services: ['jellyfin'] }, { name: 'Transmission', description: 'Downloads', type: 'systemd', localURL: 'https://transmission.home.arpa', port: 9091, services: ['transmission'] }, { name: 'Relay', description: 'Mixed', type: 'systemd', localURL: 'https://relay.home.arpa', port: 4040, services: ['relay', 'vpn'] }, { name: 'Docs', description: 'Reference', type: 'link', localURL: 'https://docs.home.arpa', services: [] } ] } }, stats: { config: { displayName: 'Stats', isEnabled: true, adminOnly: false }, visibility: { tab: true, elements: {} } } } }));
  await writeFile(systemctl, JSON.stringify({ jellyfin: 'active', transmission: 'inactive', relay: 'inactive', vpn: 'active' }));
  server = run(binary, [], { env: { ...process.env, CORONATIO_PORT: String(port), CORONATIO_TAB_ROOT: tabs, CORONATIO_HOMESERVER_JSON: config, CORONATIO_SYSTEMCTL_FIXTURE: systemctl, CORONATIO_STATIC_ROOT: join(root, 'static') } });
  let serverLog = ''; server.stdout.on('data', d => { serverLog += d; }); server.stderr.on('data', d => { serverLog += d; });
  await eventually('server health', async () => (await fetch(`http://127.0.0.1:${port}/health`)).ok);
  browser = run(chromium, [`--headless=new`, '--no-sandbox', '--disable-dev-shm-usage', '--disable-gpu', '--disable-background-networking', '--disable-component-update', '--disable-default-apps', '--disable-sync', '--metrics-recording-only', '--no-first-run', `--user-data-dir=${profile}`, `--remote-debugging-port=${debugPort}`, 'about:blank']);
  let chromiumLog = ''; browser.stdout.on('data', d => { chromiumLog += d; }); browser.stderr.on('data', d => { chromiumLog += d; });
  const version = await eventually('chromium CDP', async () => { const r = await fetch(`http://127.0.0.1:${debugPort}/json/version`); return r.ok ? r.json() : false; });
  const target = await fetch(`http://127.0.0.1:${debugPort}/json/new?about:blank`, { method: 'PUT' }).then(r => r.json());
  const cdp = new Cdp(target.webSocketDebuggerUrl); await cdp.open();
  const routes = []; cdp.on('Network.requestWillBeSent', event => routes.push(new URL(event.request.url).pathname));
  await Promise.all(['Page.enable', 'Runtime.enable', 'Network.enable', 'Performance.enable'].map(method => cdp.send(method)));
  try { await cdp.send('PerformanceTimeline.enable', { eventTypes: ['layout-shift', 'longtask'] }); } catch (_) {}
  await cdp.send('Page.addScriptToEvaluateOnNewDocument', { source: `(() => { window.__butteryMetrics = { longtasks: 0, layoutShifts: 0, shiftValue: 0 }; try { new PerformanceObserver(list => { for (const e of list.getEntries()) window.__butteryMetrics.longtasks++; }).observe({ type: 'longtask', buffered: true }); new PerformanceObserver(list => { for (const e of list.getEntries()) if (!e.hadRecentInput) { window.__butteryMetrics.layoutShifts++; window.__butteryMetrics.shiftValue += e.value || 0; } }).observe({ type: 'layout-shift', buffered: true }); } catch (_) {} })();` });
  await cdp.send('Page.navigate', { url: `http://127.0.0.1:${port}/` });
  await eventually('initial portals seated', () => cdp.eval(`document.documentElement.dataset.immortalFloorState === 'Seated' && window.getImmortalFloorState?.() === 'Seated' && document.querySelectorAll('[data-portals-grid] [data-portal-card]').length === 4`));
  const diagnostics = await cdp.eval(`(() => { const controls = [...document.querySelectorAll('[data-crown-diagnostic-toggle]')]; controls.find(x => x.dataset.crownDiagnosticToggle === 'layout')?.click(); const enabled = JSON.parse(localStorage.getItem('coronatioDiagnostics')).enabled.includes('layout'); localStorage.setItem('coronatioDiagnostics', JSON.stringify({ enabled:['layout'], expiresAt: Date.now() - 1 })); return { sections: controls.map(x => x.dataset.crownDiagnosticToggle), enabled, expired: !window.crownDebug.enabled('crown-layout') }; })()`);
  assert('diagnostics_sections', diagnostics.sections.length === 6 && diagnostics.sections.includes('layout'), JSON.stringify(diagnostics));
  assert('diagnostics_ttl_expiry', diagnostics.enabled && diagnostics.expired, JSON.stringify(diagnostics));
  const redaction = await cdp.eval(`fetch('/api/debug/emit', { method:'POST', headers:{'content-type':'application/json'}, body:JSON.stringify({kind:'crown-layout', event:'browser-proof', attributes:{pin:'1234', adminToken:'do-not-leak', headers:'redact', phase:'seated'}}) }).then(async r => ({ status:r.status, body:await r.text() }))`);
  assert('diagnostics_redaction', !redaction.body.includes('do-not-leak') && !redaction.body.includes('"pin":"1234"'), JSON.stringify(redaction));
  routes.length = 0;
  await sleep(3_100);
  const idle = routes.filter(p => p === '/api/portals/elements' || p === '/api/portals/currentness');
  summary.counts.idlePortalsElements = idle.filter(p => p === '/api/portals/elements').length;
  summary.counts.idlePortalsCurrentness = idle.filter(p => p === '/api/portals/currentness').length;
  assert('idle_portals_no_rehydration', summary.counts.idlePortalsElements === 0, JSON.stringify(idle));
  assert('idle_portals_cadence_lawful', summary.counts.idlePortalsCurrentness === 0, JSON.stringify(idle));
  // Initial browser layout is intentionally observed; only crossing-induced shift is a wall.
  await cdp.eval(`window.__butteryMetrics = { longtasks: 0, layoutShifts: 0, shiftValue: 0 }`);
  const cross = async (id) => { const result = await cdp.eval(`(async () => { const before = { state: window.getImmortalFloorState(), guest: [...document.querySelectorAll('[data-pane-panel].active')].map(x => x.dataset.panePanel) }; const p = showPane(${JSON.stringify(id)}, { refresh: true }); await new Promise(requestAnimationFrame); const during = { state: window.getImmortalFloorState(), guest: [...document.querySelectorAll('[data-pane-panel].active')].map(x => x.dataset.panePanel) }; await p; return { before, during, after: { state: window.getImmortalFloorState(), guest: [...document.querySelectorAll('[data-pane-panel].active')].map(x => x.dataset.panePanel), floor2: document.querySelectorAll('[data-immortal-floor-layer="2"] .pane.active').length } }; })()`); assert(`cross_${id}_single_guest`, result.after.state === 'Seated' && result.after.guest.length === 1 && result.after.floor2 === 1, JSON.stringify(result)); return result; };
  const first = await cross('test');
  assert('outgoing_guest_retained_until_reveal', first.before.guest[0] === 'portals' && first.during.guest[0] === 'portals', JSON.stringify(first));
  await cross('portals'); await cross('test'); await cross('portals');
  await cdp.send('Emulation.setEmulatedMedia', { features: [{ name: 'prefers-reduced-motion', value: 'reduce' }] });
  await cross('test'); await cross('portals');
  assert('reduced_motion_settles', await cdp.eval(`matchMedia('(prefers-reduced-motion: reduce)').matches && window.getImmortalFloorState() === 'Seated'`));
  summary.metrics = await cdp.eval(`window.__butteryMetrics`);
  assert('no_unintended_layout_shift', Number(summary.metrics.shiftValue || 0) === 0, JSON.stringify(summary.metrics));
  summary.routes = [...new Set(routes)].sort(); summary.counts.routeCensus = routes.length; summary.ok = true;
  cdp.close();
}

const watchdog = setTimeout(() => { finish(new Error(`harness timeout after ${timeoutMs}ms`)); }, timeoutMs);
let finishing = false;
async function finish(error) {
  if (finishing) return;
  finishing = true;
  clearTimeout(watchdog);
  await Promise.all([stop(browser), stop(server)]);
  if (temp) await rm(temp, { recursive: true, force: true, maxRetries: 4, retryDelay: 100 });
  if (error) { summary.error = error.message; console.log(JSON.stringify(summary)); process.exitCode = 1; } else console.log(JSON.stringify(summary));
}
process.on('SIGINT', () => finish(new Error('SIGINT'))); process.on('SIGTERM', () => finish(new Error('SIGTERM')));
main().then(() => finish()).catch(finish);
