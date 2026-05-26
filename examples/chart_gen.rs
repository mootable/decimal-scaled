//! Generate the `docs/figures/library_comparison/<op>_<width>.png`
//! layered line charts from `target/medians.tsv`.
//!
//! Input: TSV with `source<TAB>id<TAB>median<TAB>unit` lines. The
//! library-comparison bench emits ids of the form
//!   lib_cmp/<width>_s<scale>/<lib>/<op>
//! e.g. `lib_cmp/128bit_s19/decimal-scaled/mul`.
//!
//! Output: one PNG per (op × width). 6+ lines per chart, one per
//! library, with `decimal-scaled` always drawn last (top of z-stack)
//! in red + 3-px stroke so it's the focal line.
//!
//! Run with:
//!     cargo run --release --example chart_gen --features wide,x-wide

use plotters::prelude::*;
use std::collections::BTreeMap;
use std::fs;

type LibPoints = Vec<(u32, f64)>; // (scale, ns)
type ChartKey = (String, String); // (op, width)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Subcommand routing — the default (no args) runs the
    // library-comparison + per-width-summary pipeline; `history`
    // renders the cross-version trend charts from
    // tmp/bench-history-results/.
    let mode = std::env::args().nth(1).unwrap_or_default();
    if mode == "history" {
        return render_history();
    }

    let tsv = fs::read_to_string("target/medians.tsv")?;

    // (op, width) -> lib -> [(scale, ns)]
    let mut data: BTreeMap<ChartKey, BTreeMap<String, LibPoints>> = BTreeMap::new();

    for line in tsv.lines() {
        let cols: Vec<_> = line.split('\t').collect();
        if cols.len() != 4 {
            continue;
        }
        let id = cols[1];
        if !id.starts_with("lib_cmp/") {
            continue;
        }
        let parts: Vec<_> = id.split('/').collect();
        if parts.len() != 4 {
            continue;
        }
        let width_scale = parts[1];
        let lib = parts[2].to_string();
        let op = parts[3].to_string();
        let Some((width, scale_str)) = width_scale.split_once("_s") else {
            continue;
        };
        let Ok(scale) = scale_str.parse::<u32>() else {
            continue;
        };
        let Ok(ns) = to_ns(cols[2], cols[3]) else {
            continue;
        };
        data.entry((op.clone(), width.to_string()))
            .or_default()
            .entry(lib)
            .or_default()
            .push((scale, ns));
    }

    let out_dir = "docs/figures/library_comparison";
    fs::create_dir_all(out_dir)?;

    // Render every (op × width) combination found in the input. The
    // benchmarks.md §5 narrative only embeds the meaningful subset,
    // but the full set lives in docs/figures/ for anyone wanting to
    // verify the scale-invariant ops (add / sub / neg) really are
    // flat.
    let mut count = 0;
    let mut skipped = 0;
    for ((op, width), by_lib) in &data {
        // Render only when there's a real multi-library comparison
        // worth plotting: at least 2 libraries must have ≥2 scale
        // points each. This filters two degenerate cases:
        //
        //   * No library has ≥2 points (single-dot scatter chart).
        //   * Only one library (typically `decimal-scaled`) has a
        //     line while every peer has a single dot — a chart that
        //     looks like our line floating above isolated dots
        //     overstates the comparison.
        let lines = by_lib.values().filter(|points| points.len() >= 2).count();
        if lines < 2 {
            skipped += 1;
            continue;
        }
        let path = format!("{out_dir}/{op}_{width}.png");
        render_chart(&path, op, width, by_lib)?;
        count += 1;
    }
    println!(
        "wrote {count} charts to {out_dir}/ (skipped {skipped} charts \
         that would be single-line or no-line)"
    );

    // ---- New format: one summary chart per (width × centre scale).
    // x-axis = operation (add / sub / mul / div / rem / neg / ln / exp /
    // sin / sqrt), y-axis = time (log ns), one bar per library per op.
    render_per_width_summaries(&data, out_dir)?;

    Ok(())
}

/// Centre scale to feature per power-of-two storage width. Matches
/// the docs/benchmarks.md §5 subsections.
const CENTRE_SCALES: &[(&str, u32)] = &[
    ("128bit", 19),
    ("256bit", 35),
    ("512bit", 75),
    ("1024bit", 150),
    ("2048bit", 308),
    ("4096bit", 616),
];

const OP_ORDER: &[&str] = &[
    "add", "sub", "neg", "mul", "div", "rem", "sqrt", "ln", "exp", "sin", "cos", "tan", "atan",
    "sinh", "cosh", "tanh",
];

/// Render one chart per CENTRE_SCALES entry — a grouped bar chart with
/// one bar per (operation × library) cell at that width's centre scale.
fn render_per_width_summaries(
    data: &BTreeMap<ChartKey, BTreeMap<String, LibPoints>>,
    out_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Pivot: (width, op, lib) -> ns at the centre scale.
    let mut summary: BTreeMap<&str, BTreeMap<&str, BTreeMap<String, f64>>> = BTreeMap::new();

    for ((op, width), by_lib) in data {
        let Some(&(_, centre)) = CENTRE_SCALES.iter().find(|(w, _)| *w == width) else {
            continue;
        };
        if !OP_ORDER.contains(&op.as_str()) {
            continue;
        }
        for (lib, points) in by_lib {
            // Find the centre-scale point; if none, try the median of
            // whatever this library has at this (op × width). Most
            // libraries are only benched at s = mid, so they have
            // exactly one point — use it as-is.
            let chosen = points
                .iter()
                .find(|(s, _)| *s == centre)
                .or_else(|| {
                    points
                        .iter()
                        .min_by_key(|(s, _)| (*s as i64 - centre as i64).abs())
                })
                .copied();
            if let Some((_, ns)) = chosen {
                summary
                    .entry(width.as_str())
                    .or_default()
                    .entry(op.as_str())
                    .or_default()
                    .insert(lib.clone(), ns);
            }
        }
    }

    for (width, _centre) in CENTRE_SCALES {
        let Some(per_op) = summary.get(width) else {
            continue;
        };
        let path = format!("{out_dir}/summary_{width}.png");
        render_per_width_summary(&path, width, per_op)?;
    }
    Ok(())
}

fn render_per_width_summary(
    path: &str,
    width: &str,
    per_op: &BTreeMap<&str, BTreeMap<String, f64>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Drop ops that have no data at this width.
    let ops: Vec<&&str> = OP_ORDER
        .iter()
        .filter(|o| per_op.contains_key(**o))
        .collect();
    if ops.is_empty() {
        return Ok(());
    }
    // Union of libraries appearing anywhere in this width.
    let mut libs: Vec<String> = per_op.values().flat_map(|m| m.keys().cloned()).collect();
    libs.sort();
    libs.dedup();
    // Put decimal-scaled last so it draws on top.
    libs.sort_by_key(|l| l == "decimal-scaled");
    if libs.is_empty() {
        return Ok(());
    }

    let backend = BitMapBackend::new(path, (1100, 600)).into_drawing_area();
    backend.fill(&WHITE)?;

    let y_min = per_op
        .values()
        .flat_map(|m| m.values().copied())
        .fold(f64::INFINITY, f64::min)
        .max(0.01);
    let y_max = per_op
        .values()
        .flat_map(|m| m.values().copied())
        .fold(0.0_f64, f64::max);
    let y_floor = (y_min * 0.5).max(0.01);
    let y_ceil = y_max * 2.0;

    let title = format!("operations @ {width} (centre scale)");
    let n_ops = ops.len();
    let n_libs = libs.len();
    // x layout: each op group is centred on its integer x (0, 1, 2,
    // ...). Bars within a group span [-group_half, +group_half] from
    // that centre so the x-axis label at integer x sits directly
    // under the bar group.
    let group_total: f64 = 0.8; // width of all bars stacked, leaving 0.2 between groups
    let bar_width: f64 = group_total / n_libs as f64;
    let group_half: f64 = group_total / 2.0;
    let x_min: f64 = -0.5;
    let x_max: f64 = n_ops as f64 - 0.5;

    let mut chart = ChartBuilder::on(&backend)
        .caption(&title, ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(55)
        .y_label_area_size(75)
        .right_y_label_area_size(20)
        .build_cartesian_2d(x_min..x_max, (y_floor..y_ceil).log_scale())?;

    let op_labels: Vec<String> = ops.iter().map(|o| (*o).to_string()).collect();
    chart
        .configure_mesh()
        .x_desc("operation")
        .y_desc("time (ns, log)")
        // n_ops + 1 label positions to ensure plotters lands a tick at
        // each integer 0..n_ops-1 rather than skipping alternates for
        // spacing reasons.
        .x_labels(n_ops + 1)
        .x_label_formatter(&|x| {
            // Plotters hands us float positions near the integer ticks
            // it chose; round to the nearest integer and look up.
            let i = (x.round()) as i64;
            if (0..n_ops as i64).contains(&i) {
                op_labels[i as usize].clone()
            } else {
                String::new()
            }
        })
        // Suppress the auto x-axis gridlines that would otherwise draw
        // a vertical through the centre of each bar group. We'll draw
        // our own separators at the half-integer slot boundaries
        // below.
        .x_max_light_lines(0)
        .disable_x_mesh()
        .draw()?;

    let palette: &[(RGBColor, &str)] = &[
        (RGBColor(31, 119, 180), "rust_decimal"),
        (RGBColor(255, 127, 14), "fastnum"),
        (RGBColor(44, 160, 44), "bigdecimal"),
        (RGBColor(148, 103, 189), "dashu-float"),
        (RGBColor(140, 86, 75), "decimal-rs"),
        (RGBColor(227, 119, 194), "g_math"),
        (RGBColor(127, 127, 127), "fixed_i64f64"),
        (RGBColor(127, 127, 127), "fixed_i16f16"),
        (RGBColor(127, 127, 127), "fixed_i32f32"),
        (RGBColor(127, 127, 127), "fixed_i128f128"),
        (RED, "decimal-scaled"),
    ];
    let color_for = |lib: &str| -> RGBColor {
        palette
            .iter()
            .find(|(_, name)| *name == lib)
            .map(|(c, _)| *c)
            .unwrap_or(RGBColor(100, 100, 100))
    };

    for (li, lib) in libs.iter().enumerate() {
        let color = color_for(lib);
        let bars: Vec<_> = ops
            .iter()
            .enumerate()
            .filter_map(|(oi, op)| {
                per_op.get(**op).and_then(|m| m.get(lib)).map(|&ns| {
                    let centre = oi as f64;
                    // Each lib's bar is the li-th slot of n_libs,
                    // packed [-group_half, +group_half] around centre.
                    let left = centre - group_half + li as f64 * bar_width;
                    let right = left + bar_width;
                    Rectangle::new([(left, y_floor), (right, ns)], color.filled())
                })
            })
            .collect();
        let label = lib.clone();
        chart
            .draw_series(bars)?
            .label(label)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 12, y + 5)], color.filled()));
    }

    // Major x separators at the boundaries between operation groups
    // (x = 0.5, 1.5, ..., n_ops - 1.5). Drawn AFTER the bars so they
    // sit on top instead of being overlaid. Read as "this region
    // belongs to op N" — the prior auto-gridline bisected each group.
    let separator_style = ShapeStyle {
        color: RGBColor(120, 120, 120).to_rgba(),
        filled: false,
        stroke_width: 2,
    };
    for i in 1..n_ops {
        let x = i as f64 - 0.5;
        chart.draw_series(LineSeries::new(
            [(x, y_floor), (x, y_ceil)],
            separator_style,
        ))?;
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.9))
        .draw()?;

    backend.present()?;
    Ok(())
}

// ------------------------------------------------------------------
// History mode — read tmp/bench-history-results/ and render per-width
// cross-version trend lines at the fixed reference scale.
//
// Layout on disk (rooted at tmp/bench-history-results/):
//
//   bench-history-<tag>/<op>_<width>_s<scale>/t/new/estimates.json
//
// where:
//   <tag>   ∈ { v0.2.5, v0.3.2, v0.3.3, v0.4.0, v0.4.2, v0.4.3, v0.4.4, HEAD }
//   <op>    ∈ { add, mul, div, sqrt, ln, sin }
//   <width> ∈ { D38, D76, D307 }
//   <scale> ∈ the harness scale set; the PNG trend uses the reference scale.
//
// The harness fans out over (width × scale); the cross-version PNG trend
// plots ONE scale (the fixed reference, `HISTORY_CHART_SCALE`) so the line
// chart stays one-line-per-function. The full (version × scale) surface is
// rendered as a table by `summarise_history.py` in the aggregate job.
//
// HEAD is the current dev source (main; 0.5.0 in development). A version
// that lacks a function leaves a gap in that function's line (no point
// plotted) rather than a fabricated value.
// ------------------------------------------------------------------

const HISTORY_VERSIONS: &[(&str, &str)] = &[
    ("v0.2.5", "v0.2.5"),
    ("v0.3.2", "v0.3.2"),
    ("v0.3.3", "v0.3.3"),
    ("v0.4.0", "v0.4.0"),
    ("v0.4.2", "v0.4.2"),
    ("v0.4.3", "v0.4.3"),
    ("v0.4.4", "v0.4.4"),
    ("HEAD", "main"),
];

// op directory stem -> chart line label (now identical; kept as a pair so
// the renderer's label vs path distinction stays explicit).
const HISTORY_GROUPS: &[(&str, &str)] = &[
    ("add", "add"),
    ("mul", "mul"),
    ("div", "div"),
    ("sqrt", "sqrt"),
    ("ln", "ln"),
    ("sin", "sin"),
];

const HISTORY_WIDTHS: &[&str] = &["D38", "D76", "D307"];

// The fixed reference scale plotted in the cross-version line charts (the
// harness scale set is {0, 10, 30}; 30 is the continuity reference point).
const HISTORY_CHART_SCALE: usize = 30;

fn render_history() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;

    // (width, fn_label) -> Vec<(version_label, ns)>
    let mut data: BTreeMap<(String, String), Vec<(String, f64)>> = BTreeMap::new();

    for (tag, vlabel) in HISTORY_VERSIONS {
        for (op, fn_label) in HISTORY_GROUPS {
            for width in HISTORY_WIDTHS {
                let p = format!(
                    "tmp/bench-history-results/bench-history-{tag}/{op}_{width}_s{HISTORY_CHART_SCALE}/t/new/estimates.json",
                );
                if !Path::new(&p).exists() {
                    eprintln!("history: missing {p}");
                    continue;
                }
                let raw = fs::read_to_string(&p)?;
                let parsed: serde_json::Value = serde_json::from_str(&raw)?;
                let ns = parsed["mean"]["point_estimate"]
                    .as_f64()
                    .ok_or_else(|| format!("no mean in {p}"))?;
                data.entry(((*width).to_string(), (*fn_label).to_string()))
                    .or_default()
                    .push(((*vlabel).to_string(), ns));
            }
        }
    }

    let out_dir = "docs/figures/history";
    fs::create_dir_all(out_dir)?;

    for width in HISTORY_WIDTHS {
        let path = format!("{out_dir}/{}.png", width.to_lowercase());
        render_history_chart(&path, width, &data)?;
    }
    println!(
        "wrote {} history charts to {out_dir}/",
        HISTORY_WIDTHS.len()
    );
    Ok(())
}

fn render_history_chart(
    path: &str,
    width: &str,
    data: &BTreeMap<(String, String), Vec<(String, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new(path, (900, 540)).into_drawing_area();
    backend.fill(&WHITE)?;

    // Collect rows for this width.
    let fn_order: &[&str] = &["add", "mul", "div", "sqrt", "ln", "sin"];
    let mut rows: Vec<(&str, &Vec<(String, f64)>)> = Vec::new();
    for fn_label in fn_order {
        if let Some(points) = data.get(&(width.to_string(), (*fn_label).to_string())) {
            rows.push((*fn_label, points));
        }
    }
    if rows.is_empty() {
        return Ok(());
    }

    let n_versions = HISTORY_VERSIONS.len();
    let x_min = 0.0_f64;
    let x_max = (n_versions - 1) as f64;
    let y_min = rows
        .iter()
        .flat_map(|(_, p)| p.iter().map(|(_, n)| *n))
        .fold(f64::INFINITY, f64::min)
        .max(0.01);
    let y_max = rows
        .iter()
        .flat_map(|(_, p)| p.iter().map(|(_, n)| *n))
        .fold(0.0_f64, f64::max);
    let y_floor = (y_min * 0.5).max(0.01);
    let y_ceil = y_max * 2.0;

    let title = format!("history — {width}, cross-version improvement");
    let mut chart = ChartBuilder::on(&backend)
        .caption(&title, ("sans-serif", 26))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(75)
        .right_y_label_area_size(20)
        .build_cartesian_2d(x_min..x_max, (y_floor..y_ceil).log_scale())?;

    let version_labels: Vec<String> = HISTORY_VERSIONS
        .iter()
        .map(|(_, v)| (*v).to_string())
        .collect();

    chart
        .configure_mesh()
        .x_desc("version")
        .y_desc("time (ns, log)")
        .x_labels(n_versions)
        .x_label_formatter(&|x| {
            let i = (x.round()) as i64;
            if (0..n_versions as i64).contains(&i) {
                version_labels[i as usize].clone()
            } else {
                String::new()
            }
        })
        .x_max_light_lines(0)
        .disable_x_mesh()
        .draw()?;

    // One distinct colour per function, kept consistent across the
    // three width charts so a reader can scan vertically.
    let color_for = |fn_label: &str| -> RGBColor {
        match fn_label {
            "add" => RGBColor(31, 119, 180),
            "mul" => RGBColor(255, 127, 14),
            "div" => RGBColor(44, 160, 44),
            "sqrt" => RGBColor(148, 103, 189),
            "ln" => RGBColor(214, 39, 40),
            "sin" => RGBColor(140, 86, 75),
            _ => RGBColor(127, 127, 127),
        }
    };

    for (fn_label, points) in &rows {
        let color = color_for(fn_label);
        // Map version label -> x index; skip missing versions so the
        // line just spans the cells we have.
        let xy: Vec<(f64, f64)> = points
            .iter()
            .filter_map(|(vlabel, ns)| {
                HISTORY_VERSIONS
                    .iter()
                    .position(|(_, v)| *v == vlabel.as_str())
                    .map(|i| (i as f64, *ns))
            })
            .collect();
        if xy.is_empty() {
            continue;
        }
        let label = (*fn_label).to_string();
        let xy_for_points = xy.clone();
        chart
            .draw_series(LineSeries::new(xy, color.stroke_width(3)))?
            .label(label)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(3))
            });
        chart.draw_series(
            xy_for_points
                .into_iter()
                .map(|(x, y)| Circle::new((x, y), 5, color.filled())),
        )?;
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.9))
        .draw()?;

    backend.present()?;
    Ok(())
}

fn to_ns(median: &str, unit: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let v: f64 = median.parse()?;
    let mul = match unit {
        "ps" => 1e-3,
        "ns" => 1.0,
        "µs" | "us" => 1e3,
        "ms" => 1e6,
        "s" => 1e9,
        other => return Err(format!("unknown unit {other}").into()),
    };
    Ok(v * mul)
}

fn render_chart(
    path: &str,
    op: &str,
    width: &str,
    by_lib: &BTreeMap<String, LibPoints>,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new(path, (900, 540)).into_drawing_area();
    backend.fill(&WHITE)?;

    // Sort each library's points by scale for clean line draw.
    let mut sorted: BTreeMap<String, LibPoints> = by_lib.clone();
    for v in sorted.values_mut() {
        v.sort_by_key(|&(s, _)| s);
    }

    let all_points: Vec<&(u32, f64)> = sorted.values().flatten().collect();
    if all_points.is_empty() {
        return Ok(());
    }

    let x_min = all_points.iter().map(|p| p.0).min().unwrap() as f64;
    let x_max = all_points.iter().map(|p| p.0).max().unwrap().max(1) as f64;
    let y_min = all_points
        .iter()
        .map(|p| p.1)
        .fold(f64::INFINITY, f64::min)
        .max(0.01);
    let y_max = all_points.iter().map(|p| p.1).fold(0.0_f64, f64::max);

    let y_floor = y_min * 0.5;
    let y_ceil = y_max * 2.0;

    let title = format!("{op} @ {width}");
    let mut chart = ChartBuilder::on(&backend)
        .caption(&title, ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(45)
        .y_label_area_size(70)
        .right_y_label_area_size(20)
        .build_cartesian_2d(x_min.max(0.0)..x_max + 1.0, (y_floor..y_ceil).log_scale())?;

    chart
        .configure_mesh()
        .x_desc("SCALE")
        .y_desc("time (ns, log)")
        .x_label_formatter(&|x| format!("{}", *x as u32))
        .draw()?;

    let color_for = |lib: &str| -> RGBColor {
        match lib {
            "rust_decimal" => RGBColor(31, 119, 180),
            "fastnum" => RGBColor(255, 127, 14),
            "bigdecimal" => RGBColor(44, 160, 44),
            "dashu-float" => RGBColor(148, 103, 189),
            "decimal-rs" => RGBColor(140, 86, 75),
            "g_math" => RGBColor(227, 119, 194),
            lib if lib.starts_with("fixed_") => RGBColor(127, 127, 127),
            _ => RGBColor(20, 20, 20),
        }
    };

    // Draw all non-decimal-scaled lines first
    for (lib, points) in sorted
        .iter()
        .filter(|(l, _)| l.as_str() != "decimal-scaled")
    {
        let color = color_for(lib);
        let label = lib.clone();
        if points.len() >= 2 {
            chart
                .draw_series(LineSeries::new(
                    points.iter().map(|(s, n)| (*s as f64, *n)),
                    color.stroke_width(2),
                ))?
                .label(label)
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2))
                });
        } else if let Some(&(s, n)) = points.first() {
            chart
                .draw_series(std::iter::once(Circle::new(
                    (s as f64, n),
                    5,
                    color.filled(),
                )))?
                .label(label)
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(2))
                });
        }
    }

    // decimal-scaled on top, red, thicker
    if let Some(points) = sorted.get("decimal-scaled") {
        if points.len() >= 2 {
            chart
                .draw_series(LineSeries::new(
                    points.iter().map(|(s, n)| (*s as f64, *n)),
                    RED.stroke_width(3),
                ))?
                .label("decimal-scaled")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(3)));
        } else if let Some(&(s, n)) = points.first() {
            chart
                .draw_series(std::iter::once(Circle::new((s as f64, n), 6, RED.filled())))?
                .label("decimal-scaled")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(3)));
        }
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(BLACK)
        .background_style(WHITE.mix(0.85))
        .draw()?;

    backend.present()?;
    Ok(())
}
