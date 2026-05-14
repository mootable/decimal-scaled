//! Construction macros for `decimal-scaled`.
//!
//! See `macros/Macros.md` for the full spec. Current sub-phase ships:
//!
//! - `d128!(literal)` — auto-scale inference from the literal's
//!   fractional-digit count.
//! - `d128!(literal, scale N)` — explicit target scale.
//! - `d128!(literal, rounded)` — opt into round-half-to-even when the
//!   target scale is smaller than the literal's natural scale.
//! - Scientific notation: `1.5e3`, `6.022e23`, `1e-9`.
//!
//! Deferred to follow-on commits:
//! - `radix N` qualifier (`d128!(0x7B, radix 16, scale 2)`).
//! - Inline-expression input (`d128!(some_var, scale N)`).
//! - Per-scale variants `d128s2!`, `d128s12!`, etc. as macro_rules in
//!   the parent crate.
//! - Wide-tier macros `d256!`, `d512!`, `d1024!` (Phase 5+).

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    LitFloat, LitInt, Result, Token,
    parse::{Parse, ParseStream},
};

const D128_MAX_SCALE: u32 = 38;

/// `d128!` — construct a `D128<SCALE>` value at compile time.
///
/// See the crate-level docs and `macros/Macros.md` for the full spec.
#[proc_macro]
pub fn d128(input: TokenStream) -> TokenStream {
    match syn::parse::<D128Invocation>(input) {
        Ok(invocation) => invocation.expand(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// One invocation of `d128!`: parsed value + qualifiers.
struct D128Invocation {
    /// Negative-or-positive sign of the value (`1` or `-1`).
    sign: i128,
    /// All decimal digits of the mantissa, with no sign / dot / underscores.
    digits: String,
    /// Number of decimal digits AFTER the dot in the mantissa as written
    /// (counts trailing zeros).
    mantissa_fractional: u32,
    /// The signed `e` exponent (0 if no scientific notation).
    sci_exponent: i32,
    /// Explicit `scale N` qualifier, or `None` if scale should be inferred.
    scale_qualifier: Option<(u32, Span)>,
    /// `true` if the `rounded` qualifier was given.
    rounded: bool,
    /// Span of the literal value, for error reporting.
    value_span: Span,
}

impl Parse for D128Invocation {
    fn parse(input: ParseStream) -> Result<Self> {
        // ----- value position ------------------------------------------
        let (sign, digits, mantissa_fractional, sci_exponent, value_span) =
            parse_value(input)?;

        // ----- qualifiers (comma-separated) ----------------------------
        let mut scale_qualifier: Option<(u32, Span)> = None;
        let mut rounded = false;
        while !input.is_empty() {
            let _: Token![,] = input.parse()?;
            let qualifier_ident: syn::Ident = input.parse()?;
            match qualifier_ident.to_string().as_str() {
                "scale" => {
                    if scale_qualifier.is_some() {
                        return Err(syn::Error::new(
                            qualifier_ident.span(),
                            "duplicate `scale` qualifier",
                        ));
                    }
                    let lit: LitInt = input.parse()?;
                    let n: u32 = lit.base10_parse()?;
                    scale_qualifier = Some((n, lit.span()));
                }
                "rounded" => {
                    if rounded {
                        return Err(syn::Error::new(
                            qualifier_ident.span(),
                            "duplicate `rounded` qualifier",
                        ));
                    }
                    rounded = true;
                }
                "radix" => {
                    return Err(syn::Error::new(
                        qualifier_ident.span(),
                        "`radix` qualifier is not yet implemented; pending Phase 1C extension",
                    ));
                }
                other => {
                    return Err(syn::Error::new(
                        qualifier_ident.span(),
                        format!(
                            "unknown qualifier `{other}`; expected one of: scale, rounded"
                        ),
                    ));
                }
            }
        }

        Ok(D128Invocation {
            sign,
            digits,
            mantissa_fractional,
            sci_exponent,
            scale_qualifier,
            rounded,
            value_span,
        })
    }
}

impl D128Invocation {
    fn expand(self) -> TokenStream {
        // Effective fractional count after applying the sci exponent:
        // 1.5e3 has mantissa_fractional = 1, sci_exponent = 3 -> 0 fractional digits.
        // 1e-9  has mantissa_fractional = 0, sci_exponent = -9 -> 9 fractional digits.
        let signed_natural_scale =
            (self.mantissa_fractional as i64) - (self.sci_exponent as i64);
        if signed_natural_scale < 0 {
            // sci_exponent > mantissa_fractional: mantissa effectively has
            // trailing zeros to add. Pad the digits with the extra zeros so
            // the "digits as integer" calculation captures them, then set
            // natural scale to 0.
            let pad = (-signed_natural_scale) as usize;
            return finish(
                self.sign,
                pad_with_zeros(&self.digits, pad),
                0,
                self.scale_qualifier,
                self.rounded,
                self.value_span,
            );
        }
        let natural_scale = signed_natural_scale as u32;
        finish(
            self.sign,
            self.digits,
            natural_scale,
            self.scale_qualifier,
            self.rounded,
            self.value_span,
        )
    }
}

fn pad_with_zeros(digits: &str, pad: usize) -> String {
    let mut out = String::with_capacity(digits.len() + pad);
    out.push_str(digits);
    for _ in 0..pad {
        out.push('0');
    }
    out
}

/// Final stage of expansion: given the post-sci digits and natural scale,
/// resolve the target scale (qualifier or inference), compute the storage
/// bits, and emit the token stream.
fn finish(
    sign: i128,
    digits: String,
    natural_scale: u32,
    scale_qualifier: Option<(u32, Span)>,
    rounded: bool,
    value_span: Span,
) -> TokenStream {
    let (target_scale, scale_span) = match scale_qualifier {
        Some((n, sp)) => (n, sp),
        None => (natural_scale, value_span),
    };
    if target_scale > D128_MAX_SCALE {
        return error(
            scale_span,
            format!(
                "scale {target_scale} exceeds max for D128 (max = {D128_MAX_SCALE})"
            ),
        );
    }

    // Parse the digits as a magnitude in i128.
    let magnitude: i128 = match digits.parse::<i128>() {
        Ok(v) => v,
        Err(_) => {
            return error(
                value_span,
                "literal magnitude overflows i128 before scaling".to_string(),
            );
        }
    };
    let signed = match magnitude.checked_mul(sign) {
        Some(v) => v,
        None => {
            return error(
                value_span,
                "literal magnitude overflows i128 after applying sign".to_string(),
            );
        }
    };

    // Adjust to the target scale.
    let bits: i128 = if target_scale == natural_scale {
        signed
    } else if target_scale > natural_scale {
        // Scale-up: multiply by 10^(diff). Always exact.
        let shift = target_scale - natural_scale;
        let multiplier = match 10i128.checked_pow(shift) {
            Some(v) => v,
            None => {
                return error(
                    scale_span,
                    format!("10^{shift} overflows i128 during scale-up"),
                );
            }
        };
        match signed.checked_mul(multiplier) {
            Some(v) => v,
            None => {
                return error(
                    value_span,
                    "scaled value overflows D128 storage (i128)".to_string(),
                );
            }
        }
    } else {
        // Scale-down: lossy unless `rounded` is present.
        let shift = natural_scale - target_scale;
        let divisor = 10i128.pow(shift);
        if !rounded {
            // Reject any non-exact loss.
            if signed % divisor != 0 {
                return error(
                    value_span,
                    format!(
                        "literal has {natural_scale} fractional digits, target scale {target_scale} would lose precision; pass `rounded` to opt into half-to-even rounding"
                    ),
                );
            }
            signed / divisor
        } else {
            // Half-to-even rounding (mirrors src/rounding.rs).
            let quotient = signed / divisor;
            let remainder = signed % divisor;
            if remainder == 0 {
                quotient
            } else {
                let abs_rem = remainder.unsigned_abs();
                let half = (divisor / 2) as u128;
                if abs_rem < half {
                    quotient
                } else if abs_rem > half {
                    if signed >= 0 { quotient + 1 } else { quotient - 1 }
                } else if quotient % 2 == 0 {
                    quotient
                } else if signed >= 0 {
                    quotient + 1
                } else {
                    quotient - 1
                }
            }
        }
    };

    // `quote!` literalises the values automatically: target_scale is a u32,
    // bits is an i128 literal.
    let expanded = quote! {
        ::decimal_scaled::D128::<#target_scale>::from_bits(#bits)
    };
    expanded.into()
}

fn error(span: Span, msg: String) -> TokenStream {
    syn::Error::new(span, msg).into_compile_error().into()
}

// ---------------------------------------------------------------------
// Value parsing
//
// Accepts:
// - decimal literals: `1.23`, `-1.23`, `+1.23`, `1`, `1.0`, `1_234.567_89`
// - scientific notation: `1.5e3`, `1.5e-3`, `6.022e23`, `-2.5e-2`
//
// Rejects:
// - `.5` (require leading zero)
// - `1.` (require trailing zero)
// - radix-prefix literals 0x/0o/0b (Phase 1C extension)
// - inline expressions (Phase 1C extension)
// ---------------------------------------------------------------------

fn parse_value(
    input: ParseStream,
) -> Result<(i128, String, u32, i32, Span)> {
    // Optional leading sign.
    let sign = if input.peek(Token![-]) {
        let neg: Token![-] = input.parse()?;
        let _ = neg; // span used via fold below if needed
        -1_i128
    } else if input.peek(Token![+]) {
        let _: Token![+] = input.parse()?;
        1_i128
    } else {
        1_i128
    };

    // We accept either a Float lit (digits.fraction[e±exp]) or an Int lit
    // (digits[e±exp] is also possible because Rust tokenises `1e3` as Int).
    let lookahead = input.lookahead1();
    let (raw_str, lit_span) = if lookahead.peek(LitFloat) {
        let lit: LitFloat = input.parse()?;
        (lit.to_string(), lit.span())
    } else if lookahead.peek(LitInt) {
        let lit: LitInt = input.parse()?;
        (lit.to_string(), lit.span())
    } else {
        return Err(lookahead.error());
    };

    // syn's LitFloat::to_string() preserves the source form.
    let (digits, mantissa_fractional, sci_exponent) =
        parse_numeric_token(&raw_str, lit_span)?;

    Ok((sign, digits, mantissa_fractional, sci_exponent, lit_span))
}

/// Parses a numeric token string (e.g. `"1.5e3"`, `"123"`, `"1_234.567_89"`)
/// into `(digits_without_dot_or_underscore, fractional_digit_count, sci_exponent)`.
///
/// Rejects malformed forms (bare leading `.5`, bare trailing `1.`,
/// radix-prefix `0x/0o/0b`, suffixed `_i64` / `_f64`).
fn parse_numeric_token(
    raw: &str,
    span: Span,
) -> Result<(String, u32, i32)> {
    // Reject Rust suffixes (`1.5_f64`, `1_i64`, etc.).
    if let Some(idx) = raw.find(|c: char| c == 'i' || c == 'u' || c == 'f') {
        // 'e' is part of sci notation, NOT a suffix. Suffixes 'i', 'u', 'f'
        // appear only after digits, and only after any 'e' exponent.
        // (We won't see 'I'/'U'/'F' from syn token parsing.)
        if raw.as_bytes()[idx] != b'e' && raw.as_bytes()[idx] != b'E' {
            return Err(syn::Error::new(
                span,
                "type suffixes (e.g. _i64, _f32) are not accepted in d128! literals",
            ));
        }
    }

    if raw.starts_with("0x") || raw.starts_with("0o") || raw.starts_with("0b") {
        return Err(syn::Error::new(
            span,
            "radix-prefix literals (0x, 0o, 0b) are not yet implemented; pending Phase 1C extension",
        ));
    }

    // Split off the exponent if present.
    let (mantissa_str, sci_exponent) = match raw.find(['e', 'E']) {
        Some(idx) => {
            let m = &raw[..idx];
            let e_str = &raw[idx + 1..];
            let e: i32 = e_str.parse().map_err(|_| {
                syn::Error::new(
                    span,
                    format!("invalid scientific exponent: `{e_str}`"),
                )
            })?;
            (m, e)
        }
        None => (raw, 0_i32),
    };

    // Split mantissa on the dot.
    let (int_part, frac_part) = match mantissa_str.find('.') {
        Some(idx) => {
            let int_part = &mantissa_str[..idx];
            let frac_part = &mantissa_str[idx + 1..];
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
        None => (mantissa_str, ""),
    };

    // Strip underscores from each half.
    let int_digits: String = int_part.chars().filter(|c| *c != '_').collect();
    let frac_digits: String = frac_part.chars().filter(|c| *c != '_').collect();

    // Validate every char is a decimal digit.
    for d in int_digits.chars().chain(frac_digits.chars()) {
        if !d.is_ascii_digit() {
            return Err(syn::Error::new(
                span,
                format!("invalid digit `{d}` in decimal literal"),
            ));
        }
    }

    let mantissa_fractional = frac_digits.len() as u32;
    let digits = format!("{int_digits}{frac_digits}");
    Ok((digits, mantissa_fractional, sci_exponent))
}
