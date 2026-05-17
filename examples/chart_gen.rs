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
    ("128bit",  19),
    ("256bit",  35),
    ("512bit",  75),
    ("1024bit", 150),
    ("2048bit", 308),
    ("4096bit", 616),
];

const OP_ORDER: &[&str] = &[
    "add", "sub", "neg", "mul", "div", "rem", "sqrt", "ln", "exp", "sin",
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
        if !OP_ORDER.iter().any(|o| *o == op.as_str()) {
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
                .or_else(|| points.iter().min_by_key(|(s, _)| (*s as i64 - centre as i64).abs()))
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
        let Some(per_op) = summary.get(width) else { continue; };
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
    let ops: Vec<&&str> = OP_ORDER.iter().filter(|o| per_op.contains_key(**o)).collect();
    if ops.is_empty() {
        return Ok(());
    }
    // Union of libraries appearing anywhere in this width.
    let mut libs: Vec<String> = per_op
        .values()
        .flat_map(|m| m.keys().cloned())
        .collect();
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
        .draw()?;

    let palette: &[(RGBColor, &str)] = &[
        (RGBColor(31, 119, 180),  "rust_decimal"),
        (RGBColor(255, 127, 14),  "fastnum"),
        (RGBColor(44, 160, 44),   "bigdecimal"),
        (RGBColor(148, 103, 189), "dashu-float"),
        (RGBColor(140, 86, 75),   "decimal-rs"),
        (RGBColor(227, 119, 194), "g_math"),
        (RGBColor(127, 127, 127), "fixed_i64f64"),
        (RGBColor(127, 127, 127), "fixed_i16f16"),
        (RGBColor(127, 127, 127), "fixed_i32f32"),
        (RGBColor(127, 127, 127), "fixed_i128f128"),
        (RED,                     "decimal-scaled"),
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
                    Rectangle::new(
                        [(left, y_floor), (right, ns)],
                        color.filled(),
                    )
                })
            })
            .collect();
        let label = lib.clone();
        chart
            .draw_series(bars)?
            .label(label)
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 12, y + 5)], color.filled()));
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
        .build_cartesian_2d(
            x_min.max(0.0)..x_max + 1.0,
            (y_floor..y_ceil).log_scale(),
        )?;

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
    for (lib, points) in sorted.iter().filter(|(l, _)| l.as_str() != "decimal-scaled") {
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
                .draw_series(std::iter::once(Circle::new(
                    (s as f64, n),
                    6,
                    RED.filled(),
                )))?
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
