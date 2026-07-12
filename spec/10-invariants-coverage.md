# 10 — Invariant coverage (T14)

Generated report. It lists every FHIR `ElementDefinition.constraint` invariant
`key` in the R5 specification, its occurrence count, and whether this crate
enforces it in `Validate`. Recognized invariants are checked by `invariant_stmts`
in `fhir-derive-macros`; everything else is **listed here, not silently dropped**,
as a candidate for future coverage.

- Distinct invariant keys: **314**
- Total constraint occurrences: **10992**
- Enforced keys: **3** — `dom-2`, `dom-4`, `ext-1`

## Enforced

| Key | Count | Rule |
|---|---:|---|
| `ext-1` | 1637 | Must have either extensions or value[x], not both |
| `dom-2` | 122 | A contained resource SHALL NOT itself contain nested resources |
| `dom-4` | 122 | A contained resource SHALL NOT have a meta.versionId or meta.lastUpdated |

These are the structurally-checkable, high-frequency invariants. `ext-1` alone
covers 1637 elements; the two `dom-*` rules apply to every domain resource's
`contained` list.

## Not yet enforced (top 25 by frequency)

Most remaining invariants require a FHIRPath evaluator. `ele-1` ("every element
must have a value or children") alone accounts for the large majority of all
constraint occurrences; enforcing it meaningfully needs FHIRPath and is deferred.

| Key | Count | Rule |
|---|---:|---|
| `ele-1` | 8363 | All FHIR elements must have a @value or children |
| `dom-3` | 122 | A contained resource SHALL be referred to from elsewhere in the resource |
| `dom-5` | 122 | A contained resource SHALL NOT have a security label |
| `dom-6` | 122 | A resource should have narrative for robust management |
| `cnl-1` | 33 | URL should not contain pipe or # — these make canonical processing hard |
| `cnl-0` | 32 | Name should be usable as a machine-processing identifier |
| `qty-3` | 7 | If a unit code is present, the system SHALL also be present |
| `inv-1` | 3 | A parameter must have one and only one of (value, resource, part) |
| `org-3` | 2 | An organization's telecom can never be of use 'home' |
| `org-4` | 2 | An organization's address can never be of use 'home' |
| `pld-0` | 2 | Input data elements must have a requirement or relatedData, not both |
| `pld-1` | 2 | Output data elements must have a requirement or relatedData, not both |
| `app-5` | 1 | Appointment start must be <= end |
| `app-2` | 1 | Either start and end are specified, or neither |
| `app-1` | 1 | Either the type or actor on the participant SHALL be specified |
| `bdl-1` | 1 | Bundle total only when a search or history |
| `bdl-2` | 1 | Bundle entry.search only when a search |

(The full 314-key set lives in the spec JSON; regenerate this table from
`ElementDefinition.constraint` when coverage changes.)

## Acceptance criteria (T14)

- [x] >=3 invariant classes enforced with tests (`ext-1`, `dom-2`, `dom-4` — see
      `src/r5/validate.rs` tests `invariant_ext_1`, `invariant_dom_2`).
- [x] Unrecognized constraints are enumerated here rather than dropped.
