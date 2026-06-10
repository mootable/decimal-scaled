// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Construction macros for `decimal-scaled`.
//!
//! See `macros/README.md` for the full spec. This crate now ships:
//!
//! - `d18!(…)`, `d38!(…)` — narrow-tier entry points
//!   (i32 / i64 / i128 storage).
//! - `d76!(…)`, `d153!(…)`, `d307!(…)` — wide-tier entry points
//!   (Int::<4> / Int::<8> / Int::<16> storage). Available when the
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
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{Expr, ExprLit, ExprUnary, Lit, Result, UnOp};

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
    /// Leaf identifier of the decimal type (`D38`, `D76`, …). At
    /// emit time we prepend the resolved root path
    /// (`::<consumer-import-name>`) via [`crate_root`].
    type_leaf: &'static str,
    /// Storage type for the inline-expression form's `let _v: …`
    /// anchor. For narrow widths this is a primitive (`"i32"` /
    /// `"i64"` / `"i128"`) and used as-is. For wide widths it's a
    /// leaf inside the decimal-scaled crate (`"Int::<4>"` /
    /// `"Int::<8>"` / `"Int::<16>"`) and prefixed with the resolved
    /// root path at emit time.
    storage_path: &'static str,
    /// `true` for D76 / D153 / D307 (hand-rolled wide integer
    /// storage). Drives the emit-via-`from_str_radix` path.
    wide: bool,
}

/// Resolves the consuming crate's import name for `decimal-scaled`
/// and returns it as a leading absolute path (`::<name>`), so
/// `type_path()` / `storage_path()` can prepend it to a leaf
/// identifier. Falls back to `::decimal_scaled` if the lookup
/// fails — same behaviour as the original hard-coded path.
///
/// Note: `proc-macro-crate` can only see the *direct* dependencies
/// of the consumer crate. A consumer that wants to use `d38!`
/// without listing `decimal-scaled` itself (relying on a transitive
/// dep through some wrapper crate) will still fail — there is no
/// proc-macro mechanism to resolve transitive deps. The
/// fixed-macro-style wrapper pattern hits the same limit.
fn crate_root() -> proc_macro2::TokenStream {
    use proc_macro_crate::{FoundCrate, crate_name};
    use quote::quote;
    match crate_name("decimal-scaled") {
        Ok(FoundCrate::Itself) => quote! { ::decimal_scaled },
        Ok(FoundCrate::Name(name)) => {
            let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { ::#ident }
        }
        Err(_) => quote! { ::decimal_scaled },
    }
}

/// Build the absolute path to a decimal type (`<root>::D38`).
fn type_path(width: Width) -> proc_macro2::TokenStream {
    let root = crate_root();
    let leaf = proc_macro2::Ident::new(width.type_leaf, proc_macro2::Span::call_site());
    quote::quote! { #root :: #leaf }
}

/// Build the storage-path token stream. Narrow widths just emit
/// the primitive (`i32` / `i64` / `i128`); wide widths emit
/// `<root>::Int<NNN>`.
fn storage_path_tokens(width: Width) -> proc_macro2::TokenStream {
    if width.wide {
        let root = crate_root();
        // Parsed (not a bare `Ident`) so a storage leaf may carry generic
        // args, e.g. D38's `Int::<2>`. Plain leaves like `Int::<4>` parse to a
        // single ident, unchanged.
        let leaf: proc_macro2::TokenStream = width
            .storage_path
            .parse()
            .expect("storage_path is a valid type path");
        quote::quote! { #root :: #leaf }
    } else {
        width.storage_path.parse().unwrap()
    }
}

const D18: Width = Width {
    name: "d18",
    max_scale: 17,
    type_leaf: "D18",
    // D18 now backs onto `Int<1>` (was `i64`); emit via the wide path.
    storage_path: "Int::<1>",
    wide: true,
};
const D38: Width = Width {
    name: "d38",
    max_scale: 37,
    type_leaf: "D38",
    // D38 now backs onto `Int<2>` (was `i128`); emit via the wide
    // `from_str_radix` path so the raw bits build at the storage type.
    storage_path: "Int::<2>",
    wide: true,
};
const D76: Width = Width {
    name: "d76",
    max_scale: 75,
    type_leaf: "D76",
    storage_path: "Int::<4>",
    wide: true,
};
const D153: Width = Width {
    name: "d153",
    max_scale: 152,
    type_leaf: "D153",
    storage_path: "Int::<8>",
    wide: true,
};
const D307: Width = Width {
    name: "d307",
    max_scale: 306,
    type_leaf: "D307",
    storage_path: "Int::<16>",
    wide: true,
};
const D57: Width = Width {
    name: "d57",
    max_scale: 56,
    type_leaf: "D57",
    storage_path: "Int::<3>",
    wide: true,
};
const D115: Width = Width {
    name: "d115",
    max_scale: 114,
    type_leaf: "D115",
    storage_path: "Int::<6>",
    wide: true,
};
const D230: Width = Width {
    name: "d230",
    max_scale: 229,
    type_leaf: "D230",
    storage_path: "Int::<12>",
    wide: true,
};
const D462: Width = Width {
    name: "d462",
    max_scale: 461,
    type_leaf: "D462",
    storage_path: "Int::<24>",
    wide: true,
};
const D616: Width = Width {
    name: "d616",
    max_scale: 615,
    type_leaf: "D616",
    storage_path: "Int::<32>",
    wide: true,
};
const D924: Width = Width {
    name: "d924",
    max_scale: 923,
    type_leaf: "D924",
    storage_path: "Int::<48>",
    wide: true,
};
const D1232: Width = Width {
    name: "d1232",
    max_scale: 1231,
    type_leaf: "D1232",
    storage_path: "Int::<64>",
    wide: true,
};

// ── Public proc-macro entry points ────────────────────────────────────

/// `d18!` — construct a `decimal_scaled::D18<SCALE>` value at
/// compile time. See the crate-level docs and `macros/README.md`.
#[proc_macro]
pub fn d18(input: TokenStream) -> TokenStream {
    expand_for(D18, input)
}

/// `d38!` — construct a `decimal_scaled::D38<SCALE>` value at
/// compile time. See the crate-level docs and `macros/README.md`.
#[proc_macro]
pub fn d38(input: TokenStream) -> TokenStream {
    expand_for(D38, input)
}

/// `d76!` — construct a `decimal_scaled::D76<SCALE>` value at
/// compile time. Requires the parent crate's `d76` / `wide` feature.
#[proc_macro]
pub fn d76(input: TokenStream) -> TokenStream {
    expand_for(D76, input)
}

/// `d153!` — construct a `decimal_scaled::D153<SCALE>` value at
/// compile time. Requires the parent crate's `d153` / `wide` feature.
#[proc_macro]
pub fn d153(input: TokenStream) -> TokenStream {
    expand_for(D153, input)
}

/// `d307!` — construct a `decimal_scaled::D307<SCALE>` value at
/// compile time. Requires the parent crate's `d307` / `x-wide`
/// feature.
#[proc_macro]
pub fn d307(input: TokenStream) -> TokenStream {
    expand_for(D307, input)
}

/// `d57!` — construct a `decimal_scaled::D57<SCALE>` value at
/// compile time. Requires the parent crate's `d57` / `wide` feature.
#[proc_macro]
pub fn d57(input: TokenStream) -> TokenStream {
    expand_for(D57, input)
}

/// `d115!` — construct a `decimal_scaled::D115<SCALE>` value at
/// compile time. Requires the parent crate's `d115` / `wide` feature.
#[proc_macro]
pub fn d115(input: TokenStream) -> TokenStream {
    expand_for(D115, input)
}

/// `d230!` — construct a `decimal_scaled::D230<SCALE>` value at
/// compile time. Requires the parent crate's `d230` / `wide` feature.
#[proc_macro]
pub fn d230(input: TokenStream) -> TokenStream {
    expand_for(D230, input)
}

/// `d462!` — construct a `decimal_scaled::D462<SCALE>` value at
/// compile time. Requires the parent crate's `d462` / `x-wide`
/// feature.
#[proc_macro]
pub fn d462(input: TokenStream) -> TokenStream {
    expand_for(D462, input)
}

/// `d616!` — construct a `decimal_scaled::D616<SCALE>` value at
/// compile time. Requires the parent crate's `d616` / `x-wide`
/// feature.
#[proc_macro]
pub fn d616(input: TokenStream) -> TokenStream {
    expand_for(D616, input)
}

/// `d924!` — construct a `decimal_scaled::D924<SCALE>` value at
/// compile time. Requires the parent crate's `d924` / `xx-wide`
/// feature.
#[proc_macro]
pub fn d924(input: TokenStream) -> TokenStream {
    expand_for(D924, input)
}

/// `d1232!` — construct a `decimal_scaled::D1232<SCALE>` value at
/// compile time. Requires the parent crate's `d1232` / `xx-wide`
/// feature.
#[proc_macro]
pub fn d1232(input: TokenStream) -> TokenStream {
    expand_for(D1232, input)
}

fn expand_for(width: Width, input: TokenStream) -> TokenStream {
    // Convert to proc_macro2::TokenStream so we can manipulate
    // token trees directly. Rust's lexer won't accept
    // `1.A3` as a single token, so we have to do our own
    // splitting for the radix-fractional case.
    let tokens: TokenStream2 = input.into();
    match parse_invocation(tokens, width) {
        Ok(inv) => inv.expand(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// Split the input on top-level commas, scan qualifier segments
/// to find any `radix N`, then dispatch the value segment to the
/// right parser. The radix-fractional path (`1.A3, radix 16`)
/// goes through a custom token walker because the value position
/// isn't valid Rust syntax. Decimal / radix-prefixed / expression
/// shapes go through the standard Rust-Expr path.
fn parse_invocation(tokens: TokenStream2, width: Width) -> Result<Invocation> {
    let segments = split_top_commas(tokens);
    if segments.is_empty() || segments[0].is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            format!("{}!() requires a value argument", width.name),
        ));
    }

    // Quick pre-scan over qualifier segments: did the user pass
    // an explicit `radix N`? We need that to decide which value
    // parser to use, since `radix 16` lets `1.A3` be a literal.
    let mut explicit_radix: Option<(u32, Span)> = None;
    for seg in &segments[1..] {
        let Some((TokenTree::Ident(id), TokenTree::Literal(lit))) =
            seg.first().zip(seg.get(1))
        else {
            continue;
        };
        if id != "radix" {
            continue;
        }
        if let Ok(r) = lit.to_string().parse::<u32>() {
            explicit_radix = Some((r, lit.span()));
        }
    }

    // Parse the value segment. The custom radix-fractional walker
    // only fires when the user passed a non-decimal `radix N`
    // *and* the segment doesn't already parse as a Rust Expr.
    let value_segment = &segments[0];
    let value_parse = try_radix_fractional(value_segment, explicit_radix)?;

    if let Some((digits, sign, natural_scale, value_span)) = value_parse {
        // Custom radix-fractional path. We've already established
        // a non-decimal radix, so parse the qualifiers normally and
        // skip pick_radix.
        let (scale_qualifier, _radix_q, rounded) = parse_qualifier_segments(&segments[1..], width)?;
        return Ok(Invocation::Literal(LiteralForm {
            width,
            digits,
            sign,
            natural_scale,
            scale_qualifier,
            rounded,
            radix_literal: true,
            value_span,
        }));
    }

    // Standard Rust-Expr path. Re-assemble the value tokens for the
    // syn Expr parser.
    let value_ts: TokenStream2 = value_segment.iter().cloned().collect();
    let value_expr: Expr = syn::parse2(value_ts)?;
    let value_span = expr_span(&value_expr);

    let (scale_qualifier, radix_qualifier, rounded) =
        parse_qualifier_segments(&segments[1..], width)?;

    if let Some((sign, raw_str, lit_span)) = try_decimal_literal(&value_expr) {
        let radix = pick_radix(&raw_str, radix_qualifier, lit_span)?;
        let (digits, natural_scale) = parse_value_token(&raw_str, lit_span, radix)?;
        Ok(Invocation::Literal(LiteralForm {
            width,
            digits,
            sign,
            natural_scale,
            scale_qualifier,
            rounded,
            radix_literal: radix != 10,
            value_span,
        }))
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
        let _ = rounded;
        Ok(Invocation::Expression {
            width,
            expr: value_expr,
            scale,
            scale_span,
        })
    }
}

/// Split a token stream on top-level commas (commas inside
/// brackets / parens / braces stay with their content). Returns
/// a vector of segments; each segment is a `Vec<TokenTree>`.
fn split_top_commas(tokens: TokenStream2) -> Vec<Vec<TokenTree>> {
    let mut out: Vec<Vec<TokenTree>> = vec![Vec::new()];
    for tt in tokens {
        match &tt {
            TokenTree::Punct(p)
                if p.as_char() == ',' && p.spacing() == proc_macro2::Spacing::Alone =>
            {
                out.push(Vec::new());
            }
            _ => out.last_mut().unwrap().push(tt),
        }
    }
    out
}

/// If the value segment looks like a radix-fractional literal
/// (possibly sign-prefixed `INT . IDENT-or-INT`) *and* an explicit
/// non-decimal radix was requested, return the parsed
/// `(digit-string, sign, natural_scale, span)`. Returns `Ok(None)`
/// to defer to the standard Rust-Expr parser when the shape
/// doesn't match.
fn try_radix_fractional(
    segment: &[TokenTree],
    explicit_radix: Option<(u32, Span)>,
) -> Result<Option<(String, i128, u32, Span)>> {
    let Some((radix, _radix_span)) = explicit_radix else {
        return Ok(None);
    };
    if radix == 10 {
        return Ok(None);
    }

    let mut i = 0;
    let mut sign: i128 = 1;
    if let Some(TokenTree::Punct(p)) = segment.first() {
        if p.as_char() == '-' {
            sign = -1;
            i += 1;
        } else if p.as_char() == '+' {
            i += 1;
        }
    }

    // After the optional sign, we accept any of:
    //   single Float literal — e.g. `11.0110` (Rust tokenises this
    //       as one Float token; we split on the embedded `.`)
    //   `INT . IDENT` — e.g. `1.A3` in radix 16 (Rust can't lex
    //       this as one token, so it arrives as three)
    //   `INT . INT`   — same situation, e.g. `1.10` where the
    //       fractional part happens to be digit-only
    //   single INT    — pure integer in the given radix
    let int_tok = segment.get(i);
    let dot_tok = segment.get(i + 1);
    let frac_tok = segment.get(i + 2);
    let extra = segment.get(i + 3);

    let int_lit = match int_tok {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        Some(TokenTree::Ident(id)) => id.to_string(),
        _ => return Ok(None),
    };

    let (int_part, frac_part, span) = if int_lit.contains('.') && dot_tok.is_none() {
        // Single literal that already contains the dot (`11.0110`).
        let span = match int_tok.unwrap() {
            TokenTree::Literal(lit) => lit.span(),
            _ => Span::call_site(),
        };
        let (head, tail) = int_lit.split_once('.').unwrap();
        (head.to_string(), tail.to_string(), span)
    } else {
        match (dot_tok, frac_tok, extra) {
            (Some(TokenTree::Punct(p)), Some(frac), None) if p.as_char() == '.' => {
                let frac_str = match frac {
                    TokenTree::Literal(lit) => lit.to_string(),
                    TokenTree::Ident(id) => id.to_string(),
                    _ => return Ok(None),
                };
                let span = match int_tok.unwrap() {
                    TokenTree::Literal(lit) => lit.span(),
                    TokenTree::Ident(id) => id.span(),
                    _ => Span::call_site(),
                };
                (int_lit, frac_str, span)
            }
            (None, None, _) => {
                let span = match int_tok.unwrap() {
                    TokenTree::Literal(lit) => lit.span(),
                    TokenTree::Ident(id) => id.span(),
                    _ => Span::call_site(),
                };
                (int_lit, String::new(), span)
            }
            _ => return Ok(None),
        }
    };

    // Strip a Rust integer prefix (`0x`, `0o`, `0b`) on the
    // integer part — it must match the explicit radix or it's an
    // error. The fractional part doesn't carry a prefix.
    let cleaned_int = if let Some((prefix_r, rest)) = strip_radix_prefix(&int_part) {
        if prefix_r != radix {
            return Err(syn::Error::new(
                span,
                format!(
                    "radix qualifier ({radix}) disagrees with integer-part prefix (radix {prefix_r})"
                ),
            ));
        }
        rest.to_string()
    } else {
        int_part
    };

    let int_cleaned: String = cleaned_int.chars().filter(|c| *c != '_').collect();
    let frac_cleaned: String = frac_part.chars().filter(|c| *c != '_').collect();

    if int_cleaned.is_empty() {
        return Err(syn::Error::new(
            span,
            "decimal literals require a digit on each side of the dot (write `0.A3` not `.A3`)",
        ));
    }
    for c in int_cleaned.chars().chain(frac_cleaned.chars()) {
        if !is_radix_digit(c, radix) {
            return Err(syn::Error::new(
                span,
                format!("digit `{c}` not valid for radix {radix}"),
            ));
        }
    }

    // Concatenate int + frac digits, parse as a magnitude in `radix`.
    // `natural_scale` is the number of fractional digits in the
    // source — the user must still supply `, scale N` because the
    // bits at the source's natural scale are not normally what they
    // want as storage bits anyway.
    let combined = format!("{int_cleaned}{frac_cleaned}");
    let magnitude = match i128::from_str_radix(&combined, radix) {
        Ok(v) => v,
        Err(_) => {
            return Err(syn::Error::new(
                span,
                format!("digit string `{combined}` overflows i128 when parsed in radix {radix}"),
            ));
        }
    };
    Ok(Some((
        magnitude.to_string(),
        sign,
        frac_cleaned.len() as u32,
        span,
    )))
}

fn is_radix_digit(c: char, radix: u32) -> bool {
    c.is_digit(radix)
}

/// Parsed qualifier triple: `(scale N, radix N, rounded)`, where each
/// `Option<(u32, Span)>` carries the value and its source span for
/// diagnostics.
type ParsedQualifiers = (Option<(u32, Span)>, Option<(u32, Span)>, bool);

/// Re-parse the qualifier segments to find `scale N` / `radix N` /
/// `rounded`. Equivalent to the old `parse_qualifiers` but works on
/// token-vec segments instead of a `ParseStream`.
fn parse_qualifier_segments(
    segments: &[Vec<TokenTree>],
    width: Width,
) -> Result<ParsedQualifiers> {
    let _ = width;
    let mut scale: Option<(u32, Span)> = None;
    let mut radix: Option<(u32, Span)> = None;
    let mut rounded = false;
    for seg in segments {
        if seg.is_empty() {
            continue;
        }
        let TokenTree::Ident(kw) = &seg[0] else {
            return Err(syn::Error::new(
                tt_span(&seg[0]),
                "expected qualifier identifier (scale | radix | rounded)",
            ));
        };
        match kw.to_string().as_str() {
            "scale" => {
                let lit = seg.get(1).and_then(|t| match t {
                    TokenTree::Literal(l) => Some(l),
                    _ => None,
                });
                let lit = lit.ok_or_else(|| {
                    syn::Error::new(kw.span(), "`scale` requires an integer literal: `scale N`")
                })?;
                let n: u32 = lit.to_string().parse().map_err(|_| {
                    syn::Error::new(lit.span(), "scale must be a non-negative integer")
                })?;
                if scale.is_some() {
                    return Err(syn::Error::new(kw.span(), "duplicate `scale` qualifier"));
                }
                scale = Some((n, lit.span()));
            }
            "radix" => {
                let lit = seg.get(1).and_then(|t| match t {
                    TokenTree::Literal(l) => Some(l),
                    _ => None,
                });
                let lit = lit.ok_or_else(|| {
                    syn::Error::new(kw.span(), "`radix` requires an integer literal: `radix N`")
                })?;
                let r: u32 = lit.to_string().parse().map_err(|_| {
                    syn::Error::new(lit.span(), "radix must be one of 2, 8, 10, 16")
                })?;
                if !matches!(r, 2 | 8 | 10 | 16) {
                    return Err(syn::Error::new(
                        lit.span(),
                        format!("radix must be one of 2, 8, 10, 16 (got {r})"),
                    ));
                }
                if radix.is_some() {
                    return Err(syn::Error::new(kw.span(), "duplicate `radix` qualifier"));
                }
                radix = Some((r, lit.span()));
            }
            "rounded" => {
                if rounded {
                    return Err(syn::Error::new(kw.span(), "duplicate `rounded` qualifier"));
                }
                if seg.len() > 1 {
                    return Err(syn::Error::new(
                        tt_span(&seg[1]),
                        "`rounded` takes no argument",
                    ));
                }
                rounded = true;
            }
            other => {
                return Err(syn::Error::new(
                    kw.span(),
                    format!("unknown qualifier `{other}`; expected one of: scale, radix, rounded"),
                ));
            }
        }
    }
    Ok((scale, radix, rounded))
}

fn tt_span(tt: &TokenTree) -> Span {
    match tt {
        TokenTree::Group(g) => g.span(),
        TokenTree::Ident(i) => i.span(),
        TokenTree::Punct(p) => p.span(),
        TokenTree::Literal(l) => l.span(),
    }
}

// ── Invocation model ──────────────────────────────────────────────────

/// The fully-parsed payload of a literal-form invocation. Grouped into
/// one struct so it travels as a single argument (rather than the eight
/// positional parameters the codegen would otherwise take).
struct LiteralForm {
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
}

enum Invocation {
    Literal(LiteralForm),
    Expression {
        width: Width,
        expr: Expr,
        scale: u32,
        scale_span: Span,
    },
}

impl Invocation {
    fn expand(self) -> TokenStream {
        match self {
            Invocation::Literal(form) => expand_literal(form),
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

/// Resolve the effective radix for a literal. Reconciles an explicit
/// `radix N` qualifier with a Rust prefix (`0x`, `0o`, `0b`); reports
/// a conflict if the two disagree.
fn pick_radix(raw: &str, qualifier: Option<(u32, Span)>, _span: Span) -> Result<u32> {
    let prefix_radix = [("0x", "0X", 16u32), ("0o", "0O", 8), ("0b", "0B", 2)]
        .into_iter()
        .find_map(|(lower, upper, radix)| {
            raw.strip_prefix(lower)
                .or_else(|| raw.strip_prefix(upper))
                .map(|stripped| (radix, stripped))
        });
    match (prefix_radix, qualifier) {
        (None, None) => Ok(10),
        (None, Some((r, _))) => Ok(r),
        (Some((p, _)), None) => Ok(p),
        (Some((p, _)), Some((r, _sp))) if p == r => Ok(r),
        (Some((p, _)), Some((r, sp))) => Err(syn::Error::new(
            sp,
            format!("radix qualifier ({r}) disagrees with literal prefix (radix {p})"),
        )),
    }
}

// ── Literal-form codegen ─────────────────────────────────────────────

fn expand_literal(form: LiteralForm) -> TokenStream {
    let LiteralForm {
        width,
        digits,
        sign,
        natural_scale,
        scale_qualifier,
        rounded,
        radix_literal,
        value_span,
    } = form;
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
            shifted_digits = if kept.is_empty() {
                "0".to_string()
            } else {
                kept.to_string()
            };
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
                format!(
                    "scaled value overflows i128 before narrowing to {}'s storage",
                    width.name.to_uppercase()
                ),
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
    let tp = type_path(width);
    let out = quote! {
        #tp :: <#target_scale> :: from_bits(#bits_tokens)
    };
    out.into()
}

fn emit_wide(width: Width, target_scale: u32, sign: i128, digits: &str) -> TokenStream {
    let signed_str = if sign < 0 {
        format!("-{digits}")
    } else {
        digits.to_string()
    };
    let tp = type_path(width);
    let sp = storage_path_tokens(width);
    let err_msg = format!("{}! bits parse failed", width.name);
    let out = quote! {
        #tp :: <#target_scale> :: from_bits({
            const BITS: #sp = match <#sp>::from_str_radix(#signed_str, 10) {
                ::core::result::Result::Ok(v) => v,
                ::core::result::Result::Err(_) => panic!(#err_msg),
            };
            BITS
        })
    };
    out.into()
}

// ── Expression-form codegen ───────────────────────────────────────────

fn expand_expression(width: Width, expr: Expr, scale: u32, scale_span: Span) -> TokenStream {
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
    let tp = type_path(width);
    let sp = storage_path_tokens(width);
    let err_msg = format!(
        "{}! overflow: expression * 10^SCALE exceeds storage range",
        width.name
    );
    let out = if width.wide && (width.storage_path == "Int::<2>" || width.storage_path == "Int::<1>") {
        // D38 / D18: the storage is `Int<2>` / `Int<1>`, but an expression
        // value is naturally an `i128` / `i64`-valued expression (as it was
        // when these stored `i128` / `i64`). Bridge it to the storage type so
        // callers keep the ergonomic `dNN!(some_int_expr, scale N)` form.
        let bridged = if width.storage_path == "Int::<1>" {
            quote! { <#sp as ::core::convert::From<i64>>::from((#expr) as i64) }
        } else {
            quote! { <#sp as ::core::convert::TryFrom<i128>>::try_from((#expr) as i128).unwrap() }
        };
        quote! {
            #tp :: <#scale> :: from_bits({
                let _v: #sp = #bridged;
                let mult: #sp = <#sp>::from_str_radix("10", 10)
                    .expect("dNN! mult literal")
                    .pow(#scale);
                _v.checked_mul(mult).expect(#err_msg)
            })
        }
    } else if width.wide {
        quote! {
            #tp :: <#scale> :: from_bits({
                let _v: #sp = (#expr);
                let mult: #sp = <#sp>::from_str_radix("10", 10)
                    .expect("d{}! mult literal")
                    .pow(#scale);
                _v.checked_mul(mult).expect(#err_msg)
            })
        }
    } else if scale == 0 {
        quote! {
            #tp :: <0> :: from_bits({
                let _v: #sp = (#expr);
                _v
            })
        }
    } else {
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
            #tp :: <#scale> :: from_bits({
                let _v: #sp = (#expr);
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
fn parse_value_token(raw: &str, span: Span, radix: u32) -> Result<(String, u32)> {
    // Reject Rust type suffixes (1.5_f64 etc.).
    for (i, c) in raw.char_indices() {
        if (c == 'i' || c == 'u')
            || (c == 'f'
                && i > 0
                && !raw[..i].contains('.')
                && !raw[..i]
                    .chars()
                    .last()
                    .is_some_and(|x| x.is_ascii_digit()))
        {
            // No-op: we'll handle the `f`/`i`/`u` filter via parse failures.
            let _ = i;
        }
    }
    if let Some(idx) = raw.find('f') {
        // `1_f64`-style suffix.
        if idx > 0 && raw.as_bytes()[idx - 1] == b'_' {
            return Err(syn::Error::new(
                span,
                "type suffixes (e.g. _i64, _f32) are not accepted in decimal-scaled literals",
            ));
        }
    }
    if let Some(idx) = raw.rfind(['i', 'u']) {
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
    let digits_part = strip_radix_prefix(raw)
        .map(|(p, rest)| {
            // p must match radix; if not, the caller's pick_radix already
            // flagged it.
            let _ = p;
            rest
        })
        .unwrap_or(raw);

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
                format!("digit string `{cleaned}` is not valid in radix {radix} or overflows i128"),
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
    let digits = if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    };

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
