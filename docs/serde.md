# Serde

Serde support sits behind the `serde` Cargo feature (enabled by
default; dropped under `default-features = false` — see
[Cargo features](features.md)). Every width implements
`Serialize` / `Deserialize`, choosing its encoding from the
serializer's `is_human_readable` flag.

## Wire format

For human-readable formats (JSON, TOML, YAML) the wire format is the
**raw scaled-integer storage** rendered as a base-10 string — not the
displayed decimal value. `D38s12::ONE` serialises as
`"1000000000000"` (the storage `1 × 10^12`), and a native integer in
the payload is likewise read as storage: the JSON number `1`
deserialises to `0.000000000001` at scale 12, **not** to `1.0`.

> **Cross-system warning:** the wire value is meaningless without the
> SCALE. The scale is a compile-time type parameter and is *not*
> encoded in the payload, so any other system reading or writing this
> data must agree on the scale out-of-band. Treating the wire value as
> a plain number yields a result `10^SCALE` times too large.

The integer-string form is deliberate: it is the only lossless,
JavaScript-`BigInt`-compatible encoding for storage values wider than
the 53-bit safe-integer range of a JSON number.

## Binary formats

Binary formats (postcard, bincode) carry the same storage as raw
little-endian bytes — no string round-trip, same exact value.

## Round-tripping

Because the wire value is the storage integer, serialise → deserialise
at the same width and scale is always bit-exact: no parsing, no
rounding, no normalisation. To exchange values with a system that
expects the *displayed* decimal instead, format with `Display` and
parse with `FromStr` explicitly — that path is also exact at the
type's scale.
