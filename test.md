Korlix Workflow Inspector — Complete End-to-End Test Case

Test ID: E2E-WF-001Title: Checkout payment workflowPhase: Phase 1 — Korlix-nativePriority: CriticalType: Static extraction, validation, runtime, security, and UI acceptanceStatus: Implementation baselineUpdated: 2026-08-02

1. Purpose

Verify that Korlix can correctly:

Discover a checkout workflow.

Lower it into Workflow IR.

Preserve all success, failure, validation, cleanup, state, and navigation paths.

Produce stable semantic node IDs.

Validate the workflow without false positives.

Instrument the generated application without changing behavior.

Record runtime execution with valid parent-child causality.

Redact payment and authorization secrets.

Display static and runtime states correctly.

Export deterministic JSON and valid SARIF.

This is an end-to-end acceptance test, not only a unit test.

2. Business workflow

The user opens Checkout and selects Pay.

Required behavior:

Clear the previous error.

Validate the form.

Stop without calling the API when validation fails.

Set loading = true before submission.

Call POST /api/orders.

On success:

save the returned order ID;

navigate to /orders/{orderId}/success.

On failure:

display a user-facing payment error.

Always reset loading = false after the request.

Block duplicate submission while loading.

3. Preconditions

Korlix parser and semantic resolver are available.

Workflow lowering executes after semantic resolution.

Source spans include byte offsets and editor-compatible positions.

Development instrumentation can be enabled.

The local runtime collector is available on loopback.

A configurable mock order API is available.

The inspector can load Workflow IR and runtime events.

4. Test fixture

The syntax below is illustrative until mapped to the actual Korlix grammar.

File

tests/fixtures/e2e_checkout/input.klx

Source

page Checkout at "/checkout":
    state loading: bool = false
    state error: text? = null
    state order_id: text? = null

    form PaymentForm:
        field card_token: secret_text
        field product_id: text
        field quantity: number

        button "Pay":
            disabled when loading

            on click:
                error = null

                if not PaymentForm.is_valid:
                    error = "Please correct the payment form"
                    return

                loading = true

                try:
                    response = post "/api/orders" with:
                        authorization = session.authorization
                        card_token = PaymentForm.card_token
                        product_id = PaymentForm.product_id
                        quantity = PaymentForm.quantity

                    order_id = response.order_id
                    navigate "/orders/{order_id}/success"

                catch reason:
                    error = "Payment could not be completed"

                finally:
                    loading = false

        when error is not null:
            alert error

5. Semantic-resolution expectations

Source element

Required resolved meaning

Checkout

Page and workflow owner

PaymentForm

Form component owned by Checkout

Pay

User-event source

loading

Page-local Boolean state

error

Page-local nullable text state

order_id

Page-local nullable text state

PaymentForm.is_valid

Validation expression

POST /api/orders

Failure-capable HTTP effect with literal route

session.authorization

Sensitive runtime value

PaymentForm.card_token

Sensitive runtime value

response.order_id

API result field

navigate

Navigation effect

alert error

Visible consumption of error state

The extractor must not infer values that the resolver cannot establish.

6. Expected nodes

Logical node

Workflow kind

Workflow start

workflow_start

Checkout page

page

Pay clicked

user_event

Clear previous error

state_write

Validate form

validation

Validation result

condition

Set validation error

state_write

Validation-failure terminal

workflow_end

Set loading true

state_write

Create order

api_request

Store order ID

state_write

Navigate success

navigation

Payment failure handler

error_handler

Set payment error

state_write

Set loading false

state_write

Render error alert

state_read or visible-effect node

Request-flow completion

workflow_end

Each source-backed node must contain the correct .klx source span.

7. Expected graph

Start
  ↓
Checkout
  ↓
Pay clicked
  ↓
error = null
  ↓
Validate form
  ↓
Form valid?
  ├── false
  │     ↓
  │   validation error
  │     ↓
  │   validation terminal
  │
  └── true
        ↓
      loading = true
        ↓
      POST /api/orders
        ├── success
        │     ↓
        │   order_id = response.order_id
        │     ↓
        │   navigate success
        │
        └── failure
              ↓
            catch
              ↓
            payment error
                \       /
                 finally
                    ↓
             loading = false
                    ↓
                 complete

Required graph semantics:

Validation failure never reaches the API node.

Both API outcomes pass through cleanup.

loading = false post-dominates API success and failure.

The validation terminal is valid because loading was never enabled.

Error state has a visible consumer.

The Pay button depends on loading for disabled state.

8. Expected edges

From

To

Edge kind

Start

Checkout

control

Checkout

Pay clicked

user_event

Pay clicked

Clear error

control

Clear error

Validate

control

Validate

Validation condition

control

Validation condition

Validation error

condition_false

Validation condition

Loading true

condition_true

Validation error

Validation terminal

control

Loading true

API request

control

API request

Store order ID

success

API request

Failure handler

failure or exception

Store order ID

Navigate success

control

Navigate success

Loading false

cleanup control

Failure handler

Set payment error

control

Set payment error

Loading false

cleanup control

Loading false

Completion

control

Loading state

Pay disabled

state_dependency

Error state

Alert

state_dependency

9. Static properties

The valid fixture must pass:

WF-P01 Referential integrity

WF-P02 Unique identity

WF-P03 Deterministic extraction

WF-P04 Stable semantic IDs

WF-P05 Valid source spans

WF-P06 Single normalized entry

WF-P07 Reachability

WF-P08 Completion availability

WF-P09 Branch completeness

WF-P10 Failure completeness

WF-P11 Async closure

WF-P14 State explainability

WF-P15 Terminal cleanliness

WF-P20 Schema round trip

WF-P22 Incremental locality

WF-P23 Canonical order independence

WF-P24 Normalization idempotence

WF-P25 Diagnostic stability

Expected diagnostics:

{
  "errors": [],
  "warnings": []
}

The valid fixture must not trigger:

WF103_API_WITHOUT_ERROR_PATH

WF104_LOADING_NOT_CLEARED

WF106_ERROR_NOT_RENDERED

WF107_DUPLICATE_SUBMISSION

WF108_ASYNC_NOT_JOINED

WF110_DYNAMIC_TARGET_UNRESOLVED

WF112_EMPTY_SUCCESS_PATH

10. Static execution procedure

10.1 Export graph

korlix workflow export \
  tests/fixtures/e2e_checkout/input.klx \
  --format json \
  --output actual.graph.json

Assertions:

Exactly one checkout-payment workflow exists.

Every required node exists.

Every required edge exists.

API success and failure are distinct.

Cleanup is reachable from both API outcomes.

Source ranges point to the correct statements.

Shareable output contains no absolute paths.

10.2 Run validation

korlix workflow check \
  tests/fixtures/e2e_checkout/input.klx \
  --format json \
  --output actual.diagnostics.json

Assertions:

errors = 0
warnings = 0

10.3 Verify determinism

Run extraction twice:

bytes(run_1) = bytes(run_2)

Canonical output must not contain random UUIDs, timestamps, or unstable collection order.

11. Runtime scenario A — validation failure

Input

{
  "card_token": "",
  "product_id": "product-101",
  "quantity": 1
}

Expected path

Pay clicked
→ Clear error
→ Validate form
→ Condition false
→ Set validation error
→ Validation terminal

Expected state

{
  "loading": false,
  "error": "Please correct the payment form",
  "order_id": null
}

Assertions

API call count is 0.

API node is unexecuted, not failed.

Validation error is visible.

No sensitive value appears in telemetry.

12. Runtime scenario B — API success

Input

{
  "card_token": "tok_test_visa_001",
  "product_id": "product-101",
  "quantity": 2
}

Mock response

{
  "status": 201,
  "body": {
    "order_id": "order-9001"
  }
}

Expected path

Pay clicked
→ Clear error
→ Validate form
→ Condition true
→ loading=true
→ POST /api/orders completed
→ order_id=order-9001
→ navigate /orders/order-9001/success
→ loading=false
→ complete

Expected observable state

{
  "loading": false,
  "error": null,
  "order_id": "order-9001",
  "navigation": "/orders/order-9001/success",
  "api_call_count": 1
}

Allowed runtime metadata:

{
  "http.method": "POST",
  "http.route": "/api/orders",
  "http.status_code": 201,
  "request.keys": [
    "authorization",
    "card_token",
    "product_id",
    "quantity"
  ]
}

Forbidden runtime values:

tok_test_visa_001

authorization value

cookie/session value

complete request body

complete response body

13. Runtime scenario C — API failure

Input

{
  "card_token": "tok_test_declined_001",
  "product_id": "product-101",
  "quantity": 1
}

Mock response

{
  "status": 402,
  "body": {
    "error": "card_declined"
  }
}

Expected path

Pay clicked
→ Clear error
→ Validate form
→ Condition true
→ loading=true
→ POST /api/orders failed
→ Failure handler
→ Set payment error
→ loading=false
→ complete

Expected state

{
  "loading": false,
  "error": "Payment could not be completed",
  "order_id": null,
  "navigation": null,
  "api_call_count": 1
}

Assertions:

API node is failed.

Failure handler is completed.

Navigation is unexecuted.

Cleanup is completed.

Token and raw response body are absent.

14. Runtime scenario D — duplicate submission

Procedure:

Submit valid input.

Hold the mock API response for 500 ms.

Trigger Pay again after 50 ms.

Expected:

API call count = 1
API runtime span count = 1
order side-effect count = 1

The second click must be ignored or rejected while loading.

15. Runtime scenario E — collector failure

Procedure:

Stop the local workflow collector.

Execute the success scenario.

Force event transport to fail immediately.

Expected:

Checkout still succeeds.

Navigation still occurs.

Instrumentation failure does not reach application code.

No retry storm occurs.

Required equality:

observable(uninstrumented)
=
observable(instrumented_with_collector)
=
observable(instrumented_with_collector_failure)

Observable behavior includes final state, API calls, side-effect order, return values, errors, and navigation.

16. Runtime event requirements

All nodes in one execution share a trace ID.

Every non-root span has a valid parent.

Duration uses monotonic time.

Duplicate event delivery is idempotent.

Out-of-order events reconcile to the same final trace.

End-without-start is rejected or marked malformed.

Start-without-end becomes incomplete after timeout.

Unknown static node IDs are marked orphaned.

Orphaned runtime events do not modify the static graph.

17. Redaction test

Sensitive fixtures:

Bearer super-secret-access-token
tok_test_declined_001
session=private-session-value
private-api-key
sample-password

None may appear in:

runtime event JSON;

collector storage;

inspector messages;

inspector UI;

exported trace JSON;

SARIF;

runtime-emitter logs.

Permitted:

sensitive field names;

request key names;

route template;

HTTP method;

status code;

duration;

error category.

Required property:

redact(redact(payload)) = redact(payload)

18. Source-navigation test

Verify source navigation for:

Pay clicked

Validation condition

Loading true

API request

Success navigation

Failure handler

Loading false

Error alert

Repeat after changing LF to CRLF.

Expected:

semantic IDs remain unchanged;

source offsets update correctly;

editor opens the correct statement.

19. Stable-ID test

Create variants:

comments_added.klx
whitespace_changed.klx
unrelated_declaration.klx
api_route_changed.klx

Expected:

IDs(baseline)
= IDs(comments_added)
= IDs(whitespace_changed)

For unrelated edits:

checkout_IDs(baseline)
= checkout_IDs(unrelated_declaration)

Changing /api/orders to /api/order-drafts must change the API node semantic content identity according to the approved ID policy, while unrelated node IDs remain stable.

20. Normalization and serialization

Normalization law

normalize(normalize(G)) = normalize(G)

No duplicate synthetic nodes may appear after the second pass.

Serialization law

canonical(original)
=
canonical(deserialize(serialize(original)))

The semantic hash must remain unchanged.

21. Negative mutations

Mutation

Source change

Required diagnostic

MUT-001

Remove catch or error propagation

WF103_API_WITHOUT_ERROR_PATH

MUT-002

Reset loading only on success

WF104_LOADING_NOT_CLEARED

MUT-003

Remove error alert

WF106_ERROR_NOT_RENDERED

MUT-004

Remove disabled state

WF107_DUPLICATE_SUBMISSION

MUT-005

Replace literal API route with dynamic lookup

WF110_DYNAMIC_TARGET_UNRESOLVED

MUT-006

Remove semantic false continuation

WF102_MISSING_FALSE_BRANCH, when no implicit continuation exists

MUT-007

Inject reversed source span

WF007_INVALID_SOURCE_SPAN

MUT-008

Instrument missing node

WF008_INSTRUMENTATION_MISMATCH

Each negative test requires a positive control proving that the valid fixture does not trigger the rule.

22. SARIF acceptance

For MUT-002, output must include:

{
  "ruleId": "WF104_LOADING_NOT_CLEARED",
  "level": "warning",
  "message": {
    "text": "A terminal failure path can be reached while loading remains true."
  },
  "properties": {
    "workflowId": "checkout.payment",
    "state": "loading"
  }
}

Requirements:

valid SARIF 2.1.0;

project-relative source location;

stable rule ID;

no sensitive runtime values.

23. Inspector acceptance

Static view

Validation, success, and failure branches are visible.

Cleanup visibly joins request outcomes.

Unexecuted paths are not marked failed.

Unresolved behavior is not marked confirmed.

Runtime success

Executed nodes are completed.

API duration is visible.

Failure branch remains visible but unexecuted.

Runtime failure

API node is failed.

Failure handler and cleanup are completed.

Navigation remains unexecuted.

API node detail

Kind: API request
Method: POST
Route: /api/orders
Confidence: Confirmed semantic
Outcomes: Success, failure
Source: input.klx:<line>:<column>
Runtime status: Trace-specific
Duration: Trace-specific

24. Automation test shape

The actual compiler APIs are not yet available. This is the required logical test structure, not compile-ready code.

#[test]
fn checkout_workflow_is_complete() {
    let fixture = load_fixture("e2e_checkout");
    let program = compile_and_resolve(&fixture.source).unwrap();

    let graph = normalize(extract_workflows(&program).unwrap()).unwrap();
    let diagnostics = validate(&graph);

    assert_eq!(diagnostics.error_count(), 0);
    assert_eq!(diagnostics.warning_count(), 0);

    assert_workflow_exists(&graph, "checkout.payment");
    assert_api_success_and_failure_edges(&graph, "/api/orders");
    assert_loading_is_false_on_request_terminals(&graph);
    assert_error_state_has_visible_consumer(&graph);
    assert_eq!(canonical_json(&graph), fixture.expected_graph);
}

Required companion tests:

validation-failure runtime;

API-success differential;

API-failure differential;

duplicate submission;

collector unavailable;

stable IDs;

redaction;

negative mutations;

SARIF validation.

25. Required fixture files

tests/fixtures/e2e_checkout/
├── input.klx
├── variants/
│   ├── comments_added.klx
│   ├── whitespace_changed.klx
│   ├── unrelated_declaration.klx
│   ├── api_route_changed.klx
│   ├── missing_error_path.klx
│   ├── missing_cleanup.klx
│   ├── error_not_rendered.klx
│   ├── duplicate_submission.klx
│   └── unresolved_route.klx
├── expected.graph.json
├── expected.diagnostics.json
├── expected.instrumentation.json
├── expected.success.trace.json
├── expected.failure.trace.json
├── expected.validation_failure.trace.json
├── expected.sarif.json
└── fixture.toml

26. Exit criteria

Static

One checkout-payment workflow is extracted.

Required nodes and edges exist.

Validation and API branches are complete.

Cleanup covers both request outcomes.

Valid fixture has zero diagnostics.

Canonical output is deterministic.

Source spans are correct.

Stable IDs survive non-semantic edits.

Runtime

Invalid form causes zero API calls.

Success navigates correctly.

Failure displays the error.

Loading resets on every request terminal.

Duplicate click creates one request.

Collector failure does not change behavior.

Trace causality is valid.

Duplicate events are deduplicated.

Security

Card token is absent.

Authorization value is absent.

Session/cookie value is absent.

Exported paths are sanitized.

Redaction is idempotent.

Export and UI

Workflow IR round trip succeeds.

SARIF validates.

Static and runtime states remain distinct.

Source navigation works.

Failure paths remain visible.

Unresolved behavior is never shown as confirmed.

27. Failure-report format

Test: E2E-WF-001
Scenario: API failure
Stage: Runtime validation
Expected: loading=false at terminal
Actual: loading=true
Rule: WF104_LOADING_NOT_CLEARED
Workflow: checkout.payment
Node: <stable-node-id>
Source: tests/fixtures/e2e_checkout/input.klx:<line>:<column>
Trace: <trace-id>
Command: <reproduction-command>

Never include raw card tokens, authorization values, cookies, or request bodies.

28. References

Rust MIR: https://rustc-dev-guide.rust-lang.org/mir/index.html

OpenTelemetry traces: https://opentelemetry.io/docs/concepts/signals/traces/

W3C Trace Context: https://www.w3.org/TR/trace-context/

OASIS SARIF 2.1.0: https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html

ECMA-426 Source Maps: https://ecma-international.org/publications-and-standards/standards/ecma-426/

Proptest: https://docs.rs/proptest/latest/proptest/

Rust Fuzz Book: https://rust-fuzz.github.io/book/
