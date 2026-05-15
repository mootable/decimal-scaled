//! Construction macros for `decimal-scaled`.
//!
//! See `macros/README.md` for the full spec. This crate now ships:
//!
//! - `d9!(…)`, `d18!(…)`, `d38!(…)` — narrow-tier entry points
//!   (i32 / i64 / i128 storage).
//! - `d76!(…)`, `d153!(…)`, `d307!(…)` — wide-tier entry points
//!   (Int256 / Int512 / Int1024 storage). Available when the
//!   parent crate's `d76` / `d153` / `d307` (or umbrella `wide` /
//!   `x-wide`) feature is on.
//! - Per-scale wrappers `d9s2!`, `d38s12!`, etc. live in the parent
//!   crate as `macro_rules!` declarations — they pre-bake `scale N`
//!   and forward to the proc-macro.
//!
//! Argument grammar (each entry point accepts the same shape):
//!
//! - `dN!(literal)` — scale inferred from the literal's fractional
//!   digit count.
//! - `dN!(literal, scale N)` — explicit target scale.
//! - `dN!(literal, rounded)` — opt into half-to-even rounding when
//!   the literal carries more fractional digits than the target.
//! - `dN!(0x… | 0o… | 0b…)` — Rust radix-prefix integer literals.
//!   Equivalent to passing `radix 16 / 8 / 2`; scale defaults to 0.
//! - `dN!(literal, radix R)` — accepts `R ∈ {2, 8, 10, 16}` and
//!   reinterprets the digit characters in that base.
//! - `dN!(expr, scale N)` — inline expression form (runtime scale-up);
//!   `scale N` is mandatory.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Expr, ExprLit, ExprUnary, Lit, LitInt, Result, Token, UnOp,
    parse::ParseStream,
};

// ── Width descriptor ───────────────────────────────────────────────────

/// One of the public decimal widths the macros can target. Each
/// carries the per-width metadata the literal / expression paths
/// need to emit correctly-typed bits.
#[derive(Clone, Copy)]
struct Width {
    /// Macro name as seen by users (`d38`, `d76`, …). Used in error
    /// messages.
    name: &'static str,
    /// `MAX_SCALE` for this width — the largest `SCALE` that fits
    /// without overflowing storage.
    max_scale: u32,
    /// Crate-relative path to the type. Filled into the emitted
    /// `::decimal_scaled::DXX::<SCALE>::from_bits(…)` expression.
    type_path: &'static str,
    /// Crate-relative path to the underlying signed integer (used
    /// for the inline-expression form's `let _v: …` type anchor).
    storage_path: &'static str,
    /// `true` for D76 / D153 / D307 (hand-rolled wide integer
    /// storage). Drives the emit-via-`from_str_radix` path.
    wide: bool,
}

const D9: Width = Width {
    name: "d9",
    max_scale: 9,
    type_path: "::decimal_scaled::D9",
    storage_path: "i32",
    wide: false,
};
const D18: Width = Width {
    name: "d18",
    max_scale: 18,
    type_path: "::decimal_scaled::D18",
    storage_path: "i64",
    wide: false,
};
const D38: Width = Width {
    name: "d38",
    max_scale: 38,
    type_path: "::decimal_scaled::D38",
    storage_path: "i128",
    wide: false,
};
const D76: Width = Width {
    name: "d76",
    max_scale: 76,
    type_path: "::decimal_scaled::D76",
    storage_path: "::decimal_scaled::Int256",
    wide: true,
};
const D153: Width = Width {
    name: "d153",
    max_scale: 153,
    type_path: "::decimal_scaled::D153",
    storage_path: "::decimal_scaled::Int512",
    wide: true,
};
const D307: Width = Width {
    name: "d307",
    max_scale: 307,
    type_path: "::decimal_scaled::D307",
    storage_path: "::decimal_scaled::Int1024",
    wide: true,
};

// ── Public proc-macro entry points ────────────────────────────────────

/// `d9!` — construct a [`decimal_scaled::D9`]`<SCALE>` value at
/// compile time. See the crate-level docs and `macros/README.md`.
#[proc_macro]
pub fn d9(input: TokenStream) -> TokenStream {
    expand_for(D9, input)
}

/// `d18!` — construct a [`decimal_scaled::D18`]`<SCALE>` value at
/// compile time. See the crate-level docs and `macros/README.md`.
#[proc_macro]
pub fn d18(input: TokenStream) -> TokenStream {
    expand_for(D18, input)
}

/// `d38!` — construct a [`decimal_scaled::D38`]`<SCALE>` value at
/// compile time. See the crate-level docs and `macros/README.md`.
#[proc_macro]
pub fn d38(input: TokenStream) -> TokenStream {
    expand_for(D38, input)
}

/// `d76!` — construct a [`decimal_scaled::D76`]`<SCALE>` value at
/// compile time. Requires the parent crate's `d76` / `wide` feature.
#[proc_macro]
pub fn d76(input: TokenStream) -> TokenStream {
    expand_for(D76, input)
}

/// `d153!` — construct a [`decimal_scaled::D153`]`<SCALE>` value at
/// compile time. Requires the parent crate's `d153` / `wide` feature.
#[proc_macro]
pub fn d153(input: TokenStream) -> TokenStream {
    expand_for(D153, input)
}

/// `d307!` — construct a [`decimal_scaled::D307`]`<SCALE>` value at
/// compile time. Requires the parent crate's `d307` / `x-wide`
/// feature.
#[proc_macro]
pub fn d307(input: TokenStream) -> TokenStream {
    expand_for(D307, input)
}

fn expand_for(width: Width, input: TokenStream) -> TokenStream {
    let parser = |stream: ParseStream| Invocation::parse_at(stream, width);
    match syn::parse::Parser::parse(parser, input) {
        Ok(inv) => inv.expand(),
        Err(e) => e.into_compile_error().into(),
    }
}

// ── Invocation model ──────────────────────────────────────────────────

enum Invocation {
    Literal {
        width: Width,
        /// Signed magnitude as a decimal digit string (no sign, no
        /// dot — already shifted so digits represent `value · 10^natural_scale`).
        digits: String,
        /// `-1` for negative literals, `+1` for non-negative.
        sign: i128,
        natural_scale: u32,
        scale_qualifier: Option<(u32, Span)>,
        rounded: bool,
        /// `true` for radix-prefixed (non-decimal) literals. For
        /// these, the parsed magnitude *is* the storage bits — the
        /// target scale only labels the resulting type, no
        /// additional shift is applied.
        radix_literal: bool,
        value_span: Span,
    },
    Expression {
        width: Width,
        expr: Expr,
        scale: u32,
        scale_span: Span,
    },
}

impl Invocation {
    fn parse_at(input: ParseStream, width: Width) -> Result<Self> {
        let value_expr: Expr = input.parse()?;
        let value_span = expr_span(&value_expr);

        let (scale_qualifier, radix_qualifier, rounded) = parse_qualifiers(input)?;

        // Literal vs expression detection.
        if let Some((sign, raw_str, lit_span)) = try_decimal_literal(&value_expr) {
            // Radix from either the qualifier or the literal prefix.
            let radix = pick_radix(&raw_str, radix_qualifier, lit_span)?;
            let (digits, natural_scale) =
                parse_value_token(&raw_str, lit_span, radix)?;
            Ok(Invocation::Literal {
                width,
                digits,
                sign,
                natural_scale,
                scale_qualifier,
                rounded,
                radix_literal: radix != 10,
                value_span,
            })
        } else {
            if let Some((_, radix_span)) = radix_qualifier {
                return Err(syn::Error::new(
                    radix_span,
                    "`radix` qualifier is only valid with a literal value",
                ));
            }
            let (scale, scale_span) = scale_qualifier.ok_or_else(|| {
                syn::Error::new(
                    value_span,
                    format!(
                        "scale must be specified for an expression value: `{}!(expr, scale N)`",
                        width.name
                    ),
                )
            })?;
            let _ = rounded; // future: rounded-expression mode
            Ok(Invocation::Expression {
                width,
                expr: value_expr,
                scale,
                scale_span,
            })
        }
    }

    fn expand(self) -> TokenStream {
        match self {
            Invocation::Literal {
                width,
                digits,
                sign,
                natural_scale,
                scale_qualifier,
                rounded,
                radix_literal,
                value_span,
            } => expand_literal(
                width,
                digits,
                sign,
                natural_scale,
                scale_qualifier,
                rounded,
                radix_literal,
                value_span,
            ),
            Invocation::Expression {
                width,
                expr,
                scale,
                scale_span,
            } => expand_expression(width, expr, scale, scale_span),
        }
    }
}

// ── Qualifier parsing ─────────────────────────────────────────────────

fn parse_qualifiers(
    input: ParseStream,
) -> Result<(Option<(u32, Span)>, Option<(u32, Span)>, bool)> {
    let mut scale_qualifier: Option<(u32, Span)> = None;
    let mut radix_qualifier: Option<(u32, Span)> = None;
    let mut rounded = false;
    while !input.is_empty() {
        let _: Token![,] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "scale" => {
                if scale_qualifier.is_some() {
                    return Err(syn::Error::new(
                        ident.span(),
                        "duplicate `scale` qualifier",
                    ));
                }
                let lit: LitInt = input.parse()?;
                let n: u32 = lit.base10_parse()?;
                scale_qualifier = Some((n, lit.span()));
            }
            "radix" => {
                if radix_qualifier.is_some() {
                    return Err(syn::Error::new(
                        ident.span(),
                        "duplicate `radix` qualifier",
                    ));
                }
                let lit: LitInt = input.parse()?;
                let r: u32 = lit.base10_parse()?;
                if !matches!(r, 2 | 8 | 10 | 16) {
                    return Err(syn::Error::new(
                        lit.span(),
                        format!("radix must be one of 2, 8, 10, 16 (got {r})"),
                    ));
                }
                radix_qualifier = Some((r, lit.span()));
            }
            "rounded" => {
                if rounded {
                    return Err(syn::Error::new(
                        ident.span(),
                        "duplicate `rounded` qualifier",
                    ));
                }
                rounded = true;
            }
            other => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!(
                        "unknown qualifier `{other}`; expected one of: scale, radix, rounded"
                    ),
                ));
            }
        }
    }
    Ok((scale_qualifier, radix_qualifier, rounded))
}

/// Resolve the effective radix for a literal. Reconciles an explicit
/// `radix N` qualifier with a Rust prefix (`0x`, `0o`, `0b`); reports
/// a conflict if the two disagree.
fn pick_radix(
    raw: &str,
    qualifier: Option<(u32, Span)>,
    span: Span,
) -> Result<u32> {
    let prefix_radix = if let Some(stripped) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        Some((16, stripped))
    } else if let Some(stripped) = raw.strip_prefix("0o").or_else(|| raw.strip_prefix("0O")) {
        Some((8, stripped))
    } else if let Some(stripped) = raw.strip_prefix("0b").or_else(|| raw.strip_prefix("0B")) {
        Some((2, stripped))
    } else {
        None
    };
    match (prefix_radix, qualifier) {
        (None, None) => Ok(10),
        (None, Some((r, _))) => Ok(r),
        (Some((p, _)), None) => Ok(p),
        (Some((p, _)), Some((r, _sp))) if p == r => Ok(r),
        (Some((p, _)), Some((r, sp))) => Err(syn::Error::new(
            sp,
            format!("radix qualifier ({r}) disagrees with literal prefix (radix {p})"),
        )),
    }.map_err(|e: syn::Error| {
        // Force the span to point at the literal when the disagreement
        // error was produced inside the closure above (no-op for the
        // common path).
        let _ = span;
        e
    })
}

// ── Literal-form codegen ─────────────────────────────────────────────

fn expand_literal(
    width: Width,
    digits: String,
    sign: i128,
    natural_scale: u32,
    scale_qualifier: Option<(u32, Span)>,
    rounded: bool,
    radix_literal: bool,
    value_span: Span,
) -> TokenStream {
    let (target_scale, scale_span) = match scale_qualifier {
        Some((n, sp)) => (n, sp),
        None => (natural_scale, value_span),
    };

    if target_scale > width.max_scale {
        return error(
            scale_span,
            format!(
                "scale {target_scale} exceeds max for {} (max = {})",
                width.name.to_uppercase(),
                width.max_scale
            ),
        );
    }

    // For radix-prefixed (non-decimal) literals, the parsed magnitude
    // IS the storage bits — the target scale only labels the
    // resulting type. Skip the scale shift entirely.
    if radix_literal {
        let _ = rounded;
        if width.wide {
            return emit_wide(width, target_scale, sign, &digits);
        } else {
            return emit_narrow(width, target_scale, sign, &digits, value_span);
        }
    }

    // Decimal literal path — shift the digit string to express
    // `value * 10^target_scale`:
    //   target == natural   → digits unchanged
    //   target  > natural   → append (target − natural) zeros
    //   target  < natural   → drop the bottom (natural − target) digits,
    //                         applying half-to-even rounding only if
    //                         `rounded` was set.
    let shifted_digits: String;
    let final_digits: &str;

    if target_scale == natural_scale {
        final_digits = &digits;
    } else if target_scale > natural_scale {
        let pad = target_scale - natural_scale;
        shifted_digits = pad_with_zeros(&digits, pad as usize);
        final_digits = &shifted_digits;
    } else {
        let shift = natural_scale - target_scale;
        // The digits string must be at least `shift` long for a
        // scale-down to even make sense; pad with leading zeros if
        // the underlying value has fewer digits than `shift`.
        let padded = if (digits.len() as u32) <= shift {
            pad_leading_zeros(&digits, (shift + 1) as usize - digits.len())
        } else {
            digits.clone()
        };
        let split = padded.len() - shift as usize;
        let (kept, dropped) = padded.split_at(split);
        let exact = dropped.bytes().all(|b| b == b'0');
        if !exact && !rounded {
            return error(
                value_span,
                format!(
                    "literal has {natural_scale} fractional digits, target scale {target_scale} would lose precision; pass `rounded` to opt into half-to-even rounding"
                ),
            );
        }
        if exact {
            shifted_digits = if kept.is_empty() { "0".to_string() } else { kept.to_string() };
        } else {
            // Half-to-even on the kept|dropped boundary.
            shifted_digits = round_half_to_even(kept, dropped, sign < 0);
        }
        final_digits = &shifted_digits;
    }

    if width.wide {
        emit_wide(width, target_scale, sign, final_digits)
    } else {
        emit_narrow(width, target_scale, sign, final_digits, value_span)
    }
}

fn emit_narrow(
    width: Width,
    target_scale: u32,
    sign: i128,
    digits: &str,
    value_span: Span,
) -> TokenStream {
    let magnitude: i128 = match digits.parse::<i128>() {
        Ok(v) => v,
        Err(_) => {
            return error(
                value_span,
                format!("scaled value overflows i128 before narrowing to {}'s storage", width.name.to_uppercase()),
            );
        }
    };
    let signed = match magnitude.checked_mul(sign) {
        Some(v) => v,
        None => {
            return error(
                value_span,
                "scaled value overflows i128 after applying sign".to_string(),
            );
        }
    };
    // Now range-check against the target storage's actual MIN/MAX.
    let (min, max): (i128, i128) = match width.storage_path {
        "i32" => (i32::MIN as i128, i32::MAX as i128),
        "i64" => (i64::MIN as i128, i64::MAX as i128),
        "i128" => (i128::MIN, i128::MAX),
        _ => unreachable!("narrow path called with non-narrow storage"),
    };
    if signed < min || signed > max {
        return error(
            value_span,
            format!(
                "scaled value {signed} overflows {}'s storage ({})",
                width.name.to_uppercase(),
                width.storage_path
            ),
        );
    }
    let bits_tokens: proc_macro2::TokenStream = match width.storage_path {
        "i32" => {
            let v = signed as i32;
            quote! { #v }
        }
        "i64" => {
            let v = signed as i64;
            quote! { #v }
        }
        "i128" => quote! { #signed },
        _ => unreachable!(),
    };
    let type_path: proc_macro2::TokenStream = width.type_path.parse().unwrap();
    let out = quote! {
        #type_path :: <#target_scale> :: from_bits(#bits_tokens)
    };
    out.into()
}

fn emit_wide(
    width: Width,
    target_scale: u32,
    sign: i128,
    digits: &str,
) -> TokenStream {
    // Wide integers have `from_str_radix` as `const fn`. Emit a
    // const block that materialises the bits at compile time.
    let signed_str = if sign < 0 {
        format!("-{digits}")
    } else {
        digits.to_string()
    };
    let type_path: proc_macro2::TokenStream = width.type_path.parse().unwrap();
    let storage_path: proc_macro2::TokenStream = width.storage_path.parse().unwrap();
    let err_msg = format!("{}! bits parse failed", width.name);
    let out = quote! {
        #type_path :: <#target_scale> :: from_bits({
            const BITS: #storage_path = match <#storage_path>::from_str_radix(#signed_str, 10) {
                ::core::result::Result::Ok(v) => v,
                ::core::result::Result::Err(_) => panic!(#err_msg),
            };
            BITS
        })
    };
    out.into()
}

// ── Expression-form codegen ───────────────────────────────────────────

fn expand_expression(
    width: Width,
    expr: Expr,
    scale: u32,
    scale_span: Span,
) -> TokenStream {
    if scale > width.max_scale {
        return error(
            scale_span,
            format!(
                "scale {scale} exceeds max for {} (max = {})",
                width.name.to_uppercase(),
                width.max_scale
            ),
        );
    }
    let type_path: proc_macro2::TokenStream = width.type_path.parse().unwrap();
    let storage_path: proc_macro2::TokenStream = width.storage_path.parse().unwrap();
    let err_msg = format!("{}! overflow: expression * 10^SCALE exceeds storage range", width.name);
    let out = if width.wide {
        // Wide path: build the multiplier in the wide-int type via
        // its `pow(scale)` const fn, then runtime-multiply.
        quote! {
            #type_path :: <#scale> :: from_bits({
                let _v: #storage_path = (#expr);
                let mult: #storage_path = <#storage_path>::from_str_radix("10", 10)
                    .expect("d{}! mult literal")
                    .pow(#scale);
                _v.checked_mul(mult).expect(#err_msg)
            })
        }
    } else if scale == 0 {
        quote! {
            #type_path :: <0> :: from_bits({
                let _v: #storage_path = (#expr);
                _v
            })
        }
    } else {
        // Narrow path: literal i32/i64/i128 multiplier via `pow`.
        let mult_lit: proc_macro2::TokenStream = match width.storage_path {
            "i32" => {
                let v = 10i32.pow(scale);
                quote! { #v }
            }
            "i64" => {
                let v = 10i64.pow(scale);
                quote! { #v }
            }
            "i128" => {
                let v = 10i128.pow(scale);
                quote! { #v }
            }
            _ => unreachable!(),
        };
        quote! {
            #type_path :: <#scale> :: from_bits({
                let _v: #storage_path = (#expr);
                _v.checked_mul(#mult_lit).expect(#err_msg)
            })
        }
    };
    out.into()
}

// ── String-arithmetic helpers ─────────────────────────────────────────

fn pad_with_zeros(digits: &str, pad: usize) -> String {
    let mut out = String::with_capacity(digits.len() + pad);
    out.push_str(digits);
    for _ in 0..pad {
        out.push('0');
    }
    out
}

fn pad_leading_zeros(digits: &str, pad: usize) -> String {
    let mut out = String::with_capacity(digits.len() + pad);
    for _ in 0..pad {
        out.push('0');
    }
    out.push_str(digits);
    out
}

/// Half-to-even rounding on a digit-string split at the kept|dropped
/// boundary. `negative` selects sign handling for the rounding tie
/// rules. Returns the new digit string (no sign).
fn round_half_to_even(kept: &str, dropped: &str, _negative: bool) -> String {
    debug_assert!(!dropped.is_empty());
    // Determine whether dropped > / == / < `5000…0`.
    let first = dropped.as_bytes()[0];
    let rest_nonzero = dropped.bytes().skip(1).any(|b| b != b'0');
    let round_up = match first.cmp(&b'5') {
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Equal => {
            // dropped starts with 5. If anything else is non-zero,
            // we're past the halfway mark — round up. Otherwise
            // exact half — round to even on `kept`'s last digit.
            if rest_nonzero {
                true
            } else {
                let last_kept = kept.bytes().last().unwrap_or(b'0');
                (last_kept - b'0') & 1 == 1
            }
        }
    };
    let kept_or_zero = if kept.is_empty() { "0" } else { kept };
    if !round_up {
        kept_or_zero.to_string()
    } else {
        add_one_to_digits(kept_or_zero)
    }
}

/// Increment a decimal-digit string by 1, propagating carry.
fn add_one_to_digits(digits: &str) -> String {
    let mut bytes: Vec<u8> = digits.as_bytes().to_vec();
    let mut i = bytes.len();
    let mut carry = 1u8;
    while i > 0 && carry > 0 {
        i -= 1;
        let d = bytes[i] - b'0' + carry;
        if d >= 10 {
            bytes[i] = b'0';
            carry = 1;
        } else {
            bytes[i] = b'0' + d;
            carry = 0;
        }
    }
    if carry > 0 {
        bytes.insert(0, b'1');
    }
    String::from_utf8(bytes).expect("ascii digits")
}

// ── Numeric-token parsing ─────────────────────────────────────────────

fn try_decimal_literal(expr: &Expr) -> Option<(i128, String, Span)> {
    match expr {
        Expr::Lit(ExprLit { lit, .. }) => match lit {
            Lit::Float(f) => Some((1, f.to_string(), f.span())),
            Lit::Int(i) => Some((1, i.to_string(), i.span())),
            _ => None,
        },
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr: inner,
            ..
        }) => {
            if let Expr::Lit(ExprLit { lit, .. }) = inner.as_ref() {
                match lit {
                    Lit::Float(f) => Some((-1, f.to_string(), f.span())),
                    Lit::Int(i) => Some((-1, i.to_string(), i.span())),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expr_span(expr: &Expr) -> Span {
    use syn::spanned::Spanned;
    expr.span()
}

/// Parses the raw literal token (`raw`, no sign) under the chosen
/// `radix`. Returns `(digits, natural_scale)` where `digits` is the
/// integer magnitude as a decimal string and `natural_scale` is the
/// inferred decimal scale of the value as written.
///
/// For decimal literals (radix == 10) this handles fractional and
/// scientific notation. For non-decimal radices it accepts integer-
/// only Rust-prefixed forms (`0x…`, `0o…`, `0b…`) plus the raw digit
/// string from an explicit `radix N` qualifier; mid-fractional non-
/// decimal forms (`1.A3, radix 16`) are rejected — `syn` doesn't
/// tokenise them as a single literal anyway.
fn parse_value_token(
    raw: &str,
    span: Span,
    radix: u32,
) -> Result<(String, u32)> {
    // Reject Rust type suffixes (1.5_f64 etc.).
    for (i, c) in raw.char_indices() {
        if (c == 'i' || c == 'u') || (c == 'f' && i > 0 && !raw[..i].contains('.') && !raw[..i].chars().last().map_or(false, |x| x.is_ascii_digit())) {
            // No-op: we'll handle the `f`/`i`/`u` filter via parse failures.
            let _ = i;
        }
    }
    if let Some(idx) = raw.find(|c: char| c == 'f') {
        // `1_f64`-style suffix.
        if idx > 0 && raw.as_bytes()[idx - 1] == b'_' {
            return Err(syn::Error::new(
                span,
                "type suffixes (e.g. _i64, _f32) are not accepted in decimal-scaled literals",
            ));
        }
    }
    if let Some(idx) = raw.rfind(|c: char| c == 'i' || c == 'u') {
        // Ignore the `i` in `radix` (impossible here — `raw` is the
        // value token only) and the `i` that follows a digit
        // (`0o755_i32`).
        if idx > 0 && raw.as_bytes()[idx - 1] == b'_' {
            return Err(syn::Error::new(
                span,
                "type suffixes (e.g. _i64, _f32) are not accepted in decimal-scaled literals",
            ));
        }
    }

    if radix == 10 {
        return parse_decimal_token(raw, span);
    }

    // Non-decimal: accept Rust-prefix forms and bare digit strings.
    // Strip prefix if present and verify it matches `radix`.
    let digits_part = strip_radix_prefix(raw).map(|(p, rest)| {
        // p must match radix; if not, the caller's pick_radix already
        // flagged it.
        let _ = p;
        rest
    }).unwrap_or(raw);

    if digits_part.contains('.') {
        return Err(syn::Error::new(
            span,
            "fractional non-decimal literals are not supported (use an explicit `scale N` with an integer-only digit string instead)",
        ));
    }

    // Underscore separators are stripped.
    let cleaned: String = digits_part.chars().filter(|c| *c != '_').collect();
    if cleaned.is_empty() {
        return Err(syn::Error::new(span, "empty digit string"));
    }

    // Parse the digit string in the given radix as an i128, then
    // re-render as a base-10 string. (Wide-tier widths may need more
    // than i128 — for now we narrow through i128 and let the wide
    // path's range check catch any overflow.)
    let magnitude = match i128::from_str_radix(&cleaned, radix) {
        Ok(v) => v,
        Err(_) => {
            return Err(syn::Error::new(
                span,
                format!(
                    "digit string `{cleaned}` is not valid in radix {radix} or overflows i128"
                ),
            ));
        }
    };
    Ok((magnitude.to_string(), 0))
}

fn strip_radix_prefix(raw: &str) -> Option<(u32, &str)> {
    if let Some(rest) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        Some((16, rest))
    } else if let Some(rest) = raw.strip_prefix("0o").or_else(|| raw.strip_prefix("0O")) {
        Some((8, rest))
    } else if let Some(rest) = raw.strip_prefix("0b").or_else(|| raw.strip_prefix("0B")) {
        Some((2, rest))
    } else {
        None
    }
}

fn parse_decimal_token(raw: &str, span: Span) -> Result<(String, u32)> {
    let (mantissa, sci_exp) = match raw.find(['e', 'E']) {
        Some(idx) => {
            let m = &raw[..idx];
            let e: i32 = raw[idx + 1..].parse().map_err(|_| {
                syn::Error::new(
                    span,
                    format!("invalid scientific exponent: `{}`", &raw[idx + 1..]),
                )
            })?;
            (m, e)
        }
        None => (raw, 0_i32),
    };
    let (int_part, frac_part) = match mantissa.find('.') {
        Some(idx) => {
            let int_part = &mantissa[..idx];
            let frac_part = &mantissa[idx + 1..];
            if int_part.is_empty() {
                return Err(syn::Error::new(
                    span,
                    "decimal literals require a digit on each side of the dot (write `0.5` not `.5`)",
                ));
            }
            if frac_part.is_empty() {
                return Err(syn::Error::new(
                    span,
                    "decimal literals require a digit on each side of the dot (write `1.0` not `1.`)",
                ));
            }
            (int_part, frac_part)
        }
        None => (mantissa, ""),
    };
    let cleaned_int: String = int_part.chars().filter(|c| *c != '_').collect();
    let cleaned_frac: String = frac_part.chars().filter(|c| *c != '_').collect();
    for c in cleaned_int.chars().chain(cleaned_frac.chars()) {
        if !c.is_ascii_digit() {
            return Err(syn::Error::new(
                span,
                format!("invalid digit `{c}` in decimal literal"),
            ));
        }
    }
    let mantissa_scale = cleaned_frac.len() as u32;
    let mut digits = cleaned_int;
    digits.push_str(&cleaned_frac);
    // Strip leading zeros so the digit string canonicalises to its
    // numerical magnitude.
    let trimmed = digits.trim_start_matches('0');
    let digits = if trimmed.is_empty() { "0".to_string() } else { trimmed.to_string() };

    // Apply scientific exponent: natural_scale = max(0, mantissa_scale - sci_exp).
    let signed_natural = (mantissa_scale as i64) - (sci_exp as i64);
    if signed_natural >= 0 {
        Ok((digits, signed_natural as u32))
    } else {
        // sci_exp > mantissa_scale: pad trailing zeros, natural scale = 0.
        let pad = (-signed_natural) as usize;
        Ok((pad_with_zeros(&digits, pad), 0))
    }
}

fn error(span: Span, msg: String) -> TokenStream {
    syn::Error::new(span, msg).into_compile_error().into()
}
