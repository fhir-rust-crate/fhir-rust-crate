# Releasing

Releasing a new version needs a few steps that we want to automate, yet are currently manual:

## Update the version in file `Cargo.toml`

Edit the file `Cargo.toml`.

Update the version number.

## Update the version and date in the files `lib.rs` and `README.md`

Edit the file `lib.rs` and `README.md`.

Update the version number.

Update the date stamp, which uses ISO UTC format.

## Update the version in all source files

Search for:

```text
fhir/1.0.0/
```

Replace with:

```text
fhir/1.1.0/
```

## Verify

Verify everything is correct locally:

```sh
cargo fmt
cargo test
cargo deny check
cargo semver-checks
cargo build --release
cargo doc
```

## Generate docs as JSON & markdown & llms.txt

Run:

```sh
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
rustdoc-md --path target/doc/fhir.json --output fhir.md
cp fhir.md llms.txt
```

The code above generates the crate's documentation:

1. Generate one JSON file `target/doc/fhir.json`.

2. Convert from the JSON file into a Markdown file `target/doc/fhir.md`.

3. Copy the Markdown file to the standard file name `llms.txt`; do a copy instead of a symlink because symlinks don't work well on some operating systems.

Note that this runs at the **default** feature set, so the per-item listing
covers R5 only; the crate-level summary at the top still describes every
release. `llms.txt` ships inside the published crate (see `include` in
`Cargo.toml`), and adding `--features "r3 r4 xml client"` here triples it from
about 8 MB to about 21 MB — so widen it only deliberately.

## Release

Releasing a new version uses these steps:

```sh
git commit "Your message here"
git tag "1.1.1"
git push --tags
cargo publish
```

Confirm that the crate is published:

- <https://crates.io/crates/fhir>
