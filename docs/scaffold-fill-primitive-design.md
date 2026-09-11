# Scaffold-fill primitive design

## Review brief

- **Base:** `672a1c78ed33df6949a9260b4002ba98ba494843`
- **Ground:** Coronatio ground for `coronatio-scaffold-fill-primitive-design`
- **Scope:** design documentation only, in this file
- **Status:** proposal; no implementation, schema, or verification is claimed

This design addresses progressive settlement of a guest's already-admitted
scaffold. It does not redesign admission, invent a second lifecycle, or turn a
pulse into a data transport.

## Contract and current shape

Coronatio has one browser lifecycle authority: the four-state `ImmortalFloor`
(`BootFloor`, `Seated`, `GuestRevolution`, and `BareFloor`). `DrawnGuestTwo`
remains inside that floor. A guest is revealed atomically only after its owned
handoff, fresh payload admission, healthy observation, and the existing
double-`requestAnimationFrame` crossing. Progressive filling begins only after
the admitted floor-2 scaffold exists. “Pending”, “ready”, and “fault” below are
slot-local metadata; they are not floor states and not a parallel machine.

The current Stats path makes the limitation visible:

- `stat_element_templates()` in `src/bands/crown-law/element-fragments.rs`
  extracts seven real per-element skeletons: CPU, network, I/O, memory, disk
  usage, KEA leases, and process usage. Their content contains placeholders,
  loading text, or empty containers.
- `hydrateStats()` in `src/bands/shell/document-4.rs` first waits for
  `/api/stats/history` when necessary, then performs one `/api/stats` fetch and
  fans that one decoded response into all renderers.
- `morphLivePane()` in `src/bands/shell/document-3.rs` owns structural
  `elements.changed` replacement. A Stats element wrapper preserves its live
  descendants, including charts and readouts; a structural pull must not be
  used as a data update.
- `elements.changed` calls `pullHeldElementFragments()`. `stats.tick` calls
  `hydrateStats()` directly. Pulse pokes carry no data. Each data read remains
  a session-projected pull, and expensive truth must be held, collected, or
  TTL-governed rather than recomputed per tick.

The page-load fetch-flood incident is a relevant warning: DOM projection and
network refresh must not re-enter each other. This design therefore separates
scaffold discovery, data acquisition, and filling even where an option keeps
the same endpoint.

The two historical audit documents named by the governing brief are not
present in this base's documentation tree. This document does not describe
them as local files or use them as local evidence.

## Primitive invariants

Every option below obeys these invariants:

1. **Atomic admission remains atomic.** A guest shell and its owned scaffold
   cross the floor only through the existing handoff, fresh payload, healthy
   observation, and double-rAF path. A partially filled admitted guest is not a
   partially admitted guest.
2. **Progress is inside the guest.** A slot may be pending, ready, or faulted
   after the floor-2 scaffold exists. These are attributes or registry records
   local to an element. They never appear in `ImmortalFloor` and never gate a
   fifth transition.
3. **Structural and data work stay disjoint.** `morphLivePane` handles
   structural `elements.changed` pulls and preserves live descendants. Data
   pokes call a data filler directly; they do not morph structure. A structural
   refresh rebinds the filler after the morph rather than making the filler
   responsible for the morph.
4. **Pokes are invalidations, not payloads.** An event names a thing that may
   have changed. The browser performs a session-projected read and validates
   the response before filling. No event carries a Stats object or slot value.
5. **No hot-loop door.** A one-second `stats.tick` cannot cause unbounded
   source work. In-flight deduplication, held collector state, TTLs, and
   coalescing are required wherever a source is expensive.
6. **Wire ownership is explicit.** A browser-only attribute contract is not a
   public HTTP schema. If an option adds or changes a public event envelope,
   route payload, or other public wire schema, it requires an unresolved later
   ticket governed by `pali:fulcrum-schema-root-folder-law`; the one public seat
   is under `caduceus/schema/`, not Coronatio.

---

## Option 1 — bundled snapshot, progressive settlement

### Fragment declaration

Keep the existing Stats element response as the structural source, but make
its loading semantics explicit. The seven wrappers can declare slots without
changing the `/api/stats` response:

```html
<div class="stat-element" data-stat-element-id="cpu-chart"
     data-scaffold-root="stats"
     data-visible="true">
  <div class="stat-header">...</div>
  <div class="stat-content">
    <div data-scaffold-slot="stats.cpu.load"
         data-scaffold-state="pending"
         aria-busy="true">
      <span class="loading-spinner" role="progressbar">Loading CPU…</span>
    </div>
    <div data-scaffold-slot="stats.cpu.chart"
         data-scaffold-state="pending"
         aria-busy="true"></div>
  </div>
</div>
```

The other six wrappers use the same local declaration. A response key that is
present settles its corresponding slot independently; a key that is absent
stays `pending` and retains truthful placeholder text. “Absent” is not
silently converted to zero or an empty success state. A response-level fault
leaves the affected slots pending or marks them locally faulted according to
the existing truthful error policy.

### Client mechanism and genericity

`stat_element_templates()` remains the source of the seven skeletons. A small
Stats filler walks the admitted Stats root, maps known response paths to
`data-scaffold-slot` nodes, and invokes the existing render functions for each
available path. The loop is independent per key, so a malformed optional
branch cannot prevent an unrelated branch from rendering.

This option is only **locally generic**: the same slot/filler pattern can be
copied to another guest, but it does not yet define a guest-declared shared
registry. It is therefore a transport improvement for Stats, not the durable
cross-guest primitive.

### Poke, wire, and schema consequences

No public wire change is required. `stats.tick` remains data-free and calls the
Stats data filler through `hydrateStats()`. The existing session-projected
`/api/stats` response remains the one source read. `elements.changed` remains a
structural invalidation and continues to call `pullHeldElementFragments()`;
it never supplies the snapshot or calls a structural morph from a data filler.

The important limitation is temporal: a single bundled response has one
completion boundary. CPU cannot visibly settle before `/api/stats` resolves,
even if the CPU source was internally ready while roster or storage was slow.
Independent key settlement improves rendering after the response arrives, not
cross-source latency before that response arrives. This is a necessary honest
limit of the option.

### Stats before and after

**Before:**

```text
stat_element_templates()
  -> seven skeletons with dashes/spinners
hydrateStats()
  -> await /api/stats/history when needed
  -> await /api/stats
  -> renderCpuChart(data)
     renderNetwork(data)
     renderDiskIo(data)
     renderMemory(data)
     renderDiskUsage(data)
     renderStatsRoster()
     renderProcesses(data)
```

**After:**

```text
stat_element_templates()
  -> seven admitted skeletons with data-scaffold-slot="..."
     and data-scaffold-state="pending"
hydrateStats()
  -> await /api/stats/history when needed
  -> await /api/stats                         # still one boundary
  -> for each declared slot:
       if response path is present:
         fill(slot, value); slot.state = "ready"
       else:
         retain placeholder; slot.state = "pending"
  -> only the relevant renderers run for present keys
```

For example, `data.resources.load` can settle the CPU load/chart slots,
`data.network` the network slots, and `data.keaLeases.entries` the roster
slot. The seven element wrappers still originate in `stat_element_templates()`;
`hydrateStats()` still owns the one session-projected pull, but filling is
slot-by-slot rather than one all-or-nothing render branch.

### No fifth state

There is no new floor state. The Stats pane is admitted through the existing
`ImmortalFloor` flow. After the admitted floor-2 scaffold is present,
`data-scaffold-state="pending|ready|fault"` describes individual descendants
only. The floor remains `Seated` while slots settle, and a failed slot does not
create a parallel recovery machine.

### Cost and risk

This is the smallest change and has no transport fanout. It reduces visual
all-or-nothing behavior after a response and makes missing keys truthful. It
cannot provide true cross-source progressive loading, can preserve a large
response and server coupling, and leaves every future guest to invent its own
filler. It is a useful first rider but not a sufficient primitive by itself.

---

## Option 2 — independently pulled slot groups

### Fragment declaration

Declare coarse groups rather than one slot per network request. For example:

```html
<section data-scaffold-root="stats" data-stat-element-id="cpu-chart">
  <div data-scaffold-slot="stats.telemetry"
       data-scaffold-group="telemetry"
       data-scaffold-state="pending"
       data-scaffold-read="stats.telemetry">
    <span class="loading-spinner" role="progressbar">Loading telemetry…</span>
  </div>
</section>
<section data-scaffold-root="stats" data-stat-element-id="kea-leases">
  <div data-scaffold-slot="stats.roster"
       data-scaffold-group="roster"
       data-scaffold-state="pending"
       data-scaffold-read="stats.roster">
    <span class="loading-spinner" role="progressbar">Loading network activity…</span>
  </div>
</section>
```

The group is the request and invalidation unit; individual descendants may
still fill independently from the group payload. A practical first grouping
would separate fast telemetry from the roster, with history as its own
one-time or TTL-governed read. More groups are justified only by measured
source latency, not by the visual count of elements.

### Client mechanism and genericity

A group coordinator registers the mounted groups, deduplicates an in-flight
read per `(guest, group, session)`, and fans the returned group payload into
its slots. A group invalidation marks that group stale and schedules at most
one read. The coordinator must not issue one request per slot or one request
per browser tick.

The server-side collectors remain held or TTL-backed. A fast telemetry group
may read a held recent snapshot at a suitable cadence; a roster group may use
its own freshness window. The browser can coalesce several invalidations in
one microtask or short debounce window, while preserving a per-group request
fanout ceiling. Switching tabs retires the guest's outstanding group work as
part of the existing floor/guest lifecycle.

This option is more reusable than Option 1, but its abstraction is still
Stats-shaped: groups and routes are chosen around one guest's sources rather
than declared by any DrawnGuestTwo guest through a common contract.

### Poke, wire, and schema consequences

A data-free group invalidation can be delivered by a coalesced topic, for
example “telemetry changed” or “roster changed”. The browser then performs a
session-projected group fetch. The existing `stats.tick` may invalidate the
fast telemetry group, but it must not bypass the collector/TTL boundary. The
existing `elements.changed` topic remains structural only; it continues to
invoke `pullHeldElementFragments()` and `morphLivePane`, never a group data
morph.

Adding public group-specific event names or changing public group payloads is
an unresolved later wire/schema ticket. It must be governed by
`pali:fulcrum-schema-root-folder-law` and seat its one public schema in
`caduceus/schema/`, not in Coronatio. An implementation could initially reuse
existing data-free topics and private route contracts, but it must not hide a
new public envelope in Coronatio merely to make the prototype convenient.

The transport split also changes collector accounting. Two source groups mean
two freshness policies, two error faces, two route authorizations, and two
possible concurrent pulls. Coalescing prevents a request storm; it does not
make those costs disappear.

### Stats before and after

**Before:** `stat_element_templates()` emits all seven skeletons, and
`hydrateStats()` waits for history and then one `/api/stats` response before
rendering CPU, network, I/O, memory, storage, roster, and processes.

**After:**

```text
stat_element_templates()
  -> skeletons declare stats.telemetry and stats.roster groups
hydrateStats() / stats group coordinator
  -> ensure history read once if chart history is stale
  -> schedule telemetry group read (deduped, session-projected)
  -> schedule roster group read (deduped, session-projected)
  -> on telemetry response:
       fill CPU/network/I/O/memory/storage/process slots independently
  -> on roster response:
       fill stats.kea-leases roster independently
  -> keep missing fields pending and mark only failed group slots faulted
```

Here the roster can settle without waiting for telemetry, and fast telemetry
can settle without waiting for roster. `hydrateStats()` becomes an admission
and coordinator entry point rather than a monolithic renderer, while
`stat_element_templates()` remains the structural declaration source.

### No fifth state

The group coordinator starts only after the existing admitted floor-2 scaffold
exists. Its records are group/slot metadata, not `ImmortalFloor` states. A
roster timeout yields a roster-local fault and leaves the floor `Seated`; a
telemetry refresh does not reopen `GuestRevolution`. Unseating or changing
hosts uses the existing floor-owned teardown, not a second machine.

### Cost and risk

This is the only option that can provide genuine cross-source settlement
without waiting for a bundled `/api/stats` response. The cost is substantial:
new or split server reads, collector cadence decisions, route and authorization
contracts, invalidation topics, response fanout, freshness races, and more
observability. If the groups are chosen only for today's Stats fields, the
system gains transport complexity while remaining a special-purpose Stats
solution. Per-slot invalidation would be especially risky; group invalidation
is the lower-fanout form, but it still multiplies moving parts.

---

## Option 3 — generic guest-declared scaffold slots

### Fragment declaration

Define a small browser contract that any DrawnGuestTwo guest can declare in its
own admitted fragment. The contract is declarative and does not dictate the
source transport:

```html
<section data-guest-id="stats"
         data-scaffold-root
         data-scaffold-version="1">
  <div class="stat-element" data-stat-element-id="cpu-chart">
    <div class="stat-header">...</div>
    <div class="stat-content">
      <div data-scaffold-slot="stats.cpu.load"
           data-scaffold-key="cpu.load"
           data-scaffold-state="pending"
           data-scaffold-fill="text"
           aria-busy="true">Loading CPU…</div>
      <div data-scaffold-slot="stats.cpu.chart"
           data-scaffold-key="cpu.chart"
           data-scaffold-state="pending"
           data-scaffold-fill="stats.cpu.chart"
           aria-busy="true"></div>
    </div>
  </div>
</section>
```

The generic attributes are intentionally small:

- `data-scaffold-root` identifies the guest-owned root to bind after admission.
- `data-guest-id` identifies the already-admitted guest, not a new lifecycle.
- `data-scaffold-slot` is the stable DOM identity; `data-scaffold-key` is the
  adapter's value key.
- `data-scaffold-state` is local `pending`, `ready`, or `fault` metadata.
- `data-scaffold-fill` selects a registered filler, not arbitrary executable
  markup. Unknown fillers fail locally and visibly.
- Optional `data-scaffold-group` and `data-scaffold-placeholder` refine
  grouping and accessibility without making a request by themselves.

A guest registers its adapter with a shared client registry, for example:

```js
scaffoldRegistry.registerGuest({
  guest: 'stats',
  root: '[data-scaffold-root][data-guest-id="stats"]',
  read: async ({ session, signal }) => statsAdapter.readBundled(session, signal),
  fillers: {
    'text': fillText,
    'stats.cpu.chart': fillCpuChart,
    'stats.roster': fillRoster
  }
});
```

The registry is generic; the adapter and fillers are guest-owned. Registration
must not be treated as admission. The existing floor code first admits the
fresh fragment and observes it healthy. Only after the floor-2 scaffold exists
does the registry bind slots and allow progressive reads.

### Client mechanism and genericity

The shared registry owns a narrow lifecycle:

1. **Bind:** discover the admitted guest root and its declared slots; initialize
   missing state to `pending` without replacing the guest subtree.
2. **Read:** invoke the guest adapter's session-projected read, with an
   `AbortSignal`, in-flight deduplication, and the adapter's held/TTL policy.
3. **Fill:** for each present key, call the registered filler and set that slot's
   local state to `ready`; absent keys retain `pending` with an accessible
   explanation.
4. **Fault:** record a slot-local or group-local `fault`, preserve the last
   truthful value where policy permits, and expose retry through the same
   adapter. Never change `ImmortalFloor` for a slot fault.
5. **Rebind and retire:** after `morphLivePane` structurally updates a held
   fragment, rebind the registry to new declarations while preserving live
   descendants; when the guest is unseated, abort and retire only that guest's
   registrations.

A generic guest may choose one bundled read today, several group reads later,
or a non-Stats source, without changing the registry contract. The registry
never fetches a URL named in arbitrary markup and never smuggles data through a
pulse event. It is a client primitive, not a second admission or state
machine.

### Poke, wire, and schema consequences

The first rider can use the existing bundled `/api/stats` response and the
existing data-free `stats.tick`. The registry invokes the Stats adapter on the
poke; the adapter performs the same session-projected pull and fills slots.
`elements.changed` remains exclusively structural: it asks held consumers for a
fragment, passes it through `morphLivePane`, and then rebinds the registry. A
data poke never calls `morphLivePane`.

The DOM attributes and registration API are private client contracts in this
rider. They do not require a public HTTP schema. If a later adapter introduces
new group routes or public invalidation envelopes, that is a separate,
unresolved later ticket under `pali:fulcrum-schema-root-folder-law`; its one
public schema belongs under `caduceus/schema/`, never in Coronatio. This keeps
the durable primitive from forcing an early wire split.

### Stats before and after

**Before:** `stat_element_templates()` supplies seven static wrappers;
`hydrateStats()` serially awaits history and `/api/stats`, then directly calls
all Stats renderers. The DOM has incidental placeholders but no shared slot
lifecycle.

**After, first bundled rider:**

```text
admit Stats through existing ImmortalFloor flow
  -> fresh fragment contains data-scaffold-root and data-scaffold-slot markers
  -> healthy observation + existing double-rAF completes admission
  -> scaffoldRegistry.bind(stats)                 # now permitted

stat_element_templates()
  -> still emits the seven wrappers, now with generic slot markers

hydrateStats()
  -> Stats adapter performs the existing history/read sequence
  -> registry.fill(response):
       stats.cpu.*       -> CPU filler, state ready when keys exist
       stats.network.*   -> network filler, state ready when keys exist
       stats.io.*        -> I/O filler
       stats.memory.*    -> memory filler
       stats.storage.*   -> disk filler
       stats.kea-leases  -> roster filler
       stats.processes   -> process filler
       missing keys      -> remain pending; read/fault status stays local

on stats.tick: adapter refreshes data directly
on elements.changed: pull fragment -> morphLivePane -> registry rebind
```

The first rider deliberately does not promise cross-source progress: the
bundled response still has its one completion boundary, as in Option 1. It
lands the stable marker, filler, and lifecycle semantics before transport is
split. A later adapter can read groups without changing the markup contract or
floor law.

### No fifth state

The registry has no authority over `ImmortalFloor`. `BootFloor`, `GuestRevolution`,
`Seated`, and `BareFloor` remain the sole floor states. `pending`, `ready`, and
`fault` are attributes or registry records beneath an already admitted guest;
they cannot reveal a guest, seat one, or create a failure state beside the
floor. Registry bind/retire follows the existing handoff and teardown and does
not create an independent lifecycle.

### Cost and risk

This is the most durable primitive but introduces a shared client abstraction,
registration ownership, filler validation, rebind rules, and a need to keep
accessibility truthful for pending and faulted slots. A generic registry can
become an accidental framework if it acquires source policy, floor authority,
or arbitrary markup execution. Those boundaries must remain explicit. The
first bundled rider also does not improve cross-source latency; that payoff is
intentionally deferred to a later adapter and wire review.

---

## Comparison and recommendation

| Criterion | Option 1: bundled snapshot | Option 2: independent groups | Option 3: generic registry |
|---|---|---|---|
| Cross-source progress before `/api/stats` completes | No | Yes | Not in first bundled rider; available to later adapters |
| Transport churn | Minimal | Highest | Minimal initially; extensible later |
| Reuse beyond Stats | Low | Medium, but Stats-shaped | High; guest-declared |
| Public wire/schema pressure | None initially | Likely route/topic/schema work | None initially; deferred per adapter |
| Main failure mode | False impression of progressive source latency | Fanout, cadence, and freshness complexity | Registry becomes a second authority if boundaries slip |
| Fit with current morph/pulse law | Good | Good only with strict separation | Good if registry never morphs or carries payloads |

**Recommendation: Option 3, with its first rider over today's bundled
`/api/stats` source.** This is one recommendation, not a recommendation to
ship two competing primitives. The durable decision is the generic
`data-scaffold-slot` declaration plus shared registry/filler lifecycle. The
initial transport deliberately stays bundled so marker semantics, local slot
state, structural rebind, and guest lifecycle can be established without
multiplying public routes or topics.

Option 1 alone cannot produce true cross-source progress: independent source
latency is hidden behind the one `/api/stats` response. Option 2 alone can
produce that progress, but it immediately multiplies transport, invalidation,
collector cadence, authorization, freshness, and schema concerns while still
encoding a Stats-specific grouping model. Option 3 separates the durable DOM
and client contract from that transport choice. When measured latency justifies
it, an independently pulled adapter can be added behind the same registry and
reviewed as its own wire/schema ticket.

The weighed downside is real: Option 3 asks Coronatio to maintain a shared
client seam and makes lifecycle mistakes more subtle. The guardrail is that the
registry begins only after admitted floor-2 content, owns only slot metadata
and filling, and remains subordinate to the four-state `ImmortalFloor`;
`morphLivePane` remains the sole structural morph path and pulse events remain
data-free invalidations.

## Explicit non-scope and later tickets

- No fifth `ImmortalFloor` state, parallel guest machine, or slot-owned
  admission path.
- No change here to Rust, JavaScript, CSS, routes, collectors, pulse topics,
  or public schemas.
- No public schema is authored in Coronatio. Any later public event or payload
  change must be a separate ticket under
  `pali:fulcrum-schema-root-folder-law`, seated in `caduceus/schema/`.
- No claim is made that the historical audit documents absent from this base
  were inspected locally.
- This file records the design and its boundaries only; implementation and
  verification belong to a separately authorized change.
