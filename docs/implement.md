Korlix Workflow Inspector — Phase 1 Technical Implementation Specification

Document status: Engineering design baselinePhase: Phase 1 — Korlix-native implementationPrimary implementation language: RustLast updated: 2026-08-01

1. Implementation objective

Implement a compiler-integrated subsystem that converts resolved Korlix programs into a deterministic Workflow IR, validates the graph, emits runtime instrumentation in development builds, collects execution events locally, and serves an interactive inspector.

The implementation must preserve a strict separation between:

Korlix compiler-specific semantic data.

Universal workflow representation.

Runtime execution events.

UI presentation.

AI explanation.

2. Assumptions requiring confirmation

This specification assumes that Korlix has or will have:

A lexer and parser.

A resolved AST or semantic IR.

Stable resolved symbol identities.

Typed expressions or equivalent semantic classification.

A code-generation phase.

A development server or CLI.

Project-relative source file identities.

A way to map generated code back to Korlix source.

Before coding begins, replace assumed names with actual compiler structures.

Required preparatory artifact:

Korlix semantic node
→ resolved meaning
→ workflow node/effect
→ outgoing control edges
→ failure behavior
→ source span
→ instrumentation location

3. Workspace structure

korlix/
├── crates/
│   ├── korlix-workflow-schema/
│   ├── korlix-workflow-lowering/
│   ├── korlix-workflow-normalize/
│   ├── korlix-workflow-validate/
│   ├── korlix-workflow-instrument/
│   ├── korlix-workflow-runtime/
│   ├── korlix-workflow-sarif/
│   └── korlix-workflow-test-fixtures/
│
├── packages/
│   ├── workflow-inspector-ui/
│   └── workflow-client-runtime/
│
└── apps/
    └── workflow-inspector-server/

Crate responsibilities

Crate

Responsibility

korlix-workflow-schema

Versioned Workflow IR, runtime event schema, diagnostics

korlix-workflow-lowering

Korlix semantic IR → control/effect graph

korlix-workflow-normalize

Entry/terminal normalization, block collapse, canonical ordering

korlix-workflow-validate

Graph properties and domain rules

korlix-workflow-instrument

Static node → generated-code instrumentation plan

korlix-workflow-runtime

Event validation, redaction, deduplication, trace assembly

korlix-workflow-sarif

Diagnostic → SARIF conversion

korlix-workflow-test-fixtures

Shared fixture loading and assertion helpers

No UI crate may import Korlix AST or compiler-internal types.

4. End-to-end data flow

src/**/*.klx
    ↓
Korlix parser
    ↓
Resolved semantic program
    ↓
Workflow root discovery
    ↓
Per-root CFG/effect lowering
    ↓
Interprocedural reference linking
    ↓
Graph normalization
    ↓
Canonical Workflow IR
    ├─ validator
    ├─ JSON exporter
    ├─ SARIF exporter
    ├─ instrumentation planner
    └─ inspector server
            ↑
Generated development code
    ↓
Runtime events
    ↓
Local collector
    ↓
Trace assembly and static-node overlay

5. Workflow IR

5.1 Versioned envelope

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowDocument {
    pub schema_version: String,
    pub generator: GeneratorInfo,
    pub project: ProjectInfo,
    pub workflows: Vec<Workflow>,
    #[serde(default)]
    pub extensions: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratorInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectInfo {
    pub project_id: String,
    pub root_hash: String,
}

5.2 Workflow

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    pub id: WorkflowId,
    pub name: String,
    pub entry_node_id: NodeId,
    pub terminal_node_ids: Vec<NodeId>,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub source_roots: Vec<SourceSpan>,
    pub semantic_hash: String,
}

5.3 Node

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowNode {
    pub id: NodeId,
    pub kind: WorkflowNodeKind,
    pub label: String,
    pub symbol_id: Option<String>,
    pub source: Option<SourceSpan>,
    pub confidence: Confidence,
    pub effects: Vec<WorkflowEffect>,
    #[serde(default)]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

5.4 Node kinds

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowNodeKind {
    WorkflowStart,
    WorkflowEnd,
    Page,
    Layout,
    Component,
    UserEvent,
    FunctionCall,
    Condition,
    Loop,
    Validation,
    StateRead,
    StateWrite,
    ApiRequest,
    StorageRead,
    StorageWrite,
    Navigation,
    Timer,
    ParallelFork,
    ParallelJoin,
    ErrorHandler,
    ExternalCall,
    UnresolvedOperation,
}

5.5 Edge

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowEdge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    pub kind: WorkflowEdgeKind,
    pub guard: Option<Guard>,
    pub source: Option<SourceSpan>,
    #[serde(default)]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

5.6 Edge kinds

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowEdgeKind {
    Control,
    UserEvent,
    ConditionTrue,
    ConditionFalse,
    Success,
    Failure,
    Exception,
    DataDependency,
    StateDependency,
    AsyncSpawn,
    AsyncJoin,
    Navigation,
    Retry,
    Cancellation,
    Timeout,
}

5.7 Source locations

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceSpan {
    pub file: String,
    pub byte_start: u32,
    pub byte_end: u32,
    pub start: SourcePosition,
    pub end: SourcePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: u32,
    pub utf16_column: u32,
}

Use UTF-16 columns for LSP/editor interoperability, while retaining byte offsets for compiler operations.

5.8 Confidence

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    ConfirmedSemantic,
    ConfirmedRuntime,
    FrameworkInferred,
    Heuristic,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Confidence {
    pub level: ConfidenceLevel,
    pub reason: String,
}

Phase 1 Korlix-native nodes should normally be confirmed_semantic. Dynamic targets must remain unresolved.

6. Stable identities

6.1 Prohibited identity inputs

Do not use:

Source line number

Source column

Array order after parsing

Random UUID

Runtime timestamp

Display label alone

6.2 Semantic identity input

project namespace
+ module path
+ resolved symbol path
+ workflow root
+ semantic child path
+ node kind
+ effect discriminator

Example canonical identity string:

shop::checkout::Checkout::on_pay::try[0]::api_request[0]

Hash using a documented stable algorithm, for example BLAKE3 with a versioned namespace:

fn node_id(identity: &str) -> NodeId {
    let namespaced = format!("korlix.workflow.node.v1\0{identity}");
    NodeId(format!("wf_{}", &blake3::hash(namespaced.as_bytes()).to_hex()[..16]))
}

6.3 Identity tests

Comments do not change IDs.

Whitespace does not change IDs.

CRLF/LF conversion does not change IDs.

Inserting an unrelated declaration does not change existing IDs.

Moving a semantic operation to another owner changes its ID.

Two identical calls in one handler receive distinct semantic child paths.

7. Lowering pipeline

7.1 Pass 1 — Workflow root discovery

Discover roots such as:

Page load

Route entry

Component lifecycle entry

User event handlers

Explicit workflow declarations

Timer callbacks

Background task entry points

Output:

struct WorkflowRoot {
    id: WorkflowId,
    name: String,
    owner_symbol: ResolvedSymbolId,
    entry_semantic_node: SemanticNodeId,
    source: SourceSpan,
}

7.2 Pass 2 — Local control-flow lowering

For each root, create a compiler-level control/effect graph.

Rules:

Sequential operations create control edges.

Conditions create guarded true/false edges.

Early return creates a branch terminal.

try/catch/finally creates exception and cleanup paths.

Loops create back edges and explicit exits.

Awaited operations create async start/completion semantics.

Parallel constructs create fork and join nodes.

Navigation creates a route edge and may create a workflow terminal depending on Korlix semantics.

7.3 Pass 3 — Effect classification

Classify resolved operations:

enum SemanticEffect {
    StateRead { symbol: ResolvedSymbolId },
    StateWrite { symbol: ResolvedSymbolId, value_kind: ValueKind },
    ApiRequest { method: HttpMethod, route: TargetResolution },
    Navigation { target: TargetResolution },
    StorageRead { store: String },
    StorageWrite { store: String },
    Timer { duration: DurationResolution },
    ExternalCall { symbol: ResolvedSymbolId },
    Validation,
}

7.4 Pass 4 — Interprocedural linking

Use resolved symbols to connect calls to known Korlix functions.

Policy:

Inline only small workflow-relevant callees for visualization.

Preserve call boundaries in the canonical IR.

Prevent recursive expansion.

Represent recursion as a classified cycle.

Mark external or dynamically selected calls as unresolved/external.

Apply configurable depth only to presentation projection, not semantic identity.

7.5 Pass 5 — Error propagation

Every operation classified as failure-capable must produce one of:

Local failure edge

Exception edge to handler

Explicit propagated-error terminal

Declared non-failing proof from semantics

Do not emit a missing-handler warning if the failure is intentionally propagated and represented.

7.6 Pass 6 — Normalization

Normalization operations:

Add one synthetic entry node.

Add explicit terminal nodes where needed.

Collapse compiler-only blocks with no user-facing effect.

Preserve guards when collapsing.

Classify cycles.

Canonically sort nodes and edges by stable ID.

Compute semantic hash.

Validate graph invariants.

Normalization must be idempotent:

normalize(normalize(G)) = normalize(G)

8. Graph validation engine

8.1 Validator interface

pub trait WorkflowRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn default_severity(&self) -> Severity;
    fn evaluate(&self, ctx: &ValidationContext<'_>, out: &mut Vec<Diagnostic>);
}

8.2 Diagnostic structure

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Diagnostic {
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    pub workflow_id: WorkflowId,
    pub node_id: Option<NodeId>,
    pub primary_source: Option<SourceSpan>,
    pub related_sources: Vec<RelatedSource>,
    pub evidence: serde_json::Value,
    pub remediation: Option<String>,
}

8.3 Initial rule catalogue

Rule

Severity

Description

WF001_DANGLING_EDGE

Error

Edge endpoint does not exist

WF002_DUPLICATE_NODE_ID

Error

Duplicate node identity

WF003_NO_ENTRY

Error

Missing normalized entry

WF004_NO_TERMINAL_PATH

Error

No reachable completion

WF005_UNSAFE_REDIRECT_CYCLE

Error

Automatic route cycle without exit

WF006_UNBOUNDED_RETRY

Error

Retry lacks bound, timeout, or cancellation

WF007_INVALID_SOURCE_SPAN

Error

Invalid or unmappable source range

WF008_INSTRUMENTATION_MISMATCH

Error

Instrumentation references missing static node

WF101_UNREACHABLE_NODE

Warning

Node not reachable from entry

WF102_MISSING_FALSE_BRANCH

Warning

Binary branch has no represented false continuation

WF103_API_WITHOUT_ERROR_PATH

Warning

Failure-capable request has no handler/propagation

WF104_LOADING_NOT_CLEARED

Warning

Terminal path leaves loading state active

WF105_DESTRUCTIVE_WITHOUT_CONFIRMATION

Warning

Destructive event lacks confirmation

WF106_ERROR_NOT_RENDERED

Warning

Error state is written but never consumed visibly

WF107_DUPLICATE_SUBMISSION

Warning

Active mutation does not disable/reject repeat submit

WF108_ASYNC_NOT_JOINED

Warning

Async work is unclassified

WF109_WRITE_NEVER_READ

Warning

Workflow-local state write has no read

WF110_DYNAMIC_TARGET_UNRESOLVED

Warning

Target cannot be statically resolved

WF111_LOOP_WITHOUT_VISIBLE_EXIT

Warning

No demonstrable loop exit

WF112_EMPTY_SUCCESS_PATH

Warning

Success produces no visible, state, return, or navigation effect

8.4 Validation algorithms

Implement reusable graph indexes:

pub struct GraphIndex<'a> {
    pub nodes: HashMap<&'a NodeId, &'a WorkflowNode>,
    pub outgoing: HashMap<&'a NodeId, Vec<&'a WorkflowEdge>>,
    pub incoming: HashMap<&'a NodeId, Vec<&'a WorkflowEdge>>,
}

Required algorithms:

DFS/BFS reachability

Reverse reachability from terminals

Strongly connected components

Dominator/post-dominator analysis where required

Path-sensitive state tracking with bounded lattice values

Fork/join pairing

Route-transition cycle classification

For loading cleanup, use a small abstract state:

enum AbstractBool {
    Unknown,
    False,
    True,
    Conflict,
}

Propagate state along control edges. Report when a terminal is reachable with True.

9. Runtime instrumentation

9.1 Development-only behavior

Instrumentation is enabled only when explicitly requested:

korlix dev --workflow

Production builds must not emit workflow events unless a future production mode is separately designed and enabled.

9.2 Instrumentation plan

The compiler generates an intermediate plan:

pub struct InstrumentationPlan {
    pub workflow_id: WorkflowId,
    pub points: Vec<InstrumentationPoint>,
}

pub struct InstrumentationPoint {
    pub node_id: NodeId,
    pub generated_location: GeneratedLocation,
    pub event_policy: RuntimeEventPolicy,
}

9.3 Runtime event model

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub protocol_version: String,
    pub event_id: String,
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub workflow_id: WorkflowId,
    pub node_id: NodeId,
    pub kind: RuntimeEventKind,
    pub monotonic_ns: u64,
    pub wall_time: Option<String>,
    pub status: Option<RuntimeStatus>,
    pub attributes: serde_json::Map<String, serde_json::Value>,
}

Event kinds:

pub enum RuntimeEventKind {
    Started,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

9.4 Semantics-preserving wrapper

Conceptual generated code:

const span = __korlixFlow.start({
  workflowId: "checkout.payment",
  nodeId: "wf_74d9c771ce3b8df2"
});

try {
  const value = await createOrder(input);
  __korlixFlow.complete(span);
  return value;
} catch (error) {
  __korlixFlow.fail(span, classifyError(error));
  throw error;
}

Required guarantees:

Return value is unchanged.

Thrown value/error is unchanged.

Side-effect count and order are unchanged.

Promise/task settlement semantics are unchanged.

Cancellation remains cancellation.

Instrumentation failure cannot fail the application operation.

Runtime event emission is non-blocking or bounded.

9.5 Context propagation

Within Phase 1, use trace/span parentage inside Korlix-generated execution.

Recommended compatibility:

16-byte trace ID

8-byte span ID

W3C-compatible formatting where HTTP propagation exists

No requirement for a full OpenTelemetry SDK in the first implementation

This keeps the protocol ready for Phase 2 and cross-service correlation.

10. Runtime collector

10.1 Server boundary

Default listener:

127.0.0.1:<ephemeral-or-configured-port>

Do not bind to all interfaces by default.

10.2 Ingestion pipeline

HTTP/WebSocket event
    ↓
Size limit
    ↓
Schema validation
    ↓
Protocol version validation
    ↓
Recursive redaction
    ↓
Event ID deduplication
    ↓
Static node lookup
    ↓
Trace assembly
    ↓
UI broadcast

10.3 Storage

Phase 1 can use bounded in-memory storage with optional local session export.

Recommended structures:

struct RuntimeStore {
    seen_event_ids: LruCache<String, ()>,
    traces: HashMap<String, RuntimeTrace>,
    max_traces: usize,
    max_events_per_trace: usize,
}

Do not require PostgreSQL, Redis, or cloud services for Phase 1.

10.4 Redaction policy

Default forbidden key patterns, case-insensitive:

password
passwd
secret
token
api_key
apikey
authorization
cookie
set-cookie
session
card_number
cvv
private_key

Also:

Strip query parameter values unless allowlisted.

Limit string length.

Limit nesting depth.

Limit object key count.

Replace exception-message secret matches.

Store route templates rather than user-specific URLs where possible.

Redaction must be idempotent.

11. CLI commands

Development

korlix dev --workflow

Static scan

korlix workflow scan

Validation

korlix workflow check
korlix workflow check --severity error

Export

korlix workflow export --format json
korlix workflow export --format sarif
korlix workflow export --format mermaid

Diff

korlix workflow diff origin/main HEAD

The diff command can be implemented after canonical graph stability is proven.

12. Inspector UI

12.1 Stack

Recommended:

React

Vite

React Flow

Zustand

WebSocket or Server-Sent Events

ELK layered layout for directional workflows

shadcn/ui primitives with custom styling

12.2 Palette

Token

Hex

Use

korlix-ink

#111318

Main background

korlix-surface

#1B1E25

Panels and nodes

korlix-red

#EF3E4A

Failure and primary action

trace-green

#27C281

Successful runtime execution

warning-amber

#F4A340

Warnings and unresolved paths

graph-mist

#A9B0BE

Inactive text and edges

12.3 Type

Inter for application controls and labels.

JetBrains Mono for source ranges, identifiers, guards, and timings.

12.4 Layout

Workflow navigator on the left, infinite graph canvas in the center, node evidence panel on the right, and runtime timeline at the bottom.

12.5 UI state contract

type RuntimeVisualState =
  | "not_executed"
  | "active"
  | "completed"
  | "failed"
  | "cancelled"
  | "timed_out"
  | "orphaned";

unresolved belongs to static confidence, not runtime status.

13. Source navigation

Requirements:

Store project-relative source file.

Store byte range.

Store LSP-compatible line/UTF-16 column range.

Expose an editor URL/action.

Preserve generated-code source maps where runtime stack mapping is needed.

Validate source spans against the source file hash used during extraction.

Potential editor URI:

vscode://file/<absolute-path>:<line>:<column>

Do not place absolute paths in shareable exports by default.

14. SARIF export

Map diagnostics as follows:

Korlix field

SARIF field

Rule ID

ruleId

Severity

level

Message

message.text

Source span

locations[].physicalLocation

Related source

relatedLocations

Workflow/node metadata

properties

Suggested correction

fixes or message guidance when safe

Run a SARIF schema validator in CI.

15. Incremental analysis

15.1 Cache key

compiler semantic version
+ workflow schema version
+ module semantic hash
+ relevant dependency hashes
+ workflow extractor configuration

15.2 Invalidation

Invalidate:

Changed workflow root.

Changed referenced callee with workflow effects.

Route table changes affecting target resolution.

State declaration changes affecting dependency analysis.

Do not invalidate unrelated workflows.

15.3 Merge

When re-extracting one workflow:

Remove prior graph by workflow ID.

Insert normalized replacement.

Preserve stable IDs for unchanged semantic nodes.

Re-run only affected inter-workflow validation.

Broadcast a graph patch rather than the entire graph where practical.

16. Implementation sequence

Milestone 0 — Semantic mapping

Deliver:

Actual Korlix semantic node catalogue.

Workflow root definition.

Failure semantics.

Async semantics.

Route semantics.

State semantics.

Source span guarantees.

Exit criterion: every supported construct has an approved lowering rule.

Milestone 1 — Schema and canonicalization

Deliver:

korlix-workflow-schema

JSON schema

Canonical sorter

Semantic hashing

Stable IDs

Round-trip and identity tests

Exit criterion: reviewed hand-written graphs round-trip deterministically.

Milestone 2 — Static lowering

Deliver:

Root discovery

Sequential flow

Conditions

Calls

State effects

API effects

Navigation

Error propagation

Source mapping

Exit criterion: baseline fixture corpus produces approved graphs.

Milestone 3 — Normalization and validation

Deliver:

Synthetic entry/terminal

Reachability

SCC cycle classification

Branch checks

Cleanup state analysis

Initial rules

JSON/SARIF diagnostics

Exit criterion: positive and negative fixtures pass without snapshot-only assertions.

Milestone 4 — Runtime instrumentation

Deliver:

Instrumentation plan

Generated wrappers

Runtime protocol

Local collector

Redaction

Deduplication

Differential tests

Exit criterion: instrumented and baseline behavior is equivalent over the test corpus.

Milestone 5 — Inspector

Deliver:

Workflow navigation

Graph rendering

Source panel

Diagnostic panel

Runtime overlay

Timeline

Filters

Large-graph grouping

Exit criterion: three end-to-end projects can be inspected without manual graph editing.

Milestone 6 — CI and hardening

Deliver:

SARIF export

Fuzz targets

Mutation testing

Performance benchmarks

Compatibility/migration tests

Release documentation

Exit criterion: all quality gates in the test specification pass.

17. Definition of implementation complete

Implementation is complete only when:

No Korlix AST type leaks into the universal schema or UI.

Canonical graph output is deterministic.

Stable IDs survive non-semantic edits.

Required validator rules have positive and negative fixtures.

Runtime instrumentation passes differential tests.

Runtime collection passes security fixtures.

The inspector links every source-backed node to source.

CI can fail on configured workflow errors.

Schema migration behavior is documented.

Phase 2 can implement an adapter that produces the same Workflow IR without importing Korlix compiler internals.

18. References

Rust MIR: https://rustc-dev-guide.rust-lang.org/mir/index.html

Rust MIR dataflow: https://rustc-dev-guide.rust-lang.org/mir/dataflow.html

Rust RFC 1211: https://rust-lang.github.io/rfcs/1211-mir.html

OpenTelemetry traces: https://opentelemetry.io/docs/concepts/signals/traces/

OpenTelemetry context propagation: https://opentelemetry.io/docs/concepts/context-propagation/

W3C Trace Context: https://www.w3.org/TR/trace-context/

OASIS SARIF 2.1.0: https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html

ECMA-426 Source Map Format: https://ecma-international.org/publications-and-standards/standards/ecma-426/

React Flow: https://reactflow.dev/

ELK layered layout reference: https://eclipse.dev/elk/reference/algorithms/org-eclipse-elk-layered.html
