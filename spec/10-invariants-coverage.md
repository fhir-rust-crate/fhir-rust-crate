# 10 — Invariant coverage

FHIR states many rules as `ElementDefinition.constraint` **invariants** —
FHIRPath expressions such as "an extension must have a value or children, not
both". This spec records which of them the crate enforces, and enumerates the
rest so that unenforced rules are visible rather than silently dropped.

## Requirements

- **R10.1** Every invariant the crate does not enforce MUST be listed here.
  Silence is not permitted: a reader must be able to tell what validation does
  and does not cover.
- **R10.2** An invariant is enforced only if it can be checked structurally,
  without a FHIRPath evaluator. Enforcement lives in `invariant_stmts` in
  `fhir-derive-macros`, so it applies to every release at once.

## Scale

| Release | Distinct invariant keys | Constraint occurrences |
|---|---:|---:|
| R5 | 314 | 10,992 |
| R4 | 240 | 8,971 |
| R3 | 187 | 1,039 |

R3's occurrence count is an order of magnitude lower not because it states
fewer rules but because its published snapshots do not repeat the universal
constraints on every element: `ele-1` appears 383 times in R3 against 6,710 in
R4. Compare the *keys* column, not the occurrences, when reading this as
coverage.

## Enforced

| Key | R5 | R4 | R3 | Rule |
|---|---:|---:|---:|---|
| `ext-1` | 1637 | 1295 | 0 | An extension must have either extensions or `value[x]`, not both |
| `dom-2` | 122 | 145 | 116 | A contained resource SHALL NOT itself contain nested resources |
| `dom-4` | 122 | 145 | 116 | A contained resource SHALL NOT have a `meta.versionId` or `meta.lastUpdated` |

These are the structurally checkable, high-frequency invariants. `ext-1` alone
covers 1,637 R5 elements; the two `dom-*` rules apply to every domain resource's
`contained` list. All three are checked identically in every release — the zero
in the R3 column means R3's snapshots do not *restate* `ext-1` per element, not
that the rule is unenforced there.

## Not yet enforced (most frequent)

| Key | R5 | R4 | R3 | Rule |
|---|---:|---:|---:|---|
| `ele-1` | 8363 | 6710 | 383 | All FHIR elements must have a `@value` or children |
| `dom-3` | 122 | 145 | 116 | A contained resource SHALL be referred to from elsewhere in the resource |
| `dom-5` | 122 | 145 | 0 | A contained resource SHALL NOT have a security label |
| `dom-6` | 122 | 145 | 0 | A resource should have narrative for robust management |
| `dom-1` | 0 | 0 | 116 | A contained resource SHALL NOT contain narrative (dropped after R3) |
| `cnl-1` | 33 | 0 | 0 | URL should not contain a pipe or `#` — they make canonical processing hard |
| `cnl-0` | 32 | 0 | 0 | Name should be usable as a machine-processing identifier |
| `qty-3` | 7 | 7 | 7 | If a unit code is present, the system SHALL also be present |
| `inv-1` | 3 | 3 | 4 | A parameter must have one and only one of (value, resource, part) |
| `org-3` | 2 | 1 | 1 | An organization's telecom can never be of use `home` |
| `att-1` | 1 | 1 | 1 | If an Attachment has data, it SHALL have a `contentType` |
| `age-1` | 1 | 1 | 1 | An Age SHALL have a code if it has a value, expressing time |
| `cnt-3` | 1 | 1 | 1 | A Count SHALL have code `1` if it has a value |
| `drq-1` | 1 | 1 | 1 | Either a path or a searchParam must be provided, but not both |

`ele-1` alone accounts for the large majority of all constraint occurrences.
Enforcing it meaningfully needs FHIRPath, and is deferred. The remaining keys
(314 in R5, 240 in R4, 187 in R3) appear once or twice each and live in the
specification JSON; regenerate these tables from `ElementDefinition.constraint`
when coverage changes.

The releases do not state the same rules: `cnl-0` and `cnl-1` are R5-only, and
`dom-1` was dropped after R3. A column per release is shown rather than one
because a zero can mean either "not stated in this release" or "not restated per
element".

## Future work

- A FHIRPath evaluator, which would unlock `ele-1` and most of the tail.
- Enforcing the arithmetic invariants (`app-5`: start ≤ end, and similar) that
  are structurally checkable but element-specific, most likely as generated
  per-type checks driven by the `meta` table.

## Acceptance criteria

1. At least three invariant classes are enforced with tests — `ext-1`, `dom-2`,
   `dom-4`, covered by the `invariant_ext_1` and `invariant_dom_2` tests in
   every release's `validate` module.
2. Unrecognized constraints are enumerated here rather than dropped.
3. The counts above match the shipped specification JSON.
