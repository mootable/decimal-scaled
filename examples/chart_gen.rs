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
    for ((op, width), by_lib) in &data {
        let path = format!("{out_dir}/{op}_{width}.png");
        render_chart(&path, op, width, by_lib)?;
        count += 1;
    }
    println!("wrote {count} charts to {out_dir}/");
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
