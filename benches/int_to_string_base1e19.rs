//! Focused micro-bench for the wide-integer decimal `to_string` base
//! case (#38): peeling one 19-digit base-`10^19` chunk per full-width
//! divide vs the historic one-digit-per-full-width-divide loop.
//!
//! The optimized path is exercised through `Display` (`to_string()`),
//! which routes `Int<N>` / `Uint<N>` magnitudes through
//! `limbs_fmt_into_u64`. The baseline is a verbatim local copy of the
//! pre-change one-divide-per-digit loop operating on the same exposed
//! limbs (`as_limbs()`), so the only difference between the two timed
//! closures is exactly the change under test.
//!
//! Manual harness (no criterion) — completes in a few seconds.
//!
//! Run with:
//!   `cargo bench --features wide,x-wide,xx-wide --bench int_to_string_base1e19`

use decimal_scaled::{Int1024, Int256, Int4096, Int512};
use std::hint::black_box;
use std::time::Instant;

/// Baseline: the historic one-digit-per-full-width-divide loop.
/// `limbs /= 10` in place per digit, emitting one digit per pass.
fn baseline_to_string(limbs: &[u64]) -> String {
    let mut buf = [0u8; 4096];
    if limbs.iter().all(|&w| w == 0) {
        return "0".to_string();
    }
    let mut work = [0u64; 64];
    work[..limbs.len()].copy_from_slice(limbs);
    let wl = limbs.len();
    let mut pos = buf.len();
    loop {
        // limbs /= 10, returning remainder.
        let mut rem: u64 = 0;
        for limb in work[..wl].iter_mut().rev() {
            let acc = ((rem as u128) << 64) | (*limb as u128);
            *limb = (acc / 10) as u64;
            rem = (acc % 10) as u64;
        }
        pos -= 1;
        buf[pos] = b'0' + rem as u8;
        if work[..wl].iter().all(|&w| w == 0) {
            break;
        }
    }
    String::from_utf8(buf[pos..].to_vec()).unwrap()
}

fn time<F: FnMut()>(iters: u32, mut f: F) -> f64 {
    for _ in 0..(iters / 8 + 1) {
        f();
    }
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    start.elapsed().as_nanos() as f64 / iters as f64
}

macro_rules! run {
    ($label:literal, $v:expr, $iters:expr) => {{
        let v = $v;
        let limbs: &[u64] = v.as_limbs();
        let opt = v.to_string();
        let base = baseline_to_string(limbs);
        assert_eq!(opt, base, concat!($label, ": optimized and baseline disagree"));

        let ns_base = time($iters, || {
            black_box(baseline_to_string(black_box(limbs)));
        });
        let ns_opt = time($iters, || {
            black_box(black_box(&v).to_string());
        });
        let speedup = ns_base / ns_opt;
        println!(
            "{:<10} digits={:<5} baseline={:>10.1} ns  optimized={:>10.1} ns  speedup={:>6.2}x  ({:+.1}%)",
            $label,
            opt.len(),
            ns_base,
            ns_opt,
            speedup,
            (ns_opt - ns_base) / ns_base * 100.0,
        );
    }};
}

fn main() {
    println!("# wide-int decimal to_string: base-10^19 chunk vs one-divide-per-digit\n");

    // Near-full-width magnitudes so the divide loop runs the full limb
    // count every pass (the realistic worst case for wide Display).
    let s256 = "9".repeat(76);
    let s512 = "9".repeat(153);
    let s1024 = "9".repeat(307);
    let s4096 = "9".repeat(1232);

    let v256: Int256 = s256.parse().unwrap();
    let v512: Int512 = s512.parse().unwrap();
    let v1024: Int1024 = s1024.parse().unwrap();
    let v4096: Int4096 = s4096.parse().unwrap();

    run!("Int256", v256, 200_000);
    run!("Int512", v512, 100_000);
    run!("Int1024", v1024, 50_000);
    run!("Int4096", v4096, 10_000);
}
