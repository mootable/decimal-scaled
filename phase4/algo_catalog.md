<!--
SPDX-FileCopyrightText: 2026 John Moxley
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Phase-4 algo catalog + 3.1 confirmed-dead list (from the 3.1 analysis agent, 2026-05-23)

Captured from the read-only 3.1 analysis (dead-code/stale + Phase-4 relocation/consolidation inventory).
Two uses: (A) 3.1 has a small confirmed-dead actionable set; (B) Phase 4 (4.1 consolidation/rename-off-tier, #79 limbs split) has its candidate map.

## A. CONFIRMED-DEAD (resolved cluster — reviewed removal, not a sweep; algo-bearing → golden-gate)
Dispatch model: `*_strict` methods on each `DXX` are emitted by `transcendental_trait.rs`/`strict_transcendentals.rs` → delegate to `policy::<fn>::<Policy>` → policy dispatches by `(width-tag, scale)` to a concrete kernel. A module free fn is live ONLY if a policy arm or another live kernel names it.

- `algos/exp/borrow_d57.rs` `exp_strict`/`exp2_strict` — DEAD (policy D38 uses `exp::fixed_d38::*`).
- `algos/ln/borrow_d57.rs` `ln/log2/log10/log_strict` — DEAD.
- `algos/pow/borrow_d57.rs` `powf_strict`/`powf_with` — DEAD.
- `algos/trig/borrow_d57.rs` `sin/cos/tan_strict` — DEAD (but the `atan/asin/acos/atan2_strict` SIBLINGS are LIVE — keep).
- `algos/exp/lookup_d57_s18_22.rs` + `algos/ln/lookup_d57_s18_22.rs` — DEAD (policy uses the `_tang` siblings).
- `algos/exp/lookup_d307_s140_160_tang.rs::exp_strict` — DEAD (policy D307 uses `wide_kernel::exp_strict_d307`); BUT `tang_exp_fixed`+`GUARD_FOR_HYPER` in that file are LIVE (consumed by `trig/lookup_d307_s140_160_hyper`).
- `algos/exp/lookup_d616_s300_315_tang.rs` — `exp_strict`/`compute_table`/`M`/`GUARD_NARROW` DEAD; `tang_exp_fixed`/`GUARD_FOR_HYPER` only transitively dead (their consumer, the d616 hyper module, is itself dead).
- `algos/trig/lookup_d57_s18_22_sincos_tang.rs` — entire module DEAD (policy uses the non-`_tang` sincos).
- `algos/trig/lookup_d616_s300_315_hyper.rs` — entire module DEAD (hyperbolics route through the macro core).
- `algos/fixed_d38.rs` `from_decimal_split`(402), bare `round_to_i128`(721) — DEAD (live variant is `round_to_i128_with`).
- Non-algo confidently-dead (PARALLEL-SAFE): `support/display.rs` `parse_decimal`/`parse_decimal_bits`, `support/rounding.rs` `apply_rounding`, `policy/triplet.rs` const `D18`, `build.rs` `from_u64`/`fixed_mul`/`fixed_div`.
- NOT dead (cfg/gen false positives — LEAVE): `policy/*` `*_no_std` (~110, `no_std_fn` arm), `consts/wide.rs` `D*_SCALE_REF`, generated `wide_consts.rs`. `policy/table_cache.rs::table_entry` borderline (verify).

## B. PHASE-4 candidates (over-inclusive by design — INPUT, not a delete list)

### (a) #79 limbs split — `src/int/limbs/mod.rs` is two parallel families + 4 separable concerns
- **u128-limb (legacy, mostly superseded)** but KEPT ALIVE by `newton_reciprocal` (Phase-1 algo) + a public re-export: `limbs_mul`(205, re-exported lib.rs:199), `limbs_divmod_dispatch`(545, re-exported int/policy/mod.rs:71), `limbs_sub_assign`(136) are LIVE; the rest (`limbs_mul_fast`/`_karatsuba`, `limbs_isqrt`, `limbs_div_small`, `limbs_fmt_into`, `scmp`, `KARATSUBA_MIN`, …) flagged dead-as-u128. Strong removal/relocation candidate ONCE newton_reciprocal moves off the u128 path.
- **u64-limb (active):** `*_u64`/`*_fixed<L>` family — `limbs_{is_zero,eq,cmp,bit_len,add_assign,sub_assign,shl,shr,mul,divmod,isqrt,div_small,fmt_into}_u64` + Karatsuba u64 + Reciprocal/Mg3by2 + `limbs_divmod_{dispatch,knuth,bz}_u64`. Callers: `int/algos/{cmp,add_sub,shift,mul}.rs`, display. (`limbs_eq_u64`/`limbs_shl_u64`/`scmp_u64`/`Mg3by2` flagged dead = re-export/NEEDS-VERIFICATION.)
- **4 separable concerns** #79 would fan out: (i) generic limb arithmetic, (ii) division engines (Knuth/BZ/3-by-2), (iii) isqrt, (iv) decimal formatting.
- Cross-layer smell: `algos/newton_reciprocal.rs` reaches into `crate::wide_int::limbs_*` (u128) directly — #79 likely gives the reciprocal its own limb backend.

### (b) 4.1 per-tier DUPLICATED kernels — consolidation/rename-off-tier
- **`borrow_d57` family** (`{exp,ln,pow,trig}/borrow_d57.rs`): D38→D57 widen→kernel→narrow wrappers; exp/ln/pow/sin/cos/tan now DEAD, only trig inverses survive → fold survivors into the generic widen path.
- **`lookup_dXXX_sYYY[_tang]` family (~40 files, exp/ln/trig):** per-(tier,scale-band) bespoke Tang/sincos/atan/hyper kernels (`lookup_d57_s18_22*`, `_d115_s57`, `_d153_s70_82`, `_d307_s140_160`, `_d462_s225_235`, `_d616_s300_315`, `_d924_*`, `_d1232_*`). Many have a `_tang`/non-`_tang` pair where only one is wired. Rationalise naming (drop the `dXXX_sYYY` tier encoding) + de-dup the shared `compute_table`/`M`/guard machinery.
- **`wide_kernel` per-tier free fns** (`exp/wide_kernel.rs` etc.): `exp_strict_d57/_d76/.../_d1232` — collapse to one generic `*_strict_wide<W>` if tier params lift to const generics.
- **`generic_wide` vs per-tier:** sqrt/cbrt have generic_wide + `mg_divide_d38` + bespoke `lookup_d57_s20` — same fn three ways.
- **`fixed_d38.rs`:** the whole D38 256-bit "Fixed" kernel (`U256`/`U512`, `mul_u256`, `div_u512_*`, `isqrt_u512`, `round_to_i128_with`) — tier-named, heavily live for the narrow tier.

### (c) tier-named items 4.1/#79 rename/relocate
- All `*_dNN`/`*_dNNN` kernels + policy `_dNN_no_std`/`_std` triplets; `policy_triplet!` tables; `policy/triplet.rs` consts; `consts/wide.rs` `D{57..1232}_SCALE_REF`. (The `MagSign`/`wide_cast` shim was removed in 3.2.)
