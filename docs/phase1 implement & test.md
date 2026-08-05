Korlix Workflow Inspector — Phase 1 Test Properties and Acceptance Catalogue

Document status: QA and verification baselinePhase: Phase 1 — Korlix-native implementationLast updated: 2026-08-01

1. Verification objective

Prove that Phase 1:

Extracts the correct workflow.

Produces stable and deterministic graphs.

Detects required structural defects.

Preserves program behavior during instrumentation.

Safely handles malformed runtime input.

Does not leak known secret classes.

Scales within defined performance gates.

Remains compatible across schema evolution.

Snapshot tests alone are insufficient. The test system must combine:

Unit tests

Golden fixtures

Semantic assertions

Property-based tests

Differential tests

Integration tests

Fuzzing

Mutation testing

Performance benchmarks

End-to-end sample projects

2. Required graph properties

ID

Property

Required behavior

WF-P01

Referential integrity

Every edge endpoint references an existing node

WF-P02

Unique identity

Node and edge IDs are unique within the document

WF-P03

Deterministic extraction

Identical semantic input produces byte-identical canonical JSON

WF-P04

Stable semantic IDs

Non-semantic source changes do not alter node IDs

WF-P05

Valid source span

Source-backed nodes use valid ordered ranges

WF-P06

Single normalized entry

Each workflow has exactly one entry

WF-P07

Reachability

Every executable node is reachable from entry

WF-P08

Completion availability

At least one terminal is reachable

WF-P09

Branch completeness

Conditions retain every semantic continuation

WF-P10

Failure completeness

Failure-capable effects have handler, propagation, or proof of no failure

WF-P11

Async closure

Async work is joined, cancelled, or explicitly detached

WF-P12

Cycle classification

Every cycle is classified

WF-P13

Retry boundedness

Retry has max attempts, timeout, cancellation, or explicit infinite policy

WF-P14

State explainability

State writes have an identifiable triggering path

WF-P15

Terminal cleanliness

Required transient states are not left active at normal/error completion

WF-P16

Runtime causality

Non-root runtime spans have a valid parent or declared link

WF-P17

Runtime idempotence

Repeated delivery does not duplicate logical events

WF-P18

Instrumentation transparency

Instrumentation preserves values, errors, effects, and ordering

WF-P19

Redaction safety

Forbidden secret fixtures do not appear in emitted telemetry

WF-P20

Schema round trip

Encode/decode preserves semantic equality

WF-P21

Forward compatibility

Unknown optional fields do not break compatible readers

WF-P22

Incremental locality

Unrelated workflows retain identities after a local edit

WF-P23

Canonical order independence

Input collection ordering does not change canonical output

WF-P24

Normalization idempotence

Normalizing a normalized graph changes nothing

WF-P25

Diagnostic stability

Same invalid semantic input produces the same rule IDs and primary locations

3. Test repository layout

crates/korlix-workflow-test-fixtures/
├── fixtures/
│   ├── extraction/
│   ├── validation/
│   ├── runtime/
│   ├── security/
│   ├── serialization/
│   └── end_to_end/
├── expected/
│   ├── graphs/
│   ├── diagnostics/
│   ├── instrumentation/
│   └── sarif/
└── src/
    ├── fixture.rs
    ├── semantic_assertions.rs
    └── canonical.rs

fuzz/
├── fuzz_targets/
│   ├── workflow_json.rs
│   ├── runtime_event.rs
│   ├── normalize_graph.rs
│   ├── redact_payload.rs
│   └── source_span.rs
└── corpus/

benches/
├── extraction.rs
├── validation.rs
├── runtime_ingestion.rs
└── canonicalization.rs

4. Fixture contract

Every source fixture must include:

input.klx
expected.graph.json
expected.diagnostics.json
expected.instrumentation.json
fixture.toml

Example fixture.toml:

id = "EXT-020"
title = "API success and failure"
expected_properties = ["WF-P01", "WF-P03", "WF-P09", "WF-P10"]
expected_rules = []
unsupported = false

Tests must assert both:

Canonical JSON matches the reviewed output.

Semantic invariants hold independently of the snapshot.

5. Static extraction test catalogue

5.1 Basic flow

Test

Input

Expected graph

Properties

EXT-001

Empty page

Entry → page → terminal

P01, P03, P06, P08

EXT-002

One click handler

Event trigger linked to operation

P07, P14

EXT-003

Three sequential writes

Source execution order retained

P03, P14

EXT-004

Resolved function call

Caller linked to known callee

P01, P07

EXT-005

Early return

Branch-specific terminal

P08, P09

EXT-006

Nested component event

Ownership and trigger preserved

P05, P07

EXT-007

No-op compiler block

Block collapsed without changing flow

P24

EXT-008

Two identical calls

Distinct stable IDs

P02, P04

EXT-009

External call

External node with explicit confidence

P01

5.2 Conditions

Test

Input

Expected

Properties

EXT-010

if/else

True and false edges

P09

EXT-011

if without else

False continuation generated

P09

EXT-012

Nested conditions

Guard identity retained

P03, P09

EXT-013

Match/case

Case edges plus exhaustive/default marker

P09

EXT-014

Constant false branch

Dead branch diagnostic

P07

EXT-015

Condition with early return

Correct continuation after join

P08

EXT-016

Compound boolean guard

Guard retained without unsafe simplification

P09

EXT-017

Dynamic guard

Condition node remains confirmed; value remains runtime-dependent

P09

5.3 Error and cleanup

Test

Input

Expected

Properties

EXT-020

API success/error clauses

Success and failure edges

P10

EXT-021

API without local handler

Propagated-error path

P10

EXT-022

Try/catch

Exception enters catch

P10

EXT-023

Try/finally

Every exit crosses finally

P15

EXT-024

Loading reset in finally

Clean success and failure terminals

P15

EXT-025

Loading reset only on success

WF104_LOADING_NOT_CLEARED

P15

EXT-026

Catch rethrows

Catch plus propagated error terminal

P10

EXT-027

Nested try/catch

Exception targets nearest compatible handler

P10

EXT-028

Non-failing operation

No artificial failure edge

P10

5.4 Loops and retries

Test

Input

Expected

Properties

EXT-030

Counted loop

Back edge and exit

P12

EXT-031

While loop

Guarded loop and false exit

P09, P12

EXT-032

Unconditional infinite loop

No-terminal or loop diagnostic

P08, P12

EXT-033

Retry max 3

Bounded retry classification

P13

EXT-034

Retry without bound

WF006_UNBOUNDED_RETRY

P13

EXT-035

User refresh cycle

Event recurrence, not unsafe loop

P12

EXT-036

Loop with break

Break exits correct loop

P12

EXT-037

Nested loops

Back edges target correct headers

P12

EXT-038

Continue

Continue targets current loop guard

P12

EXT-039

Retry with timeout

Bounded by timeout path

P13

5.5 Async and parallel

Test

Input

Expected

Properties

EXT-040

Awaited call

Async operation completion represented

P11

EXT-041

Two parallel calls

Fork, siblings, join

P11

EXT-042

Fire-and-forget task

Explicit detached classification

P11

EXT-043

Async failure

Failure edge retained

P10

EXT-044

Cancellation

Cancellation reaches cleanup

P11, P15

EXT-045

Timeout

Timeout differs from general failure

P10

EXT-046

Parallel one-branch failure

Join/failure policy preserved

P10, P11

EXT-047

Race construct

Winner and cancellation semantics preserved

P11

EXT-048

Spawn without classification

WF108_ASYNC_NOT_JOINED

P11

5.6 Routes and UI

Test

Input

Expected

Properties

EXT-050

Known navigation

Connected target page

P01

EXT-051

Unknown route

Unresolved target warning

P01

EXT-052

Automatic redirect cycle

WF005_UNSAFE_REDIRECT_CYCLE

P12

EXT-053

Layout wraps page

Ownership retained

P07

EXT-054

Shared component

Contextual event instances remain distinct

P02

EXT-055

Modal open/close

Valid close terminal or return path

P08

EXT-056

Destructive action with confirmation

No WF105

P09

EXT-057

Destructive action without confirmation

WF105_DESTRUCTIVE_WITHOUT_CONFIRMATION

P09

EXT-058

Error written and rendered

No WF106

P14

EXT-059

Error written but never rendered

WF106_ERROR_NOT_RENDERED

P14

5.7 Dynamic and unsupported behavior

Test

Input

Expected

Properties

EXT-060

Dynamic route lookup

Unresolved navigation node

P25

EXT-061

Runtime-generated function

Unresolved operation

P25

EXT-062

Unsupported syntax node

Explicit diagnostic, no panic

P25

EXT-063

Partially resolved call

Known owner with unresolved target

P25

EXT-064

Reflection/plugin dispatch

External/unresolved, never guessed

P25

6. Validator test catalogue

Test

Graph mutation

Expected rule

VAL-001

Remove edge target node

WF001_DANGLING_EDGE

VAL-002

Duplicate node ID

WF002_DUPLICATE_NODE_ID

VAL-003

Remove entry

WF003_NO_ENTRY

VAL-004

Remove all paths to terminal

WF004_NO_TERMINAL_PATH

VAL-005

Create automatic A→B→A navigation

WF005_UNSAFE_REDIRECT_CYCLE

VAL-006

Add retry back edge without bound

WF006_UNBOUNDED_RETRY

VAL-007

Reverse source span start/end

WF007_INVALID_SOURCE_SPAN

VAL-008

Instrument missing node

WF008_INSTRUMENTATION_MISMATCH

VAL-101

Add isolated executable node

WF101_UNREACHABLE_NODE

VAL-102

Remove false continuation

WF102_MISSING_FALSE_BRANCH

VAL-103

Remove API failure/propagation edge

WF103_API_WITHOUT_ERROR_PATH

VAL-104

Terminal reachable with loading=true

WF104_LOADING_NOT_CLEARED

VAL-105

Destructive call directly from event

WF105_DESTRUCTIVE_WITHOUT_CONFIRMATION

VAL-106

Error state write without visible read

WF106_ERROR_NOT_RENDERED

VAL-107

Submit remains enabled during mutation

WF107_DUPLICATE_SUBMISSION

VAL-108

Spawn with no join/detach

WF108_ASYNC_NOT_JOINED

VAL-109

State write with no read

WF109_WRITE_NEVER_READ

VAL-110

Unresolved API/route target

WF110_DYNAMIC_TARGET_UNRESOLVED

VAL-111

Loop SCC has no exit

WF111_LOOP_WITHOUT_VISIBLE_EXIT

VAL-112

Success reaches terminal with no effect

WF112_EMPTY_SUCCESS_PATH

Each negative test must have a corresponding positive control proving the rule does not over-trigger.

7. Runtime test catalogue

Test

Scenario

Expected

RUN-001

Start then complete

One completed span, valid duration

RUN-002

Parent with child

Valid parent-child relation

RUN-003

Operation throws

Failure event; original error rethrown

RUN-004

Concurrent siblings

Shared parent, no false sibling ordering

RUN-005

Events arrive out of order

Correct reconciliation or explicit incomplete state

RUN-006

Same event twice

One logical event

RUN-007

Start without end

Incomplete after configured timeout

RUN-008

End without start

Invalid event diagnostic; collector survives

RUN-009

Browser reload

New trace ID; same static workflow IDs

RUN-010

Runtime disabled

Zero emitted workflow events

RUN-011

Instrumented success

Same return value as baseline

RUN-012

Instrumented failure

Same thrown value/type as baseline

RUN-013

Runtime node missing statically

Orphaned runtime node

RUN-014

50,000 valid events

No loss inside configured capacity

RUN-015

Wall clock changes

Duration remains valid through monotonic clock

RUN-016

Collector unavailable

Application operation still succeeds/fails normally

RUN-017

Emitter throws internally

Application behavior unaffected

RUN-018

Cancellation

Original cancellation semantics retained

RUN-019

Timeout

Timeout classified separately

RUN-020

Trace reaches capacity

Defined eviction, no unbounded memory growth

8. Security test catalogue

Test

Input

Expected

SEC-001

password

Value redacted

SEC-002

Authorization

Value redacted

SEC-003

Cookie/session

Value redacted

SEC-004

Nested token

Recursive redaction

SEC-005

Mixed-case ApiKey

Case-insensitive redaction

SEC-006

Card/CVV fields

Values redacted

SEC-007

HTTP status

Retained

SEC-008

Route template

Retained

SEC-009

Query parameter token

Value removed

SEC-010

Secret in exception text

Secret replaced

SEC-011

Excessive nesting

Bounded processing

SEC-012

Oversized event

Rejected/truncated per policy

SEC-013

Default collector

Loopback binding

SEC-014

Cloud export absent

No external network request

SEC-015

Redact twice

Same output as redact once

SEC-016

Absolute project path export

Removed/project-relative by default

SEC-017

Malformed UTF-8 boundary

No panic or data escape

SEC-018

Large key count

Bounded object handling

SEC-019

Secret embedded in URL

User-specific/query values stripped

SEC-020

Unknown field

Retained only after generic safety limits

9. Serialization and compatibility tests

Test

Scenario

Expected

SER-001

JSON round trip

Semantic equality

SER-002

Object-key reorder

Same semantic hash

SER-003

Node/edge reorder

Same canonical output

SER-004

Unknown optional field

Compatible reader accepts

SER-005

Unknown required node kind

Explicit compatibility error

SER-006

Older supported schema

Explicit migration

SER-007

Newer unsupported schema

No silent interpretation

SER-008

SARIF export

Schema-valid result

SER-009

Unicode source

Correct byte and UTF-16 ranges

SER-010

CRLF vs LF

Same semantic graph

SER-011

Empty extension map

Canonical omission/default behavior

SER-012

Large graph

No stack overflow

SER-013

Duplicate JSON keys

Defined parser behavior/rejection

SER-014

Corrupted schema version

Clear error

SER-015

Migration twice

No second semantic change

10. Property-based test laws

10.1 Normalization idempotence

proptest! {
    #[test]
    fn normalization_is_idempotent(graph in valid_workflow_graph()) {
        let once = normalize(graph.clone()).unwrap();
        let twice = normalize(once.clone()).unwrap();
        prop_assert_eq!(once, twice);
    }
}

10.2 Serialization round trip

proptest! {
    #[test]
    fn json_round_trip_preserves_semantics(graph in valid_workflow_graph()) {
        let encoded = serde_json::to_vec(&graph).unwrap();
        let decoded: WorkflowDocument = serde_json::from_slice(&encoded).unwrap();
        prop_assert!(semantic_eq(&graph, &decoded));
    }
}

10.3 Canonical order independence

proptest! {
    #[test]
    fn canonicalization_ignores_collection_order(
        graph in valid_workflow_graph(),
        seed in any::<u64>(),
    ) {
        let shuffled = shuffle_graph(graph.clone(), seed);
        prop_assert_eq!(
            canonical_json(&graph).unwrap(),
            canonical_json(&shuffled).unwrap()
        );
    }
}

10.4 Isolated-node diagnostic

For any valid reachable graph, adding one isolated executable node must add exactly one unreachable-node finding for that node.

10.5 Branch mutation

Removing one semantic continuation from a binary condition must produce WF102_MISSING_FALSE_BRANCH or the corresponding missing-branch finding.

10.6 Stable source transformation

For generated valid source:

extract(source)
≡
extract(add_comments_whitespace_and_line_ending_changes(source))

Ignore physical source positions when comparing semantic graph identity.

10.7 Instrumentation transparency

For supported generated functions and generated inputs:

observable(baseline(input))
=
observable(instrumented(input))

Observable includes:

Return value

Thrown value/type

External side-effect count

External side-effect order

State writes

Cancellation

Timeout classification

10.8 Event merge idempotence

merge(E, E) = merge(E)

10.9 Event order tolerance

For events with valid explicit identifiers:

assemble(shuffle(E)) ≡ assemble(E)

Incomplete intermediate state is allowed; final assembled trace must be equivalent.

10.10 Redaction idempotence

redact(redact(P)) = redact(P)

10.11 Secret absence

For generated payloads containing forbidden values:

contains_forbidden_value(redact(P)) = false

10.12 Diff symmetry

invert(diff(A, B)) = diff(B, A)

10.13 Migration semantic preservation

semantic_hash(migrate_v1_to_v2(Gv1))
=
semantic_hash(expected_v2(Gv1))

10.14 Incremental locality

Applying a semantic edit to workflow A must not alter node IDs or semantic hashes of unrelated workflow B.

11. Differential instrumentation test example

Non-trivial wrappers must be tested against a real observable input.

#[derive(Debug, Clone, PartialEq)]
struct Observation {
    result: Result<i32, &'static str>,
    effects: Vec<&'static str>,
}

fn baseline(input: i32, effects: &mut Vec<&'static str>) -> Result<i32, &'static str> {
    effects.push("started");
    if input < 0 {
        effects.push("failed");
        return Err("negative");
    }
    effects.push("completed");
    Ok(input * 2)
}

#[test]
fn instrumentation_preserves_success_and_failure() {
    for input in [-3, 0, 7] {
        let mut baseline_effects = Vec::new();
        let baseline_result = baseline(input, &mut baseline_effects);

        let mut instrumented_effects = Vec::new();
        let instrumented_result = instrumented_call(
            "wf_test",
            || baseline(input, &mut instrumented_effects),
            &NoopNonFailingEmitter,
        );

        assert_eq!(instrumented_result, baseline_result);
        assert_eq!(instrumented_effects, baseline_effects);
    }
}

This test must also be repeated with an emitter that fails internally. Application behavior must remain unchanged.

12. Fuzz targets

workflow_json

Input: arbitrary bytesChecks:

Parser never panics.

Memory and recursion are bounded.

Unsupported schema versions return typed errors.

Successful parsing can be reserialized.

runtime_event

Input: arbitrary bytesChecks:

Schema decoder never panics.

Oversized events are rejected.

Invalid IDs do not enter the trace store.

Redaction executes before storage.

normalize_graph

Input: generated graph structuresChecks:

No panic.

Valid results satisfy referential integrity.

Normalization is idempotent.

Canonical serialization terminates.

redact_payload

Input: arbitrary JSON-like valueChecks:

No forbidden known secret fixture survives.

Output respects maximum depth and size.

Redaction is idempotent.

source_span

Input: arbitrary file length and positionsChecks:

Invalid ranges are rejected.

Conversion does not overflow.

UTF-8/UTF-16 calculations remain bounded.

No slice panic.

13. Mutation testing targets

Run mutation testing on:

Reachability

Reverse terminal reachability

SCC/cycle classification

Retry boundedness

Loading-state analysis

Failure-path classification

Stable ID input construction

Canonical sorting

Event deduplication

Redaction key matching

Source span validation

Acceptance:

No surviving mutation in critical rules.

Surviving non-critical mutations must be reviewed and either covered or explicitly excluded with rationale.

14. Benchmark plan

Use a fixed, documented benchmark machine and checked-in synthetic generators.

Benchmark

Dataset

Extraction small

1,000 LOC, 20 workflows

Extraction medium

10,000 LOC, 200 workflows

Extraction large

50,000 LOC, 1,000 workflows

Validation

1k, 10k, and 50k nodes

Canonical serialization

1k, 10k, and 50k nodes

Runtime ingestion

1k–100k events

Redaction

Nested and wide event attributes

Incremental update

One handler edit in large project

Initial gates:

Metric

Gate

50k LOC extraction overhead

≤20% of baseline compile/dev analysis

Single workflow incremental update

<300 ms p95

10k-node validation

<250 ms

Runtime ingestion

≥5,000 events/s

Duplicate delivery

Zero logical duplicates

Disabled runtime

Zero workflow network activity

5k-node UI initial layout

<2 s

Known secret corpus

Zero leakage

Benchmark regressions above 10% require review.

15. End-to-end projects

E2E-01 — Checkout

Must include:

Page load

Validation

Loading state

API request

Success navigation

Failure message

Retry with bound

Source navigation

Runtime success and failure traces

E2E-02 — Authentication

Must include:

Login

Invalid credentials

OTP or second step

Protected route

Logout

Redirect behavior

One intentional invalid redirect fixture

E2E-03 — Dashboard

Must include:

Parallel data loading

Partial failure

Empty state

Refresh event

Modal open/close

Background detached operation

Large graph grouping

Every end-to-end project must provide:

Approved static graph

Approved diagnostics

Successful runtime recording

Failure runtime recording

SARIF output

Source-navigation checks

16. CI pipeline

format
→ lint
→ unit tests
→ fixture/golden tests
→ property tests
→ integration tests
→ SARIF schema validation
→ security fixtures
→ benchmarks with regression threshold
→ scheduled fuzzing
→ scheduled mutation testing

Recommended cadence:

Every pull request: unit, fixture, property, integration, security.

Nightly: bounded fuzzing and benchmark comparison.

Weekly or release candidate: mutation testing and extended fuzzing.

17. Release acceptance checklist

Static extraction

Approved semantic mapping exists.

All required Korlix constructs have fixtures.

Canonical output is deterministic.

Stable IDs survive non-semantic edits.

Unknown dynamic targets remain unresolved.

Validation

Every critical rule has positive and negative controls.

Every warning rule has false-positive control fixtures.

Graph property tests pass.

Cycle classification is tested for loop, retry, event recurrence, and invalid navigation.

Runtime

Baseline/instrumented differential suite passes.

Collector failure does not alter application behavior.

Out-of-order and duplicate events are handled.

Bounded storage behavior is tested.

Runtime-disabled mode emits nothing.

Security

Secret corpus has zero leakage.

Collector binds to loopback.

Event depth/size limits are enforced.

Project paths are sanitized in exports.

Fuzz targets run without crashes.

Compatibility

Workflow schema version is present.

Round-trip tests pass.

Older supported fixtures migrate.

Newer unsupported schemas fail explicitly.

SARIF validates.

UI

Static and runtime status are visually separate.

Every source-backed node opens the correct source.

Error paths are visible by default.

Unresolved targets are never displayed as confirmed.

Runtime refresh does not unnecessarily relayout the graph.

Large projects remain navigable.

18. Definition of done

Phase 1 is test-complete when:

The complete fixture matrix passes.

All required graph properties have automated tests.

Critical validator rules have no unexplained surviving mutations.

Instrumented and baseline execution are observably equivalent.

Known secret fixtures do not appear in runtime output.

Fuzz targets run for the defined release duration without reproducible crashes.

Performance gates pass on the documented benchmark machine.

Three end-to-end projects produce approved static graphs and runtime overlays.

SARIF and Workflow IR exports validate.

Phase 2 adapters can target the schema without compiler-internal dependencies.

19. References

Proptest documentation: https://docs.rs/proptest/latest/proptest/

Rust Fuzz Book: https://rust-fuzz.github.io/book/

cargo-fuzz: https://github.com/rust-fuzz/cargo-fuzz

Insta snapshot testing: https://docs.rs/insta/

trybuild compile-fail tests: https://docs.rs/trybuild/

Criterion benchmarking: https://bheisler.github.io/criterion.rs/book/

cargo-mutants: https://docs.rs/crate/cargo-mutants/latest

Rust MIR and CFG: https://rustc-dev-guide.rust-lang.org/mir/index.html

Workflow-net soundness overview: https://link.springer.com/article/10.1007/s00165-010-0161-4

OASIS SARIF 2.1.0: https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html
