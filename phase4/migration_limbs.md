<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Phase-4 migration plan — `limbs/` dissolution (#79 cleanup map)

Read-only analysis (2026-05-23). No code changed by this doc. Companion to
`research/2026_05_23_phase4_migration_roots.md` (same Phase-4 track).

**Goal of #79:** `src/int/limbs/mod.rs` is a 3746-line module holding **two parallel
limb families** (a legacy u128-slice family and the active u64-slice family) plus
**4 separable concerns**. Phase 4 fans it out so each concern lands in its rightful
home and the dead u128 family is deleted.

**Naming rule applied** (owner correction 2026-05-23): relocated limb kernels are
renamed `<function>_<method>` by what they DO — `mul_*`, `div_*`, `add_*`, `cmp_*`,
`shift_*`, `isqrt_*`, … — never the bare `limbs_*` form, never width/`dXX` in the
name (width stays a const-generic param; only a genuinely width-bespoke kernel may
suffix `_int2`-style).

**Dissolution destinations (the four buckets):**
1. **generic limb arithmetic** → `int/algos/limbs.rs` (add/sub/cmp/shift/mul/bit_len).
2. **division engines** (Knuth-D / Burnikel–Ziegler / 3-by-2 / 2-by-1) → `int/algos/div.rs`.
3. **isqrt** → the sqrt/root algos (`int/algos/`, the `isqrt_newton` of the roots doc).
4. **decimal formatting** → `support`/display.

`Int<N>` / `Uint<N>` stay in `int/types`. The u128 legacy family is DELETED **after**
`newton_reciprocal` is rewritten off the u128 path (dependency, see §4).

---

## 1. Inventory of `src/int/limbs/mod.rs`

### 1.A u128-slice family (legacy, lines ~44–1086) — DELETE after §4

These operate on `&[u128]` slices. The catalog (B.a) confirms the family is
**superseded by the u64 family** and kept alive only by `newton_reciprocal` + a
couple of `lib.rs` re-exports.

| Item (line) | Concern | Live? | Destination |
|---|---|---|---|
| `mul_128` (47) | helper | internal | dies with family |
| `limbs_is_zero` (58), `limbs_eq` (71), `limbs_cmp` (88), `limbs_bit_len` (107) | (i) generic arith | dead-as-u128 | DELETE |
| `limbs_add_assign` (120), `limbs_sub_assign` (136) | (i) | `limbs_sub_assign` LIVE (newton_reciprocal) | DELETE after §4 |
| `limbs_shl` (152), `limbs_shr` (178), `limbs_shl1` (404), `limbs_fit_one` (419) | (i) | dead-as-u128 | DELETE |
| `limbs_mul` (205) | (i) mul | LIVE — re-exported `lib.rs:199` + newton_reciprocal | DELETE after §4 + drop re-export |
| `limbs_mul_fast` (295/304), `limbs_mul_karatsuba` (323), `limbs_mul_karatsuba_padded` (390), `KARATSUBA_MIN` (283) | (i) mul | dead-as-u128 | DELETE |
| `limbs_divmod` (444), `SCRATCH_LIMBS` (524) | (ii) div | dead-as-u128 | DELETE |
| `limbs_divmod_dispatch` (545) | (ii) div | LIVE — re-exported `int/policy/mod.rs:71` | DELETE after §4 + drop re-export |
| `div_2_by_1` (684), `limbs_divmod_knuth` (733), `limbs_divmod_bz` (930), `MG2by1` (608) | (ii) div engines | dead-as-u128 | DELETE |
| `limbs_isqrt` (1006) | (iii) isqrt | dead-as-u128 | DELETE |
| `limbs_div_small` (1041) | (iv) fmt helper | dead-as-u128 | DELETE |
| `limbs_fmt_into` (1060) | (iv) fmt | dead-as-u128 | DELETE |

### 1.B u64-slice family (active, lines ~1108–2693) — RELOCATE to buckets

The `*_u64` / `*_u64_fixed<L>` family. Callers: `int/algos/{cmp,add_sub,shift,mul}.rs`,
display, and the root/div fast-arms (`int/algos/div.rs`).

| Item (line) | Concern | Live? | Destination + proposed name |
|---|---|---|---|
| `limbs_is_zero_u64` (1108) / `_fixed` (1122) | (i) | LIVE | `int/algos/limbs.rs` → `is_zero` / `is_zero_fixed` |
| `limbs_eq_u64` (1135) | (i) | re-export / verify | (i) → `eq` (drop if truly unused) |
| `limbs_cmp_u64` (1151) / `_fixed` (1171) / `_cross` (1192) | (i) cmp | LIVE (`int/algos/cmp.rs`) | (i) → `cmp` / `cmp_fixed` / `cmp_cross` |
| `limbs_bit_len_u64` (1217) / `_fixed` (1231) | (i) | LIVE | (i) → `bit_len` / `bit_len_fixed` |
| `limbs_add_assign_u64` (1244) / `_fixed` (1261) | (i) | LIVE (`add_sub.rs`) | (i) → `add_assign` / `add_assign_fixed` |
| `limbs_sub_assign_u64` (1279) / `_fixed` (1295) | (i) | LIVE | (i) → `sub_assign` / `sub_assign_fixed` |
| `limbs_shl_u64` (1376) / `_fixed` (1315), `limbs_shr_u64` (1402) / `_fixed` (1346), `limbs_shl1_u64` (1429), `limbs_fit_one_u64` (1443) | (i) shift | LIVE (`shift.rs`); `limbs_shl_u64` verify | (i) → `shl` / `shr` / `shl1` / `fit_one` (+ `_fixed`) |
| `limbs_mul_u64` (1460) / `_fixed<L,D>` (1502) / `_into<L,LP1>` (1554) | (i) mul (schoolbook) | LIVE (`mul.rs`, re-exported `lib.rs`) | (i) → `mul_schoolbook` (+ `_fixed`/`_into` variants) |
| `limbs_mul_karatsuba_u64` (1726) / `_forced` (1747) / `_with_threshold` (1772) / `_alloc` (1926) / `_padded_u64_alloc` (1976), `karatsuba_rec` (1816) / `_unbalanced` (1886), `karatsuba_scratch_needed*` (1790/1797), `KARATSUBA_THRESHOLD_U64` (1693), scratch consts (1657/1706) | (i) mul (Karatsuba) | LIVE | (i) → `mul_karatsuba` (+ variants); thresholds become policy data |
| `limbs_mul_fast_u64` (1908) | (i) mul dispatch | LIVE | **(ii)/policy** → the schoolbook-vs-karatsuba choice is a *policy* decision; the picker moves to `int/policy/mul.rs`, the kernels (`mul_schoolbook`/`mul_karatsuba`) stay in (i) |
| `limbs_divmod_u64` (1581) | (ii) div (single/2-limb fast) | LIVE | `int/algos/div.rs` → `div_rem` fast arms (already partly there via `div_rem_mag_fixed`) |
| `limbs_divmod_dispatch_u64` (2194) | (ii) div dispatch | LIVE (re-exported `int/policy/mod.rs:71`) | **(ii)/policy** → divisor-shape dispatch moves to `int/policy/div.rs`; engines stay in (ii) |
| `div_2_by_1`-equiv via `MG2by1U64` (1996), `MG3by2U64` (2060) | (ii) div engine (Möller–Granlund 2-by-1 / 3-by-2 reciprocal) | LIVE; `Mg3by2` flagged verify | (ii) `int/algos/div.rs` → `div_mg` (the MG reciprocal engine) |
| `limbs_divmod_knuth_u64` (2233) | (ii) Knuth Algorithm D | LIVE | (ii) → `div_knuth` |
| `limbs_divmod_bz_u64` (2410) | (ii) Burnikel–Ziegler | LIVE | (ii) → `div_burnikel_ziegler_with_knuth` (BZ recurses to Knuth as base case → `_with_` hybrid) |
| `limbs_isqrt_u64` (2477) | (iii) isqrt | LIVE | **(iii)** → the roots `isqrt_newton` (see roots doc §3) |
| `limbs_div_small_u64` (2602) | (iv) fmt helper (divide by radix) | LIVE | (iv) → `support`/display → `div_small_radix` |
| `limbs_fmt_into_u64` (2627), `POW10_19` (2615) / `_DIGITS` (2617) | (iv) decimal/radix formatting | LIVE (display) | (iv) → `support`/display → `fmt_into` |
| `scmp_u64` (2686) | (i) signed cmp | LIVE; flagged verify | (i) → `scmp` |

### 1.C tail items (lines ~2698+)

| Item | Concern | Destination |
|---|---|---|
| `scmp` (2701, u128) | (i) signed cmp | DELETE with u128 family |
| named `Int*`/`Uint*` `pub type` aliases + re-exports | type aliases | stay (or move with `int/types`); not a limb concern |

---

## 2. Per-bucket destination summary

### (i) generic limb arithmetic → `int/algos/limbs.rs`
All u64 `is_zero/eq/cmp/bit_len/add_assign/sub_assign/shl/shr/mul_schoolbook/
mul_karatsuba/scmp` (+ `_fixed` siblings). Renamed to bare `<op>` (the module path
`int::algos::limbs::cmp` already says "limbs"; no `limbs_` prefix, no `_u64` suffix
once the u128 twin is gone). Width stays the const-generic `L` param on `_fixed`.

### (ii) division engines → `int/algos/div.rs`
`div_knuth` (Knuth D), `div_burnikel_ziegler_with_knuth` (BZ → Knuth base case),
`div_mg` (Möller–Granlund 2-by-1/3-by-2 reciprocal, from `MG2by1U64`/`MG3by2U64`),
plus the existing `div_rem_mag_fixed`/`isqrt_mag_fixed` already in this file. The
**divisor-shape dispatcher** (`limbs_divmod_dispatch_u64`) and the **mul algorithm
picker** (`limbs_mul_fast_u64`, threshold consts) move to the **policy layer**
(`int/policy/{div,mul}.rs`) per the TODO already noted in `int/policy/mod.rs:62-65`
("move KARATSUBA_MIN, the BZ_THRESHOLD divide selection… out of limbs into this
module, leaving the kernels to take an already-chosen algorithm"). This is exactly
the matcher split: policy picks `div_knuth | div_burnikel_ziegler_with_knuth | div_mg`,
the engines stay pure.

### (iii) isqrt → root algos
`limbs_isqrt_u64` becomes the body of `isqrt_newton` (roots doc §3). Lives under
`int/algos/` with the int `isqrt` policy. The u128 `limbs_isqrt` is deleted.

### (iv) decimal formatting → `support`/display
`limbs_fmt_into_u64`, `limbs_div_small_u64`, `POW10_19`/`_DIGITS` move to the display
support module (they are radix-to-string machinery, not arithmetic). Renamed
`fmt_into` / `div_small_radix`.

---

## 3. u128 legacy family — DELETE list (after §4)

Everything in §1.A. Of these, three are LIVE today **only** through
`newton_reciprocal` + re-exports and must wait for §4:
- `limbs_mul` (re-exported `lib.rs:199`) — drop the `lib.rs` re-export too.
- `limbs_divmod_dispatch` (re-exported `int/policy/mod.rs:71`) — drop that re-export.
- `limbs_sub_assign` — only `newton_reciprocal` calls it.

The rest of §1.A (`mul_128`, `limbs_{is_zero,eq,cmp,bit_len,add_assign,shl,shr,shl1,
fit_one,mul_fast,mul_karatsuba,mul_karatsuba_padded,divmod,divmod_knuth,divmod_bz,
isqrt,div_small,fmt_into}`, `MG2by1`, `div_2_by_1`, `scmp`, `SCRATCH_LIMBS`,
`KARATSUBA_MIN`) is dead-as-u128 and can be deleted immediately (modulo confirming
no other re-export — the catalog already swept this).

---

## 4. Cross-layer reach — `newton_reciprocal` → u128 `limbs_*` (the blocker)

`src/algos/newton_reciprocal.rs:51` does:

```rust
use crate::wide_int::{limbs_divmod_dispatch, limbs_mul, limbs_sub_assign};
```

i.e. a **decimal-layer algo reaching into the u128 limb family** (`crate::wide_int`
is the legacy re-export path for `int::limbs`). Calls at lines 114
(`limbs_divmod_dispatch`), 155 & 170 (`limbs_mul`), 179 & 192 (`limbs_sub_assign`).

`newton_reciprocal` is **LIVE** — it is the Phase-1 wide-decimal `Mul`/rescale divide
algorithm (`dispatch_wide_pow10_with`, called from `macros/arithmetic.rs:301,319`
and `macros/wide_transcendental.rs:728,911`). So the u128 family cannot be deleted
while it stands.

**Dependency / sequencing for #79:**
1. **Rewrite `newton_reciprocal` onto the u64 limb family** (`limbs_mul_u64` /
   `limbs_divmod_dispatch_u64` / `limbs_sub_assign_u64`, i.e. the relocated
   `int/algos/limbs.rs` + `int/algos/div.rs` kernels). The catalog (B.a) suggests it
   "likely gives the reciprocal its own limb backend" — either way it must stop
   importing `crate::wide_int::limbs_*` (u128).
2. **Then** delete the entire u128 family (§3) and the two re-exports
   (`lib.rs:199`, `int/policy/mod.rs:71`).
3. Re-validate: golden gate + the round-div-chain audit tests that exercise
   `newton_reciprocal` (`mg_divide.rs` `round_div_chain_audit_*`).

**Layering check (naming-standard §"Layering rule"):** post-rewrite,
`newton_reciprocal` (a decimal-layer algo) should call **int-layer** kernels through
the proper surface (`BigInt` ops or `int::algos`), not reach sideways into raw limb
slices. The rewrite is also the moment to fix that smell — route through
`int/algos/div.rs`/`limbs.rs` rather than re-importing raw `limbs_*`.

---

## 5. Headline

`limbs/mod.rs` (3746 lines, two parallel families) dissolves into four homes:
**(i)** generic u64 limb arithmetic → `int/algos/limbs.rs` (renamed bare `mul_*`/
`add_*`/`cmp_*`/`shift_*`); **(ii)** division engines (`div_knuth`,
`div_burnikel_ziegler_with_knuth`, `div_mg`) → `int/algos/div.rs` with the
divisor-shape + mul-algorithm pickers lifted to `int/policy/{div,mul}.rs`;
**(iii)** isqrt → the roots `isqrt_newton`; **(iv)** decimal/radix formatting →
`support`/display. The entire **u128 legacy family is DELETED** — but **only after**
`newton_reciprocal` is rewritten off `crate::wide_int::{limbs_mul,
limbs_divmod_dispatch, limbs_sub_assign}` onto the u64 surface (the one hard
sequencing dependency, plus dropping the `lib.rs` + `int/policy/mod.rs` re-exports).
`Int<N>`/`Uint<N>` stay in `int/types`.
