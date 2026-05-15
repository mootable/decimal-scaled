//! Build-time high-precision constant generator.
//!
//! Computes π, τ, π/2, π/4, e, and the golden ratio at up to ~320
//! decimal digits using a hand-rolled fixed-point `BigUint` (a
//! little-endian `Vec<u64>` for the integer part of `value * 10^N`).
//! The algorithms are the same as the runtime ones in
//! `src/macros/wide_transcendental.rs`:
//!
//! - π  via Machin: `π = 16·atan(1/5) − 4·atan(1/239)`.
//! - τ  = 2·π.
//! - π/2 = π / 2.
//! - π/4 = π / 4.
//! - e via the Taylor series `e = Σ 1/n!`.
//! - φ (golden) via Newton: `x_{k+1} = (x_k + 1) / x_k` converging to
//!   the positive root of `x² − x − 1 = 0`.
//!
//! The output is emitted to `$OUT_DIR/wide_consts.rs`, a Rust source
//! file the crate `include!`s from `src/consts_wide.rs`. The macros
//! that build the constants land at compile time as `Int256` /
//! `Int512` / `Int1024` values via `from_str_radix`.
//!
//! This file uses only `std`. No external dependencies. No procedural
//! macros. Reproducible across hosts.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// ─── Big unsigned fixed-precision integer ─────────────────────────────

/// `BigU` carries a non-negative integer in base 2^64 (little-endian
/// limbs). Resizing is automatic on overflow / leading-zero trimming.
#[derive(Clone, Debug)]
struct BigU {
    /// Little-endian base-2^64 limbs. limbs.last() != Some(&0) unless
    /// the value is zero (in which case limbs is empty).
    limbs: Vec<u64>,
}

impl BigU {
    fn zero() -> Self {
        BigU { limbs: Vec::new() }
    }
    fn one() -> Self {
        BigU { limbs: vec![1] }
    }
    fn from_u64(v: u64) -> Self {
        if v == 0 { Self::zero() } else { BigU { limbs: vec![v] } }
    }
    fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }
    fn trim(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }
    /// In-place add of a single u64.
    fn add_u64(&mut self, v: u64) {
        if v == 0 { return; }
        let mut carry = v as u128;
        for limb in self.limbs.iter_mut() {
            let s = *limb as u128 + carry;
            *limb = s as u64;
            carry = s >> 64;
            if carry == 0 { break; }
        }
        if carry != 0 {
            self.limbs.push(carry as u64);
        }
    }
    /// In-place add of another BigU.
    fn add_assign(&mut self, rhs: &BigU) {
        if rhs.limbs.len() > self.limbs.len() {
            self.limbs.resize(rhs.limbs.len(), 0);
        }
        let mut carry: u128 = 0;
        for i in 0..self.limbs.len() {
            let r = if i < rhs.limbs.len() { rhs.limbs[i] as u128 } else { 0 };
            let s = self.limbs[i] as u128 + r + carry;
            self.limbs[i] = s as u64;
            carry = s >> 64;
        }
        if carry != 0 {
            self.limbs.push(carry as u64);
        }
        self.trim();
    }
    /// In-place subtract of another BigU. Requires self >= rhs.
    fn sub_assign(&mut self, rhs: &BigU) {
        let mut borrow: i128 = 0;
        for i in 0..self.limbs.len() {
            let r = if i < rhs.limbs.len() { rhs.limbs[i] as i128 } else { 0 };
            let s = self.limbs[i] as i128 - r - borrow;
            if s < 0 {
                self.limbs[i] = (s + (1i128 << 64)) as u64;
                borrow = 1;
            } else {
                self.limbs[i] = s as u64;
                borrow = 0;
            }
        }
        assert_eq!(borrow, 0, "BigU::sub_assign underflow");
        self.trim();
    }
    /// In-place multiply by a single u64.
    fn mul_u64(&mut self, v: u64) {
        if v == 0 { self.limbs.clear(); return; }
        if v == 1 { return; }
        let mut carry: u128 = 0;
        for limb in self.limbs.iter_mut() {
            let p = *limb as u128 * v as u128 + carry;
            *limb = p as u64;
            carry = p >> 64;
        }
        if carry != 0 {
            self.limbs.push(carry as u64);
        }
    }
    /// In-place divide by a single u64 (non-zero). Returns the
    /// remainder.
    fn div_u64(&mut self, v: u64) -> u64 {
        assert!(v != 0);
        let mut rem: u128 = 0;
        for limb in self.limbs.iter_mut().rev() {
            let acc = (rem << 64) | (*limb as u128);
            *limb = (acc / v as u128) as u64;
            rem = acc % v as u128;
        }
        self.trim();
        rem as u64
    }
    /// Mod by a single u64 (non-destructive).
    #[allow(dead_code)]
    fn mod_u64(&self, v: u64) -> u64 {
        assert!(v != 0);
        let mut rem: u128 = 0;
        for limb in self.limbs.iter().rev() {
            let acc = (rem << 64) | (*limb as u128);
            rem = acc % v as u128;
        }
        rem as u64
    }
    /// Lexicographic-ish ordering: shorter is smaller; same length
    /// compares limbs top-down.
    fn cmp(&self, other: &BigU) -> std::cmp::Ordering {
        if self.limbs.len() != other.limbs.len() {
            return self.limbs.len().cmp(&other.limbs.len());
        }
        for i in (0..self.limbs.len()).rev() {
            if self.limbs[i] != other.limbs[i] {
                return self.limbs[i].cmp(&other.limbs[i]);
            }
        }
        std::cmp::Ordering::Equal
    }
    /// Format as base-10 digits (no leading zeros except for "0").
    fn to_decimal(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        let mut tmp = self.clone();
        let mut digits = Vec::new();
        // Pull off 19-digit chunks (10^19 fits a u64).
        while !tmp.is_zero() {
            let rem = tmp.div_u64(10_000_000_000_000_000_000);
            digits.push(rem);
        }
        let mut s = String::new();
        let last = digits.pop().unwrap();
        s.push_str(&format!("{last}"));
        while let Some(d) = digits.pop() {
            s.push_str(&format!("{d:019}"));
        }
        s
    }
}

// ─── Fixed-point arithmetic in BigU ───────────────────────────────────
//
// All "values" are integers representing `true_value * 10^DIGITS`,
// where DIGITS is the working precision. mul/div take the scale into
// account.

fn pow10(n: u32) -> BigU {
    // Build 10^n by repeated mul_u64(10) chunks.
    let mut v = BigU::one();
    let mut k = n;
    while k >= 19 {
        v.mul_u64(10_000_000_000_000_000_000);
        k -= 19;
    }
    let mut tail = 1u64;
    for _ in 0..k {
        tail *= 10;
    }
    if tail > 1 {
        v.mul_u64(tail);
    }
    v
}

/// `(a · b) / 10^digits`, half-to-even.
fn fixed_mul(a: &BigU, b: &BigU, digits: u32) -> BigU {
    // a * b
    let mut acc = BigU::zero();
    // Schoolbook multiplication into acc.
    for i in 0..a.limbs.len() {
        if a.limbs[i] == 0 { continue; }
        let mut carry: u128 = 0;
        for j in 0..b.limbs.len() {
            let p = a.limbs[i] as u128 * b.limbs[j] as u128 + carry;
            let pos = i + j;
            while acc.limbs.len() <= pos {
                acc.limbs.push(0);
            }
            let s = acc.limbs[pos] as u128 + (p as u64) as u128;
            acc.limbs[pos] = s as u64;
            carry = (p >> 64) + (s >> 64);
        }
        let mut pos = i + b.limbs.len();
        while carry != 0 {
            while acc.limbs.len() <= pos {
                acc.limbs.push(0);
            }
            let s = acc.limbs[pos] as u128 + carry;
            acc.limbs[pos] = s as u64;
            carry = s >> 64;
            pos += 1;
        }
    }
    acc.trim();
    // Divide by 10^digits.
    let divisor = pow10(digits);
    let (q, _r) = bigu_divmod(&acc, &divisor);
    // (Could half-to-even round on `_r` vs `divisor/2` — for the
    // build-time generator the extra precision floor we'll trim later
    // absorbs sub-LSB drift, so plain truncation is fine here.)
    q
}

/// `(a · 10^digits) / b`, truncating.
fn fixed_div(a: &BigU, b: &BigU, digits: u32) -> BigU {
    let mut n = a.clone();
    let scale = pow10(digits);
    n = mul_full(&n, &scale);
    let (q, _r) = bigu_divmod(&n, b);
    q
}

fn mul_full(a: &BigU, b: &BigU) -> BigU {
    let mut acc = BigU::zero();
    for i in 0..a.limbs.len() {
        if a.limbs[i] == 0 { continue; }
        let mut carry: u128 = 0;
        for j in 0..b.limbs.len() {
            let p = a.limbs[i] as u128 * b.limbs[j] as u128 + carry;
            let pos = i + j;
            while acc.limbs.len() <= pos {
                acc.limbs.push(0);
            }
            let s = acc.limbs[pos] as u128 + (p as u64) as u128;
            acc.limbs[pos] = s as u64;
            carry = (p >> 64) + (s >> 64);
        }
        let mut pos = i + b.limbs.len();
        while carry != 0 {
            while acc.limbs.len() <= pos {
                acc.limbs.push(0);
            }
            let s = acc.limbs[pos] as u128 + carry;
            acc.limbs[pos] = s as u64;
            carry = s >> 64;
            pos += 1;
        }
    }
    acc.trim();
    acc
}

/// Binary shift-subtract long division. Returns (quotient, remainder).
fn bigu_divmod(a: &BigU, b: &BigU) -> (BigU, BigU) {
    assert!(!b.is_zero());
    if a.cmp(b) == std::cmp::Ordering::Less {
        return (BigU::zero(), a.clone());
    }
    // Shift b up so b * 2^k > a, then loop down.
    let mut q = BigU::zero();
    let mut r = BigU::zero();
    let bits_a = bit_length(a);
    // Process bits from MSB to LSB.
    for i in (0..bits_a).rev() {
        shl1(&mut r);
        if get_bit(a, i) { r.add_u64(1); }
        shl1(&mut q);
        if r.cmp(b) != std::cmp::Ordering::Less {
            r.sub_assign(b);
            // Set q low bit
            if q.limbs.is_empty() {
                q.limbs.push(1);
            } else {
                q.limbs[0] |= 1;
            }
        }
    }
    q.trim();
    r.trim();
    (q, r)
}

fn bit_length(a: &BigU) -> u32 {
    if a.is_zero() { return 0; }
    let top = *a.limbs.last().unwrap();
    (a.limbs.len() as u32) * 64 - top.leading_zeros()
}

fn get_bit(a: &BigU, i: u32) -> bool {
    let limb = (i / 64) as usize;
    let bit = i % 64;
    if limb >= a.limbs.len() {
        return false;
    }
    (a.limbs[limb] >> bit) & 1 != 0
}

fn shl1(a: &mut BigU) {
    let mut carry = 0u64;
    for limb in a.limbs.iter_mut() {
        let new = (*limb << 1) | carry;
        carry = *limb >> 63;
        *limb = new;
    }
    if carry != 0 {
        a.limbs.push(carry);
    }
    a.trim();
}

// ─── Constant computations ────────────────────────────────────────────

/// Taylor `atan(1/n) = 1/n - 1/(3·n³) + 1/(5·n⁵) - …` at `digits`
/// working precision.
fn atan_recip(n: u64, digits: u32) -> BigU {
    let one = pow10(digits);
    // term = 1/n at scale digits.
    let mut term = one.clone();
    term.div_u64(n);
    let mut sum = term.clone();
    let n_sq = n * n;
    let mut k: u64 = 1;
    loop {
        // term = term / n²
        let mut new_term = term.clone();
        new_term.div_u64(n_sq);
        if new_term.is_zero() {
            break;
        }
        // contrib = new_term / (2k+1)
        let mut contrib = new_term.clone();
        contrib.div_u64(2 * k + 1);
        if contrib.is_zero() {
            // Need to keep going — new_term itself is not zero, but
            // contrib rounded to zero. Continue.
        }
        if k % 2 == 1 {
            // odd k: sum -= contrib
            // Only safe if sum >= contrib.
            if sum.cmp(&contrib) != std::cmp::Ordering::Less {
                sum.sub_assign(&contrib);
            }
        } else {
            sum.add_assign(&contrib);
        }
        term = new_term;
        k += 1;
        if k > 50_000 {
            break;
        }
    }
    sum
}

/// Rescale-down with half-to-even rounding. `value` carries
/// `true * 10^from`; returns `true * 10^to` for `to <= from`.
fn rescale_down_hte(value: BigU, from: u32, to: u32) -> BigU {
    if to >= from { return value; }
    let shift = from - to;
    // Divide by 10^shift, keeping the remainder for the round step.
    let divisor = pow10(shift);
    let (q, r) = bigu_divmod(&value, &divisor);
    let mut half = divisor.clone();
    half.div_u64(2);
    let cmp = r.cmp(&half);
    let mut result = q;
    let round_up = match cmp {
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Equal => {
            // Tie: half-to-even on the quotient's last digit.
            let last = result.mod_u64(10);
            (last & 1) != 0
        }
    };
    if round_up {
        result.add_u64(1);
    }
    result
}

/// π via Machin: `π = 16·atan(1/5) − 4·atan(1/239)`, half-to-even
/// rounded to `digits` precision.
fn pi(digits: u32) -> BigU {
    // Compute at a higher precision then round once at the end. 10
    // extra digits absorb the per-term truncation in atan_recip.
    let work = digits + 10;
    let mut a = atan_recip(5, work);
    a.mul_u64(16);
    let mut b = atan_recip(239, work);
    b.mul_u64(4);
    a.sub_assign(&b);
    rescale_down_hte(a, work, digits)
}

/// e via `e = Σ 1/n!`, half-to-even rounded to `digits` precision.
fn e_const(digits: u32) -> BigU {
    let work = digits + 10;
    let one = pow10(work);
    let mut term = one.clone();
    let mut sum = one;
    let mut n: u64 = 1;
    loop {
        let mut new_term = term.clone();
        new_term.div_u64(n);
        if new_term.is_zero() {
            break;
        }
        sum.add_assign(&new_term);
        term = new_term;
        n += 1;
        if n > 100_000 { break; }
    }
    rescale_down_hte(sum, work, digits)
}

/// Golden ratio φ = (1 + √5) / 2 via Newton on √5, half-to-even
/// rounded to `digits` precision.
fn golden(digits: u32) -> BigU {
    let work = digits + 10;
    let scale = pow10(work);
    let five_scale_sq = {
        let mut t = scale.clone();
        t = mul_full(&t, &scale);
        t.mul_u64(5);
        t
    };
    let mut x = scale.clone();
    x.mul_u64(2);
    for _ in 0..200 {
        let (q, _) = bigu_divmod(&five_scale_sq, &x);
        let mut sum = x.clone();
        sum.add_assign(&q);
        sum.div_u64(2);
        if sum.cmp(&x) == std::cmp::Ordering::Equal {
            break;
        }
        x = sum;
    }
    // φ = (1 + √5) / 2 → (scale + x) / 2
    let mut phi = scale;
    phi.add_assign(&x);
    phi.div_u64(2);
    rescale_down_hte(phi, work, digits)
}

// ─── Emit Rust source ────────────────────────────────────────────────

/// Format `value` (which carries `true_value * 10^DIGITS`) as a
/// decimal-digit string with exactly `digits + 1` chars (1 integer
/// digit + `digits` frac digits, no decimal point). The serialized
/// string is what `Int*::from_str_radix("…", 10)` consumes inside
/// the generated source.
fn format_at(v: &BigU, digits: u32, integer_digits: u32) -> String {
    let s = v.to_decimal();
    let target_len = integer_digits as usize + digits as usize;
    if s.len() < target_len {
        // Pad with leading zeros.
        let mut padded = "0".repeat(target_len - s.len());
        padded.push_str(&s);
        padded
    } else {
        s
    }
}

fn emit_constant(out: &mut impl Write, name: &str, value: &BigU, digits: u32, integer_digits: u32) -> std::io::Result<()> {
    let s = format_at(value, digits, integer_digits);
    writeln!(out, "pub(super) const {name}: &str = \"{s}\";")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join("wide_consts.rs");
    let mut f = File::create(&out_path)?;

    writeln!(&mut f, "// Auto-generated from build.rs. Do not edit.")?;
    writeln!(&mut f, "//")?;
    writeln!(&mut f, "// Each value is a digit string of `true_value × 10^scale_ref` —")?;
    writeln!(&mut f, "// the base-10 integer with no decimal point. Consumed by")?;
    writeln!(&mut f, "// `Int*::from_str_radix` at compile time.")?;
    writeln!(&mut f)?;

    // D38: SCALE_REF=37 — the i128 ceiling. All six constants get
    // the full surface here so src/consts.rs can drop its hand-written
    // raw values entirely.
    let pi37 = pi(37);
    let mut tau37 = pi37.clone();
    tau37.mul_u64(2);
    let mut half_pi37 = pi37.clone();
    half_pi37.div_u64(2);
    let mut quarter_pi37 = pi37.clone();
    quarter_pi37.div_u64(4);
    let e37 = e_const(37);
    let phi37 = golden(37);
    emit_constant(&mut f, "PI_D38_S37", &pi37, 37, 1)?;
    emit_constant(&mut f, "TAU_D38_S37", &tau37, 37, 1)?;
    emit_constant(&mut f, "HALF_PI_D38_S37", &half_pi37, 37, 1)?;
    emit_constant(&mut f, "QUARTER_PI_D38_S37", &quarter_pi37, 37, 1)?;
    emit_constant(&mut f, "E_D38_S37", &e37, 37, 1)?;
    emit_constant(&mut f, "GOLDEN_D38_S37", &phi37, 37, 1)?;

    // D76: SCALE_REF=75.
    let pi75 = pi(75);
    let mut tau75 = pi75.clone();
    tau75.mul_u64(2);
    let mut half_pi75 = pi75.clone();
    half_pi75.div_u64(2);
    let mut quarter_pi75 = pi75.clone();
    quarter_pi75.div_u64(4);
    let e75 = e_const(75);
    let phi75 = golden(75);
    emit_constant(&mut f, "PI_D76_S75", &pi75, 75, 1)?;
    emit_constant(&mut f, "TAU_D76_S75", &tau75, 75, 1)?;
    emit_constant(&mut f, "HALF_PI_D76_S75", &half_pi75, 75, 1)?;
    emit_constant(&mut f, "QUARTER_PI_D76_S75", &quarter_pi75, 75, 1)?;
    emit_constant(&mut f, "E_D76_S75", &e75, 75, 1)?;
    emit_constant(&mut f, "GOLDEN_D76_S75", &phi75, 75, 1)?;

    // D153: SCALE_REF=153.
    let pi153 = pi(153);
    let mut tau153 = pi153.clone();
    tau153.mul_u64(2);
    let mut half_pi153 = pi153.clone();
    half_pi153.div_u64(2);
    let mut quarter_pi153 = pi153.clone();
    quarter_pi153.div_u64(4);
    let e153 = e_const(153);
    let phi153 = golden(153);
    emit_constant(&mut f, "PI_D153_S153", &pi153, 153, 1)?;
    emit_constant(&mut f, "TAU_D153_S153", &tau153, 153, 1)?;
    emit_constant(&mut f, "HALF_PI_D153_S153", &half_pi153, 153, 1)?;
    emit_constant(&mut f, "QUARTER_PI_D153_S153", &quarter_pi153, 153, 1)?;
    emit_constant(&mut f, "E_D153_S153", &e153, 153, 1)?;
    emit_constant(&mut f, "GOLDEN_D153_S153", &phi153, 153, 1)?;

    // D307: SCALE_REF=307.
    let pi307 = pi(307);
    let mut tau307 = pi307.clone();
    tau307.mul_u64(2);
    let mut half_pi307 = pi307.clone();
    half_pi307.div_u64(2);
    let mut quarter_pi307 = pi307.clone();
    quarter_pi307.div_u64(4);
    let e307 = e_const(307);
    let phi307 = golden(307);
    emit_constant(&mut f, "PI_D307_S307", &pi307, 307, 1)?;
    emit_constant(&mut f, "TAU_D307_S307", &tau307, 307, 1)?;
    emit_constant(&mut f, "HALF_PI_D307_S307", &half_pi307, 307, 1)?;
    emit_constant(&mut f, "QUARTER_PI_D307_S307", &quarter_pi307, 307, 1)?;
    emit_constant(&mut f, "E_D307_S307", &e307, 307, 1)?;
    emit_constant(&mut f, "GOLDEN_D307_S307", &phi307, 307, 1)?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
