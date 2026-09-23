//! Batch runner: simulate every parameter row and write one summary row each.
//!
//! coarse --grid GRID.json --params PARAMS.csv --sites SITES.csv --out OUT.csv
//!        [--dt YEARS] [--southern-crossing] [--source-max-lat DEGREES]
//!        [--strait STRAIT.json [--strait-series narrow|median|wide]]
//!
//! With --strait, the optional PARAMS.csv column crossing_km sets the longest
//! open-water leg people can cross (default 0, walking only).
//!
//! PARAMS.csv columns: id,seed,growth,diffusion,rain_half,density,detection
//! SITES.csv columns:  name,lat,lon
//! Ages in the output are years BP; empty fields mean "never happened".

use dispersal_coarse::grid::Grid;
use dispersal_coarse::model::{simulate, Params, Scenario, Strait};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::process::ExitCode;
use std::time::Instant;

fn read_csv(path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header: Vec<String> = lines
        .next()
        .ok_or(format!("{path}: empty"))?
        .split(',')
        .map(|h| h.trim().to_string())
        .collect();
    lines
        .enumerate()
        .map(|(i, line)| {
            let values: Vec<&str> = line.split(',').map(str::trim).collect();
            if values.len() != header.len() {
                return Err(format!(
                    "{path}: row {} has {} fields, expected {}",
                    i + 2,
                    values.len(),
                    header.len()
                ));
            }
            Ok(header
                .iter()
                .cloned()
                .zip(values.into_iter().map(String::from))
                .collect())
        })
        .collect()
}

fn field<T: std::str::FromStr>(row: &HashMap<String, String>, key: &str) -> Result<T, String> {
    row.get(key)
        .ok_or(format!("missing column {key}"))?
        .parse()
        .map_err(|_| format!("bad {key}: {}", row[key]))
}

fn age(v: Option<f64>) -> String {
    v.map(|x| format!("{x:.1}")).unwrap_or_default()
}

fn read_strait(grid: &Grid, path: &str, series: &str) -> Result<Strait, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
    let cell = |side: &str| -> Result<usize, String> {
        let e = &v["gridEdge"][side];
        let (lat, lon) = (e["lat"].as_f64(), e["lon"].as_f64());
        grid.cell_at(lat.ok_or("bad gridEdge")?, lon.ok_or("bad gridEdge")?)
            .ok_or(format!("strait {side} cell outside grid"))
    };
    let snapshots = v["snapshots"].as_array().ok_or("strait has no snapshots")?;
    let mut gaps = Vec::new();
    for (s, &bp) in snapshots.iter().zip(&grid.snapshots_bp) {
        if s["yearsBP"].as_f64() != Some(bp) {
            return Err("strait snapshots do not match the grid".into());
        }
        gaps.push(
            s["gapKm"][series]
                .as_f64()
                .ok_or(format!("strait series {series} missing"))?,
        );
    }
    Strait::new(grid, cell("africa")?, cell("arabia")?, gaps)
}

fn run(args: &[String]) -> Result<(), String> {
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let need = |name: &str| flag(name).ok_or(format!("missing {name}"));
    let grid = Grid::from_json(&std::fs::read_to_string(need("--grid")?).map_err(|e| e.to_string())?)?;
    let mut scenario = Scenario {
        southern_crossing: args.iter().any(|a| a == "--southern-crossing"),
        ..Default::default()
    };
    if let Some(dt) = flag("--dt") {
        scenario.dt = dt.parse().map_err(|_| "bad --dt")?;
    }
    if let Some(lat) = flag("--source-max-lat") {
        scenario.source_max_lat = lat.parse().map_err(|_| "bad --source-max-lat")?;
    }
    if let Some(path) = flag("--strait") {
        let series = flag("--strait-series").unwrap_or_else(|| "median".into());
        scenario.strait = Some(read_strait(&grid, &path, &series)?);
    }

    let sites = read_csv(&need("--sites")?)?;
    let mut names = Vec::new();
    let mut cells = Vec::new();
    for s in &sites {
        let name: String = field(s, "name")?;
        let cell = grid
            .cell_at(field(s, "lat")?, field(s, "lon")?)
            .ok_or(format!("site {name} is outside the grid"))?;
        if !(0..grid.snapshots_bp.len()).any(|i| grid.is_land(i, cell)) {
            return Err(format!("site {name} falls in a cell that is never land"));
        }
        names.push(name);
        cells.push(cell);
    }

    let rows = read_csv(&need("--params")?)?;
    let jobs: Vec<(u64, u64, Params)> = rows
        .iter()
        .map(|r| {
            Ok((
                field(r, "id")?,
                field(r, "seed")?,
                Params {
                    growth: field(r, "growth")?,
                    diffusion: field(r, "diffusion")?,
                    rain_half: field(r, "rain_half")?,
                    density: field(r, "density")?,
                    detection: field(r, "detection")?,
                    crossing_km: if r.contains_key("crossing_km") {
                        field(r, "crossing_km")?
                    } else {
                        0.0
                    },
                },
            ))
        })
        .collect::<Result<_, String>>()?;

    let started = Instant::now();
    let results: Vec<String> = jobs
        .par_iter()
        .map(|(id, seed, p)| {
            let o = simulate(&grid, p, &scenario, &cells, *seed).map_err(|e| format!("row {id}: {e}"))?;
            let mut line = format!(
                "{id},{},{:.5},{},{},{:.5},{},{},{}",
                age(o.arabia.onset_bp),
                o.arabia.occupied_fraction,
                o.arabia.final_population,
                age(o.levant.onset_bp),
                o.levant.occupied_fraction,
                o.levant.final_population,
                o.final_population,
                o.lost_to_sea
            );
            for r in &o.sites {
                let _ = write!(
                    line,
                    ",{},{},{}",
                    age(r.oldest_find_bp),
                    age(r.first_visit_bp),
                    age(r.first_established_bp)
                );
            }
            Ok(line)
        })
        .collect::<Result<_, String>>()?;

    let mut out = String::from(
        "id,arabia_onset_bp,arabia_occupied_fraction,arabia_final_population,levant_onset_bp,levant_occupied_fraction,levant_final_population,final_population,lost_to_sea",
    );
    for n in &names {
        let _ = write!(
            out,
            ",{n}__oldest_find_bp,{n}__first_visit_bp,{n}__established_bp"
        );
    }
    out.push('\n');
    for line in results {
        out.push_str(&line);
        out.push('\n');
    }
    let path = need("--out")?;
    std::fs::write(&path, out).map_err(|e| format!("{path}: {e}"))?;
    eprintln!(
        "{} runs, {} sites, dt {} yr, southern crossing {}, strait {}, source south of {}N: {:.1} s",
        jobs.len(),
        cells.len(),
        scenario.dt,
        scenario.southern_crossing,
        scenario.strait.is_some(),
        scenario.source_max_lat,
        started.elapsed().as_secs_f64()
    );
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
