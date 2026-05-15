//! Construction macros for `decimal-scaled`.
//!
//! See `macros/Macros.md` for the full spec. Current sub-phase ships:
//!
//! - `d38!(literal)` — auto-scale inference from the literal's
//!   fractional-digit count.
//! - `d38!(literal, scale N)` — explicit target scale.
//! - `d38!(literal, rounded)` — opt into round-half-to-even when the
//!   target scale is smaller than the literal's natural scale.
//! - Scientific notation: `1.5e3`, `6.022e23`, `1e-9`.
//! - Inline expressions: `d38!(some_var, scale 4)`,
//!   `d38!(10 * 12 + 3, scale 0)`. `scale N` is mandatory when the
//!   value is anything other than a numeric literal.
//!
//! Deferred to follow-on commits:
//! - `radix N` qualifier (`d38!(0x7B, radix 16, scale 2)`).
//! - Per-scale variants `d38s2!`, `d38s12!`, etc. as macro_rules in
//!   the parent crate.
//! - Wide-tier macros `d76!`, `d153!`, `d307!` (Phase 5+).

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Expr, ExprLit, ExprUnary, Lit, LitInt, Result, Token, UnOp,
    parse::{Parse, ParseStream},
};

const D38_MAX_SCALE: u32 = 38;

/// `d38!` — construct a `D38<SCALE>` value at compile time (or, for
/// inline-expression input, by evaluating the expression at runtime
/// and scaling it).
///
/// See the crate-level docs and `macros/Macros.md` for the full spec.
#[proc_macro]
pub fn d38(input: TokenStream) -> TokenStream {
    match syn::parse::<D38Invocation>(input) {
        Ok(invocation) => invocation.expand(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// One invocation of `d38!`: either a parsed numeric literal or an
/// inline expression, plus qualifiers.
enum D38Invocation {
    Literal {
        sign: i128,
        digits: String,
        mantissa_fractional: u32,
        sci_exponent: i32,
        scale_qualifier: Option<(u32, Span)>,
        rounded: bool,
        value_span: Span,
    },
    Expression {
        expr: Expr,
        scale: u32,
        scale_span: Span,
    },
}

impl Parse for D38Invocation {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the value position as a Rust expression. This is
        // permissive: it accepts numeric literals, sign-prefixed
        // literals, identifiers, arithmetic expressions, function
        // calls, etc.
        let value_expr: Expr = input.parse()?;
        let value_span = expr_span(&value_expr);

        // Parse the qualifier list.
        let (scale_qualifier, rounded) = parse_qualifiers(input)?;

        // Determine whether the value is a numeric literal (possibly
        // sign-prefixed). If so, take the compile-time literal path;
        // otherwise the runtime expression path.
        match try_decimal_literal(&value_expr) {
            Some((sign, raw_str, lit_span)) => {
                let (digits, mantissa_fractional, sci_exponent) =
                    parse_numeric_token(&raw_str, lit_span)?;
                Ok(D38Invocation::Literal {
                    sign,
                    digits,
                    mantissa_fractional,
                    sci_exponent,
                    scale_qualifier,
                    rounded,
                    value_span,
                })
            }
            None => {
                // Expression path: scale is mandatory.
                let (scale, scale_span) = match scale_qualifier {
                    Some(s) => s,
                    None => {
                        return Err(syn::Error::new(
                            value_span,
                            "scale must be specified for an expression value: `d38!(expr, scale N)`",
                        ));
                    }
                };
                let _ = rounded; // future: rounded-expression mode
                let _ = value_span;
                Ok(D38Invocation::Expression {
                    expr: value_expr,
                    scale,
                    scale_span,
                })
            }
        }
    }
}

fn parse_qualifiers(
    input: ParseStream,
) -> Result<(Option<(u32, Span)>, bool)> {
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
    Ok((scale_qualifier, rounded))
}

impl D38Invocation {
    fn expand(self) -> TokenStream {
        match self {
            D38Invocation::Literal {
                sign,
                digits,
                mantissa_fractional,
                sci_exponent,
                scale_qualifier,
                rounded,
                value_span,
            } => expand_literal(
                sign,
                digits,
                mantissa_fractional,
                sci_exponent,
                scale_qualifier,
                rounded,
                value_span,
            ),
            D38Invocation::Expression {
                expr,
                scale,
                scale_span,
            } => expand_expression(expr, scale, scale_span),
        }
    }
}

fn expand_literal(
    sign: i128,
    digits: String,
    mantissa_fractional: u32,
    sci_exponent: i32,
    scale_qualifier: Option<(u32, Span)>,
    rounded: bool,
    value_span: Span,
) -> TokenStream {
    // Apply sci exponent to digits/scale.
    let signed_natural_scale =
        (mantissa_fractional as i64) - (sci_exponent as i64);
    let (digits, natural_scale) = if signed_natural_scale < 0 {
        let pad = (-signed_natural_scale) as usize;
        (pad_with_zeros(&digits, pad), 0_u32)
    } else {
        (digits, signed_natural_scale as u32)
    };

    let (target_scale, scale_span) = match scale_qualifier {
        Some((n, sp)) => (n, sp),
        None => (natural_scale, value_span),
    };
    if target_scale > D38_MAX_SCALE {
        return error(
            scale_span,
            format!(
                "scale {target_scale} exceeds max for D38 (max = {D38_MAX_SCALE})"
            ),
        );
    }

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

    let bits: i128 = if target_scale == natural_scale {
        signed
    } else if target_scale > natural_scale {
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
                    "scaled value overflows D38 storage (i128)".to_string(),
                );
            }
        }
    } else {
        let shift = natural_scale - target_scale;
        let divisor = 10i128.pow(shift);
        if !rounded {
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
            half_to_even(signed, divisor)
        }
    };

    let expanded = quote! {
        ::decimal_scaled::D38::<#target_scale>::from_bits(#bits)
    };
    expanded.into()
}

fn expand_expression(expr: Expr, scale: u32, scale_span: Span) -> TokenStream {
    if scale > D38_MAX_SCALE {
        return error(
            scale_span,
            format!(
                "scale {scale} exceeds max for D38 (max = {D38_MAX_SCALE})"
            ),
        );
    }

    let expanded = if scale == 0 {
        // Scale 0: no multiply needed; just bind to enforce i128 type.
        quote! {
            ::decimal_scaled::D38::<0>::from_bits({
                let _v: i128 = (#expr);
                _v
            })
        }
    } else {
        // General case: scale-factor multiply with overflow check.
        let multiplier = 10i128.pow(scale);
        quote! {
            ::decimal_scaled::D38::<#scale>::from_bits({
                let _v: i128 = (#expr);
                _v.checked_mul(#multiplier)
                    .expect("d38! overflow: expression * 10^SCALE exceeds i128 range")
            })
        }
    };
    expanded.into()
}

fn half_to_even(signed: i128, divisor: i128) -> i128 {
    let quotient = signed / divisor;
    let remainder = signed % divisor;
    if remainder == 0 {
        return quotient;
    }
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

fn pad_with_zeros(digits: &str, pad: usize) -> String {
    let mut out = String::with_capacity(digits.len() + pad);
    out.push_str(digits);
    for _ in 0..pad {
        out.push('0');
    }
    out
}

fn error(span: Span, msg: String) -> TokenStream {
    syn::Error::new(span, msg).into_compile_error().into()
}

// ---------------------------------------------------------------------
// Expr inspection — does this look like a sign-prefixed numeric literal?
// ---------------------------------------------------------------------

/// If `expr` is a numeric literal (with an optional unary `-`), returns
/// `(sign, raw_string, span)` where `raw_string` is the literal as
/// written (digits, dot, sci-exponent — but no sign). Otherwise returns
/// `None`, meaning the value is a general expression.
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

// ---------------------------------------------------------------------
// Numeric token parsing — string -> (digits, fractional count, sci exp).
// ---------------------------------------------------------------------

fn parse_numeric_token(
    raw: &str,
    span: Span,
) -> Result<(String, u32, i32)> {
    // Reject Rust type suffixes (`1.5_f64`, `1_i64`, etc.).
    if let Some(idx) = raw.find(|c: char| c == 'i' || c == 'u' || c == 'f') {
        if raw.as_bytes()[idx] != b'e' && raw.as_bytes()[idx] != b'E' {
            return Err(syn::Error::new(
                span,
                "type suffixes (e.g. _i64, _f32) are not accepted in d38! literals",
            ));
        }
    }

    if raw.starts_with("0x") || raw.starts_with("0o") || raw.starts_with("0b") {
        return Err(syn::Error::new(
            span,
            "radix-prefix literals (0x, 0o, 0b) are not yet implemented; pending Phase 1C extension",
        ));
    }

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

    let int_digits: String = int_part.chars().filter(|c| *c != '_').collect();
    let frac_digits: String = frac_part.chars().filter(|c| *c != '_').collect();

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
