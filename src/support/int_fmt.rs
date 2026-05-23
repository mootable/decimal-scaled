//! Radix-to-string formatting for little-endian `u64` limb magnitudes.
//!
//! The radix-conversion machinery the `Int<N>` / `Uint<N>` `Display` /
//! `LowerHex` / … impls emit through. Pure text I/O over a limb slice —
//! no arithmetic policy, so it lives in the cross-cutting `support`
//! bucket alongside the other formatters, not in the integer algorithm
//! layer.

use crate::int::algos::limbs::is_zero;

/// Scratch capacity for the formatter's working buffer — 288 u64 limbs
/// (18432 bits), covering the widest integer the crate formats.
const SCRATCH_LIMBS: usize = 288;

/// `limbs /= radix` in place, returning the remainder. `radix` must be a
/// u64 (so the per-limb divide stays inside `u128 / u64`).
fn div_small_radix(limbs: &mut [u64], radix: u64) -> u64 {
    let mut rem: u64 = 0;
    for limb in limbs.iter_mut().rev() {
        let acc = ((rem as u128) << 64) | (*limb as u128);
        *limb = (acc / (radix as u128)) as u64;
        rem = (acc % (radix as u128)) as u64;
    }
    rem
}

/// `10^19` — the largest power of ten that fits in a `u64`
/// (`10^19 < 2^64 < 10^20`). Dividing the magnitude by this constant
/// peels off 19 decimal digits per full-width pass instead of one.
const POW10_19: u64 = 10_000_000_000_000_000_000;
/// Number of decimal digits emitted per `POW10_19` chunk.
const POW10_19_DIGITS: usize = 19;

/// Format a u64 limb slice into `buf` in the given radix (`2..=16`).
///
/// For the decimal radix this peels 19 digits per full-width divide by
/// dividing the magnitude by `10^19` (the largest power of ten below
/// `2^64`) and emitting the 19-digit `u64` remainder with cheap native
/// arithmetic. The expensive `O(limbs)` full-width small-divide then runs
/// once per 19 digits rather than once per digit. The other radixes
/// (2 / 8 / 16) keep the one-divide-per-digit loop.
pub(crate) fn fmt_into<'a>(limbs: &[u64], radix: u64, lower: bool, buf: &'a mut [u8]) -> &'a str {
    if is_zero(limbs) {
        let last = buf.len() - 1;
        buf[last] = b'0';
        return core::str::from_utf8(&buf[last..]).unwrap();
    }
    let mut work = [0u64; SCRATCH_LIMBS];
    work[..limbs.len()].copy_from_slice(limbs);
    let wl = limbs.len();
    let mut pos = buf.len();

    if radix == 10 {
        // Peel one 19-digit base-10^19 chunk per full-width divide.
        loop {
            let chunk = div_small_radix(&mut work[..wl], POW10_19);
            if is_zero(&work[..wl]) {
                // Most-significant chunk: emit without leading-zero pad.
                let mut v = chunk;
                loop {
                    pos -= 1;
                    buf[pos] = b'0' + (v % 10) as u8;
                    v /= 10;
                    if v == 0 {
                        break;
                    }
                }
                break;
            }
            // Interior chunk: always exactly 19 zero-padded digits.
            let mut v = chunk;
            for _ in 0..POW10_19_DIGITS {
                pos -= 1;
                buf[pos] = b'0' + (v % 10) as u8;
                v /= 10;
            }
        }
        return core::str::from_utf8(&buf[pos..]).unwrap();
    }

    let digits: &[u8] = if lower {
        b"0123456789abcdef"
    } else {
        b"0123456789ABCDEF"
    };
    while !is_zero(&work[..wl]) {
        let r = div_small_radix(&mut work[..wl], radix);
        pos -= 1;
        buf[pos] = digits[r as usize];
    }
    core::str::from_utf8(&buf[pos..]).unwrap()
}
